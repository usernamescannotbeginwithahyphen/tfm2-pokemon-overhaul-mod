//! Bounded diagnostics for foreground matches and two background simulations.
use std::cell::Cell;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};

use mod_api_stable::{InputV1, StableSim};
use serde_json::{json, Value};

use crate::ai_world::World;
use crate::runtime_state::{Key, SimOnceLock};

const MAX_BYTES: [usize; 2] = [32 * 1024 * 1024, 4 * 1024 * 1024];
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Subject { Decision(usize), World, End, Strategy, Gate(usize, u8, bool) }
static SAMPLES: SimOnceLock<Mutex<BTreeMap<Subject, usize>>> = SimOnceLock::new();
static WRITERS: [OnceLock<Mutex<TraceWriter>>; 2] = [const { OnceLock::new() }; 2];
static START: OnceLock<std::time::Instant> = OnceLock::new();
static SELECTED: OnceLock<Mutex<Vec<Key>>> = OnceLock::new();
static CLOSED: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];
thread_local! { static SELECTION: Cell<Option<(Key, bool)>> = const { Cell::new(None) }; }

fn foreground(key: Key) -> bool { matches!(key.0, 2 | 3 | 4) }
fn stream(key: Key) -> usize { usize::from(!foreground(key)) }

fn select_background(selected: &mut Vec<Key>, key: Key) -> bool {
    if key.0 != 1 { return false; }
    if selected.contains(&key) { return true; }
    if selected.len() >= 2 { return false; }
    selected.push(key);
    true
}

fn enabled(key: Key) -> bool {
    if cfg!(test) { return false; }
    if CLOSED[stream(key)].load(Ordering::Relaxed) { return false; }
    if foreground(key) { return true; }
    SELECTION.with(|cache| {
        if let Some((previous, enabled)) = cache.get() { if previous == key { return enabled; } }
        let enabled = SELECTED.get_or_init(Default::default).lock()
            .map(|mut selected| select_background(&mut selected, key)).unwrap_or(false);
        cache.set(Some((key, enabled)));
        enabled
    })
}

pub fn initialize() {
    START.get_or_init(std::time::Instant::now);
    for kind in [1, 2] {
        record(Key(kind, 0, 0, 0, 0), json!({"type": "diagnostics_start", "background_match_limit": 2}));
    }
}

fn sample_due(last: &mut Option<usize>, tick: usize, interval: usize) -> bool {
    if last.is_some_and(|previous| tick >= previous && tick - previous < interval) {
        return false;
    }
    *last = Some(tick);
    true
}

fn sample(subject: Subject, tick: usize, interval: usize) -> bool {
    let mut samples = SAMPLES.get_or_init(Default::default).lock().unwrap();
    let mut last = samples.get(&subject).copied();
    if !sample_due(&mut last, tick, interval) { return false; }
    samples.insert(subject, tick);
    true
}

pub(crate) struct TraceWriter { file: Option<File>, bytes: usize, limit: usize }
impl TraceWriter {
    pub(crate) fn open(name: &str, limit: usize) -> Self {
        let file = std::env::var_os("APPDATA").and_then(|root| {
            let path = std::path::PathBuf::from(root)
                .join("TeamSamoyed/TeamfightManager2/data").join(name);
            OpenOptions::new().create(true).truncate(true).write(true).open(path).ok()
        });
        Self { file, bytes: 0, limit }
    }
    pub(crate) fn write(&mut self, value: &Value) {
        if self.file.is_none() || self.bytes >= self.limit { return; }
        let line = value.to_string();
        if line.len() + 1 > self.limit - self.bytes { self.file = None; return; }
        if writeln!(self.file.as_mut().unwrap(), "{line}").is_err() { self.file = None; }
        self.bytes += line.len() + 1;
    }
}

