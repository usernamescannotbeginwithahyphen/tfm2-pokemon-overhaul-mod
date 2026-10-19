//! Read-only equipment stat riders. This never edits inventory or copies item procs.
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

use mod_api_stable::{SettingTargetV1, StableMatchHook, StableServerCtx, StableServerExtension, StableSim};
use serde_json::Value;

use crate::stable_legacy_api::{BuffState, BuffType, EntityStat, GameCtx};

const FIELDS: [&str; 18] = [
    "attack", "magic_power", "hp", "defence", "magic_resistance", "crit_chance",
    "attack_speed_mult", "move_speed_mult", "skill_cooldown_mult", "ult_cooldown_mult",
    "attack_mult", "magic_power_mult", "hp_mult", "defence_mult", "magic_resistance_mult",
    "hp_regen", "vamp", "radius_mult",
];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EquipmentStats([i32; FIELDS.len()]);

impl EquipmentStats {
    fn from_json(value: &Value) -> Option<Self> {
        let object = value.as_object()?;
        let mut stats = Self::default();
        for (index, field) in FIELDS.iter().enumerate() {
            if let Some(value) = object.get(*field) {
                stats.0[index] = value.as_i64()?.clamp(0, i32::MAX as i64) as i32;
            }
        }
        Some(stats)
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|value| *value == 0)
    }

    fn add(&mut self, other: &Self) {
        for (total, value) in self.0.iter_mut().zip(other.0) {
            *total = total.saturating_add(value);
        }
    }

    pub fn buff(&self, percent: i32, ticks: usize) -> BuffState {
        let value = |index: usize| -> i32 {
            (i64::from(self.0[index]) * i64::from(percent) / 100)
                .clamp(i32::MIN as i64, i32::MAX as i64) as i32
        };
        BuffState {
            duration: BuffType::Time { tick: ticks },
            attack: value(0), magic_power: value(1), hp: value(2),
            defence: value(3), magic_resistance: value(4), crit_chance: value(5),
            attack_speed_mult: value(6), move_speed_mult: value(7),
            skill_cooldown_mult: value(8), ult_cooldown_mult: value(9),
            attack_mult: value(10), magic_power_mult: value(11), hp_mult: value(12),
            defence_mult: value(13), magic_resistance_mult: value(14),
            hp_regen: value(15), vamp: value(16), radius_mult: value(17),
            ..Default::default()
        }
    }

    pub fn steal(&self, roll: u64, percent: i32) -> Option<Self> {
        let percent = percent.clamp(0, 100);
        let eligible: Vec<usize> = self.0.iter().enumerate()
            .filter(|(_, value)| i64::from(**value) * i64::from(percent) / 100 > 0)
            .map(|(index, _)| index).collect();
        if eligible.is_empty() { return None; }
        let index = eligible[(roll % eligible.len() as u64) as usize];
        let mut result = Self::default();
        result.0[index] = (i64::from(self.0[index]) * i64::from(percent) / 100) as i32;
        Some(result)
    }

    pub fn poltergeist_stats(&self, base: EntityStat) -> EntityStat {
        let flat = |index: usize| self.0[index] as usize;
        let scaled = |flat_index: usize, mult_index: usize, base: usize| {
            flat(flat_index).saturating_add(base.saturating_mul(flat(mult_index)) / 100)
        };
        EntityStat {
            attack: scaled(0, 10, base.attack),
            magic_power: scaled(1, 11, base.magic_power),
            hp: scaled(2, 12, base.hp),
            defence: scaled(3, 13, base.defence),
            magic_resistance: scaled(4, 14, base.magic_resistance),
            crit_chance: flat(5),
            move_speed: base.move_speed.saturating_mul(flat(7)) / 100,
            ..Default::default()
        }
    }
}

type Catalog = BTreeMap<String, EquipmentStats>;

