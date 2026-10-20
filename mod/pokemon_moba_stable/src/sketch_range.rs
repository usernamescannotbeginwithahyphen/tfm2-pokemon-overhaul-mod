//! Copied range metadata lives on the entity: range_adjust has no simulation context.
use mod_api_stable::{BuffV1, StableEntityView, StableSim};

const PREFIX: &str = "pokemon_sketch_range_";
pub(crate) type Ranges = [Option<(u64, u64)>; 4];

fn marker(slot: usize, range: u64, growth: u64) -> BuffV1 {
    BuffV1::named(&format!("{PREFIX}{slot}_{range}_{growth}"))
}

fn parse(name: &str) -> Option<(usize, u64, u64)> {
    let mut parts = name.strip_prefix(PREFIX)?.split('_');
    let slot = parts.next()?.parse().ok()?;
    let range = parts.next()?.parse().ok()?;
    let growth = parts.next()?.parse().ok()?;
    (slot < 4 && parts.next().is_none()).then_some((slot, range, growth))
}

pub(crate) fn sync(sim: &mut StableSim<'_>, caster: usize, ranges: Ranges) {
    let Some(entity) = sim.get_entity(caster) else { return; };
    let mut present = [false; 4];
    let mut stale = Vec::new();
    for index in 0..entity.buff_count() {
        let Some(buff) = entity.buff_at(index) else { continue; };
        if !buff.name().starts_with(PREFIX) { continue; }
        match parse(buff.name()) {
            Some((slot, range, growth)) if ranges[slot] == Some((range, growth)) => {
                present[slot] = true;
            }
            _ => stale.push(buff.name().to_owned()),
        }
    }
    for name in stale { sim.entity_remove_buff(caster, &name); }
    for (slot, range) in ranges.into_iter().enumerate() {
        if present[slot] { continue; }
        if let Some((range, growth)) = range {
            sim.entity_stack_buff(caster, &marker(slot, range, growth), 1, false);
        }
    }
}

pub(crate) fn adjustment(caster: &StableEntityView<'_>, slot: usize, base: u64) -> u64 {
    for index in 0..caster.buff_count() {
        let Some(buff) = caster.buff_at(index) else { continue; };
        if let Some((learned_slot, range, growth)) = parse(buff.name()) {
            if learned_slot == slot {
                return range.saturating_add(growth.saturating_mul(caster.level().saturating_sub(1) as u64))
                    .saturating_sub(base);
            }
        }
    }
    0
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use mod_api_stable::{EntityViewV1, EntityViewVtableV1};
    use std::ffi::c_void;

    struct View { level: usize, buffs: Vec<BuffV1> }
    unsafe extern "C" fn level(state: *const c_void) -> usize { (*(state as *const View)).level }
    unsafe extern "C" fn count(state: *const c_void) -> usize { (*(state as *const View)).buffs.len() }
    unsafe extern "C" fn at(state: *const c_void, index: usize, out: *mut BuffV1) -> bool {
        if let Some(buff) = (&(*(state as *const View)).buffs).get(index) { *out = *buff; true } else { false }
    }

    pub(crate) fn with_view<T>(buffs: Vec<BuffV1>, level_value: usize, f: impl FnOnce(&StableEntityView<'_>) -> T) -> T {
        let state = View { level: level_value, buffs };
        let table = EntityViewVtableV1 {
            size: std::mem::size_of::<EntityViewVtableV1>(), id: None, team: None,
            level: Some(level), kind: None, hp: None, stat: None, pos: None, shield: None,
            buff_count: Some(count), buff_at: Some(at), cc_count: None, cc_at: None,
        };
        let raw = EntityViewV1 { size: std::mem::size_of::<EntityViewV1>(), vtable: &table,
            state: &state as *const View as *const c_void };
        let view = unsafe { StableEntityView::from_raw(&raw).unwrap() };
        f(&view)
    }

    #[test]
    fn range_queries_are_entity_local_without_any_simulation_scope() {
        with_view(vec![], 1, |view| {
            for slot in 0..4 { assert_eq!(adjustment(view, slot, 42000), 0); }
        });
        with_view(vec![marker(0, 900000, 0), marker(1, 50000, 1000), marker(2, 20000, 0)], 5, |view| {
            assert_eq!(adjustment(view, 0, 42000), 858000);
            assert_eq!(adjustment(view, 1, 42000), 12000);
            // The SDK only adds range. The copied-move target gate enforces shorter ranges.
            assert_eq!(adjustment(view, 2, 42000), 0);
            assert_eq!(adjustment(view, 3, 42000), 0);
            with_view(vec![], 5, |other| assert_eq!(adjustment(other, 0, 42000), 0));
        });
    }

    #[test]
    fn markers_are_neutral_and_fit_the_abi_even_at_maximum_range() {
        let mut buff = marker(3, u64::MAX, u64::MAX);
        assert_eq!(parse(buff.name()), Some((3, u64::MAX, u64::MAX)));
        buff.set_name("");
        buff.name_buf.fill(0);
        assert_eq!(buff, BuffV1::default());
        for invalid in ["other", "pokemon_sketch_range_4_1_0", "pokemon_sketch_range_0_bad_0", "pokemon_sketch_range_0_1_0_extra"] {
            assert_eq!(parse(invalid), None);
        }
    }
}
