//! Native frame events only: no companion gameplay, private renderer, or damage effects.
use crate::legacy::pokemon_content::{PokemonMove, PokemonMoveEffect as E};
use crate::runtime_state::SimOnceLock;
use crate::stable_legacy_api::{EntityPos, GameCtx, InputTarget};
use mod_api_stable::{InputTargetV1, StableEffectType, StableMod, StableSim};
use std::{collections::BTreeMap, sync::Mutex};

const FIELD_LEASE_TICKS: usize = 30;
type FieldKey = (String, usize, u64, u64, u64, u64);
static FIELD_LEASES: SimOnceLock<Mutex<BTreeMap<FieldKey, usize>>> = SimOnceLock::new();
static ATTACHED_LEASES: SimOnceLock<Mutex<BTreeMap<(usize, &'static str), usize>>> =
    SimOnceLock::new();
static CHIT_LEASES: SimOnceLock<Mutex<BTreeMap<usize, (usize, usize)>>> = SimOnceLock::new();

pub const VISUAL_HIT: &str = "pokemon_moba_stable:visual_projectile_hit";
struct VisualHit;
impl StableEffectType for VisualHit {
    fn apply(&self, _: &mut StableSim<'_>, _: u64, _: usize, _: InputTargetV1) {}
}
pub fn register(decl: &mut StableMod) {
    decl.add_native_effect(VISUAL_HIT, VisualHit);
}

struct Binding {
    champion: &'static str,
    slot: i8,
    stem: &'static str,
    name: &'static str,
    ticks: u64,
    projectile: bool,
    direction: Option<&'static str>,
}
include!("vfx_catalog.rs");

fn binding(stem: &str) -> Option<&'static Binding> {
    BINDINGS.iter().find(|b| b.stem == stem)
}

pub fn projectile_name(champion: &str, slot: usize, tick: usize) -> Option<&'static str> {
    BINDINGS
        .iter()
        .find(|b| {
            b.projectile
                && b.champion == champion
                && b.slot == slot as i8
                && !(b.stem == "blastoise_basic_projectile_2" && tick % 2 == 0)
                && !(b.stem == "blastoise_basic_projectile_1" && tick % 2 != 0)
        })
        .map(|b| b.name)
}

pub fn burst(ctx: &GameCtx, stem: &str, caster: usize, target: InputTarget, radius: u64) {
    if let (Some(b), Some(pos)) = (binding(stem), world_position(ctx, caster, target)) {
        ctx.play_view_effect(b.name, caster, at(pos), radius, radius, b.ticks);
    }
}

pub fn line_pulse(ctx: &GameCtx, stem: &str, caster: usize, start: EntityPos, end: EntityPos, half_width: u64) {
    if let Some(b) = binding(stem) {
        let name = oriented_name(b, start, end);
        ctx.play_view_effect(&name, caster, at(midpoint(start, end)), distance(start, end), half_width, b.ticks);
    }
}

pub fn toxic_chain(ctx: &GameCtx, caster: usize, target: usize) {
    let (Some(source), Some(victim)) = (ctx.get_entity(caster), ctx.get_entity(target)) else { return; };
    let (start, end) = (source.pos(), victim.pos());
    // This is an instantaneous poison/execute cast, not a maintained poison-duration link.
    if start != end {
        if let Some(b) = binding("dragalge_ult_toxic_chain_tether") {
            let name = oriented_name(b, start, end);
            // Animation has no documented stretch control. Place links explicitly instead.
            let links = distance(start, end).div_ceil(8000).clamp(1, 32);
            for index in 0..links {
                let numerator = (index * 2 + 1) as i128;
                let denominator = (links * 2) as i128;
                let interpolate = |a: u64, z: u64| (a as i128
                    + (z as i128 - a as i128) * numerator / denominator) as u64;
                let pos = EntityPos { x: interpolate(start.x, end.x), y: interpolate(start.y, end.y) };
                ctx.play_view_effect(&name, caster, at(pos), 0, 0, b.ticks);
            }
        }
    }
    burst(ctx, "dragalge_ult_toxic_chain_impact", caster, at(end), 8000);
}

