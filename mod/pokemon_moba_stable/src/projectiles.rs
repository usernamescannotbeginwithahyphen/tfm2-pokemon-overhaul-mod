//! Engine movement owns collision; the preserved Pokemon resolver owns each hit.
use mod_api_stable::{CastingTargetV1, CastingTypeV1, InputTargetV1, ProjectileMoveKindV1,
    ProjectileSpawnV1, StableEffectType, StableMod, StableSim};
use crate::legacy::pokemon_content::{PokemonChampion, PokemonMove, PokemonMoveEffect as E,
    POKEMON_CHAMPIONS, resolve_projectile_impact};
use crate::stable_legacy_api::{EntityPos, GameCtx, InputTarget};
use std::cell::Cell;
use std::sync::Mutex;
use crate::runtime_state::SimOnceLock;

struct Bounce { champion: PokemonChampion, action: PokemonMove, caster: usize,
    centers: [EntityPos; 3], index: usize, due: usize }
static BOUNCES: SimOnceLock<Mutex<Vec<Bounce>>> = SimOnceLock::new();

pub fn reset() { BOUNCES.get_or_init(Default::default).lock().unwrap().clear(); }

fn bounce_leg(ctx: &GameCtx, caster: usize, start: EntityPos, end: EntityPos, ticks: usize) {
    let Some(source) = ctx.get_entity(caster) else { return; };
    let distance = ((end.x as f64 - start.x as f64).hypot(end.y as f64 - start.y as f64)) as u64;
    ctx.spawn_visual_projectile("pokemon_moba_stable_octillery_skill2_bubble_flight_travel",
        &ProjectileSpawnV1 { caster_id: caster, team: source.team(), x: start.x, y: start.y,
            target_x: end.x, target_y: end.y, speed: distance.div_ceil(ticks.max(1) as u64).max(1),
            radius: 1, penetrate: true, move_kind: ProjectileMoveKindV1::Linear.code(), ..Default::default() });
}

pub fn update(ctx: &mut GameCtx, seed: u64) {
    let states = std::mem::take(&mut *BOUNCES.get_or_init(Default::default).lock().unwrap());
    let mut pending = Vec::new();
    for mut state in states {
        if ctx.tick() < state.due { pending.push(state); continue; }
        let E::BouncingAreaDamage { base_ad, ad_ratio, base_ap, ap_ratio, radius, .. } = state.action.effect else { continue; };
        let pos = state.centers[state.index];
        let mut impact = state.action;
        impact.effect = E::AreaDamage { base_ad, ad_ratio, base_ap, ap_ratio, radius };
        with_original_action(state.action,|| resolve_projectile_impact(ctx, state.champion, impact, seed, state.caster,
            InputTarget::Pos { x: pos.x, y: pos.y }));
        crate::native_vfx::burst(ctx, if state.index == 2 { "octillery_skill2_bouncy_bubble_path" }
            else { "octillery_skill2_bubble_landing" }, state.caster, InputTarget::Pos { x: pos.x, y: pos.y }, radius);
        state.index += 1;
        if state.index < 3 {
            bounce_leg(ctx, state.caster, pos, state.centers[state.index], 12);
            state.due = ctx.tick().saturating_add(12);
            pending.push(state);
        }
    }
    BOUNCES.get_or_init(Default::default).lock().unwrap().extend(pending);
}

pub fn launch_bounces(ctx: &GameCtx, champion: PokemonChampion, action: PokemonMove,
    caster: usize, input: InputTarget) -> bool {
    let E::BouncingAreaDamage { radius, bounce_distance, .. } = action.effect else { return false; };
    let Some(source) = ctx.get_entity(caster) else { return true; };
    let start = source.pos();
    let aim = match input {
        InputTarget::Pos { x, y } => EntityPos { x, y },
        InputTarget::Target { target_id } => match ctx.get_entity(target_id) { Some(e) => e.pos(), None => return true },
        _ => return true,
    };
    if start==aim {return true;}
    let centers = crate::legacy::pokemon_content::bouncing_area_centers(start, aim, bounce_distance.max(radius.saturating_mul(2)))
        .map(|pos|crate::map_terrain::clamp_to_known_map(pos.x,pos.y).map_or(pos,|(x,y)|EntityPos{x,y}));
    let ticks = 12;
    bounce_leg(ctx, caster, start, centers[0], ticks);
    BOUNCES.get_or_init(Default::default).lock().unwrap().push(Bounce {
        champion, action, caster, centers, index: 0, due: ctx.tick().saturating_add(ticks) });
    true
}