fn record(key: Key, event: Value) {
    let index = stream(key);
    let name = if index == 0 { "pokemon_moba_stable-ai.log" } else { "pokemon_moba_stable-ai-background.log" };
    let Ok(mut writer) = WRITERS[index].get_or_init(|| Mutex::new(TraceWriter::open(
        name, MAX_BYTES[index]))).lock() else { return; };
    writer.write(&json!({"build": env!("CARGO_PKG_VERSION"), "origin": key.0,
        "match": key.1, "replay": key.2, "set": key.3, "seed": key.4,
        "wall_ms": START.get_or_init(std::time::Instant::now).elapsed().as_millis(), "event": event}));
    if writer.file.is_none() { CLOSED[index].store(true, Ordering::Relaxed); }
}

fn input(input: Option<InputV1>) -> Value {
    input.map_or(Value::Null, |input| json!({"kind": input.kind,
        "move": [input.x, input.y], "target_kind": input.target.kind,
        "target": input.target.target_id, "aim": [input.target.x, input.target.y],
        "direction": [input.target.dir_x, input.target.dir_y]}))
}

pub fn match_start(sim: &StableSim<'_>) {
    let key = crate::runtime_state::current_key();
    if !foreground(key) || !enabled(key) { return; }
    SAMPLES.get_or_init(Default::default).lock().unwrap().clear();
    record(key, json!({"type": "start", "tick": sim.tick(), "entities": sim.entity_count(),
        "players": sim.player_count()}));
}

pub fn teleport(tick: usize, entity: usize, x: u64, y: u64, accepted: bool) {
    let key = crate::runtime_state::current_key();
    if !enabled(key) { return; }
    record(key, json!({"type": "mod_teleport", "tick": tick, "entity": entity,
        "destination": [x, y], "accepted": accepted}));
}

pub fn decision(world: Option<&World>, champion: &str, base: Option<InputV1>,
    selected: Option<InputV1>, final_input: Option<InputV1>, disposition: &str) {
    let key = crate::runtime_state::current_key();
    if !enabled(key) { return; }
    let Some(world) = world else { return; };
    if !sample(Subject::Decision(world.caster), world.tick, if foreground(key) { 60 } else { 240 }) { return; }
    let Some(caster) = world.caster() else { return; };
    record(key, json!({"type": "decision", "tick": world.tick, "player": world.player,
        "entity": caster.id, "champion": champion, "team": caster.team,
        "pos": [caster.pos.x, caster.pos.y], "hp": [caster.hp, caster.max_hp], "cc": caster.cc,
        "base": input(base), "selected": input(selected), "final": input(final_input),
        "disposition": disposition,
        "cooldowns": caster.cooldowns.map(|cooldown| cooldown.map(|c|
            json!({"unlocked": c.unlocked, "charges": c.charges, "remaining": c.remaining}))),
        "detected_enemies": world.units.iter().filter(|u| u.team != caster.team && u.remembered)
            .map(|u| json!({"id": u.id, "pos": [u.pos.x, u.pos.y], "visible": u.visible,
                "detected": u.detected})).collect::<Vec<_>>() }));
}

pub fn gate(world: &World, action: &str, target: Option<usize>, allowed: bool) {
    let key = crate::runtime_state::current_key();
    if !enabled(key) { return; }
    let slot = match action { "attack" => 0, "skill" => 1, "skill2" => 2, "ult" => 3, _ => 4 };
    if !sample(Subject::Gate(world.caster, slot, target.is_some()), world.tick, 600) { return; }
    let unit = |id| world.get(id).map(|u| json!({"id": u.id, "team": u.team,
        "pos": [u.pos.x, u.pos.y], "hp": [u.hp, u.max_hp], "champion": u.champion,
        "tower": u.tower, "visible": u.visible, "targetable": u.targetable,
        "cooldowns": u.cooldowns.map(|c| c.map(|c| json!({"unlocked": c.unlocked, "charges": c.charges})))}));
    record(key, json!({"type": "gate", "tick": world.tick, "action": action, "allowed": allowed,
        "caster_id": world.caster, "caster": unit(world.caster), "target_id": target,
        "target": target.and_then(unit)}));
}

