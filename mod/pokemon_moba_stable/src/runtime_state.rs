//! Partition legacy state by the public simulation origin; never by entity ID alone.
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::sync::atomic::{AtomicU64, Ordering};
use mod_api_stable::StableSim;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(pub u32, pub u64, pub u64, pub u64, pub u64);
const STATE_SHARDS: usize = 64;
#[derive(Clone, Copy)]
struct Context { key: Key, shard: usize }
impl Context {
    fn new(key: Key) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        Self { key, shard: hasher.finish() as usize % STATE_SHARDS }
    }
}
thread_local! { static CURRENT: Cell<Context> = Cell::new(Context::new(Key::default())); }
pub(crate) fn current_key() -> Key { CURRENT.with(|current| current.get().key) }
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

pub struct Scope(Context);
impl Scope {
    #[cfg(test)]
    pub(crate) fn test_key(key: Key) -> Self { Self::key(key) }
    pub fn enter(sim: &StableSim<'_>) -> Self {
        retire_finished(sim);
        Self::key(sim_key(sim))
    }
    fn key(key: Key) -> Self { Self(CURRENT.with(|current| current.replace(Context::new(key)))) }
}
impl Drop for Scope {
    fn drop(&mut self) { CURRENT.with(|current| current.set(self.0)); }
}