fn direction_index(start: EntityPos, end: EntityPos) -> usize {
    let angle = (end.y as f64-start.y as f64).atan2(end.x as f64-start.x as f64);
    ((angle / std::f64::consts::TAU * 16.0).round() as i32).rem_euclid(16) as usize
}
fn oriented_name(b: &Binding, start: EntityPos, end: EntityPos) -> String {
    b.direction.map_or_else(|| b.name.to_owned(), |prefix| format!("{prefix}_{}",direction_index(start,end)))
}
#[cfg(test)]
pub(crate) fn test_direction_name(stem: &str, direction: usize) -> String {
    let b=binding(stem).unwrap();
    format!("{}_{}",b.direction.unwrap(),direction)
}

// The source sheet contains six fixed states, not a five-to-zero animation.
pub fn wonder_guard_chits(ctx: &GameCtx, target: usize, count: Option<usize>) {
    let count = count.map(|n| n.min(5));
    let tick = ctx.tick();
    let mut leases = CHIT_LEASES.get_or_init(Default::default).lock().unwrap();
    let old = leases.get(&target).copied();
    if old.is_some_and(|(n, until)| count == Some(n) && until > tick) {
        return;
    }
    if let Some(n) = count { leases.insert(target, (n, tick.saturating_add(15))); }
    else { leases.remove(&target); }
    drop(leases);
    if let Some((n, _)) = old {
        if count != Some(n) { ctx.entity_remove_buff(target, &format!("pm_wonder_guard_{n}")); }
    }
    if let Some(n) = count { ctx.visual_buff(target, &format!("pm_wonder_guard_{n}"), 30); }
}

fn at(pos: EntityPos) -> InputTarget {
    InputTarget::Pos { x: pos.x, y: pos.y }
}

// Snapshot at the gameplay event. Only explicit visual buffs may follow an entity.
fn world_position(ctx: &GameCtx, caster: usize, input: InputTarget) -> Option<EntityPos> {
    match input {
        InputTarget::Pos { x, y } => Some(EntityPos { x, y }),
        InputTarget::Target { target_id } => ctx.get_entity(target_id).map(|e| e.pos()),
        InputTarget::None => ctx.get_entity(caster).map(|e| e.pos()),
        InputTarget::Dir { x, y } => ctx.get_entity(caster).map(|e| EntityPos {
            x: (e.pos().x as i128 + x as i128).clamp(0, u64::MAX as i128) as u64,
            y: (e.pos().y as i128 + y as i128).clamp(0, u64::MAX as i128) as u64,
        }),
    }
}

pub fn traveling_wave(
    ctx: &GameCtx,
    champion: &str,
    action: PokemonMove,
    caster: usize,
    origin: EntityPos,
    end: EntityPos,
    speed: u64,
) {
    let Some(source) = ctx.get_entity(caster) else {
        return;
    };
    for b in BINDINGS.iter().filter(|b| {
        b.champion == champion && b.slot == action.slot.index() as i8 && !state_driven(b.stem)
    }) {
        ctx.spawn_visual_projectile(
            &format!("{}_travel", b.name),
            &mod_api_stable::ProjectileSpawnV1 {
                caster_id: caster,
                team: source.team(),
                x: origin.x,
                y: origin.y,
                target_x: end.x,
                target_y: end.y,
                speed,
                radius: 1,
                penetrate: true,
                move_kind: mod_api_stable::ProjectileMoveKindV1::Linear.code(),
                ..Default::default()
            },
        );
    }
}

// Refresh from authoritative state. New fields render immediately; consumed or
// expired fields leave no more than a thirty-tick visual tail. The public API
// cannot cancel an emitted animation; do not shorten its playback to one frame.
pub fn field(ctx: &GameCtx, stem: &str, caster: usize, pos: EntityPos, radius: u64) {
    field_geometry(ctx, stem, caster, pos, radius, radius, None);
}

fn field_geometry(
    ctx: &GameCtx,
    stem: &str,
    caster: usize,
    pos: EntityPos,
    range: u64,
    radius: u64,
    direction: Option<usize>,
) {
    if let Some(b) = binding(stem) {
        let name = match (b.direction, direction) {
            (Some(prefix), Some(d)) => format!("{prefix}_{d}"),
            _ => b.name.to_owned(),
        };
        let tick = ctx.tick();
        {
            let mut leases = FIELD_LEASES.get_or_init(Default::default).lock().unwrap();
            leases.retain(|_, until| *until > tick);
            let key = (name.clone(), caster, pos.x, pos.y, range, radius);
            if leases.contains_key(&key) {
                return;
            }
            leases.insert(key, tick.saturating_add(FIELD_LEASE_TICKS));
        }
        ctx.play_view_effect(
            &format!("{}_loop", name),
            caster,
            InputTarget::Pos { x: pos.x, y: pos.y },
            range,
            radius,
            FIELD_LEASE_TICKS as u64,
        );
    }
}