fn parse_catalog(json: &str) -> Option<Catalog> {
    let root: Value = serde_json::from_str(json).ok()?;
    let root = root.as_object()?;
    let mut catalog = Catalog::new();
    let mut insert = |item: &Value| -> Option<()> {
        let key = item.get("key")?.as_str()?;
        let stats = match item.get("stat") {
            Some(stat) => EquipmentStats::from_json(stat)?,
            None => EquipmentStats::default(),
        };
        if catalog.insert(key.to_owned(), stats).is_some() { return None; }
        Some(())
    };
    for (slot, item) in root {
        if slot == "mod_items" {
            for item in item.as_array()? { insert(item)?; }
        } else if item.get("key").is_some() {
            insert(item)?;
        }
    }
    Some(catalog)
}

#[derive(Default)]
struct CatalogState {
    json: Option<String>,
    current: Option<Arc<Catalog>>,
    // Freeze settings for a running match, including same-process replay/resimulation.
    matches: BTreeMap<u64, Option<Arc<Catalog>>>,
}

impl CatalogState {
    fn update(&mut self, json: Option<String>, reset: bool) {
        if reset { self.matches.clear(); }
        if reset || self.json != json {
            self.current = json.as_deref().and_then(parse_catalog).map(Arc::new);
            self.json = json;
            crate::legacy::crash_probe::log_stat_probe_event(&format!(
                "equipment_catalog ready={} items={}", self.current.is_some(),
                self.current.as_ref().map_or(0, |catalog| catalog.len())
            ));
        }
    }

    fn freeze(&mut self, seed: u64) {
        self.matches.entry(seed).or_insert_with(|| self.current.clone());
    }
}

static CATALOGS: OnceLock<Mutex<CatalogState>> = OnceLock::new();

pub struct EquipmentSettings;

impl EquipmentSettings {
    fn refresh(&self, ctx: &StableServerCtx<'_>, reset: bool) {
        let json = ctx.setting_get_json(SettingTargetV1::ItemSetting, "");
        let Ok(mut state) = CATALOGS.get_or_init(Default::default).lock() else { return; };
        state.update(json, reset);
    }
}