// Retain the existing get_or_init(...).lock() call sites while separating their
// contents. Shards prevent unrelated background matches from sharing every
// hot-path lock. The map guard owns the borrow; no leaked refs/engine pointers.
pub trait Value { type Inner; fn into_value(self) -> Self::Inner; }
impl<T> Value for Mutex<T> {
    type Inner = T;
    fn into_value(self) -> T { self.into_inner().unwrap_or_else(|error| error.into_inner()) }
}
struct Shard<T> { values: Mutex<BTreeMap<Key, T>>, retired: AtomicU64 }
impl<T> Shard<T> {
    const fn new() -> Self { Self { values: Mutex::new(BTreeMap::new()), retired: AtomicU64::new(0) } }
}
pub struct SimOnceLock<M: Value> { shards: [Shard<M::Inner>; STATE_SHARDS] }
impl<M: Value> SimOnceLock<M> {
    pub const fn new() -> Self { Self { shards: [const { Shard::new() }; STATE_SHARDS] } }
    /// Initialize and borrow under one lock instead of get_or_init().lock().
    pub fn lock_or_init(&self, init: impl FnOnce() -> M) -> Result<Guard<'_, M::Inner>, &'static str> {
        #[cfg(feature = "profile-state-access")]
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::State);
        let current = CURRENT.with(Cell::get);
        let shard = &self.shards[current.shard];
        let mut values = shard.values.lock().map_err(|_| "simulation state poisoned")?;
        let version = RETIRE_VERSION.load(Ordering::Acquire);
        if shard.retired.load(Ordering::Relaxed) != version {
            if let Some(finished) = FINISHED.get() {
                let finished = finished.lock().unwrap_or_else(|e| e.into_inner());
                values.retain(|key, _| *key == current.key || !finished.contains(key));
            }
            shard.retired.store(version, Ordering::Relaxed);
        }
        values.entry(current.key).or_insert_with(|| init().into_value());
        Ok(Guard { values, key: current.key })
    }
    pub fn get_or_init(&self, init: impl FnOnce() -> M) -> &Self {
        #[cfg(feature = "profile-state-access")]
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::State);
        let current = CURRENT.with(Cell::get);
        let shard = &self.shards[current.shard];
        let mut values = shard.values.lock().unwrap_or_else(|error| error.into_inner());
        let version = RETIRE_VERSION.load(Ordering::Acquire);
        if shard.retired.load(Ordering::Relaxed) != version {
            if let Some(finished) = FINISHED.get() {
                let finished = finished.lock().unwrap_or_else(|e| e.into_inner());
                values.retain(|key, _| *key == current.key || !finished.contains(key));
            }
            shard.retired.store(version, Ordering::Relaxed);
        }
        values.entry(current.key).or_insert_with(|| init().into_value());
        self
    }
    pub fn get(&self) -> Option<&Self> {
        #[cfg(feature = "profile-state-access")]
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::State);
        let current = CURRENT.with(Cell::get);
        self.shards[current.shard].values.lock().ok()?.contains_key(&current.key).then_some(self)
    }
    pub fn lock(&self) -> Result<Guard<'_, M::Inner>, &'static str> {
        #[cfg(feature = "profile-state-access")]
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::State);
        let current = CURRENT.with(Cell::get);
        let values = self.shards[current.shard].values.lock().map_err(|_| "simulation state poisoned")?;
        let key = current.key;
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
    fn combined_lock_initializes_once_and_preserves_nested_scope_identity() {
        let store = SimOnceLock::new();
        let _outer = Scope::key(Key(1, 98000, 0, 0, 1));
        *store.lock_or_init(|| Mutex::new(4)).unwrap() += 1;
        assert_eq!(*store.lock_or_init(|| panic!("must not reinitialize")).unwrap(), 5);
        {
            let _inner = Scope::key(Key(2, 98000, 0, 0, 1));
            *store.lock_or_init(|| Mutex::new(40)).unwrap() += 2;
            assert_eq!(*store.get_or_init(|| Mutex::new(0)).lock().unwrap(), 42);
        }
        assert_eq!(*store.lock_or_init(|| Mutex::new(0)).unwrap(), 5);
    }

    #[test]
    fn combined_lock_reclaims_finished_payloads_on_colliding_shards() {
        let store = SimOnceLock::new();
        let old = Key(1, 98100, 0, 0, 11);
        { let _scope = Scope::key(old); store.lock_or_init(|| Mutex::new(vec![1])).unwrap(); }
        retire_key(old);
        let next = (98101..).map(|id| Key(1, id, 0, 0, 12))
            .find(|key| Context::new(*key).shard == Context::new(old).shard).unwrap();
        let _scope = Scope::key(next);
        assert_eq!(*store.lock_or_init(|| Mutex::new(vec![2])).unwrap(), vec![2]);
        assert_eq!(store.shards[Context::new(old).shard].values.lock().unwrap().len(), 1);
    }

    #[test]
    #[ignore = "manual old/new state-lock benchmark, not end-to-end gameplay"]
    fn measure_single_lock_against_double_lock() {
        for threads in [1, 8, 16] {
            for combined in [false, true] {
                let store = SimOnceLock::new();
                let barrier = std::sync::Barrier::new(threads);
                let elapsed = std::thread::scope(|scope| {
                    let handles: Vec<_> = (0..threads).map(|index| {
                        let (store, barrier) = (&store, &barrier);
                        scope.spawn(move || {
                            let _scope = Scope::key(Key(1, 99000 + index as u64, 0, 0, index as u64));
                            store.lock_or_init(|| Mutex::new(0usize)).unwrap();
                            barrier.wait();
                            let start = std::time::Instant::now();
                            for _ in 0..100000 {
                                let mut value = if combined { store.lock_or_init(|| Mutex::new(0)).unwrap() }
                                    else { store.get_or_init(|| Mutex::new(0)).lock().unwrap() };
                                *value += 1;
                                std::hint::black_box(&*value);
                            }
                            assert_eq!(*store.lock().unwrap(), 100000);
                            start.elapsed()
                        })
                    }).collect();
                    handles.into_iter().map(|handle| handle.join().unwrap()).max().unwrap()
                });
                eprintln!("state_lock combined={combined} threads={threads} iterations_per_thread=100000 elapsed_us={}", elapsed.as_micros());
            }
        }
    }
    #[test]
    #[ignore = "manual timing probe; not a gameplay benchmark"]
    fn measure_parallel_state_access() {
        for threads in [1, 8, 16] {
            let store = SimOnceLock::new();
            let barrier = std::sync::Barrier::new(threads);
            let elapsed = std::thread::scope(|scope| {
                let handles: Vec<_> = (0..threads).map(|index| {
                    let (store, barrier) = (&store, &barrier);
                    scope.spawn(move || {
                        let _scope = Scope::key(Key(1, 50000 + index as u64, 0, 0, index as u64));
                        store.get_or_init(|| Mutex::new(vec![0usize; 64]));
                        barrier.wait();
                        let start = std::time::Instant::now();
                        for _ in 0..20000 {
                            let mut values = store.get_or_init(|| Mutex::new(vec![])).lock().unwrap();
                            for value in values.iter_mut() { *value += 1; }
                            std::hint::black_box(&*values);
                        }
                        assert!(store.lock().unwrap().iter().all(|value| *value == 20000));
                        start.elapsed()
                    })
                }).collect();
                handles.into_iter().map(|h| h.join().unwrap()).max().unwrap()
            });
            eprintln!("state_access threads={threads} updates={} elapsed_ms={} ns_per_update={}",
                threads * 20000, elapsed.as_millis(), elapsed.as_nanos() / (threads * 20000) as u128);
        }
    }
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
        // Retirement is lazy per shard, so reuse the old match's shard.
        let next = (1000..).map(|id| Key(1, id, 0, 0, 12))
            .find(|key| Context::new(*key).shard == Context::new(old).shard).unwrap();
        let _scope = Scope::key(next);
        store.get_or_init(|| Mutex::new(vec![4]));
        assert_eq!(store.shards[Context::new(old).shard].values.lock().unwrap().len(), 1);
        assert_eq!(*store.lock().unwrap(), vec![4]);
    }

    #[test]
    fn unrelated_shards_progress_while_a_match_holds_its_guard() {
        let store = SimOnceLock::new();
        let a = Key(1, 70000, 0, 0, 0);
        let b = (70001..).map(|id| Key(1, id, 0, 0, 0))
            .find(|key| Context::new(*key).shard != Context::new(a).shard).unwrap();
        let _a = Scope::key(a);
        let guard = store.get_or_init(|| Mutex::new(7)).lock().unwrap();
        std::thread::scope(|scope| {
            let (send, receive) = std::sync::mpsc::channel();
            let store = &store;
            scope.spawn(move || {
                let _b = Scope::key(b);
                *store.get_or_init(|| Mutex::new(0)).lock().unwrap() = 42;
                send.send(()).unwrap();
            });
            let completed = receive.recv_timeout(std::time::Duration::from_secs(5));
            assert_eq!(*guard, 7);
            drop(guard);
            assert!(completed.is_ok(), "unrelated match blocked behind held state guard");
        });
        let _b = Scope::key(b);
        assert_eq!(*store.lock().unwrap(), 42);
    }

    #[test]
    fn concurrent_callbacks_for_the_same_match_are_serialized() {
        let store = SimOnceLock::new();
        let key = Key(1, 80000, 0, 0, 123);
        std::thread::scope(|scope| {
            for worker in 0..8 {
                let store = &store;
                scope.spawn(move || {
                    let _scope = Scope::key(key);
                    for _ in 0..1000 {
                        let mut value = if worker % 2 == 0 { store.lock_or_init(|| Mutex::new(0)).unwrap() }
                            else { store.get_or_init(|| Mutex::new(0)).lock().unwrap() };
                        *value += 1;
                    }
                });
            }
        });
        let _scope = Scope::key(key);
        assert_eq!(*store.lock().unwrap(), 8000);
    }

    #[test]
    fn colliding_shards_keep_match_payloads_separate() {
        let store = SimOnceLock::new();
        let a = Key(1, 90000, 0, 0, 0);
        let b = (90001..).map(|id| Key(1, id, 0, 0, 0))
            .find(|key| Context::new(*key).shard == Context::new(a).shard).unwrap();
        { let _scope = Scope::key(a); *store.get_or_init(|| Mutex::new(0)).lock().unwrap() = 11; }
        { let _scope = Scope::key(b); *store.get_or_init(|| Mutex::new(0)).lock().unwrap() = 22; }
        { let _scope = Scope::key(a); assert_eq!(*store.lock().unwrap(), 11); }
        { let _scope = Scope::key(b); assert_eq!(*store.lock().unwrap(), 22); }
    }
}