pub fn rectangle(
    ctx: &GameCtx,
    stem: &str,
    caster: usize,
    start: EntityPos,
    target: EntityPos,
    length: u64,
    width: u64,
) {
    let center = rectangle_center(start, target, length);
    // Native animation bindings accept a position/range/radius but expose no
    // rotation parameter. Keep the center/extents correct without moving the caster.
    field_geometry(ctx, stem, caster, center, length, width / 2, Some(direction_index(start,target)));
}

pub fn segment(
    ctx: &GameCtx,
    stem: &str,
    caster: usize,
    start: EntityPos,
    end: EntityPos,
    half_width: u64,
) {
    field_geometry(
        ctx,
        stem,
        caster,
        midpoint(start, end),
        distance(start, end),
        half_width,
        Some(direction_index(start,end)),
    );
}

fn midpoint(a: EntityPos, b: EntityPos) -> EntityPos {
    EntityPos {
        x: (a.x as u128 + b.x as u128).div_euclid(2) as u64,
        y: (a.y as u128 + b.y as u128).div_euclid(2) as u64,
    }
}

fn distance(a: EntityPos, b: EntityPos) -> u64 {
    ((a.x as f64 - b.x as f64).hypot(a.y as f64 - b.y as f64)) as u64
}

fn rectangle_center(start: EntityPos, target: EntityPos, length: u64) -> EntityPos {
    let dx = target.x as i128 - start.x as i128;
    let dy = target.y as i128 - start.y as i128;
    let distance = (dx * dx + dy * dy).isqrt();
    if distance == 0 {
        return start;
    }
    EntityPos {
        x: (start.x as i128 + dx * length as i128 / (2 * distance)).max(0) as u64,
        y: (start.y as i128 + dy * length as i128 / (2 * distance)).max(0) as u64,
    }
}

pub fn attached(ctx: &GameCtx, stem: &str, target: usize) {
    if let Some(b) = binding(stem) {
        let tick = ctx.tick();
        {
            let mut leases = ATTACHED_LEASES
                .get_or_init(Default::default)
                .lock()
                .unwrap();
            leases.retain(|_, until| *until > tick);
            if leases.contains_key(&(target, b.name)) {
                return;
            }
            leases.insert((target, b.name), tick.saturating_add(15));
        }
        ctx.visual_buff(target, b.name, 30);
    }
}

fn dash_streak(stem: &str) -> bool {
    matches!(
        stem,
        "pikachu_skill1_volt_tackle_dash_trail"
            | "dedenne_skill1_play_rough_dash"
            | "drednaw_ult_razor_shell_trail"
            | "ribombee_skill2_u_turn_trail"
            | "greninja_skill1_quick_attack_streak"
            | "dragapult_skill2_diving_swipe_ghost_dash"
    )
}

pub fn dash_start(ctx: &GameCtx, champion: &str, action: PokemonMove, caster: usize, ticks: usize) {
    for b in BINDINGS.iter().filter(|b| {
        b.champion == champion && b.slot == action.slot.index() as i8 && dash_streak(b.stem)
    }) {
        ctx.visual_buff(caster, b.name, ticks.max(1));
    }
}

pub fn dash_end(ctx: &GameCtx, champion: &str, action: PokemonMove, caster: usize) {
    for b in BINDINGS.iter().filter(|b| {
        b.champion == champion && b.slot == action.slot.index() as i8 && dash_streak(b.stem)
    }) {
        ctx.entity_remove_buff(caster, b.name);
    }
}

