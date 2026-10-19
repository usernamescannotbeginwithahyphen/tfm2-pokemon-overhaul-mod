//! Explicit ownership: normal basic hits may use engine crit; resolved/custom damage may not.
use std::cell::Cell;
thread_local! { static ENGINE_CRIT: Cell<bool> = const { Cell::new(false) }; }
thread_local! { static ENGINE_HIT: Cell<Option<(usize, usize)>> = const { Cell::new(None) }; }

pub fn engine_owns_roll() -> bool { ENGINE_CRIT.with(Cell::get) }

pub fn engine_owns_hit(attacker: usize, target: usize) -> bool {
    engine_owns_roll() && ENGINE_HIT.with(|hit| hit.get().map_or(true, |pair| pair == (attacker, target)))
}

pub fn with_engine_hit<T>(enabled: bool, attacker: usize, target: usize, operation: impl FnOnce() -> T) -> T {
    struct Restore(Option<(usize, usize)>);
    impl Drop for Restore {
        fn drop(&mut self) { ENGINE_HIT.with(|hit| hit.set(self.0)); }
    }
    let _restore = Restore(ENGINE_HIT.with(|hit| hit.replace(Some((attacker, target)))));
    with_engine_roll(enabled, operation)
}

pub fn with_engine_roll<T>(enabled: bool, operation: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) { ENGINE_CRIT.with(|state| state.set(self.0)); }
    }
    let _restore = Restore(ENGINE_CRIT.with(|state| state.replace(enabled)));
    operation()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ownership_restores_on_nested_hits_and_unwind() {
        assert!(!engine_owns_roll());
        with_engine_hit(true, 7, 8, || {
            assert!(engine_owns_hit(7, 8));
            assert!(!engine_owns_hit(8, 7));
            assert!(!engine_owns_hit(7, 9));
        });
        with_engine_roll(true, || {
            assert!(engine_owns_roll());
            with_engine_roll(false, || assert!(!engine_owns_roll()));
            assert!(engine_owns_roll());
        });
        let _ = std::panic::catch_unwind(|| with_engine_roll(true, || panic!("test unwind")));
        assert!(!engine_owns_roll());
    }
}