thread_local! { static HIT_INDEX: Cell<Option<usize>> = const { Cell::new(None) }; }
thread_local! { static ORIGINAL_ACTION: Cell<Option<PokemonMove>> = const { Cell::new(None) }; }
pub fn original_action() -> Option<PokemonMove> { ORIGINAL_ACTION.with(Cell::get) }
pub fn with_original_action<T>(action: PokemonMove, f: impl FnOnce() -> T) -> T {
    struct Restore(Option<PokemonMove>);
    impl Drop for Restore { fn drop(&mut self) { ORIGINAL_ACTION.with(|v|v.set(self.0)); } }
    let _scope=Restore(ORIGINAL_ACTION.with(|v|v.replace(Some(action))));
    f()
}
pub fn hit_index() -> Option<usize> { HIT_INDEX.with(Cell::get) }
struct HitScope(Option<usize>);
impl Drop for HitScope { fn drop(&mut self) { HIT_INDEX.with(|v| v.set(self.0)); } }

fn volley_count(effect: E, level: usize) -> usize {
    match effect {
        E::MultiHitDamage { hits, .. } => hits.max(1),
        E::CloysterIcicleCrash { hits_1, hits_2, hits_3, hits_4, .. } =>
            if level >= 10 { hits_4 } else if level >= 7 { hits_3 } else if level >= 4 { hits_2 } else { hits_1 },
        _ => 1,
    }
}
fn indexed_name(champion: PokemonChampion, action: PokemonMove, index: usize, launch: bool) -> String {
    format!("{}:{}:{}", hit_name(champion, action), if launch { "launch" } else { "hit" }, index)
}
struct Missile { champion: PokemonChampion, action: PokemonMove, index: usize, launch: bool }
#[cfg(test)]
pub(crate) fn test_missile(sim: &mut StableSim<'_>, champion: PokemonChampion, action: PokemonMove,
    index: usize, launch: bool, input: InputTargetV1) {
    Missile {champion,action,index,launch}.apply(sim,42,7,input);
}
impl StableEffectType for Missile {
    fn apply(&self, sim: &mut StableSim<'_>, seed: u64, caster: usize, input: InputTargetV1) {
        let mut ctx = GameCtx::from_stable(sim);
        let input = InputTarget::from_stable(input);
        if self.launch {
            if ctx.get_entity(caster).is_some_and(|e| e.is_alive()) {
                launch_one(&ctx, self.champion, self.action, caster, input,
                    &indexed_name(self.champion, self.action, self.index, false), Some(self.index));
            }
        } else {
            let _scope = HitScope(HIT_INDEX.with(|v| v.replace(Some(self.index))));
            resolve_projectile_impact(&mut ctx, self.champion, self.action, seed, caster, input);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Flight { Target, Line { radius: u64, pierce: bool }, Fan { radius: u64, spread: u64 }, Ground }

fn flight(effect: E) -> Option<Flight> {
    match effect {
        E::MagmortarEruption { .. } | E::MarowakBonemerang { .. } => None,
        E::AreaMiasmaSlow { .. } => Some(Flight::Ground),
        E::TripleArcDamage { width, spread, .. } => Some(Flight::Fan {radius:width,spread}),
        E::LinePierceDamage { width, .. } => Some(Flight::Line { radius: width, pierce: true }),
        E::LineFirstDamage { width, .. } | E::LineFirstDamageSelfBuff { width, .. } | E::LineFirstLeechSeed { width, .. }
        | E::CloysterIcicleCrash { width, .. } | E::QuaquavalSpiralShot { width, .. } =>
            Some(Flight::Line { radius: width, pierce: false }),
        _ => Some(Flight::Target),
    }
}

fn hit_name(champion: PokemonChampion, action: PokemonMove) -> String {
    format!("pokemon_moba_stable:impact:{}:{}", champion.id, action.slot.index())
}

struct Hit { champion: PokemonChampion, action: PokemonMove }
impl StableEffectType for Hit {
    fn apply(&self, sim: &mut StableSim<'_>, seed: u64, caster: usize, input: InputTargetV1) {
        let mut ctx = GameCtx::from_stable(sim);
        if matches!(self.action.effect,E::KecleonColorfulWhip{..}) && !ctx.get_entity(caster).is_some_and(|e|e.is_alive()) { return; }
        resolve_projectile_impact(&mut ctx, self.champion, self.action, seed, caster,
            InputTarget::from_stable(input));
    }
}

pub fn register(reg: &mut StableMod) {
    reg.add_native_effect("pokemon_moba_stable:dreepy_hit", SpecialHit::Dreepy);
    for empowered in [false, true] {
        reg.add_native_effect(bone_name(empowered, false), SpecialHit::Bone { empowered });
        reg.add_native_effect(bone_name(empowered, true), SpecialHit::BoneReturn { empowered });
    }
    for champion in POKEMON_CHAMPIONS {
        for action in [Some(champion.attack), Some(champion.skill), Some(champion.skill2), champion.ult].into_iter().flatten() {
            if crate::native_vfx::projectile_name(champion.id, action.slot.index(), 0).is_some()
                || matches!(action.effect, E::StarmiePsybeam { .. } | E::KecleonColorfulWhip { .. }) {
                reg.add_native_effect(hit_name(champion, action), Hit { champion, action });
                for index in 0..volley_count(action.effect, 10) {
                    for launch in [false, true] {
                        reg.add_native_effect(indexed_name(champion, action, index, launch),
                            Missile { champion, action, index, launch });
                    }
                }
            }
        }
    }
}

pub fn queue_release(ctx: &GameCtx, champion: PokemonChampion, action: PokemonMove,
    caster: usize, input: InputTarget, ticks: usize) -> bool {
    ctx.queue_native_effect(&hit_name(champion, action), caster, input, ticks.max(1))
}

enum SpecialHit { Dreepy, Bone { empowered: bool }, BoneReturn { empowered: bool } }
impl StableEffectType for SpecialHit {
    fn apply(&self, sim: &mut StableSim<'_>, seed: u64, caster: usize, input: InputTargetV1) {
        let mut ctx = GameCtx::from_stable(sim);
        let input = InputTarget::from_stable(input);
        match *self {
            Self::Dreepy => if let InputTarget::Target { target_id } = input {
                crate::legacy::pokemon_content::resolve_dreepy_hit(&mut ctx, caster, target_id);
            },
            Self::Bone { empowered } => if let InputTarget::Target { target_id } = input {
                crate::legacy::pokemon_content::resolve_bonemerang_hit(&mut ctx, caster, target_id, empowered, seed);
            },
            Self::BoneReturn { empowered } => if let InputTarget::Pos { x, y } = input {
                if let Some(source) = ctx.get_entity(caster) {
                    let end = source.pos();
                    launch_bone_leg(&ctx, caster, EntityPos { x, y }, end, empowered);
                }
            },
        }
    }
}

fn bone_name(empowered: bool, returning: bool) -> String {
    format!("pokemon_moba_stable:bone_{}_{}", empowered as u8, returning as u8)
}

pub fn launch_named(ctx: &GameCtx, name: &str, hit: &str, caster: usize, target: usize) -> bool {
    let Some(source) = ctx.get_entity(caster) else { return false; };
    let pos = source.pos();
    ctx.spawn_projectile(name, hit, &ProjectileSpawnV1 { caster_id: caster, team: source.team(),
        x: pos.x, y: pos.y, target_id: target, radius: 1, ..Default::default() })
}

pub fn launch_eruption(ctx: &GameCtx, caster: usize, target: usize) -> bool {
    let Some(champion) = POKEMON_CHAMPIONS.iter().find(|champion| champion.id == "pokemon_moba_magmortar") else { return false; };
    let Some(action) = champion.ult else { return false; };
    let Some(name) = crate::native_vfx::projectile_name(champion.id, action.slot.index(), ctx.tick()) else { return false; };
    launch_named(ctx, name, &hit_name(*champion, action), caster, target)
}

fn launch_bone_leg(ctx: &GameCtx, caster: usize, origin: EntityPos, end: EntityPos, empowered: bool) -> bool {
    if origin == end { return false; }
    let Some(source) = ctx.get_entity(caster) else { return false; };
    let spec = ProjectileSpawnV1 { caster_id: caster, team: source.team(), x: origin.x, y: origin.y,
        radius: 7200, speed: if empowered { 7500 } else { 5000 }, target_x: end.x, target_y: end.y,
        move_kind: ProjectileMoveKindV1::Linear.code(), penetrate: true,
        attack_type: mod_api_stable::AttackTypeV1::BaseAttack.code(), ..Default::default() };
    ctx.spawn_projectile("pokemon_moba_stable_marowak_basicattack_bonemerang_projectile", &bone_name(empowered, false), &spec)
}

pub fn launch_bonemerang(ctx: &GameCtx, caster: usize, origin: EntityPos, end: EntityPos, empowered: bool) -> bool {
    if !launch_bone_leg(ctx, caster, origin, end, empowered) { return false; }
    let dx = end.x as i128 - origin.x as i128;
    let dy = end.y as i128 - origin.y as i128;
    let ticks = ((dx * dx + dy * dy).isqrt() as u64).div_ceil(if empowered { 7500 } else { 5000 }).max(1);
    ctx.queue_native_effect(&bone_name(empowered, true), caster, InputTarget::Pos { x: end.x, y: end.y }, ticks as usize);
    true
}

fn endpoint(origin: EntityPos, aim: EntityPos, range: u64) -> Option<EntityPos> {
    let dx = aim.x as i128 - origin.x as i128;
    let dy = aim.y as i128 - origin.y as i128;
    let distance = (dx * dx + dy * dy).isqrt();
    if distance == 0 { return None; }
    Some(EntityPos { x: (origin.x as i128 + dx * range as i128 / distance).max(0) as u64,
        y: (origin.y as i128 + dy * range as i128 / distance).max(0) as u64 })
}

pub fn launch(ctx: &GameCtx, champion: PokemonChampion, action: PokemonMove, caster: usize, input: InputTarget) -> bool {
    if crate::native_vfx::projectile_name(champion.id, action.slot.index(), ctx.tick()).is_none() {
        return false;
    }
    let count = ctx.get_entity(caster).map_or(1, |e| volley_count(action.effect, e.level()));
    if count <= 1 { return launch_one(ctx, champion, action, caster, input, &hit_name(champion, action), None); }
    // Each missile owns one collision and one resolver hit. Cast costs stay at launch.
    let interval = (action.duration.saturating_sub(action.start_timing) / count).clamp(3, 8);
    if !launch_one(ctx, champion, action, caster, input, &indexed_name(champion, action, 0, false), Some(0)) {
        return false;
    }
    for index in 1..count {
        ctx.queue_native_effect(&indexed_name(champion, action, index, true), caster, input, interval * index);
    }
    true
}

fn launch_one(ctx: &GameCtx, champion: PokemonChampion, action: PokemonMove, caster: usize,
    input: InputTarget, hit: &str, volley_index: Option<usize>) -> bool {
    let Some(name) = crate::native_vfx::projectile_name(champion.id, action.slot.index(), volley_index.unwrap_or(ctx.tick())) else { return false; };
    let Some(flight) = flight(action.effect) else { return false; };
    let Some(source) = ctx.get_entity(caster) else { return false; };
    let origin = source.pos();
    let aim = match input {
        InputTarget::Target { target_id } => ctx.get_entity(target_id).map(|target| target.pos()),
        InputTarget::Pos { x, y } => Some(EntityPos { x, y }),
        InputTarget::Dir { x, y } => Some(EntityPos {
            x: (origin.x as i128 + x as i128).max(0) as u64,
            y: (origin.y as i128 + y as i128).max(0) as u64,
        }),
        InputTarget::None => None,
    };
    let Some(aim) = aim else { return false; };
    let mut spec = ProjectileSpawnV1 { caster_id: caster, team: source.team(), x: origin.x, y: origin.y,
        radius: 1, speed: 5000, attack_type: action.attack_type.to_stable().code(),
        casting_type: CastingTypeV1::Targeting.code(), ..Default::default() };
    match flight {
        Flight::Fan { radius,spread } => {
            let Some(end)=endpoint(origin,aim,action.range) else{return false;};
            let ends=crate::legacy::pokemon_content::triple_arc_endpoints(origin,end,spread);
            spec.move_kind=ProjectileMoveKindV1::Linear.code();
            spec.speed=10000;spec.radius=radius.max(1);spec.penetrate=true;
            for end in ends {
                spec.target_x=end.x;spec.target_y=end.y;
                ctx.spawn_projectile(name,hit,&spec);
            }
            true
        }
        Flight::Target => {
            let InputTarget::Target { target_id } = input else { return false; };
            spec.target_id = target_id;
            if ctx.get_entity(target_id).map_or(false, |target| target.team() == source.team()) {
                spec.casting_target = CastingTargetV1::Ally.code();
            }
            ctx.spawn_projectile(name, hit, &spec)
        }
        Flight::Line { radius, pierce } => {
            let Some(end) = endpoint(origin, aim, action.range) else { return false; };
            spec.move_kind = ProjectileMoveKindV1::Linear.code();
            spec.target_x = end.x; spec.target_y = end.y;
            spec.radius = radius.max(1); spec.penetrate = pierce;
            ctx.spawn_projectile(name, hit, &spec)
        }
        Flight::Ground => {
            // Ground lobs must land even when no entity occupies the destination.
            let dx = aim.x as i128 - origin.x as i128;
            let dy = aim.y as i128 - origin.y as i128;
            let distance = (dx * dx + dy * dy).isqrt() as u64;
            let ticks = distance.div_ceil(spec.speed).max(1) as usize;
            if !ctx.queue_native_effect(hit, caster, InputTarget::Pos { x: aim.x, y: aim.y }, ticks) { return false; }
            spec.move_kind = ProjectileMoveKindV1::Linear.code();
            spec.target_x = aim.x; spec.target_y = aim.y; spec.penetrate = true;
            ctx.spawn_visual_projectile(name, &spec);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directions_use_full_range_and_reject_zero_length() {
        let origin = EntityPos { x: 100, y: 100 };
        assert_eq!(endpoint(origin, EntityPos { x: 103, y: 104 }, 50), Some(EntityPos { x: 130, y: 140 }));
        assert_eq!(endpoint(origin, origin, 50), None);
    }
}