// These are driven by their actual persistent state or follow-up event, not every cast.
fn state_driven(stem: &str) -> bool {
    matches!(
        stem,
        "kricketune_skill1_sticky_web_deployable"
            | "dragalge_ult_toxic_chain_tether"
            | "dragalge_ult_toxic_chain_impact"
            | "blissey_skill1_heal_pulse_aura"
            | "ampharos_ult_gigavolt_zone"
            | "coalossal_ult_magma_storm_line"
            | "coalossal_ult_magma_segment"
            | "coalossal_ult_magma_burst"
            | "swanna_ult_sky_circus_aoe"
            | "swanna_ult_sky_circus_slash"
            | "swanna_ult_sky_circus_landing"
            | "decidueye_basicattack_leafage_triple_arc"
            | "turtonator_skill1_shell_trap_burst"
            | "octillery_skill2_bouncy_bubble_path"
            | "octillery_skill2_bubble_landing"
            | "octillery_skill2_bubble_flight"
            | "beeheeyem_skill2_light_screen"
            | "oranguru_skill1_nasty_plot_mark"
            | "oranguru_skill2_stored_power_burst"
            | "octillery_skill1_lock_on_reticle"
            | "banette_ult_phantom_splash"
            | "starmie_skill2_heart_swap_shockwave"
            | "starmie_ult_psybeam_line"
            | "gholdengo_ult_make_it_rain_aoe"
            | "zeraora_skill2_zing_zap_counter"
            | "zeraora_ult_wild_charge_bolts"
            | "eevee_skill2_wish_heal_ring"
            | "ampharos_ult_gigavolt_strike_zone"
            | "xatu_ult_super_psy_beam"
            | "yanmega_ult_giga_drain_tether"
            | "coalossal_skill2_smokescreen_field"
            | "missingno_ult_trick_room_field"
            | "magmortar_skill2_heat_wave_ring"
            | "rillaboom_ult_grassy_surge_wave"
            | "blaziken_ult_flame_trail"
            | "ceruledge_skill1_flame_trail"
            | "heliolisk_skill2_shed_tail_decoy"
            | "goodra_ult_life_dew_pickup"
            | "goodra_ult_life_dew_heal_burst"
            | "arboliva_skill2_fiery_terrain"
            | "dedenne_ult_spark_grid_nodes"
            | "dedenne_ult_spark_grid_chain"
            | "dedenne_ult_spark_grid_consume_heal"
            | "dedenne_ult_spark_grid_overload"
            | "missingno_basicattack_stray_glitch_line"
            | "missingno_skill2_chain_spark_arc"
            | "blastoise_ult_cannon"
            | "wishiwashi_ult_massive_catch_water_burst"
            | "electrode_ult_self_destruct_explosion_aoe"
            | "magmortar_ult_eruption_cannonball"
            | "turtonator_skill1_shell_trap_field"
            | "turtonator_skill2_sunny_day_field"
            | "goodra_skill1_rain_dance_field"
            | "goodra_skill2_dragon_cheer_aura"
            | "comfey_skill2_petal_dance_aura"
            | "gyarados_basic_whirlpool_field"
            | "glaceon_skill1_snowscape_field"
            | "glaceon_ult_blizzard_field"
            | "sylveon_skill1_misty_terrain_field"
            | "mantine_skill1_brine_water_field"
            | "frosmoth_skill2_sleep_powder_field"
            | "shiftry_ult_fan_tornado_field"
            | "garganacl_ult_blessed_salt_spire_ring"
            | "zeraora_skill1_thunder_cage_field"
            | "thievul_ult_stakeout_field"
    ) || stem.contains("_passive_")
        || matches!(
            stem,
            "vaporeon_ult_aqua_ring_aura"
                | "swanna_skill2_feathery_cyclone_aura"
                | "marowak_ult_bone_windmill_aura"
                | "armarouge_skill1_fire_aura"
                | "weezing_skill2_body_odor_aura"
                | "weezing_ult_sludge_whirlpool_aura"
                | "ribombee_ult_alluring_voice_aura"
                | "ludicolo_skill2_grass_whistle_aura"
                | "ursaluna_skill2_earthquake_aura"
                | "missingno_skill2_glitch_storm_aura"
                | "ampharos_skill2_searchlight_tail_aura"
                | "ursaluna_ult_blood_moon_transformation"
                | "kingdra_skill2_dragon_dance_aura"
                | "rillaboom_skill1_drum_beating_aura"
                | "rillaboom_skill2_drum_roll_aura"
        )
}

