//! Team intelligence from ability detections, without granting omniscient AI.
use crate::ai_world::{distance_sq, tower_entry, Unit, World};
use crate::stable_legacy_api::{EntityPos, Input};

const LOCAL: u64 = 120_000;
const SEARCH: u64 = 230_000;
const STEP: i128 = 60_000;
const MAP_MAX: i128 = 30 * 32_000 - 1;

fn known_enemy(unit: &Unit, caster: &Unit) -> bool {
    unit.champion && unit.team != caster.team && (unit.visible || unit.remembered)
}

fn nearby(a: EntityPos, b: EntityPos, radius: u64) -> bool {
    distance_sq(a, b) <= u128::from(radius).pow(2)
}

fn outnumbered(world: &World, at: EntityPos) -> bool {
    let Some(caster) = world.caster() else { return false; };
    let enemies = world.units.iter().filter(|unit| known_enemy(unit, caster) && nearby(unit.pos, at, LOCAL)).count();
    let allies = world.units.iter().filter(|unit| unit.champion && unit.team == caster.team
        && nearby(unit.pos, at, LOCAL)).count();
    enemies > allies.max(1)
}

pub fn safe_step(world: &World, end: EntityPos) -> bool {
    let Some(caster) = world.caster() else { return false; };
    !world.units.iter().any(|unit| unit.tower && unit.team != caster.team
        && tower_entry(caster.pos, end, unit.pos, 70_000, false))
}

/// Only intervene when shared detection adds knowledge absent from engine vision.
/// Return ranked alternatives so host path validation can reject blocked terrain.
pub fn defensive_moves(world: &World, base: Option<&Input>) -> Vec<Input> {
    let Some(caster) = world.caster() else { return vec![]; };
    if !matches!(base, None | Some(Input::Move { .. } | Input::Attack { .. } | Input::Return)) {
        return vec![];
    }
    let detected: Vec<_> = world.units.iter().filter(|unit| known_enemy(unit, caster)
        && unit.remembered && !unit.visible && nearby(caster.pos, unit.pos, SEARCH)).collect();
    if detected.is_empty() { return vec![]; }
    let threats: Vec<_> = world.units.iter().filter(|unit| known_enemy(unit, caster)
        && nearby(caster.pos, unit.pos, SEARCH)).collect();
    let low_hp = caster.hp.saturating_mul(100) < caster.max_hp.max(1).saturating_mul(40);
    let danger_here = threats.iter().any(|unit| nearby(caster.pos, unit.pos, LOCAL))
        && (low_hp || outnumbered(world, caster.pos));
    let destination = match base { Some(Input::Move { x, y }) => Some(EntityPos { x: *x, y: *y }), _ => None };
    let unsafe_route = destination.map_or(false, |end| detected.iter().any(|unit|
        tower_entry(caster.pos, end, unit.pos, 70_000, false)
            && (low_hp || outnumbered(world, unit.pos))));
    if !danger_here && !unsafe_route { return vec![]; }
    let clearance = |pos| threats.iter().map(|unit| distance_sq(pos, unit.pos)).min().unwrap_or(0);
    let before = clearance(caster.pos);
    let mut alternatives: Vec<_> = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .into_iter().map(|(dx, dy)| EntityPos {
            x: (caster.pos.x as i128 + dx * STEP).clamp(0, MAP_MAX) as u64,
            y: (caster.pos.y as i128 + dy * STEP).clamp(0, MAP_MAX) as u64,
        }).filter(|end| safe_step(world, *end) && clearance(*end) > before
            && threats.iter().all(|unit| !tower_entry(caster.pos, *end, unit.pos, 45_000, false)))
        .collect();
    alternatives.sort_by_key(|end| {
        let progress = if danger_here { 0 } else { destination.map_or(0, |goal| distance_sq(*end, goal)) };
        (progress, std::cmp::Reverse(clearance(*end)), end.x, end.y)
    });
    let mut inputs: Vec<_> = alternatives.into_iter().map(|end| Input::Move { x: end.x, y: end.y }).collect();
    // Never fall back to a known-dangerous advance when all escape paths are blocked.
    inputs.push(Input::Move { x: caster.pos.x, y: caster.pos.y });
    inputs
}

