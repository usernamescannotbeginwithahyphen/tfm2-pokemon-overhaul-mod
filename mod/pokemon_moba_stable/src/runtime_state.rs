//! Partition legacy state by the public simulation origin; never by entity ID alone.
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::sync::atomic::{AtomicU64, Ordering};
use mod_api_stable::StableSim;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(pub u32, pub u64, pub u64, pub u64, pub u64);
thread_local! { static CURRENT: Cell<Key> = Cell::new(Key::default()); }
static FINISHED: OnceLock<Mutex<BTreeSet<Key>>> = OnceLock::new();
static RETIRE_VERSION: AtomicU64 = AtomicU64::new(0);

fn sim_key(sim: &StableSim<'_>) -> Key {
    let origin = sim.sim_origin().unwrap_or_default();
    Key(origin.kind, origin.match_id, origin.replay_id, origin.set_index, sim.seed())
}
fn retire_key(key: Key) {
    if FINISHED.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner()).insert(key) {
        RETIRE_VERSION.fetch_add(1, Ordering::Release);
    }
}
pub fn retire_finished(sim: &StableSim<'_>) { if sim.is_end() { retire_key(sim_key(sim)); } }
pub fn start(sim: &StableSim<'_>) {
    FINISHED.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner()).remove(&sim_key(sim));
}

pub struct Scope(Key);
impl Scope {
    pub fn enter(sim: &StableSim<'_>) -> Self {
        retire_finished(sim);
        Self::key(sim_key(sim))
    }
    fn key(key: Key) -> Self { Self(CURRENT.with(|current| current.replace(key))) }
}
impl Drop for Scope {
    fn drop(&mut self) { CURRENT.with(|current| current.set(self.0)); }
}

// Retain the existing get_or_init(...).lock() call sites while separating their
// contents. The map guard owns the borrow; no leaked references or engine pointers.
pub trait Value { type Inner; fn into_value(self) -> Self::Inner; }
impl<T> Value for Mutex<T> {
    type Inner = T;
    fn into_value(self) -> T { self.into_inner().unwrap_or_else(|error| error.into_inner()) }
}
pub struct SimOnceLock<M: Value> { values: Mutex<BTreeMap<Key, M::Inner>>, retired: AtomicU64 }
impl<M: Value> SimOnceLock<M> {
    pub const fn new() -> Self { Self { values: Mutex::new(BTreeMap::new()), retired: AtomicU64::new(0) } }
    pub fn get_or_init(&self, init: impl FnOnce() -> M) -> &Self {
        let current = CURRENT.with(Cell::get);
        let mut values = self.values.lock().unwrap_or_else(|error| error.into_inner());
        let version = RETIRE_VERSION.load(Ordering::Acquire);
        if self.retired.load(Ordering::Relaxed) != version {
            if let Some(finished) = FINISHED.get() {
                let finished = finished.lock().unwrap_or_else(|e| e.into_inner());
                values.retain(|key, _| *key == current || !finished.contains(key));
            }
            self.retired.store(version, Ordering::Relaxed);
        }
        values.entry(current).or_insert_with(|| init().into_value());
        self
    }
    pub fn get(&self) -> Option<&Self> {
        self.values.lock().ok()?.contains_key(&CURRENT.with(Cell::get)).then_some(self)
    }
    pub fn lock(&self) -> Result<Guard<'_, M::Inner>, &'static str> {
        let values = self.values.lock().map_err(|_| "simulation state poisoned")?;
        let key = CURRENT.with(Cell::get);
        if !values.contains_key(&key) { return Err("simulation state not initialized"); }
        Ok(Guard { values, key })
    }
}
pub struct Guard<'a, T> { values: MutexGuard<'a, BTreeMap<Key, T>>, key: Key }
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { &self.values[&self.key] }
}
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { self.values.get_mut(&self.key).expect("guard retains key") }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interleaved_matches_replays_and_nested_callbacks_are_isolated() {
        let store = SimOnceLock::new();
        let a = Key(1, 12, u64::MAX, 0, 99);
        let b = Key(2, 12, 8, 0, 99);
        let _a = Scope::key(a);
        *store.get_or_init(|| Mutex::new(0)).lock().unwrap() = 7;
        {
            let _b = Scope::key(b);
            assert!(store.get().is_none());
            *store.get_or_init(|| Mutex::new(0)).lock().unwrap() = 42;
        }
        assert_eq!(*store.lock().unwrap(), 7);
        { let _b = Scope::key(b); assert_eq!(*store.lock().unwrap(), 42); }
    }
    #[test]
    fn finished_match_payloads_are_reclaimed_without_deleting_live_matches() {
        let store = SimOnceLock::new();
        let old = Key(1, 999, 0, 0, 11);
        { let _scope = Scope::key(old); store.get_or_init(|| Mutex::new(vec![1, 2, 3])); }
        retire_key(old);
        let _scope = Scope::key(Key(1, 1000, 0, 0, 12));
        store.get_or_init(|| Mutex::new(vec![4]));
        assert_eq!(store.values.lock().unwrap().len(), 1);
        assert_eq!(*store.lock().unwrap(), vec![4]);
    }
}