pub fn on_cast(
    ctx: &GameCtx,
    champion: &str,
    action: PokemonMove,
    caster: usize,
    input: InputTarget,
) {
    let Some(entity) = ctx.get_entity(caster) else {
        return;
    };
    let origin = entity.pos();
    drop(entity);
    let placements = placements(ctx, action, caster, origin, input);
    for b in BINDINGS
        .iter()
        .filter(|b| !b.projectile && b.champion == champion && b.slot == action.slot.index() as i8)
    {
        if matches!(b.stem, "blastoise_ult_cannon" | "delibird_ult_drill_peck_flourish") {
            // Reference-only artwork: these effects are baked into the body sheet.
            continue;
        }
        if b.stem == "wishiwashi_ult_schooling_channel_aura" {
            if let E::WishiwashiMassiveCatch { channel_ticks, .. } = action.effect {
                ctx.visual_buff(caster, b.name, channel_ticks.max(1));
            }
            continue;
        }
        if state_driven(b.stem) || dash_streak(b.stem) {
            continue;
        }
        if b.direction.is_some() {
            let aim = world_position(ctx,caster,input).unwrap_or(origin);
            let name = oriented_name(b,origin,aim);
            let cone = matches!(action.effect,E::ConeDamage {..}|E::ConeDamageFreeze {..}|E::ConeDamageBlind {..}
                |E::ConeDamageConfusion {..}|E::ConeTauntThenConfuse {..}|E::ConeForceAwaySlow {..});
            if cone && action.can_use_with_move {
                // One attachment for this cast, not repeated first-frame restarts.
                ctx.visual_buff(caster,&name,action.duration.saturating_sub(action.start_timing).max(1));
            } else if cone || matches!(action.effect,E::KecleonColorfulWhip {..}) {
                ctx.play_view_effect(&name,caster,at(origin),action.range,geometry(action).1,b.ticks);
            } else {
                for &(pos,range,radius) in &placements {
                    ctx.play_view_effect(&name,caster,at(pos),range,radius,b.ticks);
                }
            }
            continue;
        }
        if let Some(ticks) = self_buff_duration(action.effect) {
            ctx.visual_buff(caster, b.name, ticks.max(1));
        } else {
            for &(pos, range, radius) in &placements {
                ctx.play_view_effect(b.name, caster, at(pos), range, radius, b.ticks);
            }
        }
    }
}

fn placements(
    ctx: &GameCtx,
    action: PokemonMove,
    caster: usize,
    origin: EntityPos,
    input: InputTarget,
) -> Vec<(EntityPos, u64, u64)> {
    let (range, radius, self_centered) = geometry(action);
    if self_centered {
        return vec![(origin, range, radius)];
    }
    let Some(aim) = world_position(ctx, caster, input) else {
        return Vec::new();
    };
    match action.effect {
        E::TripleAreaDamage { radius, spread, .. }
        | E::TripleAreaDamagePoison { radius, spread, .. } => {
            crate::legacy::pokemon_content::triangle_area_centers(origin, aim, spread)
                .into_iter()
                .map(|p| (p, radius, radius))
                .collect()
        }
        E::BouncingAreaDamage {
            radius,
            bounce_distance,
            ..
        } => crate::legacy::pokemon_content::bouncing_area_centers(origin, aim, bounce_distance)
            .into_iter()
            .map(|p| (p, radius, radius))
            .collect(),
        E::ConeDamage {
            length,
            width_at_end,
            ..
        }
        | E::ConeDamageFreeze {
            length,
            width_at_end,
            ..
        }
        | E::ConeDamageBlind {
            length,
            width_at_end,
            ..
        }
        | E::ConeDamageConfusion {
            length,
            width_at_end,
            ..
        }
        | E::ConeTauntThenConfuse {
            length,
            width_at_end,
            ..
        }
        | E::ConeForceAwaySlow {
            length,
            width_at_end,
            ..
        } => vec![(
            rectangle_center(origin, aim, length),
            length,
            width_at_end / 2,
        )],
        E::MeleeArcDamage {
            arc_range,
            arc_width,
            ..
        } => vec![(
            rectangle_center(origin, aim, arc_range),
            arc_range,
            arc_width / 2,
        )],
        E::DashEndConeDamage {
            cone_length,
            cone_width,
            ..
        } => {
            let start = crate::motion::origin(caster).unwrap_or(origin);
            let forward = EntityPos {
                x: (2 * origin.x as i128 - start.x as i128).clamp(0, u64::MAX as i128) as u64,
                y: (2 * origin.y as i128 - start.y as i128).clamp(0, u64::MAX as i128) as u64,
            };
            vec![(
                rectangle_center(origin, forward, cone_length),
                cone_length,
                cone_width / 2,
            )]
        }
        E::StarmiePsybeam {
            travel_range,
            width,
            ..
        } => vec![(
            rectangle_center(origin, aim, travel_range),
            travel_range,
            width,
        )],
        E::LinePierceDamage { width, .. }
        | E::LineFirstDamage { width, .. }
        | E::LineFirstDamageSelfBuff { width, .. }
        | E::LinePierceDamageDrain { width, .. }
        | E::LinePierceDamageStun { width, .. }
        | E::LineDamageAndParalysis { width, .. }
        | E::LinePierceDamageChanceBurn { width, .. }
        | E::LinePierceDamageChancePoison { width, .. }
        | E::LinePierceDamageSelfDefenseDebuff { width, .. }
        | E::LinePierceDamageChanceFreeze { width, .. }
        | E::LinePierceDamageSlowApToAd { width, .. }
        | E::LinePierceDamageMiasmaCoil { width, .. }
        | E::LinePierceDamageSelfRetreat { width, .. }
        | E::LineExecute { width }
        | E::TriAttack { width, .. }
        | E::YanmegaBuzzingBoost { width, .. }
        | E::LineFirstChargedDamage { width, .. }
        | E::LineFirstLeechSeed { width, .. }
        | E::LineFirstBindBoth { width, .. }
        | E::LinePoisonSlow { width, .. } => {
            vec![(midpoint(origin, aim), distance(origin, aim), width)]
        }
        _ => vec![(aim, range, radius)],
    }
}