pub fn targets(world: &World) -> Vec<usize> {
    let Some(caster) = world.caster() else { return vec![]; };
    if caster.hp.saturating_mul(100) < caster.max_hp.max(1).saturating_mul(40) { return vec![]; }
    let mut targets: Vec<_> = world.units.iter().filter(|unit| known_enemy(unit, caster)
        && unit.remembered && !unit.visible && nearby(caster.pos, unit.pos, SEARCH)
        && !outnumbered(world, unit.pos)).collect();
    targets.sort_by_key(|unit| (distance_sq(caster.pos, unit.pos), unit.id));
    targets.into_iter().map(|unit| unit.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn unit(id: usize, team: usize, x: u64, y: u64) -> Unit {
        Unit { id, team, pos: EntityPos { x, y }, hp: 100, max_hp: 100,
            champion: true, tower: false, targetable: true, visible: team == 0,
            detected: team != 0, remembered: team != 0, cc: 0,
            stat: Default::default(), cooldowns: [None; 4] }
    }
    fn world() -> World {
        World { caster: 1, tick: 1, player: None, units: vec![unit(1, 0, 300000, 300000), unit(2, 1, 350000, 300000)] }
    }
    #[test]
    fn healthy_teammates_consider_detected_targets_but_not_unknown_enemies() {
        let mut world = world();
        assert_eq!(targets(&world), vec![2]);
        assert!(defensive_moves(&world, None).is_empty());
        world.units[1].detected = false;
        world.units[1].remembered = false;
        assert!(targets(&world).is_empty());
        world.units[0].hp = 10;
        assert!(defensive_moves(&world, None).is_empty());
    }
    #[test]
    fn detection_changes_retreat_and_outnumbered_decisions() {
        let mut world = world();
        world.units[0].hp = 25;
        let base = Input::Attack { target: crate::stable_legacy_api::InputTarget::Target { target_id: 2 } };
        let retreat = defensive_moves(&world, Some(&base));
        assert!(matches!(retreat[0], Input::Move { x, .. } if x < 300000));
        assert!(targets(&world).is_empty());
        world.units[0].hp = 100;
        world.units.push(unit(3, 1, 355000, 310000));
        assert!(!defensive_moves(&world, None).is_empty());
        assert!(targets(&world).is_empty());
        world.units.push(unit(4, 0, 310000, 300000));
        assert!(defensive_moves(&world, None).is_empty());
        assert_eq!(targets(&world).len(), 2);
    }
    #[test]
    fn hidden_ambush_changes_rotation_without_interrupting_safe_recalls_or_casts() {
        let mut world = world();
        world.units[1].pos.x = 480000;
        world.units.push(unit(3, 1, 490000, 310000));
        let route = Input::Move { x: 600000, y: 300000 };
        assert!(!defensive_moves(&world, Some(&route)).is_empty());
        assert!(defensive_moves(&world, Some(&Input::Return)).is_empty());
        let cast = Input::Skill { target: crate::stable_legacy_api::InputTarget::None };
        assert!(defensive_moves(&world, Some(&cast)).is_empty());
    }
    #[test]
    fn retreat_candidates_never_enter_enemy_tower_range() {
        let mut world = world();
        world.units[0].hp = 25;
        let mut tower = unit(3, 1, 200000, 300000);
        tower.tower = true; tower.champion = false;
        world.units.push(tower);
        for input in defensive_moves(&world, None) {
            if let Input::Move { x, y } = input {
                assert!(safe_step(&world, EntityPos { x, y }));
            }
        }
    }
}