impl StableServerExtension for EquipmentSettings {
    fn on_server_start(&self, ctx: &mut StableServerCtx<'_>) { self.refresh(ctx, true); }
    fn before_management_tick(&self, ctx: &mut StableServerCtx<'_>) { self.refresh(ctx, false); }
    fn after_management_tick(&self, ctx: &mut StableServerCtx<'_>) { self.refresh(ctx, false); }
}

impl StableMatchHook for EquipmentSettings {
    fn on_match_start(&self, sim: &mut StableSim<'_>) {
        crate::runtime_state::start(sim);
        let _scope = crate::runtime_state::Scope::enter(sim);
        crate::legacy::pokemon_status::reset_pokemon_status_runtime_state_for_new_match();
        crate::legacy::pokemon_content::reset_pokemon_content_runtime_state_for_new_match();
        crate::map_terrain::on_match_start(sim);
        crate::motion::reset();
        crate::projectiles::reset();
        let Ok(mut state) = CATALOGS.get_or_init(Default::default).lock() else { return; };
        state.freeze(sim.seed());
    }
    fn on_match_tick(&self, sim: &mut StableSim<'_>, _seed: u64) {
        crate::runtime_state::retire_finished(sim);
        let mut ctx = GameCtx::from_stable(sim);
        crate::motion::update(&mut ctx, _seed);
        crate::projectiles::update(&mut ctx, _seed);
    }
}

pub fn for_entity(ctx: &GameCtx, entity: usize) -> Option<EquipmentStats> {
    let keys = ctx.entity_item_keys(entity)?;
    let catalog = CATALOGS.get_or_init(Default::default).lock().ok()?
        .matches.get(&ctx.seed())?.clone()?;
    stats_for_keys(&catalog, &keys)
}

fn stats_for_keys(catalog: &Catalog, keys: &[String]) -> Option<EquipmentStats> {
    let mut total = EquipmentStats::default();
    for key in keys {
        // Unknown inventory entries must never become unrelated total-stat surplus.
        total.add(catalog.get(key)?);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEMS: &str = r#"{
        "iron_blade": {"key":"ironsword","stat":{"attack":15}},
        "mod_items":[{"key":"custom","stat":{"magic_power":40,"attack_speed_mult":20},
            "procs":[{"proc_ref":"guardian_blade"}]}]
    }"#;

    #[test]
    fn resolves_owned_keys_and_mod_items_not_setting_slot_names() {
        let catalog = parse_catalog(ITEMS).unwrap();
        let stats = stats_for_keys(&catalog, &["ironsword".into(), "custom".into()]).unwrap();
        let buff = stats.buff(50, 45);
        assert_eq!((buff.attack, buff.magic_power, buff.attack_speed_mult), (7, 20, 10));
        assert!(stats_for_keys(&catalog, &["iron_blade".into()]).is_none());
        assert!(stats_for_keys(&catalog, &["unknown".into()]).is_none());
        assert!(stats_for_keys(&catalog, &[]).unwrap().is_empty());
    }

    #[test]
    fn thief_transfers_equal_expiring_contributions_without_dividing_by_zero() {
        let stats = EquipmentStats::from_json(&serde_json::json!({"attack":50})).unwrap();
        let stolen = stats.steal(9, 20).unwrap();
        let gain = stolen.buff(100, 180);
        let loss = stolen.buff(-100, 180);
        assert_eq!(gain.attack, 10);
        assert_eq!(loss.attack, -10);
        assert!(matches!(gain.duration, BuffType::Time { tick: 180 }));
        assert!(matches!(loss.duration, BuffType::Time { tick: 180 }));
        assert!(stats.steal(0, 0).is_none());
        assert!(EquipmentStats::default().steal(0, 20).is_none());
        assert_eq!(stats.steal(0, 500).unwrap().buff(100, 1).attack, 50);
    }

    #[test]
    fn rejects_malformed_and_ambiguous_settings_and_ignores_procs() {
        assert!(parse_catalog("not json").is_none());
        assert!(parse_catalog(r#"{"x":{"key":"x","stat":{"attack":"lots"}}}"#).is_none());
        assert!(parse_catalog(r#"{"x":{"key":"x"},"mod_items":[{"key":"x"}]}"#).is_none());
        assert!(EquipmentStats::from_json(&serde_json::json!({"attack":-50,"cc_immune":true})).unwrap().is_empty());
        let catalog = parse_catalog(ITEMS).unwrap();
        assert_eq!(catalog["custom"].buff(100, 1).hp, 0);
    }

    #[test]
    fn poltergeist_uses_equipment_and_base_scaling_not_live_buff_surplus() {
        let stats = EquipmentStats::from_json(&serde_json::json!({
            "attack":50,"hp":100,"move_speed_mult":10,"defence_mult":20
        })).unwrap();
        let result = stats.poltergeist_stats(EntityStat { defence:50, move_speed:1000, ..Default::default() });
        assert_eq!((result.attack, result.hp, result.move_speed, result.defence), (50, 100, 100, 10));
        assert_eq!(result.magic_power, 0);
    }

    #[test]
    fn settings_changes_only_affect_new_matches_and_missing_settings_fail_closed() {
        let mut state = CatalogState::default();
        state.update(Some(ITEMS.into()), true);
        state.freeze(1);
        state.update(Some(ITEMS.replace("15", "30")), false);
        state.freeze(1);
        state.freeze(2);
        assert_eq!(state.matches[&1].as_ref().unwrap()["ironsword"].buff(100, 1).attack, 15);
        assert_eq!(state.matches[&2].as_ref().unwrap()["ironsword"].buff(100, 1).attack, 30);
        state.update(None, true);
        state.freeze(3);
        assert!(state.matches[&3].is_none());
        assert!(!state.matches.contains_key(&1));
    }
}
