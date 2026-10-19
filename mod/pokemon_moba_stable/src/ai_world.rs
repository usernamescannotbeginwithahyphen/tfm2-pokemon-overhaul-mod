//! Owned, read-only AI facts. No simulation pointer escapes the SDK callback.
use mod_api_stable::{CooldownV1, SkillSlotV1, StableSim, StatV1};
use crate::stable_legacy_api::EntityPos;

#[derive(Clone, Debug)]
pub struct Unit {
    pub id: usize,
    pub team: usize,
    pub pos: EntityPos,
    pub hp: usize,
    pub max_hp: usize,
    pub champion: bool,
    pub tower: bool,
    pub targetable: bool,
    pub visible: bool,
    pub detected: bool,
    pub remembered: bool,
    pub cc: usize,
    pub stat: StatV1,
    pub cooldowns: [Option<CooldownV1>; 4],
}

#[derive(Clone, Debug)]
pub struct World {
    pub tick: usize,
    pub caster: usize,
    pub player: Option<usize>,
    pub units: Vec<Unit>,
}

impl World {
    pub fn capture(sim: &StableSim<'_>, caster: usize) -> Self {
        Self::capture_members(sim, caster, None, true)
    }
    pub fn for_caster(sim: &StableSim<'_>, caster: usize) -> Self {
        Self::capture_members(sim, caster, None, false)
    }
    pub fn for_target(sim: &StableSim<'_>, caster: usize, target: usize) -> Self {
        Self::capture_members(sim, caster, Some(target), false)
    }
    fn capture_members(sim: &StableSim<'_>, caster: usize, target: Option<usize>, all: bool) -> Self {
        let team = sim.get_entity(caster).map(|unit| unit.team()).unwrap_or(usize::MAX);
        let spots = crate::legacy::pokemon_status::team_spotted_targets(team, sim.tick());
        let player = (0..sim.player_count()).filter_map(|i| sim.player_at(i))
            .find(|player| player.champion().map(|unit| unit.id()) == Some(caster)).map(|player| player.id());
        let units = (0..sim.entity_count()).filter_map(|i| sim.entity_at(i))
            .filter(|unit| all || unit.id() == caster || Some(unit.id()) == target
                || (target.is_some() && (unit.is_tower() || unit.is_champion())))
            .filter(|unit| unit.is_alive()).map(|unit| {
                let (x, y) = unit.pos();
                let (hp, max_hp) = unit.hp();
                let visible = unit.team() == team || sim.is_visible(team, unit.id());
                let spot = spots.iter().find(|spot| spot.target_id == unit.id());
                // Hidden enemies only contribute the position our detector actually observed.
                let pos = if visible { EntityPos { x, y } }
                    else { spot.map(|spot| spot.target_pos).unwrap_or(EntityPos { x, y }) };
                Unit { id: unit.id(), team: unit.team(), pos, hp, max_hp,
                    champion: unit.is_champion(), tower: unit.is_tower(), targetable: unit.is_targetable(),
                    visible, detected: spot.map_or(false, |spot| sim.tick().saturating_sub(spot.observed_at) <= 8),
                    remembered: spot.is_some(), cc: unit.cc_count(),
                    stat: if unit.id() == caster { unit.stat() } else { StatV1::default() },
                    cooldowns: [SkillSlotV1::Attack, SkillSlotV1::Skill, SkillSlotV1::Skill2, SkillSlotV1::Ult]
                        .map(|slot| if unit.id() == caster { unit.cooldown(slot) } else { None }),
                }
            }).collect();
        Self { tick: sim.tick(), caster, player, units }
    }
    pub fn get(&self, id: usize) -> Option<&Unit> { self.units.iter().find(|unit| unit.id == id) }
    pub fn caster(&self) -> Option<&Unit> { self.get(self.caster) }
}

pub fn distance_sq(a: EntityPos, b: EntityPos) -> u128 {
    let x = a.x as i128 - b.x as i128;
    let y = a.y as i128 - b.y as i128;
    (x * x + y * y) as u128
}

pub fn project(start: EntityPos, aim: EntityPos, range: u64) -> EntityPos {
    let dx = aim.x as i128 - start.x as i128;
    let dy = aim.y as i128 - start.y as i128;
    let length = (dx * dx + dy * dy).isqrt();
    if length == 0 { return start; }
    EntityPos { x: (start.x as i128 + dx * range as i128 / length).max(0) as u64,
        y: (start.y as i128 + dy * range as i128 / length).max(0) as u64 }
}

pub fn tower_entry(start: EntityPos, end: EntityPos, tower: EntityPos, radius: u64, blink: bool) -> bool {
    let radius_sq = u128::from(radius).pow(2);
    let before = distance_sq(start, tower);
    let after = distance_sq(end, tower);
    // Allow an escape from an already-dangerous position, not a deeper dive.
    if before <= radius_sq && after > before { return false; }
    if after <= radius_sq { return true; }
    if blink { return false; }
    let dx = end.x as i128 - start.x as i128;
    let dy = end.y as i128 - start.y as i128;
    let length_sq = dx * dx + dy * dy;
    if length_sq == 0 { return false; }
    let dot = ((tower.x as i128 - start.x as i128) * dx + (tower.y as i128 - start.y as i128) * dy).clamp(0, length_sq);
    let nearest = EntityPos { x: (start.x as i128 + dx * dot / length_sq).max(0) as u64,
        y: (start.y as i128 + dy * dot / length_sq).max(0) as u64 };
    distance_sq(nearest, tower) <= radius_sq
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tower_guard_checks_dashes_blinks_and_escapes_differently() {
        let tower = EntityPos { x: 100, y: 100 };
        let left = EntityPos { x: 0, y: 100 };
        let right = EntityPos { x: 200, y: 100 };
        assert!(tower_entry(left, right, tower, 30, false));
        assert!(!tower_entry(left, right, tower, 30, true));
        assert!(tower_entry(left, tower, tower, 30, true));
        assert!(!tower_entry(tower, right, tower, 30, false));
        assert!(tower_entry(EntityPos { x: 120, y: 100 }, tower, tower, 30, false));
    }
}