pub fn match_tick(sim: &StableSim<'_>) {
    let key = crate::runtime_state::current_key();
    if sim.player_count() == 0 || !enabled(key) { return; }
    if sim.is_end() {
        if sample(Subject::End, sim.tick(), usize::MAX) {
            record(key, json!({"type": "end", "tick": sim.tick()}));
        }
        return;
    }
    if !sample(Subject::World, sim.tick(), if foreground(key) { 120 } else { 240 }) { return; }
    let players: Vec<_> = (0..sim.player_count()).filter_map(|i| sim.player_at(i)).map(|p| {
        let entity = p.champion().map(|e| {
            let (x, y) = e.pos();
            let (hp, max_hp) = e.hp();
            let name = e.name();
            let learned = if name.as_deref() == Some("pokemon_moba_smeargle") {
                use crate::legacy::pokemon_content::ActionSlot;
                Some([ActionSlot::Attack, ActionSlot::Skill, ActionSlot::Skill2, ActionSlot::Ult].map(|slot| {
                    crate::legacy::pokemon_status::smeargle_learned_action_for_player(p.id(), slot)
                        .map(|copied| json!({"source": copied.source_champion.id, "move": copied.action.name_key}))
                }))
            } else { None };
            json!({"id": e.id(), "name": name, "pos": [x, y], "hp": [hp, max_hp], "sketch_slots": learned,
                "alive": e.is_alive(), "cc": e.cc_count(), "speed": e.stat().move_speed})
        });
        json!({"id": p.id(), "team": p.team(), "lane": format!("{:?}", p.lane()), "entity": entity,
            "level": p.level(), "gold": p.gold(), "cs": p.cs(), "items": p.item_keys()})
    }).collect();
    record(key, json!({"type": "world", "tick": sim.tick(), "players": players}));
    if sample(Subject::Strategy, sim.tick(), 600) {
        let strategies: Vec<_> = [0, 1].into_iter().map(|team|
            json!({"team": team, "strategy": sim.strategy_get_json(team, "")
                .and_then(|json| serde_json::from_str::<Value>(&json).ok())})).collect();
        let structures: Vec<_> = (0..sim.entity_count()).filter_map(|i| sim.entity_at(i))
            .filter(|e| matches!(e.kind(), Some(mod_api_stable::EntityKindV1::Tower | mod_api_stable::EntityKindV1::Nexus)))
            .map(|e| json!({"id": e.id(), "team": e.team(), "kind": format!("{:?}", e.kind()),
                "pos": e.pos(), "hp": e.hp()})).collect();
        record(key, json!({"type": "strategy", "tick": sim.tick(), "teams": strategies, "structures": structures}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trace_skips_background_and_unknown_origins() {
        for kind in [0, 1, 5, 99] { assert!(!foreground(Key(kind, 0, 0, 0, 0))); }
        for kind in [2, 3, 4] { assert!(foreground(Key(kind, 0, 0, 0, 0))); }
    }
    #[test]
    fn background_budget_does_not_share_foreground_stream() {
        assert_eq!(stream(Key(1, 5, 0, 0, 1)), 1);
        for kind in [2, 3, 4] { assert_eq!(stream(Key(kind, 5, 0, 0, 1)), 0); }
        assert!(MAX_BYTES[0] > MAX_BYTES[1]);
    }
    #[test]
    fn trace_sampling_is_bounded_and_recovers_on_rewind() {
        let mut last = None;
        assert!(sample_due(&mut last, 0, 30));
        for tick in 0..30 { assert!(!sample_due(&mut last, tick, 30)); }
        assert!(sample_due(&mut last, 30, 30));
        assert!(sample_due(&mut last, 1, 30));
    }
    #[test]
    fn background_trace_selects_only_two_distinct_simulations() {
        let mut selected = vec![];
        let a = Key(1, 1, 0, 0, 10);
        let b = Key(1, 1, 0, 0, 20);
        assert!(!select_background(&mut selected, Key::default()));
        assert!(select_background(&mut selected, a));
        assert!(select_background(&mut selected, a));
        assert!(select_background(&mut selected, b));
        assert!(!select_background(&mut selected, Key(1, 2, 0, 0, 30)));
        assert!(select_background(&mut selected, a));
        assert_eq!(selected, vec![a, b]);
    }
}