fn self_buff_duration(effect: E) -> Option<usize> {
    match effect {
        E::SelfOffenseBuff { ticks, .. }
        | E::SelfOffenseCritBuff { ticks, .. }
        | E::SelfAllStatsBuff { ticks, .. }
        | E::SelfSpeedCritBuff { ticks, .. }
        | E::SelfAttackDefenseBuff { ticks, .. }
        | E::SelfAttackDualDefenseBuff { ticks, .. }
        | E::SelfCritAntiHealBuff { ticks, .. }
        | E::SelfHpCostOffenseSpeedBuff { ticks, .. }
        | E::SelfCleanseOffenseBuff { ticks, .. } => Some(ticks),
        _ => None,
    }
}

fn geometry(action: PokemonMove) -> (u64, u64, bool) {
    let range = action.range.max(1);
    match action.effect {
        E::SelfAreaDamage { radius, .. }
        | E::SelfAreaDamageRetreatLine { radius, .. }
        | E::SelfBuffAreaDamage { radius, .. }
        | E::SelfBuffAreaDamageKnockbackChanceBurn { radius, .. }
        | E::SelfAreaDamageFreezeAll { radius, .. }
        | E::SelfAreaDamageBurn { radius, .. }
        | E::SelfAreaDamageBurnConfusion { radius, .. }
        | E::SelfAreaMiasmaDetonate { radius, .. }
        | E::SelfAreaDamageParalysis { radius, .. }
        | E::SelfAreaDamageKnockbackChancePoison { radius, .. }
        | E::SelfAreaDamageThrow { radius, .. }
        | E::AreaDamageDefenseDebuff { radius, .. }
        | E::AreaDamageAttackDebuff { radius, .. }
        | E::AreaAttackDefenseDebuff { radius, .. }
        | E::DazzlingGleam { radius, .. }
        | E::AmpharosFlash { radius, .. }
        | E::KecleonChromashift { radius, .. }
        | E::ArcanineFlamesOfRage { radius, .. }
        | E::DedenneParabolicDischarge { radius, .. } => (radius, radius, true),
        E::AreaDamage { radius, .. }
        | E::AreaDamageBurn { radius, .. }
        | E::AreaDamageSelfSlow { radius, .. }
        | E::AreaMiasmaSlow { radius, .. }
        | E::AreaSoak { radius, .. }
        | E::SwannaSkyCircus { radius, .. } => (range, radius, false),
        E::ConeDamage {
            length,
            width_at_end,
            ..
        }
        | E::ConeDamageFreeze {
            length,
            width_at_end,
            ..
        }
        | E::ConeDamageBlind {
            length,
            width_at_end,
            ..
        }
        | E::ConeDamageConfusion {
            length,
            width_at_end,
            ..
        }
        | E::ConeTauntThenConfuse {
            length,
            width_at_end,
            ..
        }
        | E::ConeForceAwaySlow {
            length,
            width_at_end,
            ..
        } => (length, width_at_end / 2, false),
        E::LinePierceDamage { width, .. }
        | E::LineFirstDamage { width, .. }
        | E::LineFirstDamageSelfBuff { width, .. }
        | E::LinePierceDamageDrain { width, .. }
        | E::LinePierceDamageStun { width, .. }
        | E::LineDamageAndParalysis { width, .. } => (range, width / 2, false),
        _ => (
            range,
            10000,
            matches!(action.target, crate::stable_legacy_api::CastingTarget::None),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn export_roster_visual_inventory() {
        use crate::legacy::pokemon_content::POKEMON_CHAMPIONS;
        let rows: Vec<_> = POKEMON_CHAMPIONS.iter().map(|champion| {
            let actions: Vec<_> = [Some(champion.attack), Some(champion.skill), Some(champion.skill2), champion.ult]
                .into_iter().flatten().map(|action| {
                    let assets: Vec<_> = BINDINGS.iter().filter(|b| b.champion == champion.id && b.slot == action.slot.index() as i8)
                        .map(|b| serde_json::json!({"stem":b.stem,"ticks":b.ticks,"projectile":b.projectile,
                            "state_driven":state_driven(b.stem),"dash_streak":dash_streak(b.stem)})).collect();
                    serde_json::json!({"slot":action.slot.index(),"effect":format!("{:?}",action.effect),
                        "duration":action.duration,"start":action.start_timing,"cooldown":action.cooldown,
                        "range":action.range,"moving":action.can_use_with_move,"target":format!("{:?}",action.target),
                        "casting":format!("{:?}",action.casting),"assets":assets})
                }).collect();
            let passives: Vec<_> = BINDINGS.iter().filter(|b| b.champion == champion.id && b.slot == -1)
                .map(|b| b.stem).collect();
            serde_json::json!({"id":champion.id,"name":champion.display_name,"actions":actions,"passive_assets":passives})
        }).collect();
        assert_eq!(rows.len(),103);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scratch/vfx-runtime-inventory.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path,serde_json::to_string_pretty(&rows).unwrap()).unwrap();
    }
    #[test]
    fn projectile_action_inventory() {
        for b in BINDINGS.iter().filter(|b| b.projectile) {
            let champion = crate::legacy::pokemon_content::POKEMON_CHAMPIONS
                .iter()
                .find(|champion| champion.id == b.champion)
                .unwrap();
            let action = [
                Some(champion.attack),
                Some(champion.skill),
                Some(champion.skill2),
                champion.ult,
            ]
            .get(b.slot as usize)
            .copied()
            .flatten();
            if b.slot >= 0 {
                assert!(action.is_some(), "Missing action for {}", b.stem);
                assert!(projectile_name(champion.id, b.slot as usize, 0).is_some());
            } else {
                assert_eq!(b.stem, "dragapult_passive_dragon_launcher_dreepy");
            }
        }
    }
    #[test]
    fn catalog_names_are_unique_and_roster_owned() {
        let mut names = std::collections::HashSet::new();
        for b in BINDINGS {
            assert!(names.insert(b.name));
            assert!(b.name.ends_with(b.stem));
            assert!(
                crate::legacy::pokemon_positions::info_for_champion_id(b.champion).is_some(),
                "{}",
                b.champion
            );
            assert!(b.ticks > 0);
        }
        assert_eq!(BINDINGS.len(), 291);
    }

    #[test]
    fn state_driven_visuals_have_explicit_event_sites() {
        let gameplay = concat!(
            include_str!("legacy/pokemon_content.rs"),
            include_str!("legacy/pokemon_status.rs"),
            include_str!("stable_legacy_api.rs"),
            include_str!("projectiles.rs")
        );
        for b in BINDINGS.iter().filter(|b| state_driven(b.stem)) {
            if matches!(b.stem, "dragalge_ult_toxic_chain_tether" | "dragalge_ult_toxic_chain_impact") {
                assert!(gameplay.contains("crate::native_vfx::toxic_chain(ctx, caster_id, target_id)"));
                continue;
            }
            if matches!(b.stem,"blastoise_ult_cannon"|"magmortar_ult_eruption_cannonball"|"coalossal_ult_magma_storm_line"|"decidueye_basicattack_leafage_triple_arc") {
                // Magmortar's scheduled launch resolves the binding by champion/slot.
                continue;
            }
            assert!(
                gameplay.contains(b.stem),
                "No gameplay event for {}",
                b.stem
            );
        }
    }
}
