use std::sync::{Mutex, OnceLock};

use game_core::PlayerState;
use mod_api::{AttackType, BuffState, BuffType, CCState, EntityPos, GameCtx, Position};

use crate::pokemon_content::{
    champion_base_stat_for_level, ActionSlot, PokemonChampion, PokemonMove, PokemonMoveEffect,
};
use crate::pokemon_types::{PokemonType, TypeSet};

fn combat_ctx_id(ctx: &GameCtx) -> usize {
    let seed = ctx.seed() as usize;
    if seed != 0 {
        return seed;
    }

    ctx as *const GameCtx as usize
}

#[allow(dead_code)]
const PARALYSIS_DURATION_TICKS: usize = 30 * 60;
const PARALYSIS_ROLL_INTERVAL_TICKS: usize = 60;
const PARALYSIS_STUN_TICKS: u64 = 60;
const PARALYSIS_CHANCE_DEN: u64 = 3;
const BURN_DURATION_TICKS: usize = 15 * 60;
const BURN_TICK_INTERVAL: usize = 60;
const POISON_DURATION_TICKS: usize = 15 * 60;
const POISON_TICK_INTERVAL: usize = 60;
const BLEED_TICK_INTERVAL: usize = 60;
const INFESTATION_TICK_INTERVAL: usize = 15;
const ICE_FIELD_TICK_INTERVAL: usize = 60;
const BLAZE_CHAIN_WINDOW_TICKS: usize = 90;
const BLAZE_REQUIRED_CONTACT_TICKS: usize = 3 * 60;
const MEGA_LAUNCHER_WINDOW_TICKS: usize = 150;
const MEGA_LAUNCHER_CRIT_PER_HIT: usize = 12;
const LEECH_TICK_INTERVAL: usize = 60;
const LEECH_SPREAD_WIDTH: u64 = 9000;
const TANGLING_AURA_INTERVAL_TICKS: usize = 30;
const TANGLING_AURA_RADIUS: u64 = 70000;
const TANGLING_TRIGGER_RADIUS: u64 = 90000;
const HELPING_HAND_INTERVAL_TICKS: usize = 30;
const HELPING_HAND_RADIUS: u64 = 65000;
const INTIMIDATE_INTERVAL_TICKS: usize = 30;
const INTIMIDATE_RADIUS: u64 = 42000;
const HOUNDOOM_FOUL_PLAY_WINDOW_TICKS: usize = 5 * 60;
const MIASMA_DURATION_TICKS: usize = 15 * 60;
const MIASMA_DEATH_SPREAD_RADIUS: u64 = 32000;
const CLAWITZER_CLING_INTERVAL_TICKS: usize = 1;
const CLAWITZER_CLING_BUFF_TICKS: usize = 5;
const CLAWITZER_INITIAL_ATTACH_DELAY_TICKS: usize = 90;
const CLAWITZER_REATTACH_TICKS: usize = 15 * 60;
const CLAWITZER_REATTACH_RADIUS: u64 = 120000;
const COMFEY_ATTACH_INTERVAL_TICKS: usize = 1;
const COMFEY_ATTACH_BUFF_TICKS: usize = 5;
const COMFEY_INITIAL_ATTACH_DELAY_TICKS: usize = 90;
const COMFEY_REATTACH_TICKS: usize = 5 * 60;
const COMFEY_REATTACH_RADIUS: u64 = 100000;
const ATTACH_TETHER_DEADZONE: u64 = 1200;
const ATTACH_TETHER_SPEED: u64 = 900000;
const ATTACH_TETHER_TICKS: u64 = 2;
const ATTACH_RETURN_DETACH_WINDOW_TICKS: usize = 8 * 60;
const WISH_MOVE_THRESHOLD: u64 = 2500;
const EEVEELUTION_INTERVAL_TICKS: usize = 30;
const AQUA_RING_INTERVAL_TICKS: usize = 30;
const MISTY_TERRAIN_INTERVAL_TICKS: usize = 30;
const BRINE_FIELD_INTERVAL_TICKS: usize = 30;
const STICKY_WEB_INTERVAL_TICKS: usize = 30;
const WEB_WALKER_SPOT_TICKS: usize = 2 * 60;
const GRASSY_TERRAIN_INTERVAL_TICKS: usize = 30;
const SHIFTRY_BUSH_PROBE_RADIUS: u64 = 42000;
const SHIFTRY_BUSH_REQUIRED_TICKS: usize = 90;
const SHIFTRY_BUSH_LINGER_TICKS: usize = 2 * 60;
const SHIFTRY_BUSH_BUFF_REFRESH_TICKS: usize = 10;
const SHIFTRY_BUSH_BUFF_TICKS: usize = 20;
const AMPHAROS_LUMINOUS_RADIUS: u64 = 42000;
const AMPHAROS_LUMINOUS_INTERVAL_TICKS: usize = 5 * 60;
const AMPHAROS_LUMINOUS_BASE_AP: usize = 50;
const AMPHAROS_LUMINOUS_AP_RATIO: usize = 10;
const AMPHAROS_LUMINOUS_SLOW_PERCENT: i32 = 10;
const AMPHAROS_LUMINOUS_SLOW_TICKS: usize = 60;
const AMPHAROS_TRUE_SIGHT_REFRESH_TICKS: usize = 8;
const XATU_STILL_MOVE_THRESHOLD_SQ: u64 = SNORLAX_MOVE_THRESHOLD_SQ;
const XATU_PROPHECY_RADIUS: u64 = 180000;
const XATU_PROPHECY_REVEAL_TICKS: usize = 18;
const XATU_PROPHECY_DEBUFF_TICKS: usize = 3 * 60;
const XATU_PROPHECY_MAGIC_RESIST_MULT: i32 = -10;
const QUAQUAVAL_AQUA_STEP_DROP_DISTANCE: u64 = 9500;
const QUAQUAVAL_AQUA_STEP_DROP_INTERVAL_TICKS: usize = 8;
const QUAQUAVAL_AQUA_STEP_SEGMENT_TICKS: usize = 8 * 60;
const QUAQUAVAL_AQUA_STEP_MAX_SEGMENTS: usize = 8;
const QUAQUAVAL_AQUA_STEP_WIDTH: u64 = 9500;
const QUAQUAVAL_AQUA_STEP_INTERVAL_TICKS: usize = 15;
const QUAQUAVAL_AQUA_STEP_ALLY_SPEED_MULT: i32 = 14;
const QUAQUAVAL_AQUA_STEP_EMPOWERED_ALLY_SPEED_MULT: i32 = 26;
const QUAQUAVAL_AQUA_STEP_ENEMY_SLOW_MULT: i32 = 10;
const QUAQUAVAL_AQUA_STEP_EMPOWERED_ENEMY_SLOW_MULT: i32 = 22;
const QUAQUAVAL_AQUA_STEP_WATER_DAMAGE_BONUS: usize = 12;
const QUAQUAVAL_AQUA_STEP_EMPOWERED_WATER_DAMAGE_BONUS: usize = 24;
const QUAQUAVAL_EXCITING_DANCE_TRAIL_GRACE_TICKS: usize = 8;
const ARCANINE_BLAZING_MANE_ATTACK_AP_REDUCE: i32 = 7;
const ARCANINE_BLAZING_MANE_TICKS: usize = 3 * 60;
const ARCANINE_BLAZING_MANE_MAX_STACKS: usize = 3;
const ARCANINE_BLAZING_MANE_MELEE_BURN_CHANCE: usize = 20;
const ARCANINE_BLAZING_MANE_BURN_TICKS: usize = 5 * 60;
const ARCANINE_BLAZING_MANE_BURN_DAMAGE: usize = 12;
const ARCANINE_BLAZING_MANE_ABILITY_HEAL_PERCENT: usize = 2;
const ARCANINE_MELEE_CONTACT_RANGE: u64 = 18000;
const WISHIWASHI_SCHOOLING_RADIUS: u64 = 56000;
const WISHIWASHI_SCHOOLING_BUFF_TICKS: usize = 35;
const WISHIWASHI_SCHOOLING_DEFENCE_PER_ALLY: i32 = 12;
const WISHIWASHI_SCHOOLING_COOLDOWN_PER_ALLY: i32 = 5;
const WISHIWASHI_SCHOOLING_MAX_ALLIES: usize = 4;
const WISHIWASHI_ALONE_HP_MULT: i32 = -8;
const WISHIWASHI_ALONE_MOVE_SPEED_MULT: i32 = 15;
const MISSINGNO_PASSIVE_BUFF_TICKS: usize = 2 * 60;
const MISSINGNO_PASSIVE_MOVE_SPEED_MULT: i32 = 20;
const MISSINGNO_PASSIVE_COOLDOWN_MULT: i32 = 18;
const MISSINGNO_PASSIVE_CHANCE_PERCENT: usize = 25;
const SIGILYPH_GLYPH_DURATION_TICKS: usize = 5 * 60;
const SIGILYPH_GLYPH_PROXIMITY_RADIUS: u64 = 22000;
const SIGILYPH_GLYPH_EXPLOSION_RADIUS: u64 = 17000;
const SIGILYPH_GLYPH_DAMAGE_BONUS_PERCENT: usize = 24;
const SIGILYPH_GLYPH_EXPLOSION_BASE_AP: usize = 24;
const SIGILYPH_GLYPH_EXPLOSION_AP_RATIO: usize = 30;
const SIGILYPH_GLYPH_SPLASH_PERCENT: usize = 50;
pub const WEAVILE_LONE_PREDATOR_RADIUS: u64 = 65000;
pub const WEAVILE_LONE_PREDATOR_DAMAGE_BONUS_PERCENT: usize = 20;
const SWANNA_TAILWIND_TRIGGER_DISTANCE: u64 = 55000;
const SWANNA_TAILWIND_BUFF_TICKS: usize = 4 * 60;
const SWANNA_TAILWIND_COOLDOWN_TICKS: usize = 5 * 60;
const SWANNA_TAILWIND_REFRESH_TICKS: usize = 25;
const SWANNA_TAILWIND_DIRECTION_DOT_MIN: i128 = 0;
const SWANNA_SKY_CIRCUS_HIT_RADIUS: u64 = 13500;
const KRICKETUNE_AURA_INTERVAL_TICKS: usize = 30;
const FROSMOTH_SLEEP_INTERVAL_TICKS: usize = 30;
const FROSMOTH_SLEEP_MOVE_INTERVAL_TICKS: usize = 12;
const FROSMOTH_SLEEP_PATH_POINTS: usize = 8;
const LUDICOLO_RAIN_DISH_INTERVAL_TICKS: usize = 60;
const LUDICOLO_RAIN_DISH_RADIUS: u64 = 36000;
const LUDICOLO_RAIN_DISH_HEAL: usize = 18;
const LUDICOLO_RAIN_DISH_WATER_BONUS: usize = 18;
const LUDICOLO_RAIN_DISH_FIRE_REDUCE: usize = 25;
const LUDICOLO_FIRE_RESIST_REDUCE: usize = 35;
const KINGDRA_FOCUS_TICKS: usize = 3 * 60;
const KINGDRA_DRAGON_DANCE_BOOST_TICKS: usize = 2 * 60;
const KINGDRA_DRAGON_DANCE_BUFF_TICKS: usize = 35;
const CLOYSTER_OVERCOAT_TICKS: usize = 35;
const SNORLAX_BERRY_INTERVAL_TICKS: usize = 90;
const SNORLAX_PASSIVE_HEAL_INTERVAL_TICKS: usize = 60;
const SNORLAX_SLEEP_DELAY_TICKS: usize = 2 * 60;
const SNORLAX_SLEEP_HEAL_INTERVAL_TICKS: usize = 60;
const SNORLAX_FULL_BELLY_BUFF_TICKS: usize = 35;
const SNORLAX_MOVE_THRESHOLD_SQ: u64 = 2500 * 2500;
const CHARM_HEAL_INTERVAL_TICKS: usize = 60;
const CLEANSE_IMMUNITY_TICKS: usize = 90;
const FLAME_TRAIL_INTERVAL_TICKS: usize = 30;
const WHIRLPOOL_INTERVAL_TICKS: usize = 30;
const SPEED_BOOST_INTERVAL_TICKS: usize = 30;
const SPEED_BOOST_PERCENT_PER_SECOND: usize = 8;
const SPEED_BOOST_MAX_PERCENT: usize = 200;
const SAWK_THROH_INTERVAL_TICKS: usize = 30;
const AUDINO_REGENERATOR_INTERVAL_TICKS: usize = 60;
const AUDINO_REGENERATOR_DELAY_TICKS: usize = 2 * 60;
const AUDINO_REGENERATOR_RADIUS: u64 = 50000;
const SYMBIOSIS_INTERVAL_TICKS: usize = 30;
const SYMBIOSIS_RADIUS: u64 = 70000;
const HONEY_GATHER_INTERVAL_TICKS: usize = 5 * 60;
const HONEY_GATHER_MAX_STACKS: usize = 5;
const SMEARGLE_CANDIDATE_TICKS: usize = 20 * 60;
const PYUKUMUKU_BARB_STACK_WINDOW_TICKS: usize = 4 * 60;
const HEAL_BLOCK_AURA_INTERVAL_TICKS: usize = 30;
const HEAL_BLOCK_AURA_RADIUS: u64 = 42000;
const HAWLUCHA_MOMENTUM_ATTACK_SPEED: i32 = 30;
const HAWLUCHA_MOMENTUM_MOVE_SPEED: i32 = 15;
const HAWLUCHA_MOMENTUM_TICKS: usize = 3 * 60;
const HAWLUCHA_FLYING_PRESS_AD_PER_UNIQUE_CHAMPION: i32 = 6;
const BOUFFALANT_AFRO_CC_REDUCE_NUM: usize = 2;
const BOUFFALANT_AFRO_CC_REDUCE_DEN: usize = 3;
const BOUFFALANT_AFRO_DAMAGE_REDUCE_PERCENT: usize = 30;
const BOUFFALANT_RECENT_CC_WINDOW_TICKS: usize = 5 * 60;
const POKEMON_DAMAGE_ASSIST_WINDOW_TICKS: usize = 10 * 60;
const POKEMON_PARTICIPATION_ASSIST_WINDOW_TICKS: usize = 15 * 60;
const POKEMON_KILL_CREDIT_HISTORY_TICKS: usize = 15 * 60;
const POKEMON_CONTEXT_LEDGER_MAX: usize = 4096;
const POKEMON_COMBAT_STATS_MAX: usize = 1024;
const POKEMON_BASE_STAT_SYNC_MAX: usize = 1024;
const POKEMON_BASE_STAT_SYNC_LOG_LIMIT: usize = 80;
const POKEMON_KILL_LEDGER_MAX: usize = 2048;
const POKEMON_ASSIST_LEDGER_MAX: usize = 2048;
const POKEMON_PARTICIPATION_LEDGER_MAX: usize = 4096;
const STARMIE_ILLUMINATE_CHARGE_INTERVAL_TICKS: usize = 5 * 60;
const STARMIE_ILLUMINATE_MARK_TICKS: usize = 8 * 60;
const VFX_FIRE: u32 = 0xffff6b2c;
const VFX_WATER: u32 = 0xff3d8dff;
const VFX_GRASS: u32 = 0xff53d86a;
const VFX_ICE: u32 = 0xff76e4ff;
const VFX_ELECTRIC: u32 = 0xffffdf3d;
const VFX_POISON: u32 = 0xffb864d8;
const VFX_FAIRY: u32 = 0xffff9ad6;
const VFX_PSYCHIC: u32 = 0xffff5fa8;
const VFX_NORMAL: u32 = 0xffd6d6c8;
const VFX_BLEED: u32 = 0xffd44141;
const VFX_FIGHTING: u32 = 0xffc84b34;
const VFX_GROUND: u32 = 0xffb8854b;
const VFX_STEEL: u32 = 0xffb8c0d0;
const VFX_BUG: u32 = 0xffa8c94a;
const VFX_DARK: u32 = 0xff5a4a70;
const VFX_DRAGON: u32 = 0xff8a5cff;
const VFX_FLYING: u32 = 0xff9ad7ff;

#[derive(Clone, Copy, Debug)]
struct ParalysisState {
    caster_id: usize,
    entity_id: usize,
    expires_at: usize,
    next_roll_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct BurnState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct PoisonState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PokemonBuffPolarity {
    Beneficial,
    Harmful,
    Neutral,
}

#[derive(Clone, Debug)]
struct TrackedBuffState {
    source_id: usize,
    target_id: usize,
    buff: BuffState,
    polarity: PokemonBuffPolarity,
    expires_at: usize,
    transferable: bool,
}

#[derive(Clone, Copy, Debug)]
struct KingdraFocusState {
    caster_id: usize,
    target_id: usize,
    consecutive_hits: usize,
    focus_stacks: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct KingdraDragonDanceState {
    entity_id: usize,
    boosted_until: usize,
    primed_until: usize,
    dragon_pulse_cooldown_until: usize,
}

#[derive(Clone, Copy, Debug)]
struct DelibirdPresentState {
    entity_id: usize,
    basic_casts: usize,
    last_pos: EntityPos,
    moving_until: usize,
}

#[derive(Clone, Copy, Debug)]
struct MiasmaState {
    caster_id: usize,
    target_id: usize,
    stacks: usize,
    poison_awards: usize,
    poison_damage_per_tick: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct BleedState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct InfestationState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct YanmegaTintedLensState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct YanmegaGigaDrainState {
    caster_id: usize,
    target_id: usize,
    started_at: usize,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    damage_per_tick: usize,
    poison_health_unit: usize,
    poison_damage_per_tick: usize,
    poison_ticks: usize,
    total_drained: usize,
    last_caster_hp: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct PokemonCcEvent {
    source_id: usize,
    target_id: usize,
    applied_at: usize,
    expires_at: usize,
    disruptive: bool,
}

#[derive(Clone, Copy, Debug)]
struct FrozenState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ArmarougeWeakArmorState {
    entity_id: usize,
    weak_stacks: usize,
    armor_cannon_until: usize,
    armor_cannon_defence_mult: i32,
    armor_cannon_magic_resistance_mult: i32,
}

#[derive(Clone, Copy, Debug)]
struct ArmarougeMysticalFireAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    radius: u64,
    damage: usize,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
    confusion_chance_percent: usize,
    confusion_stacks: usize,
    confusion_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct IceFieldState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
    freeze_chance_percent: usize,
    freeze_ticks: usize,
    slow_percent: i32,
    control_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct LeechSeedState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
    break_range: u64,
}

#[derive(Clone, Copy, Debug)]
struct BlazeContactState {
    caster_id: usize,
    target_id: usize,
    accumulated_ticks: usize,
    last_contact_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct EntityTypeState {
    entity_id: usize,
    types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct MegaLauncherState {
    caster_id: usize,
    target_id: usize,
    stacks: usize,
    last_hit_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct TanglingAuraState {
    venusaur_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct HelpingHandAuraState {
    eevee_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct WishChannelState {
    caster_id: usize,
    target_id: usize,
    started_at: usize,
    max_ticks: usize,
    heal_amount: usize,
    last_pos: EntityPos,
}

#[derive(Clone, Copy, Debug)]
struct PlayerEntityState {
    ctx_id: usize,
    player_id: usize,
    entity_id: usize,
    life_id: usize,
    last_seen_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct GholdengoGoldState {
    player_id: usize,
    entity_id: usize,
    last_seen_gold: usize,
    earned_gold: usize,
}

#[derive(Clone, Copy, Debug)]
struct EntityChampionState {
    ctx_id: usize,
    entity_id: usize,
    champion_id: &'static str,
}

#[derive(Clone, Copy, Debug)]
struct EntityOwnerState {
    ctx_id: usize,
    entity_id: usize,
    player_id: usize,
    life_id: usize,
    champion_id: Option<&'static str>,
    last_seen_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct ReceiverState {
    entity_id: usize,
    copied_champion_id: Option<&'static str>,
}

#[derive(Clone, Copy, Debug)]
struct ReversalState {
    entity_id: usize,
    expires_at: usize,
    retaliation_damage: usize,
}

#[derive(Clone, Copy, Debug)]
struct GalladePredictionState {
    gallade_id: usize,
    enemy_id: usize,
    expires_at: usize,
    damaged_reduce: usize,
}

#[derive(Clone, Copy, Debug)]
struct JustifiedState {
    entity_id: usize,
}

#[derive(Clone, Copy, Debug)]
struct AudinoProtectState {
    audino_id: usize,
    target_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct EndureState {
    target_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct AudinoRegeneratorState {
    audino_id: usize,
    last_damaged_tick: usize,
    next_heal_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct SymbiosisState {
    oranguru_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct NastyPlotState {
    caster_id: usize,
    target_id: usize,
    resolves_at: usize,
    damage: usize,
    stun_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct StoredPowerState {
    caster_id: usize,
    caster_team: usize,
    radius: u64,
    resolves_at: usize,
    stored_damage: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct HoneyGathererState {
    entity_id: usize,
    stacks: usize,
    next_stack_at: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SmeargleCopiedMove {
    pub source_champion: PokemonChampion,
    pub action: PokemonMove,
}

#[derive(Clone, Copy, Debug)]
struct SmeargleCandidateState {
    copied: SmeargleCopiedMove,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct SmeargleState {
    entity_id: usize,
    last_candidate: Option<SmeargleCandidateState>,
    learned: [Option<SmeargleCopiedMove>; 4],
    ready_at: [usize; 4],
}

#[derive(Clone, Copy, Debug)]
struct PowerUpPunchCooldownState {
    player_id: usize,
    ready_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct PowerUpPunchChannelState {
    player_id: usize,
    caster_id: usize,
    caster_team: usize,
    started_at: usize,
    max_ticks: usize,
    target_pos: EntityPos,
    last_pos: EntityPos,
    ad_damage: usize,
    width: u64,
    full_cooldown_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct QuickReturnDashState {
    entity_id: usize,
    origin: EntityPos,
    trigger_at: usize,
    speed: u64,
    ticks: u64,
}

#[derive(Clone, Copy, Debug)]
struct ScheduledForceAwayState {
    entity_id: usize,
    away_from: EntityPos,
    trigger_at: usize,
    speed: u64,
    ticks: u64,
}

#[derive(Clone, Copy, Debug)]
struct StarmieIlluminateState {
    entity_id: usize,
    charges: usize,
    next_charge_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct StarmieIlluminateMarkState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
}

#[derive(Clone, Debug)]
struct OrbeetleAgilityChainState {
    caster_id: usize,
    targets: Vec<usize>,
    next_index: usize,
    next_hop_at: usize,
    expires_at: usize,
    damage: usize,
    attacker_types: TypeSet,
    force_move_speed: u64,
    force_move_ticks: u64,
    hop_interval_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct BattleBondState {
    entity_id: usize,
    stacks: usize,
}

#[derive(Clone, Copy, Debug)]
struct SpiritShackleState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct SoakState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct WillOWispChargeState {
    entity_id: usize,
    charges: usize,
    next_charge_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct StickyWebChargeState {
    entity_id: usize,
    charges: usize,
    next_charge_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct PorygonTypeState {
    entity_id: usize,
    current_type: PokemonType,
    seen_mask: u32,
}

#[derive(Clone, Copy, Debug)]
struct EeveelutionState {
    entity_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct AquaRingState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    heal_per_tick: usize,
    enemy_attack_mult: i32,
    enemy_debuff_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct MistyTerrainState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    expires_at: usize,
    next_tick_at: usize,
    heal_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct BrineFieldState {
    center: EntityPos,
    radius: u64,
    expires_at: usize,
    next_tick_at: usize,
    slow_percent: i32,
    slow_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct GrassyTerrainState {
    caster_id: usize,
    center: EntityPos,
    radius: u64,
    created_at: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_bonus_percent: usize,
    attack_speed_mult: i32,
    buff_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct RillaboomDrumAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    heal_per_tick: usize,
    ally_move_speed_mult: i32,
    ally_buff_ticks: usize,
    enemy_slow_percent: i32,
    enemy_slow_ticks: usize,
    final_stun_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct RillaboomGrassySurgeState {
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    expires_at: usize,
    next_tick_at: usize,
    width: u64,
    tick_interval: usize,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct ShiftryTornadoState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    tick_interval: usize,
    damage_per_tick: usize,
    damage_growth_percent: usize,
    ticks_done: usize,
    lift_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct ShiftryForestCamouflageState {
    entity_id: usize,
    entered_bush_at: usize,
    in_bush: bool,
    linger_until: usize,
    last_buff_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct RillaboomDrumSoloState {
    entity_id: usize,
    window_start: usize,
    casts: usize,
}

#[derive(Clone, Copy, Debug)]
struct SigilyphGlyphState {
    caster_id: usize,
    caster_team: usize,
    target_id: usize,
    target_team: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct SigilyphGravityState {
    entity_id: usize,
    original_types: TypeSet,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct WeavileHuntState {
    entity_id: usize,
    stealth_expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct AmpharosSearchlightTailState {
    entity_id: usize,
    expires_at: usize,
    move_speed_mult: i32,
    damage_bonus_percent: usize,
    extend_per_champion_ticks: usize,
    paralysis_chance_percent: usize,
    paralysis_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct AmpharosLuminousPulseState {
    entity_id: usize,
    next_pulse_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct AmpharosTrueSightState {
    source_id: usize,
    team: usize,
    radius: u64,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct AmpharosGigavoltState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    trigger_at: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage: usize,
    radius: u64,
    zone_tick_interval: usize,
    zone_slow_percent: i32,
    zone_slow_ticks: usize,
    attack_speed_mult: i32,
    attack_speed_buff_ticks: usize,
    attacker_types: TypeSet,
    triggered: bool,
}

#[derive(Clone, Copy, Debug)]
struct XatuStillnessState {
    entity_id: usize,
    last_pos: Option<EntityPos>,
    still_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct XatuPainAmplifierState {
    target_id: usize,
    expires_at: usize,
    damage_taken_percent: usize,
}

#[derive(Clone, Copy, Debug)]
struct XatuSuperPsyState {
    caster_id: usize,
    caster_team: usize,
    end: EntityPos,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    width: u64,
    travel_range: u64,
    damage: usize,
    close_bonus_percent: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct QuaquavalAquaStepEmitterState {
    entity_id: usize,
    last_pos: Option<EntityPos>,
    last_drop_pos: Option<EntityPos>,
    last_drop_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct QuaquavalAquaStepSegmentState {
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    width: u64,
    expires_at: usize,
    next_tick_at: usize,
    empowered: bool,
}

#[derive(Clone, Copy, Debug)]
struct QuaquavalSpiralShotState {
    target_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct QuaquavalExcitingDanceState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ArcanineExtremespeedShieldState {
    entity_id: usize,
    expires_at: usize,
    broken_move_speed_mult: i32,
    broken_move_speed_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct ArcanineBlazingManeState {
    arcanine_id: usize,
    attacker_id: usize,
    stacks: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct WishiwashiSchoolingState {
    entity_id: usize,
    force_school_until: usize,
}

#[derive(Clone, Copy, Debug)]
struct WishiwashiMassiveCatchState {
    caster_id: usize,
    target_id: usize,
    caster_team: usize,
    start_pos: EntityPos,
    target_pos: EntityPos,
    spit_pos: EntityPos,
    trigger_at: usize,
    catch_at: usize,
    spit_at: usize,
    next_chew_at: usize,
    line_width: u64,
    damage: usize,
    chew_damage: usize,
    outbound_ticks: usize,
    return_ticks: usize,
    chew_interval_ticks: usize,
    throw_ticks: usize,
    force_move_speed: u64,
    attacker_types: TypeSet,
    outbound_started: bool,
    caught: bool,
}

#[derive(Clone, Copy, Debug)]
struct MissingNoGlitchStormState {
    caster_id: usize,
    caster_team: usize,
    radius: u64,
    chain_radius: u64,
    damage: usize,
    chain_jumps: usize,
    tick_interval_min: usize,
    tick_interval_max: usize,
    next_tick_at: usize,
    expires_at: usize,
    attacker_types: TypeSet,
    seed: u64,
}

#[derive(Clone, Copy, Debug)]
struct MissingNoTrickRoomState {
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    length: u64,
    width: u64,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    enemy_speed_slow: i32,
    enemy_attack_speed_slow: i32,
    enemy_hp_random_min: i32,
    enemy_hp_random_max: i32,
    enemy_cooldown_random_min: i32,
    enemy_cooldown_random_max: i32,
    missingno_speed_mult: i32,
    missingno_attack_speed_mult: i32,
    missingno_cooldown_mult: i32,
    ally_speed_mult: i32,
    ally_attack_speed_mult: i32,
    buff_ticks: usize,
    seed: u64,
}

#[derive(Clone, Copy, Debug)]
struct MissingNoPendingDebuffState {
    source_id: usize,
    target_id: usize,
    trigger_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct SwannaTailwindState {
    entity_id: usize,
    last_pos: EntityPos,
    segment_dx: i64,
    segment_dy: i64,
    segment_distance: u64,
    buff_until: usize,
    cooldown_until: usize,
    last_buff_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct SwannaCycloneState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    tick_interval: usize,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Debug)]
struct SwannaSkyCircusState {
    caster_id: usize,
    start_pos: EntityPos,
    center: EntityPos,
    radius: u64,
    waypoints: Vec<EntityPos>,
    waypoint_index: usize,
    next_waypoint_at: usize,
    waypoint_ticks: usize,
    force_move_speed: u64,
    next_hit_at: usize,
    hit_interval_ticks: usize,
    hits_remaining: usize,
    hit_damage: usize,
    landing_damage: usize,
    target_damage_percent: usize,
    targets: Vec<usize>,
    expires_at: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct MarowakBoneWindmillState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    tick_interval: usize,
    damage: usize,
    knockback_speed: u64,
    knockback_ticks: u64,
    empowered_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct MarowakWindmillBonemerangState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct GarganaclSaltPatchState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    created_at: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage: usize,
    slow_percent: i32,
    slow_ticks: usize,
    water_steel_bonus_percent: usize,
}

#[derive(Clone, Copy, Debug)]
struct GarganaclBlessedSaltState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    outer_radius: u64,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    damage: usize,
    slow_percent: i32,
    slow_ticks: usize,
    anti_heal_percent: i32,
    anti_heal_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct GarganaclPermanentSaltState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    outer_radius: u64,
    inner_radius: u64,
    next_tick_at: usize,
    tick_interval: usize,
    damage: usize,
    slow_percent: i32,
    slow_ticks: usize,
    water_steel_bonus_percent: usize,
}

#[derive(Clone, Copy, Debug)]
struct StickyWebState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    created_at: usize,
    next_tick_at: usize,
    kricketune_speed_percent: i32,
    enemy_slow_percent: i32,
    buff_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct WebWalkerSpot {
    pub source_id: usize,
    pub team: usize,
    pub target_id: usize,
    pub target_pos: EntityPos,
    pub expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct WhirlpoolState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    self_heal_per_tick: usize,
    self_attack_mult: i32,
    self_buff_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct CharmHealState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    heal_per_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct BlisseyHealAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    heal_per_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct KleavorHitState {
    entity_id: usize,
    hits: usize,
    last_hit_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct StealthRockState {
    entity_id: usize,
    expires_at: usize,
    next_toggle_at: usize,
    untargetable: bool,
}

#[derive(Clone, Copy, Debug)]
struct LightMetalState {
    entity_id: usize,
    last_pos: EntityPos,
    distance_without_damage: u64,
    shield_until: usize,
}

#[derive(Clone, Copy, Debug)]
struct SoftUntargetableState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct HawluchaCounterState {
    entity_id: usize,
    expires_at: usize,
    reduce_percent: usize,
    retaliation_damage: usize,
    radius: u64,
    slow_percent: i32,
    slow_ticks: usize,
    lifesteal_percent: i32,
    lifesteal_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct ZeraoraZingZapState {
    entity_id: usize,
    expires_at: usize,
    damage: usize,
    blink_range: u64,
    force_move_speed: u64,
    force_move_ticks: u64,
}

#[derive(Clone, Debug)]
struct ZeraoraThunderCageState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    tick_damage: usize,
    exit_damage: usize,
    slow_percent: i32,
    occupants: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
struct ZeraoraMercilessHitState {
    caster_id: usize,
    target_id: usize,
    slot: ActionSlot,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ZeraoraMercilessCooldownState {
    caster_id: usize,
    target_id: usize,
    ready_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ZeraoraWildChargeMarkState {
    caster_id: usize,
    target_id: usize,
    bonus_percent: usize,
    expires_at: usize,
}

#[derive(Clone, Debug)]
struct HawluchaMomentumState {
    entity_id: usize,
    flying_press_targets: Vec<usize>,
}

#[derive(Clone, Debug)]
struct BouffalantRetaliateState {
    entity_id: usize,
    caster_team: usize,
    expires_at: usize,
    damage_reduce_percent: usize,
    retaliation_damage_percent: usize,
    bonus_ad_damage: usize,
    attacker_types: TypeSet,
    attackers: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, Debug)]
struct BouffalantCcState {
    entity_id: usize,
    last_cc_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct BouffalantUnstoppableState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct BouffalantHeadChargeMarkState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    bonus_max_hp_percent: usize,
}

#[derive(Clone, Copy, Debug)]
struct ConfusionState {
    entity_id: usize,
    stacks: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct DelayedConfusionState {
    caster_id: usize,
    target_id: usize,
    trigger_at: usize,
    stacks: usize,
    ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct SingAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    sleep_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct FrosmothSleepCircleState {
    caster_id: usize,
    caster_team: usize,
    origin: EntityPos,
    center: EntityPos,
    expires_at: usize,
    next_tick_at: usize,
    next_move_at: usize,
    path_index: usize,
    radius: u64,
    sleep_ticks: usize,
    force_move_speed: u64,
    force_move_ticks: u64,
}

#[derive(Clone, Copy, Debug)]
struct BugBuzzAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    radius: u64,
    damage_per_tick: usize,
    confusion_stacks: usize,
    confusion_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct LudicoloRainDishState {
    entity_id: usize,
    next_tick_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct AlluringVoiceAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    outer_radius: u64,
    inner_radius: u64,
    taunt_ticks: usize,
    confusion_stacks: usize,
    confusion_ticks: usize,
}

#[derive(Clone, Copy, Debug)]
struct EarthquakeAuraState {
    caster_id: usize,
    caster_team: usize,
    expires_at: usize,
    next_tick_at: usize,
    tick_interval: usize,
    radius: u64,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct BloodMoonState {
    entity_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    hp_loss_per_second_percent: usize,
}

#[derive(Clone, Copy, Debug)]
struct RoostState {
    entity_id: usize,
    original_types: TypeSet,
    expires_at: usize,
    next_tick_at: usize,
    heal_per_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct FlameTrailState {
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    width: u64,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
    attacker_types: TypeSet,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
}

#[derive(Clone, Copy, Debug)]
struct SpeedBoostState {
    entity_id: usize,
    last_reset_tick: usize,
    last_buff_tick: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SawkThrohForm {
    Sawk,
    Throh,
}

#[derive(Clone, Copy, Debug)]
struct SawkThrohState {
    entity_id: usize,
    form: SawkThrohForm,
    last_buff_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct SturdyState {
    entity_id: usize,
    ready_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct IntimidateAuraState {
    houndoom_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct HoundoomDamageState {
    entity_id: usize,
    last_damaged_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct ArbokBasicHitState {
    entity_id: usize,
    hits: usize,
    last_hit_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct PyukumukuBarbState {
    defender_id: usize,
    attacker_id: usize,
    stacks: usize,
    last_hit_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct ClawitzerClingState {
    entity_id: usize,
    attached_ally: Option<usize>,
    detached_until: usize,
    last_ally_hp: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct ComfeyAttachState {
    entity_id: usize,
    attached_ally: Option<usize>,
    detached_until: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct PlayerReturnIntentState {
    player_id: usize,
    entity_id: Option<usize>,
    tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct OctilleryLockOnState {
    caster_id: usize,
    target_id: usize,
    crit_chance: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct DirectDamageLedgerState {
    attacker_id: usize,
    target_id: usize,
    damage: usize,
    tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct PokemonDamageCreditState {
    ctx_id: usize,
    attacker_player_id: usize,
    attacker_team: usize,
    target_id: usize,
    target_team: usize,
    target_player_id: usize,
    target_generation: usize,
    target_champion_id: Option<&'static str>,
    damage: usize,
    tick: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct PokemonKillCreditState {
    ctx_id: usize,
    killer_id: usize,
    killer_player_id: usize,
    killed_id: usize,
    killed_player_id: usize,
    killed_generation: usize,
    assist_ids: Vec<usize>,
    tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct PokemonAssistAwardState {
    ctx_id: usize,
    assist_player_id: usize,
    killed_id: usize,
    killed_player_id: Option<usize>,
    killed_generation: Option<usize>,
    tick: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PokemonParticipationKind {
    EnemyDamage,
    EnemyHarmfulBuff,
    AllyBeneficialBuff,
    AllyHealing,
}

#[derive(Clone, Copy, Debug)]
struct PokemonParticipationState {
    ctx_id: usize,
    participant_player_id: usize,
    participant_team: usize,
    subject_player_id: usize,
    subject_generation: Option<usize>,
    subject_team: usize,
    kind: PokemonParticipationKind,
    tick: usize,
}

#[derive(Clone, Debug)]
struct PokemonPlayerIdentityState {
    player_id: usize,
    athlete_id: usize,
    team: usize,
    position: Position,
    champion_name: String,
    seen_count: usize,
}

#[derive(Clone, Debug)]
pub struct PokemonPlayerIdentitySnapshot {
    pub player_id: usize,
    pub athlete_id: usize,
    pub team: usize,
    pub position: Position,
    pub champion_name: String,
    pub seen_count: usize,
}

#[derive(Clone, Debug)]
struct PokemonCombatStatState {
    ctx_id: usize,
    player_id: usize,
    athlete_id: Option<usize>,
    entity_id: usize,
    team: usize,
    position: Position,
    champion_id: Option<&'static str>,
    last_seen_tick: usize,
    damage_dealt: usize,
    damage_taken: usize,
    healing_done: usize,
    kills: usize,
    deaths: usize,
    assists: usize,
}

#[derive(Clone, Debug)]
struct PokemonBaseStatSyncState {
    ctx_id: usize,
    player_id: usize,
    damage_dealt: usize,
    damage_taken: usize,
    healing_done: usize,
    kills: usize,
    assists: usize,
}

#[derive(Clone, Debug)]
pub struct PokemonCombatStatSnapshot {
    pub ctx_id: usize,
    pub player_id: usize,
    pub athlete_id: Option<usize>,
    pub athlete_source: &'static str,
    pub team: usize,
    pub position: Position,
    pub champion_id: Option<&'static str>,
    pub damage_dealt: usize,
    pub damage_taken: usize,
    pub healing_done: usize,
    pub kills: usize,
    pub deaths: usize,
    pub assists: usize,
}

#[derive(Clone, Copy, Debug)]
struct LightScreenState {
    start: EntityPos,
    end: EntityPos,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct SmokeScreenState {
    start: EntityPos,
    end: EntityPos,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct DetectGuardState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct GrapploctSubmissionState {
    caster_id: usize,
    caster_team: usize,
    target_id: usize,
    next_tick_at: usize,
    expires_at: usize,
    tick_interval: usize,
    damage_per_tick: usize,
    execute_threshold_percent: usize,
    attacker_types: TypeSet,
}

#[derive(Clone, Copy, Debug)]
struct CoalossalMagmaStormState {
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    next_stage_at: usize,
    stage: usize,
    expires_at: usize,
    line_damage: usize,
    center_damage: usize,
    attacker_types: TypeSet,
    line_width: u64,
    side_offset: u64,
    center_radius: u64,
    stage_interval_ticks: usize,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
}

#[derive(Clone, Copy, Debug)]
struct DrampaBerserkState {
    entity_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct KommooDuelState {
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_refresh_at: usize,
    refresh_ticks: usize,
    target_attack_mult: i32,
    target_defence_mult: i32,
    caster_attack_speed_mult: i32,
}

#[derive(Clone, Copy, Debug)]
struct AntiHealState {
    source_id: usize,
    target_id: usize,
    percent: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ThievulHoneClawsState {
    entity_id: usize,
    anti_heal_percent: usize,
    anti_heal_ticks: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ThievulHealBlockAuraState {
    entity_id: usize,
    last_tick: usize,
}

#[derive(Clone, Copy, Debug)]
struct ThievulStakeoutState {
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    root_ticks: usize,
    damage_bonus_percent: usize,
    expires_at: usize,
    next_tick_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct WideGuardState {
    entity_id: usize,
    last_pos: EntityPos,
    facing_dx: i64,
    facing_dy: i64,
}

#[derive(Clone, Copy, Debug)]
struct ArchaludonIronDefenseState {
    entity_id: usize,
    expires_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct WonderGuardState {
    shedinja_id: usize,
    shell_broken_until: usize,
    knocked_until: usize,
    chits: usize,
}

#[derive(Clone, Copy, Debug)]
struct SnorlaxGluttonyState {
    entity_id: usize,
    berries: usize,
    last_pos: Option<EntityPos>,
    still_ticks: usize,
    next_move_berry_at: usize,
    next_berry_heal_at: usize,
    next_sleep_heal_at: usize,
    primed_basic_bonus_percent: usize,
    primed_until: usize,
}

#[derive(Clone, Copy, Debug)]
struct ShedinjaKillState {
    shedinja_id: usize,
    killer_id: usize,
    kills: usize,
}

static PARALYSIS: OnceLock<Mutex<Vec<ParalysisState>>> = OnceLock::new();
static BURNS: OnceLock<Mutex<Vec<BurnState>>> = OnceLock::new();
static POISONS: OnceLock<Mutex<Vec<PoisonState>>> = OnceLock::new();
static MIASMAS: OnceLock<Mutex<Vec<MiasmaState>>> = OnceLock::new();
static BLEEDS: OnceLock<Mutex<Vec<BleedState>>> = OnceLock::new();
static ARMAROUGE_WEAK_ARMORS: OnceLock<Mutex<Vec<ArmarougeWeakArmorState>>> = OnceLock::new();
static INFESTATIONS: OnceLock<Mutex<Vec<InfestationState>>> = OnceLock::new();
static YANMEGA_TINTED_LENSES: OnceLock<Mutex<Vec<YanmegaTintedLensState>>> = OnceLock::new();
static YANMEGA_GIGA_DRAINS: OnceLock<Mutex<Vec<YanmegaGigaDrainState>>> = OnceLock::new();
static POKEMON_CC_EVENTS: OnceLock<Mutex<Vec<PokemonCcEvent>>> = OnceLock::new();
static FROZENS: OnceLock<Mutex<Vec<FrozenState>>> = OnceLock::new();
static ICE_FIELDS: OnceLock<Mutex<Vec<IceFieldState>>> = OnceLock::new();
static LEECH_SEEDS: OnceLock<Mutex<Vec<LeechSeedState>>> = OnceLock::new();
static BLAZE_CONTACTS: OnceLock<Mutex<Vec<BlazeContactState>>> = OnceLock::new();
static ENTITY_TYPES: OnceLock<Mutex<Vec<EntityTypeState>>> = OnceLock::new();
static ENTITY_OWNERS: OnceLock<Mutex<Vec<EntityOwnerState>>> = OnceLock::new();
static MEGA_LAUNCHER: OnceLock<Mutex<Vec<MegaLauncherState>>> = OnceLock::new();
static TANGLING_AURAS: OnceLock<Mutex<Vec<TanglingAuraState>>> = OnceLock::new();
static HELPING_HAND_AURAS: OnceLock<Mutex<Vec<HelpingHandAuraState>>> = OnceLock::new();
static WISH_CHANNELS: OnceLock<Mutex<Vec<WishChannelState>>> = OnceLock::new();
static PLAYER_ENTITIES: OnceLock<Mutex<Vec<PlayerEntityState>>> = OnceLock::new();
static GHOLDENGO_GOLD: OnceLock<Mutex<Vec<GholdengoGoldState>>> = OnceLock::new();
static GALLADE_PREDICTIONS: OnceLock<Mutex<Vec<GalladePredictionState>>> = OnceLock::new();
static JUSTIFIED_ENTITIES: OnceLock<Mutex<Vec<JustifiedState>>> = OnceLock::new();
static AUDINO_PROTECTS: OnceLock<Mutex<Vec<AudinoProtectState>>> = OnceLock::new();
static ENDURES: OnceLock<Mutex<Vec<EndureState>>> = OnceLock::new();
static AUDINO_REGENERATORS: OnceLock<Mutex<Vec<AudinoRegeneratorState>>> = OnceLock::new();
static SMEARGLES: OnceLock<Mutex<Vec<SmeargleState>>> = OnceLock::new();
static POWER_UP_PUNCH_COOLDOWNS: OnceLock<Mutex<Vec<PowerUpPunchCooldownState>>> = OnceLock::new();
static POWER_UP_PUNCH_CHANNELS: OnceLock<Mutex<Vec<PowerUpPunchChannelState>>> = OnceLock::new();
static QUICK_RETURN_DASHES: OnceLock<Mutex<Vec<QuickReturnDashState>>> = OnceLock::new();
static SCHEDULED_FORCE_AWAYS: OnceLock<Mutex<Vec<ScheduledForceAwayState>>> = OnceLock::new();
static STARMIE_ILLUMINATES: OnceLock<Mutex<Vec<StarmieIlluminateState>>> = OnceLock::new();
static STARMIE_ILLUMINATE_MARKS: OnceLock<Mutex<Vec<StarmieIlluminateMarkState>>> = OnceLock::new();
static ORBEETLE_AGILITY_CHAINS: OnceLock<Mutex<Vec<OrbeetleAgilityChainState>>> = OnceLock::new();
static BATTLE_BONDS: OnceLock<Mutex<Vec<BattleBondState>>> = OnceLock::new();
static SPIRIT_SHACKLES: OnceLock<Mutex<Vec<SpiritShackleState>>> = OnceLock::new();
static SOAKS: OnceLock<Mutex<Vec<SoakState>>> = OnceLock::new();
static WILL_O_WISP_CHARGES: OnceLock<Mutex<Vec<WillOWispChargeState>>> = OnceLock::new();
static STICKY_WEB_CHARGES: OnceLock<Mutex<Vec<StickyWebChargeState>>> = OnceLock::new();
static PORYGON_TYPES: OnceLock<Mutex<Vec<PorygonTypeState>>> = OnceLock::new();
static EEVEELUTIONS: OnceLock<Mutex<Vec<EeveelutionState>>> = OnceLock::new();
static AQUA_RINGS: OnceLock<Mutex<Vec<AquaRingState>>> = OnceLock::new();
static MISTY_TERRAINS: OnceLock<Mutex<Vec<MistyTerrainState>>> = OnceLock::new();
static BRINE_FIELDS: OnceLock<Mutex<Vec<BrineFieldState>>> = OnceLock::new();
static GRASSY_TERRAINS: OnceLock<Mutex<Vec<GrassyTerrainState>>> = OnceLock::new();
static RILLABOOM_DRUM_AURAS: OnceLock<Mutex<Vec<RillaboomDrumAuraState>>> = OnceLock::new();
static RILLABOOM_GRASSY_SURGES: OnceLock<Mutex<Vec<RillaboomGrassySurgeState>>> = OnceLock::new();
static SHIFTRY_TORNADOES: OnceLock<Mutex<Vec<ShiftryTornadoState>>> = OnceLock::new();
static SHIFTRY_FOREST_CAMOUFLAGES: OnceLock<Mutex<Vec<ShiftryForestCamouflageState>>> =
    OnceLock::new();
static RILLABOOM_DRUM_SOLOS: OnceLock<Mutex<Vec<RillaboomDrumSoloState>>> = OnceLock::new();
static SIGILYPH_GLYPHS: OnceLock<Mutex<Vec<SigilyphGlyphState>>> = OnceLock::new();
static SIGILYPH_GRAVITIES: OnceLock<Mutex<Vec<SigilyphGravityState>>> = OnceLock::new();
static WEAVILE_HUNTS: OnceLock<Mutex<Vec<WeavileHuntState>>> = OnceLock::new();
static AMPHAROS_SEARCHLIGHT_TAILS: OnceLock<Mutex<Vec<AmpharosSearchlightTailState>>> =
    OnceLock::new();
static AMPHAROS_LUMINOUS_PULSES: OnceLock<Mutex<Vec<AmpharosLuminousPulseState>>> = OnceLock::new();
static AMPHAROS_TRUE_SIGHTS: OnceLock<Mutex<Vec<AmpharosTrueSightState>>> = OnceLock::new();
static AMPHAROS_GIGAVOLTS: OnceLock<Mutex<Vec<AmpharosGigavoltState>>> = OnceLock::new();
static XATU_STILLNESSES: OnceLock<Mutex<Vec<XatuStillnessState>>> = OnceLock::new();
static XATU_PAIN_AMPLIFIERS: OnceLock<Mutex<Vec<XatuPainAmplifierState>>> = OnceLock::new();
static XATU_SUPER_PSYS: OnceLock<Mutex<Vec<XatuSuperPsyState>>> = OnceLock::new();
static QUAQUAVAL_AQUA_STEP_EMITTERS: OnceLock<Mutex<Vec<QuaquavalAquaStepEmitterState>>> =
    OnceLock::new();
static QUAQUAVAL_AQUA_STEP_SEGMENTS: OnceLock<Mutex<Vec<QuaquavalAquaStepSegmentState>>> =
    OnceLock::new();
static QUAQUAVAL_SPIRAL_SHOTS: OnceLock<Mutex<Vec<QuaquavalSpiralShotState>>> = OnceLock::new();
static QUAQUAVAL_EXCITING_DANCES: OnceLock<Mutex<Vec<QuaquavalExcitingDanceState>>> =
    OnceLock::new();
static ARCANINE_EXTREMESPEED_SHIELDS: OnceLock<Mutex<Vec<ArcanineExtremespeedShieldState>>> =
    OnceLock::new();
static ARCANINE_BLAZING_MANES: OnceLock<Mutex<Vec<ArcanineBlazingManeState>>> = OnceLock::new();
static WISHIWASHI_SCHOOLINGS: OnceLock<Mutex<Vec<WishiwashiSchoolingState>>> = OnceLock::new();
static WISHIWASHI_MASSIVE_CATCHES: OnceLock<Mutex<Vec<WishiwashiMassiveCatchState>>> =
    OnceLock::new();
static MISSINGNO_GLITCH_STORMS: OnceLock<Mutex<Vec<MissingNoGlitchStormState>>> = OnceLock::new();
static MISSINGNO_TRICK_ROOMS: OnceLock<Mutex<Vec<MissingNoTrickRoomState>>> = OnceLock::new();
static MISSINGNO_PENDING_DEBUFFS: OnceLock<Mutex<Vec<MissingNoPendingDebuffState>>> =
    OnceLock::new();
static SWANNA_TAILWINDS: OnceLock<Mutex<Vec<SwannaTailwindState>>> = OnceLock::new();
static SWANNA_CYCLONES: OnceLock<Mutex<Vec<SwannaCycloneState>>> = OnceLock::new();
static SWANNA_SKY_CIRCUSES: OnceLock<Mutex<Vec<SwannaSkyCircusState>>> = OnceLock::new();
static MAROWAK_BONE_WINDMILLS: OnceLock<Mutex<Vec<MarowakBoneWindmillState>>> = OnceLock::new();
static MAROWAK_WINDMILL_BONEMERANGS: OnceLock<Mutex<Vec<MarowakWindmillBonemerangState>>> =
    OnceLock::new();
static GARGANACL_SALT_PATCHES: OnceLock<Mutex<Vec<GarganaclSaltPatchState>>> = OnceLock::new();
static GARGANACL_BLESSED_SALTS: OnceLock<Mutex<Vec<GarganaclBlessedSaltState>>> = OnceLock::new();
static GARGANACL_PERMANENT_SALTS: OnceLock<Mutex<Vec<GarganaclPermanentSaltState>>> =
    OnceLock::new();
static STICKY_WEBS: OnceLock<Mutex<Vec<StickyWebState>>> = OnceLock::new();
static WEB_WALKER_SPOTS: OnceLock<Mutex<Vec<WebWalkerSpot>>> = OnceLock::new();
static WHIRLPOOLS: OnceLock<Mutex<Vec<WhirlpoolState>>> = OnceLock::new();
static CHARM_HEALS: OnceLock<Mutex<Vec<CharmHealState>>> = OnceLock::new();
static BLISSEY_HEAL_AURAS: OnceLock<Mutex<Vec<BlisseyHealAuraState>>> = OnceLock::new();
static KLEAVOR_SHARPNESS_HITS: OnceLock<Mutex<Vec<KleavorHitState>>> = OnceLock::new();
static KLEAVOR_STONE_AXE_HITS: OnceLock<Mutex<Vec<KleavorHitState>>> = OnceLock::new();
static STEALTH_ROCKS: OnceLock<Mutex<Vec<StealthRockState>>> = OnceLock::new();
static LIGHT_METALS: OnceLock<Mutex<Vec<LightMetalState>>> = OnceLock::new();
static SOFT_UNTARGETABLES: OnceLock<Mutex<Vec<SoftUntargetableState>>> = OnceLock::new();
static HAWLUCHA_COUNTERS: OnceLock<Mutex<Vec<HawluchaCounterState>>> = OnceLock::new();
static ZERAORA_ZING_ZAPS: OnceLock<Mutex<Vec<ZeraoraZingZapState>>> = OnceLock::new();
static ZERAORA_THUNDER_CAGES: OnceLock<Mutex<Vec<ZeraoraThunderCageState>>> = OnceLock::new();
static ZERAORA_MERCILESS_HITS: OnceLock<Mutex<Vec<ZeraoraMercilessHitState>>> = OnceLock::new();
static ZERAORA_MERCILESS_COOLDOWNS: OnceLock<Mutex<Vec<ZeraoraMercilessCooldownState>>> =
    OnceLock::new();
static ZERAORA_WILD_CHARGE_MARKS: OnceLock<Mutex<Vec<ZeraoraWildChargeMarkState>>> =
    OnceLock::new();
static HAWLUCHA_MOMENTUMS: OnceLock<Mutex<Vec<HawluchaMomentumState>>> = OnceLock::new();
static BOUFFALANT_RETALIATES: OnceLock<Mutex<Vec<BouffalantRetaliateState>>> = OnceLock::new();
static BOUFFALANT_CCS: OnceLock<Mutex<Vec<BouffalantCcState>>> = OnceLock::new();
static BOUFFALANT_UNSTOPPABLES: OnceLock<Mutex<Vec<BouffalantUnstoppableState>>> = OnceLock::new();
static BOUFFALANT_HEAD_CHARGE_MARKS: OnceLock<Mutex<Vec<BouffalantHeadChargeMarkState>>> =
    OnceLock::new();
static SNORLAX_GLUTTONIES: OnceLock<Mutex<Vec<SnorlaxGluttonyState>>> = OnceLock::new();
static CONFUSIONS: OnceLock<Mutex<Vec<ConfusionState>>> = OnceLock::new();
static DELAYED_CONFUSIONS: OnceLock<Mutex<Vec<DelayedConfusionState>>> = OnceLock::new();
static SING_AURAS: OnceLock<Mutex<Vec<SingAuraState>>> = OnceLock::new();
static FROSMOTH_SLEEP_CIRCLES: OnceLock<Mutex<Vec<FrosmothSleepCircleState>>> = OnceLock::new();
static BUG_BUZZ_AURAS: OnceLock<Mutex<Vec<BugBuzzAuraState>>> = OnceLock::new();
static ALLURING_VOICE_AURAS: OnceLock<Mutex<Vec<AlluringVoiceAuraState>>> = OnceLock::new();
static EARTHQUAKES: OnceLock<Mutex<Vec<EarthquakeAuraState>>> = OnceLock::new();
static BLOOD_MOONS: OnceLock<Mutex<Vec<BloodMoonState>>> = OnceLock::new();
static ROOSTS: OnceLock<Mutex<Vec<RoostState>>> = OnceLock::new();
static FLAME_TRAILS: OnceLock<Mutex<Vec<FlameTrailState>>> = OnceLock::new();
static SPEED_BOOSTS: OnceLock<Mutex<Vec<SpeedBoostState>>> = OnceLock::new();
static SAWK_THROH_FORMS: OnceLock<Mutex<Vec<SawkThrohState>>> = OnceLock::new();
static STURDIES: OnceLock<Mutex<Vec<SturdyState>>> = OnceLock::new();
static INTIMIDATE_AURAS: OnceLock<Mutex<Vec<IntimidateAuraState>>> = OnceLock::new();
static HOUNDOOM_DAMAGE: OnceLock<Mutex<Vec<HoundoomDamageState>>> = OnceLock::new();
static ARBOK_BASIC_HITS: OnceLock<Mutex<Vec<ArbokBasicHitState>>> = OnceLock::new();
static PYUKUMUKU_BARBS: OnceLock<Mutex<Vec<PyukumukuBarbState>>> = OnceLock::new();
static CLAWITZER_CLINGS: OnceLock<Mutex<Vec<ClawitzerClingState>>> = OnceLock::new();
static COMFEY_ATTACHES: OnceLock<Mutex<Vec<ComfeyAttachState>>> = OnceLock::new();
static PLAYER_RETURN_INTENTS: OnceLock<Mutex<Vec<PlayerReturnIntentState>>> = OnceLock::new();
static OCTILLERY_LOCK_ONS: OnceLock<Mutex<Vec<OctilleryLockOnState>>> = OnceLock::new();
static POKEMON_DAMAGE_CREDITS: OnceLock<Mutex<Vec<PokemonDamageCreditState>>> = OnceLock::new();
static POKEMON_KILL_CREDITS: OnceLock<Mutex<Vec<PokemonKillCreditState>>> = OnceLock::new();
static POKEMON_ASSIST_AWARDS: OnceLock<Mutex<Vec<PokemonAssistAwardState>>> = OnceLock::new();
static POKEMON_PARTICIPATIONS: OnceLock<Mutex<Vec<PokemonParticipationState>>> = OnceLock::new();
static POKEMON_COMBAT_STATS: OnceLock<Mutex<Vec<PokemonCombatStatState>>> = OnceLock::new();
static POKEMON_BASE_STAT_SYNC: OnceLock<Mutex<Vec<PokemonBaseStatSyncState>>> = OnceLock::new();
static POKEMON_BASE_STAT_SYNC_LOG_COUNT: OnceLock<Mutex<usize>> = OnceLock::new();
static POKEMON_PLAYER_IDENTITIES: OnceLock<Mutex<Vec<PokemonPlayerIdentityState>>> =
    OnceLock::new();
static LIGHT_SCREENS: OnceLock<Mutex<Vec<LightScreenState>>> = OnceLock::new();
static SMOKE_SCREENS: OnceLock<Mutex<Vec<SmokeScreenState>>> = OnceLock::new();
static DETECT_GUARDS: OnceLock<Mutex<Vec<DetectGuardState>>> = OnceLock::new();
static GRAPPLOCT_SUBMISSIONS: OnceLock<Mutex<Vec<GrapploctSubmissionState>>> = OnceLock::new();
static COALOSSAL_MAGMA_STORMS: OnceLock<Mutex<Vec<CoalossalMagmaStormState>>> = OnceLock::new();
static DRAMPA_BERSERKS: OnceLock<Mutex<Vec<DrampaBerserkState>>> = OnceLock::new();
static ARMAROUGE_MYSTICAL_FIRE_AURAS: OnceLock<Mutex<Vec<ArmarougeMysticalFireAuraState>>> =
    OnceLock::new();
static SHELL_ARMORS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static SCRAPPYS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static LIMBERS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static ENTITY_CHAMPIONS: OnceLock<Mutex<Vec<EntityChampionState>>> = OnceLock::new();
static RECEIVERS: OnceLock<Mutex<Vec<ReceiverState>>> = OnceLock::new();
static REVERSALS: OnceLock<Mutex<Vec<ReversalState>>> = OnceLock::new();
static HITMONTOPS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static KILOWATTRELS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static BEEHEEYEMS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static TECHNICIANS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static DOT_IMMUNES: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static DOT_ABSORBERS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static BLISSEYS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static SCIZORS: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
static SPIKED_HIDE_REFLECTS: OnceLock<Mutex<Vec<(usize, usize, usize)>>> = OnceLock::new();
static PHYSICAL_BASIC_HITS: OnceLock<Mutex<Vec<(usize, usize, usize)>>> = OnceLock::new();
static DIRECT_DAMAGE_LEDGER: OnceLock<Mutex<Vec<DirectDamageLedgerState>>> = OnceLock::new();
static TRACKED_BUFFS: OnceLock<Mutex<Vec<TrackedBuffState>>> = OnceLock::new();
static SYMBIOSIS_AURAS: OnceLock<Mutex<Vec<SymbiosisState>>> = OnceLock::new();
static NASTY_PLOTS: OnceLock<Mutex<Vec<NastyPlotState>>> = OnceLock::new();
static STORED_POWERS: OnceLock<Mutex<Vec<StoredPowerState>>> = OnceLock::new();
static HONEY_GATHERERS: OnceLock<Mutex<Vec<HoneyGathererState>>> = OnceLock::new();
static KOMMOO_DUELS: OnceLock<Mutex<Vec<KommooDuelState>>> = OnceLock::new();
static ANTI_HEALS: OnceLock<Mutex<Vec<AntiHealState>>> = OnceLock::new();
static THIEVUL_HONE_CLAWS: OnceLock<Mutex<Vec<ThievulHoneClawsState>>> = OnceLock::new();
static THIEVUL_HEAL_BLOCK_AURAS: OnceLock<Mutex<Vec<ThievulHealBlockAuraState>>> = OnceLock::new();
static THIEVUL_STAKEOUTS: OnceLock<Mutex<Vec<ThievulStakeoutState>>> = OnceLock::new();
static WIDE_GUARDS: OnceLock<Mutex<Vec<WideGuardState>>> = OnceLock::new();
static ARCHALUDON_IRON_DEFENSES: OnceLock<Mutex<Vec<ArchaludonIronDefenseState>>> = OnceLock::new();
static WONDER_GUARDS: OnceLock<Mutex<Vec<WonderGuardState>>> = OnceLock::new();
static SHEDINJA_KILLS: OnceLock<Mutex<Vec<ShedinjaKillState>>> = OnceLock::new();
static LUDICOLO_RAIN_DISHES: OnceLock<Mutex<Vec<LudicoloRainDishState>>> = OnceLock::new();
static KINGDRA_FOCUSES: OnceLock<Mutex<Vec<KingdraFocusState>>> = OnceLock::new();
static KINGDRA_DRAGON_DANCES: OnceLock<Mutex<Vec<KingdraDragonDanceState>>> = OnceLock::new();
static DELIBIRD_PRESENTS: OnceLock<Mutex<Vec<DelibirdPresentState>>> = OnceLock::new();
static LAST_UPDATE_TICK: OnceLock<Mutex<usize>> = OnceLock::new();

fn clear_vec_store<T>(store: &OnceLock<Mutex<Vec<T>>>) {
    if let Some(states) = store.get() {
        states
            .lock()
            .expect("pokemon runtime state poisoned")
            .clear();
    }
}

pub fn has_status_tick_rolled_back(tick: usize) -> bool {
    let last_update = LAST_UPDATE_TICK.get_or_init(|| Mutex::new(usize::MAX));
    let last_update = last_update.lock().expect("status update tick poisoned");
    *last_update != usize::MAX && tick < *last_update
}

pub fn reset_pokemon_status_runtime_state_for_new_match() {
    macro_rules! clear_all {
        ($($store:ident),* $(,)?) => {
            $(clear_vec_store(&$store);)*
        };
    }

    clear_all!(
        PARALYSIS,
        BURNS,
        POISONS,
        MIASMAS,
        BLEEDS,
        ARMAROUGE_WEAK_ARMORS,
        INFESTATIONS,
        FROZENS,
        ICE_FIELDS,
        LEECH_SEEDS,
        BLAZE_CONTACTS,
        ENTITY_TYPES,
        ENTITY_OWNERS,
        PLAYER_ENTITIES,
        MEGA_LAUNCHER,
        TANGLING_AURAS,
        HELPING_HAND_AURAS,
        WISH_CHANNELS,
        GHOLDENGO_GOLD,
        GALLADE_PREDICTIONS,
        JUSTIFIED_ENTITIES,
        AUDINO_PROTECTS,
        ENDURES,
        AUDINO_REGENERATORS,
        SMEARGLES,
        POWER_UP_PUNCH_COOLDOWNS,
        POWER_UP_PUNCH_CHANNELS,
        QUICK_RETURN_DASHES,
        SCHEDULED_FORCE_AWAYS,
        STARMIE_ILLUMINATES,
        STARMIE_ILLUMINATE_MARKS,
        ORBEETLE_AGILITY_CHAINS,
        BATTLE_BONDS,
        SPIRIT_SHACKLES,
        SOAKS,
        WILL_O_WISP_CHARGES,
        STICKY_WEB_CHARGES,
        PORYGON_TYPES,
        EEVEELUTIONS,
        AQUA_RINGS,
        MISTY_TERRAINS,
        BRINE_FIELDS,
        GRASSY_TERRAINS,
        RILLABOOM_DRUM_AURAS,
        RILLABOOM_GRASSY_SURGES,
        SHIFTRY_TORNADOES,
        SHIFTRY_FOREST_CAMOUFLAGES,
        RILLABOOM_DRUM_SOLOS,
        SIGILYPH_GLYPHS,
        SIGILYPH_GRAVITIES,
        WEAVILE_HUNTS,
        AMPHAROS_SEARCHLIGHT_TAILS,
        AMPHAROS_LUMINOUS_PULSES,
        AMPHAROS_TRUE_SIGHTS,
        AMPHAROS_GIGAVOLTS,
        XATU_STILLNESSES,
        XATU_PAIN_AMPLIFIERS,
        XATU_SUPER_PSYS,
        QUAQUAVAL_AQUA_STEP_EMITTERS,
        QUAQUAVAL_AQUA_STEP_SEGMENTS,
        QUAQUAVAL_SPIRAL_SHOTS,
        QUAQUAVAL_EXCITING_DANCES,
        ARCANINE_EXTREMESPEED_SHIELDS,
        ARCANINE_BLAZING_MANES,
        WISHIWASHI_SCHOOLINGS,
        WISHIWASHI_MASSIVE_CATCHES,
        MISSINGNO_GLITCH_STORMS,
        MISSINGNO_TRICK_ROOMS,
        MISSINGNO_PENDING_DEBUFFS,
        YANMEGA_TINTED_LENSES,
        YANMEGA_GIGA_DRAINS,
        POKEMON_CC_EVENTS,
        SWANNA_TAILWINDS,
        SWANNA_CYCLONES,
        SWANNA_SKY_CIRCUSES,
        MAROWAK_BONE_WINDMILLS,
        MAROWAK_WINDMILL_BONEMERANGS,
        GARGANACL_SALT_PATCHES,
        GARGANACL_BLESSED_SALTS,
        GARGANACL_PERMANENT_SALTS,
        STICKY_WEBS,
        WEB_WALKER_SPOTS,
        WHIRLPOOLS,
        CHARM_HEALS,
        BLISSEY_HEAL_AURAS,
        KLEAVOR_SHARPNESS_HITS,
        KLEAVOR_STONE_AXE_HITS,
        STEALTH_ROCKS,
        LIGHT_METALS,
        SOFT_UNTARGETABLES,
        HAWLUCHA_COUNTERS,
        ZERAORA_ZING_ZAPS,
        ZERAORA_THUNDER_CAGES,
        ZERAORA_MERCILESS_HITS,
        ZERAORA_MERCILESS_COOLDOWNS,
        ZERAORA_WILD_CHARGE_MARKS,
        HAWLUCHA_MOMENTUMS,
        BOUFFALANT_RETALIATES,
        BOUFFALANT_CCS,
        BOUFFALANT_UNSTOPPABLES,
        BOUFFALANT_HEAD_CHARGE_MARKS,
        SNORLAX_GLUTTONIES,
        CONFUSIONS,
        DELAYED_CONFUSIONS,
        SING_AURAS,
        FROSMOTH_SLEEP_CIRCLES,
        BUG_BUZZ_AURAS,
        ALLURING_VOICE_AURAS,
        EARTHQUAKES,
        BLOOD_MOONS,
        ROOSTS,
        FLAME_TRAILS,
        SPEED_BOOSTS,
        SAWK_THROH_FORMS,
        STURDIES,
        INTIMIDATE_AURAS,
        HOUNDOOM_DAMAGE,
        ARBOK_BASIC_HITS,
        PYUKUMUKU_BARBS,
        CLAWITZER_CLINGS,
        COMFEY_ATTACHES,
        PLAYER_RETURN_INTENTS,
        OCTILLERY_LOCK_ONS,
        LIGHT_SCREENS,
        SMOKE_SCREENS,
        DETECT_GUARDS,
        GRAPPLOCT_SUBMISSIONS,
        COALOSSAL_MAGMA_STORMS,
        DRAMPA_BERSERKS,
        ARMAROUGE_MYSTICAL_FIRE_AURAS,
        SHELL_ARMORS,
        SCRAPPYS,
        LIMBERS,
        ENTITY_CHAMPIONS,
        RECEIVERS,
        REVERSALS,
        HITMONTOPS,
        KILOWATTRELS,
        BEEHEEYEMS,
        TECHNICIANS,
        DOT_IMMUNES,
        DOT_ABSORBERS,
        BLISSEYS,
        SCIZORS,
        SPIKED_HIDE_REFLECTS,
        PHYSICAL_BASIC_HITS,
        DIRECT_DAMAGE_LEDGER,
        POKEMON_DAMAGE_CREDITS,
        POKEMON_KILL_CREDITS,
        POKEMON_ASSIST_AWARDS,
        POKEMON_PARTICIPATIONS,
        TRACKED_BUFFS,
        SYMBIOSIS_AURAS,
        NASTY_PLOTS,
        STORED_POWERS,
        HONEY_GATHERERS,
        KOMMOO_DUELS,
        ANTI_HEALS,
        THIEVUL_HONE_CLAWS,
        THIEVUL_HEAL_BLOCK_AURAS,
        THIEVUL_STAKEOUTS,
        WIDE_GUARDS,
        ARCHALUDON_IRON_DEFENSES,
        WONDER_GUARDS,
        SHEDINJA_KILLS,
        LUDICOLO_RAIN_DISHES,
        KINGDRA_FOCUSES,
        KINGDRA_DRAGON_DANCES,
        DELIBIRD_PRESENTS,
    );

    let last_update = LAST_UPDATE_TICK.get_or_init(|| Mutex::new(usize::MAX));
    *last_update.lock().expect("status update tick poisoned") = usize::MAX;
}

pub fn add_tracked_buff(
    ctx: &mut GameCtx,
    mut source_id: usize,
    target_id: usize,
    mut buff: BuffState,
    mut polarity: PokemonBuffPolarity,
) {
    if is_shedinja_entity(target_id) {
        buff.hp = 0;
        buff.hp_mult = 0;
    }
    if polarity == PokemonBuffPolarity::Beneficial {
        if let Some(missingno_id) = trick_room_inverts_allied_effect(ctx, source_id, target_id) {
            invert_buff_state(&mut buff);
            polarity = PokemonBuffPolarity::Harmful;
            source_id = missingno_id;
        } else if source_id != target_id && trick_room_doubles_missingno_buffs(ctx, target_id) {
            double_positive_buff_state(&mut buff);
        }
    }
    let duration_ticks = buff_duration_ticks(&buff);
    let transferable = can_transfer_buff(&buff);
    ctx.add_buff(target_id, buff.clone());

    if polarity == PokemonBuffPolarity::Neutral {
        return;
    }

    if let (Some(source_info), Some(target_info)) = (
        combat_stat_source_entity_info(ctx, source_id),
        combat_stat_target_entity_info(ctx, target_id),
    ) {
        match polarity {
            PokemonBuffPolarity::Harmful => record_pokemon_enemy_participation(
                ctx,
                source_info,
                target_info,
                PokemonParticipationKind::EnemyHarmfulBuff,
            ),
            PokemonBuffPolarity::Beneficial => record_pokemon_ally_participation(
                ctx,
                source_info,
                target_info,
                PokemonParticipationKind::AllyBeneficialBuff,
            ),
            PokemonBuffPolarity::Neutral => {}
        }
    }

    let Some(duration_ticks) = duration_ticks else {
        return;
    };
    if duration_ticks == 0 {
        return;
    }

    let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("tracked buff ledger poisoned");
    states.push(TrackedBuffState {
        source_id,
        target_id,
        buff,
        polarity,
        expires_at: ctx.tick().saturating_add(duration_ticks),
        transferable,
    });
}

pub fn add_beneficial_buff(ctx: &mut GameCtx, source_id: usize, target_id: usize, buff: BuffState) {
    add_tracked_buff(
        ctx,
        source_id,
        target_id,
        buff,
        PokemonBuffPolarity::Beneficial,
    );
}

pub fn add_harmful_buff(ctx: &mut GameCtx, source_id: usize, target_id: usize, buff: BuffState) {
    add_tracked_buff(
        ctx,
        source_id,
        target_id,
        buff,
        PokemonBuffPolarity::Harmful,
    );
}

pub fn tracked_bonus_attack_speed_mult(ctx: &GameCtx, target_id: usize) -> usize {
    prune_tracked_buffs(ctx);
    let tick = ctx.tick();
    let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("tracked buff ledger poisoned");
    states
        .iter()
        .filter(|state| {
            state.target_id == target_id
                && state.polarity == PokemonBuffPolarity::Beneficial
                && state.expires_at > tick
                && state.buff.attack_speed_mult > 0
        })
        .map(|state| state.buff.attack_speed_mult as usize)
        .sum()
}

pub fn tracked_bonus_move_speed_mult(ctx: &GameCtx, target_id: usize) -> usize {
    prune_tracked_buffs(ctx);
    let tick = ctx.tick();
    let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("tracked buff ledger poisoned");
    states
        .iter()
        .filter(|state| {
            state.target_id == target_id
                && state.polarity == PokemonBuffPolarity::Beneficial
                && state.expires_at > tick
                && state.buff.move_speed_mult > 0
        })
        .map(|state| state.buff.move_speed_mult as usize)
        .sum()
}

pub fn heart_swap_tracked_buffs(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
) -> (usize, usize) {
    prune_tracked_buffs(ctx);
    let tick = ctx.tick();
    let mut moved_harmful = Vec::new();
    let mut moved_beneficial = Vec::new();

    {
        let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
        let states = states.lock().expect("tracked buff ledger poisoned");
        for state in states.iter() {
            if !state.transferable || state.expires_at <= tick {
                continue;
            }
            if state.target_id == caster_id && state.polarity == PokemonBuffPolarity::Harmful {
                moved_harmful.push(state.clone());
            } else if state.target_id == target_id
                && state.polarity == PokemonBuffPolarity::Beneficial
            {
                moved_beneficial.push(state.clone());
            }
        }
    }

    remove_transferred_entries(ctx, caster_id, target_id, &moved_harmful, &moved_beneficial);

    for state in &moved_harmful {
        let moved = buff_with_remaining_duration(state.buff.clone(), state.expires_at, tick);
        neutralize_tracked_buff(ctx, state.target_id, &moved);
        add_harmful_buff(ctx, caster_id, target_id, moved);
    }
    for state in &moved_beneficial {
        let moved = buff_with_remaining_duration(state.buff.clone(), state.expires_at, tick);
        neutralize_tracked_buff(ctx, state.target_id, &moved);
        add_beneficial_buff(ctx, caster_id, caster_id, moved);
    }

    (moved_harmful.len(), moved_beneficial.len())
}

pub fn heart_swap_custom_states(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
) -> (usize, usize) {
    let tick = ctx.tick();
    let mut harmful_moved = 0usize;
    let mut beneficial_stolen = 0usize;

    harmful_moved += transfer_paralysis(ctx, caster_id, target_id, tick);
    harmful_moved += transfer_burns(caster_id, target_id, tick);
    harmful_moved += transfer_poisons(caster_id, target_id, tick);
    harmful_moved += transfer_miasmas(caster_id, target_id, tick);
    harmful_moved += transfer_bleeds(caster_id, target_id, tick);
    harmful_moved += transfer_infestations(caster_id, target_id, tick);
    harmful_moved += transfer_frozen(ctx, caster_id, target_id, tick);
    harmful_moved += transfer_confusions(caster_id, target_id, tick);
    harmful_moved += transfer_entity_timed_states(&SPIRIT_SHACKLES, caster_id, target_id, tick);
    harmful_moved += transfer_entity_timed_states(&SOAKS, caster_id, target_id, tick);
    harmful_moved += transfer_anti_heals(caster_id, target_id, tick);
    harmful_moved += transfer_leech_seeds(caster_id, target_id, tick);
    harmful_moved += transfer_octillery_lock_ons(caster_id, target_id, tick);
    harmful_moved += transfer_nasty_plots(caster_id, target_id, tick);

    beneficial_stolen += transfer_reversals(target_id, caster_id, tick);
    beneficial_stolen += transfer_endures(target_id, caster_id, tick);
    beneficial_stolen += transfer_audino_protects(target_id, caster_id, tick);
    beneficial_stolen += transfer_soft_untargetables(target_id, caster_id, tick);
    beneficial_stolen += transfer_hawlucha_counters(target_id, caster_id, tick);
    beneficial_stolen += transfer_detect_guards(target_id, caster_id, tick);
    beneficial_stolen += transfer_charm_heals(target_id, caster_id, tick);
    beneficial_stolen += transfer_gallade_predictions(target_id, caster_id, tick);

    if harmful_moved > 0 {
        if let Some(pos) = ctx.get_entity(target_id).map(|entity| entity.pos()) {
            draw_status_marker(ctx, pos, 14000, VFX_PSYCHIC);
        }
    }
    if beneficial_stolen > 0 {
        if let Some(pos) = ctx.get_entity(caster_id).map(|entity| entity.pos()) {
            draw_status_marker(ctx, pos, 14000, VFX_PSYCHIC);
        }
    }

    (harmful_moved, beneficial_stolen)
}

fn transfer_paralysis(ctx: &mut GameCtx, from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = PARALYSIS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("paralysis state poisoned");
    let mut moved = 0usize;
    for state in states.iter_mut() {
        if state.entity_id == from_id && state.expires_at > tick {
            state.entity_id = to_id;
            state.next_roll_at = state.next_roll_at.max(tick + PARALYSIS_ROLL_INTERVAL_TICKS);
            moved += 1;
        }
    }
    drop(states);
    if moved > 0 {
        synchronize_paralysis(ctx, from_id, to_id, 1);
    }
    moved
}

fn transfer_burns(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = BURNS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("burn state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_poisons(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = POISONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("poison state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_miasmas(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = MIASMAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("miasma state poisoned");
    let mut moved = Vec::new();
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if state.target_id == from_id {
            moved.push(*state);
            return false;
        }
        true
    });
    let moved_count = moved.len();
    for mut moved_state in moved {
        moved_state.target_id = to_id;
        if let Some(existing) = states
            .iter_mut()
            .find(|state| state.caster_id == moved_state.caster_id && state.target_id == to_id)
        {
            existing.stacks = existing.stacks.saturating_add(moved_state.stacks);
            existing.poison_awards = existing
                .poison_awards
                .saturating_add(moved_state.poison_awards);
            existing.poison_damage_per_tick = existing
                .poison_damage_per_tick
                .max(moved_state.poison_damage_per_tick);
            existing.expires_at = existing.expires_at.max(moved_state.expires_at);
        } else {
            states.push(moved_state);
        }
    }
    moved_count
}

fn transfer_bleeds(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = BLEEDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("bleed state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_infestations(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = INFESTATIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("infestation state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_frozen(ctx: &mut GameCtx, from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = FROZENS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("frozen state poisoned");
    let mut max_remaining = 0usize;
    let mut moved = 0usize;
    for state in states.iter_mut() {
        if state.entity_id == from_id && state.expires_at > tick {
            max_remaining = max_remaining.max(state.expires_at.saturating_sub(tick));
            state.entity_id = to_id;
            moved += 1;
        }
    }
    drop(states);
    if max_remaining > 0 {
        apply_pokemon_cc(
            ctx,
            to_id,
            to_id,
            CCState::Stun {
                tick: max_remaining as u64,
            },
        );
        apply_pokemon_cc(
            ctx,
            to_id,
            to_id,
            CCState::BlockSkill {
                tick: max_remaining,
            },
        );
        note_steadfast_cc(ctx, to_id);
    }
    moved
}

fn transfer_confusions(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = CONFUSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("confusion state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.entity_id == from_id && state.expires_at > tick {
                Some((&mut state.entity_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_entity_timed_states<T>(
    store: &OnceLock<Mutex<Vec<T>>>,
    from_id: usize,
    to_id: usize,
    tick: usize,
) -> usize
where
    T: EntityTimedState,
{
    let states = store.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("entity timed state poisoned");
    let mut moved = 0usize;
    for state in states.iter_mut() {
        if state.entity_id() == from_id && state.expires_at() > tick {
            state.set_entity_id(to_id);
            moved += 1;
        }
    }
    moved
}

trait EntityTimedState {
    fn entity_id(&self) -> usize;
    fn set_entity_id(&mut self, entity_id: usize);
    fn expires_at(&self) -> usize;
}

impl EntityTimedState for SpiritShackleState {
    fn entity_id(&self) -> usize {
        self.entity_id
    }
    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
    fn expires_at(&self) -> usize {
        self.expires_at
    }
}

impl EntityTimedState for SoakState {
    fn entity_id(&self) -> usize {
        self.entity_id
    }
    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
    fn expires_at(&self) -> usize {
        self.expires_at
    }
}

fn transfer_anti_heals(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = ANTI_HEALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("anti-heal state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_leech_seeds(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = LEECH_SEEDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("leech seed state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_octillery_lock_ons(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = OCTILLERY_LOCK_ONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("octillery lock-on state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_nasty_plots(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = NASTY_PLOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("nasty plot state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.resolves_at > tick {
                Some((&mut state.target_id, state.resolves_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_reversals(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = REVERSALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("reversal state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.entity_id == from_id && state.expires_at > tick {
                Some((&mut state.entity_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_endures(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = ENDURES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("endure state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_audino_protects(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = AUDINO_PROTECTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino protect state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_soft_untargetables(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = SOFT_UNTARGETABLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("soft untargetable state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.entity_id == from_id && state.expires_at > tick {
                Some((&mut state.entity_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_hawlucha_counters(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = HAWLUCHA_COUNTERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("hawlucha counter state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.entity_id == from_id && state.expires_at > tick {
                Some((&mut state.entity_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_detect_guards(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = DETECT_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("detect guard state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.entity_id == from_id && state.expires_at > tick {
                Some((&mut state.entity_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_charm_heals(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = CHARM_HEALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("charm heal state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.target_id == from_id && state.expires_at > tick {
                Some((&mut state.target_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn transfer_gallade_predictions(from_id: usize, to_id: usize, tick: usize) -> usize {
    let states = GALLADE_PREDICTIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("gallade prediction state poisoned");
    rekey_target_states(
        states.iter_mut().filter_map(|state| {
            if state.gallade_id == from_id && state.expires_at > tick {
                Some((&mut state.gallade_id, state.expires_at))
            } else {
                None
            }
        }),
        to_id,
    )
}

fn rekey_target_states<'a>(
    states: impl Iterator<Item = (&'a mut usize, usize)>,
    to_id: usize,
) -> usize {
    let mut moved = 0usize;
    for (entity_id, _) in states {
        *entity_id = to_id;
        moved += 1;
    }
    moved
}

fn remove_transferred_entries(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    harmful: &[TrackedBuffState],
    beneficial: &[TrackedBuffState],
) {
    if harmful.is_empty() && beneficial.is_empty() {
        return;
    }

    let tick = ctx.tick();
    let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("tracked buff ledger poisoned");
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let is_harmful_transfer = state.target_id == caster_id
            && state.polarity == PokemonBuffPolarity::Harmful
            && harmful
                .iter()
                .any(|moved| tracked_buff_matches(state, moved));
        let is_beneficial_transfer = state.target_id == target_id
            && state.polarity == PokemonBuffPolarity::Beneficial
            && beneficial
                .iter()
                .any(|moved| tracked_buff_matches(state, moved));
        !(is_harmful_transfer || is_beneficial_transfer)
    });
}

fn tracked_buff_matches(left: &TrackedBuffState, right: &TrackedBuffState) -> bool {
    left.source_id == right.source_id
        && left.target_id == right.target_id
        && left.expires_at == right.expires_at
        && left.polarity == right.polarity
}

fn prune_tracked_buffs(ctx: &GameCtx) {
    let tick = ctx.tick();
    let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("tracked buff ledger poisoned");
    states.retain(|state| {
        state.expires_at > tick
            && ctx
                .get_entity(state.target_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
}

fn buff_duration_ticks(buff: &BuffState) -> Option<usize> {
    match buff.duration.clone() {
        BuffType::Time { tick } => Some(tick),
        _ => None,
    }
}

fn buff_with_remaining_duration(buff: BuffState, expires_at: usize, tick: usize) -> BuffState {
    BuffState {
        duration: BuffType::Time {
            tick: expires_at.saturating_sub(tick).max(1),
        },
        ..buff
    }
}

fn neutralize_tracked_buff(ctx: &mut GameCtx, target_id: usize, buff: &BuffState) {
    let counter = inverse_buff(buff);
    if has_transferable_numeric_effect(&counter) {
        ctx.add_buff(target_id, counter);
    }
}

fn inverse_buff(buff: &BuffState) -> BuffState {
    BuffState {
        duration: buff.duration.clone(),
        attack: -buff.attack,
        magic_power: -buff.magic_power,
        hp: -buff.hp,
        defence: -buff.defence,
        magic_resistance: -buff.magic_resistance,
        attack_mult: -buff.attack_mult,
        magic_power_mult: -buff.magic_power_mult,
        hp_mult: -buff.hp_mult,
        defence_mult: -buff.defence_mult,
        magic_resistance_mult: -buff.magic_resistance_mult,
        attack_speed_mult: -buff.attack_speed_mult,
        move_speed_mult: -buff.move_speed_mult,
        crit_chance: -buff.crit_chance,
        vamp: -buff.vamp,
        skill_cooldown_mult: -buff.skill_cooldown_mult,
        ult_cooldown_mult: -buff.ult_cooldown_mult,
        ..Default::default()
    }
}

fn can_transfer_buff(buff: &BuffState) -> bool {
    has_transferable_numeric_effect(buff)
        && buff.damaged_reduce == 0
        && buff.damaged_amplify == 0
        && buff.heal_reduce == 0
        && !buff.cc_immune
}

fn has_transferable_numeric_effect(buff: &BuffState) -> bool {
    buff.attack != 0
        || buff.magic_power != 0
        || buff.hp != 0
        || buff.defence != 0
        || buff.magic_resistance != 0
        || buff.attack_mult != 0
        || buff.magic_power_mult != 0
        || buff.hp_mult != 0
        || buff.defence_mult != 0
        || buff.magic_resistance_mult != 0
        || buff.attack_speed_mult != 0
        || buff.move_speed_mult != 0
        || buff.crit_chance != 0
        || buff.vamp != 0
        || buff.skill_cooldown_mult != 0
        || buff.ult_cooldown_mult != 0
}

pub fn register_entity_types(entity_id: usize, types: TypeSet) {
    let states = ENTITY_TYPES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("entity type state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.types = types;
        return;
    }
    states.push(EntityTypeState { entity_id, types });
}

pub fn register_entity_champion(ctx: &GameCtx, entity_id: usize, champion_id: &'static str) {
    let ctx_id = combat_ctx_id(ctx);
    let states = ENTITY_CHAMPIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("entity champion state poisoned");
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.ctx_id == ctx_id && state.entity_id == entity_id)
    {
        existing.champion_id = champion_id;
        drop(states);
        refresh_pokemon_combat_stat_champion(ctx, entity_id, champion_id);
        refresh_entity_owner_champion(ctx, entity_id, champion_id);
        return;
    }
    states.push(EntityChampionState {
        ctx_id,
        entity_id,
        champion_id,
    });
    while states.len() > POKEMON_CONTEXT_LEDGER_MAX {
        states.remove(0);
    }
    drop(states);
    refresh_pokemon_combat_stat_champion(ctx, entity_id, champion_id);
    refresh_entity_owner_champion(ctx, entity_id, champion_id);
}

fn refresh_pokemon_combat_stat_champion(
    ctx: &GameCtx,
    entity_id: usize,
    champion_id: &'static str,
) {
    let ctx_id = combat_ctx_id(ctx);
    let Some(states) = POKEMON_COMBAT_STATS.get() else {
        return;
    };
    if let Ok(mut states) = states.lock() {
        if let Some(state) = states
            .iter_mut()
            .find(|state| state.ctx_id == ctx_id && state.entity_id == entity_id)
        {
            state.champion_id = Some(champion_id);
        }
    }
}

fn refresh_entity_owner_champion(ctx: &GameCtx, entity_id: usize, champion_id: &'static str) {
    let ctx_id = combat_ctx_id(ctx);
    let Some(states) = ENTITY_OWNERS.get() else {
        return;
    };
    if let Ok(mut states) = states.lock() {
        for state in states
            .iter_mut()
            .filter(|state| state.ctx_id == ctx_id && state.entity_id == entity_id)
        {
            state.champion_id = Some(champion_id);
        }
    }
}

pub fn champion_id_for_entity(entity_id: usize) -> Option<&'static str> {
    ENTITY_CHAMPIONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("entity champion state poisoned")
        .iter()
        .rev()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.champion_id)
}

fn champion_id_for_entity_in_ctx(ctx: &GameCtx, entity_id: usize) -> Option<&'static str> {
    let ctx_id = combat_ctx_id(ctx);
    ENTITY_CHAMPIONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("entity champion state poisoned")
        .iter()
        .rev()
        .find(|state| state.ctx_id == ctx_id && state.entity_id == entity_id)
        .map(|state| state.champion_id)
}

fn entity_is_champion_id(entity_id: usize, champion_id: &str) -> bool {
    champion_id_for_entity(entity_id)
        .map(|current_champion_id| current_champion_id == champion_id)
        .unwrap_or(false)
}

pub fn is_shedinja_entity(entity_id: usize) -> bool {
    entity_is_champion_id(entity_id, "pokemon_moba_shedinja")
        || receiver_has_copied(entity_id, "pokemon_moba_shedinja")
}

pub fn register_wonder_guard(entity_id: usize) {
    let states = WONDER_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("wonder guard state poisoned");
    if states.iter().any(|state| state.shedinja_id == entity_id) {
        return;
    }
    states.push(WonderGuardState {
        shedinja_id: entity_id,
        shell_broken_until: 0,
        knocked_until: 0,
        chits: 0,
    });
}

pub fn update_wonder_guard(ctx: &mut GameCtx, entity_id: usize) {
    register_wonder_guard(entity_id);
    let tick = ctx.tick();
    let states = WONDER_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("wonder guard state poisoned");
    let Some(state) = states
        .iter_mut()
        .find(|state| state.shedinja_id == entity_id)
    else {
        return;
    };
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        state.shell_broken_until = 0;
        state.knocked_until = 0;
        state.chits = 0;
        return;
    }
    let pos = entity.pos();
    drop(entity);

    if state.knocked_until > 0 && tick >= state.knocked_until {
        state.knocked_until = 0;
        state.chits = 0;
        state.shell_broken_until = 0;
    }

    if state.knocked_until > tick {
        ctx.debug_draw_circle(pos.x, pos.y, 15000, VFX_PSYCHIC);
    } else if state.shell_broken_until > tick {
        apply_wonder_guard_engine_block(ctx, entity_id);
        ctx.debug_draw_circle(pos.x, pos.y, 12000, VFX_BUG);
    } else {
        apply_wonder_guard_engine_block(ctx, entity_id);
    }
}

fn apply_wonder_guard_engine_block(ctx: &mut GameCtx, entity_id: usize) {
    ctx.add_buff(
        entity_id,
        BuffState {
            duration: BuffType::Time { tick: 3 },
            base_attack_damaged_reduce: 100,
            skill_damaged_reduce: 100,
            ..Default::default()
        },
    );
}

pub fn grudge_kill_count(shedinja_id: usize, target_id: usize) -> usize {
    SHEDINJA_KILLS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("shedinja kill state poisoned")
        .iter()
        .find(|state| state.shedinja_id == shedinja_id && state.killer_id == target_id)
        .map(|state| state.kills)
        .unwrap_or(0)
}

pub fn try_wonder_guard_damage(
    ctx: &mut GameCtx,
    attacker_id: usize,
    target_id: usize,
    attack_type: AttackType,
    move_type: PokemonType,
    defender_types: TypeSet,
) -> bool {
    if !is_shedinja_entity(target_id) {
        return false;
    }
    register_wonder_guard(target_id);

    let tick = ctx.tick();
    let states = WONDER_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("wonder guard state poisoned");
    let Some(state) = states
        .iter_mut()
        .find(|state| state.shedinja_id == target_id)
    else {
        return false;
    };

    if state.knocked_until > tick {
        if state.chits > 0 {
            state.chits = state.chits.saturating_sub(1);
        }
        if state.chits == 0 {
            note_shedinja_killed_by(target_id, attacker_id);
            drop(states);
            crate::pokemon_status::deal_tracked_damage(
                ctx,
                attacker_id,
                target_id,
                1_000_000,
                0,
                AttackType::Item,
            );
        }
        return true;
    }

    let super_effective = crate::pokemon_types::type_modifier_ratio(move_type, defender_types).0
        > crate::pokemon_types::type_modifier_ratio(move_type, defender_types).1;
    if matches!(attack_type, AttackType::BaseAttack) || !super_effective {
        return true;
    }

    if state.shell_broken_until > tick {
        state.knocked_until = tick.saturating_add(7 * 60);
        state.chits = 5;
        apply_pokemon_cc(ctx, target_id, target_id, CCState::Bind { tick: 7 * 60 });
        apply_pokemon_cc(
            ctx,
            target_id,
            target_id,
            CCState::BlockSkill { tick: 7 * 60 },
        );
        apply_pokemon_cc(
            ctx,
            target_id,
            target_id,
            CCState::BlockAttack { tick: 7 * 60 },
        );
    } else {
        state.shell_broken_until = tick.saturating_add(5 * 60);
    }
    true
}

pub fn try_wonder_guard_dot_tick(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    move_type: PokemonType,
) -> bool {
    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
    if is_shedinja_entity(target_id) && move_type != PokemonType::Fire {
        let tick = ctx.tick();
        let states = WONDER_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
        let states = states.lock().expect("wonder guard state poisoned");
        if states
            .iter()
            .find(|state| state.shedinja_id == target_id)
            .map(|state| state.knocked_until <= tick)
            .unwrap_or(true)
        {
            return true;
        }
    }
    try_wonder_guard_damage(
        ctx,
        caster_id,
        target_id,
        AttackType::Dot,
        move_type,
        defender_types,
    )
}

fn note_shedinja_killed_by(shedinja_id: usize, killer_id: usize) {
    let states = SHEDINJA_KILLS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("shedinja kill state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.shedinja_id == shedinja_id && state.killer_id == killer_id)
    {
        state.kills = state.kills.saturating_add(1);
        return;
    }
    states.push(ShedinjaKillState {
        shedinja_id,
        killer_id,
        kills: 1,
    });
}

pub fn register_receiver(entity_id: usize) {
    let states = RECEIVERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("receiver state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.copied_champion_id = None;
        return;
    }
    states.push(ReceiverState {
        entity_id,
        copied_champion_id: None,
    });
}

pub fn register_honey_gatherer(ctx: &GameCtx, entity_id: usize) {
    let states = HONEY_GATHERERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("honey gatherer state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.stacks = 0;
        existing.next_stack_at = ctx.tick().saturating_add(HONEY_GATHER_INTERVAL_TICKS);
        return;
    }
    states.push(HoneyGathererState {
        entity_id,
        stacks: 0,
        next_stack_at: ctx.tick().saturating_add(HONEY_GATHER_INTERVAL_TICKS),
    });
}

pub fn update_honey_gatherer(ctx: &GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    drop(entity);

    let tick = ctx.tick();
    let states = HONEY_GATHERERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("honey gatherer state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(HoneyGathererState {
            entity_id,
            stacks: 0,
            next_stack_at: tick.saturating_add(HONEY_GATHER_INTERVAL_TICKS),
        });
        return;
    };
    while state.next_stack_at <= tick {
        state.stacks = state.stacks.saturating_add(1).min(HONEY_GATHER_MAX_STACKS);
        state.next_stack_at = state
            .next_stack_at
            .saturating_add(HONEY_GATHER_INTERVAL_TICKS);
    }
}

pub fn consume_honey_stacks(entity_id: usize) -> usize {
    let states = HONEY_GATHERERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("honey gatherer state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return 0;
    };
    let stacks = state.stacks;
    state.stacks = 0;
    stacks
}

pub fn update_starmie_illuminate_passive(ctx: &GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    drop(entity);

    let tick = ctx.tick();
    let states = STARMIE_ILLUMINATES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("starmie illuminate state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(StarmieIlluminateState {
            entity_id,
            charges: 0,
            next_charge_at: tick.saturating_add(STARMIE_ILLUMINATE_CHARGE_INTERVAL_TICKS),
        });
        return;
    };
    while state.next_charge_at <= tick {
        state.charges = state.charges.saturating_add(1).min(1);
        state.next_charge_at = state
            .next_charge_at
            .saturating_add(STARMIE_ILLUMINATE_CHARGE_INTERVAL_TICKS);
    }
}

pub fn note_starmie_damage_hit(ctx: &GameCtx, attacker_id: usize, target_id: usize) {
    let has_illuminate = champion_id_for_entity(attacker_id)
        .map(|champion_id| champion_id == "pokemon_moba_starmie")
        .unwrap_or(false)
        || receiver_has_copied(attacker_id, "pokemon_moba_starmie");
    if !has_illuminate {
        return;
    }

    let charges = STARMIE_ILLUMINATES.get_or_init(|| Mutex::new(Vec::new()));
    let mut charges = charges.lock().expect("starmie illuminate state poisoned");
    let Some(charge_state) = charges
        .iter_mut()
        .find(|state| state.entity_id == attacker_id)
    else {
        return;
    };
    if charge_state.charges == 0 {
        return;
    }
    charge_state.charges = charge_state.charges.saturating_sub(1);
    drop(charges);

    let tick = ctx.tick();
    let marks = STARMIE_ILLUMINATE_MARKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut marks = marks
        .lock()
        .expect("starmie illuminate mark state poisoned");
    if let Some(existing) = marks
        .iter_mut()
        .find(|state| state.caster_id == attacker_id && state.target_id == target_id)
    {
        existing.expires_at = tick.saturating_add(STARMIE_ILLUMINATE_MARK_TICKS);
    } else {
        marks.push(StarmieIlluminateMarkState {
            caster_id: attacker_id,
            target_id,
            expires_at: tick.saturating_add(STARMIE_ILLUMINATE_MARK_TICKS),
        });
    }
}

pub fn is_illuminated_by(ctx: &GameCtx, caster_id: usize, target_id: usize) -> bool {
    let tick = ctx.tick();
    STARMIE_ILLUMINATE_MARKS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("starmie illuminate mark state poisoned")
        .iter()
        .any(|state| {
            state.caster_id == caster_id && state.target_id == target_id && state.expires_at > tick
        })
}

pub fn schedule_force_away_from_pos(
    ctx: &GameCtx,
    entity_id: usize,
    away_from: EntityPos,
    delay_ticks: usize,
    speed: u64,
    ticks: u64,
) {
    let states = SCHEDULED_FORCE_AWAYS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("scheduled force-away state poisoned");
    states.retain(|state| state.entity_id != entity_id);
    states.push(ScheduledForceAwayState {
        entity_id,
        away_from,
        trigger_at: ctx.tick().saturating_add(delay_ticks),
        speed,
        ticks,
    });
}

pub fn receiver_copied_champion_id(entity_id: usize) -> Option<&'static str> {
    RECEIVERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("receiver state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .and_then(|state| state.copied_champion_id)
}

pub fn receiver_has_copied(entity_id: usize, champion_id: &str) -> bool {
    receiver_copied_champion_id(entity_id)
        .map(|copied| copied == champion_id)
        .unwrap_or(false)
}

pub fn has_kings_rock(entity_id: usize) -> bool {
    champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_kingdra")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_kingdra")
}

pub fn has_dragon_launcher(entity_id: usize) -> bool {
    champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_dragapult")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_dragapult")
}

pub fn kingdra_focus_crit_bonus(ctx: &GameCtx, caster_id: usize, target_id: usize) -> usize {
    if !has_kings_rock(caster_id) {
        return 0;
    }
    let tick = ctx.tick();
    let states = KINGDRA_FOCUSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kingdra focus state poisoned");
    states.retain(|state| state.expires_at > tick);
    states
        .iter()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id)
        .map(|state| state.focus_stacks.saturating_mul(10))
        .unwrap_or(0)
}

pub fn note_kingdra_basic_hit(ctx: &mut GameCtx, caster_id: usize, target_id: usize) {
    if !has_kings_rock(caster_id) {
        return;
    }
    let tick = ctx.tick();
    let states = KINGDRA_FOCUSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut apply_slow = false;
    {
        let mut states = states.lock().expect("kingdra focus state poisoned");
        states.retain(|state| state.expires_at > tick);
        if let Some(existing) = states
            .iter_mut()
            .find(|state| state.caster_id == caster_id && state.target_id == target_id)
        {
            existing.consecutive_hits = existing.consecutive_hits.saturating_add(1);
            if existing.consecutive_hits % 3 == 0 {
                existing.focus_stacks = existing.focus_stacks.saturating_add(1).min(3);
            }
            existing.expires_at = tick.saturating_add(KINGDRA_FOCUS_TICKS);
            apply_slow = existing.focus_stacks >= 3;
        } else {
            states.push(KingdraFocusState {
                caster_id,
                target_id,
                consecutive_hits: 1,
                focus_stacks: 0,
                expires_at: tick.saturating_add(KINGDRA_FOCUS_TICKS),
            });
        }
    }
    if apply_slow {
        add_harmful_buff(
            ctx,
            caster_id,
            target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: KINGDRA_FOCUS_TICKS,
                },
                move_speed_mult: -20,
                ..Default::default()
            },
        );
    }
}

fn kingdra_attack_speed_bonus_for_level(level: usize) -> i32 {
    if level >= 10 {
        14
    } else if level >= 7 {
        12
    } else if level >= 4 {
        10
    } else {
        8
    }
}

pub fn note_kingdra_ability_used(ctx: &GameCtx, entity_id: usize) {
    let tick = ctx.tick();
    let states = KINGDRA_DRAGON_DANCES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kingdra dragon dance state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.boosted_until = tick.saturating_add(KINGDRA_DRAGON_DANCE_BOOST_TICKS);
    } else {
        states.push(KingdraDragonDanceState {
            entity_id,
            boosted_until: tick.saturating_add(KINGDRA_DRAGON_DANCE_BOOST_TICKS),
            primed_until: 0,
            dragon_pulse_cooldown_until: 0,
        });
    }
}

pub fn prime_kingdra_dragon_dance_basic(ctx: &GameCtx, entity_id: usize, ticks: usize) {
    let tick = ctx.tick();
    let states = KINGDRA_DRAGON_DANCES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kingdra dragon dance state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.primed_until = tick.saturating_add(ticks);
    } else {
        states.push(KingdraDragonDanceState {
            entity_id,
            boosted_until: tick,
            primed_until: tick.saturating_add(ticks),
            dragon_pulse_cooldown_until: 0,
        });
    }
}

pub fn note_kingdra_dragon_pulse_cast(ctx: &GameCtx, entity_id: usize, cooldown_ticks: usize) {
    let tick = ctx.tick();
    let states = KINGDRA_DRAGON_DANCES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kingdra dragon dance state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.dragon_pulse_cooldown_until = tick.saturating_add(cooldown_ticks);
    } else {
        states.push(KingdraDragonDanceState {
            entity_id,
            boosted_until: tick,
            primed_until: 0,
            dragon_pulse_cooldown_until: tick.saturating_add(cooldown_ticks),
        });
    }
}

pub fn kingdra_kings_rock_bonus_percent(ctx: &GameCtx, entity_id: usize) -> usize {
    if !has_kings_rock(entity_id) {
        return 0;
    }
    let level = ctx
        .get_entity(entity_id)
        .map(|entity| entity.level())
        .unwrap_or(1);
    let base = if level >= 10 {
        18
    } else if level >= 5 {
        14
    } else {
        10
    };
    let tick = ctx.tick();
    let dragon_pulse_ready = KINGDRA_DRAGON_DANCES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("kingdra dragon dance state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.dragon_pulse_cooldown_until <= tick)
        .unwrap_or(true);
    if !dragon_pulse_ready {
        return base;
    }
    base + if level >= 10 {
        9
    } else if level >= 5 {
        6
    } else {
        3
    }
}

pub fn consume_kingdra_dragon_dance_basic(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = KINGDRA_DRAGON_DANCES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kingdra dragon dance state poisoned");
    let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return false;
    };
    if existing.primed_until <= tick {
        return false;
    }
    existing.primed_until = 0;
    true
}

pub fn update_kingdra_dragon_dance_passive(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let level = entity.level();
    drop(entity);

    let tick = ctx.tick();
    let boosted = KINGDRA_DRAGON_DANCES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("kingdra dragon dance state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.boosted_until > tick)
        .unwrap_or(false);
    let base_bonus = kingdra_attack_speed_bonus_for_level(level);
    let attack_speed_mult = if boosted {
        base_bonus.saturating_mul(2)
    } else {
        base_bonus
    };
    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: KINGDRA_DRAGON_DANCE_BUFF_TICKS,
            },
            attack_speed_mult,
            ..Default::default()
        },
    );
}

pub fn register_delibird_present(ctx: &GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    let pos = entity.pos();
    let tick = ctx.tick();
    drop(entity);

    let states = DELIBIRD_PRESENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("delibird present state poisoned");
    if states.iter().any(|state| state.entity_id == entity_id) {
        return;
    }
    states.push(DelibirdPresentState {
        entity_id,
        basic_casts: 0,
        last_pos: pos,
        moving_until: tick,
    });
}

pub fn update_delibird_hustle(ctx: &GameCtx, entity_id: usize) {
    const MOVING_THRESHOLD: u64 = 1_200;
    const MOVING_GRACE_TICKS: usize = 18;

    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let pos = entity.pos();
    let tick = ctx.tick();
    drop(entity);

    let states = DELIBIRD_PRESENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("delibird present state poisoned");
    states.retain(|state| {
        ctx.get_entity(state.entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false)
    });
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(DelibirdPresentState {
            entity_id,
            basic_casts: 0,
            last_pos: pos,
            moving_until: tick,
        });
        return;
    };

    let distance = state
        .last_pos
        .x
        .abs_diff(pos.x)
        .saturating_add(state.last_pos.y.abs_diff(pos.y));
    state.last_pos = pos;
    if distance >= MOVING_THRESHOLD {
        state.moving_until = tick.saturating_add(MOVING_GRACE_TICKS);
    }
}

pub fn delibird_hustle_crit_bonus(ctx: &GameCtx, entity_id: usize) -> usize {
    let Some(champion_id) = champion_id_for_entity(entity_id) else {
        return 0;
    };
    if champion_id != "pokemon_moba_delibird"
        && !receiver_has_copied(entity_id, "pokemon_moba_delibird")
    {
        return 0;
    }
    let tick = ctx.tick();
    DELIBIRD_PRESENTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("delibird present state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .filter(|state| state.moving_until > tick)
        .map(|_| 20)
        .unwrap_or(0)
}

pub fn update_cloyster_overcoat(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let crit_chance = entity.stat().crit_chance;
    drop(entity);

    let bonus_armor = crit_chance.saturating_mul(20) / 100;
    if bonus_armor == 0 {
        return;
    }
    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: CLOYSTER_OVERCOAT_TICKS,
            },
            defence: bonus_armor as i32,
            ..Default::default()
        },
    );
}

pub fn adjust_incoming_critical_damage_percent(
    ctx: &GameCtx,
    target_id: usize,
    crit_damage_percent: usize,
) -> usize {
    let Some(champion_id) = champion_id_for_entity(target_id) else {
        return crit_damage_percent;
    };
    if champion_id != "pokemon_moba_cloyster"
        && !receiver_has_copied(target_id, "pokemon_moba_cloyster")
    {
        return crit_damage_percent;
    }
    let level = ctx
        .get_entity(target_id)
        .map(|entity| entity.level())
        .unwrap_or(1);
    let reduce_percent = if level >= 10 {
        25
    } else if level >= 5 {
        20
    } else {
        15
    };
    crit_damage_percent.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100
}

pub fn note_delibird_present_basic(ctx: &GameCtx, entity_id: usize) {
    let Some(champion_id) = champion_id_for_entity(entity_id) else {
        return;
    };
    if champion_id != "pokemon_moba_delibird" {
        return;
    }
    let states = DELIBIRD_PRESENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("delibird present state poisoned");
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    let pos = entity.pos();
    drop(entity);
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.basic_casts = state.basic_casts.saturating_add(1).min(4);
    } else {
        states.push(DelibirdPresentState {
            entity_id,
            basic_casts: 1,
            last_pos: pos,
            moving_until: ctx.tick(),
        });
    }
}

pub fn delibird_present_heal_ready_for_player(
    player_id: usize,
    required_basic_casts: usize,
) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return false;
    };
    DELIBIRD_PRESENTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("delibird present state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.basic_casts >= required_basic_casts)
        .unwrap_or(false)
}

pub fn consume_delibird_present_heal_charge(
    ctx: &GameCtx,
    entity_id: usize,
    required_basic_casts: usize,
) -> bool {
    let Some(champion_id) = champion_id_for_entity(entity_id) else {
        return false;
    };
    if champion_id != "pokemon_moba_delibird" {
        return false;
    }
    let states = DELIBIRD_PRESENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("delibird present state poisoned");
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    let pos = entity.pos();
    drop(entity);
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(DelibirdPresentState {
            entity_id,
            basic_casts: 0,
            last_pos: pos,
            moving_until: ctx.tick(),
        });
        return false;
    };
    if state.basic_casts < required_basic_casts {
        return false;
    }
    state.basic_casts = state.basic_casts.saturating_sub(required_basic_casts);
    true
}

pub fn copy_receiver_passive(ctx: &mut GameCtx, receiver_id: usize, killed_entity_id: usize) {
    let Some(receiver) = ctx.get_entity(receiver_id) else {
        return;
    };
    if !receiver.is_alive() || !receiver.is_champion() {
        return;
    }
    drop(receiver);

    let Some(killed_champion_id) = champion_id_for_entity(killed_entity_id) else {
        return;
    };
    if matches!(
        killed_champion_id,
        "pokemon_moba_passimian"
            | "pokemon_moba_clawitzer"
            | "pokemon_moba_comfey"
            | "pokemon_moba_smeargle"
    ) {
        return;
    }

    let states = RECEIVERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("receiver state poisoned");
    let Some(state) = states
        .iter_mut()
        .find(|state| state.entity_id == receiver_id)
    else {
        return;
    };
    state.copied_champion_id = Some(killed_champion_id);
    drop(states);

    if killed_champion_id == "pokemon_moba_audino" {
        register_audino(ctx, receiver_id);
    }
    if killed_champion_id == "pokemon_moba_scizor" {
        register_scizor(receiver_id);
        reset_light_metal(ctx, receiver_id);
    }
    if killed_champion_id == "pokemon_moba_blaziken" {
        reset_speed_boost(ctx, receiver_id);
    }
    if killed_champion_id == "pokemon_moba_skarmory" {
        register_sturdy(receiver_id);
    }
    if killed_champion_id == "pokemon_moba_ribombee" {
        register_honey_gatherer(ctx, receiver_id);
    }
    if killed_champion_id == "pokemon_moba_delibird" {
        register_delibird_present(ctx, receiver_id);
    }
    if killed_champion_id == "pokemon_moba_porygonz" {
        register_porygon_type(receiver_id);
    }
    if killed_champion_id == "pokemon_moba_shedinja" {
        register_wonder_guard(receiver_id);
    }
}

pub fn begin_reversal(
    ctx: &GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    retaliation_damage: usize,
) {
    let tick = ctx.tick();
    let states = REVERSALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("reversal state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.expires_at = tick.saturating_add(duration_ticks);
        existing.retaliation_damage = retaliation_damage;
        return;
    }
    states.push(ReversalState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
        retaliation_damage,
    });
}

pub fn try_consume_reversal(ctx: &GameCtx, entity_id: usize, attacker_id: usize) -> Option<usize> {
    if entity_id == attacker_id {
        return None;
    }
    let tick = ctx.tick();
    let states = REVERSALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("reversal state poisoned");
    let mut consumed_damage = None;
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if consumed_damage.is_none() && state.entity_id == entity_id {
            consumed_damage = Some(state.retaliation_damage);
            return false;
        }
        true
    });
    consumed_damage
}

pub fn register_shell_armor(entity_id: usize) {
    let states = SHELL_ARMORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("shell armor state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn register_scrappy(entity_id: usize) {
    let states = SCRAPPYS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("scrappy state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn register_sturdy(entity_id: usize) {
    let states = STURDIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sturdy state poisoned");
    if !states.iter().any(|state| state.entity_id == entity_id) {
        states.push(SturdyState {
            entity_id,
            ready_at: 0,
        });
    }
}

pub fn register_justified(entity_id: usize) {
    let states = JUSTIFIED_ENTITIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("justified state poisoned");
    if !states.iter().any(|state| state.entity_id == entity_id) {
        states.push(JustifiedState { entity_id });
    }
}

pub fn apply_gallade_prediction(
    ctx: &GameCtx,
    gallade_id: usize,
    enemy_id: usize,
    damaged_reduce: usize,
    ticks: usize,
) {
    let expires_at = ctx.tick().saturating_add(ticks);
    let states = GALLADE_PREDICTIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("gallade prediction state poisoned");
    states.retain(|state| {
        state.expires_at > ctx.tick()
            && !(state.gallade_id == gallade_id && state.enemy_id == enemy_id)
    });
    states.push(GalladePredictionState {
        gallade_id,
        enemy_id,
        expires_at,
        damaged_reduce,
    });
}

pub fn gallade_prediction_reduce_percent(
    ctx: &GameCtx,
    gallade_id: usize,
    attacker_id: usize,
) -> usize {
    let tick = ctx.tick();
    let states = GALLADE_PREDICTIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("gallade prediction state poisoned");
    states.retain(|state| state.expires_at > tick);
    states
        .iter()
        .find(|state| state.gallade_id == gallade_id && state.enemy_id == attacker_id)
        .map(|state| state.damaged_reduce.min(95))
        .unwrap_or(0)
}

pub fn trigger_justified_if_dark_damage(
    ctx: &mut GameCtx,
    target_id: usize,
    move_type: PokemonType,
    damage: usize,
) {
    if damage == 0 || !matches!(move_type, PokemonType::Dark) {
        return;
    }
    let has_justified = JUSTIFIED_ENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("justified state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == target_id && entity_is_champion_id(target_id, "pokemon_moba_gallade")
        })
        || receiver_has_copied(target_id, "pokemon_moba_gallade");
    if !has_justified {
        return;
    }

    ctx.add_buff(
        target_id,
        BuffState {
            duration: BuffType::Time { tick: 5 * 60 },
            attack_mult: 15,
            ..Default::default()
        },
    );
}

pub fn register_audino(ctx: &GameCtx, audino_id: usize) {
    let states = AUDINO_REGENERATORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino regenerator state poisoned");
    if !states.iter().any(|state| state.audino_id == audino_id) {
        states.push(AudinoRegeneratorState {
            audino_id,
            last_damaged_tick: ctx.tick(),
            next_heal_tick: ctx.tick().saturating_add(2 * 60),
        });
    }
}

pub fn note_audino_damaged(ctx: &GameCtx, audino_id: usize) {
    let states = AUDINO_REGENERATORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino regenerator state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.audino_id == audino_id) {
        state.last_damaged_tick = ctx.tick();
        state.next_heal_tick = ctx.tick().saturating_add(2 * 60);
    }
}

pub fn register_smeargle(_ctx: &GameCtx, entity_id: usize) {
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("smeargle state poisoned");
    if !states.iter().any(|state| state.entity_id == entity_id) {
        states.push(SmeargleState {
            entity_id,
            last_candidate: None,
            learned: [None, None, None, None],
            ready_at: [0, 0, 0, 0],
        });
    }
}

pub fn note_smeargle_affected(
    ctx: &GameCtx,
    target_id: usize,
    source_champion: PokemonChampion,
    action: PokemonMove,
) {
    if !is_smeargle_copyable(action) {
        return;
    }

    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == target_id) else {
        return;
    };
    let target_alive = ctx
        .get_entity(target_id)
        .map(|entity| entity.is_alive())
        .unwrap_or(false);

    let copied = SmeargleCopiedMove {
        source_champion,
        action,
    };
    if smeargle_has_learned(state, copied) {
        return;
    }
    state.last_candidate = Some(SmeargleCandidateState {
        copied,
        expires_at: ctx.tick().saturating_add(if target_alive {
            SMEARGLE_CANDIDATE_TICKS
        } else {
            SMEARGLE_CANDIDATE_TICKS.max(45 * 60)
        }),
    });
}

pub fn smeargle_learned_action(
    ctx: &GameCtx,
    entity_id: usize,
    slot: ActionSlot,
) -> Option<SmeargleCopiedMove> {
    let tick = ctx.tick();
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("smeargle state poisoned");
    let state = states
        .iter_mut()
        .find(|state| state.entity_id == entity_id)?;
    if state
        .last_candidate
        .map(|candidate| candidate.expires_at <= tick)
        .unwrap_or(false)
    {
        state.last_candidate = None;
    }
    state.learned[slot.index()]
}

pub fn smeargle_slot_is_active_sketch(_ctx: &GameCtx, entity_id: usize, slot: ActionSlot) -> bool {
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter().find(|state| state.entity_id == entity_id) else {
        return matches!(slot, ActionSlot::Attack);
    };
    smeargle_next_open_slot(state) == Some(slot.index())
}

pub fn smeargle_slot_is_active_sketch_for_player(
    player_id: usize,
    _tick: usize,
    slot: ActionSlot,
) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return false;
    };
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter().find(|state| state.entity_id == entity_id) else {
        return matches!(slot, ActionSlot::Attack);
    };
    smeargle_next_open_slot(state) == Some(slot.index())
}

pub fn smeargle_has_valid_candidate_for_active_slot_for_player(
    player_id: usize,
    tick: usize,
    slot: ActionSlot,
) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return false;
    };
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter().find(|state| state.entity_id == entity_id) else {
        return false;
    };
    if smeargle_next_open_slot(state) != Some(slot.index()) {
        return false;
    }
    state
        .last_candidate
        .map(|candidate| {
            candidate.expires_at > tick
                && !smeargle_has_learned(state, candidate.copied)
                && smeargle_ai_should_sketch(state, candidate.copied, tick)
        })
        .unwrap_or(false)
}

pub fn smeargle_active_sketch_slot_with_valid_candidate_for_player(
    player_id: usize,
    tick: usize,
) -> Option<ActionSlot> {
    let entity_id = entity_for_player(player_id)?;
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("smeargle state poisoned");
    let state = states.iter().find(|state| state.entity_id == entity_id)?;
    let open_slot = smeargle_next_open_slot(state)?;
    let candidate = state.last_candidate?;
    if candidate.expires_at <= tick
        || smeargle_has_learned(state, candidate.copied)
        || !smeargle_ai_should_sketch(state, candidate.copied, tick)
    {
        return None;
    }
    match open_slot {
        0 => Some(ActionSlot::Attack),
        1 => Some(ActionSlot::Skill),
        2 => Some(ActionSlot::Skill2),
        3 => Some(ActionSlot::Ult),
        _ => None,
    }
}

pub fn smeargle_copied_slot_ready_for_player(
    player_id: usize,
    tick: usize,
    slot: ActionSlot,
) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return true;
    };
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("smeargle state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| tick >= state.ready_at[slot.index()])
        .unwrap_or(true)
}

pub fn smeargle_learned_action_for_player(
    player_id: usize,
    slot: ActionSlot,
) -> Option<SmeargleCopiedMove> {
    let entity_id = entity_for_player(player_id)?;
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("smeargle state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .and_then(|state| state.learned[slot.index()])
}

pub fn smeargle_learned_count_for_player(player_id: usize) -> usize {
    let Some(entity_id) = entity_for_player(player_id) else {
        return 0;
    };
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("smeargle state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.learned.iter().flatten().count())
        .unwrap_or(0)
}

pub fn smeargle_force_learn_for_player(
    player_id: usize,
    source_champion: PokemonChampion,
    action: PokemonMove,
) -> bool {
    if !is_smeargle_copyable(action) {
        return false;
    }
    let Some(entity_id) = entity_for_player(player_id) else {
        return false;
    };
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return false;
    };
    let copied = SmeargleCopiedMove {
        source_champion,
        action,
    };
    if smeargle_has_learned(state, copied) {
        return false;
    }
    let Some(open_slot) = smeargle_next_open_slot(state) else {
        return false;
    };
    state.learned[open_slot] = Some(copied);
    state.last_candidate = None;
    true
}

pub fn smeargle_try_sketch(ctx: &GameCtx, entity_id: usize, slot: ActionSlot) -> bool {
    let tick = ctx.tick();
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return false;
    };
    let Some(open_slot) = smeargle_next_open_slot(state) else {
        return false;
    };
    if open_slot != slot.index() {
        return false;
    }
    let Some(candidate) = state.last_candidate else {
        return false;
    };
    if candidate.expires_at <= tick || smeargle_has_learned(state, candidate.copied) {
        state.last_candidate = None;
        return false;
    }
    state.learned[open_slot] = Some(candidate.copied);
    state.last_candidate = None;
    true
}

pub fn smeargle_copied_slot_ready(ctx: &GameCtx, entity_id: usize, slot: ActionSlot) -> bool {
    let tick = ctx.tick();
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("smeargle state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| tick >= state.ready_at[slot.index()])
        .unwrap_or(true)
}

pub fn smeargle_start_copied_slot_cooldown(ctx: &GameCtx, entity_id: usize, slot: ActionSlot) {
    let states = SMEARGLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("smeargle state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return;
    };
    let Some(copied) = state.learned[slot.index()] else {
        return;
    };
    let fallback = match slot {
        ActionSlot::Attack => 48,
        ActionSlot::Skill => 240,
        ActionSlot::Skill2 => 420,
        ActionSlot::Ult => 900,
    };
    let cooldown = copied.action.cooldown.max(fallback);
    state.ready_at[slot.index()] = ctx.tick().saturating_add(cooldown);
}

fn is_smeargle_copyable(action: PokemonMove) -> bool {
    !matches!(
        action.effect,
        PokemonMoveEffect::Sketch | PokemonMoveEffect::None
    )
}

fn smeargle_next_open_slot(state: &SmeargleState) -> Option<usize> {
    state.learned.iter().position(|learned| learned.is_none())
}

fn smeargle_has_learned(state: &SmeargleState, copied: SmeargleCopiedMove) -> bool {
    state.learned.iter().flatten().any(|learned| {
        learned.source_champion.id == copied.source_champion.id
            && learned.action.slot == copied.action.slot
            && learned.action.name_key == copied.action.name_key
    })
}

fn smeargle_ai_should_sketch(
    state: &SmeargleState,
    copied: SmeargleCopiedMove,
    tick: usize,
) -> bool {
    let Some(open_slot) = smeargle_next_open_slot(state) else {
        return false;
    };
    let quality = smeargle_candidate_quality(state, copied);
    quality >= smeargle_ai_quality_threshold(open_slot, tick)
}

fn smeargle_candidate_quality(state: &SmeargleState, copied: SmeargleCopiedMove) -> usize {
    let learned_basic = state
        .learned
        .iter()
        .flatten()
        .any(|learned| matches!(learned.action.slot, ActionSlot::Attack));
    if copied.source_champion.id == "pokemon_moba_smeargle"
        && copied.action.name_key == "tower_shot"
    {
        return 90;
    }
    match copied.action.slot {
        ActionSlot::Ult => 100,
        ActionSlot::Skill2 => 78,
        ActionSlot::Skill => 70,
        ActionSlot::Attack if learned_basic => 20,
        ActionSlot::Attack => 58,
    }
}

fn smeargle_ai_quality_threshold(open_slot: usize, tick: usize) -> usize {
    match open_slot {
        0 if tick < 20 * 60 => 20,
        0 => 0,
        1 if tick < 60 * 60 => 70,
        1 if tick < 120 * 60 => 50,
        1 => 20,
        2 if tick < 90 * 60 => 75,
        2 if tick < 180 * 60 => 55,
        2 => 25,
        3 if tick < 150 * 60 => 80,
        3 if tick < 270 * 60 => 60,
        3 => 30,
        _ => 100,
    }
}

pub fn begin_audino_protect(ctx: &GameCtx, audino_id: usize, target_id: usize, ticks: usize) {
    if audino_id == target_id || ticks == 0 {
        return;
    }
    let expires_at = ctx.tick().saturating_add(ticks);
    let states = AUDINO_PROTECTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino protect state poisoned");
    states.retain(|state| {
        state.expires_at > ctx.tick()
            && !(state.audino_id == audino_id && state.target_id == target_id)
    });
    states.push(AudinoProtectState {
        audino_id,
        target_id,
        expires_at,
    });
}

pub fn audino_protect_redirect(ctx: &GameCtx, target_id: usize) -> Option<usize> {
    let tick = ctx.tick();
    let states = AUDINO_PROTECTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino protect state poisoned");
    states.retain(|state| state.expires_at > tick);

    let candidates: Vec<usize> = states
        .iter()
        .filter(|state| state.target_id == target_id && state.audino_id != target_id)
        .map(|state| state.audino_id)
        .collect();
    drop(states);

    candidates.into_iter().find(|audino_id| {
        let Some(audino) = ctx.get_entity(*audino_id) else {
            return false;
        };
        let Some(target) = ctx.get_entity(target_id) else {
            return false;
        };
        audino.is_alive()
            && target.is_alive()
            && audino.team() == target.team()
            && audino.is_champion()
    })
}

fn audino_protect_blocks_new_damage_status(ctx: &GameCtx, target_id: usize) -> bool {
    audino_protect_redirect(ctx, target_id).is_some()
}

pub fn begin_endure(ctx: &GameCtx, target_id: usize, ticks: usize) {
    if ticks == 0 {
        return;
    }
    let expires_at = ctx.tick().saturating_add(ticks);
    let states = ENDURES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("endure state poisoned");
    states.retain(|state| state.expires_at > ctx.tick() && state.target_id != target_id);
    states.push(EndureState {
        target_id,
        expires_at,
    });
}

pub fn adjust_endure_damage(
    ctx: &GameCtx,
    target_id: usize,
    ad_damage: usize,
    ap_damage: usize,
) -> (usize, usize) {
    let incoming = ad_damage.saturating_add(ap_damage);
    if incoming == 0 {
        return (ad_damage, ap_damage);
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return (ad_damage, ap_damage);
    };
    if !target.is_alive() {
        return (ad_damage, ap_damage);
    }
    let hp = target.hp();
    let current_hp = hp.current;
    let max_hp = hp.max;
    drop(target);

    if incoming < current_hp {
        return (ad_damage, ap_damage);
    }

    if !consume_endure(ctx, target_id) {
        return (ad_damage, ap_damage);
    }

    let desired_hp = (max_hp / 2).max(1);
    let allowed_damage = current_hp.saturating_sub(desired_hp);
    if ad_damage > 0 {
        (allowed_damage, 0)
    } else {
        (0, allowed_damage)
    }
}

pub fn try_post_damage_endure_heal(ctx: &mut GameCtx, target_id: usize) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    let hp = target.hp();
    let current_hp = hp.current;
    let max_hp = hp.max;
    drop(target);

    if current_hp > 0 || !consume_endure(ctx, target_id) {
        return;
    }

    heal_with_antiheal(ctx, target_id, target_id, (max_hp / 2).max(1));
}

pub fn handle_audino_protect_damage(ctx: &mut GameCtx, target_id: usize, damage: usize) {
    // Fallback for engine damage callbacks that bypass the Pokemon damage resolver.
    if damage == 0 {
        return;
    }
    let tick = ctx.tick();
    let states = AUDINO_PROTECTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino protect state poisoned");
    states.retain(|state| state.expires_at > tick);
    let Some(audino_id) = states
        .iter()
        .find(|state| state.target_id == target_id)
        .map(|state| state.audino_id)
    else {
        return;
    };
    drop(states);

    if audino_id == target_id {
        return;
    }
    let Some(audino) = ctx.get_entity(audino_id) else {
        return;
    };
    if !audino.is_alive() {
        return;
    }
    drop(audino);

    heal_with_antiheal(ctx, audino_id, target_id, damage);
    let (redirect_ad, redirect_ap) = adjust_endure_damage(ctx, audino_id, damage, 0);
    crate::pokemon_status::deal_tracked_damage(
        ctx,
        target_id,
        audino_id,
        redirect_ad,
        redirect_ap,
        AttackType::Item,
    );
}

fn consume_endure(ctx: &GameCtx, target_id: usize) -> bool {
    let tick = ctx.tick();
    let states = ENDURES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("endure state poisoned");
    let mut consumed = false;
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if !consumed && state.target_id == target_id {
            consumed = true;
            return false;
        }
        true
    });
    consumed
}

pub fn adjust_sturdy_damage(
    ctx: &GameCtx,
    target_id: usize,
    attack_type: AttackType,
    ad_damage: usize,
    ap_damage: usize,
) -> (usize, usize) {
    if matches!(attack_type, AttackType::Dot) {
        return (ad_damage, ap_damage);
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return (ad_damage, ap_damage);
    };
    if !target.is_alive() {
        return (ad_damage, ap_damage);
    }
    let current_hp = target.hp().current;
    drop(target);

    if ad_damage.saturating_add(ap_damage) < current_hp {
        return (ad_damage, ap_damage);
    }

    let has_sturdy_identity = champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_skarmory")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_skarmory");
    if !has_sturdy_identity {
        return (ad_damage, ap_damage);
    }

    let tick = ctx.tick();
    let states = STURDIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sturdy state poisoned");
    let Some(state) = states
        .iter_mut()
        .find(|state| state.entity_id == target_id && state.ready_at <= tick)
    else {
        return (ad_damage, ap_damage);
    };

    state.ready_at = tick.saturating_add(5 * 60 * 60);
    let allowed_damage = current_hp.saturating_sub(1);
    if ad_damage > 0 {
        (allowed_damage, 0)
    } else {
        (0, allowed_damage)
    }
}

pub fn register_limber(entity_id: usize) {
    let states = LIMBERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("limber state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn is_limber(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = LIMBERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("limber state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
                && entity_is_champion_id(entity_id, "pokemon_moba_hitmonlee")
        });
    explicit || receiver_has_copied(entity_id, "pokemon_moba_hitmonlee")
}

pub fn register_hitmontop(entity_id: usize) {
    let states = HITMONTOPS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("hitmontop state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn note_steadfast_cc(ctx: &mut GameCtx, entity_id: usize) {
    let is_hitmontop = HITMONTOPS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("hitmontop state poisoned")
        .iter()
        .any(|id| *id == entity_id && entity_is_champion_id(entity_id, "pokemon_moba_hitmontop"))
        || receiver_has_copied(entity_id, "pokemon_moba_hitmontop");
    if !is_hitmontop {
        return;
    }

    ctx.add_buff(
        entity_id,
        BuffState {
            duration: BuffType::Permanent,
            move_speed_mult: 4,
            ..Default::default()
        },
    );
}

pub fn register_kilowattrel(entity_id: usize) {
    let states = KILOWATTRELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kilowattrel state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn register_beeheeyem(entity_id: usize) {
    let states = BEEHEEYEMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("beeheeyem state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

fn is_beeheeyem(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = BEEHEEYEMS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("beeheeyem state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive() && entity.is_champion())
                    .unwrap_or(false)
                && entity_is_champion_id(entity_id, "pokemon_moba_beeheeyem")
        });
    explicit || receiver_has_copied(entity_id, "pokemon_moba_beeheeyem")
}

pub fn register_technician(entity_id: usize) {
    let states = TECHNICIANS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("technician state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn note_ally_buff_received(
    ctx: &mut GameCtx,
    source_id: usize,
    target_id: usize,
    ticks: usize,
) {
    if source_id == target_id {
        return;
    }
    let is_technician = TECHNICIANS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("technician state poisoned")
        .iter()
        .any(|id| {
            *id == target_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
                && entity_is_champion_id(target_id, "pokemon_moba_ambipom")
        })
        || receiver_has_copied(target_id, "pokemon_moba_ambipom");
    if !is_technician {
        return;
    }
    ctx.add_buff(
        target_id,
        BuffState {
            duration: BuffType::Time { tick: ticks },
            move_speed_mult: 18,
            ..Default::default()
        },
    );
}

pub fn register_dot_immunity(entity_id: usize) {
    let states = DOT_IMMUNES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("dot immunity state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn register_dot_absorb(entity_id: usize) {
    let states = DOT_ABSORBERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("dot absorb state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

fn has_dot_immunity(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = DOT_IMMUNES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("dot immunity state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });
    let current_has_immunity = entity_is_champion_id(entity_id, "pokemon_moba_cryogonal")
        || entity_is_champion_id(entity_id, "pokemon_moba_kommoo");
    explicit && current_has_immunity
        || receiver_has_copied(entity_id, "pokemon_moba_cryogonal")
        || receiver_has_copied(entity_id, "pokemon_moba_kommoo")
}

pub fn begin_kommoo_duel(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    max_ticks: usize,
    refresh_ticks: usize,
    target_attack_mult: i32,
    target_defence_mult: i32,
    caster_attack_speed_mult: i32,
) {
    if caster_id == target_id {
        return;
    }
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    let caster_alive = caster.is_alive();
    let caster_team = caster.team();
    drop(caster);
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    let target_alive = target.is_alive();
    let target_team = target.team();
    drop(target);
    if !caster_alive || !target_alive || caster_team == target_team {
        return;
    }

    let tick = ctx.tick();
    let refresh_ticks = refresh_ticks.max(10);
    let state = KommooDuelState {
        caster_id,
        target_id,
        expires_at: tick.saturating_add(max_ticks),
        next_refresh_at: tick,
        refresh_ticks,
        target_attack_mult,
        target_defence_mult,
        caster_attack_speed_mult,
    };

    let states = KOMMOO_DUELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kommo-o duel state poisoned");
    states.retain(|existing| {
        existing.caster_id != caster_id
            && existing.target_id != target_id
            && existing.caster_id != target_id
            && existing.target_id != caster_id
    });
    states.push(state);
    drop(states);

    apply_kommoo_duel_pulse(ctx, state);
}

fn apply_kommoo_duel_pulse(ctx: &mut GameCtx, state: KommooDuelState) {
    let Some(caster) = ctx.get_entity(state.caster_id) else {
        return;
    };
    let caster_alive = caster.is_alive();
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);
    let Some(target) = ctx.get_entity(state.target_id) else {
        return;
    };
    let target_alive = target.is_alive();
    let target_team = target.team();
    let target_pos = target.pos();
    drop(target);
    if !caster_alive || !target_alive || caster_team == target_team {
        return;
    }

    let ticks = state.refresh_ticks.saturating_add(5);
    apply_pokemon_cc(
        ctx,
        state.caster_id,
        state.caster_id,
        CCState::Bind { tick: ticks as u64 },
    );
    apply_pokemon_cc(
        ctx,
        state.caster_id,
        state.caster_id,
        CCState::BlockSkill { tick: ticks },
    );
    apply_pokemon_cc(
        ctx,
        state.target_id,
        state.caster_id,
        CCState::Taunt {
            tick: ticks as u64,
            target: state.target_id,
        },
    );
    apply_pokemon_cc(
        ctx,
        state.caster_id,
        state.target_id,
        CCState::Bind { tick: ticks as u64 },
    );
    apply_pokemon_cc(
        ctx,
        state.caster_id,
        state.target_id,
        CCState::BlockSkill { tick: ticks },
    );
    apply_pokemon_cc(
        ctx,
        state.caster_id,
        state.target_id,
        CCState::Taunt {
            tick: ticks as u64,
            target: state.caster_id,
        },
    );
    add_beneficial_buff(
        ctx,
        state.caster_id,
        state.caster_id,
        BuffState {
            duration: BuffType::Time { tick: ticks },
            attack_speed_mult: state.caster_attack_speed_mult,
            ..Default::default()
        },
    );
    ctx.add_buff(
        state.target_id,
        BuffState {
            duration: BuffType::Time { tick: ticks },
            attack_mult: state.target_attack_mult,
            defence_mult: state.target_defence_mult,
            ..Default::default()
        },
    );
    ctx.debug_draw_line(
        caster_pos.x,
        caster_pos.y,
        target_pos.x,
        target_pos.y,
        VFX_FIGHTING,
    );
    draw_status_marker(ctx, caster_pos, 18000, VFX_FIGHTING);
    draw_status_marker(ctx, target_pos, 18000, VFX_FIGHTING);
}

fn update_kommoo_duels(ctx: &mut GameCtx, tick: usize) {
    let states = KOMMOO_DUELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kommo-o duel state poisoned");
    let mut pulses = Vec::new();
    states.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        let caster_alive = caster.is_alive();
        let caster_team = caster.team();
        drop(caster);
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        let target_alive = target.is_alive();
        let target_team = target.team();
        drop(target);
        if !caster_alive || !target_alive || caster_team == target_team {
            return false;
        }
        if state.next_refresh_at <= tick {
            pulses.push(*state);
            state.next_refresh_at = tick.saturating_add(state.refresh_ticks);
        }
        true
    });
    drop(states);

    for state in pulses {
        apply_kommoo_duel_pulse(ctx, state);
    }
}

pub fn break_kommoo_duel_on_hard_cc(ctx: &GameCtx, source_id: usize, target_id: usize) {
    if source_id == target_id {
        return;
    }
    let Some(source) = ctx.get_entity(source_id) else {
        return;
    };
    let source_team = source.team();
    let source_alive = source.is_alive();
    drop(source);
    if !source_alive {
        return;
    }

    let states = KOMMOO_DUELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kommo-o duel state poisoned");
    states.retain(|state| {
        if state.caster_id != target_id {
            return true;
        }
        ctx.get_entity(state.target_id)
            .map(|duel_target| source_team != duel_target.team())
            .unwrap_or(true)
    });
}

fn has_dot_absorb(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = DOT_ABSORBERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("dot absorb state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });
    let current_has_absorb = entity_is_champion_id(entity_id, "pokemon_moba_pyukumuku");
    explicit && current_has_absorb || receiver_has_copied(entity_id, "pokemon_moba_pyukumuku")
}

pub fn can_volt_absorb(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = KILOWATTRELS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("kilowattrel state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });
    let current_has_absorb = entity_is_champion_id(entity_id, "pokemon_moba_kilowattrel");
    (explicit && current_has_absorb || receiver_has_copied(entity_id, "pokemon_moba_kilowattrel"))
        && !is_crowd_controlled(ctx, entity_id)
}

pub fn register_blissey(entity_id: usize) {
    let states = BLISSEYS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("blissey state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn register_scizor(entity_id: usize) {
    let states = SCIZORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("scizor state poisoned");
    if !states.iter().any(|id| *id == entity_id) {
        states.push(entity_id);
    }
}

pub fn register_sawk_throh(entity_id: usize) {
    let states = SAWK_THROH_FORMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sawk/throh stance state poisoned");
    if states.iter().any(|state| state.entity_id == entity_id) {
        return;
    }
    states.push(SawkThrohState {
        entity_id,
        form: SawkThrohForm::Sawk,
        last_buff_tick: 0,
    });
}

pub fn sawk_throh_form(ctx: &GameCtx, entity_id: usize) -> SawkThrohForm {
    SAWK_THROH_FORMS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("sawk/throh stance state poisoned")
        .iter()
        .find(|state| {
            state.entity_id == entity_id
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
        .map(|state| state.form)
        .unwrap_or(SawkThrohForm::Sawk)
}

pub fn is_sawk_form(ctx: &GameCtx, entity_id: usize) -> bool {
    sawk_throh_form(ctx, entity_id) == SawkThrohForm::Sawk
}

pub fn toggle_sawk_throh_form(ctx: &mut GameCtx, entity_id: usize) -> SawkThrohForm {
    register_sawk_throh(entity_id);
    let states = SAWK_THROH_FORMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sawk/throh stance state poisoned");
    let tick = ctx.tick();
    let mut next_form = SawkThrohForm::Sawk;

    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.form = match state.form {
            SawkThrohForm::Sawk => SawkThrohForm::Throh,
            SawkThrohForm::Throh => SawkThrohForm::Sawk,
        };
        state.last_buff_tick = tick.saturating_sub(SAWK_THROH_INTERVAL_TICKS);
        next_form = state.form;
    }
    drop(states);

    refresh_sawk_throh_buff(ctx, entity_id, next_form);
    next_form
}

pub fn should_sawk_throh_switch_for_player(player_id: usize, hp_ratio: usize) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return false;
    };
    let form = SAWK_THROH_FORMS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("sawk/throh stance state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.form)
        .unwrap_or(SawkThrohForm::Sawk);

    match form {
        SawkThrohForm::Sawk => hp_ratio <= 45,
        SawkThrohForm::Throh => hp_ratio >= 72,
    }
}

pub fn adjusted_cc_ticks(ctx: &GameCtx, entity_id: usize, ticks: usize) -> usize {
    if is_bouffalant_unstoppable(ctx, entity_id) {
        return 0;
    }
    let has_inner_focus = SAWK_THROH_FORMS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("sawk/throh stance state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
        || receiver_has_copied(entity_id, "pokemon_moba_sawk_throh");
    let has_afro = champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_bouffalant")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_bouffalant");
    if has_afro {
        note_bouffalant_cc(ctx, entity_id);
    }
    let ticks = if has_inner_focus {
        ticks.saturating_add(1).saturating_div(2).max(1)
    } else {
        ticks
    };
    if has_afro {
        ticks
            .saturating_mul(BOUFFALANT_AFRO_CC_REDUCE_NUM)
            .saturating_add(BOUFFALANT_AFRO_CC_REDUCE_DEN - 1)
            .saturating_div(BOUFFALANT_AFRO_CC_REDUCE_DEN)
            .max(1)
    } else {
        ticks
    }
}

#[allow(dead_code)]
pub fn register_player_entity(player_id: usize, entity_id: usize) {
    let _ = (player_id, entity_id);
}

#[allow(dead_code)]
pub fn register_player_entity_at_tick(player_id: usize, entity_id: usize, tick: usize) {
    let _ = (player_id, entity_id, tick);
}

pub fn register_player_entity_life_at_tick(
    ctx: &GameCtx,
    player_id: usize,
    entity_id: usize,
    tick: usize,
    life_id: Option<usize>,
) {
    let ctx_id = combat_ctx_id(ctx);
    let life_id = life_id.unwrap_or(0);
    let states = PLAYER_ENTITIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("player entity state poisoned");

    if let Some(existing) = states.iter_mut().find(|state| {
        state.ctx_id == ctx_id
            && state.player_id == player_id
            && state.entity_id == entity_id
            && state.life_id == life_id
    }) {
        existing.last_seen_tick = tick;
        drop(states);
        register_entity_owner_snapshot(ctx, player_id, entity_id, Some(life_id), tick);
        reset_pokemon_combat_stats_for_spawn(ctx, player_id, entity_id, tick);
        return;
    }

    states.push(PlayerEntityState {
        ctx_id,
        player_id,
        entity_id,
        life_id,
        last_seen_tick: tick,
    });
    while states.len() > POKEMON_CONTEXT_LEDGER_MAX {
        states.remove(0);
    }
    drop(states);

    register_entity_owner_snapshot(ctx, player_id, entity_id, Some(life_id), tick);
    reset_pokemon_combat_stats_for_spawn(ctx, player_id, entity_id, tick);
}

pub fn register_player_identity_from_ai(
    player_id: usize,
    athlete_id: usize,
    team: usize,
    position: Position,
    champion_name: &str,
) {
    let Some(athlete_id) = reliable_athlete_id(athlete_id) else {
        return;
    };

    let identities = POKEMON_PLAYER_IDENTITIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut identities = identities
        .lock()
        .expect("pokemon player identity state poisoned");

    if let Some(existing) = identities.iter_mut().find(|state| {
        state.player_id == player_id
            && state.team == team
            && state.position == position
            && state.champion_name == champion_name
    }) {
        existing.athlete_id = athlete_id;
        existing.seen_count = existing.seen_count.saturating_add(1);
        return;
    }

    identities.push(PokemonPlayerIdentityState {
        player_id,
        athlete_id,
        team,
        position,
        champion_name: champion_name.to_string(),
        seen_count: 1,
    });
    while identities.len() > 2048 {
        identities.remove(0);
    }
}

fn reliable_athlete_id(athlete_id: usize) -> Option<usize> {
    if athlete_id > 1 {
        Some(athlete_id)
    } else {
        None
    }
}

pub fn pokemon_player_identity_snapshots() -> Vec<PokemonPlayerIdentitySnapshot> {
    let identities = POKEMON_PLAYER_IDENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("pokemon player identity state poisoned");

    identities
        .iter()
        .map(|identity| PokemonPlayerIdentitySnapshot {
            player_id: identity.player_id,
            athlete_id: identity.athlete_id,
            team: identity.team,
            position: identity.position,
            champion_name: identity.champion_name.clone(),
            seen_count: identity.seen_count,
        })
        .collect()
}

fn athlete_id_for_player_identity(
    player_id: usize,
    team: usize,
    position: Position,
    champion_id: Option<&str>,
) -> Option<usize> {
    athlete_id_for_player_identity_with_source(player_id, team, position, champion_id).0
}

fn athlete_id_for_player_identity_with_source(
    player_id: usize,
    team: usize,
    position: Position,
    champion_id: Option<&str>,
) -> (Option<usize>, &'static str) {
    let identities = POKEMON_PLAYER_IDENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("pokemon player identity state poisoned");

    if let Some(champion_id) = champion_id {
        if let Some(athlete_id) = identities
            .iter()
            .rev()
            .find(|state| {
                state.player_id == player_id
                    && state.team == team
                    && state.position == position
                    && state.champion_name == champion_id
            })
            .and_then(|state| reliable_athlete_id(state.athlete_id))
        {
            return (Some(athlete_id), "exact");
        }
    }

    if let Some(athlete_id) = identities
        .iter()
        .rev()
        .find(|state| {
            state.player_id == player_id && state.team == team && state.position == position
        })
        .and_then(|state| reliable_athlete_id(state.athlete_id))
    {
        return (Some(athlete_id), "slot");
    }

    if let Some(athlete_id) = identities
        .iter()
        .rev()
        .find(|state| state.player_id == player_id)
        .and_then(|state| reliable_athlete_id(state.athlete_id))
    {
        return (Some(athlete_id), "player");
    }

    (None, "none")
}

fn register_entity_owner_snapshot(
    ctx: &GameCtx,
    player_id: usize,
    entity_id: usize,
    life_id: Option<usize>,
    tick: usize,
) {
    let ctx_id = combat_ctx_id(ctx);
    let champion_id = champion_id_for_entity_in_ctx(ctx, entity_id);
    let life_id = life_id.unwrap_or(0);
    let owners = ENTITY_OWNERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut owners = owners.lock().expect("entity owner state poisoned");
    if let Some(existing) = owners.iter_mut().find(|state| {
        state.ctx_id == ctx_id
            && state.entity_id == entity_id
            && state.player_id == player_id
            && state.life_id == life_id
    }) {
        existing.champion_id = champion_id.or(existing.champion_id);
        existing.last_seen_tick = tick;
        return;
    }
    owners.push(EntityOwnerState {
        ctx_id,
        entity_id,
        player_id,
        life_id,
        champion_id,
        last_seen_tick: tick,
    });
    while owners.len() > POKEMON_CONTEXT_LEDGER_MAX {
        owners.remove(0);
    }
}

fn reset_pokemon_combat_stats_for_spawn(
    ctx: &GameCtx,
    player_id: usize,
    entity_id: usize,
    tick: usize,
) {
    let ctx_id = combat_ctx_id(ctx);
    let champion_id = champion_id_for_entity_in_ctx(ctx, entity_id);
    let (team, position) = ctx
        .get_player(player_id)
        .map(|player| (player.team(), player.position()))
        .unwrap_or((usize::MAX, Position::Top));
    let athlete_id = athlete_id_for_player_identity(player_id, team, position, champion_id);
    let states = POKEMON_COMBAT_STATS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("pokemon combat stat state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.ctx_id == ctx_id && state.player_id == player_id)
    {
        state.entity_id = entity_id;
        state.team = team;
        state.position = position;
        state.last_seen_tick = tick;
        if champion_id.is_some() {
            state.champion_id = champion_id;
        }
        if athlete_id.is_some() {
            state.athlete_id = athlete_id;
        }
        drop(states);
        return;
    }
    states.push(PokemonCombatStatState {
        ctx_id,
        player_id,
        athlete_id,
        entity_id,
        team,
        position,
        champion_id,
        last_seen_tick: tick,
        damage_dealt: 0,
        damage_taken: 0,
        healing_done: 0,
        kills: 0,
        deaths: 0,
        assists: 0,
    });
    while states.len() > POKEMON_COMBAT_STATS_MAX {
        states.remove(0);
    }
    drop(states);
}

pub fn player_for_entity(entity_id: usize) -> Option<usize> {
    PLAYER_ENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("player entity state poisoned")
        .iter()
        .rev()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.player_id)
}

fn player_life_for_entity_at_tick_on_team(
    ctx: &GameCtx,
    entity_id: usize,
    tick: usize,
    team: usize,
) -> Option<(usize, usize)> {
    for player_id in 0..16 {
        let Some(player) = ctx.get_player(player_id) else {
            continue;
        };
        if player.team() != team {
            continue;
        }
        let Some(champion) = player.champion() else {
            continue;
        };
        if champion.id() == entity_id {
            return Some((player_id, player.deaths()));
        }
    }

    let ctx_id = combat_ctx_id(ctx);
    PLAYER_ENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("player entity state poisoned")
        .iter()
        .rev()
        .find(|state| {
            state.ctx_id == ctx_id
                && state.entity_id == entity_id
                && (tick == usize::MAX || state.last_seen_tick <= tick)
                && (tick == usize::MAX || tick.saturating_sub(state.last_seen_tick) <= 5 * 60 * 60)
                && ctx
                    .get_player(state.player_id)
                    .map(|player| player.team() == team)
                    .unwrap_or(false)
        })
        .map(|state| (state.player_id, state.life_id))
}

fn owner_for_entity_at_tick_in_ctx(
    ctx: &GameCtx,
    entity_id: usize,
    tick: usize,
) -> Option<EntityOwnerState> {
    for player_id in 0..16 {
        let Some(player) = ctx.get_player(player_id) else {
            continue;
        };
        let Some(champion) = player.champion() else {
            continue;
        };
        if champion.id() == entity_id {
            return Some(EntityOwnerState {
                ctx_id: combat_ctx_id(ctx),
                entity_id,
                player_id,
                life_id: player.deaths(),
                champion_id: champion_id_for_entity(entity_id),
                last_seen_tick: tick,
            });
        }
    }

    let ctx_id = combat_ctx_id(ctx);
    ENTITY_OWNERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("entity owner state poisoned")
        .iter()
        .rev()
        .find(|state| {
            state.ctx_id == ctx_id
                && state.entity_id == entity_id
                && (tick == usize::MAX || state.last_seen_tick <= tick)
                && (tick == usize::MAX || tick.saturating_sub(state.last_seen_tick) <= 5 * 60 * 60)
        })
        .copied()
}

fn owner_for_entity_at_tick_on_team(
    ctx: &GameCtx,
    entity_id: usize,
    tick: usize,
    team: usize,
) -> Option<EntityOwnerState> {
    for player_id in 0..16 {
        let Some(player) = ctx.get_player(player_id) else {
            continue;
        };
        if player.team() != team {
            continue;
        }
        let Some(champion) = player.champion() else {
            continue;
        };
        if champion.id() == entity_id {
            return Some(EntityOwnerState {
                ctx_id: combat_ctx_id(ctx),
                entity_id,
                player_id,
                life_id: player.deaths(),
                champion_id: champion_id_for_entity(entity_id),
                last_seen_tick: tick,
            });
        }
    }

    let ctx_id = combat_ctx_id(ctx);
    ENTITY_OWNERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("entity owner state poisoned")
        .iter()
        .rev()
        .find(|state| {
            state.ctx_id == ctx_id
                && state.entity_id == entity_id
                && (tick == usize::MAX || state.last_seen_tick <= tick)
                && (tick == usize::MAX || tick.saturating_sub(state.last_seen_tick) <= 5 * 60 * 60)
                && ctx
                    .get_player(state.player_id)
                    .map(|player| player.team() == team)
                    .unwrap_or(false)
        })
        .copied()
}

pub fn entity_for_player(player_id: usize) -> Option<usize> {
    PLAYER_ENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("player entity state poisoned")
        .iter()
        .rev()
        .find(|state| state.player_id == player_id)
        .map(|state| state.entity_id)
}

pub fn entity_for_player_in_ctx(ctx: &GameCtx, player_id: usize) -> Option<usize> {
    if let Some(entity_id) = ctx
        .get_player(player_id)
        .and_then(|player| player.champion().map(|champion| champion.id()))
    {
        return Some(entity_id);
    }

    let ctx_id = combat_ctx_id(ctx);
    PLAYER_ENTITIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("player entity state poisoned")
        .iter()
        .rev()
        .find(|state| state.ctx_id == ctx_id && state.player_id == player_id)
        .map(|state| state.entity_id)
}

pub fn note_player_return_input(player_id: usize, tick: usize) {
    let entity_id = entity_for_player(player_id);
    let states = PLAYER_RETURN_INTENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("player return intent state poisoned");
    states.retain(|state| tick.saturating_sub(state.tick) <= ATTACH_RETURN_DETACH_WINDOW_TICKS);
    if let Some(existing) = states.iter_mut().find(|state| state.player_id == player_id) {
        existing.entity_id = entity_id;
        existing.tick = tick;
        return;
    }
    states.push(PlayerReturnIntentState {
        player_id,
        entity_id,
        tick,
    });
}

fn entity_recently_chose_return(entity_id: usize, tick: usize) -> bool {
    let player_id = player_for_entity(entity_id);
    PLAYER_RETURN_INTENTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("player return intent state poisoned")
        .iter()
        .any(|state| {
            tick.saturating_sub(state.tick) <= ATTACH_RETURN_DETACH_WINDOW_TICKS
                && (state.entity_id == Some(entity_id) || player_id == Some(state.player_id))
        })
}

fn visible_player_gold(ctx: &GameCtx, entity_id: usize) -> Option<(usize, usize)> {
    let player_id = player_for_entity(entity_id)?;
    let gold = ctx.get_player(player_id)?.gold();
    Some((player_id, gold))
}

pub fn update_gholdengo_gold_passive(ctx: &GameCtx, entity_id: usize) {
    let Some((player_id, visible_gold)) = visible_player_gold(ctx, entity_id) else {
        return;
    };
    let states = GHOLDENGO_GOLD.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("gholdengo gold state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.entity_id == entity_id || state.player_id == player_id)
    {
        if visible_gold > state.last_seen_gold {
            let gained = visible_gold.saturating_sub(state.last_seen_gold);
            state.earned_gold = state.earned_gold.saturating_add(gained);
        }
        state.player_id = player_id;
        state.entity_id = entity_id;
        state.last_seen_gold = visible_gold;
        return;
    }
    states.push(GholdengoGoldState {
        player_id,
        entity_id,
        last_seen_gold: visible_gold,
        earned_gold: 0,
    });
}

pub fn gholdengo_damage_bonus_percent(ctx: &GameCtx, entity_id: usize) -> usize {
    update_gholdengo_gold_passive(ctx, entity_id);
    let Some((player_id, visible_gold)) = visible_player_gold(ctx, entity_id) else {
        return 0;
    };
    let states = GHOLDENGO_GOLD.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("gholdengo gold state poisoned");
    let state_index = if let Some(index) = states
        .iter()
        .position(|state| state.entity_id == entity_id || state.player_id == player_id)
    {
        index
    } else {
        states.push(GholdengoGoldState {
            player_id,
            entity_id,
            last_seen_gold: visible_gold,
            earned_gold: 0,
        });
        states.len().saturating_sub(1)
    };
    let state = &mut states[state_index];
    state.player_id = player_id;
    state.entity_id = entity_id;
    state.last_seen_gold = visible_gold;
    (state.earned_gold / 250).min(20)
}

pub fn has_shell_armor(ctx: &GameCtx, entity_id: usize) -> bool {
    if is_blood_moon_active(ctx, entity_id) {
        return true;
    }
    if is_archaludon_iron_defense_active(ctx, entity_id) {
        return true;
    }
    let states = SHELL_ARMORS.get_or_init(|| Mutex::new(Vec::new()));
    let explicit = states
        .lock()
        .expect("shell armor state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
                && entity_is_champion_id(entity_id, "pokemon_moba_torterra")
        });
    explicit || receiver_has_copied(entity_id, "pokemon_moba_torterra")
}

pub fn begin_archaludon_iron_defense(ctx: &GameCtx, entity_id: usize, ticks: usize) {
    let expires_at = ctx.tick().saturating_add(ticks);
    let states = ARCHALUDON_IRON_DEFENSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("archaludon iron defense state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.expires_at = expires_at;
        return;
    }
    states.push(ArchaludonIronDefenseState {
        entity_id,
        expires_at,
    });
}

pub fn is_archaludon_iron_defense_active(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = ARCHALUDON_IRON_DEFENSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("archaludon iron defense state poisoned");
    states.retain(|state| {
        state.expires_at > tick
            && ctx
                .get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    states.iter().any(|state| state.entity_id == entity_id)
}

pub fn has_stalwart(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_archaludon")
        .unwrap_or(false)
        && ctx
            .get_entity(entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
    explicit || receiver_has_copied(entity_id, "pokemon_moba_archaludon")
}

pub fn has_ripen(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_appletun")
        .unwrap_or(false)
        && ctx
            .get_entity(entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
    explicit || receiver_has_copied(entity_id, "pokemon_moba_appletun")
}

pub fn has_sap_sipper(ctx: &GameCtx, entity_id: usize) -> bool {
    let explicit = champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_goodra")
        .unwrap_or(false)
        && ctx
            .get_entity(entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
    explicit || receiver_has_copied(entity_id, "pokemon_moba_goodra")
}

pub fn has_scrappy(ctx: &GameCtx, entity_id: usize) -> bool {
    let states = SCRAPPYS.get_or_init(|| Mutex::new(Vec::new()));
    let explicit = states
        .lock()
        .expect("scrappy state poisoned")
        .iter()
        .any(|id| {
            *id == entity_id
                && ctx
                    .get_entity(*id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
                && entity_is_champion_id(entity_id, "pokemon_moba_pangoro")
        });
    explicit || receiver_has_copied(entity_id, "pokemon_moba_pangoro")
}

fn cc_duration_ticks(cc: &CCState) -> usize {
    match cc {
        CCState::Stun { tick }
        | CCState::Airborne { tick }
        | CCState::Fear { tick, .. }
        | CCState::Taunt { tick, .. }
        | CCState::Bind { tick }
        | CCState::ForceMove { tick, .. } => (*tick).try_into().unwrap_or(usize::MAX),
        CCState::BlockSkill { tick } | CCState::BlockAttack { tick } => *tick,
        _ => 1,
    }
}

fn cc_is_disruptive(cc: &CCState) -> bool {
    matches!(
        cc,
        CCState::Stun { .. }
            | CCState::Airborne { .. }
            | CCState::Fear { .. }
            | CCState::Taunt { .. }
            | CCState::Bind { .. }
            | CCState::BlockSkill { .. }
            | CCState::BlockAttack { .. }
            | CCState::ForceMove { .. }
    )
}

pub fn apply_pokemon_cc(ctx: &mut GameCtx, source_id: usize, target_id: usize, cc: CCState) {
    let tick = ctx.tick();
    let duration = cc_duration_ticks(&cc).max(1);
    let disruptive = cc_is_disruptive(&cc);
    let events = POKEMON_CC_EVENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut events = events.lock().expect("pokemon cc event state poisoned");
    events.retain(|event| event.expires_at > tick);
    events.push(PokemonCcEvent {
        source_id,
        target_id,
        applied_at: tick,
        expires_at: tick.saturating_add(duration).saturating_add(2),
        disruptive,
    });
    drop(events);
    ctx.apply_cc(target_id, cc);
}

pub fn had_external_disruptive_cc_since(
    ctx: &GameCtx,
    target_id: usize,
    since_tick: usize,
) -> bool {
    let tick = ctx.tick();
    let events = POKEMON_CC_EVENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut events = events.lock().expect("pokemon cc event state poisoned");
    events.retain(|event| event.expires_at > tick);
    events.iter().any(|event| {
        event.target_id == target_id
            && event.source_id != target_id
            && event.disruptive
            && event.applied_at >= since_tick
    })
}

pub fn entity_types(entity_id: usize) -> Option<TypeSet> {
    ENTITY_TYPES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("entity type state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.types)
}

pub fn apply_anti_heal(
    ctx: &mut GameCtx,
    source_id: usize,
    target_id: usize,
    percent: usize,
    ticks: usize,
) {
    if percent == 0 || ticks == 0 {
        return;
    }
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }
    drop(target);

    let percent = percent.min(100);
    let expires_at = ctx.tick().saturating_add(ticks);
    let states = ANTI_HEALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("anti-heal state poisoned");
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.source_id == source_id && state.target_id == target_id)
    {
        existing.percent = existing.percent.max(percent);
        existing.expires_at = existing.expires_at.max(expires_at);
    } else {
        states.push(AntiHealState {
            source_id,
            target_id,
            percent,
            expires_at,
        });
    }
    drop(states);

    ctx.add_buff(
        target_id,
        BuffState {
            duration: BuffType::Time { tick: ticks },
            heal_reduce: percent,
            ..Default::default()
        },
    );
    if let Some(pos) = ctx.get_entity(target_id).map(|target| target.pos()) {
        draw_status_marker(ctx, pos, 9000, VFX_DARK);
    }
}

pub fn anti_heal_percent(ctx: &GameCtx, target_id: usize) -> usize {
    let tick = ctx.tick();
    let states = ANTI_HEALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("anti-heal state poisoned");
    states.retain(|state| state.expires_at > tick);
    states
        .iter()
        .filter(|state| state.target_id == target_id)
        .map(|state| state.percent)
        .max()
        .unwrap_or(0)
}

pub fn heal_with_antiheal(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    amount: usize,
) -> usize {
    if amount == 0 {
        return 0;
    }
    if let Some(missingno_id) = trick_room_inverts_allied_effect(ctx, caster_id, target_id) {
        deal_tracked_damage(
            ctx,
            missingno_id,
            target_id,
            0,
            amount.max(1),
            AttackType::Skill,
        );
        return amount;
    }
    let target_is_champion = ctx
        .get_entity(target_id)
        .map(|entity| entity.is_champion())
        .unwrap_or(false);
    if !target_is_champion {
        log_pokemon_heal_skip(ctx, caster_id, target_id, amount, "non_champion_target");
        return 0;
    }
    let percent = anti_heal_percent(ctx, target_id).min(100);
    let mut adjusted = amount.saturating_mul(100usize.saturating_sub(percent)) / 100;
    if has_ripen(ctx, target_id) {
        adjusted = adjusted.saturating_mul(2);
    }
    if adjusted > 0 {
        let before_hp_state = ctx.get_entity(target_id).map(|entity| entity.hp());
        let before_hp = before_hp_state.map(|hp| hp.current).unwrap_or(0);
        let heal_amount = before_hp_state
            .map(|hp| hp.max.saturating_sub(hp.current).min(adjusted))
            .unwrap_or(0);
        if heal_amount > 0 {
            ctx.heal(caster_id, target_id, heal_amount);
        }
        let after_hp = ctx
            .get_entity(target_id)
            .map(|entity| entity.hp().current)
            .unwrap_or(0);
        let hp_gained = after_hp.saturating_sub(before_hp);
        if hp_gained > 0 {
            let caster_info = combat_stat_source_entity_info(ctx, caster_id);
            let target_info = combat_stat_target_entity_info(ctx, target_id);
            if let (Some(caster_info), Some(target_info)) = (caster_info, target_info) {
                if caster_info.is_champion && caster_info.team == target_info.team {
                    if let Some(caster_player_id) = caster_info.player_id {
                        add_pokemon_healing_for_player(ctx, caster_player_id, hp_gained);
                    }
                    record_pokemon_ally_participation(
                        ctx,
                        caster_info,
                        target_info,
                        PokemonParticipationKind::AllyHealing,
                    );
                }
            }
        }
        let caster_player = log_player_for_entity(caster_id);
        let target_player = log_player_for_entity(target_id);
        let caster_champion = champion_id_for_entity(caster_id).unwrap_or("unknown");
        let target_champion = champion_id_for_entity(target_id).unwrap_or("unknown");
        crate::crash_probe::log_damage_probe(&format!(
            "event=pokemon_heal tick={} caster={} caster_player={} caster_champion=\"{}\" target={} target_player={} target_champion=\"{}\" requested={} adjusted={} native_heal={} anti_heal_percent={} before_hp={} after_hp={} hp_gained={}",
            ctx.tick(),
            caster_id,
            caster_player,
            crate::crash_probe::sanitize_log_field(caster_champion),
            target_id,
            target_player,
            crate::crash_probe::sanitize_log_field(target_champion),
            amount,
            adjusted,
            heal_amount,
            percent,
            before_hp,
            after_hp,
            hp_gained,
        ));
        crate::crash_probe::log_stat_probe(&format!(
            "event=pokemon_heal tick={} caster={} caster_player={} caster_champion=\"{}\" target={} target_player={} target_champion=\"{}\" requested={} adjusted={} native_heal={} anti_heal_percent={} before_hp={} after_hp={} hp_gained={}",
            ctx.tick(),
            caster_id,
            caster_player,
            crate::crash_probe::sanitize_log_field(caster_champion),
            target_id,
            target_player,
            crate::crash_probe::sanitize_log_field(target_champion),
            amount,
            adjusted,
            heal_amount,
            percent,
            before_hp,
            after_hp,
            hp_gained,
        ));
        return hp_gained;
    }
    0
}

fn log_pokemon_heal_skip(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    amount: usize,
    reason: &'static str,
) {
    let caster_player = log_player_for_entity(caster_id);
    let target_player = log_player_for_entity(target_id);
    let caster_champion = champion_id_for_entity(caster_id).unwrap_or("unknown");
    let target_champion = champion_id_for_entity(target_id).unwrap_or("unknown");
    crate::crash_probe::log_stat_probe(&format!(
        "event=pokemon_heal_skipped tick={} caster={} caster_player={} caster_champion=\"{}\" target={} target_player={} target_champion=\"{}\" requested={} reason=\"{}\"",
        ctx.tick(),
        caster_id,
        caster_player,
        crate::crash_probe::sanitize_log_field(caster_champion),
        target_id,
        target_player,
        crate::crash_probe::sanitize_log_field(target_champion),
        amount,
        reason,
    ));
}

fn log_player_for_entity(entity_id: usize) -> String {
    player_for_entity(entity_id)
        .map(|player_id| player_id.to_string())
        .unwrap_or_else(|| "none".to_string())
}

fn log_dot_absorb_heal(
    ctx: &GameCtx,
    dot_type: &str,
    original_caster_id: usize,
    original_target_id: usize,
    healed_entity_id: usize,
    amount: usize,
) {
    let original_caster_player =
        player_for_entity(original_caster_id).unwrap_or(original_caster_id);
    let original_target_player =
        player_for_entity(original_target_id).unwrap_or(original_target_id);
    let healed_player = player_for_entity(healed_entity_id).unwrap_or(healed_entity_id);
    let original_caster_champion = champion_id_for_entity(original_caster_id).unwrap_or("unknown");
    let original_target_champion = champion_id_for_entity(original_target_id).unwrap_or("unknown");
    let healed_champion = champion_id_for_entity(healed_entity_id).unwrap_or("unknown");
    crate::crash_probe::log_damage_probe(&format!(
        "event=dot_absorb_heal dot_type=\"{}\" tick={} original_caster={} original_caster_player={} original_caster_champion=\"{}\" original_target={} original_target_player={} original_target_champion=\"{}\" healed_entity={} healed_player={} healed_champion=\"{}\" amount={}",
        crate::crash_probe::sanitize_log_field(dot_type),
        ctx.tick(),
        original_caster_id,
        original_caster_player,
        crate::crash_probe::sanitize_log_field(original_caster_champion),
        original_target_id,
        original_target_player,
        crate::crash_probe::sanitize_log_field(original_target_champion),
        healed_entity_id,
        healed_player,
        crate::crash_probe::sanitize_log_field(healed_champion),
        amount,
    ));
}

pub fn register_snorlax(entity_id: usize) {
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("snorlax gluttony state poisoned");
    if states.iter().any(|state| state.entity_id == entity_id) {
        return;
    }
    states.push(SnorlaxGluttonyState {
        entity_id,
        berries: 0,
        last_pos: None,
        still_ticks: 0,
        next_move_berry_at: 0,
        next_berry_heal_at: 0,
        next_sleep_heal_at: 0,
        primed_basic_bonus_percent: 0,
        primed_until: 0,
    });
}

pub fn update_snorlax_gluttony(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let pos = entity.pos();
    let hp = entity.hp();
    drop(entity);

    let tick = ctx.tick();
    let mut passive_heal = 0usize;
    let mut sleep_heal = 0usize;
    let mut slow_offset = 0i32;
    let mut draw_full_belly = false;
    {
        let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states.lock().expect("snorlax gluttony state poisoned");
        states.retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive() && entity.is_champion())
                .unwrap_or(false)
        });
        if !states.iter().any(|state| state.entity_id == entity_id) {
            states.push(SnorlaxGluttonyState {
                entity_id,
                berries: 0,
                last_pos: Some(pos),
                still_ticks: 0,
                next_move_berry_at: tick.saturating_add(SNORLAX_BERRY_INTERVAL_TICKS),
                next_berry_heal_at: tick.saturating_add(SNORLAX_PASSIVE_HEAL_INTERVAL_TICKS),
                next_sleep_heal_at: tick.saturating_add(SNORLAX_SLEEP_HEAL_INTERVAL_TICKS),
                primed_basic_bonus_percent: 0,
                primed_until: 0,
            });
        }
        if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
            let moved = state
                .last_pos
                .map(|last_pos| distance_sq(last_pos, pos) >= SNORLAX_MOVE_THRESHOLD_SQ)
                .unwrap_or(false);
            state.last_pos = Some(pos);
            if moved {
                state.still_ticks = 0;
                if tick >= state.next_move_berry_at {
                    state.berries = state.berries.saturating_add(1).min(15);
                    state.next_move_berry_at = tick.saturating_add(SNORLAX_BERRY_INTERVAL_TICKS);
                }
            } else {
                state.still_ticks = state.still_ticks.saturating_add(1);
            }

            if state.next_move_berry_at <= tick {
                state.next_move_berry_at = tick.saturating_add(SNORLAX_BERRY_INTERVAL_TICKS);
            }
            if state.primed_until <= tick {
                state.primed_basic_bonus_percent = 0;
            }
            if state.berries > 0 && tick >= state.next_berry_heal_at {
                passive_heal = hp.max.saturating_mul(state.berries) / 200;
                state.next_berry_heal_at = tick.saturating_add(SNORLAX_PASSIVE_HEAL_INTERVAL_TICKS);
            }
            if state.still_ticks >= SNORLAX_SLEEP_DELAY_TICKS
                && tick >= state.next_sleep_heal_at
                && hp.max > hp.current
            {
                sleep_heal = hp.max.saturating_sub(hp.current).saturating_mul(3) / 100;
                state.next_sleep_heal_at = tick.saturating_add(SNORLAX_SLEEP_HEAL_INTERVAL_TICKS);
            }
            if state.berries >= 10 {
                slow_offset = tracked_harmful_move_speed_reduction(ctx, entity_id);
            }
            draw_full_belly = state.berries >= 15;
        }
    }

    if passive_heal > 0 {
        heal_with_antiheal(ctx, entity_id, entity_id, passive_heal.max(1));
    }
    if sleep_heal > 0 {
        heal_with_antiheal(ctx, entity_id, entity_id, sleep_heal.max(1));
    }
    if slow_offset > 0 {
        ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: 35 },
                move_speed_mult: slow_offset,
                ..Default::default()
            },
        );
    }
    if draw_full_belly {
        ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time {
                    tick: SNORLAX_FULL_BELLY_BUFF_TICKS,
                },
                attack_mult: 20,
                move_speed_mult: -15,
                ..Default::default()
            },
        );
    }
}

fn tracked_harmful_move_speed_reduction(ctx: &GameCtx, target_id: usize) -> i32 {
    prune_tracked_buffs(ctx);
    let tick = ctx.tick();
    let states = TRACKED_BUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("tracked buff ledger poisoned");
    states
        .iter()
        .filter(|state| {
            state.target_id == target_id
                && state.polarity == PokemonBuffPolarity::Harmful
                && state.expires_at > tick
                && state.buff.move_speed_mult < 0
        })
        .map(|state| state.buff.move_speed_mult.abs())
        .sum()
}

pub fn add_snorlax_berries(entity_id: usize, amount: usize) {
    if amount == 0 {
        return;
    }
    register_snorlax(entity_id);
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("snorlax gluttony state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.berries = state.berries.saturating_add(amount).min(15);
    }
}

pub fn consume_snorlax_berries(entity_id: usize, max_berries: usize) -> usize {
    if max_berries == 0 {
        return 0;
    }
    register_snorlax(entity_id);
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("snorlax gluttony state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return 0;
    };
    let consumed = state.berries.min(max_berries);
    state.berries = state.berries.saturating_sub(consumed);
    consumed
}

pub fn snorlax_is_sleeping(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("snorlax gluttony state poisoned");
    states
        .retain(|state| state.primed_until > tick || state.berries > 0 || state.last_pos.is_some());
    states
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.still_ticks >= SNORLAX_SLEEP_DELAY_TICKS)
        .unwrap_or(false)
}

pub fn prime_snorlax_basic(ctx: &GameCtx, entity_id: usize, bonus_percent: usize, ticks: usize) {
    if bonus_percent == 0 || ticks == 0 {
        return;
    }
    register_snorlax(entity_id);
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("snorlax gluttony state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.primed_basic_bonus_percent = bonus_percent;
        state.primed_until = ctx.tick().saturating_add(ticks);
    }
}

pub fn consume_snorlax_primed_basic(ctx: &GameCtx, entity_id: usize) -> usize {
    let tick = ctx.tick();
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("snorlax gluttony state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return 0;
    };
    if state.primed_until <= tick {
        state.primed_basic_bonus_percent = 0;
        return 0;
    }
    let bonus = state.primed_basic_bonus_percent;
    state.primed_basic_bonus_percent = 0;
    state.primed_until = 0;
    bonus
}

pub fn snorlax_full_belly_basic_bonus(ctx: &GameCtx, entity_id: usize) -> usize {
    let _ = ctx;
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("snorlax gluttony state poisoned");
    states
        .iter()
        .find(|state| state.entity_id == entity_id)
        .filter(|state| state.berries >= 15 && state.last_pos.is_some())
        .map(|_| 25)
        .unwrap_or(0)
}

pub fn snorlax_knockback_percent(target_id: usize) -> usize {
    let states = SNORLAX_GLUTTONIES.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("snorlax gluttony state poisoned");
    if states
        .iter()
        .any(|state| state.entity_id == target_id && state.berries >= 10)
    {
        50
    } else {
        100
    }
}

pub fn begin_thievul_hone_claws(
    ctx: &mut GameCtx,
    entity_id: usize,
    ticks: usize,
    anti_heal_percent: usize,
    anti_heal_ticks: usize,
) {
    let expires_at = ctx.tick().saturating_add(ticks);
    let states = THIEVUL_HONE_CLAWS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("thievul hone claws state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.anti_heal_percent = anti_heal_percent;
        existing.anti_heal_ticks = anti_heal_ticks;
        existing.expires_at = expires_at;
    } else {
        states.push(ThievulHoneClawsState {
            entity_id,
            anti_heal_percent,
            anti_heal_ticks,
            expires_at,
        });
    }
}

pub fn apply_thievul_hone_claws_on_basic(ctx: &mut GameCtx, caster_id: usize, target_id: usize) {
    let tick = ctx.tick();
    let state = {
        let states = THIEVUL_HONE_CLAWS.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states.lock().expect("thievul hone claws state poisoned");
        states.retain(|state| state.expires_at > tick);
        states
            .iter()
            .find(|state| state.entity_id == caster_id)
            .copied()
    };
    if let Some(state) = state {
        apply_anti_heal(
            ctx,
            caster_id,
            target_id,
            state.anti_heal_percent,
            state.anti_heal_ticks,
        );
    }
}

pub fn thievul_hone_claws_crit_active(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = THIEVUL_HONE_CLAWS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("thievul hone claws state poisoned");
    states.retain(|state| state.expires_at > tick);
    states.iter().any(|state| state.entity_id == entity_id)
}

pub fn begin_thievul_stakeout(
    ctx: &mut GameCtx,
    caster_id: usize,
    center: EntityPos,
    radius: u64,
    duration_ticks: usize,
    root_ticks: usize,
    damage_bonus_percent: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    let caster_team = caster.team();
    drop(caster);
    let state = ThievulStakeoutState {
        caster_id,
        caster_team,
        center,
        radius,
        root_ticks,
        damage_bonus_percent,
        expires_at: ctx.tick().saturating_add(duration_ticks),
        next_tick_at: ctx.tick(),
    };
    let states = THIEVUL_STAKEOUTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("thievul stakeout state poisoned");
    states.push(state);
    drop(states);
    draw_field_circle(ctx, center, radius, VFX_DARK);
}

pub fn thievul_stakeout_bonus(ctx: &GameCtx, caster_id: usize, target_id: usize) -> Option<usize> {
    let tick = ctx.tick();
    let Some(target_pos) = ctx.get_entity(target_id).map(|target| target.pos()) else {
        return None;
    };
    let states = THIEVUL_STAKEOUTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("thievul stakeout state poisoned");
    states.retain(|state| state.expires_at > tick);
    states
        .iter()
        .find(|state| {
            state.caster_id == caster_id
                && distance_sq(state.center, target_pos)
                    <= state.radius.saturating_mul(state.radius)
        })
        .map(|state| state.damage_bonus_percent)
}

pub fn thievul_stakeout_bypasses_resistance(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
) -> bool {
    thievul_stakeout_bonus(ctx, attacker_id, target_id).is_some()
}

pub fn update_heal_block_aura(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let entity_team = entity.team();
    let entity_pos = entity.pos();
    drop(entity);

    let tick = ctx.tick();
    let should_pulse = {
        let states = THIEVUL_HEAL_BLOCK_AURAS.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states
            .lock()
            .expect("thievul heal block aura state poisoned");
        if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
            if existing
                .last_tick
                .saturating_add(HEAL_BLOCK_AURA_INTERVAL_TICKS)
                > tick
            {
                false
            } else {
                existing.last_tick = tick;
                true
            }
        } else {
            states.push(ThievulHealBlockAuraState {
                entity_id,
                last_tick: tick,
            });
            true
        }
    };
    if !should_pulse {
        return;
    }

    let targets: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != entity_team
                && entity.is_alive()
                && entity.is_champion()
                && distance_sq(entity.pos(), entity_pos)
                    <= HEAL_BLOCK_AURA_RADIUS.saturating_mul(HEAL_BLOCK_AURA_RADIUS)
        })
        .map(|entity| entity.id())
        .collect();
    for target_id in targets {
        apply_anti_heal(
            ctx,
            entity_id,
            target_id,
            35,
            HEAL_BLOCK_AURA_INTERVAL_TICKS + 10,
        );
    }
}

fn flower_veil_blocks_status(ctx: &GameCtx, caster_id: usize, target_id: usize) -> bool {
    const FLOWER_VEIL_RADIUS: u64 = 42000;

    if caster_id == target_id {
        return false;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return false;
    };
    if !target.is_alive() || !target.is_champion() {
        return false;
    }
    let target_team = target.team();
    let target_pos = target.pos();
    drop(target);

    if !entity_types(target_id)
        .map(|types| {
            types
                .iter()
                .any(|pokemon_type| matches!(pokemon_type, PokemonType::Grass))
        })
        .unwrap_or(false)
    {
        return false;
    }

    COMFEY_ATTACHES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("comfey attach state poisoned")
        .iter()
        .filter_map(|state| ctx.get_entity(state.entity_id))
        .filter(|comfey| comfey.is_alive() && comfey.team() == target_team)
        .any(|comfey| {
            distance_sq(comfey.pos(), target_pos)
                <= FLOWER_VEIL_RADIUS.saturating_mul(FLOWER_VEIL_RADIUS)
        })
}

pub fn telepathy_blocks_ally_harm(ctx: &GameCtx, caster_id: usize, target_id: usize) -> bool {
    if caster_id == target_id {
        return false;
    }

    let Some(caster) = ctx.get_entity(caster_id) else {
        return false;
    };
    let Some(target) = ctx.get_entity(target_id) else {
        return false;
    };
    if !caster.is_alive() || !target.is_alive() || caster.team() != target.team() {
        return false;
    }
    drop(caster);
    drop(target);

    champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_orbeetle")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_orbeetle")
}

fn blocks_harmful_status(ctx: &GameCtx, caster_id: usize, target_id: usize) -> bool {
    flower_veil_blocks_status(ctx, caster_id, target_id)
        || telepathy_blocks_ally_harm(ctx, caster_id, target_id)
}

#[allow(dead_code)]
pub fn apply_paralysis(ctx: &GameCtx, target_id: usize) {
    apply_paralysis_for(ctx, target_id, PARALYSIS_DURATION_TICKS);
}

#[allow(dead_code)]
pub fn apply_paralysis_for(ctx: &GameCtx, target_id: usize, duration_ticks: usize) {
    apply_paralysis_from(ctx, target_id, target_id, duration_ticks);
}

pub fn apply_paralysis_from(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    duration_ticks: usize,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }
    if is_status_immune(
        target_id,
        &[
            PokemonType::Electric,
            PokemonType::Ground,
            PokemonType::Rock,
        ],
    ) || is_limber(ctx, target_id)
    {
        return;
    }

    let tick = ctx.tick();
    let states = PARALYSIS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("paralysis state poisoned");

    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == target_id) {
        existing.caster_id = caster_id;
        existing.expires_at = tick + duration_ticks;
        existing.next_roll_at = existing
            .next_roll_at
            .max(tick + PARALYSIS_ROLL_INTERVAL_TICKS);
        drop(states);
        synchronize_paralysis(ctx, caster_id, target_id, duration_ticks);
        return;
    }

    states.push(ParalysisState {
        caster_id,
        entity_id: target_id,
        expires_at: tick + duration_ticks,
        next_roll_at: tick + PARALYSIS_ROLL_INTERVAL_TICKS,
    });
    drop(states);
    synchronize_paralysis(ctx, caster_id, target_id, duration_ticks);
}

pub fn is_paralyzed(ctx: &GameCtx, target_id: usize) -> bool {
    let tick = ctx.tick();
    let states = PARALYSIS.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("paralysis state poisoned")
        .iter()
        .any(|state| state.entity_id == target_id && state.expires_at > tick)
}

pub fn apply_soft_untargetable(ctx: &GameCtx, entity_id: usize, ticks: usize) {
    if ticks == 0 {
        return;
    }
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }

    let tick = ctx.tick();
    let expires_at = tick.saturating_add(ticks);
    let states = SOFT_UNTARGETABLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("soft untargetable state poisoned");
    states.retain(|state| state.expires_at > tick);

    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.expires_at = existing.expires_at.max(expires_at);
        return;
    }

    states.push(SoftUntargetableState {
        entity_id,
        expires_at,
    });
}

pub fn clear_soft_untargetable(entity_id: usize) {
    let states = SOFT_UNTARGETABLES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("soft untargetable state poisoned")
        .retain(|state| state.entity_id != entity_id);
}

pub fn is_soft_untargetable(ctx: &GameCtx, entity_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    if !entity.is_alive() {
        return false;
    }

    let tick = ctx.tick();
    let hidden_by_weavile = is_weavile_hidden_from_pokemon_ai_at(entity_id, tick);
    SOFT_UNTARGETABLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("soft untargetable state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && state.expires_at > tick
                && !(hidden_by_weavile && ampharos_true_sight_reveals(ctx, entity_id))
        })
}

pub fn begin_weavile_hunt_stealth(ctx: &GameCtx, entity_id: usize, stealth_ticks: usize) {
    if stealth_ticks == 0 {
        return;
    }
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }

    let expires_at = ctx.tick().saturating_add(stealth_ticks);
    let states = WEAVILE_HUNTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("weavile hunt state poisoned");
    states.retain(|state| state.stealth_expires_at > ctx.tick());
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.stealth_expires_at = existing.stealth_expires_at.max(expires_at);
    } else {
        states.push(WeavileHuntState {
            entity_id,
            stealth_expires_at: expires_at,
        });
    }
    drop(states);

    apply_soft_untargetable(ctx, entity_id, stealth_ticks);
}

pub fn break_weavile_hunt_stealth(entity_id: usize) {
    WEAVILE_HUNTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("weavile hunt state poisoned")
        .retain(|state| state.entity_id != entity_id);
    clear_soft_untargetable(entity_id);
}

pub fn is_weavile_hidden_from_pokemon_ai_at(entity_id: usize, tick: usize) -> bool {
    WEAVILE_HUNTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("weavile hunt state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.stealth_expires_at > tick)
}

pub fn is_weavile_hidden_from_pokemon_ai(ctx: &GameCtx, entity_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    entity.is_alive()
        && is_weavile_hidden_from_pokemon_ai_at(entity_id, ctx.tick())
        && !ampharos_true_sight_reveals(ctx, entity_id)
}

fn has_ampharos_luminous_pulse(entity_id: usize) -> bool {
    entity_is_champion_id(entity_id, "pokemon_moba_ampharos")
        || receiver_has_copied(entity_id, "pokemon_moba_ampharos")
}

pub fn ampharos_true_sight_reveals(ctx: &GameCtx, target_id: usize) -> bool {
    let Some(target) = ctx.get_entity(target_id) else {
        return false;
    };
    if !target.is_alive() {
        return false;
    }
    let target_team = target.team();
    let target_pos = target.pos();
    drop(target);

    let tick = ctx.tick();
    let states = AMPHAROS_TRUE_SIGHTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("ampharos true sight state poisoned");
    states.retain(|state| state.expires_at > tick);
    states.iter().any(|state| {
        if state.team == target_team {
            return false;
        }
        let Some(source) = ctx.get_entity(state.source_id) else {
            return false;
        };
        source.is_alive()
            && distance_sq(source.pos(), target_pos) <= state.radius.saturating_mul(state.radius)
    })
}

fn refresh_ampharos_true_sight(ctx: &GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let team = entity.team();
    drop(entity);

    let tick = ctx.tick();
    let states = AMPHAROS_TRUE_SIGHTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("ampharos true sight state poisoned");
    states.retain(|state| state.expires_at > tick && state.source_id != entity_id);
    states.push(AmpharosTrueSightState {
        source_id: entity_id,
        team,
        radius: AMPHAROS_LUMINOUS_RADIUS,
        expires_at: tick.saturating_add(AMPHAROS_TRUE_SIGHT_REFRESH_TICKS),
    });
}

fn active_searchlight_tail(entity_id: usize, tick: usize) -> Option<AmpharosSearchlightTailState> {
    let states = AMPHAROS_SEARCHLIGHT_TAILS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("ampharos searchlight tail state poisoned");
    states.retain(|state| state.expires_at > tick);
    states
        .iter()
        .find(|state| state.entity_id == entity_id)
        .copied()
}

fn extend_searchlight_tail(
    ctx: &mut GameCtx,
    entity_id: usize,
    tick: usize,
    extend_ticks: usize,
    hits: usize,
) {
    if hits == 0 || extend_ticks == 0 {
        return;
    }
    let mut move_speed_mult = 0;
    let states = AMPHAROS_SEARCHLIGHT_TAILS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("ampharos searchlight tail state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.entity_id == entity_id && state.expires_at > tick)
    {
        move_speed_mult = state.move_speed_mult;
        state.expires_at = state
            .expires_at
            .saturating_add(extend_ticks.saturating_mul(hits));
    }
    drop(states);

    if move_speed_mult != 0 {
        add_beneficial_buff(
            ctx,
            entity_id,
            entity_id,
            BuffState {
                duration: BuffType::Time {
                    tick: extend_ticks.saturating_mul(hits),
                },
                move_speed_mult,
                ..Default::default()
            },
        );
    }
}

pub fn begin_ampharos_searchlight_tail(
    ctx: &mut GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    move_speed_mult: i32,
    damage_bonus_percent: usize,
    extend_per_champion_ticks: usize,
    paralysis_chance_percent: usize,
    paralysis_ticks: usize,
) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    drop(entity);

    let tick = ctx.tick();
    let expires_at = tick.saturating_add(duration_ticks);
    let states = AMPHAROS_SEARCHLIGHT_TAILS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("ampharos searchlight tail state poisoned");
    states.retain(|state| state.entity_id != entity_id && state.expires_at > tick);
    states.push(AmpharosSearchlightTailState {
        entity_id,
        expires_at,
        move_speed_mult,
        damage_bonus_percent,
        extend_per_champion_ticks,
        paralysis_chance_percent,
        paralysis_ticks,
    });
    drop(states);

    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: duration_ticks,
            },
            move_speed_mult,
            ..Default::default()
        },
    );
}

pub fn apply_ampharos_flash(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    caster_pos: EntityPos,
    damage: usize,
    radius: u64,
    slow_percent: i32,
    slow_ticks: usize,
    silence_ticks: usize,
    attacker_types: TypeSet,
) {
    let radius_sq = radius.saturating_mul(radius);
    let targets: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != caster_team
                && entity.is_alive()
                && !entity.is_tower()
                && distance_sq(entity.pos(), caster_pos) <= radius_sq
        })
        .map(|entity| entity.id())
        .collect();

    for target_id in targets {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage.max(1),
            AttackType::Skill,
            PokemonType::Electric,
            attacker_types,
            defender_types,
        );
        add_harmful_buff(
            ctx,
            caster_id,
            target_id,
            BuffState {
                duration: BuffType::Time { tick: slow_ticks },
                move_speed_mult: -slow_percent.abs(),
                ..Default::default()
            },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill {
                tick: silence_ticks,
            },
        );
    }
    draw_status_marker(ctx, caster_pos, radius, VFX_ELECTRIC);
}

pub fn update_ampharos_luminous_pulse(ctx: &mut GameCtx, entity_id: usize) {
    if !has_ampharos_luminous_pulse(entity_id) {
        return;
    }
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let team = entity.team();
    let pos = entity.pos();
    let magic_power = entity.stat().magic_power;
    drop(entity);

    refresh_ampharos_true_sight(ctx, entity_id);

    let tick = ctx.tick();
    let states = AMPHAROS_LUMINOUS_PULSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("ampharos luminous pulse state poisoned");
    states.retain(|state| {
        state.entity_id == entity_id
            || ctx
                .get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    let state = if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id)
    {
        existing
    } else {
        states.push(AmpharosLuminousPulseState {
            entity_id,
            next_pulse_at: tick.saturating_add(AMPHAROS_LUMINOUS_INTERVAL_TICKS),
        });
        states
            .last_mut()
            .expect("inserted ampharos luminous pulse state")
    };
    if state.next_pulse_at > tick {
        return;
    }
    state.next_pulse_at = tick.saturating_add(AMPHAROS_LUMINOUS_INTERVAL_TICKS);
    drop(states);

    let searchlight = active_searchlight_tail(entity_id, tick);
    let mut damage = AMPHAROS_LUMINOUS_BASE_AP
        .saturating_add(magic_power.saturating_mul(AMPHAROS_LUMINOUS_AP_RATIO) / 100);
    if let Some(state) = searchlight {
        damage = damage.saturating_mul(100 + state.damage_bonus_percent) / 100;
    }

    let radius_sq = AMPHAROS_LUMINOUS_RADIUS.saturating_mul(AMPHAROS_LUMINOUS_RADIUS);
    let targets: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != team
                && entity.is_alive()
                && !entity.is_tower()
                && distance_sq(entity.pos(), pos) <= radius_sq
        })
        .map(|entity| entity.id())
        .collect();

    let mut champion_hits = 0usize;
    for target_id in targets {
        let target_is_champion = ctx
            .get_entity(target_id)
            .map(|target| target.is_champion())
            .unwrap_or(false);
        if target_is_champion {
            champion_hits = champion_hits.saturating_add(1);
        }
        note_web_walker_spot(
            tick,
            entity_id,
            team,
            target_id,
            ctx.get_entity(target_id)
                .map(|target| target.pos())
                .unwrap_or(pos),
        );
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            entity_id,
            target_id,
            0,
            damage.max(1),
            AttackType::Skill,
            PokemonType::Electric,
            TypeSet::single(PokemonType::Electric),
            defender_types,
        );
        add_harmful_buff(
            ctx,
            entity_id,
            target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: AMPHAROS_LUMINOUS_SLOW_TICKS,
                },
                move_speed_mult: -AMPHAROS_LUMINOUS_SLOW_PERCENT.abs(),
                ..Default::default()
            },
        );
        if let Some(state) = searchlight {
            if chance_percent(
                ctx.seed(),
                entity_id,
                target_id,
                tick,
                state.paralysis_chance_percent,
            ) {
                apply_paralysis_from(ctx, entity_id, target_id, state.paralysis_ticks);
            }
        }
    }

    if let Some(state) = searchlight {
        extend_searchlight_tail(
            ctx,
            entity_id,
            tick,
            state.extend_per_champion_ticks,
            champion_hits,
        );
    }
    draw_status_marker(ctx, pos, AMPHAROS_LUMINOUS_RADIUS, VFX_ELECTRIC);
}

pub fn begin_ampharos_gigavolt(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    damage: usize,
    radius: u64,
    channel_ticks: usize,
    zone_duration_ticks: usize,
    zone_tick_interval: usize,
    zone_slow_percent: i32,
    zone_slow_ticks: usize,
    attack_speed_mult: i32,
    attack_speed_buff_ticks: usize,
    attacker_types: TypeSet,
) {
    let tick = ctx.tick();
    let states = AMPHAROS_GIGAVOLTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("ampharos gigavolt state poisoned");
    states.push(AmpharosGigavoltState {
        caster_id,
        caster_team,
        center,
        trigger_at: tick.saturating_add(channel_ticks),
        expires_at: tick
            .saturating_add(channel_ticks)
            .saturating_add(zone_duration_ticks),
        next_tick_at: tick.saturating_add(channel_ticks),
        damage,
        radius,
        zone_tick_interval,
        zone_slow_percent,
        zone_slow_ticks,
        attack_speed_mult,
        attack_speed_buff_ticks,
        attacker_types,
        triggered: false,
    });
    drop(states);

    apply_pokemon_cc(
        ctx,
        caster_id,
        caster_id,
        CCState::BlockSkill {
            tick: channel_ticks,
        },
    );
}

pub fn has_lone_predator(_ctx: &GameCtx, entity_id: usize) -> bool {
    entity_is_champion_id(entity_id, "pokemon_moba_weavile")
        || receiver_has_copied(entity_id, "pokemon_moba_weavile")
}

pub fn target_is_lone_predator_isolated(ctx: &GameCtx, target_id: usize, radius: u64) -> bool {
    let Some(target) = ctx.get_entity(target_id) else {
        return false;
    };
    if !target.is_alive() {
        return false;
    }
    let target_team = target.team();
    let target_pos = target.pos();
    drop(target);

    let radius_sq = radius.saturating_mul(radius);
    for index in 0..ctx.entity_count() {
        let Some(ally) = ctx.entity_at(index) else {
            continue;
        };
        if ally.id() == target_id
            || ally.team() != target_team
            || !ally.is_alive()
            || !ally.is_champion()
        {
            continue;
        }
        if distance_sq(ally.pos(), target_pos) <= radius_sq {
            return false;
        }
    }
    true
}

pub fn apply_confusion(ctx: &GameCtx, entity_id: usize, stacks: usize, ticks: usize) {
    apply_confusion_from(ctx, entity_id, entity_id, stacks, ticks);
}

pub fn apply_confusion_from(
    ctx: &GameCtx,
    caster_id: usize,
    entity_id: usize,
    stacks: usize,
    ticks: usize,
) {
    if stacks == 0 || ticks == 0 {
        return;
    }
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_champion() || !entity.is_alive() {
        return;
    }
    if blocks_harmful_status(ctx, caster_id, entity_id) {
        return;
    }

    let tick = ctx.tick();
    let expires_at = tick.saturating_add(ticks);
    let states = CONFUSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("confusion state poisoned");
    states.retain(|state| state.expires_at > tick && state.stacks > 0);

    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.stacks = existing.stacks.saturating_add(stacks).min(9);
        existing.expires_at = existing.expires_at.max(expires_at);
        drop(states);
        synchronize_confusion(ctx, caster_id, entity_id, stacks, ticks);
        return;
    }

    states.push(ConfusionState {
        entity_id,
        stacks,
        expires_at,
    });
    drop(states);
    synchronize_confusion(ctx, caster_id, entity_id, stacks, ticks);
}

pub fn schedule_confusion_from(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    delay_ticks: usize,
    stacks: usize,
    ticks: usize,
) {
    if stacks == 0 || ticks == 0 {
        return;
    }
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_champion() || !target.is_alive() {
        return;
    }
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }
    drop(target);

    let trigger_at = ctx.tick().saturating_add(delay_ticks);
    let states = DELAYED_CONFUSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("delayed confusion state poisoned");
    states.retain(|state| state.trigger_at > ctx.tick());
    states.push(DelayedConfusionState {
        caster_id,
        target_id,
        trigger_at,
        stacks,
        ticks,
    });
}

pub fn consume_confusion(ctx: &GameCtx, entity_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    if !entity.is_champion() || !entity.is_alive() {
        return false;
    }

    let tick = ctx.tick();
    let states = CONFUSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("confusion state poisoned");
    let mut consumed = false;
    states.retain_mut(|state| {
        if state.expires_at <= tick || state.stacks == 0 {
            return false;
        }
        if state.entity_id == entity_id && !consumed {
            state.stacks = state.stacks.saturating_sub(1);
            consumed = true;
        }
        state.stacks > 0
    });
    consumed
}

pub fn note_mega_launcher_hit(ctx: &GameCtx, caster_id: usize, target_id: usize) -> usize {
    let tick = ctx.tick();
    let states = MEGA_LAUNCHER.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("mega launcher state poisoned");
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id)
    {
        if tick.saturating_sub(existing.last_hit_tick) <= MEGA_LAUNCHER_WINDOW_TICKS {
            existing.stacks += 1;
        } else {
            existing.stacks = 1;
        }
        existing.last_hit_tick = tick;
        return existing.stacks.saturating_mul(MEGA_LAUNCHER_CRIT_PER_HIT);
    }

    states.push(MegaLauncherState {
        caster_id,
        target_id,
        stacks: 1,
        last_hit_tick: tick,
    });
    MEGA_LAUNCHER_CRIT_PER_HIT
}

pub fn is_crowd_controlled(ctx: &GameCtx, target_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(target_id) else {
        return false;
    };
    (0..entity.cc_count()).any(|index| entity.cc_at(index).cc_type != 255)
}

pub fn apply_airborne_hard_cc(ctx: &mut GameCtx, caster_id: usize, target_id: usize, ticks: usize) {
    if is_limber(ctx, target_id) {
        return;
    }
    let ticks = adjusted_cc_ticks(ctx, target_id, ticks);
    break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
    apply_pokemon_cc(
        ctx,
        caster_id,
        target_id,
        CCState::Airborne { tick: ticks as u64 },
    );
    note_steadfast_cc(ctx, target_id);
}

pub fn update_tangling_vines_aura(ctx: &mut GameCtx, venusaur_id: usize) {
    let tick = ctx.tick();
    let aura_states = TANGLING_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut aura_states = aura_states.lock().expect("tangling aura state poisoned");
        if let Some(existing) = aura_states
            .iter_mut()
            .find(|state| state.venusaur_id == venusaur_id)
        {
            if tick.saturating_sub(existing.last_tick) < TANGLING_AURA_INTERVAL_TICKS {
                return;
            }
            existing.last_tick = tick;
        } else {
            aura_states.push(TanglingAuraState {
                venusaur_id,
                last_tick: tick,
            });
        }
    }

    let Some(venusaur) = ctx.get_entity(venusaur_id) else {
        return;
    };
    if !venusaur.is_alive() {
        return;
    }
    let venusaur_team = venusaur.team();
    let venusaur_pos = venusaur.pos();

    let cc_enemy_nearby = (0..ctx.entity_count()).any(|index| {
        let Some(entity) = ctx.entity_at(index) else {
            return false;
        };
        entity.team() != venusaur_team
            && entity.is_champion()
            && entity.is_alive()
            && is_crowd_controlled(ctx, entity.id())
            && distance_sq(entity.pos(), venusaur_pos)
                <= TANGLING_TRIGGER_RADIUS.saturating_mul(TANGLING_TRIGGER_RADIUS)
    });

    if !cc_enemy_nearby {
        return;
    }

    let ally_ids: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() == venusaur_team
                && entity.is_champion()
                && entity.is_alive()
                && distance_sq(entity.pos(), venusaur_pos)
                    <= TANGLING_AURA_RADIUS.saturating_mul(TANGLING_AURA_RADIUS)
        })
        .map(|entity| entity.id())
        .collect();

    for ally_id in ally_ids {
        add_beneficial_buff(
            ctx,
            venusaur_id,
            ally_id,
            BuffState {
                duration: BuffType::Time { tick: 45 },
                attack_speed_mult: 25,
                attack_mult: 15,
                ..Default::default()
            },
        );
        note_ally_buff_received(ctx, venusaur_id, ally_id, 45);
    }
}

pub fn update_helping_hand_aura(ctx: &mut GameCtx, eevee_id: usize) {
    let tick = ctx.tick();
    let aura_states = HELPING_HAND_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut aura_states = aura_states
            .lock()
            .expect("helping hand aura state poisoned");
        if let Some(existing) = aura_states
            .iter_mut()
            .find(|state| state.eevee_id == eevee_id)
        {
            if tick.saturating_sub(existing.last_tick) < HELPING_HAND_INTERVAL_TICKS {
                return;
            }
            existing.last_tick = tick;
        } else {
            aura_states.push(HelpingHandAuraState {
                eevee_id,
                last_tick: tick,
            });
        }
    }

    let Some(eevee) = ctx.get_entity(eevee_id) else {
        return;
    };
    if !eevee.is_alive() {
        return;
    }
    let eevee_team = eevee.team();
    let eevee_pos = eevee.pos();

    let ally_ids: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() == eevee_team
                && entity.is_champion()
                && entity.is_alive()
                && distance_sq(entity.pos(), eevee_pos)
                    <= HELPING_HAND_RADIUS.saturating_mul(HELPING_HAND_RADIUS)
        })
        .map(|entity| entity.id())
        .collect();

    for ally_id in ally_ids {
        ctx.add_buff(
            ally_id,
            BuffState {
                duration: BuffType::Time { tick: 45 },
                attack_mult: 10,
                magic_power_mult: 10,
                defence_mult: 10,
                magic_resistance_mult: 10,
                hp_mult: 10,
                skill_cooldown_mult: 10,
                ult_cooldown_mult: 10,
                ..Default::default()
            },
        );
        note_ally_buff_received(ctx, eevee_id, ally_id, 45);
    }
}

pub fn note_houndoom_damaged(ctx: &GameCtx, houndoom_id: usize) {
    let states = HOUNDOOM_DAMAGE.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("houndoom damage state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.entity_id == houndoom_id)
    {
        state.last_damaged_tick = ctx.tick();
    } else {
        states.push(HoundoomDamageState {
            entity_id: houndoom_id,
            last_damaged_tick: ctx.tick(),
        });
    }
}

pub fn houndoom_foul_play_bonus(ctx: &GameCtx, houndoom_id: usize) -> usize {
    let states = HOUNDOOM_DAMAGE.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("houndoom damage state poisoned");
    let recently_damaged = states
        .iter()
        .find(|state| state.entity_id == houndoom_id)
        .map(|state| {
            ctx.tick().saturating_sub(state.last_damaged_tick) < HOUNDOOM_FOUL_PLAY_WINDOW_TICKS
        })
        .unwrap_or(false);
    if recently_damaged {
        0
    } else {
        35
    }
}

pub fn update_intimidate_aura(ctx: &mut GameCtx, houndoom_id: usize) {
    let tick = ctx.tick();
    let aura_states = INTIMIDATE_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut aura_states = aura_states.lock().expect("intimidate aura state poisoned");
        if let Some(existing) = aura_states
            .iter_mut()
            .find(|state| state.houndoom_id == houndoom_id)
        {
            if tick.saturating_sub(existing.last_tick) < INTIMIDATE_INTERVAL_TICKS {
                return;
            }
            existing.last_tick = tick;
        } else {
            aura_states.push(IntimidateAuraState {
                houndoom_id,
                last_tick: tick,
            });
        }
    }

    let Some(houndoom) = ctx.get_entity(houndoom_id) else {
        return;
    };
    if !houndoom.is_alive() {
        return;
    }
    let team = houndoom.team();
    let pos = houndoom.pos();

    let enemy_ids: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != team
                && entity.is_alive()
                && distance_sq(entity.pos(), pos)
                    <= INTIMIDATE_RADIUS.saturating_mul(INTIMIDATE_RADIUS)
        })
        .map(|entity| entity.id())
        .collect();

    for enemy_id in enemy_ids {
        ctx.add_buff(
            enemy_id,
            BuffState {
                duration: BuffType::Time { tick: 45 },
                attack_mult: -15,
                ..Default::default()
            },
        );
    }
}

pub fn is_clawitzer_clinging(ctx: &GameCtx, entity_id: usize) -> bool {
    is_clawitzer_attached_at(entity_id, ctx.tick())
}

pub fn is_clawitzer_attached_at(entity_id: usize, tick: usize) -> bool {
    CLAWITZER_CLINGS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("clawitzer cling state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && state.attached_ally.is_some()
                && state.detached_until <= tick
        })
}

pub fn is_clawitzer_attach_ready_at(entity_id: usize, tick: usize) -> bool {
    CLAWITZER_CLINGS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("clawitzer cling state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.attached_ally.is_none() && state.detached_until <= tick)
        .unwrap_or(false)
}

pub fn comfey_attached_ally(ctx: &GameCtx, entity_id: usize) -> Option<usize> {
    comfey_attached_ally_at(entity_id, ctx.tick())
}

pub fn is_comfey_attached_at(entity_id: usize, tick: usize) -> bool {
    comfey_attached_ally_at(entity_id, tick).is_some()
}

pub fn comfey_attached_ally_at(entity_id: usize, tick: usize) -> Option<usize> {
    COMFEY_ATTACHES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("comfey attach state poisoned")
        .iter()
        .find(|state| {
            state.entity_id == entity_id
                && state.attached_ally.is_some()
                && state.detached_until <= tick
        })
        .and_then(|state| state.attached_ally)
}

pub fn is_comfey_attach_ready_at(entity_id: usize, tick: usize) -> bool {
    COMFEY_ATTACHES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("comfey attach state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.attached_ally.is_none() && state.detached_until <= tick)
        .unwrap_or(false)
}

pub fn update_flower_veil_passive(ctx: &mut GameCtx, comfey_id: usize) {
    let tick = ctx.tick();
    let Some(comfey) = ctx.get_entity(comfey_id) else {
        return;
    };
    if !comfey.is_alive() {
        return;
    }
    let comfey_team = comfey.team();
    let comfey_pos = comfey.pos();
    let comfey_stat = comfey.stat();
    drop(comfey);

    let states = COMFEY_ATTACHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("comfey attach state poisoned");
    let index = if let Some(index) = states.iter().position(|state| state.entity_id == comfey_id) {
        index
    } else {
        states.push(ComfeyAttachState {
            entity_id: comfey_id,
            attached_ally: None,
            detached_until: tick.saturating_add(COMFEY_INITIAL_ATTACH_DELAY_TICKS),
            last_tick: 0,
        });
        states.len().saturating_sub(1)
    };

    let mut attach_target = None;
    let mut detach = false;
    let mut should_refresh = false;

    if let Some(ally_id) = states[index].attached_ally {
        let ally_alive = ctx
            .get_entity(ally_id)
            .map(|ally| ally.is_alive() && ally.hp().current > 0)
            .unwrap_or(false);
        if !ally_alive || entity_recently_chose_return(ally_id, tick) {
            detach = true;
        } else if tick.saturating_sub(states[index].last_tick) >= COMFEY_ATTACH_INTERVAL_TICKS {
            states[index].last_tick = tick;
            should_refresh = true;
        }
    } else if tick >= states[index].detached_until {
        attach_target = nearest_ally_champion(
            ctx,
            comfey_id,
            comfey_team,
            comfey_pos,
            COMFEY_REATTACH_RADIUS,
        );
    }

    if detach {
        states[index].attached_ally = None;
        states[index].detached_until = tick.saturating_add(COMFEY_REATTACH_TICKS);
        states[index].last_tick = tick;
        drop(states);
        return;
    }

    if let Some(ally_id) = attach_target {
        states[index].attached_ally = Some(ally_id);
        states[index].last_tick = 0;
        should_refresh = true;
    }

    let attached_ally = states[index].attached_ally;
    drop(states);

    let Some(ally_id) = attached_ally else {
        return;
    };

    if should_refresh {
        ctx.add_buff(
            ally_id,
            BuffState {
                duration: BuffType::Time {
                    tick: COMFEY_ATTACH_BUFF_TICKS,
                },
                hp: stat_percent_i32(comfey_stat.hp, 99),
                attack: stat_percent_i32(comfey_stat.attack, 99),
                magic_power: stat_percent_i32(comfey_stat.magic_power, 99),
                defence: stat_percent_i32(comfey_stat.defence, 99),
                magic_resistance: stat_percent_i32(comfey_stat.magic_resistance, 99),
                ..Default::default()
            },
        );
        apply_soft_untargetable(ctx, comfey_id, COMFEY_ATTACH_BUFF_TICKS);
        apply_pokemon_cc(
            ctx,
            comfey_id,
            comfey_id,
            CCState::Bind {
                tick: COMFEY_ATTACH_BUFF_TICKS as u64,
            },
        );
        apply_attach_tether(ctx, comfey_id, comfey_pos, ally_id, 11000, VFX_FAIRY);
    }
}

pub fn update_clingy_passive(ctx: &mut GameCtx, clawitzer_id: usize) {
    let tick = ctx.tick();
    let Some(clawitzer) = ctx.get_entity(clawitzer_id) else {
        return;
    };
    if !clawitzer.is_alive() {
        return;
    }
    let claw_team = clawitzer.team();
    let claw_pos = clawitzer.pos();
    let claw_stat = clawitzer.stat();
    drop(clawitzer);

    let states = CLAWITZER_CLINGS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("clawitzer cling state poisoned");
    let index = if let Some(index) = states
        .iter()
        .position(|state| state.entity_id == clawitzer_id)
    {
        index
    } else {
        states.push(ClawitzerClingState {
            entity_id: clawitzer_id,
            attached_ally: None,
            detached_until: tick.saturating_add(CLAWITZER_INITIAL_ATTACH_DELAY_TICKS),
            last_ally_hp: 0,
            last_tick: 0,
        });
        states.len().saturating_sub(1)
    };

    let mut attach_target = None;
    let mut detach = false;
    let mut should_refresh = false;

    if let Some(ally_id) = states[index].attached_ally {
        let ally_hp = ctx.get_entity(ally_id).map(|ally| ally.hp());
        if let Some(ally_hp) = ally_hp {
            let hp_drop = states[index].last_ally_hp.saturating_sub(ally_hp.current);
            if ally_hp.current == 0
                || entity_recently_chose_return(ally_id, tick)
                || hp_drop.saturating_mul(3) >= ally_hp.max
            {
                detach = true;
            } else if tick.saturating_sub(states[index].last_tick) >= CLAWITZER_CLING_INTERVAL_TICKS
            {
                states[index].last_tick = tick;
                states[index].last_ally_hp = ally_hp.current;
                should_refresh = true;
            }
        } else {
            detach = true;
        }
    } else if tick >= states[index].detached_until {
        attach_target = nearest_ally_champion(
            ctx,
            clawitzer_id,
            claw_team,
            claw_pos,
            CLAWITZER_REATTACH_RADIUS,
        );
    }

    if detach {
        states[index].attached_ally = None;
        states[index].detached_until = tick.saturating_add(CLAWITZER_REATTACH_TICKS);
        states[index].last_ally_hp = 0;
        states[index].last_tick = tick;
        drop(states);
        ctx.add_buff(
            clawitzer_id,
            BuffState {
                duration: BuffType::Time {
                    tick: CLAWITZER_REATTACH_TICKS,
                },
                move_speed_mult: 45,
                ..Default::default()
            },
        );
        return;
    }

    if let Some(ally_id) = attach_target {
        let Some(ally) = ctx.get_entity(ally_id) else {
            return;
        };
        let ally_hp = ally.hp().current;
        drop(ally);
        states[index].attached_ally = Some(ally_id);
        states[index].last_ally_hp = ally_hp;
        states[index].last_tick = 0;
        should_refresh = true;
    }

    let attached_ally = states[index].attached_ally;
    drop(states);

    let Some(ally_id) = attached_ally else {
        return;
    };

    if should_refresh {
        ctx.add_buff(
            ally_id,
            BuffState {
                duration: BuffType::Time {
                    tick: CLAWITZER_CLING_BUFF_TICKS,
                },
                hp: stat_percent_i32(claw_stat.hp, 99),
                defence: stat_percent_i32(claw_stat.defence, 99),
                magic_resistance: stat_percent_i32(claw_stat.magic_resistance, 99),
                ..Default::default()
            },
        );
        apply_soft_untargetable(ctx, clawitzer_id, CLAWITZER_CLING_BUFF_TICKS);
        apply_pokemon_cc(
            ctx,
            clawitzer_id,
            clawitzer_id,
            CCState::Bind {
                tick: CLAWITZER_CLING_BUFF_TICKS as u64,
            },
        );
        apply_attach_tether(ctx, clawitzer_id, claw_pos, ally_id, 12000, VFX_WATER);
    }
}

fn nearest_ally_champion(
    ctx: &GameCtx,
    entity_id: usize,
    team: usize,
    pos: EntityPos,
    radius: u64,
) -> Option<usize> {
    let tick = ctx.tick();
    let mut best = None;
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        let candidate_id = entity.id();
        let is_player_pokemon = entity.is_champion()
            || player_life_for_entity_at_tick_on_team(ctx, candidate_id, tick, team).is_some()
            || owner_for_entity_at_tick_on_team(ctx, candidate_id, tick, team).is_some();
        if entity.id() == entity_id
            || entity.team() != team
            || !is_player_pokemon
            || !entity.is_alive()
        {
            continue;
        }
        let distance = distance_sq(entity.pos(), pos);
        if distance > radius.saturating_mul(radius) {
            continue;
        }
        if best
            .map(|(best_distance, _)| distance < best_distance)
            .unwrap_or(true)
        {
            best = Some((distance, candidate_id));
        }
    }
    best.map(|(_, id)| id)
}

fn apply_attach_tether(
    ctx: &mut GameCtx,
    entity_id: usize,
    entity_pos: EntityPos,
    ally_id: usize,
    marker_radius: u64,
    marker_color: u32,
) {
    let Some(ally) = ctx.get_entity(ally_id) else {
        return;
    };
    if !ally.is_alive() {
        return;
    }
    let ally_pos = ally.pos();
    drop(ally);

    let distance = distance_sq(entity_pos, ally_pos);
    if distance <= ATTACH_TETHER_DEADZONE.saturating_mul(ATTACH_TETHER_DEADZONE) {
        draw_status_marker(ctx, ally_pos, marker_radius, marker_color);
        return;
    }

    let dx = ally_pos.x as i64 - entity_pos.x as i64;
    let dy = ally_pos.y as i64 - entity_pos.y as i64;
    apply_pokemon_cc(
        ctx,
        entity_id,
        entity_id,
        CCState::ForceMove {
            tick: ATTACH_TETHER_TICKS,
            dx,
            dy,
            speed: ATTACH_TETHER_SPEED,
        },
    );
    draw_status_marker(ctx, ally_pos, marker_radius, marker_color);
}

fn stat_percent_i32(value: usize, percent: usize) -> i32 {
    value
        .saturating_mul(percent)
        .saturating_div(100)
        .min(i32::MAX as usize) as i32
}

pub fn apply_octillery_lock_on(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    crit_chance: usize,
    ticks: usize,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || !target.is_champion() {
        return;
    }
    drop(target);

    let expires_at = ctx.tick().saturating_add(ticks);
    let states = OCTILLERY_LOCK_ONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("octillery lock-on state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id)
    {
        state.crit_chance = crit_chance;
        state.expires_at = expires_at;
    } else {
        states.push(OctilleryLockOnState {
            caster_id,
            target_id,
            crit_chance,
            expires_at,
        });
    }
}

pub fn octillery_lock_on_bonus(ctx: &GameCtx, caster_id: usize, target_id: usize) -> usize {
    let tick = ctx.tick();
    OCTILLERY_LOCK_ONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("octillery lock-on state poisoned")
        .iter()
        .find(|state| {
            state.caster_id == caster_id && state.target_id == target_id && state.expires_at > tick
        })
        .map(|state| state.crit_chance)
        .unwrap_or(0)
}

pub fn update_eeveelution_passive(ctx: &mut GameCtx, entity_id: usize) {
    if !eeveelution_identity_active_in_ctx(ctx, entity_id) {
        let states = EEVEELUTIONS.get_or_init(|| Mutex::new(Vec::new()));
        states
            .lock()
            .expect("eeveelution state poisoned")
            .retain(|state| state.entity_id != entity_id);
        return;
    }

    let tick = ctx.tick();
    let states = EEVEELUTIONS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut states = states.lock().expect("eeveelution state poisoned");
        if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
            if tick.saturating_sub(existing.last_tick) < EEVEELUTION_INTERVAL_TICKS {
                return;
            }
            existing.last_tick = tick;
        } else {
            states.push(EeveelutionState {
                entity_id,
                last_tick: tick,
            });
        }

        states.retain(|state| eeveelution_identity_active_in_ctx(ctx, state.entity_id));
    }

    let count = (0..ctx.entity_count())
        .filter(|other_id| {
            *other_id != entity_id && eeveelution_identity_active_in_ctx(ctx, *other_id)
        })
        .count();
    if count == 0 {
        return;
    }

    let bonus = (count.saturating_mul(2)).min(20) as i32;
    let speed_bonus = (count as i32).min(10);
    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time { tick: 45 },
            attack_mult: bonus,
            magic_power_mult: bonus,
            defence_mult: bonus,
            magic_resistance_mult: bonus,
            hp_mult: bonus,
            move_speed_mult: speed_bonus,
            attack_speed_mult: bonus,
            skill_cooldown_mult: bonus / 2,
            ult_cooldown_mult: bonus / 2,
            ..Default::default()
        },
    );
}

fn eeveelution_identity_active_in_ctx(ctx: &GameCtx, entity_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return false;
    }
    drop(entity);

    let champion_id = champion_id_for_entity_in_ctx(ctx, entity_id);
    if champion_id.map(is_eeveelution_champion_id).unwrap_or(false) {
        return true;
    }

    champion_id == Some("pokemon_moba_passimian")
        && receiver_copied_champion_id(entity_id)
            .map(is_eeveelution_champion_id)
            .unwrap_or(false)
}

fn is_eeveelution_champion_id(champion_id: &str) -> bool {
    matches!(
        champion_id,
        "pokemon_moba_jolteon"
            | "pokemon_moba_flareon"
            | "pokemon_moba_vaporeon"
            | "pokemon_moba_leafeon"
            | "pokemon_moba_glaceon"
            | "pokemon_moba_umbreon"
            | "pokemon_moba_espeon"
            | "pokemon_moba_sylveon"
    )
}

pub fn note_blaze_contact(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
) -> bool {
    let Some(target) = ctx.get_entity(target_id) else {
        return false;
    };
    if !target.is_champion() || !target.is_alive() {
        return false;
    }

    let tick = ctx.tick();
    let contacts = BLAZE_CONTACTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut contacts = contacts.lock().expect("blaze contact state poisoned");

    let contact = contacts
        .iter_mut()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id);

    let should_burn = if let Some(contact) = contact {
        if tick.saturating_sub(contact.last_contact_tick) <= BLAZE_CHAIN_WINDOW_TICKS {
            contact.accumulated_ticks += BURN_TICK_INTERVAL;
        } else {
            contact.accumulated_ticks = BURN_TICK_INTERVAL;
        }
        contact.last_contact_tick = tick;
        contact.accumulated_ticks >= BLAZE_REQUIRED_CONTACT_TICKS
    } else {
        contacts.push(BlazeContactState {
            caster_id,
            target_id,
            accumulated_ticks: BURN_TICK_INTERVAL,
            last_contact_tick: tick,
        });
        false
    };

    if should_burn {
        if let Some(contact) = contacts
            .iter_mut()
            .find(|state| state.caster_id == caster_id && state.target_id == target_id)
        {
            contact.accumulated_ticks = 0;
        }
        drop(contacts);
        apply_burn(ctx, caster_id, target_id, damage_per_tick);
        return true;
    }

    false
}

fn apply_burn(ctx: &GameCtx, caster_id: usize, target_id: usize, damage_per_tick: usize) {
    apply_burn_for(
        ctx,
        caster_id,
        target_id,
        damage_per_tick,
        BURN_DURATION_TICKS,
    );
}

pub fn apply_burn_for(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
) {
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }

    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }

    if has_dot_immunity(ctx, target_id) {
        return;
    }

    if !has_dot_absorb(ctx, target_id)
        && is_status_immune(
            target_id,
            &[PokemonType::Fire, PokemonType::Ground, PokemonType::Water],
        )
    {
        return;
    }

    apply_burn_for_unchecked(ctx, caster_id, target_id, damage_per_tick, duration_ticks);
}

pub fn apply_burn_for_unchecked(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    drop(target);

    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }

    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }

    if has_dot_immunity(ctx, target_id) {
        return;
    }

    let tick = ctx.tick();
    let burns = BURNS.get_or_init(|| Mutex::new(Vec::new()));
    burns.lock().expect("burn state poisoned").push(BurnState {
        caster_id,
        target_id,
        expires_at: tick + duration_ticks,
        next_tick_at: tick + BURN_TICK_INTERVAL,
        damage_per_tick,
    });
    synchronize_burn(ctx, caster_id, target_id, damage_per_tick, duration_ticks);
}

pub fn is_burned(ctx: &GameCtx, target_id: usize) -> bool {
    let tick = ctx.tick();
    BURNS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("burn state poisoned")
        .iter()
        .any(|state| state.target_id == target_id && state.expires_at > tick)
}

fn synchronize_target(ctx: &GameCtx, caster_id: usize, target_id: usize) -> Option<usize> {
    if caster_id == target_id || !is_beeheeyem(ctx, target_id) {
        return None;
    }
    let source = ctx.get_entity(caster_id)?;
    if source.is_alive() && source.is_champion() {
        Some(caster_id)
    } else {
        None
    }
}

fn synchronize_burn(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
) {
    let Some(reflect_target) = synchronize_target(ctx, caster_id, target_id) else {
        return;
    };
    let tick = ctx.tick();
    BURNS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("burn state poisoned")
        .push(BurnState {
            caster_id: target_id,
            target_id: reflect_target,
            expires_at: tick + duration_ticks,
            next_tick_at: tick + BURN_TICK_INTERVAL,
            damage_per_tick,
        });
}

fn synchronize_poison(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    stacks: usize,
    damage_per_tick: usize,
) {
    let Some(reflect_target) = synchronize_target(ctx, caster_id, target_id) else {
        return;
    };
    let tick = ctx.tick();
    let mut poisons = POISONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("poison state poisoned");
    for _ in 0..stacks {
        poisons.push(PoisonState {
            caster_id: target_id,
            target_id: reflect_target,
            expires_at: tick + POISON_DURATION_TICKS,
            next_tick_at: tick + POISON_TICK_INTERVAL,
            damage_per_tick,
        });
    }
}

fn synchronize_poison_for(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
) {
    let Some(reflect_target) = synchronize_target(ctx, caster_id, target_id) else {
        return;
    };
    let tick = ctx.tick();
    POISONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("poison state poisoned")
        .push(PoisonState {
            caster_id: target_id,
            target_id: reflect_target,
            expires_at: tick + duration_ticks,
            next_tick_at: tick + POISON_TICK_INTERVAL,
            damage_per_tick,
        });
}

fn synchronize_paralysis(ctx: &GameCtx, caster_id: usize, target_id: usize, duration_ticks: usize) {
    let Some(reflect_target) = synchronize_target(ctx, caster_id, target_id) else {
        return;
    };
    let tick = ctx.tick();
    let mut states = PARALYSIS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("paralysis state poisoned");
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.entity_id == reflect_target)
    {
        existing.caster_id = target_id;
        existing.expires_at = existing.expires_at.max(tick + duration_ticks);
        return;
    }
    states.push(ParalysisState {
        caster_id: target_id,
        entity_id: reflect_target,
        expires_at: tick + duration_ticks,
        next_roll_at: tick + PARALYSIS_ROLL_INTERVAL_TICKS,
    });
}

fn synchronize_confusion(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    stacks: usize,
    ticks: usize,
) {
    let Some(reflect_target) = synchronize_target(ctx, caster_id, target_id) else {
        return;
    };
    let tick = ctx.tick();
    let expires_at = tick.saturating_add(ticks);
    let mut states = CONFUSIONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("confusion state poisoned");
    states.retain(|state| state.expires_at > tick && state.stacks > 0);
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.entity_id == reflect_target)
    {
        existing.stacks = existing.stacks.saturating_add(stacks).min(9);
        existing.expires_at = existing.expires_at.max(expires_at);
        return;
    }
    states.push(ConfusionState {
        entity_id: reflect_target,
        stacks,
        expires_at,
    });
}

fn synchronize_frozen(ctx: &mut GameCtx, caster_id: usize, target_id: usize, ticks: usize) {
    let Some(reflect_target) = synchronize_target(ctx, caster_id, target_id) else {
        return;
    };
    let tick = ctx.tick();
    let expires_at = tick.saturating_add(ticks);
    let mut states = FROZENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("frozen state poisoned");
    states.retain(|state| state.expires_at > tick);
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.entity_id == reflect_target)
    {
        existing.expires_at = existing.expires_at.max(expires_at);
    } else {
        states.push(FrozenState {
            entity_id: reflect_target,
            expires_at,
        });
    }
    drop(states);
    break_kommoo_duel_on_hard_cc(ctx, target_id, reflect_target);
    apply_pokemon_cc(
        ctx,
        target_id,
        reflect_target,
        CCState::Stun { tick: ticks as u64 },
    );
    apply_pokemon_cc(
        ctx,
        target_id,
        reflect_target,
        CCState::BlockSkill { tick: ticks },
    );
    note_steadfast_cc(ctx, reflect_target);
}

pub fn register_will_o_wisp_charges(entity_id: usize) {
    let states = WILL_O_WISP_CHARGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("will-o-wisp charge state poisoned");
    if states.iter().any(|state| state.entity_id == entity_id) {
        return;
    }
    states.push(WillOWispChargeState {
        entity_id,
        charges: 3,
        next_charge_at: 0,
    });
}

pub fn register_porygon_type(entity_id: usize) {
    let states = PORYGON_TYPES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("porygon type state poisoned");
    if states.iter().any(|state| state.entity_id == entity_id) {
        return;
    }
    states.push(PorygonTypeState {
        entity_id,
        current_type: PokemonType::Normal,
        seen_mask: type_bit(PokemonType::Normal),
    });
}

pub fn porygon_type(ctx: &GameCtx, entity_id: usize) -> PokemonType {
    PORYGON_TYPES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("porygon type state poisoned")
        .iter()
        .find(|state| {
            state.entity_id == entity_id
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
        .map(|state| state.current_type)
        .unwrap_or(PokemonType::Normal)
}

pub fn porygon_seen_type_count(ctx: &GameCtx, entity_id: usize) -> usize {
    PORYGON_TYPES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("porygon type state poisoned")
        .iter()
        .find(|state| {
            state.entity_id == entity_id
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
        .map(|state| state.seen_mask.count_ones() as usize)
        .unwrap_or(1)
}

pub fn set_porygon_type(ctx: &GameCtx, entity_id: usize, new_type: PokemonType) {
    if ctx
        .get_entity(entity_id)
        .map(|entity| entity.is_alive())
        .unwrap_or(false)
    {
        let states = PORYGON_TYPES.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states.lock().expect("porygon type state poisoned");
        if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
            state.current_type = new_type;
            state.seen_mask |= type_bit(new_type);
        } else {
            states.push(PorygonTypeState {
                entity_id,
                current_type: new_type,
                seen_mask: type_bit(PokemonType::Normal) | type_bit(new_type),
            });
        }
        register_entity_types(entity_id, TypeSet::single(new_type));
    }
}

fn type_bit(pokemon_type: PokemonType) -> u32 {
    1 << match pokemon_type {
        PokemonType::Normal => 0,
        PokemonType::Fire => 1,
        PokemonType::Water => 2,
        PokemonType::Electric => 3,
        PokemonType::Grass => 4,
        PokemonType::Ice => 5,
        PokemonType::Fighting => 6,
        PokemonType::Poison => 7,
        PokemonType::Ground => 8,
        PokemonType::Flying => 9,
        PokemonType::Psychic => 10,
        PokemonType::Bug => 11,
        PokemonType::Rock => 12,
        PokemonType::Ghost => 13,
        PokemonType::Dragon => 14,
        PokemonType::Dark => 15,
        PokemonType::Steel => 16,
        PokemonType::Fairy => 17,
        PokemonType::Bird => 18,
    }
}

pub fn consume_will_o_wisp_charge(
    ctx: &GameCtx,
    entity_id: usize,
    max_charges: usize,
    recharge_ticks: usize,
) -> bool {
    refill_will_o_wisp_charge(ctx.tick(), entity_id, max_charges, recharge_ticks);
    let states = WILL_O_WISP_CHARGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("will-o-wisp charge state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return false;
    };
    if state.charges == 0 {
        return false;
    }
    state.charges -= 1;
    if state.charges < max_charges && state.next_charge_at <= ctx.tick() {
        state.next_charge_at = ctx.tick().saturating_add(recharge_ticks);
    }
    true
}

pub fn will_o_wisp_ready_for_player(
    player_id: usize,
    tick: usize,
    max_charges: usize,
    recharge_ticks: usize,
) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return true;
    };
    refill_will_o_wisp_charge(tick, entity_id, max_charges, recharge_ticks);
    WILL_O_WISP_CHARGES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("will-o-wisp charge state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.charges > 0)
        .unwrap_or(true)
}

pub fn register_sticky_web_charges(entity_id: usize) {
    let states = STICKY_WEB_CHARGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sticky web charge state poisoned");
    if states.iter().any(|state| state.entity_id == entity_id) {
        return;
    }
    states.push(StickyWebChargeState {
        entity_id,
        charges: 5,
        next_charge_at: 0,
    });
}

pub fn consume_sticky_web_charge(
    ctx: &GameCtx,
    entity_id: usize,
    max_charges: usize,
    recharge_ticks: usize,
) -> bool {
    refill_sticky_web_charge(ctx.tick(), entity_id, max_charges, recharge_ticks);
    let states = STICKY_WEB_CHARGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sticky web charge state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return false;
    };
    if state.charges == 0 {
        return false;
    }
    state.charges -= 1;
    if state.charges < max_charges && state.next_charge_at <= ctx.tick() {
        state.next_charge_at = ctx.tick().saturating_add(recharge_ticks);
    }
    true
}

pub fn sticky_web_ready_for_player(
    player_id: usize,
    tick: usize,
    max_charges: usize,
    recharge_ticks: usize,
) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return true;
    };
    refill_sticky_web_charge(tick, entity_id, max_charges, recharge_ticks);
    STICKY_WEB_CHARGES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("sticky web charge state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.charges > 0)
        .unwrap_or(true)
}

fn refill_sticky_web_charge(
    tick: usize,
    entity_id: usize,
    max_charges: usize,
    recharge_ticks: usize,
) {
    let states = STICKY_WEB_CHARGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sticky web charge state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return;
    };
    while state.charges < max_charges && state.next_charge_at <= tick {
        state.charges += 1;
        if state.charges < max_charges {
            state.next_charge_at = state.next_charge_at.saturating_add(recharge_ticks);
        }
    }
}

fn refill_will_o_wisp_charge(
    tick: usize,
    entity_id: usize,
    max_charges: usize,
    recharge_ticks: usize,
) {
    let states = WILL_O_WISP_CHARGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("will-o-wisp charge state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        return;
    };
    while state.charges < max_charges && state.next_charge_at <= tick {
        state.charges += 1;
        if state.charges < max_charges {
            state.next_charge_at = state.next_charge_at.saturating_add(recharge_ticks);
        }
    }
}

pub fn begin_flame_trail(
    ctx: &GameCtx,
    caster_id: usize,
    start: EntityPos,
    end: EntityPos,
    width: u64,
    duration_ticks: usize,
    damage_per_tick: usize,
    attacker_types: TypeSet,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    drop(caster);

    let tick = ctx.tick();
    FLAME_TRAILS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("flame trail state poisoned")
        .push(FlameTrailState {
            caster_id,
            caster_team,
            start,
            end,
            width,
            expires_at: tick.saturating_add(duration_ticks),
            next_tick_at: tick.saturating_add(FLAME_TRAIL_INTERVAL_TICKS),
            damage_per_tick,
            attacker_types,
            burn_chance_percent,
            burn_ticks,
            burn_damage,
        });
}

pub fn reset_speed_boost(ctx: &GameCtx, entity_id: usize) {
    let tick = ctx.tick();
    let states = SPEED_BOOSTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("speed boost state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.last_reset_tick = tick;
        existing.last_buff_tick = tick;
        return;
    }
    states.push(SpeedBoostState {
        entity_id,
        last_reset_tick: tick,
        last_buff_tick: tick,
    });
}

pub fn update_speed_boost_passive(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    drop(entity);

    let tick = ctx.tick();
    let states = SPEED_BOOSTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("speed boost state poisoned");
    states.retain(|state| {
        ctx.get_entity(state.entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false)
    });

    let state = if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id)
    {
        existing
    } else {
        states.push(SpeedBoostState {
            entity_id,
            last_reset_tick: tick,
            last_buff_tick: tick,
        });
        return;
    };

    if tick.saturating_sub(state.last_buff_tick) < SPEED_BOOST_INTERVAL_TICKS {
        return;
    }
    state.last_buff_tick = tick;

    let idle_seconds = tick.saturating_sub(state.last_reset_tick) / 60;
    let bonus = idle_seconds
        .saturating_mul(SPEED_BOOST_PERCENT_PER_SECOND)
        .min(SPEED_BOOST_MAX_PERCENT);
    drop(states);

    if bonus == 0 {
        return;
    }

    ctx.add_buff(
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: SPEED_BOOST_INTERVAL_TICKS,
            },
            move_speed_mult: bonus as i32,
            ..Default::default()
        },
    );
}

pub fn update_sawk_throh_forms(ctx: &mut GameCtx) {
    let tick = ctx.tick();
    let states = SAWK_THROH_FORMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut refreshes = Vec::new();
    {
        let mut states = states.lock().expect("sawk/throh stance state poisoned");
        states.retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

        for state in states.iter_mut() {
            if tick.saturating_sub(state.last_buff_tick) >= SAWK_THROH_INTERVAL_TICKS {
                state.last_buff_tick = tick;
                refreshes.push((state.entity_id, state.form));
            }
        }
    }

    for (entity_id, form) in refreshes {
        refresh_sawk_throh_buff(ctx, entity_id, form);
    }
}

fn refresh_sawk_throh_buff(ctx: &mut GameCtx, entity_id: usize, form: SawkThrohForm) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    match form {
        SawkThrohForm::Sawk => {
            add_beneficial_buff(
                ctx,
                entity_id,
                entity_id,
                BuffState {
                    duration: BuffType::Time { tick: 45 },
                    attack_mult: 20,
                    attack_speed_mult: 18,
                    move_speed_mult: 14,
                    defence_mult: -8,
                    magic_resistance_mult: -8,
                    ..Default::default()
                },
            );
            draw_status_marker(ctx, pos, 9000, VFX_FIGHTING);
        }
        SawkThrohForm::Throh => {
            ctx.add_buff(
                entity_id,
                BuffState {
                    duration: BuffType::Time { tick: 45 },
                    hp_mult: 18,
                    defence_mult: 36,
                    magic_resistance_mult: 24,
                    damaged_reduce: 8,
                    attack_mult: -12,
                    move_speed_mult: -8,
                    ..Default::default()
                },
            );
            draw_status_marker(ctx, pos, 9000, VFX_GROUND);
        }
    }
}

pub fn apply_poison_stacks(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    stacks: usize,
    damage_per_tick: usize,
) {
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }

    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }

    if has_dot_immunity(ctx, target_id) {
        return;
    }

    if !has_dot_absorb(ctx, target_id)
        && is_status_immune(
            target_id,
            &[PokemonType::Poison, PokemonType::Grass, PokemonType::Steel],
        )
    {
        return;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }

    let tick = ctx.tick();
    let poisons = POISONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut poisons = poisons.lock().expect("poison state poisoned");
    for _ in 0..stacks {
        poisons.push(PoisonState {
            caster_id,
            target_id,
            expires_at: tick + POISON_DURATION_TICKS,
            next_tick_at: tick + POISON_TICK_INTERVAL,
            damage_per_tick,
        });
    }
    drop(poisons);
    synchronize_poison(ctx, caster_id, target_id, stacks, damage_per_tick);
}

pub fn apply_poison_for(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
) {
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }

    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }

    if has_dot_immunity(ctx, target_id) {
        return;
    }

    if !has_dot_absorb(ctx, target_id)
        && is_status_immune(
            target_id,
            &[PokemonType::Poison, PokemonType::Grass, PokemonType::Steel],
        )
    {
        return;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }

    let tick = ctx.tick();
    POISONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("poison state poisoned")
        .push(PoisonState {
            caster_id,
            target_id,
            expires_at: tick + duration_ticks,
            next_tick_at: tick + POISON_TICK_INTERVAL,
            damage_per_tick,
        });
    synchronize_poison_for(ctx, caster_id, target_id, damage_per_tick, duration_ticks);
}

pub fn poison_stack_count(ctx: &GameCtx, target_id: usize) -> usize {
    let tick = ctx.tick();
    POISONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("poison state poisoned")
        .iter()
        .filter(|state| state.target_id == target_id && state.expires_at > tick)
        .count()
}

pub fn is_poisoned(ctx: &GameCtx, target_id: usize) -> bool {
    poison_stack_count(ctx, target_id) > 0
}

pub fn note_arbok_basic_hit(ctx: &GameCtx, entity_id: usize, window_ticks: usize) -> usize {
    let tick = ctx.tick();
    let states = ARBOK_BASIC_HITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("arbok basic hit state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(ArbokBasicHitState {
            entity_id,
            hits: 1,
            last_hit_tick: tick,
        });
        return 1;
    };
    if tick.saturating_sub(state.last_hit_tick) > window_ticks {
        state.hits = 0;
    }
    state.hits = state.hits.saturating_add(1);
    state.last_hit_tick = tick;
    state.hits
}

pub fn add_miasma_stacks(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    stacks: usize,
    poison_damage_per_tick: usize,
) {
    if stacks == 0 {
        return;
    }
    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    drop(target);

    let tick = ctx.tick();
    let poison_awards_to_apply;
    {
        let states = MIASMAS.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states.lock().expect("miasma state poisoned");
        if let Some(state) = states
            .iter_mut()
            .find(|state| state.caster_id == caster_id && state.target_id == target_id)
        {
            state.stacks = state.stacks.saturating_add(stacks);
            state.poison_damage_per_tick = state.poison_damage_per_tick.max(poison_damage_per_tick);
            state.expires_at = tick.saturating_add(MIASMA_DURATION_TICKS);
            let new_awards = state.stacks / 10;
            poison_awards_to_apply = new_awards.saturating_sub(state.poison_awards);
            state.poison_awards = new_awards;
        } else {
            let poison_awards = stacks / 10;
            poison_awards_to_apply = poison_awards;
            states.push(MiasmaState {
                caster_id,
                target_id,
                stacks,
                poison_awards,
                poison_damage_per_tick,
                expires_at: tick.saturating_add(MIASMA_DURATION_TICKS),
            });
        }
    }

    if poison_awards_to_apply > 0 {
        apply_poison_stacks(
            ctx,
            caster_id,
            target_id,
            poison_awards_to_apply,
            poison_damage_per_tick.max(1),
        );
    }
}

pub fn miasma_stacks(caster_id: usize, target_id: usize) -> usize {
    MIASMAS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("miasma state poisoned")
        .iter()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id)
        .map(|state| state.stacks)
        .unwrap_or(0)
}

pub fn consume_miasma_stacks(caster_id: usize, target_id: usize) -> usize {
    let states = MIASMAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("miasma state poisoned");
    let Some(index) = states
        .iter()
        .position(|state| state.caster_id == caster_id && state.target_id == target_id)
    else {
        return 0;
    };
    states.swap_remove(index).stacks
}

pub fn apply_bleed_for(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
) {
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }

    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }

    if is_yanmega_blood(ctx, target_id) {
        return;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }

    let tick = ctx.tick();
    let bleeds = BLEEDS.get_or_init(|| Mutex::new(Vec::new()));
    bleeds
        .lock()
        .expect("bleed state poisoned")
        .push(BleedState {
            caster_id,
            target_id,
            expires_at: tick + duration_ticks,
            next_tick_at: tick + BLEED_TICK_INTERVAL,
            damage_per_tick,
        });
}

pub fn apply_infestation_for(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    duration_ticks: usize,
    attacker_types: TypeSet,
) {
    if damage_per_tick == 0 || duration_ticks == 0 {
        return;
    }
    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    drop(target);

    let tick = ctx.tick();
    let expires_at = tick.saturating_add(duration_ticks);
    let infestations = INFESTATIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut infestations = infestations.lock().expect("infestation state poisoned");
    infestations.retain(|state| state.expires_at > tick);
    if let Some(existing) = infestations
        .iter_mut()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id)
    {
        existing.expires_at = existing.expires_at.max(expires_at);
        existing.damage_per_tick = existing.damage_per_tick.max(damage_per_tick);
        existing.attacker_types = attacker_types;
        return;
    }
    infestations.push(InfestationState {
        caster_id,
        target_id,
        expires_at,
        next_tick_at: tick.saturating_add(INFESTATION_TICK_INTERVAL),
        damage_per_tick,
        attacker_types,
    });
}

pub fn is_infested(ctx: &GameCtx, target_id: usize) -> bool {
    let tick = ctx.tick();
    INFESTATIONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("infestation state poisoned")
        .iter()
        .any(|state| state.target_id == target_id && state.expires_at > tick)
}

pub fn apply_infestation_stacks_for(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_per_tick: usize,
    stacks: usize,
    duration_ticks: usize,
    attacker_types: TypeSet,
) {
    apply_infestation_for(
        ctx,
        caster_id,
        target_id,
        damage_per_tick.saturating_mul(stacks.max(1)).max(1),
        duration_ticks,
        attacker_types,
    );
}

pub fn begin_yanmega_tinted_lens(ctx: &mut GameCtx, entity_id: usize, ticks: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let expires_at = ctx.tick().saturating_add(ticks);
    let states = YANMEGA_TINTED_LENSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("yanmega tinted lens state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.expires_at = existing.expires_at.max(expires_at);
    } else {
        states.push(YanmegaTintedLensState {
            entity_id,
            expires_at,
        });
    }
    draw_status_marker(ctx, pos, 10000, VFX_BUG);
}

pub fn yanmega_tinted_lens_active(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    YANMEGA_TINTED_LENSES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("yanmega tinted lens state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.expires_at > tick)
}

fn update_yanmega_tinted_lenses(ctx: &GameCtx, tick: usize) {
    YANMEGA_TINTED_LENSES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("yanmega tinted lens state poisoned")
        .retain(|state| {
            state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });
}

fn is_yanmega_blood(ctx: &GameCtx, entity_id: usize) -> bool {
    champion_id_for_entity(entity_id) == Some("pokemon_moba_yanmega")
        || receiver_has_copied(entity_id, "pokemon_moba_yanmega")
        || champion_id_for_entity_in_ctx(ctx, entity_id) == Some("pokemon_moba_yanmega")
}

pub fn convert_lifesteal_against_yanmega(
    ctx: &GameCtx,
    lifestealer_id: usize,
    target_id: usize,
    heal_amount: usize,
) -> bool {
    if heal_amount == 0 || !is_yanmega_blood(ctx, target_id) {
        return false;
    }
    let poison_damage = heal_amount.saturating_div(6).max(8);
    apply_poison_for(ctx, target_id, lifestealer_id, poison_damage, 5 * 60);
    true
}

pub fn heal_from_damage_or_poison_yanmega(
    ctx: &mut GameCtx,
    healer_id: usize,
    damaged_target_id: usize,
    heal_amount: usize,
) -> usize {
    if heal_amount == 0 {
        return 0;
    }
    if convert_lifesteal_against_yanmega(ctx, healer_id, damaged_target_id, heal_amount) {
        return 0;
    }
    heal_with_antiheal(ctx, healer_id, healer_id, heal_amount)
}

pub fn begin_yanmega_giga_drain(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    duration_ticks: usize,
    tick_interval: usize,
    damage_per_tick: usize,
    poison_health_unit: usize,
    poison_damage_per_tick: usize,
    poison_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_hp = caster.hp().current;
    let caster_pos = caster.pos();
    drop(caster);
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    let target_pos = target.pos();
    drop(target);

    let tick = ctx.tick();
    let states = YANMEGA_GIGA_DRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("yanmega giga drain state poisoned");
    states.retain(|state| state.caster_id != caster_id);
    states.push(YanmegaGigaDrainState {
        caster_id,
        target_id,
        started_at: tick,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        tick_interval: tick_interval.max(1),
        damage_per_tick: damage_per_tick.max(1),
        poison_health_unit: poison_health_unit.max(1),
        poison_damage_per_tick: poison_damage_per_tick.max(1),
        poison_ticks,
        total_drained: 0,
        last_caster_hp: caster_hp,
        attacker_types,
    });
    drop(states);

    apply_pokemon_cc(
        ctx,
        caster_id,
        caster_id,
        CCState::Bind {
            tick: tick_interval as u64,
        },
    );
    apply_pokemon_cc(
        ctx,
        caster_id,
        caster_id,
        CCState::BlockSkill {
            tick: tick_interval,
        },
    );
    apply_pokemon_cc(
        ctx,
        caster_id,
        target_id,
        CCState::Bind {
            tick: tick_interval as u64,
        },
    );
    apply_pokemon_cc(
        ctx,
        caster_id,
        target_id,
        CCState::BlockSkill {
            tick: tick_interval,
        },
    );
    draw_line_band(ctx, caster_pos, target_pos, 5500, VFX_GRASS);
}

fn finalize_yanmega_giga_drain(ctx: &GameCtx, state: YanmegaGigaDrainState) {
    let stacks = state.total_drained / state.poison_health_unit;
    if stacks == 0 {
        return;
    }
    for _ in 0..stacks {
        apply_poison_for(
            ctx,
            state.caster_id,
            state.target_id,
            state.poison_damage_per_tick,
            state.poison_ticks,
        );
    }
}

fn update_yanmega_giga_drains(ctx: &mut GameCtx, tick: usize) {
    let states = YANMEGA_GIGA_DRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("yanmega giga drain state poisoned");
    let mut ticks_to_apply = Vec::new();
    let mut finished = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            finished.push(*state);
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            finished.push(*state);
            return false;
        }
        let caster_hp = caster.hp();
        if caster_hp.current < state.last_caster_hp
            || caster_hp.current >= caster_hp.max
            || had_external_disruptive_cc_since(ctx, state.caster_id, state.started_at)
        {
            finished.push(*state);
            return false;
        }
        drop(caster);

        let Some(target) = ctx.get_entity(state.target_id) else {
            finished.push(*state);
            return false;
        };
        if !target.is_alive() {
            finished.push(*state);
            return false;
        }
        drop(target);

        while state.next_tick_at <= tick {
            ticks_to_apply.push(*state);
            state.next_tick_at = state
                .next_tick_at
                .saturating_add(state.tick_interval.max(1));
        }
        true
    });
    drop(states);

    for mut state in ticks_to_apply {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            finished.push(state);
            continue;
        };
        let caster_pos = caster.pos();
        drop(caster);
        let Some(target) = ctx.get_entity(state.target_id) else {
            finished.push(state);
            continue;
        };
        let target_pos = target.pos();
        drop(target);

        let mut damage = state.damage_per_tick;
        if is_infested(ctx, state.target_id) || is_poisoned(ctx, state.target_id) {
            damage = damage.saturating_mul(150) / 100;
        }
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, state.target_id);
        let result = crate::pokemon_types::deal_pokemon_damage(
            ctx,
            state.caster_id,
            state.target_id,
            0,
            damage.max(1),
            AttackType::Dot,
            PokemonType::Grass,
            state.attacker_types,
            defender_types,
        );
        let heal = result.applied_damage.max(1);
        heal_from_damage_or_poison_yanmega(ctx, state.caster_id, state.target_id, heal);
        draw_line_band(ctx, caster_pos, target_pos, 5500, VFX_GRASS);

        if let Some(caster) = ctx.get_entity(state.caster_id) {
            state.last_caster_hp = caster.hp().current;
        }
        state.total_drained = state.total_drained.saturating_add(heal);
        apply_pokemon_cc(
            ctx,
            state.caster_id,
            state.caster_id,
            CCState::Bind {
                tick: state.tick_interval as u64 + 4,
            },
        );
        apply_pokemon_cc(
            ctx,
            state.caster_id,
            state.caster_id,
            CCState::BlockSkill {
                tick: state.tick_interval + 4,
            },
        );
        apply_pokemon_cc(
            ctx,
            state.caster_id,
            state.target_id,
            CCState::Bind {
                tick: state.tick_interval as u64 + 4,
            },
        );
        apply_pokemon_cc(
            ctx,
            state.caster_id,
            state.target_id,
            CCState::BlockSkill {
                tick: state.tick_interval + 4,
            },
        );

        let states = YANMEGA_GIGA_DRAINS.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states.lock().expect("yanmega giga drain state poisoned");
        if let Some(existing) = states.iter_mut().find(|existing| {
            existing.caster_id == state.caster_id && existing.target_id == state.target_id
        }) {
            existing.total_drained = state.total_drained;
            existing.last_caster_hp = state.last_caster_hp;
        }
    }

    for state in finished {
        finalize_yanmega_giga_drain(ctx, state);
    }
}

pub fn apply_frozen_for(ctx: &mut GameCtx, target_id: usize, ticks: usize) {
    apply_frozen_from(ctx, target_id, target_id, ticks);
}

pub fn apply_frozen_from(ctx: &mut GameCtx, caster_id: usize, target_id: usize, ticks: usize) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    drop(target);

    if blocks_harmful_status(ctx, caster_id, target_id) {
        return;
    }
    if is_status_immune(target_id, &[PokemonType::Ice]) || is_limber(ctx, target_id) {
        return;
    }

    let tick = ctx.tick();
    let expires_at = tick.saturating_add(ticks);
    let states = FROZENS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("frozen state poisoned");
    states.retain(|state| state.expires_at > tick);
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == target_id) {
        existing.expires_at = existing.expires_at.max(expires_at);
    } else {
        states.push(FrozenState {
            entity_id: target_id,
            expires_at,
        });
    }
    drop(states);

    if let Some(target_pos) = ctx.get_entity(target_id).map(|target| target.pos()) {
        draw_status_marker(ctx, target_pos, 12000, VFX_ICE);
    }

    break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
    apply_pokemon_cc(
        ctx,
        caster_id,
        target_id,
        CCState::Stun { tick: ticks as u64 },
    );
    apply_pokemon_cc(
        ctx,
        caster_id,
        target_id,
        CCState::BlockSkill { tick: ticks },
    );
    note_steadfast_cc(ctx, target_id);
    synchronize_frozen(ctx, caster_id, target_id, ticks);
}

pub fn is_frozen(ctx: &GameCtx, entity_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    if !entity.is_alive() {
        return false;
    }
    let tick = ctx.tick();
    FROZENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("frozen state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.expires_at > tick)
}

pub fn begin_ice_field(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    radius: u64,
    duration_ticks: usize,
    damage_per_tick: usize,
    freeze_chance_percent: usize,
    freeze_ticks: usize,
    slow_percent: i32,
    control_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };

    let tick = ctx.tick();
    let fields = ICE_FIELDS.get_or_init(|| Mutex::new(Vec::new()));
    fields
        .lock()
        .expect("ice field state poisoned")
        .push(IceFieldState {
            caster_id,
            caster_team: caster.team(),
            center,
            radius,
            expires_at: tick + duration_ticks,
            next_tick_at: tick,
            damage_per_tick,
            freeze_chance_percent,
            freeze_ticks,
            slow_percent,
            control_ticks,
        });
}

pub fn apply_leech_seed(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    duration_ticks: usize,
    damage_per_tick: usize,
    break_range: u64,
) {
    if audino_protect_blocks_new_damage_status(ctx, target_id) {
        return;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_champion() || !target.is_alive() {
        return;
    }

    let tick = ctx.tick();
    push_leech_seed(
        caster_id,
        target_id,
        tick + duration_ticks,
        tick + LEECH_TICK_INTERVAL,
        damage_per_tick,
        break_range,
    );
}

pub fn begin_wish_channel(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    max_ticks: usize,
    heal_amount: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !caster.is_alive()
        || !target.is_alive()
        || caster.team() != target.team()
        || !target.is_champion()
    {
        return;
    }

    let channels = WISH_CHANNELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut channels = channels.lock().expect("wish channel state poisoned");
    channels.retain(|state| state.caster_id != caster_id);
    channels.push(WishChannelState {
        caster_id,
        target_id,
        started_at: ctx.tick(),
        max_ticks,
        heal_amount,
        last_pos: caster.pos(),
    });
}

pub fn wish_channel_percent_for_player(player_id: usize, tick: usize) -> Option<usize> {
    let caster_id = entity_for_player(player_id)?;
    let channels = WISH_CHANNELS.get_or_init(|| Mutex::new(Vec::new()));
    channels
        .lock()
        .expect("wish channel state poisoned")
        .iter()
        .find(|state| state.caster_id == caster_id)
        .map(|state| {
            tick.saturating_sub(state.started_at)
                .saturating_mul(100)
                .saturating_div(state.max_ticks.max(1))
                .min(100)
        })
}

pub fn should_release_wish_for_player(
    player_id: usize,
    tick: usize,
    hp_ratio_percent: usize,
) -> bool {
    let threshold = if hp_ratio_percent <= 35 { 45 } else { 75 };
    wish_channel_percent_for_player(player_id, tick)
        .map(|percent| percent >= threshold)
        .unwrap_or(false)
}

pub fn begin_power_up_punch_channel(
    ctx: &GameCtx,
    caster_id: usize,
    target_pos: EntityPos,
    max_ticks: usize,
    ad_damage: usize,
    width: u64,
    full_cooldown_ticks: usize,
    attacker_types: TypeSet,
) -> bool {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return false;
    };
    if !caster.is_alive() {
        return false;
    }

    let tick = ctx.tick();
    let player_id = player_for_entity(caster_id).unwrap_or(caster_id);
    if !power_up_punch_ready_for_player(player_id, tick) {
        return false;
    }

    let cooldowns = POWER_UP_PUNCH_COOLDOWNS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut cooldowns = cooldowns
            .lock()
            .expect("power-up punch cooldown state poisoned");
        if let Some(existing) = cooldowns
            .iter_mut()
            .find(|state| state.player_id == player_id)
        {
            existing.ready_at = tick.saturating_add(full_cooldown_ticks);
        } else {
            cooldowns.push(PowerUpPunchCooldownState {
                player_id,
                ready_at: tick.saturating_add(full_cooldown_ticks),
            });
        }
    }

    let channels = POWER_UP_PUNCH_CHANNELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut channels = channels
        .lock()
        .expect("power-up punch channel state poisoned");
    channels.retain(|state| state.caster_id != caster_id);
    channels.push(PowerUpPunchChannelState {
        player_id,
        caster_id,
        caster_team: caster.team(),
        started_at: tick,
        max_ticks,
        target_pos,
        last_pos: caster.pos(),
        ad_damage,
        width,
        full_cooldown_ticks,
        attacker_types,
    });
    true
}

pub fn power_up_punch_ready_for_player(player_id: usize, tick: usize) -> bool {
    POWER_UP_PUNCH_COOLDOWNS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("power-up punch cooldown state poisoned")
        .iter()
        .find(|state| state.player_id == player_id)
        .map(|state| tick >= state.ready_at)
        .unwrap_or(true)
}

pub fn power_up_punch_channel_percent_for_player(player_id: usize, tick: usize) -> Option<usize> {
    POWER_UP_PUNCH_CHANNELS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("power-up punch channel state poisoned")
        .iter()
        .find(|state| state.player_id == player_id)
        .map(|state| {
            tick.saturating_sub(state.started_at)
                .saturating_mul(100)
                .saturating_div(state.max_ticks.max(1))
                .min(100)
        })
}

pub fn should_release_power_up_punch_for_player(
    player_id: usize,
    tick: usize,
    hp_ratio_percent: usize,
) -> bool {
    let threshold = if hp_ratio_percent <= 40 { 55 } else { 80 };
    power_up_punch_channel_percent_for_player(player_id, tick)
        .map(|percent| percent >= threshold)
        .unwrap_or(false)
}

pub fn schedule_quick_return_dash(
    ctx: &GameCtx,
    entity_id: usize,
    origin: EntityPos,
    delay_ticks: usize,
    speed: u64,
    ticks: u64,
) {
    if ctx
        .get_entity(entity_id)
        .map(|entity| entity.is_alive())
        .unwrap_or(false)
    {
        let states = QUICK_RETURN_DASHES.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states.lock().expect("quick return dash state poisoned");
        states.retain(|state| state.entity_id != entity_id);
        states.push(QuickReturnDashState {
            entity_id,
            origin,
            trigger_at: ctx.tick().saturating_add(delay_ticks),
            speed,
            ticks,
        });
    }
}

pub fn begin_orbeetle_agility_chain(
    ctx: &GameCtx,
    caster_id: usize,
    targets: Vec<usize>,
    damage: usize,
    attacker_types: TypeSet,
    force_move_speed: u64,
    force_move_ticks: u64,
) {
    if targets.is_empty() {
        return;
    }
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    drop(caster);

    let states = ORBEETLE_AGILITY_CHAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("orbeetle agility state poisoned");
    states.retain(|state| state.caster_id != caster_id);
    states.push(OrbeetleAgilityChainState {
        caster_id,
        targets,
        next_index: 0,
        next_hop_at: ctx.tick(),
        expires_at: ctx.tick().saturating_add(90),
        damage: damage.max(1),
        attacker_types,
        force_move_speed,
        force_move_ticks,
        hop_interval_ticks: (force_move_ticks as usize).saturating_add(2).max(4),
    });
}

pub fn add_battle_bond_stack(ctx: &GameCtx, entity_id: usize) {
    if !ctx
        .get_entity(entity_id)
        .map(|entity| entity.is_alive())
        .unwrap_or(false)
    {
        return;
    }

    let states = BATTLE_BONDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("battle bond state poisoned");
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        existing.stacks = existing.stacks.saturating_add(1).min(12);
    } else {
        states.push(BattleBondState {
            entity_id,
            stacks: 1,
        });
    }
}

pub fn battle_bond_stacks(ctx: &GameCtx, entity_id: usize) -> usize {
    BATTLE_BONDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("battle bond state poisoned")
        .iter()
        .find(|state| {
            state.entity_id == entity_id
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
        .map(|state| state.stacks)
        .unwrap_or(0)
}

pub fn add_long_reach_stack(ctx: &mut GameCtx, entity_id: usize, range_bonus: usize, ticks: usize) {
    if ctx
        .get_entity(entity_id)
        .map(|entity| entity.is_alive())
        .unwrap_or(false)
    {
        ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                range: range_bonus,
                ..Default::default()
            },
        );
    }
}

pub fn apply_spirit_shackle(ctx: &GameCtx, target_id: usize, duration_ticks: usize) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || !target.is_champion() {
        return;
    }

    let states = SPIRIT_SHACKLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("spirit shackle state poisoned");
    let expires_at = ctx.tick().saturating_add(duration_ticks);
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == target_id) {
        existing.expires_at = existing.expires_at.max(expires_at);
    } else {
        states.push(SpiritShackleState {
            entity_id: target_id,
            expires_at,
        });
    }
}

#[allow(dead_code)]
pub fn is_spirit_shackled(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    SPIRIT_SHACKLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("spirit shackle state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
}

pub fn is_player_spirit_shackled(player_id: usize, tick: usize) -> bool {
    let Some(entity_id) = entity_for_player(player_id) else {
        return false;
    };
    SPIRIT_SHACKLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("spirit shackle state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.expires_at > tick)
}

pub fn apply_soak(ctx: &GameCtx, target_id: usize, duration_ticks: usize) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }

    let states = SOAKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("soak state poisoned");
    let expires_at = ctx.tick().saturating_add(duration_ticks);
    if let Some(existing) = states.iter_mut().find(|state| state.entity_id == target_id) {
        existing.expires_at = existing.expires_at.max(expires_at);
    } else {
        states.push(SoakState {
            entity_id: target_id,
            expires_at,
        });
    }
}

pub fn is_soaked(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    SOAKS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("soak state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
}

fn set_power_up_punch_ready_at(player_id: usize, ready_at: usize) {
    let cooldowns = POWER_UP_PUNCH_COOLDOWNS.get_or_init(|| Mutex::new(Vec::new()));
    let mut cooldowns = cooldowns
        .lock()
        .expect("power-up punch cooldown state poisoned");
    if let Some(existing) = cooldowns
        .iter_mut()
        .find(|state| state.player_id == player_id)
    {
        existing.ready_at = ready_at;
    } else {
        cooldowns.push(PowerUpPunchCooldownState {
            player_id,
            ready_at,
        });
    }
}

pub fn begin_aqua_ring(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    heal_per_tick: usize,
    enemy_attack_mult: i32,
    enemy_debuff_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    let rings = AQUA_RINGS.get_or_init(|| Mutex::new(Vec::new()));
    let mut rings = rings.lock().expect("aqua ring state poisoned");
    rings.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    rings.push(AquaRingState {
        caster_id,
        caster_team: caster.team(),
        expires_at: tick + duration_ticks,
        next_tick_at: tick,
        radius,
        heal_per_tick,
        enemy_attack_mult,
        enemy_debuff_ticks,
    });
}

pub fn begin_misty_terrain(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    duration_ticks: usize,
    radius: u64,
    heal_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }

    let terrains = MISTY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut terrains = terrains.lock().expect("misty terrain state poisoned");
    terrains.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    terrains.push(MistyTerrainState {
        caster_id,
        caster_team: caster.team(),
        center,
        radius,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        heal_per_tick,
        slow_percent,
        slow_ticks,
    });
}

pub fn begin_brine_field(
    ctx: &GameCtx,
    center: EntityPos,
    duration_ticks: usize,
    radius: u64,
    slow_percent: i32,
    slow_ticks: usize,
) {
    let fields = BRINE_FIELDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("brine field state poisoned");
    let tick = ctx.tick();
    fields.push(BrineFieldState {
        center,
        radius,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        slow_percent,
        slow_ticks,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn maybe_spawn_grassy_terrain(
    ctx: &GameCtx,
    caster_id: usize,
    max_fields: usize,
    radius: u64,
    cooldown_ticks: usize,
    damage_bonus_percent: usize,
    attack_speed_mult: i32,
    buff_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let center = caster.pos();
    drop(caster);

    let tick = ctx.tick();
    spawn_grassy_terrain_at_tick(
        ctx,
        caster_id,
        center,
        max_fields,
        radius,
        cooldown_ticks,
        tick,
        usize::MAX,
        damage_bonus_percent,
        attack_speed_mult,
        buff_ticks,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_grassy_terrain_at(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    max_fields: usize,
    radius: u64,
    duration_ticks: usize,
    damage_bonus_percent: usize,
    attack_speed_mult: i32,
    buff_ticks: usize,
) {
    let tick = ctx.tick();
    spawn_grassy_terrain_at_tick(
        ctx,
        caster_id,
        center,
        max_fields,
        radius,
        0,
        tick,
        tick.saturating_add(duration_ticks),
        damage_bonus_percent,
        attack_speed_mult,
        buff_ticks,
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_grassy_terrain_at_tick(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    max_fields: usize,
    radius: u64,
    cooldown_ticks: usize,
    tick: usize,
    expires_at: usize,
    damage_bonus_percent: usize,
    attack_speed_mult: i32,
    buff_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    drop(caster);

    let fields = GRASSY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("grassy terrain state poisoned");
    let latest_for_caster = fields
        .iter()
        .filter(|state| state.caster_id == caster_id)
        .map(|state| state.created_at)
        .max();
    if latest_for_caster
        .map(|created_at| tick.saturating_sub(created_at) < cooldown_ticks)
        .unwrap_or(false)
    {
        return;
    }

    fields.push(GrassyTerrainState {
        caster_id,
        center,
        radius,
        created_at: tick,
        expires_at,
        next_tick_at: tick,
        damage_bonus_percent,
        attack_speed_mult,
        buff_ticks,
    });

    while fields
        .iter()
        .filter(|state| state.caster_id == caster_id)
        .count()
        > max_fields
    {
        let Some((index, _)) = fields
            .iter()
            .enumerate()
            .filter(|(_, state)| state.caster_id == caster_id)
            .min_by_key(|(_, state)| state.created_at)
        else {
            break;
        };
        fields.remove(index);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn begin_rillaboom_drum_aura(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    heal_per_tick: usize,
    ally_move_speed_mult: i32,
    ally_buff_ticks: usize,
    enemy_slow_percent: i32,
    enemy_slow_ticks: usize,
    final_stun_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let auras = RILLABOOM_DRUM_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("rillaboom drum aura state poisoned");
    auras.retain(|state| {
        !(state.caster_id == caster_id
            && state.enemy_slow_percent == enemy_slow_percent
            && state.final_stun_ticks == final_stun_ticks)
    });
    let tick = ctx.tick();
    auras.push(RillaboomDrumAuraState {
        caster_id,
        caster_team: caster.team(),
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        radius,
        heal_per_tick,
        ally_move_speed_mult,
        ally_buff_ticks,
        enemy_slow_percent,
        enemy_slow_ticks,
        final_stun_ticks,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn begin_rillaboom_grassy_surge(
    ctx: &GameCtx,
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    duration_ticks: usize,
    width: u64,
    tick_interval: usize,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    terrain_radius: u64,
    terrain_count: usize,
    terrain_damage_bonus_percent: usize,
    terrain_attack_speed_mult: i32,
    terrain_buff_ticks: usize,
) {
    let tick = ctx.tick();
    let terrain_count = terrain_count.max(1);
    for index in 0..terrain_count {
        let center = segment_point(start, end, index, terrain_count.saturating_sub(1).max(1));
        spawn_grassy_terrain_at(
            ctx,
            caster_id,
            center,
            terrain_count,
            terrain_radius,
            duration_ticks,
            terrain_damage_bonus_percent,
            terrain_attack_speed_mult,
            terrain_buff_ticks,
        );
    }

    let surges = RILLABOOM_GRASSY_SURGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut surges = surges
        .lock()
        .expect("rillaboom grassy surge state poisoned");
    surges.push(RillaboomGrassySurgeState {
        caster_id,
        caster_team,
        start,
        end,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        width,
        tick_interval: tick_interval.max(1),
        damage_per_tick,
        slow_percent,
        slow_ticks,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn begin_shiftry_tornado(
    ctx: &GameCtx,
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    duration_ticks: usize,
    radius: u64,
    tick_interval: usize,
    damage_per_tick: usize,
    damage_growth_percent: usize,
    lift_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    drop(caster);

    let tick = ctx.tick();
    let states = SHIFTRY_TORNADOES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("shiftry tornado state poisoned")
        .push(ShiftryTornadoState {
            caster_id,
            caster_team,
            center,
            expires_at: tick.saturating_add(duration_ticks),
            next_tick_at: tick,
            radius,
            tick_interval: tick_interval.max(1),
            damage_per_tick,
            damage_growth_percent,
            ticks_done: 0,
            lift_ticks,
        });
}

fn apply_status_force_move_toward_pos(
    ctx: &mut GameCtx,
    entity_id: usize,
    entity_pos: EntityPos,
    target_pos: EntityPos,
    speed: u64,
    tick: u64,
) {
    let distance = integer_sqrt(distance_sq(entity_pos, target_pos));
    let speed = bounded_force_move_speed(speed, tick, distance);
    if speed == 0 {
        return;
    }
    apply_pokemon_cc(ctx, entity_id, entity_id, CCState::Airborne { tick });
    apply_pokemon_cc(
        ctx,
        entity_id,
        entity_id,
        CCState::ForceMove {
            tick,
            dx: target_pos.x as i64 - entity_pos.x as i64,
            dy: target_pos.y as i64 - entity_pos.y as i64,
            speed,
        },
    );
}

fn bounded_force_move_speed(speed: u64, tick: u64, distance: u64) -> u64 {
    if speed == 0 || tick == 0 || distance == 0 {
        return 0;
    }
    let speed_to_cover_distance = distance
        .saturating_add(tick.saturating_sub(1))
        .saturating_div(tick)
        .max(1);
    speed.min(speed_to_cover_distance)
}

fn sky_circus_waypoints(start: EntityPos, center: EntityPos, radius: u64) -> Vec<EntityPos> {
    let dx = center.x as f64 - start.x as f64;
    let dy = center.y as f64 - start.y as f64;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let dir_x = dx / len;
    let dir_y = dy / len;
    let perp_x = -dir_y;
    let perp_y = dir_x;
    let radius = radius as f64;
    let entry_x = center.x as f64 - dir_x * radius * 0.82;
    let entry_y = center.y as f64 - dir_y * radius * 0.82;
    let exit_x = center.x as f64 + dir_x * radius * 0.82;
    let exit_y = center.y as f64 + dir_y * radius * 0.82;
    let mut points = Vec::new();
    for index in 0..6 {
        let t = index as f64 / 5.0;
        let base_x = entry_x + (exit_x - entry_x) * t;
        let base_y = entry_y + (exit_y - entry_y) * t;
        let wave = (std::f64::consts::PI * t).sin();
        let sign = if index % 2 == 0 { -1.0 } else { 1.0 };
        points.push(pos_from_f64(
            base_x + perp_x * radius * 0.46 * wave * sign,
            base_y + perp_y * radius * 0.46 * wave * sign,
        ));
    }
    points.push(start);
    points
}

#[allow(clippy::too_many_arguments)]
pub fn begin_swanna_cyclone(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    tick_interval: usize,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let tick = ctx.tick();
    let states = SWANNA_CYCLONES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("swanna cyclone state poisoned")
        .push(SwannaCycloneState {
            caster_id,
            caster_team: caster.team(),
            expires_at: tick.saturating_add(duration_ticks),
            next_tick_at: tick,
            radius,
            tick_interval: tick_interval.max(1),
            damage_per_tick: damage_per_tick.max(1),
            slow_percent: slow_percent.abs(),
            slow_ticks,
            attacker_types,
        });
}

#[allow(clippy::too_many_arguments)]
pub fn begin_swanna_sky_circus(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    start_pos: EntityPos,
    center: EntityPos,
    radius: u64,
    hits: usize,
    hit_interval_ticks: usize,
    hit_damage: usize,
    landing_damage: usize,
    bonus_damage_per_target_percent: usize,
    airborne_ticks: usize,
    waypoint_ticks: usize,
    force_move_speed: u64,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    drop(caster);

    let mut targets = Vec::new();
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if !entity.is_alive()
            || entity.team() == caster_team
            || entity.is_tower()
            || distance_sq(entity.pos(), center) > radius.saturating_mul(radius)
        {
            continue;
        }
        targets.push(entity.id());
    }
    if targets.is_empty() {
        return;
    }

    let target_damage_percent = 100usize.saturating_add(
        targets
            .len()
            .saturating_sub(1)
            .saturating_mul(bonus_damage_per_target_percent),
    );
    for target_id in targets.iter().copied() {
        apply_airborne_hard_cc(ctx, caster_id, target_id, airborne_ticks);
    }

    let tick = ctx.tick();
    let waypoints = sky_circus_waypoints(start_pos, center, radius);
    let first_waypoint = waypoints.first().copied().unwrap_or(center);
    apply_soft_untargetable(
        ctx,
        caster_id,
        waypoint_ticks.saturating_mul(waypoints.len() + 1),
    );
    ctx.add_buff(
        caster_id,
        BuffState {
            duration: BuffType::Time {
                tick: waypoint_ticks.saturating_mul(waypoints.len() + 1),
            },
            damaged_reduce: 100,
            ..Default::default()
        },
    );
    apply_status_force_move_toward_pos(
        ctx,
        caster_id,
        start_pos,
        first_waypoint,
        force_move_speed,
        waypoint_ticks as u64,
    );

    let states = SWANNA_SKY_CIRCUSES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("swanna sky circus state poisoned")
        .push(SwannaSkyCircusState {
            caster_id,
            start_pos,
            center,
            radius,
            waypoints,
            waypoint_index: 0,
            next_waypoint_at: tick.saturating_add(waypoint_ticks),
            waypoint_ticks: waypoint_ticks.max(1),
            force_move_speed: force_move_speed.max(1),
            next_hit_at: tick.saturating_add(hit_interval_ticks.max(1) / 2),
            hit_interval_ticks: hit_interval_ticks.max(1),
            hits_remaining: hits.max(1),
            hit_damage: hit_damage.max(1),
            landing_damage: landing_damage.max(1),
            target_damage_percent,
            targets,
            expires_at: tick.saturating_add(
                waypoint_ticks
                    .saturating_mul(8)
                    .max(airborne_ticks)
                    .saturating_add(30),
            ),
            attacker_types,
        });
}

pub fn update_swanna_tailwind(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let tick = ctx.tick();
    let states = SWANNA_TAILWINDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("swanna tailwind state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(SwannaTailwindState {
            entity_id,
            last_pos: pos,
            segment_dx: 0,
            segment_dy: 0,
            segment_distance: 0,
            buff_until: 0,
            cooldown_until: 0,
            last_buff_at: 0,
        });
        return;
    };

    let dx = pos.x as i64 - state.last_pos.x as i64;
    let dy = pos.y as i64 - state.last_pos.y as i64;
    state.last_pos = pos;
    let moved = squared_len_i128(dx as i128, dy as i128);
    if moved > 0 {
        let distance = integer_sqrt(moved.min(u64::MAX as i128) as u64);
        let dot = state.segment_dx as i128 * dx as i128 + state.segment_dy as i128 * dy as i128;
        if state.segment_distance == 0 || dot >= SWANNA_TAILWIND_DIRECTION_DOT_MIN {
            state.segment_dx = state.segment_dx.saturating_add(dx);
            state.segment_dy = state.segment_dy.saturating_add(dy);
            state.segment_distance = state.segment_distance.saturating_add(distance);
        } else {
            state.segment_dx = dx;
            state.segment_dy = dy;
            state.segment_distance = distance;
        }

        if state.segment_distance >= SWANNA_TAILWIND_TRIGGER_DISTANCE
            && tick >= state.cooldown_until
        {
            state.buff_until = tick.saturating_add(SWANNA_TAILWIND_BUFF_TICKS);
            state.cooldown_until = tick.saturating_add(SWANNA_TAILWIND_COOLDOWN_TICKS);
            state.segment_dx = 0;
            state.segment_dy = 0;
            state.segment_distance = 0;
            state.last_buff_at = 0;
        }
    }

    let should_refresh = state.buff_until > tick
        && tick.saturating_sub(state.last_buff_at) >= SWANNA_TAILWIND_REFRESH_TICKS;
    if should_refresh {
        state.last_buff_at = tick;
    }
    drop(states);

    if should_refresh {
        add_beneficial_buff(
            ctx,
            entity_id,
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: 35 },
                move_speed_mult: 25,
                attack_speed_mult: 18,
                skill_cooldown_mult: 18,
                ult_cooldown_mult: 18,
                ..Default::default()
            },
        );
        draw_status_marker(ctx, pos, 14000, VFX_FLYING);
    }
}

pub fn consume_swanna_tailwind_damage_amp(
    ctx: &GameCtx,
    entity_id: usize,
    caster_ap: usize,
) -> Option<(usize, usize)> {
    let tick = ctx.tick();
    let states = SWANNA_TAILWINDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("swanna tailwind state poisoned");
    let state = states
        .iter_mut()
        .find(|state| state.entity_id == entity_id)?;
    if state.buff_until <= tick {
        return None;
    }
    state.buff_until = tick;
    Some((10, caster_ap / 10))
}

fn update_swanna_cyclones(ctx: &mut GameCtx, tick: usize) {
    let states = SWANNA_CYCLONES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("swanna cyclone state poisoned");
    let mut ticks = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);
        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        state.next_tick_at = tick.saturating_add(state.tick_interval);
        let targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), caster_pos)
                        <= state.radius.saturating_mul(state.radius)
            })
            .map(|entity| entity.id())
            .collect();
        ticks.push((
            state.caster_id,
            state.damage_per_tick,
            state.slow_percent,
            state.slow_ticks,
            state.attacker_types,
            targets,
        ));
        true
    });
    drop(states);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_WATER);
    }
    for (caster_id, damage, slow_percent, slow_ticks, attacker_types, targets) in ticks {
        for target_id in targets {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                caster_id,
                target_id,
                0,
                damage.max(1),
                AttackType::Dot,
                PokemonType::Water,
                attacker_types,
                defender_types,
            );
            ctx.add_buff(
                target_id,
                BuffState {
                    duration: BuffType::Time { tick: slow_ticks },
                    move_speed_mult: -slow_percent.abs(),
                    ..Default::default()
                },
            );
        }
    }
}

fn update_swanna_sky_circuses(ctx: &mut GameCtx, tick: usize) {
    let states = SWANNA_SKY_CIRCUSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("swanna sky circus state poisoned");
    let mut protections = Vec::new();
    let mut force_moves = Vec::new();
    let mut hit_batches = Vec::new();
    let mut landing_batches = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);
        protections.push(state.caster_id);
        visuals.push((state.center, state.radius, caster_pos));

        if tick >= state.next_waypoint_at && state.waypoint_index + 1 < state.waypoints.len() {
            state.waypoint_index += 1;
            let target_pos = state.waypoints[state.waypoint_index];
            state.next_waypoint_at = tick.saturating_add(state.waypoint_ticks);
            force_moves.push((
                state.caster_id,
                caster_pos,
                target_pos,
                state.force_move_speed,
                state.waypoint_ticks as u64,
            ));
        }

        if tick >= state.next_hit_at && state.hits_remaining > 0 {
            let segment_start = if state.waypoint_index == 0 {
                state.start_pos
            } else {
                state.waypoints[state.waypoint_index - 1]
            };
            let segment_end = state
                .waypoints
                .get(state.waypoint_index)
                .copied()
                .unwrap_or(caster_pos);
            let hit_targets: Vec<usize> = state
                .targets
                .iter()
                .copied()
                .filter(|target_id| {
                    ctx.get_entity(*target_id)
                        .map(|entity| {
                            entity.is_alive()
                                && distance_to_segment_sq(entity.pos(), segment_start, segment_end)
                                    <= SWANNA_SKY_CIRCUS_HIT_RADIUS
                                        .saturating_mul(SWANNA_SKY_CIRCUS_HIT_RADIUS)
                        })
                        .unwrap_or(false)
                })
                .collect();
            if !hit_targets.is_empty() {
                hit_batches.push((
                    state.caster_id,
                    state
                        .hit_damage
                        .saturating_mul(state.target_damage_percent)
                        .saturating_div(100)
                        .max(1),
                    state.attacker_types,
                    hit_targets,
                ));
            }
            state.hits_remaining = state.hits_remaining.saturating_sub(1);
            state.next_hit_at = tick.saturating_add(state.hit_interval_ticks);
        }

        if tick >= state.expires_at
            || (state.hits_remaining == 0 && state.waypoint_index + 1 >= state.waypoints.len())
        {
            let live_targets: Vec<usize> = state
                .targets
                .iter()
                .copied()
                .filter(|target_id| {
                    ctx.get_entity(*target_id)
                        .map(|e| e.is_alive())
                        .unwrap_or(false)
                })
                .collect();
            if !live_targets.is_empty() {
                landing_batches.push((
                    state.caster_id,
                    state
                        .landing_damage
                        .saturating_mul(state.target_damage_percent)
                        .saturating_div(100)
                        .max(1),
                    state.attacker_types,
                    live_targets,
                ));
            }
            if distance_sq(caster_pos, state.start_pos) > 1200_u64.saturating_mul(1200) {
                force_moves.push((
                    state.caster_id,
                    caster_pos,
                    state.start_pos,
                    state.force_move_speed,
                    state.waypoint_ticks as u64,
                ));
            }
            return false;
        }
        true
    });
    drop(states);

    for caster_id in protections {
        apply_soft_untargetable(ctx, caster_id, 8);
        ctx.add_buff(
            caster_id,
            BuffState {
                duration: BuffType::Time { tick: 8 },
                damaged_reduce: 100,
                ..Default::default()
            },
        );
    }
    for (center, radius, caster_pos) in visuals {
        draw_field_circle(ctx, center, radius, VFX_FLYING);
        draw_status_marker(ctx, caster_pos, 12000, VFX_FLYING);
    }
    for (entity_id, entity_pos, target_pos, speed, ticks) in force_moves {
        apply_status_force_move_toward_pos(ctx, entity_id, entity_pos, target_pos, speed, ticks);
    }
    for (caster_id, damage, attacker_types, targets) in
        hit_batches.into_iter().chain(landing_batches)
    {
        for target_id in targets {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                caster_id,
                target_id,
                0,
                damage.max(1),
                AttackType::Skill,
                PokemonType::Flying,
                attacker_types,
                defender_types,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn begin_marowak_bone_windmill(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    tick_interval: usize,
    damage: usize,
    knockback_speed: u64,
    knockback_ticks: u64,
    empowered_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let tick = ctx.tick();
    let states = MAROWAK_BONE_WINDMILLS.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("marowak bone windmill state poisoned")
        .push(MarowakBoneWindmillState {
            caster_id,
            caster_team: caster.team(),
            expires_at: tick.saturating_add(duration_ticks),
            next_tick_at: tick,
            radius,
            tick_interval: tick_interval.max(1),
            damage: damage.max(1),
            knockback_speed,
            knockback_ticks,
            empowered_ticks,
            attacker_types,
        });
}

pub fn consume_marowak_windmill_bonemerang(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = MAROWAK_WINDMILL_BONEMERANGS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("marowak windmill bonemerang state poisoned");
    states.retain(|state| state.expires_at > tick);
    let Some(index) = states
        .iter()
        .position(|state| state.entity_id == entity_id && state.expires_at > tick)
    else {
        return false;
    };
    states.remove(index);
    true
}

fn update_marowak_bone_windmills(ctx: &mut GameCtx, tick: usize) {
    let states = MAROWAK_BONE_WINDMILLS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("marowak bone windmill state poisoned");
    let mut protections = Vec::new();
    let mut ticks = Vec::new();
    let mut visuals = Vec::new();
    let mut empowered = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        if state.expires_at <= tick {
            empowered.push((state.caster_id, state.empowered_ticks));
            return false;
        }

        protections.push(state.caster_id);
        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        state.next_tick_at = tick.saturating_add(state.tick_interval);
        let targets: Vec<(usize, EntityPos)> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && entity.is_targetable()
                    && distance_sq(entity.pos(), caster_pos)
                        <= state.radius.saturating_mul(state.radius)
            })
            .map(|entity| (entity.id(), entity.pos()))
            .collect();
        ticks.push((
            state.caster_id,
            caster_pos,
            state.damage,
            state.knockback_speed,
            state.knockback_ticks,
            state.attacker_types,
            targets,
        ));
        true
    });
    drop(states);

    for caster_id in protections {
        apply_pokemon_cc(ctx, caster_id, caster_id, CCState::Bind { tick: 8 });
        apply_pokemon_cc(ctx, caster_id, caster_id, CCState::BlockSkill { tick: 8 });
        add_beneficial_buff(
            ctx,
            caster_id,
            caster_id,
            BuffState {
                duration: BuffType::Time { tick: 8 },
                damaged_reduce: 100,
                ..Default::default()
            },
        );
    }
    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_GROUND);
    }
    for (caster_id, center, damage, knockback_speed, knockback_ticks, attacker_types, targets) in
        ticks
    {
        for (target_id, target_pos) in targets {
            let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                caster_id,
                actual_target_id,
                damage.max(1),
                0,
                AttackType::Skill,
                PokemonType::Ground,
                attacker_types,
                defender_types,
            );
            if !has_stalwart(ctx, actual_target_id) {
                let mut dx = target_pos.x as i64 - center.x as i64;
                let dy = target_pos.y as i64 - center.y as i64;
                if dx == 0 && dy == 0 {
                    dx = 1;
                }
                apply_pokemon_cc(
                    ctx,
                    caster_id,
                    actual_target_id,
                    CCState::ForceMove {
                        tick: knockback_ticks,
                        dx,
                        dy,
                        speed: knockback_speed,
                    },
                );
            }
        }
    }
    if !empowered.is_empty() {
        let states = MAROWAK_WINDMILL_BONEMERANGS.get_or_init(|| Mutex::new(Vec::new()));
        let mut states = states
            .lock()
            .expect("marowak windmill bonemerang state poisoned");
        for (entity_id, empowered_ticks) in empowered {
            states.retain(|state| state.entity_id != entity_id && state.expires_at > tick);
            states.push(MarowakWindmillBonemerangState {
                entity_id,
                expires_at: tick.saturating_add(empowered_ticks),
            });
        }
    }
}

pub fn garganacl_spawn_passive_salt(
    ctx: &GameCtx,
    caster_id: usize,
    max_patches: usize,
    radius: u64,
    duration_ticks: usize,
    damage: usize,
    slow_percent: i32,
    slow_ticks: usize,
    water_steel_bonus_percent: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let center = caster.pos();
    let caster_team = caster.team();
    drop(caster);

    let tick = ctx.tick();
    let patches = GARGANACL_SALT_PATCHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut patches = patches.lock().expect("garganacl salt patch state poisoned");
    patches.retain(|state| state.expires_at > tick);
    patches.push(GarganaclSaltPatchState {
        caster_id,
        caster_team,
        center,
        radius,
        created_at: tick,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        damage: damage.max(1),
        slow_percent,
        slow_ticks,
        water_steel_bonus_percent,
    });

    while patches
        .iter()
        .filter(|state| state.caster_id == caster_id)
        .count()
        > max_patches
    {
        let Some((index, _)) = patches
            .iter()
            .enumerate()
            .filter(|(_, state)| state.caster_id == caster_id)
            .min_by_key(|(_, state)| state.created_at)
        else {
            break;
        };
        patches.remove(index);
    }
}

pub fn trigger_garganacl_salt_cure_from_physical_damage(
    ctx: &GameCtx,
    target_id: usize,
    ad_damage: usize,
) {
    if ad_damage == 0 {
        return;
    }
    if !(entity_is_champion_id(target_id, "pokemon_moba_garganacl")
        || receiver_has_copied(target_id, "pokemon_moba_garganacl"))
    {
        return;
    }
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }
    let target_stat = target.stat();
    drop(target);

    let salt_damage = 22usize.saturating_add(target_stat.attack.saturating_mul(25) / 100);
    garganacl_spawn_passive_salt(ctx, target_id, 6, 22000, 5 * 60, salt_damage, 20, 90, 50);
}

#[allow(clippy::too_many_arguments)]
pub fn begin_garganacl_blessed_salt(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    duration_ticks: usize,
    outer_radius: u64,
    inner_radius: u64,
    tick_interval: usize,
    damage: usize,
    slow_percent: i32,
    slow_ticks: usize,
    anti_heal_percent: i32,
    anti_heal_ticks: usize,
    permanent_tick_interval: usize,
    permanent_damage: usize,
    water_steel_bonus_percent: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    drop(caster);

    let tick = ctx.tick();
    let states = GARGANACL_BLESSED_SALTS.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("garganacl blessed salt state poisoned")
        .push(GarganaclBlessedSaltState {
            caster_id,
            caster_team,
            center,
            outer_radius,
            expires_at: tick.saturating_add(duration_ticks),
            next_tick_at: tick,
            tick_interval: tick_interval.max(1),
            damage: damage.max(1),
            slow_percent,
            slow_ticks,
            anti_heal_percent,
            anti_heal_ticks,
        });

    let permanent = GARGANACL_PERMANENT_SALTS.get_or_init(|| Mutex::new(Vec::new()));
    permanent
        .lock()
        .expect("garganacl permanent salt state poisoned")
        .push(GarganaclPermanentSaltState {
            caster_id,
            caster_team,
            center,
            outer_radius,
            inner_radius,
            next_tick_at: tick.saturating_add(duration_ticks),
            tick_interval: permanent_tick_interval.max(1),
            damage: permanent_damage.max(1),
            slow_percent,
            slow_ticks,
            water_steel_bonus_percent,
        });
}

pub fn garganacl_entity_in_salt(ctx: &GameCtx, entity_id: usize) -> bool {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return false;
    };
    let pos = entity.pos();
    drop(entity);

    let tick = ctx.tick();
    if GARGANACL_SALT_PATCHES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("garganacl salt patch state poisoned")
        .iter()
        .any(|state| {
            state.expires_at > tick
                && distance_sq(pos, state.center) <= state.radius.saturating_mul(state.radius)
        })
    {
        return true;
    }

    GARGANACL_PERMANENT_SALTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("garganacl permanent salt state poisoned")
        .iter()
        .any(|state| {
            let dist = distance_sq(pos, state.center);
            dist <= state.outer_radius.saturating_mul(state.outer_radius)
                && dist >= state.inner_radius.saturating_mul(state.inner_radius)
        })
}

fn salt_damage_for_target(
    ctx: &GameCtx,
    target_id: usize,
    base_damage: usize,
    water_steel_bonus_percent: usize,
) -> usize {
    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
    let vulnerable = defender_types
        .iter()
        .any(|ty| matches!(ty, PokemonType::Water | PokemonType::Steel));
    if vulnerable {
        base_damage.saturating_mul(100usize.saturating_add(water_steel_bonus_percent)) / 100
    } else {
        base_damage
    }
}

fn update_garganacl_salt(ctx: &mut GameCtx, tick: usize) {
    let mut patch_hits = Vec::new();
    let mut patch_visuals = Vec::new();
    let patches = GARGANACL_SALT_PATCHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut patches = patches.lock().expect("garganacl salt patch state poisoned");
    patches.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        patch_visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        state.next_tick_at = tick.saturating_add(30);

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || !entity.is_targetable()
                || distance_sq(entity.pos(), state.center)
                    > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            patch_hits.push((
                state.caster_id,
                entity.id(),
                state.damage,
                state.slow_percent,
                state.slow_ticks,
                state.water_steel_bonus_percent,
            ));
            return false;
        }
        true
    });
    drop(patches);

    let mut pulse_hits = Vec::new();
    let blessed = GARGANACL_BLESSED_SALTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut blessed = blessed
        .lock()
        .expect("garganacl blessed salt state poisoned");
    blessed.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        patch_visuals.push((state.center, state.outer_radius));
        if state.next_tick_at > tick {
            return true;
        }
        state.next_tick_at = tick.saturating_add(state.tick_interval);
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || !entity.is_targetable()
                || distance_sq(entity.pos(), state.center)
                    > state.outer_radius.saturating_mul(state.outer_radius)
            {
                continue;
            }
            pulse_hits.push((
                state.caster_id,
                entity.id(),
                state.damage,
                state.slow_percent,
                state.slow_ticks,
                state.anti_heal_percent,
                state.anti_heal_ticks,
            ));
        }
        true
    });
    drop(blessed);

    let mut permanent_hits = Vec::new();
    let permanent = GARGANACL_PERMANENT_SALTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut permanent = permanent
        .lock()
        .expect("garganacl permanent salt state poisoned");
    permanent.retain_mut(|state| {
        patch_visuals.push((state.center, state.outer_radius));
        if state.next_tick_at > tick {
            return true;
        }
        state.next_tick_at = tick.saturating_add(state.tick_interval);
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            let dist = distance_sq(entity.pos(), state.center);
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || !entity.is_targetable()
                || dist > state.outer_radius.saturating_mul(state.outer_radius)
                || dist < state.inner_radius.saturating_mul(state.inner_radius)
            {
                continue;
            }
            permanent_hits.push((
                state.caster_id,
                entity.id(),
                state.damage,
                state.slow_percent,
                state.slow_ticks,
                state.water_steel_bonus_percent,
            ));
        }
        true
    });
    drop(permanent);

    for (center, radius) in patch_visuals {
        draw_field_circle(ctx, center, radius, VFX_GROUND);
    }
    for (caster_id, target_id, damage, slow_percent, slow_ticks, bonus) in
        patch_hits.into_iter().chain(permanent_hits.into_iter())
    {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        let damage = salt_damage_for_target(ctx, actual_target_id, damage, bonus);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Rock,
            TypeSet::single(PokemonType::Rock),
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id),
        );
        ctx.add_buff(
            actual_target_id,
            BuffState {
                duration: BuffType::Time { tick: slow_ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }
    for (
        caster_id,
        target_id,
        damage,
        slow_percent,
        slow_ticks,
        anti_heal_percent,
        anti_heal_ticks,
    ) in pulse_hits
    {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Rock,
            TypeSet::single(PokemonType::Rock),
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id),
        );
        ctx.add_buff(
            actual_target_id,
            BuffState {
                duration: BuffType::Time { tick: slow_ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
        apply_anti_heal(
            ctx,
            caster_id,
            actual_target_id,
            anti_heal_percent.max(0) as usize,
            anti_heal_ticks,
        );
    }
}

fn update_ampharos_gigavolts(ctx: &mut GameCtx, tick: usize) {
    let states = AMPHAROS_GIGAVOLTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("ampharos gigavolt state poisoned");
    states.retain(|state| {
        state.expires_at > tick
            && ctx
                .get_entity(state.caster_id)
                .map(|caster| caster.is_alive())
                .unwrap_or(false)
    });

    let mut initial_strikes = Vec::new();
    let mut zone_ticks = Vec::new();
    for state in states.iter_mut() {
        if !state.triggered && tick >= state.trigger_at {
            state.triggered = true;
            state.next_tick_at = tick;
            initial_strikes.push(*state);
        }
        if state.triggered && tick >= state.next_tick_at {
            state.next_tick_at = tick.saturating_add(state.zone_tick_interval.max(1));
            zone_ticks.push(*state);
        }
    }
    drop(states);

    for state in initial_strikes {
        let radius_sq = state.radius.saturating_mul(state.radius);
        let targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), state.center) <= radius_sq
            })
            .map(|entity| entity.id())
            .collect();
        for target_id in targets {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                state.caster_id,
                target_id,
                0,
                state.damage.max(1),
                AttackType::Skill,
                PokemonType::Electric,
                state.attacker_types,
                defender_types,
            );
        }
        draw_status_marker(ctx, state.center, state.radius, VFX_ELECTRIC);
    }

    for state in zone_ticks {
        let radius_sq = state.radius.saturating_mul(state.radius);
        let targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), state.center) <= radius_sq
            })
            .map(|entity| entity.id())
            .collect();
        for target_id in targets {
            add_harmful_buff(
                ctx,
                state.caster_id,
                target_id,
                BuffState {
                    duration: BuffType::Time {
                        tick: state.zone_slow_ticks,
                    },
                    move_speed_mult: -state.zone_slow_percent.abs(),
                    ..Default::default()
                },
            );
        }

        if let Some(caster) = ctx.get_entity(state.caster_id) {
            if caster.is_alive()
                && distance_sq(caster.pos(), state.center)
                    <= state.radius.saturating_mul(state.radius)
            {
                drop(caster);
                add_beneficial_buff(
                    ctx,
                    state.caster_id,
                    state.caster_id,
                    BuffState {
                        duration: BuffType::Time {
                            tick: state.attack_speed_buff_ticks,
                        },
                        attack_speed_mult: state.attack_speed_mult,
                        ..Default::default()
                    },
                );
            }
        }
        draw_field_circle(ctx, state.center, state.radius, VFX_ELECTRIC);
    }
}

pub fn update_xatu_stillness(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let states = XATU_STILLNESSES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("xatu stillness state poisoned");
    states.retain(|state| {
        ctx.get_entity(state.entity_id)
            .map(|entity| entity.is_alive() && entity.is_champion())
            .unwrap_or(false)
    });
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        let moved = state
            .last_pos
            .map(|last_pos| distance_sq(last_pos, pos) >= XATU_STILL_MOVE_THRESHOLD_SQ)
            .unwrap_or(false);
        state.last_pos = Some(pos);
        if moved {
            state.still_ticks = 0;
        } else {
            state.still_ticks = state.still_ticks.saturating_add(1);
        }
        return;
    }
    states.push(XatuStillnessState {
        entity_id,
        last_pos: Some(pos),
        still_ticks: 0,
    });
}

pub fn xatu_stood_still_for(ctx: &GameCtx, entity_id: usize, ticks: usize) -> bool {
    let states = XATU_STILLNESSES.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("xatu stillness state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && state.still_ticks >= ticks
                && ctx
                    .get_entity(entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        })
}

pub fn apply_xatu_mind_bend(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    damage: usize,
    confusion_ticks: usize,
    still_ticks_required: usize,
    attacker_types: TypeSet,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    drop(target);

    apply_confusion_from(ctx, caster_id, target_id, 1, confusion_ticks);
    if xatu_stood_still_for(ctx, caster_id, still_ticks_required) {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage.max(1),
            AttackType::Skill,
            PokemonType::Psychic,
            attacker_types,
            defender_types,
        );
    }
}

pub fn apply_xatu_pain_amplifier(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    damage_taken_percent: usize,
    duration_ticks: usize,
    stun_ticks: usize,
    still_ticks_required: usize,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    let target_pos = target.pos();
    drop(target);

    let tick = ctx.tick();
    let states = XATU_PAIN_AMPLIFIERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("xatu pain amplifier state poisoned");
    states.retain(|state| state.expires_at > tick && state.target_id != target_id);
    states.push(XatuPainAmplifierState {
        target_id,
        expires_at: tick.saturating_add(duration_ticks),
        damage_taken_percent,
    });
    drop(states);

    add_harmful_buff(
        ctx,
        caster_id,
        target_id,
        BuffState {
            duration: BuffType::Time {
                tick: duration_ticks,
            },
            magic_resistance_mult: -10,
            ..Default::default()
        },
    );

    if xatu_stood_still_for(ctx, caster_id, still_ticks_required) && !is_limber(ctx, target_id) {
        let ticks = adjusted_cc_ticks(ctx, target_id, stun_ticks);
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Stun { tick: ticks as u64 },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill { tick: ticks },
        );
    }
    draw_status_marker(ctx, target_pos, 14000, VFX_PSYCHIC);
}

pub fn xatu_pain_amplifier_bonus_percent(ctx: &GameCtx, target_id: usize) -> usize {
    let tick = ctx.tick();
    let states = XATU_PAIN_AMPLIFIERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("xatu pain amplifier state poisoned");
    states.retain(|state| {
        state.expires_at > tick
            && ctx
                .get_entity(state.target_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    states
        .iter()
        .filter(|state| state.target_id == target_id)
        .map(|state| state.damage_taken_percent)
        .max()
        .unwrap_or(0)
}

pub fn note_xatu_prophecy_skill_cast(ctx: &mut GameCtx, caster_id: usize) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() || !caster.is_champion() {
        return;
    }
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);

    let radius_sq = XATU_PROPHECY_RADIUS.saturating_mul(XATU_PROPHECY_RADIUS);
    let tick = ctx.tick();
    let xatus: Vec<(usize, usize)> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != caster_team
                && entity.is_alive()
                && entity.is_champion()
                && (entity_is_champion_id(entity.id(), "pokemon_moba_xatu")
                    || receiver_has_copied(entity.id(), "pokemon_moba_xatu"))
                && distance_sq(entity.pos(), caster_pos) <= radius_sq
        })
        .map(|entity| (entity.id(), entity.team()))
        .collect();

    for (xatu_id, xatu_team) in xatus {
        note_team_spot_for_ticks(
            tick,
            xatu_id,
            xatu_team,
            caster_id,
            caster_pos,
            XATU_PROPHECY_REVEAL_TICKS,
        );
        add_harmful_buff(
            ctx,
            xatu_id,
            caster_id,
            BuffState {
                duration: BuffType::Time {
                    tick: XATU_PROPHECY_DEBUFF_TICKS,
                },
                magic_resistance_mult: XATU_PROPHECY_MAGIC_RESIST_MULT,
                ..Default::default()
            },
        );
        draw_status_marker(ctx, caster_pos, 12000, VFX_PSYCHIC);
    }
}

pub fn begin_xatu_super_psy(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    channel_ticks: usize,
    tick_interval: usize,
    width: u64,
    travel_range: u64,
    damage: usize,
    close_bonus_percent: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    drop(caster);

    let tick = ctx.tick();
    let states = XATU_SUPER_PSYS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("xatu super psy state poisoned");
    states.retain(|state| state.caster_id != caster_id && state.expires_at > tick);
    states.push(XatuSuperPsyState {
        caster_id,
        caster_team,
        end,
        expires_at: tick.saturating_add(channel_ticks),
        next_tick_at: tick,
        tick_interval,
        width,
        travel_range,
        damage,
        close_bonus_percent,
        attacker_types,
    });
    drop(states);

    apply_pokemon_cc(
        ctx,
        caster_id,
        caster_id,
        CCState::Bind {
            tick: channel_ticks as u64,
        },
    );
    ctx.add_buff(
        caster_id,
        BuffState {
            duration: BuffType::Time {
                tick: channel_ticks,
            },
            cc_immune: true,
            ..Default::default()
        },
    );
    draw_line_band(ctx, start, end, width, VFX_PSYCHIC);
}

fn xatu_super_psy_aim_end(
    ctx: &GameCtx,
    state: XatuSuperPsyState,
    caster_pos: EntityPos,
) -> EntityPos {
    let fallback = if distance_sq(caster_pos, state.end) == 0 {
        EntityPos {
            x: caster_pos.x.saturating_add(state.travel_range),
            y: caster_pos.y,
        }
    } else {
        state.end
    };

    let target_pos = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != state.caster_team
                && entity.is_alive()
                && entity.is_champion()
                && !is_soft_untargetable(ctx, entity.id())
        })
        .map(|entity| {
            let hp = entity.hp();
            let hp_score = hp.current.saturating_mul(1000) / hp.max.max(1);
            (
                hp_score,
                distance_sq(caster_pos, entity.pos()),
                entity.pos(),
            )
        })
        .min_by_key(|(hp_score, distance, _)| (*hp_score, *distance))
        .map(|(_, _, pos)| pos)
        .unwrap_or(fallback);

    let dx = target_pos.x as f64 - caster_pos.x as f64;
    let dy = target_pos.y as f64 - caster_pos.y as f64;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0 {
        return fallback;
    }

    pos_from_f64(
        caster_pos.x as f64 + dx / len * state.travel_range as f64,
        caster_pos.y as f64 + dy / len * state.travel_range as f64,
    )
}

fn update_xatu_super_psys(ctx: &mut GameCtx, tick: usize) {
    let states = XATU_SUPER_PSYS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("xatu super psy state poisoned");
    let mut tick_states = Vec::new();
    let mut active_states = Vec::new();
    states.retain_mut(|state| {
        let alive = ctx
            .get_entity(state.caster_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        if !alive || state.expires_at <= tick {
            return false;
        }
        active_states.push(*state);
        if tick >= state.next_tick_at {
            tick_states.push(*state);
            state.next_tick_at = tick.saturating_add(state.tick_interval.max(1));
        }
        true
    });
    drop(states);

    for state in active_states {
        if let Some(caster) = ctx.get_entity(state.caster_id) {
            let caster_pos = caster.pos();
            drop(caster);
            let aim_end = xatu_super_psy_aim_end(ctx, state, caster_pos);
            apply_pokemon_cc(
                ctx,
                state.caster_id,
                state.caster_id,
                CCState::Bind { tick: 8 },
            );
            ctx.add_buff(
                state.caster_id,
                BuffState {
                    duration: BuffType::Time { tick: 8 },
                    cc_immune: true,
                    ..Default::default()
                },
            );
            for index in 0..ctx.entity_count() {
                let Some(entity) = ctx.entity_at(index) else {
                    continue;
                };
                if entity.team() != state.caster_team && entity.is_alive() && entity.is_champion() {
                    note_team_spot_for_ticks(
                        tick,
                        state.caster_id,
                        state.caster_team,
                        entity.id(),
                        entity.pos(),
                        XATU_PROPHECY_REVEAL_TICKS,
                    );
                }
            }
            draw_line_band(ctx, caster_pos, aim_end, state.width, VFX_PSYCHIC);
        }
    }

    for state in tick_states {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            continue;
        };
        let caster_pos = caster.pos();
        drop(caster);
        let aim_end = xatu_super_psy_aim_end(ctx, state, caster_pos);
        let width_sq = state.width.saturating_mul(state.width);
        let targets: Vec<(usize, EntityPos)> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_to_segment_sq(entity.pos(), caster_pos, aim_end) <= width_sq
            })
            .map(|entity| (entity.id(), entity.pos()))
            .collect();
        for (target_id, target_pos) in targets {
            let distance = integer_sqrt(distance_sq(caster_pos, target_pos));
            let closeness = state
                .travel_range
                .saturating_sub(distance.min(state.travel_range));
            let bonus = state.close_bonus_percent.saturating_mul(closeness as usize)
                / state.travel_range.max(1) as usize;
            let damage = state.damage.saturating_mul(100 + bonus) / 100;
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                state.caster_id,
                target_id,
                0,
                damage.max(1),
                AttackType::Skill,
                PokemonType::Psychic,
                state.attacker_types,
                defender_types,
            );
        }
    }
}

fn has_quaquaval_aqua_step(entity_id: usize) -> bool {
    entity_is_champion_id(entity_id, "pokemon_moba_quaquaval")
        || receiver_has_copied(entity_id, "pokemon_moba_quaquaval")
}

pub fn begin_quaquaval_exciting_dance(
    ctx: &mut GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    attack_speed_mult: i32,
    move_speed_mult: i32,
) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let tick = ctx.tick();
    let states = QUAQUAVAL_EXCITING_DANCES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("quaquaval exciting dance state poisoned");
    states.retain(|state| state.entity_id != entity_id && state.expires_at > tick);
    states.push(QuaquavalExcitingDanceState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
    });
    drop(states);

    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: duration_ticks,
            },
            attack_speed_mult,
            move_speed_mult,
            ..Default::default()
        },
    );
    draw_status_marker(ctx, pos, 24000, VFX_WATER);
}

fn quaquaval_exciting_dance_active(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = QUAQUAVAL_EXCITING_DANCES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("quaquaval exciting dance state poisoned");
    states.retain(|state| {
        state
            .expires_at
            .saturating_add(QUAQUAVAL_EXCITING_DANCE_TRAIL_GRACE_TICKS)
            > tick
            && ctx
                .get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    states
        .iter()
        .any(|state| state.entity_id == entity_id && state.expires_at >= tick)
}

pub fn update_quaquaval_aqua_step(ctx: &mut GameCtx, entity_id: usize) {
    if !has_quaquaval_aqua_step(entity_id) {
        return;
    }
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let pos = entity.pos();
    let team = entity.team();
    drop(entity);

    let tick = ctx.tick();
    let emitters = QUAQUAVAL_AQUA_STEP_EMITTERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut emitters = emitters
        .lock()
        .expect("quaquaval aqua step emitter state poisoned");
    emitters.retain(|state| {
        ctx.get_entity(state.entity_id)
            .map(|entity| entity.is_alive() && has_quaquaval_aqua_step(state.entity_id))
            .unwrap_or(false)
    });
    let state = if let Some(existing) = emitters
        .iter_mut()
        .find(|state| state.entity_id == entity_id)
    {
        existing
    } else {
        emitters.push(QuaquavalAquaStepEmitterState {
            entity_id,
            last_pos: Some(pos),
            last_drop_pos: Some(pos),
            last_drop_tick: tick,
        });
        return;
    };

    let previous_drop = state.last_drop_pos.unwrap_or(pos);
    let moved_enough = distance_sq(previous_drop, pos)
        >= QUAQUAVAL_AQUA_STEP_DROP_DISTANCE.saturating_mul(QUAQUAVAL_AQUA_STEP_DROP_DISTANCE);
    let interval_ready =
        tick.saturating_sub(state.last_drop_tick) >= QUAQUAVAL_AQUA_STEP_DROP_INTERVAL_TICKS;
    state.last_pos = Some(pos);
    if !moved_enough || !interval_ready {
        return;
    }
    state.last_drop_pos = Some(pos);
    state.last_drop_tick = tick;
    drop(emitters);

    let empowered = quaquaval_exciting_dance_active(ctx, entity_id);
    let segments = QUAQUAVAL_AQUA_STEP_SEGMENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut segments = segments
        .lock()
        .expect("quaquaval aqua step segment state poisoned");
    segments.retain(|segment| {
        segment.expires_at > tick
            && ctx
                .get_entity(segment.caster_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    segments.push(QuaquavalAquaStepSegmentState {
        caster_id: entity_id,
        caster_team: team,
        start: previous_drop,
        end: pos,
        width: QUAQUAVAL_AQUA_STEP_WIDTH,
        expires_at: tick.saturating_add(QUAQUAVAL_AQUA_STEP_SEGMENT_TICKS),
        next_tick_at: tick,
        empowered,
    });
    while segments
        .iter()
        .filter(|segment| segment.caster_id == entity_id)
        .count()
        > QUAQUAVAL_AQUA_STEP_MAX_SEGMENTS
    {
        if let Some((remove_index, _)) = segments
            .iter()
            .enumerate()
            .filter(|(_, segment)| segment.caster_id == entity_id)
            .min_by_key(|(_, segment)| segment.expires_at)
        {
            segments.remove(remove_index);
        } else {
            break;
        }
    }
}

fn update_quaquaval_aqua_step_segments(ctx: &mut GameCtx, tick: usize) {
    let segments = QUAQUAVAL_AQUA_STEP_SEGMENTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut segments = segments
        .lock()
        .expect("quaquaval aqua step segment state poisoned");
    let mut visuals = Vec::new();
    let mut ally_buffs = Vec::new();
    let mut enemy_slows = Vec::new();

    segments.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if !ctx
            .get_entity(state.caster_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false)
        {
            return false;
        }
        visuals.push((state.start, state.end, state.width, state.empowered));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at = state
                .next_tick_at
                .saturating_add(QUAQUAVAL_AQUA_STEP_INTERVAL_TICKS);
        }
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || !entity.is_champion()
                || distance_to_segment_sq(entity.pos(), state.start, state.end)
                    > state.width.saturating_mul(state.width)
            {
                continue;
            }
            if entity.team() == state.caster_team {
                let speed = if state.empowered {
                    QUAQUAVAL_AQUA_STEP_EMPOWERED_ALLY_SPEED_MULT
                } else {
                    QUAQUAVAL_AQUA_STEP_ALLY_SPEED_MULT
                };
                ally_buffs.push((state.caster_id, entity.id(), speed));
            } else {
                let slow = if state.empowered {
                    QUAQUAVAL_AQUA_STEP_EMPOWERED_ENEMY_SLOW_MULT
                } else {
                    QUAQUAVAL_AQUA_STEP_ENEMY_SLOW_MULT
                };
                enemy_slows.push((state.caster_id, entity.id(), slow));
            }
        }
        true
    });
    drop(segments);

    for (start, end, width, empowered) in visuals {
        draw_line_band(
            ctx,
            start,
            end,
            if empowered {
                width.saturating_mul(3) / 2
            } else {
                width
            },
            VFX_WATER,
        );
    }
    for (caster_id, target_id, move_speed_mult) in ally_buffs {
        add_beneficial_buff(
            ctx,
            caster_id,
            target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: QUAQUAVAL_AQUA_STEP_INTERVAL_TICKS.saturating_mul(2),
                },
                move_speed_mult,
                ..Default::default()
            },
        );
    }
    for (caster_id, target_id, slow_percent) in enemy_slows {
        add_harmful_buff(
            ctx,
            caster_id,
            target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: QUAQUAVAL_AQUA_STEP_INTERVAL_TICKS.saturating_mul(2),
                },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }
}

pub fn quaquaval_water_damage_bonus_percent(ctx: &GameCtx, caster_id: usize) -> usize {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return 0;
    };
    if !caster.is_alive() {
        return 0;
    }
    let team = caster.team();
    let pos = caster.pos();
    drop(caster);

    let tick = ctx.tick();
    let segments = QUAQUAVAL_AQUA_STEP_SEGMENTS.get_or_init(|| Mutex::new(Vec::new()));
    segments
        .lock()
        .expect("quaquaval aqua step segment state poisoned")
        .iter()
        .filter(|segment| {
            segment.expires_at > tick
                && segment.caster_team == team
                && distance_to_segment_sq(pos, segment.start, segment.end)
                    <= segment.width.saturating_mul(segment.width)
        })
        .map(|segment| {
            if segment.empowered {
                QUAQUAVAL_AQUA_STEP_EMPOWERED_WATER_DAMAGE_BONUS
            } else {
                QUAQUAVAL_AQUA_STEP_WATER_DAMAGE_BONUS
            }
        })
        .max()
        .unwrap_or(0)
}

pub fn quaquaval_recently_hit_by_spiral_shot(ctx: &GameCtx, target_id: usize) -> bool {
    let tick = ctx.tick();
    let states = QUAQUAVAL_SPIRAL_SHOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("quaquaval spiral shot state poisoned");
    states.retain(|state| {
        state.expires_at > tick
            && ctx
                .get_entity(state.target_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    states.iter().any(|state| state.target_id == target_id)
}

pub fn apply_quaquaval_spiral_shot(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    damage: usize,
    slow_percent: i32,
    slow_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.is_tower() {
        return;
    }
    let target_pos = target.pos();
    drop(target);

    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
    crate::pokemon_types::deal_pokemon_damage(
        ctx,
        caster_id,
        target_id,
        damage.max(1),
        0,
        AttackType::Skill,
        PokemonType::Water,
        attacker_types,
        defender_types,
    );
    add_harmful_buff(
        ctx,
        caster_id,
        target_id,
        BuffState {
            duration: BuffType::Time { tick: slow_ticks },
            move_speed_mult: -slow_percent.abs(),
            ..Default::default()
        },
    );
    let states = QUAQUAVAL_SPIRAL_SHOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("quaquaval spiral shot state poisoned");
    let expires_at = ctx.tick().saturating_add(slow_ticks);
    states.retain(|state| state.target_id != target_id && state.expires_at > ctx.tick());
    states.push(QuaquavalSpiralShotState {
        target_id,
        expires_at,
    });
    draw_status_marker(ctx, target_pos, 13000, VFX_WATER);
}

fn quaquaval_path_crosses_own_aqua_step(
    ctx: &GameCtx,
    caster_id: usize,
    start: EntityPos,
    end: EntityPos,
    check_width: u64,
) -> bool {
    let tick = ctx.tick();
    let segments = QUAQUAVAL_AQUA_STEP_SEGMENTS.get_or_init(|| Mutex::new(Vec::new()));
    let segments = segments
        .lock()
        .expect("quaquaval aqua step segment state poisoned");
    segments
        .iter()
        .filter(|segment| segment.caster_id == caster_id && segment.expires_at > tick)
        .any(|segment| {
            (0..=5).any(|step| {
                let t = step as f64 / 5.0;
                let sample = pos_from_f64(
                    start.x as f64 + (end.x as f64 - start.x as f64) * t,
                    start.y as f64 + (end.y as f64 - start.y as f64) * t,
                );
                distance_to_segment_sq(sample, segment.start, segment.end)
                    <= check_width
                        .saturating_add(segment.width)
                        .saturating_mul(check_width.saturating_add(segment.width))
            })
        })
}

pub fn apply_quaquaval_up_tempo(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    base_damage: usize,
    empowered_bonus_percent: usize,
    dash_speed: u64,
    dash_ticks: usize,
    knockback_speed: u64,
    knockback_ticks: usize,
    empowered_knockback_ticks: usize,
    stun_ticks: usize,
    trail_check_width: u64,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_pos = caster.pos();
    drop(caster);
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || !target.is_champion() {
        return;
    }
    let target_pos = target.pos();
    drop(target);

    let empowered = quaquaval_path_crosses_own_aqua_step(
        ctx,
        caster_id,
        caster_pos,
        target_pos,
        trail_check_width,
    );
    force_move_toward_pos(
        ctx, caster_id, caster_pos, target_pos, dash_speed, dash_ticks,
    );
    let damage = if empowered {
        base_damage.saturating_mul(100usize.saturating_add(empowered_bonus_percent)) / 100
    } else {
        base_damage
    };
    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
    crate::pokemon_types::deal_pokemon_damage(
        ctx,
        caster_id,
        target_id,
        damage.max(1),
        0,
        AttackType::Skill,
        PokemonType::Fighting,
        attacker_types,
        defender_types,
    );
    if !has_stalwart(ctx, target_id) {
        let dx = target_pos.x as f64 - caster_pos.x as f64;
        let dy = target_pos.y as f64 - caster_pos.y as f64;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let distance = if empowered { 32000.0 } else { 17000.0 };
        let knock_to = pos_from_f64(
            target_pos.x as f64 + dx / len * distance,
            target_pos.y as f64 + dy / len * distance,
        );
        force_move_toward_pos(
            ctx,
            target_id,
            target_pos,
            knock_to,
            knockback_speed,
            if empowered {
                empowered_knockback_ticks
            } else {
                knockback_ticks
            },
        );
    }
    if empowered && !is_limber(ctx, target_id) {
        let ticks = adjusted_cc_ticks(ctx, target_id, stun_ticks);
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Stun { tick: ticks as u64 },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill { tick: ticks },
        );
    }
    draw_line_band(ctx, caster_pos, target_pos, trail_check_width, VFX_WATER);
}

#[allow(clippy::too_many_arguments)]
pub fn apply_arcanine_extremespeed(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    damage: usize,
    attacker_types: TypeSet,
    line_width: u64,
    dash_speed: u64,
    dash_ticks: usize,
    shield_hp_percent: usize,
    shield_ticks: usize,
    broken_move_speed_mult: i32,
    broken_move_speed_ticks: usize,
) {
    force_move_toward_pos(ctx, caster_id, start, end, dash_speed, dash_ticks);
    let width_sq = line_width.saturating_mul(line_width);
    let targets: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != caster_team
                && entity.is_alive()
                && !entity.is_tower()
                && distance_to_segment_sq(entity.pos(), start, end) <= width_sq
        })
        .map(|entity| entity.id())
        .collect();

    for target_id in targets {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Normal,
            attacker_types,
            defender_types,
        );
    }

    if let Some(caster) = ctx.get_entity(caster_id) {
        let shield = caster.hp().max.saturating_mul(shield_hp_percent) / 100;
        drop(caster);
        if shield > 0 {
            add_beneficial_buff(
                ctx,
                caster_id,
                caster_id,
                BuffState {
                    duration: BuffType::Time { tick: shield_ticks },
                    hp: shield.min(i32::MAX as usize) as i32,
                    ..Default::default()
                },
            );
            let states = ARCANINE_EXTREMESPEED_SHIELDS.get_or_init(|| Mutex::new(Vec::new()));
            let mut states = states
                .lock()
                .expect("arcanine extremespeed shield state poisoned");
            let expires_at = ctx.tick().saturating_add(shield_ticks);
            states.retain(|state| state.entity_id != caster_id && state.expires_at > ctx.tick());
            states.push(ArcanineExtremespeedShieldState {
                entity_id: caster_id,
                expires_at,
                broken_move_speed_mult,
                broken_move_speed_ticks,
            });
        }
    }
    draw_line_band(ctx, start, end, line_width, VFX_NORMAL);
}

#[allow(clippy::too_many_arguments)]
pub fn apply_arcanine_white_flames(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    damage: usize,
    attacker_types: TypeSet,
    line_width: u64,
    side_offset: u64,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
    heal_percent: usize,
) {
    let (left_start, left_end, right_start, right_end) =
        parallel_line_segments(start, end, side_offset);
    let width_sq = line_width.saturating_mul(line_width);
    let targets: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != caster_team
                && entity.is_alive()
                && !entity.is_tower()
                && (distance_to_segment_sq(entity.pos(), left_start, left_end) <= width_sq
                    || distance_to_segment_sq(entity.pos(), right_start, right_end) <= width_sq)
        })
        .map(|entity| entity.id())
        .collect();

    for target_id in targets {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        let result = crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage.max(1),
            AttackType::Skill,
            PokemonType::Fire,
            attacker_types,
            defender_types,
        );
        arcanine_heal_from_ability_damage(
            ctx,
            caster_id,
            target_id,
            result.applied_damage,
            heal_percent,
        );
        if burn_chance_percent >= 100
            || chance_percent(
                ctx.seed(),
                caster_id,
                target_id,
                ctx.tick(),
                burn_chance_percent,
            )
        {
            apply_burn_for(ctx, caster_id, target_id, burn_damage.max(1), burn_ticks);
        }
    }
    draw_line_band(ctx, left_start, left_end, line_width, VFX_FIRE);
    draw_line_band(ctx, right_start, right_end, line_width, VFX_FIRE);
}

pub fn apply_arcanine_flames_of_rage(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    radius: u64,
    base_damage: usize,
    burned_bonus_percent: usize,
    heal_missing_hp_percent_per_champion: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_pos = caster.pos();
    let max_hp = caster.hp().max.max(1);
    let current_hp = caster.hp().current.min(max_hp);
    drop(caster);

    let missing_bonus_percent = max_hp.saturating_sub(current_hp).saturating_mul(100) / max_hp;
    let radius_sq = radius.saturating_mul(radius);
    let targets: Vec<(usize, bool)> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != caster_team
                && entity.is_alive()
                && !entity.is_tower()
                && distance_sq(entity.pos(), caster_pos) <= radius_sq
        })
        .map(|entity| (entity.id(), entity.is_champion()))
        .collect();

    let mut champion_hits = 0usize;
    for (target_id, is_champion) in targets {
        let mut damage = base_damage.saturating_mul(100 + missing_bonus_percent) / 100;
        if is_burned(ctx, target_id) {
            damage = damage.saturating_mul(100 + burned_bonus_percent) / 100;
        }
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        let result = crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage.max(1),
            AttackType::Skill,
            PokemonType::Fire,
            attacker_types,
            defender_types,
        );
        arcanine_heal_from_ability_damage(
            ctx,
            caster_id,
            target_id,
            result.applied_damage,
            ARCANINE_BLAZING_MANE_ABILITY_HEAL_PERCENT,
        );
        if is_champion {
            champion_hits = champion_hits.saturating_add(1);
        }
    }

    if champion_hits > 0 {
        if let Some(caster) = ctx.get_entity(caster_id) {
            let missing_hp = caster.hp().max.saturating_sub(caster.hp().current);
            drop(caster);
            let heal = missing_hp
                .saturating_mul(heal_missing_hp_percent_per_champion)
                .saturating_mul(champion_hits)
                / 100;
            if heal > 0 {
                heal_with_antiheal(ctx, caster_id, caster_id, heal);
            }
        }
    }
    draw_field_circle(ctx, caster_pos, radius, VFX_FIRE);
}

fn arcanine_heal_from_ability_damage(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    applied_damage: usize,
    heal_percent: usize,
) {
    if applied_damage == 0 || heal_percent == 0 {
        return;
    }
    let heal = applied_damage.saturating_mul(heal_percent) / 100;
    if heal > 0 {
        heal_from_damage_or_poison_yanmega(ctx, caster_id, target_id, heal);
    }
}

fn has_blazing_mane(entity_id: usize) -> bool {
    entity_is_champion_id(entity_id, "pokemon_moba_arcanine")
        || receiver_has_copied(entity_id, "pokemon_moba_arcanine")
}

pub fn handle_arcanine_damaged(
    ctx: &mut GameCtx,
    arcanine_id: usize,
    attacker_id: usize,
    damage: usize,
) {
    if damage == 0 || !has_blazing_mane(arcanine_id) {
        return;
    }
    let Some(arcanine) = ctx.get_entity(arcanine_id) else {
        return;
    };
    if !arcanine.is_alive() {
        return;
    }
    let arcanine_team = arcanine.team();
    let arcanine_pos = arcanine.pos();
    let shield_after = arcanine.shield();
    drop(arcanine);

    let Some(attacker) = ctx.get_entity(attacker_id) else {
        return;
    };
    if !attacker.is_alive() || !attacker.is_champion() || attacker.team() == arcanine_team {
        return;
    }
    let attacker_pos = attacker.pos();
    drop(attacker);

    handle_arcanine_extremespeed_shield_hit(ctx, arcanine_id, shield_after);
    apply_arcanine_blazing_mane_stack(ctx, arcanine_id, attacker_id);

    if distance_sq(arcanine_pos, attacker_pos)
        <= ARCANINE_MELEE_CONTACT_RANGE.saturating_mul(ARCANINE_MELEE_CONTACT_RANGE)
        && chance_percent(
            ctx.seed(),
            arcanine_id,
            attacker_id,
            ctx.tick(),
            ARCANINE_BLAZING_MANE_MELEE_BURN_CHANCE,
        )
    {
        apply_burn_for(
            ctx,
            arcanine_id,
            attacker_id,
            ARCANINE_BLAZING_MANE_BURN_DAMAGE,
            ARCANINE_BLAZING_MANE_BURN_TICKS,
        );
    }
}

fn handle_arcanine_extremespeed_shield_hit(
    ctx: &mut GameCtx,
    entity_id: usize,
    shield_after: usize,
) {
    let tick = ctx.tick();
    let states = ARCANINE_EXTREMESPEED_SHIELDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("arcanine extremespeed shield state poisoned");
    let mut speed_buff = None;
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if state.entity_id == entity_id && shield_after == 0 && speed_buff.is_none() {
            speed_buff = Some((state.broken_move_speed_mult, state.broken_move_speed_ticks));
            return false;
        }
        true
    });
    drop(states);

    if let Some((move_speed_mult, ticks)) = speed_buff {
        add_beneficial_buff(
            ctx,
            entity_id,
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                move_speed_mult,
                ..Default::default()
            },
        );
    }
}

fn apply_arcanine_blazing_mane_stack(ctx: &mut GameCtx, arcanine_id: usize, attacker_id: usize) {
    let tick = ctx.tick();
    let expires_at = tick.saturating_add(ARCANINE_BLAZING_MANE_TICKS);
    let states = ARCANINE_BLAZING_MANES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("arcanine blazing mane state poisoned");
    states.retain(|state| {
        state.expires_at > tick
            && ctx
                .get_entity(state.arcanine_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
            && ctx
                .get_entity(state.attacker_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    let mut should_add_stack = false;
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.arcanine_id == arcanine_id && state.attacker_id == attacker_id)
    {
        if state.stacks < ARCANINE_BLAZING_MANE_MAX_STACKS {
            state.stacks = state.stacks.saturating_add(1);
            should_add_stack = true;
        }
        state.expires_at = expires_at;
    } else {
        states.push(ArcanineBlazingManeState {
            arcanine_id,
            attacker_id,
            stacks: 1,
            expires_at,
        });
        should_add_stack = true;
    }
    drop(states);

    if should_add_stack {
        add_harmful_buff(
            ctx,
            arcanine_id,
            attacker_id,
            BuffState {
                duration: BuffType::Time {
                    tick: ARCANINE_BLAZING_MANE_TICKS,
                },
                attack_mult: -ARCANINE_BLAZING_MANE_ATTACK_AP_REDUCE,
                magic_power_mult: -ARCANINE_BLAZING_MANE_ATTACK_AP_REDUCE,
                ..Default::default()
            },
        );
    }
}

fn wishiwashi_nearby_ally_count(ctx: &GameCtx, entity_id: usize, radius: u64) -> usize {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return 0;
    };
    if !entity.is_alive() {
        return 0;
    }
    let team = entity.team();
    let pos = entity.pos();
    drop(entity);

    let radius_sq = radius.saturating_mul(radius);
    (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|ally| {
            ally.id() != entity_id
                && ally.team() == team
                && ally.is_alive()
                && ally.is_champion()
                && distance_sq(ally.pos(), pos) <= radius_sq
        })
        .count()
}

pub fn force_wishiwashi_schooling(ctx: &GameCtx, entity_id: usize, ticks: usize) {
    let until = ctx.tick().saturating_add(ticks);
    let states = WISHIWASHI_SCHOOLINGS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("wishiwashi schooling state poisoned");
    states.retain(|state| {
        state.entity_id == entity_id
            || ctx
                .get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.force_school_until = state.force_school_until.max(until);
        return;
    }
    states.push(WishiwashiSchoolingState {
        entity_id,
        force_school_until: until,
    });
}

pub fn wishiwashi_is_schooling(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let forced = WISHIWASHI_SCHOOLINGS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("wishiwashi schooling state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.force_school_until > tick);
    forced || wishiwashi_nearby_ally_count(ctx, entity_id, WISHIWASHI_SCHOOLING_RADIUS) > 0
}

pub fn update_wishiwashi_schooling(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let nearby_allies = wishiwashi_nearby_ally_count(ctx, entity_id, WISHIWASHI_SCHOOLING_RADIUS)
        .min(WISHIWASHI_SCHOOLING_MAX_ALLIES);
    let schooling = wishiwashi_is_schooling(ctx, entity_id);
    if schooling {
        let ally_count = nearby_allies.max(1) as i32;
        let defence_bonus = WISHIWASHI_SCHOOLING_DEFENCE_PER_ALLY.saturating_mul(ally_count);
        let cooldown_bonus = WISHIWASHI_SCHOOLING_COOLDOWN_PER_ALLY.saturating_mul(ally_count);
        add_beneficial_buff(
            ctx,
            entity_id,
            entity_id,
            BuffState {
                duration: BuffType::Time {
                    tick: WISHIWASHI_SCHOOLING_BUFF_TICKS,
                },
                defence_mult: defence_bonus,
                magic_resistance_mult: defence_bonus,
                skill_cooldown_mult: cooldown_bonus,
                ult_cooldown_mult: cooldown_bonus,
                ..Default::default()
            },
        );
        draw_status_marker(ctx, pos, 18000, VFX_WATER);
    } else {
        add_beneficial_buff(
            ctx,
            entity_id,
            entity_id,
            BuffState {
                duration: BuffType::Time {
                    tick: WISHIWASHI_SCHOOLING_BUFF_TICKS,
                },
                hp_mult: WISHIWASHI_ALONE_HP_MULT,
                move_speed_mult: WISHIWASHI_ALONE_MOVE_SPEED_MULT,
                ..Default::default()
            },
        );
    }
}

pub fn apply_wishiwashi_wave_splash(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    wall_length: u64,
    damage: usize,
    heal: usize,
    schooling: bool,
    airborne_ticks: usize,
    attacker_types: TypeSet,
) {
    let half_wall = wall_length / 2;
    let width_sq = half_wall.saturating_mul(half_wall);
    let mut ally_hits = Vec::new();
    let mut enemy_hits = Vec::new();
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if !entity.is_alive() || entity.is_tower() {
            continue;
        }
        if distance_to_segment_sq(entity.pos(), start, end) > width_sq {
            continue;
        }
        if entity.team() == caster_team && entity.is_champion() {
            ally_hits.push(entity.id());
        } else if entity.team() != caster_team {
            enemy_hits.push(entity.id());
        }
    }

    for ally_id in ally_hits {
        heal_with_antiheal(ctx, caster_id, ally_id, heal);
        if schooling {
            cleanse_harmful_statuses(ctx, ally_id);
            add_beneficial_buff(
                ctx,
                caster_id,
                ally_id,
                BuffState {
                    duration: BuffType::Time { tick: 45 },
                    move_speed_mult: 10,
                    ..Default::default()
                },
            );
        }
    }

    for target_id in enemy_hits {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage.max(1),
            AttackType::Skill,
            PokemonType::Water,
            attacker_types,
            defender_types,
        );
        if schooling {
            apply_airborne_hard_cc(ctx, caster_id, target_id, airborne_ticks);
        }
    }
    draw_status_marker(ctx, end, half_wall.max(9000), VFX_WATER);
}

pub fn apply_wishiwashi_cowardice(
    ctx: &mut GameCtx,
    caster_id: usize,
    ally_radius: u64,
    heal_hp_percent: usize,
    shield_hp_percent: usize,
    shield_ticks: usize,
    solo_move_speed_mult: i32,
    solo_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);

    if !wishiwashi_is_schooling(ctx, caster_id) {
        add_beneficial_buff(
            ctx,
            caster_id,
            caster_id,
            BuffState {
                duration: BuffType::Time { tick: solo_ticks },
                move_speed_mult: solo_move_speed_mult,
                ..Default::default()
            },
        );
        draw_status_marker(ctx, caster_pos, 14000, VFX_NORMAL);
        return;
    }

    let radius_sq = ally_radius.saturating_mul(ally_radius);
    let allies: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() == caster_team
                && entity.is_alive()
                && entity.is_champion()
                && distance_sq(entity.pos(), caster_pos) <= radius_sq
        })
        .map(|entity| entity.id())
        .collect();

    for ally_id in allies {
        let Some(ally) = ctx.get_entity(ally_id) else {
            continue;
        };
        let max_hp = ally.hp().max;
        drop(ally);
        let heal = max_hp.saturating_mul(heal_hp_percent) / 100;
        let shield = max_hp.saturating_mul(shield_hp_percent) / 100;
        heal_with_antiheal(ctx, caster_id, ally_id, heal.max(1));
        add_beneficial_buff(
            ctx,
            caster_id,
            ally_id,
            BuffState {
                duration: BuffType::Time { tick: shield_ticks },
                hp: shield.min(i32::MAX as usize) as i32,
                ..Default::default()
            },
        );
    }
    draw_status_marker(ctx, caster_pos, ally_radius, VFX_WATER);
}

fn force_move_toward_pos(
    ctx: &mut GameCtx,
    entity_id: usize,
    from: EntityPos,
    to: EntityPos,
    speed: u64,
    ticks: usize,
) {
    let distance = integer_sqrt(distance_sq(from, to));
    let speed = bounded_force_move_speed(speed, ticks as u64, distance);
    if speed == 0 {
        return;
    }
    apply_pokemon_cc(
        ctx,
        entity_id,
        entity_id,
        CCState::ForceMove {
            tick: ticks as u64,
            dx: to.x as i64 - from.x as i64,
            dy: to.y as i64 - from.y as i64,
            speed,
        },
    );
}

fn wishiwashi_spit_pos(start: EntityPos, target: EntityPos, distance: u64) -> EntityPos {
    let dx = start.x as i128 - target.x as i128;
    let dy = start.y as i128 - target.y as i128;
    let len = integer_sqrt(squared_len_i128(dx, dy).min(u64::MAX as i128) as u64).max(1) as i128;
    let spit_x = start.x as i128 + dx.saturating_mul(distance as i128) / len;
    let spit_y = start.y as i128 + dy.saturating_mul(distance as i128) / len;
    EntityPos {
        x: spit_x.clamp(0, u64::MAX as i128) as u64,
        y: spit_y.clamp(0, u64::MAX as i128) as u64,
    }
}

pub fn begin_wishiwashi_massive_catch(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    dash_range: u64,
    schooling_dash_bonus: u64,
    line_width: u64,
    damage: usize,
    chew_damage: usize,
    channel_ticks: usize,
    outbound_ticks: usize,
    return_ticks: usize,
    chew_interval_ticks: usize,
    throw_ticks: usize,
    force_move_speed: u64,
    schooling_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    let start_pos = caster.pos();
    drop(caster);

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() || target.team() == caster_team || !target.is_champion() {
        return;
    }
    let target_pos = target.pos();
    drop(target);

    force_wishiwashi_schooling(ctx, caster_id, schooling_ticks);
    let range = dash_range.saturating_add(if wishiwashi_is_schooling(ctx, caster_id) {
        schooling_dash_bonus
    } else {
        0
    });
    let distance = integer_sqrt(distance_sq(start_pos, target_pos));
    if distance > range {
        return;
    }

    let spit_pos = wishiwashi_spit_pos(start_pos, target_pos, 26000);
    let tick = ctx.tick();
    let states = WISHIWASHI_MASSIVE_CATCHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("wishiwashi massive catch state poisoned");
    states.retain(|state| state.caster_id != caster_id && state.target_id != target_id);
    states.push(WishiwashiMassiveCatchState {
        caster_id,
        target_id,
        caster_team,
        start_pos,
        target_pos,
        spit_pos,
        trigger_at: tick.saturating_add(channel_ticks),
        catch_at: tick
            .saturating_add(channel_ticks)
            .saturating_add(outbound_ticks),
        spit_at: tick
            .saturating_add(channel_ticks)
            .saturating_add(outbound_ticks)
            .saturating_add(return_ticks),
        next_chew_at: tick
            .saturating_add(channel_ticks)
            .saturating_add(outbound_ticks)
            .saturating_add(chew_interval_ticks),
        line_width,
        damage,
        chew_damage,
        outbound_ticks,
        return_ticks,
        chew_interval_ticks,
        throw_ticks,
        force_move_speed,
        attacker_types,
        outbound_started: false,
        caught: false,
    });
    drop(states);

    apply_pokemon_cc(
        ctx,
        caster_id,
        caster_id,
        CCState::BlockSkill {
            tick: channel_ticks
                .saturating_add(outbound_ticks)
                .saturating_add(return_ticks),
        },
    );
    draw_status_marker(ctx, start_pos, 22000, VFX_WATER);
}

fn update_wishiwashi_massive_catches(ctx: &mut GameCtx, tick: usize) {
    let states = WISHIWASHI_MASSIVE_CATCHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("wishiwashi massive catch state poisoned");

    let mut outbound = Vec::new();
    let mut caught = Vec::new();
    let mut chewing = Vec::new();
    let mut tethering = Vec::new();
    let mut spitting = Vec::new();

    states.retain_mut(|state| {
        let caster_alive = ctx
            .get_entity(state.caster_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        let target_alive = ctx
            .get_entity(state.target_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        if !caster_alive || !target_alive {
            return false;
        }

        if !state.outbound_started && tick >= state.trigger_at {
            state.outbound_started = true;
            outbound.push(*state);
        }

        if state.outbound_started && !state.caught && tick >= state.catch_at {
            state.caught = true;
            caught.push(*state);
        }

        if state.caught && tick < state.spit_at {
            tethering.push(*state);
            if tick >= state.next_chew_at {
                chewing.push(*state);
                state.next_chew_at = state
                    .next_chew_at
                    .saturating_add(state.chew_interval_ticks.max(1));
            }
        }

        if state.caught && tick >= state.spit_at {
            spitting.push(*state);
            return false;
        }

        true
    });
    drop(states);

    for state in outbound {
        force_move_toward_pos(
            ctx,
            state.caster_id,
            state.start_pos,
            state.target_pos,
            state.force_move_speed,
            state.outbound_ticks,
        );
        let width_sq = state.line_width.saturating_mul(state.line_width);
        let pass_through_targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.is_alive()
                    && !entity.is_tower()
                    && !entity.is_champion()
                    && entity.team() != state.caster_team
                    && distance_to_segment_sq(entity.pos(), state.start_pos, state.target_pos)
                        <= width_sq
            })
            .map(|entity| entity.id())
            .collect();

        for target_id in pass_through_targets {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                state.caster_id,
                target_id,
                0,
                state.damage.max(1),
                AttackType::Skill,
                PokemonType::Water,
                state.attacker_types,
                defender_types,
            );
        }
        draw_status_marker(
            ctx,
            state.target_pos,
            state.line_width.max(12000),
            VFX_WATER,
        );
    }

    for state in caught {
        let target_ticks = state.return_ticks.saturating_add(state.throw_ticks).max(1);
        if !is_limber(ctx, state.target_id) {
            let adjusted = adjusted_cc_ticks(ctx, state.target_id, target_ticks);
            break_kommoo_duel_on_hard_cc(ctx, state.caster_id, state.target_id);
            apply_pokemon_cc(
                ctx,
                state.caster_id,
                state.target_id,
                CCState::Stun {
                    tick: adjusted as u64,
                },
            );
            apply_pokemon_cc(
                ctx,
                state.caster_id,
                state.target_id,
                CCState::BlockSkill { tick: adjusted },
            );
        }
        force_move_toward_pos(
            ctx,
            state.caster_id,
            state.target_pos,
            state.start_pos,
            state.force_move_speed,
            state.return_ticks,
        );
        if !has_stalwart(ctx, state.target_id) {
            force_move_toward_pos(
                ctx,
                state.target_id,
                state.target_pos,
                state.start_pos,
                state.force_move_speed,
                state.return_ticks,
            );
        }
    }

    for state in tethering {
        if has_stalwart(ctx, state.target_id) {
            continue;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            continue;
        };
        let caster_pos = caster.pos();
        drop(caster);
        let Some(target) = ctx.get_entity(state.target_id) else {
            continue;
        };
        let target_pos = target.pos();
        drop(target);
        force_move_toward_pos(
            ctx,
            state.target_id,
            target_pos,
            caster_pos,
            state.force_move_speed,
            4,
        );
    }

    for state in chewing {
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, state.target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            state.caster_id,
            state.target_id,
            0,
            state.chew_damage.max(1),
            AttackType::Skill,
            PokemonType::Water,
            state.attacker_types,
            defender_types,
        );
    }

    for state in spitting {
        if !has_stalwart(ctx, state.target_id) {
            let Some(target) = ctx.get_entity(state.target_id) else {
                continue;
            };
            let target_pos = target.pos();
            drop(target);
            force_move_toward_pos(
                ctx,
                state.target_id,
                target_pos,
                state.spit_pos,
                state.force_move_speed,
                state.throw_ticks,
            );
        }
        draw_status_marker(ctx, state.spit_pos, 16000, VFX_WATER);
    }
}

pub fn apply_missingno_glitch_basic(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    damage: usize,
    stray_damage: usize,
    stray_chance_percent: usize,
    stray_range: u64,
    stray_width: u64,
    attacker_types: TypeSet,
) {
    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
    crate::pokemon_types::deal_pokemon_damage(
        ctx,
        caster_id,
        target_id,
        0,
        damage.max(1),
        AttackType::BaseAttack,
        PokemonType::Bird,
        attacker_types,
        defender_types,
    );

    let seed = ctx.seed()
        ^ ((caster_id as u64) << 36)
        ^ ((target_id as u64) << 12)
        ^ ctx.tick() as u64
        ^ 0x4d495353494e47_u64;
    if !chance_percent(seed, caster_id, target_id, ctx.tick(), stray_chance_percent) {
        return;
    }

    let Some(caster_pos) = ctx.get_entity(caster_id).map(|entity| entity.pos()) else {
        return;
    };
    let angle_seed = splitmix64(seed ^ 0x6e6f_u64);
    let angle = (angle_seed % 6284) as f64 / 1000.0;
    let end = pos_from_f64(
        caster_pos.x as f64 + angle.cos() * stray_range as f64,
        caster_pos.y as f64 + angle.sin() * stray_range as f64,
    );
    draw_line_band(ctx, caster_pos, end, stray_width, VFX_PSYCHIC);
    let Some(stray_target) =
        first_target_on_line_any_team(ctx, caster_id, caster_pos, end, stray_width)
    else {
        return;
    };
    deal_tracked_damage(
        ctx,
        caster_id,
        stray_target,
        0,
        stray_damage.max(1),
        AttackType::Skill,
    );
}

pub fn apply_missingno_random_status(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    dot_damage: usize,
    dot_duration_ticks: usize,
    control_ticks: usize,
    move_speed_mult: i32,
    cooldown_mult: i32,
    buff_ticks: usize,
) {
    let seed = ctx.seed()
        ^ ((caster_id as u64) << 40)
        ^ ((target_id as u64) << 16)
        ^ ctx.tick() as u64
        ^ 0x00b1_7d00_u64;
    let roll = (splitmix64(seed) % 6) as usize;
    let mut damaging_status = false;
    match roll {
        0 => {
            apply_burn_for(
                ctx,
                caster_id,
                target_id,
                dot_damage.max(1),
                dot_duration_ticks,
            );
            damaging_status = true;
        }
        1 => {
            apply_poison_for(
                ctx,
                caster_id,
                target_id,
                dot_damage.max(1),
                dot_duration_ticks,
            );
            damaging_status = true;
        }
        2 => {
            apply_bleed_for(
                ctx,
                caster_id,
                target_id,
                dot_damage.max(1),
                dot_duration_ticks,
            );
            damaging_status = true;
        }
        3 => apply_frozen_from(ctx, caster_id, target_id, control_ticks),
        4 => apply_paralysis_from(ctx, caster_id, target_id, control_ticks),
        _ => apply_confusion_from(ctx, caster_id, target_id, 1, control_ticks),
    }
    if !damaging_status {
        add_beneficial_buff(
            ctx,
            caster_id,
            caster_id,
            BuffState {
                duration: BuffType::Time { tick: buff_ticks },
                move_speed_mult,
                skill_cooldown_mult: cooldown_mult,
                ult_cooldown_mult: cooldown_mult,
                ..Default::default()
            },
        );
    }
    maybe_trigger_missingno_positive_glitch(ctx, caster_id, target_id);
}

pub fn begin_missingno_glitch_storm(
    ctx: &mut GameCtx,
    caster_id: usize,
    radius: u64,
    chain_radius: u64,
    damage: usize,
    chain_jumps: usize,
    duration_ticks: usize,
    tick_interval_min: usize,
    tick_interval_max: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);

    let seed = ctx.seed() ^ ((caster_id as u64) << 32) ^ ctx.tick() as u64 ^ 0x4d5f3030_u64;
    let states = MISSINGNO_GLITCH_STORMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("missingno storm state poisoned");
    states.retain(|state| state.caster_id != caster_id);
    states.push(MissingNoGlitchStormState {
        caster_id,
        caster_team,
        radius,
        chain_radius,
        damage,
        chain_jumps,
        tick_interval_min: tick_interval_min.max(1),
        tick_interval_max: tick_interval_max.max(tick_interval_min.max(1)),
        next_tick_at: ctx.tick().saturating_add(tick_interval_min.max(1)),
        expires_at: ctx.tick().saturating_add(duration_ticks),
        attacker_types,
        seed,
    });
    draw_field_circle(ctx, caster_pos, radius, VFX_PSYCHIC);
}

pub fn begin_missingno_trick_room(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_pos: EntityPos,
    length: u64,
    width: u64,
    duration_ticks: usize,
    enemy_speed_slow: i32,
    enemy_attack_speed_slow: i32,
    enemy_hp_random_min: i32,
    enemy_hp_random_max: i32,
    enemy_cooldown_random_min: i32,
    enemy_cooldown_random_max: i32,
    missingno_speed_mult: i32,
    missingno_attack_speed_mult: i32,
    missingno_cooldown_mult: i32,
    ally_speed_mult: i32,
    ally_attack_speed_mult: i32,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);

    let end = point_at_distance(caster_pos, target_pos, length);
    let states = MISSINGNO_TRICK_ROOMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("missingno trick room state poisoned");
    states.retain(|state| state.caster_id != caster_id);
    states.push(MissingNoTrickRoomState {
        caster_id,
        caster_team,
        start: caster_pos,
        end,
        length,
        width,
        expires_at: ctx.tick().saturating_add(duration_ticks),
        next_tick_at: ctx.tick(),
        tick_interval: 30,
        enemy_speed_slow,
        enemy_attack_speed_slow,
        enemy_hp_random_min,
        enemy_hp_random_max,
        enemy_cooldown_random_min,
        enemy_cooldown_random_max,
        missingno_speed_mult,
        missingno_attack_speed_mult,
        missingno_cooldown_mult,
        ally_speed_mult,
        ally_attack_speed_mult,
        buff_ticks: 40,
        seed: ctx.seed() ^ ((caster_id as u64) << 28) ^ ctx.tick() as u64 ^ 0x747269636b_u64,
    });
    draw_line_band(ctx, caster_pos, end, width, VFX_PSYCHIC);
}

fn update_missingno_glitch_storms(ctx: &mut GameCtx, tick: usize) {
    let states = MISSINGNO_GLITCH_STORMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("missingno storm state poisoned");
    let mut sparks = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            return false;
        }
        let caster_pos = caster.pos();
        visuals.push((caster_pos, state.radius));
        while state.next_tick_at <= tick {
            sparks.push((*state, caster_pos));
            let span = state
                .tick_interval_max
                .saturating_sub(state.tick_interval_min)
                .saturating_add(1);
            let seed = splitmix64(state.seed ^ ((state.next_tick_at as u64) << 8));
            let offset = (seed as usize) % span.max(1);
            state.next_tick_at = state
                .next_tick_at
                .saturating_add(state.tick_interval_min.saturating_add(offset));
        }
        true
    });
    drop(states);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_PSYCHIC);
    }

    for (state, caster_pos) in sparks {
        let seed = splitmix64(state.seed ^ tick as u64 ^ 0x737061726b_u64);
        let Some(first_target) =
            random_enemy_in_radius(ctx, state.caster_team, caster_pos, state.radius, seed)
        else {
            continue;
        };
        missingno_chain_damage(
            ctx,
            state.caster_id,
            state.caster_team,
            first_target,
            state.damage,
            state.chain_radius,
            state.chain_jumps,
            state.attacker_types,
            seed,
        );
    }
}

fn update_missingno_trick_rooms(ctx: &mut GameCtx, tick: usize) {
    let states = MISSINGNO_TRICK_ROOMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("missingno trick room state poisoned");
    let mut applications = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        let caster_alive = ctx
            .get_entity(state.caster_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        if !caster_alive || state.expires_at <= tick {
            return false;
        }
        visuals.push((state.start, state.end, state.width));
        if state.next_tick_at <= tick {
            applications.push(*state);
            state.next_tick_at = state
                .next_tick_at
                .saturating_add(state.tick_interval.max(1));
        }
        true
    });
    drop(states);

    for (start, end, width) in visuals {
        draw_line_band(ctx, start, end, width, VFX_PSYCHIC);
    }

    for state in applications {
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || !entity.is_champion()
                || !point_in_oriented_rect(
                    entity.pos(),
                    state.start,
                    state.end,
                    state.length,
                    state.width,
                )
            {
                continue;
            }
            let entity_id = entity.id();
            let team = entity.team();
            let stat = entity.stat();
            drop(entity);

            if entity_id == state.caster_id {
                add_beneficial_buff(
                    ctx,
                    state.caster_id,
                    entity_id,
                    BuffState {
                        duration: BuffType::Time {
                            tick: state.buff_ticks,
                        },
                        move_speed_mult: state.missingno_speed_mult,
                        attack_speed_mult: state.missingno_attack_speed_mult,
                        skill_cooldown_mult: state.missingno_cooldown_mult,
                        ult_cooldown_mult: state.missingno_cooldown_mult,
                        ..Default::default()
                    },
                );
            } else if team == state.caster_team {
                add_beneficial_buff(
                    ctx,
                    state.caster_id,
                    entity_id,
                    BuffState {
                        duration: BuffType::Time {
                            tick: state.buff_ticks,
                        },
                        move_speed_mult: state.ally_speed_mult,
                        attack_speed_mult: state.ally_attack_speed_mult,
                        ..Default::default()
                    },
                );
            } else {
                let random_hp = random_range_i32(
                    state.seed ^ ((entity_id as u64) << 18) ^ tick as u64,
                    state.enemy_hp_random_min,
                    state.enemy_hp_random_max,
                );
                let random_cooldown = random_range_i32(
                    state.seed ^ ((entity_id as u64) << 24) ^ tick as u64 ^ 0xcd_u64,
                    state.enemy_cooldown_random_min,
                    state.enemy_cooldown_random_max,
                );
                let attack_flip = percent_delta_for_swap(stat.attack, stat.magic_power);
                let ap_flip = percent_delta_for_swap(stat.magic_power, stat.attack);
                let defence_flip = percent_delta_for_swap(stat.defence, stat.magic_resistance);
                let mr_flip = percent_delta_for_swap(stat.magic_resistance, stat.defence);
                add_harmful_buff(
                    ctx,
                    state.caster_id,
                    entity_id,
                    BuffState {
                        duration: BuffType::Time {
                            tick: state.buff_ticks,
                        },
                        move_speed_mult: -state.enemy_speed_slow.abs(),
                        attack_speed_mult: -state.enemy_attack_speed_slow.abs(),
                        attack_mult: attack_flip,
                        magic_power_mult: ap_flip,
                        defence_mult: defence_flip,
                        magic_resistance_mult: mr_flip,
                        hp_mult: random_hp,
                        skill_cooldown_mult: random_cooldown,
                        ult_cooldown_mult: random_cooldown,
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn maybe_trigger_missingno_positive_glitch(ctx: &mut GameCtx, caster_id: usize, target_id: usize) {
    if !entity_is_champion_id(caster_id, "pokemon_moba_missingno") {
        return;
    }
    let seed = ctx.seed()
        ^ ((caster_id as u64) << 34)
        ^ ((target_id as u64) << 13)
        ^ ctx.tick() as u64
        ^ 0x6f6c646d616e_u64;
    if !chance_percent(
        seed,
        caster_id,
        target_id,
        ctx.tick(),
        MISSINGNO_PASSIVE_CHANCE_PERCENT,
    ) {
        return;
    }
    add_beneficial_buff(
        ctx,
        caster_id,
        caster_id,
        BuffState {
            duration: BuffType::Time {
                tick: MISSINGNO_PASSIVE_BUFF_TICKS,
            },
            move_speed_mult: MISSINGNO_PASSIVE_MOVE_SPEED_MULT,
            skill_cooldown_mult: MISSINGNO_PASSIVE_COOLDOWN_MULT,
            ult_cooldown_mult: MISSINGNO_PASSIVE_COOLDOWN_MULT,
            ..Default::default()
        },
    );
}

fn maybe_trigger_missingno_negative_glitch(
    ctx: &GameCtx,
    attacker_id: usize,
    missingno_id: usize,
    attack_type: AttackType,
) {
    if matches!(attack_type, AttackType::BaseAttack | AttackType::Dot)
        || !entity_is_champion_id(missingno_id, "pokemon_moba_missingno")
    {
        return;
    }
    let seed = ctx.seed()
        ^ ((attacker_id as u64) << 31)
        ^ ((missingno_id as u64) << 9)
        ^ ctx.tick() as u64
        ^ 0x676c69746368_u64;
    if !chance_percent(
        seed,
        missingno_id,
        attacker_id,
        ctx.tick(),
        MISSINGNO_PASSIVE_CHANCE_PERCENT,
    ) {
        return;
    }
    let states = MISSINGNO_PENDING_DEBUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("missingno pending debuff state poisoned");
    states.push(MissingNoPendingDebuffState {
        source_id: missingno_id,
        target_id: attacker_id,
        trigger_at: ctx.tick().saturating_add(1),
    });
}

fn update_missingno_pending_debuffs(ctx: &mut GameCtx, tick: usize) {
    let states = MISSINGNO_PENDING_DEBUFFS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("missingno pending debuff state poisoned");
    let mut pending = Vec::new();
    states.retain(|state| {
        if state.trigger_at > tick {
            return true;
        }
        pending.push(*state);
        false
    });
    drop(states);

    for state in pending {
        add_harmful_buff(
            ctx,
            state.source_id,
            state.target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: MISSINGNO_PASSIVE_BUFF_TICKS,
                },
                move_speed_mult: -MISSINGNO_PASSIVE_MOVE_SPEED_MULT,
                skill_cooldown_mult: -MISSINGNO_PASSIVE_COOLDOWN_MULT,
                ult_cooldown_mult: -MISSINGNO_PASSIVE_COOLDOWN_MULT,
                ..Default::default()
            },
        );
    }
}

fn missingno_chain_damage(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    first_target_id: usize,
    damage: usize,
    chain_radius: u64,
    chain_jumps: usize,
    attacker_types: TypeSet,
    seed: u64,
) {
    let mut current_id = first_target_id;
    let mut previous_ids = vec![caster_id];
    for jump in 0..=chain_jumps {
        if previous_ids.iter().any(|id| *id == current_id) {
            break;
        }
        previous_ids.push(current_id);
        let Some(target) = ctx.get_entity(current_id) else {
            break;
        };
        if !target.is_alive() {
            break;
        }
        let target_team = target.team();
        let target_pos = target.pos();
        drop(target);
        if target_team == caster_team {
            deal_tracked_damage(
                ctx,
                caster_id,
                current_id,
                0,
                damage.max(1),
                AttackType::Skill,
            );
        } else {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, current_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                caster_id,
                current_id,
                0,
                damage.max(1),
                AttackType::Skill,
                PokemonType::Bird,
                attacker_types,
                defender_types,
            );
            maybe_trigger_missingno_positive_glitch(ctx, caster_id, current_id);
        }
        draw_status_marker(ctx, target_pos, 9000, VFX_PSYCHIC);

        let prefer_enemy = jump % 2 == 1;
        let Some(next_id) = nearest_chain_target(
            ctx,
            caster_team,
            target_pos,
            chain_radius,
            &previous_ids,
            prefer_enemy,
            seed ^ ((jump as u64) << 10),
        ) else {
            break;
        };
        current_id = next_id;
    }
}

fn nearest_chain_target(
    ctx: &GameCtx,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    excluded: &[usize],
    prefer_enemy: bool,
    seed: u64,
) -> Option<usize> {
    let radius_sq = radius.saturating_mul(radius);
    let mut best_preferred: Option<(u64, u64, usize)> = None;
    let mut best_any: Option<(u64, u64, usize)> = None;
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if !entity.is_alive() || entity.is_tower() || excluded.iter().any(|id| *id == entity.id()) {
            continue;
        }
        let distance = distance_sq(entity.pos(), center);
        if distance > radius_sq {
            continue;
        }
        let jitter = splitmix64(seed ^ entity.id() as u64) % 1024;
        let candidate = (distance, jitter, entity.id());
        if best_any
            .map(|best| (distance, jitter) < (best.0, best.1))
            .unwrap_or(true)
        {
            best_any = Some(candidate);
        }
        let preferred = if prefer_enemy {
            entity.team() != caster_team
        } else {
            entity.team() == caster_team
        };
        if preferred
            && best_preferred
                .map(|best| (distance, jitter) < (best.0, best.1))
                .unwrap_or(true)
        {
            best_preferred = Some(candidate);
        }
    }
    best_preferred
        .or(best_any)
        .map(|(_, _, entity_id)| entity_id)
}

fn random_enemy_in_radius(
    ctx: &GameCtx,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    seed: u64,
) -> Option<usize> {
    let mut candidates = Vec::new();
    let radius_sq = radius.saturating_mul(radius);
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.team() == caster_team
            || !entity.is_alive()
            || !entity.is_targetable()
            || distance_sq(entity.pos(), center) > radius_sq
        {
            continue;
        }
        candidates.push(entity.id());
    }
    if candidates.is_empty() {
        None
    } else {
        Some(candidates[(seed as usize) % candidates.len()])
    }
}

fn first_target_on_line_any_team(
    ctx: &GameCtx,
    caster_id: usize,
    start: EntityPos,
    end: EntityPos,
    width: u64,
) -> Option<usize> {
    let ax = start.x as i128;
    let ay = start.y as i128;
    let bx = end.x as i128;
    let by = end.y as i128;
    let abx = bx - ax;
    let aby = by - ay;
    let ab_len_sq = abx * abx + aby * aby;
    if ab_len_sq == 0 {
        return None;
    }
    let mut best: Option<(i128, usize)> = None;
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.id() == caster_id || !entity.is_alive() || !entity.is_targetable() {
            continue;
        }
        let pos = entity.pos();
        if distance_to_segment_sq(pos, start, end) > width.saturating_mul(width) {
            continue;
        }
        let projection = (pos.x as i128 - ax) * abx + (pos.y as i128 - ay) * aby;
        if projection < 0 || projection > ab_len_sq {
            continue;
        }
        if best
            .map(|(best_projection, _)| projection < best_projection)
            .unwrap_or(true)
        {
            best = Some((projection, entity.id()));
        }
    }
    best.map(|(_, entity_id)| entity_id)
}

fn point_at_distance(start: EntityPos, target: EntityPos, distance: u64) -> EntityPos {
    let dx = target.x as f64 - start.x as f64;
    let dy = target.y as f64 - start.y as f64;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0 {
        return EntityPos {
            x: start.x.saturating_add(distance),
            y: start.y,
        };
    }
    let unit_x = dx / len;
    let unit_y = dy / len;
    pos_from_f64(
        start.x as f64 + unit_x * distance as f64,
        start.y as f64 + unit_y * distance as f64,
    )
}

fn point_in_oriented_rect(
    pos: EntityPos,
    start: EntityPos,
    end: EntityPos,
    length: u64,
    half_width: u64,
) -> bool {
    let ax = start.x as i128;
    let ay = start.y as i128;
    let bx = end.x as i128;
    let by = end.y as i128;
    let px = pos.x as i128;
    let py = pos.y as i128;
    let abx = bx - ax;
    let aby = by - ay;
    let apx = px - ax;
    let apy = py - ay;
    let ab_len_sq = squared_len_i128(abx, aby);
    if ab_len_sq <= 0 {
        return distance_sq(pos, start) <= half_width.saturating_mul(half_width);
    }
    let projection = apx * abx + apy * aby;
    if projection < 0 {
        return false;
    }
    let max_projection = (length as i128)
        .saturating_mul(integer_sqrt(ab_len_sq.min(u64::MAX as i128) as u64) as i128);
    if projection > max_projection.max(ab_len_sq) {
        return false;
    }
    distance_to_segment_sq(pos, start, end) <= half_width.saturating_mul(half_width)
}

fn random_range_i32(seed: u64, min_value: i32, max_value: i32) -> i32 {
    let low = min_value.min(max_value);
    let high = min_value.max(max_value);
    let span = high.saturating_sub(low).saturating_add(1).max(1) as u32;
    low.saturating_add((splitmix64(seed) as u32 % span) as i32)
}

fn percent_delta_for_swap(current: usize, desired: usize) -> i32 {
    if current == 0 {
        return if desired == 0 { 0 } else { 75 };
    }
    let delta = desired as i128 - current as i128;
    ((delta * 100) / current as i128).clamp(-75, 75) as i32
}

fn trick_room_inverts_allied_effect(
    ctx: &GameCtx,
    source_id: usize,
    target_id: usize,
) -> Option<usize> {
    let source_team = ctx.get_entity(source_id).map(|entity| entity.team())?;
    let target = ctx.get_entity(target_id)?;
    if source_team != target.team() {
        return None;
    }
    let target_pos = target.pos();
    let target_team = target.team();
    drop(target);
    let tick = ctx.tick();
    let states = MISSINGNO_TRICK_ROOMS.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("missingno trick room state poisoned")
        .iter()
        .find(|state| {
            state.expires_at > tick
                && target_team != state.caster_team
                && point_in_oriented_rect(
                    target_pos,
                    state.start,
                    state.end,
                    state.length,
                    state.width,
                )
        })
        .map(|state| state.caster_id)
}

fn trick_room_doubles_missingno_buffs(ctx: &GameCtx, target_id: usize) -> bool {
    if !entity_is_champion_id(target_id, "pokemon_moba_missingno") {
        return false;
    }
    let Some(target) = ctx.get_entity(target_id) else {
        return false;
    };
    let target_pos = target.pos();
    drop(target);
    let tick = ctx.tick();
    let states = MISSINGNO_TRICK_ROOMS.get_or_init(|| Mutex::new(Vec::new()));
    states
        .lock()
        .expect("missingno trick room state poisoned")
        .iter()
        .any(|state| {
            state.expires_at > tick
                && state.caster_id == target_id
                && point_in_oriented_rect(
                    target_pos,
                    state.start,
                    state.end,
                    state.length,
                    state.width,
                )
        })
}

fn invert_buff_state(buff: &mut BuffState) {
    buff.attack_mult = -buff.attack_mult;
    buff.magic_power_mult = -buff.magic_power_mult;
    buff.attack_speed_mult = -buff.attack_speed_mult;
    buff.defence_mult = -buff.defence_mult;
    buff.magic_resistance_mult = -buff.magic_resistance_mult;
    buff.hp_mult = -buff.hp_mult;
    buff.move_speed_mult = -buff.move_speed_mult;
    buff.skill_cooldown_mult = -buff.skill_cooldown_mult;
    buff.ult_cooldown_mult = -buff.ult_cooldown_mult;
}

fn double_positive_buff_state(buff: &mut BuffState) {
    if buff.attack_mult > 0 {
        buff.attack_mult = buff.attack_mult.saturating_mul(2);
    }
    if buff.magic_power_mult > 0 {
        buff.magic_power_mult = buff.magic_power_mult.saturating_mul(2);
    }
    if buff.attack_speed_mult > 0 {
        buff.attack_speed_mult = buff.attack_speed_mult.saturating_mul(2);
    }
    if buff.defence_mult > 0 {
        buff.defence_mult = buff.defence_mult.saturating_mul(2);
    }
    if buff.magic_resistance_mult > 0 {
        buff.magic_resistance_mult = buff.magic_resistance_mult.saturating_mul(2);
    }
    if buff.hp_mult > 0 {
        buff.hp_mult = buff.hp_mult.saturating_mul(2);
    }
    if buff.move_speed_mult > 0 {
        buff.move_speed_mult = buff.move_speed_mult.saturating_mul(2);
    }
    if buff.skill_cooldown_mult > 0 {
        buff.skill_cooldown_mult = buff.skill_cooldown_mult.saturating_mul(2);
    }
    if buff.ult_cooldown_mult > 0 {
        buff.ult_cooldown_mult = buff.ult_cooldown_mult.saturating_mul(2);
    }
}

fn shiftry_inferred_in_bush(ctx: &GameCtx, entity_id: usize) -> bool {
    if ampharos_true_sight_reveals(ctx, entity_id) {
        return false;
    }
    let Some(shiftry) = ctx.get_entity(entity_id) else {
        return false;
    };
    if !shiftry.is_alive() {
        return false;
    }
    let shiftry_team = shiftry.team();
    let shiftry_pos = shiftry.pos();
    let radius_sq = SHIFTRY_BUSH_PROBE_RADIUS.saturating_mul(SHIFTRY_BUSH_PROBE_RADIUS);
    drop(shiftry);

    for index in 0..ctx.entity_count() {
        let Some(observer) = ctx.entity_at(index) else {
            continue;
        };
        if !observer.is_alive() || observer.team() == shiftry_team {
            continue;
        }
        if !(observer.is_champion() || observer.is_minion() || observer.is_tower()) {
            continue;
        }
        let observer_team = observer.team();
        if distance_sq(observer.pos(), shiftry_pos) > radius_sq {
            continue;
        }
        if !ctx.is_visible(observer_team, entity_id) {
            return true;
        }
    }

    false
}

pub fn update_shiftry_forest_camouflage(ctx: &mut GameCtx, entity_id: usize) {
    let tick = ctx.tick();
    let in_bush = shiftry_inferred_in_bush(ctx, entity_id);
    let states = SHIFTRY_FOREST_CAMOUFLAGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("shiftry forest camouflage state poisoned");
    let state = if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id)
    {
        existing
    } else {
        states.push(ShiftryForestCamouflageState {
            entity_id,
            entered_bush_at: tick,
            in_bush: false,
            linger_until: 0,
            last_buff_at: 0,
        });
        states.last_mut().expect("inserted shiftry forest state")
    };

    if in_bush {
        if !state.in_bush {
            state.entered_bush_at = tick;
            state.in_bush = true;
        }
        if tick.saturating_sub(state.entered_bush_at) >= SHIFTRY_BUSH_REQUIRED_TICKS {
            state.linger_until = tick.saturating_add(SHIFTRY_BUSH_LINGER_TICKS);
        }
    } else if state.in_bush {
        if tick.saturating_sub(state.entered_bush_at) >= SHIFTRY_BUSH_REQUIRED_TICKS {
            state.linger_until = tick.saturating_add(SHIFTRY_BUSH_LINGER_TICKS);
        }
        state.in_bush = false;
    }

    let active = in_bush || tick <= state.linger_until;
    let should_refresh = active
        && (state.last_buff_at == 0
            || tick.saturating_sub(state.last_buff_at) >= SHIFTRY_BUSH_BUFF_REFRESH_TICKS);
    if should_refresh {
        state.last_buff_at = tick;
    }
    drop(states);

    if should_refresh {
        add_beneficial_buff(
            ctx,
            entity_id,
            entity_id,
            BuffState {
                duration: BuffType::Time {
                    tick: SHIFTRY_BUSH_BUFF_TICKS,
                },
                move_speed_mult: 15,
                ..Default::default()
            },
        );
    }
}

pub fn note_rillaboom_skill_cast(ctx: &mut GameCtx, entity_id: usize) {
    let has_drum_solo = champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_rillaboom")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_rillaboom");
    if !has_drum_solo {
        return;
    }
    const WINDOW_TICKS: usize = 5 * 60;
    const HEAL_AMOUNT: usize = 45;
    const ALLY_RADIUS: u64 = 42000;
    const ALLY_SPEED_MULT: i32 = 12;
    const ALLY_SPEED_TICKS: usize = 2 * 60;

    let tick = ctx.tick();
    let solos = RILLABOOM_DRUM_SOLOS.get_or_init(|| Mutex::new(Vec::new()));
    let mut solos = solos.lock().expect("rillaboom drum solo state poisoned");
    let Some(state) = solos.iter_mut().find(|state| state.entity_id == entity_id) else {
        solos.push(RillaboomDrumSoloState {
            entity_id,
            window_start: tick,
            casts: 1,
        });
        return;
    };
    if tick.saturating_sub(state.window_start) > WINDOW_TICKS {
        state.window_start = tick;
        state.casts = 0;
    }
    state.casts = state.casts.saturating_add(1);
    if state.casts < 2 {
        return;
    }
    state.window_start = tick;
    state.casts = 0;
    drop(solos);

    let Some(caster) = ctx.get_entity(entity_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let team = caster.team();
    let pos = caster.pos();
    drop(caster);

    let _ = heal_with_antiheal(ctx, entity_id, entity_id, HEAL_AMOUNT);
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if !entity.is_alive()
            || !entity.is_champion()
            || entity.team() != team
            || entity.id() == entity_id
            || distance_sq(entity.pos(), pos) > ALLY_RADIUS.saturating_mul(ALLY_RADIUS)
        {
            continue;
        }
        add_beneficial_buff(
            ctx,
            entity_id,
            entity.id(),
            BuffState {
                duration: BuffType::Time {
                    tick: ALLY_SPEED_TICKS,
                },
                move_speed_mult: ALLY_SPEED_MULT,
                ..Default::default()
            },
        );
    }
    draw_field_circle(ctx, pos, ALLY_RADIUS, VFX_GRASS);
}

pub fn clear_grassy_terrains_at(center: EntityPos, radius: u64) {
    let fields = GRASSY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("grassy terrain state poisoned");
    let radius_sq = radius.saturating_mul(radius);
    fields.retain(|state| distance_sq(state.center, center) > radius_sq);
}

pub fn grassy_terrain_damage_bonus_for_caster(
    ctx: &GameCtx,
    caster_id: usize,
    move_type: PokemonType,
) -> usize {
    if !matches!(move_type, PokemonType::Grass) {
        return 0;
    }
    let Some(caster_pos) = ctx.get_entity(caster_id).map(|caster| caster.pos()) else {
        return 0;
    };
    GRASSY_TERRAINS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("grassy terrain state poisoned")
        .iter()
        .filter(|state| {
            distance_sq(caster_pos, state.center) <= state.radius.saturating_mul(state.radius)
        })
        .map(|state| state.damage_bonus_percent)
        .sum()
}

pub fn grassy_terrain_trailblaze_targets(
    ctx: &GameCtx,
    caster_id: usize,
    search_radius: u64,
) -> Vec<usize> {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return Vec::new();
    };
    if !caster.is_alive() {
        return Vec::new();
    }
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);

    let search_radius_sq = search_radius.saturating_mul(search_radius);
    let fields = GRASSY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let fields = fields.lock().expect("grassy terrain state poisoned");
    let candidate_fields: Vec<GrassyTerrainState> = fields
        .iter()
        .copied()
        .filter(|state| {
            state.caster_id == caster_id
                && distance_sq(state.center, caster_pos) <= search_radius_sq
        })
        .collect();
    drop(fields);

    let mut targets = Vec::new();
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.team() == caster_team || !entity.is_alive() {
            continue;
        }
        if candidate_fields.iter().any(|field| {
            distance_sq(entity.pos(), field.center) <= field.radius.saturating_mul(field.radius)
        }) {
            targets.push(entity.id());
        }
    }
    targets
}

#[allow(clippy::too_many_arguments)]
pub fn detonate_grassy_terrains_near(
    ctx: &mut GameCtx,
    caster_id: usize,
    search_radius: u64,
    damage: usize,
    attacker_types: TypeSet,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    let caster_pos = caster.pos();
    drop(caster);

    let search_radius_sq = search_radius.saturating_mul(search_radius);
    let fields = GRASSY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("grassy terrain state poisoned");
    let mut detonated = Vec::new();
    fields.retain(|state| {
        let should_detonate = state.caster_id == caster_id
            && distance_sq(state.center, caster_pos) <= search_radius_sq;
        if should_detonate {
            detonated.push(*state);
            false
        } else {
            true
        }
    });
    drop(fields);

    if detonated.is_empty() {
        return;
    }

    let mut hits: Vec<(usize, EntityPos)> = Vec::new();
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.team() == caster_team || !entity.is_alive() {
            continue;
        }
        if detonated.iter().any(|field| {
            distance_sq(entity.pos(), field.center) <= field.radius.saturating_mul(field.radius)
        }) {
            hits.push((entity.id(), entity.pos()));
        }
    }

    for field in detonated {
        draw_field_circle(ctx, field.center, field.radius, VFX_FIRE);
    }

    for (target_id, target_pos) in hits {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Grass,
            attacker_types,
            defender_types,
        );
        if burn_chance_percent >= 100
            || splitmix64(
                ctx.seed()
                    ^ ((caster_id as u64) << 40)
                    ^ ((actual_target_id as u64) << 16)
                    ^ ctx.tick() as u64
                    ^ 0xa490_u64,
            ) % 100
                < burn_chance_percent as u64
        {
            apply_burn_for(
                ctx,
                caster_id,
                actual_target_id,
                burn_damage.max(1),
                burn_ticks,
            );
        }
        draw_status_marker(ctx, target_pos, 9000, VFX_FIRE);
    }
}

pub fn begin_sticky_web(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    radius: u64,
    max_webs: usize,
    kricketune_speed_percent: i32,
    enemy_slow_percent: i32,
    buff_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }

    let webs = STICKY_WEBS.get_or_init(|| Mutex::new(Vec::new()));
    let mut webs = webs.lock().expect("sticky web state poisoned");
    let tick = ctx.tick();
    webs.push(StickyWebState {
        caster_id,
        caster_team: caster.team(),
        center,
        radius,
        created_at: tick,
        next_tick_at: tick,
        kricketune_speed_percent,
        enemy_slow_percent,
        buff_ticks,
    });

    while webs
        .iter()
        .filter(|state| state.caster_id == caster_id)
        .count()
        > max_webs
    {
        let Some((index, _)) = webs
            .iter()
            .enumerate()
            .filter(|(_, state)| state.caster_id == caster_id)
            .min_by_key(|(_, state)| state.created_at)
        else {
            break;
        };
        webs.remove(index);
    }
}

pub fn burn_sticky_webs_at(center: EntityPos, radius: u64) {
    let webs = STICKY_WEBS.get_or_init(|| Mutex::new(Vec::new()));
    let mut webs = webs.lock().expect("sticky web state poisoned");
    let radius_sq = radius.saturating_mul(radius);
    webs.retain(|state| distance_sq(state.center, center) > radius_sq);
}

fn sticky_webs_overlap(left: StickyWebState, right: StickyWebState) -> bool {
    let combined_radius = left.radius.saturating_add(right.radius);
    distance_sq(left.center, right.center) <= combined_radius.saturating_mul(combined_radius)
}

fn connected_sticky_web_indices_for_caster(
    webs: &[StickyWebState],
    caster_id: usize,
    caster_pos: EntityPos,
) -> Vec<usize> {
    let mut connected = vec![false; webs.len()];
    let mut stack = Vec::new();

    for (index, web) in webs.iter().enumerate() {
        if web.caster_id == caster_id
            && distance_sq(caster_pos, web.center) <= web.radius.saturating_mul(web.radius)
        {
            connected[index] = true;
            stack.push(index);
        }
    }

    while let Some(current_index) = stack.pop() {
        let current = webs[current_index];
        for (candidate_index, candidate) in webs.iter().enumerate() {
            if connected[candidate_index] || candidate.caster_id != caster_id {
                continue;
            }
            if sticky_webs_overlap(current, *candidate) {
                connected[candidate_index] = true;
                stack.push(candidate_index);
            }
        }
    }

    connected
        .into_iter()
        .enumerate()
        .filter_map(|(index, is_connected)| is_connected.then_some(index))
        .collect()
}

fn note_web_walker_spot(
    tick: usize,
    source_id: usize,
    team: usize,
    target_id: usize,
    target_pos: EntityPos,
) {
    note_team_spot_for_ticks(
        tick,
        source_id,
        team,
        target_id,
        target_pos,
        WEB_WALKER_SPOT_TICKS,
    );
}

fn note_team_spot_for_ticks(
    tick: usize,
    source_id: usize,
    team: usize,
    target_id: usize,
    target_pos: EntityPos,
    duration_ticks: usize,
) {
    let spots = WEB_WALKER_SPOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut spots = spots.lock().expect("web walker spot state poisoned");
    spots.retain(|spot| spot.expires_at > tick);
    let expires_at = tick.saturating_add(duration_ticks);
    if let Some(existing) = spots
        .iter_mut()
        .find(|spot| spot.team == team && spot.target_id == target_id)
    {
        existing.source_id = source_id;
        existing.target_pos = target_pos;
        existing.expires_at = expires_at;
        return;
    }

    spots.push(WebWalkerSpot {
        source_id,
        team,
        target_id,
        target_pos,
        expires_at,
    });
}

fn update_web_walker_spots(ctx: &GameCtx, tick: usize, webs: &[StickyWebState]) {
    let mut caster_ids = Vec::new();
    for web in webs {
        if !caster_ids.contains(&web.caster_id) {
            caster_ids.push(web.caster_id);
        }
    }

    for caster_id in caster_ids {
        let Some(caster) = ctx.get_entity(caster_id) else {
            continue;
        };
        if !caster.is_alive() {
            continue;
        }
        let caster_team = caster.team();
        let caster_pos = caster.pos();
        drop(caster);

        let mut sources = vec![(caster_id, caster_team, caster_pos)];
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.id() == caster_id
                || !entity.is_alive()
                || !entity.is_champion()
                || !receiver_has_copied(entity.id(), "pokemon_moba_kricketune")
            {
                continue;
            }
            let entity_team = entity.team();
            let entity_pos = entity.pos();
            let in_caster_web = webs.iter().any(|web| {
                web.caster_id == caster_id
                    && distance_sq(entity_pos, web.center) <= web.radius.saturating_mul(web.radius)
            });
            if in_caster_web {
                sources.push((entity.id(), entity_team, entity_pos));
            }
        }

        for (source_id, source_team, source_pos) in sources {
            let connected_indices =
                connected_sticky_web_indices_for_caster(webs, caster_id, source_pos);
            if connected_indices.is_empty() {
                continue;
            }

            for index in 0..ctx.entity_count() {
                let Some(entity) = ctx.entity_at(index) else {
                    continue;
                };
                if entity.team() == source_team || !entity.is_alive() || !entity.is_champion() {
                    continue;
                }
                let target_id = entity.id();
                let target_pos = entity.pos();
                let in_connected_web = connected_indices.iter().any(|web_index| {
                    let web = webs[*web_index];
                    distance_sq(target_pos, web.center) <= web.radius.saturating_mul(web.radius)
                });
                if in_connected_web {
                    note_web_walker_spot(tick, source_id, source_team, target_id, target_pos);
                }
            }
        }
    }
}

pub fn best_web_walker_spotted_target(
    team: usize,
    origin_pos: EntityPos,
    max_radius: u64,
    tick: usize,
) -> Option<WebWalkerSpot> {
    let max_radius_sq = max_radius.saturating_mul(max_radius);
    let spots = WEB_WALKER_SPOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut spots = spots.lock().expect("web walker spot state poisoned");
    spots.retain(|spot| spot.expires_at > tick);

    let mut best: Option<(u64, WebWalkerSpot)> = None;
    for spot in spots.iter().filter(|spot| spot.team == team) {
        let distance = distance_sq(origin_pos, spot.target_pos);
        if distance > max_radius_sq {
            continue;
        }
        let current = *spot;
        if best
            .map(|(best_distance, _)| distance < best_distance)
            .unwrap_or(true)
        {
            best = Some((distance, current));
        }
    }

    best.map(|(_, spot)| spot)
}

pub fn begin_sing_aura(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    sleep_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let auras = SING_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("sing aura state poisoned");
    auras.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    auras.push(SingAuraState {
        caster_id,
        caster_team: caster.team(),
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        radius,
        sleep_ticks,
    });
}

pub fn begin_frosmoth_sleep_circle(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    sleep_ticks: usize,
    force_move_speed: u64,
    force_move_ticks: u64,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let origin = caster.pos();
    let center = origin;
    let caster_team = caster.team();
    drop(caster);

    let circles = FROSMOTH_SLEEP_CIRCLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut circles = circles.lock().expect("frosmoth sleep state poisoned");
    circles.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    circles.push(FrosmothSleepCircleState {
        caster_id,
        caster_team,
        origin,
        center,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        next_move_at: tick,
        path_index: 0,
        radius,
        sleep_ticks,
        force_move_speed,
        force_move_ticks,
    });
}

pub fn begin_bug_buzz_aura(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    damage_per_tick: usize,
    confusion_stacks: usize,
    confusion_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let auras = BUG_BUZZ_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("bug buzz aura state poisoned");
    auras.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    auras.push(BugBuzzAuraState {
        caster_id,
        caster_team: caster.team(),
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        radius,
        damage_per_tick,
        confusion_stacks,
        confusion_ticks,
        attacker_types,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn begin_armarouge_mystical_fire_aura(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    tick_interval: usize,
    radius: u64,
    damage: usize,
    attacker_types: TypeSet,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
    confusion_chance_percent: usize,
    confusion_stacks: usize,
    confusion_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    drop(caster);

    let tick = ctx.tick();
    let auras = ARMAROUGE_MYSTICAL_FIRE_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras
        .lock()
        .expect("armarouge mystical fire state poisoned");
    auras.retain(|state| state.caster_id != caster_id && state.expires_at > tick);
    auras.push(ArmarougeMysticalFireAuraState {
        caster_id,
        caster_team,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        tick_interval: tick_interval.max(1),
        radius,
        damage,
        burn_chance_percent,
        burn_ticks,
        burn_damage,
        confusion_chance_percent,
        confusion_stacks,
        confusion_ticks,
        attacker_types,
    });
}

pub fn begin_alluring_voice_aura(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    outer_radius: u64,
    inner_radius: u64,
    taunt_ticks: usize,
    confusion_stacks: usize,
    confusion_ticks: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let auras = ALLURING_VOICE_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("alluring voice aura state poisoned");
    auras.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    auras.push(AlluringVoiceAuraState {
        caster_id,
        caster_team: caster.team(),
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        outer_radius,
        inner_radius,
        taunt_ticks,
        confusion_stacks,
        confusion_ticks,
    });
}

pub fn begin_whirlpool(
    ctx: &GameCtx,
    caster_id: usize,
    center: EntityPos,
    duration_ticks: usize,
    radius: u64,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    self_heal_per_tick: usize,
    self_attack_mult: i32,
    self_buff_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }

    let whirlpools = WHIRLPOOLS.get_or_init(|| Mutex::new(Vec::new()));
    let mut whirlpools = whirlpools.lock().expect("whirlpool state poisoned");
    let tick = ctx.tick();
    whirlpools.push(WhirlpoolState {
        caster_id,
        caster_team: caster.team(),
        center,
        radius,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        damage_per_tick,
        slow_percent,
        slow_ticks,
        self_heal_per_tick,
        self_attack_mult,
        self_buff_ticks,
        attacker_types,
    });
}

pub fn begin_charm_heal(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    duration_ticks: usize,
    heal_per_tick: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !caster.is_alive()
        || !target.is_alive()
        || caster.team() != target.team()
        || !target.is_champion()
    {
        return;
    }

    let heals = CHARM_HEALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut heals = heals.lock().expect("charm heal state poisoned");
    heals.retain(|state| !(state.caster_id == caster_id && state.target_id == target_id));
    let tick = ctx.tick();
    heals.push(CharmHealState {
        caster_id,
        target_id,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        heal_per_tick,
    });
}

pub fn begin_blissey_heal_aura(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    heal_per_tick: usize,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let auras = BLISSEY_HEAL_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("blissey heal aura state poisoned");
    auras.retain(|state| state.caster_id != caster_id);
    let tick = ctx.tick();
    auras.push(BlisseyHealAuraState {
        caster_id,
        caster_team: caster.team(),
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        radius,
        heal_per_tick,
    });
}

pub fn note_kleavor_hit(ctx: &GameCtx, entity_id: usize, window_ticks: usize) -> usize {
    note_kleavor_hit_in(ctx, entity_id, window_ticks, &KLEAVOR_SHARPNESS_HITS)
}

pub fn note_kleavor_stone_axe_hit(ctx: &GameCtx, entity_id: usize, window_ticks: usize) -> usize {
    note_kleavor_hit_in(ctx, entity_id, window_ticks, &KLEAVOR_STONE_AXE_HITS)
}

fn note_kleavor_hit_in(
    ctx: &GameCtx,
    entity_id: usize,
    window_ticks: usize,
    store: &OnceLock<Mutex<Vec<KleavorHitState>>>,
) -> usize {
    let tick = ctx.tick();
    let states = store.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("kleavor hit state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(KleavorHitState {
            entity_id,
            hits: 1,
            last_hit_tick: tick,
        });
        return 1;
    };
    if tick.saturating_sub(state.last_hit_tick) > window_ticks {
        state.hits = 0;
    }
    state.hits = state.hits.saturating_add(1);
    state.last_hit_tick = tick;
    state.hits
}

pub fn begin_stealth_rock(
    ctx: &GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    toggle_interval_ticks: usize,
) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let states = STEALTH_ROCKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("stealth rock state poisoned");
    states.retain(|state| state.entity_id != entity_id);
    let tick = ctx.tick();
    states.push(StealthRockState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
        next_toggle_at: tick.saturating_add(toggle_interval_ticks),
        untargetable: true,
    });
    apply_soft_untargetable(ctx, entity_id, toggle_interval_ticks);
}

pub fn update_light_metal_passive(ctx: &mut GameCtx, entity_id: usize) {
    const DISTANCE_THRESHOLD: u64 = 120_000;
    const SHIELD_TICKS: usize = 6 * 60;
    const SHIELD_MOVE_SPEED_PERCENT: usize = 20;

    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }

    let tick = ctx.tick();
    let pos = entity.pos();
    let move_speed = entity.stat().move_speed;
    drop(entity);

    let states = LIGHT_METALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("light metal state poisoned");
    states.retain(|state| {
        ctx.get_entity(state.entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false)
    });

    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(LightMetalState {
            entity_id,
            last_pos: pos,
            distance_without_damage: 0,
            shield_until: 0,
        });
        return;
    };

    let distance = state
        .last_pos
        .x
        .abs_diff(pos.x)
        .saturating_add(state.last_pos.y.abs_diff(pos.y));
    state.last_pos = pos;
    state.distance_without_damage = state.distance_without_damage.saturating_add(distance);

    if tick < state.shield_until || state.distance_without_damage < DISTANCE_THRESHOLD {
        return;
    }

    state.distance_without_damage = 0;
    state.shield_until = tick.saturating_add(SHIELD_TICKS);
    drop(states);

    let shield = move_speed.saturating_mul(SHIELD_MOVE_SPEED_PERCENT) / 100;
    if shield == 0 {
        return;
    }
    ctx.add_buff(
        entity_id,
        BuffState {
            duration: BuffType::Time { tick: SHIELD_TICKS },
            hp: shield.min(i32::MAX as usize) as i32,
            ..Default::default()
        },
    );
}

pub fn update_swift_swim_passive(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    if is_in_water_aoe(ctx, pos) {
        ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: 35 },
                move_speed_mult: 35,
                ..Default::default()
            },
        );
    }
}

pub fn update_wide_guard_passive(ctx: &GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let states = WIDE_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("wide guard state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(WideGuardState {
            entity_id,
            last_pos: pos,
            facing_dx: 0,
            facing_dy: 1,
        });
        return;
    };

    let dx = pos.x as i64 - state.last_pos.x as i64;
    let dy = pos.y as i64 - state.last_pos.y as i64;
    let moved = dx.unsigned_abs().saturating_add(dy.unsigned_abs());
    state.last_pos = pos;
    if moved >= 500 {
        state.facing_dx = dx;
        state.facing_dy = dy;
    }
}

pub fn update_afro_passive(ctx: &GameCtx, entity_id: usize) {
    update_wide_guard_passive(ctx, entity_id);
}

pub fn wide_guard_reduce_percent(ctx: &GameCtx, target_id: usize, attacker_id: usize) -> usize {
    const REDUCE_PERCENT: usize = 30;

    let protected = champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_drednaw")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_drednaw");
    if !protected {
        return 0;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return 0;
    };
    if !target.is_alive() {
        return 0;
    }
    let target_pos = target.pos();
    drop(target);

    let Some(attacker) = ctx.get_entity(attacker_id) else {
        return 0;
    };
    if !attacker.is_alive() {
        return 0;
    }
    let attacker_pos = attacker.pos();
    drop(attacker);

    let states = WIDE_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("wide guard state poisoned");
    let Some(state) = states.iter().find(|state| state.entity_id == target_id) else {
        return 0;
    };

    let incoming_dx = attacker_pos.x as i64 - target_pos.x as i64;
    let incoming_dy = attacker_pos.y as i64 - target_pos.y as i64;
    let dot = incoming_dx
        .saturating_mul(state.facing_dx)
        .saturating_add(incoming_dy.saturating_mul(state.facing_dy));
    if dot < 0 {
        REDUCE_PERCENT
    } else {
        0
    }
}

pub fn afro_reduce_percent(ctx: &GameCtx, target_id: usize, attacker_id: usize) -> usize {
    let protected = champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_bouffalant")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_bouffalant");
    if !protected {
        return 0;
    }

    let Some(target) = ctx.get_entity(target_id) else {
        return 0;
    };
    if !target.is_alive() {
        return 0;
    }
    let target_pos = target.pos();
    drop(target);

    let Some(attacker) = ctx.get_entity(attacker_id) else {
        return 0;
    };
    if !attacker.is_alive() {
        return 0;
    }
    let attacker_pos = attacker.pos();
    drop(attacker);

    let states = WIDE_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("wide guard state poisoned");
    let Some(state) = states.iter().find(|state| state.entity_id == target_id) else {
        return 0;
    };

    let incoming_dx = attacker_pos.x as f64 - target_pos.x as f64;
    let incoming_dy = attacker_pos.y as f64 - target_pos.y as f64;
    let facing_dx = state.facing_dx as f64;
    let facing_dy = state.facing_dy as f64;
    let incoming_len = (incoming_dx * incoming_dx + incoming_dy * incoming_dy).sqrt();
    let facing_len = (facing_dx * facing_dx + facing_dy * facing_dy).sqrt();
    if incoming_len <= 0.0 || facing_len <= 0.0 {
        return 0;
    }
    let cos = (incoming_dx * facing_dx + incoming_dy * facing_dy) / (incoming_len * facing_len);
    if cos >= 0.5 {
        BOUFFALANT_AFRO_DAMAGE_REDUCE_PERCENT
    } else {
        0
    }
}

pub fn update_audino_regenerator(ctx: &mut GameCtx, audino_id: usize) {
    let tick = ctx.tick();
    let states = AUDINO_REGENERATORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("audino regenerator state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.audino_id == audino_id) else {
        return;
    };
    if tick.saturating_sub(state.last_damaged_tick) < AUDINO_REGENERATOR_DELAY_TICKS
        || tick < state.next_heal_tick
    {
        return;
    }
    state.next_heal_tick = tick.saturating_add(AUDINO_REGENERATOR_INTERVAL_TICKS);
    drop(states);

    let Some(audino) = ctx.get_entity(audino_id) else {
        return;
    };
    if !audino.is_alive() {
        return;
    }
    let audino_pos = audino.pos();
    let audino_hp = audino.hp();
    let audino_stat = audino.stat();
    let audino_team = audino.team();
    drop(audino);

    let heal = (18 + audino_stat.hp.saturating_mul(2) / 100).max(1);
    if audino_hp.current < audino_hp.max {
        heal_with_antiheal(ctx, audino_id, audino_id, heal);
        return;
    }

    let mut closest: Option<(u64, usize)> = None;
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.id() == audino_id
            || entity.team() != audino_team
            || !entity.is_champion()
            || !entity.is_alive()
        {
            continue;
        }
        let hp = entity.hp();
        if hp.current >= hp.max {
            continue;
        }
        let distance = distance_sq(entity.pos(), audino_pos);
        if distance > AUDINO_REGENERATOR_RADIUS.saturating_mul(AUDINO_REGENERATOR_RADIUS) {
            continue;
        }
        if closest
            .map(|(best_distance, _)| distance < best_distance)
            .unwrap_or(true)
        {
            closest = Some((distance, entity.id()));
        }
    }
    if let Some((_, ally_id)) = closest {
        heal_with_antiheal(ctx, audino_id, ally_id, heal);
        after_blissey_heal(ctx, audino_id, ally_id);
    }
}

fn is_in_water_aoe(ctx: &GameCtx, pos: EntityPos) -> bool {
    let tick = ctx.tick();
    if BRINE_FIELDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("brine field state poisoned")
        .iter()
        .any(|state| {
            state.expires_at > tick
                && distance_sq(pos, state.center) <= state.radius.saturating_mul(state.radius)
        })
    {
        return true;
    }

    if WHIRLPOOLS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("whirlpool state poisoned")
        .iter()
        .any(|state| {
            state.expires_at > tick
                && distance_sq(pos, state.center) <= state.radius.saturating_mul(state.radius)
        })
    {
        return true;
    }

    AQUA_RINGS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("aqua ring state poisoned")
        .iter()
        .any(|state| {
            state.expires_at > tick
                && ctx
                    .get_entity(state.caster_id)
                    .map(|caster| {
                        caster.is_alive()
                            && distance_sq(pos, caster.pos())
                                <= state.radius.saturating_mul(state.radius)
                    })
                    .unwrap_or(false)
        })
}

pub fn reset_light_metal(ctx: &GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    let pos = entity.pos();
    let tick = ctx.tick();
    drop(entity);

    let states = LIGHT_METALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("light metal state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.last_pos = pos;
        state.distance_without_damage = 0;
        state.shield_until = tick;
    } else {
        states.push(LightMetalState {
            entity_id,
            last_pos: pos,
            distance_without_damage: 0,
            shield_until: tick,
        });
    }
}

pub fn note_light_metal_dealt_damage(ctx: &GameCtx, entity_id: usize) {
    let is_scizor = SCIZORS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("scizor state poisoned")
        .iter()
        .any(|id| *id == entity_id && entity_is_champion_id(entity_id, "pokemon_moba_scizor"))
        || receiver_has_copied(entity_id, "pokemon_moba_scizor");
    if is_scizor {
        reset_light_metal(ctx, entity_id);
    }
}

pub fn register_armarouge_weak_armor(entity_id: usize) {
    let states = ARMAROUGE_WEAK_ARMORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("armarouge weak armor state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.weak_stacks = 0;
        state.armor_cannon_until = 0;
        state.armor_cannon_defence_mult = 0;
        state.armor_cannon_magic_resistance_mult = 0;
        return;
    }
    states.push(ArmarougeWeakArmorState {
        entity_id,
        weak_stacks: 0,
        armor_cannon_until: 0,
        armor_cannon_defence_mult: 0,
        armor_cannon_magic_resistance_mult: 0,
    });
}

pub fn note_armarouge_landed_attack(ctx: &GameCtx, entity_id: usize) {
    let tick = ctx.tick();
    let states = ARMAROUGE_WEAK_ARMORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("armarouge weak armor state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        if champion_id_for_entity(entity_id)
            .map(|champion_id| champion_id == "pokemon_moba_armarouge")
            .unwrap_or(false)
            || receiver_has_copied(entity_id, "pokemon_moba_armarouge")
        {
            states.push(ArmarougeWeakArmorState {
                entity_id,
                weak_stacks: 1,
                armor_cannon_until: 0,
                armor_cannon_defence_mult: 0,
                armor_cannon_magic_resistance_mult: 0,
            });
        }
        return;
    };
    if state.armor_cannon_until > tick {
        return;
    }
    state.weak_stacks = state.weak_stacks.saturating_add(1).min(16);
}

pub fn begin_armarouge_armor_cannon(
    ctx: &GameCtx,
    entity_id: usize,
    ticks: usize,
    defence_mult: i32,
    magic_resistance_mult: i32,
) {
    let until = ctx.tick().saturating_add(ticks);
    let states = ARMAROUGE_WEAK_ARMORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("armarouge weak armor state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.weak_stacks = 0;
        state.armor_cannon_until = until;
        state.armor_cannon_defence_mult = defence_mult;
        state.armor_cannon_magic_resistance_mult = magic_resistance_mult;
        return;
    }
    states.push(ArmarougeWeakArmorState {
        entity_id,
        weak_stacks: 0,
        armor_cannon_until: until,
        armor_cannon_defence_mult: defence_mult,
        armor_cannon_magic_resistance_mult: magic_resistance_mult,
    });
}

pub fn update_armarouge_weak_armor(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        drop(entity);
        register_armarouge_weak_armor(entity_id);
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let tick = ctx.tick();
    let states = ARMAROUGE_WEAK_ARMORS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("armarouge weak armor state poisoned");
    let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) else {
        states.push(ArmarougeWeakArmorState {
            entity_id,
            weak_stacks: 0,
            armor_cannon_until: 0,
            armor_cannon_defence_mult: 0,
            armor_cannon_magic_resistance_mult: 0,
        });
        return;
    };
    let penalty = if state.armor_cannon_until > tick {
        Some((
            state.armor_cannon_defence_mult,
            state.armor_cannon_magic_resistance_mult,
            VFX_FIRE,
        ))
    } else if state.weak_stacks > 0 {
        let stack_penalty = -((state.weak_stacks.saturating_mul(5)).min(65) as i32);
        Some((stack_penalty, stack_penalty, VFX_PSYCHIC))
    } else {
        None
    };
    drop(states);

    if let Some((defence_mult, magic_resistance_mult, color)) = penalty {
        ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: 35 },
                defence_mult,
                magic_resistance_mult,
                ..Default::default()
            },
        );
        ctx.debug_draw_circle(pos.x, pos.y, 14000, color);
    }
}

pub fn update_receiver_copied_passive(ctx: &mut GameCtx, entity_id: usize) {
    match receiver_copied_champion_id(entity_id) {
        Some("pokemon_moba_venusaur") => update_tangling_vines_aura(ctx, entity_id),
        Some("pokemon_moba_eevee") => update_helping_hand_aura(ctx, entity_id),
        Some(
            "pokemon_moba_jolteon"
            | "pokemon_moba_flareon"
            | "pokemon_moba_vaporeon"
            | "pokemon_moba_leafeon"
            | "pokemon_moba_glaceon"
            | "pokemon_moba_umbreon"
            | "pokemon_moba_espeon"
            | "pokemon_moba_sylveon",
        ) => update_eeveelution_passive(ctx, entity_id),
        Some("pokemon_moba_ludicolo") => update_ludicolo_rain_dish(ctx, entity_id),
        Some("pokemon_moba_oranguru") => update_symbiosis_passive(ctx, entity_id),
        Some("pokemon_moba_blaziken") => update_speed_boost_passive(ctx, entity_id),
        Some("pokemon_moba_scizor") => update_light_metal_passive(ctx, entity_id),
        Some("pokemon_moba_mantine") => update_swift_swim_passive(ctx, entity_id),
        Some("pokemon_moba_drednaw") => update_wide_guard_passive(ctx, entity_id),
        Some("pokemon_moba_bouffalant") => update_afro_passive(ctx, entity_id),
        Some("pokemon_moba_houndoom") => update_intimidate_aura(ctx, entity_id),
        Some("pokemon_moba_audino") => update_audino_regenerator(ctx, entity_id),
        Some("pokemon_moba_ribombee") => update_honey_gatherer(ctx, entity_id),
        Some("pokemon_moba_starmie") => update_starmie_illuminate_passive(ctx, entity_id),
        Some("pokemon_moba_drampa") => update_drampa_berserk_passive(ctx, entity_id),
        Some("pokemon_moba_shedinja") => update_wonder_guard(ctx, entity_id),
        Some("pokemon_moba_noivern") => update_infiltrator_placeholder(ctx, entity_id),
        Some("pokemon_moba_octillery") => update_suction_cups_placeholder(ctx, entity_id),
        Some("pokemon_moba_armarouge") => update_armarouge_weak_armor(ctx, entity_id),
        Some("pokemon_moba_delibird") => update_delibird_hustle(ctx, entity_id),
        Some("pokemon_moba_cloyster") => update_cloyster_overcoat(ctx, entity_id),
        Some("pokemon_moba_snorlax") => update_snorlax_gluttony(ctx, entity_id),
        Some("pokemon_moba_swanna") => update_swanna_tailwind(ctx, entity_id),
        Some("pokemon_moba_ampharos") => update_ampharos_luminous_pulse(ctx, entity_id),
        Some("pokemon_moba_quaquaval") => update_quaquaval_aqua_step(ctx, entity_id),
        Some("pokemon_moba_inteleon") => ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: 45 },
                crit_chance: 15,
                ..Default::default()
            },
        ),
        Some("pokemon_moba_emboar") => ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time { tick: 45 },
                damaged_amplify: 10,
                ..Default::default()
            },
        ),
        Some(champion_id) if is_eeveelution_champion_id(champion_id) => {
            update_eeveelution_passive(ctx, entity_id)
        }
        _ => {}
    }
}

pub fn update_infiltrator_placeholder(_ctx: &mut GameCtx, _entity_id: usize) {}

pub fn update_suction_cups_placeholder(_ctx: &mut GameCtx, _entity_id: usize) {}

pub fn update_ludicolo_rain_dish(ctx: &mut GameCtx, entity_id: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let pos = entity.pos();
    drop(entity);

    let tick = ctx.tick();
    let states = LUDICOLO_RAIN_DISHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut should_heal = false;
    {
        let mut states = states.lock().expect("ludicolo rain dish state poisoned");
        states.retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive() && entity.is_champion())
                .unwrap_or(false)
        });
        if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
            if existing.next_tick_at <= tick {
                should_heal = true;
                existing.next_tick_at = tick.saturating_add(LUDICOLO_RAIN_DISH_INTERVAL_TICKS);
            }
        } else {
            states.push(LudicoloRainDishState {
                entity_id,
                next_tick_at: tick.saturating_add(LUDICOLO_RAIN_DISH_INTERVAL_TICKS),
            });
        }
    }

    draw_field_circle(ctx, pos, LUDICOLO_RAIN_DISH_RADIUS, VFX_WATER);
    if should_heal {
        heal_with_antiheal(ctx, entity_id, entity_id, LUDICOLO_RAIN_DISH_HEAL);
    }
}

pub fn ludicolo_rain_dish_damage_modifier(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    move_type: PokemonType,
) -> Option<i32> {
    if !matches!(move_type, PokemonType::Water | PokemonType::Fire) {
        return None;
    }
    let Some(caster_pos) = ctx.get_entity(caster_id).map(|entity| entity.pos()) else {
        return None;
    };
    let Some(target_pos) = ctx.get_entity(target_id).map(|entity| entity.pos()) else {
        return None;
    };
    let radius_sq = LUDICOLO_RAIN_DISH_RADIUS.saturating_mul(LUDICOLO_RAIN_DISH_RADIUS);
    let in_rain = LUDICOLO_RAIN_DISHES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("ludicolo rain dish state poisoned")
        .iter()
        .filter_map(|state| ctx.get_entity(state.entity_id))
        .filter(|entity| entity.is_alive() && entity.is_champion())
        .map(|entity| entity.pos())
        .any(|rain_pos| {
            distance_sq(caster_pos, rain_pos) <= radius_sq
                || distance_sq(target_pos, rain_pos) <= radius_sq
        });

    if !in_rain {
        return None;
    }
    match move_type {
        PokemonType::Water => Some(LUDICOLO_RAIN_DISH_WATER_BONUS as i32),
        PokemonType::Fire => Some(-(LUDICOLO_RAIN_DISH_FIRE_REDUCE as i32)),
        _ => None,
    }
}

pub fn ludicolo_fire_resistance_percent(ctx: &GameCtx, target_id: usize) -> usize {
    let has_rain_dish = champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_ludicolo")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_ludicolo");
    if has_rain_dish
        && ctx
            .get_entity(target_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false)
    {
        LUDICOLO_FIRE_RESIST_REDUCE
    } else {
        0
    }
}

pub fn update_drampa_berserk_passive(ctx: &mut GameCtx, entity_id: usize) {
    const INTERVAL_TICKS: usize = 30;
    const BUFF_TICKS: usize = 35;

    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let hp = entity.hp();
    let pos = entity.pos();
    drop(entity);

    if hp.max == 0 || hp.current.saturating_mul(2) >= hp.max {
        return;
    }

    let tick = ctx.tick();
    let states = DRAMPA_BERSERKS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut states = states.lock().expect("drampa berserk state poisoned");
        if let Some(existing) = states.iter_mut().find(|state| state.entity_id == entity_id) {
            if tick.saturating_sub(existing.last_tick) < INTERVAL_TICKS {
                return;
            }
            existing.last_tick = tick;
        } else {
            states.push(DrampaBerserkState {
                entity_id,
                last_tick: tick,
            });
        }
    }

    ctx.add_buff(
        entity_id,
        BuffState {
            duration: BuffType::Time { tick: BUFF_TICKS },
            attack_mult: 35,
            magic_power_mult: 35,
            crit_chance: 35,
            defence_mult: -15,
            magic_resistance_mult: -15,
            ..Default::default()
        },
    );
    ctx.debug_draw_circle(pos.x, pos.y, 18000, VFX_DRAGON);
    ctx.debug_draw_circle(pos.x, pos.y, 9000, VFX_DRAGON);
}

pub fn note_receiver_copied_damage_taken(ctx: &GameCtx, entity_id: usize, damage: usize) {
    if damage == 0 {
        return;
    }
    if receiver_has_copied(entity_id, "pokemon_moba_blaziken") {
        reset_speed_boost(ctx, entity_id);
    }
    if receiver_has_copied(entity_id, "pokemon_moba_scizor") {
        reset_light_metal(ctx, entity_id);
    }
    if receiver_has_copied(entity_id, "pokemon_moba_houndoom") {
        note_houndoom_damaged(ctx, entity_id);
    }
    if receiver_has_copied(entity_id, "pokemon_moba_audino") {
        note_audino_damaged(ctx, entity_id);
    }
}

pub fn update_symbiosis_passive(ctx: &mut GameCtx, oranguru_id: usize) {
    let tick = ctx.tick();
    let states = SYMBIOSIS_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    {
        let mut states = states.lock().expect("symbiosis state poisoned");
        if let Some(existing) = states
            .iter_mut()
            .find(|state| state.oranguru_id == oranguru_id)
        {
            if tick.saturating_sub(existing.last_tick) < SYMBIOSIS_INTERVAL_TICKS {
                return;
            }
            existing.last_tick = tick;
        } else {
            states.push(SymbiosisState {
                oranguru_id,
                last_tick: tick,
            });
        }
    }

    let Some(oranguru) = ctx.get_entity(oranguru_id) else {
        return;
    };
    if !oranguru.is_alive() || !oranguru.is_champion() {
        return;
    }
    let team = oranguru.team();
    let pos = oranguru.pos();
    let stat = oranguru.stat();
    let level = oranguru.level();
    drop(oranguru);
    let base_champion_id = champion_id_for_entity(oranguru_id).unwrap_or("pokemon_moba_oranguru");
    let Some(base_stat) = champion_base_stat_for_level(base_champion_id, level) else {
        return;
    };

    let mut best_ally: Option<(u64, usize)> = None;
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.id() == oranguru_id
            || entity.team() != team
            || !entity.is_alive()
            || !entity.is_champion()
        {
            continue;
        }
        let dist = distance_sq(entity.pos(), pos);
        if dist > SYMBIOSIS_RADIUS.saturating_mul(SYMBIOSIS_RADIUS) {
            continue;
        }
        if best_ally
            .map(|(best_dist, _)| dist < best_dist)
            .unwrap_or(true)
        {
            best_ally = Some((dist, entity.id()));
        }
    }

    let Some((_, ally_id)) = best_ally else {
        return;
    };

    let buff = symbiosis_item_share_buff(stat, base_stat, 45);
    if buff.attack == 0
        && buff.magic_power == 0
        && buff.hp == 0
        && buff.defence == 0
        && buff.magic_resistance == 0
        && buff.crit_chance == 0
    {
        return;
    }

    ctx.add_buff(ally_id, buff);
    note_ally_buff_received(ctx, oranguru_id, ally_id, 45);
}

fn symbiosis_item_share_buff(
    current: mod_api::EntityStat,
    base: mod_api::EntityStat,
    ticks: usize,
) -> BuffState {
    BuffState {
        duration: BuffType::Time { tick: ticks },
        attack: stat_surplus_share(current.attack, base.attack),
        magic_power: stat_surplus_share(current.magic_power, base.magic_power),
        hp: stat_surplus_share(current.hp, base.hp),
        defence: stat_surplus_share(current.defence, base.defence),
        magic_resistance: stat_surplus_share(current.magic_resistance, base.magic_resistance),
        crit_chance: stat_surplus_share(current.crit_chance, base.crit_chance),
        ..Default::default()
    }
}

fn stat_surplus_share(current: usize, base: usize) -> i32 {
    (current.saturating_sub(base) / 2).min(i32::MAX as usize) as i32
}

pub fn begin_nasty_plot(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    delay_ticks: usize,
    damage: usize,
    stun_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !caster.is_alive()
        || !target.is_alive()
        || caster.team() == target.team()
        || !target.is_champion()
    {
        return;
    }
    drop(caster);
    drop(target);

    let states = NASTY_PLOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("nasty plot state poisoned");
    states.retain(|state| !(state.caster_id == caster_id && state.target_id == target_id));
    states.push(NastyPlotState {
        caster_id,
        target_id,
        resolves_at: ctx.tick().saturating_add(delay_ticks),
        damage,
        stun_ticks,
        attacker_types,
    });
}

pub fn begin_stored_power(
    ctx: &GameCtx,
    caster_id: usize,
    duration_ticks: usize,
    radius: u64,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() || !caster.is_champion() {
        return;
    }
    let caster_team = caster.team();
    drop(caster);

    let states = STORED_POWERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("stored power state poisoned");
    states.retain(|state| state.caster_id != caster_id);
    states.push(StoredPowerState {
        caster_id,
        caster_team,
        radius,
        resolves_at: ctx.tick().saturating_add(duration_ticks),
        stored_damage: 0,
        attacker_types,
    });
}

pub fn note_stored_power_damage(ctx: &GameCtx, entity_id: usize, damage: usize) {
    if damage == 0 {
        return;
    }
    let tick = ctx.tick();
    let states = STORED_POWERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("stored power state poisoned");
    states.retain(|state| state.resolves_at > tick);
    if let Some(state) = states.iter_mut().find(|state| state.caster_id == entity_id) {
        state.stored_damage = state.stored_damage.saturating_add(damage);
    }
}

pub fn begin_earthquake(
    ctx: &mut GameCtx,
    caster_id: usize,
    radius: u64,
    duration_ticks: usize,
    tick_interval: usize,
    damage_per_tick: usize,
    slow_percent: i32,
    slow_ticks: usize,
    attacker_types: TypeSet,
) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    drop(caster);

    let tick = ctx.tick();
    let states = EARTHQUAKES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("earthquake state poisoned");
    states.retain(|state| state.caster_id != caster_id);
    states.push(EarthquakeAuraState {
        caster_id,
        caster_team,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        tick_interval: tick_interval.max(1),
        radius,
        damage_per_tick,
        slow_percent,
        slow_ticks,
        attacker_types,
    });
}

pub fn begin_blood_moon(
    ctx: &mut GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    hp_loss_per_second_percent: usize,
    attack_mult: i32,
    attack_speed_mult: i32,
    defence_mult: i32,
    magic_resistance_mult: i32,
    vamp: i32,
) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }
    drop(entity);

    let tick = ctx.tick();
    let states = BLOOD_MOONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("blood moon state poisoned");
    states.retain(|state| state.entity_id != entity_id);
    states.push(BloodMoonState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick.saturating_add(BURN_TICK_INTERVAL),
        hp_loss_per_second_percent,
    });
    drop(states);

    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: duration_ticks,
            },
            attack_mult,
            attack_speed_mult,
            defence_mult,
            magic_resistance_mult,
            vamp,
            ..Default::default()
        },
    );
}

pub fn begin_roost(ctx: &GameCtx, entity_id: usize, duration_ticks: usize, heal_per_tick: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }

    let original_types =
        entity_types(entity_id).unwrap_or(TypeSet::dual(PokemonType::Steel, PokemonType::Flying));
    let tick = ctx.tick();
    register_entity_types(entity_id, TypeSet::single(PokemonType::Steel));

    let states = ROOSTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("roost state poisoned");
    states.retain(|state| state.entity_id != entity_id);
    states.push(RoostState {
        entity_id,
        original_types,
        expires_at: tick.saturating_add(duration_ticks),
        next_tick_at: tick,
        heal_per_tick,
    });
}

pub fn begin_sigilyph_gravity(ctx: &GameCtx, entity_id: usize, duration_ticks: usize) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() {
        return;
    }

    let tick = ctx.tick();
    let states = SIGILYPH_GRAVITIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sigilyph gravity state poisoned");
    let original_types = states
        .iter()
        .find(|state| state.entity_id == entity_id)
        .map(|state| state.original_types)
        .unwrap_or_else(|| {
            entity_types(entity_id)
                .unwrap_or(TypeSet::dual(PokemonType::Psychic, PokemonType::Flying))
        });
    states.retain(|state| state.entity_id != entity_id);
    states.push(SigilyphGravityState {
        entity_id,
        original_types,
        expires_at: tick.saturating_add(duration_ticks),
    });
    register_entity_types(entity_id, TypeSet::single(PokemonType::Psychic));
}

pub fn has_sigilyph_glypher(_ctx: &GameCtx, entity_id: usize) -> bool {
    entity_is_champion_id(entity_id, "pokemon_moba_sigilyph")
        || receiver_has_copied(entity_id, "pokemon_moba_sigilyph")
}

pub fn apply_sigilyph_glyph(ctx: &GameCtx, caster_id: usize, target_id: usize) {
    if !has_sigilyph_glypher(ctx, caster_id) {
        return;
    }
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    drop(caster);

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }
    let target_team = target.team();
    drop(target);

    let tick = ctx.tick();
    let states = SIGILYPH_GLYPHS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sigilyph glyph state poisoned");
    if let Some(existing) = states
        .iter_mut()
        .find(|state| state.caster_id == caster_id && state.target_id == target_id)
    {
        existing.caster_team = caster_team;
        existing.target_team = target_team;
        existing.expires_at = tick.saturating_add(SIGILYPH_GLYPH_DURATION_TICKS);
        return;
    }
    states.push(SigilyphGlyphState {
        caster_id,
        caster_team,
        target_id,
        target_team,
        expires_at: tick.saturating_add(SIGILYPH_GLYPH_DURATION_TICKS),
    });
}

pub fn sigilyph_glyph_damage_bonus_percent(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
) -> usize {
    if !has_sigilyph_glypher(ctx, caster_id) {
        return 0;
    }
    let tick = ctx.tick();
    let states = SIGILYPH_GLYPHS.get_or_init(|| Mutex::new(Vec::new()));
    let states = states.lock().expect("sigilyph glyph state poisoned");
    if states.iter().any(|state| {
        state.caster_id == caster_id && state.target_id == target_id && state.expires_at > tick
    }) {
        SIGILYPH_GLYPH_DAMAGE_BONUS_PERCENT
    } else {
        0
    }
}

pub fn is_blood_moon_active(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    BLOOD_MOONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("blood moon state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.expires_at > tick)
}

pub fn after_blissey_heal(ctx: &mut GameCtx, caster_id: usize, target_id: usize) {
    if !is_blissey(caster_id) || caster_id == target_id {
        return;
    }
    let roll = splitmix64(
        ctx.seed() ^ ((caster_id as u64) << 38) ^ ((target_id as u64) << 14) ^ ctx.tick() as u64,
    ) % 100;
    if roll >= 50 {
        return;
    }
    if has_harmful_statuses(target_id) {
        cleanse_harmful_statuses(ctx, target_id);
    } else {
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: 3 * 60 },
                move_speed_mult: 25,
                ..Default::default()
            },
        );
    }
}

fn is_blissey(entity_id: usize) -> bool {
    BLISSEYS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("blissey state poisoned")
        .iter()
        .any(|id| *id == entity_id && entity_is_champion_id(entity_id, "pokemon_moba_blissey"))
        || receiver_has_copied(entity_id, "pokemon_moba_blissey")
}

fn has_harmful_statuses(target_id: usize) -> bool {
    PARALYSIS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("paralysis state poisoned")
        .iter()
        .any(|state| state.entity_id == target_id)
        || BURNS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("burn state poisoned")
            .iter()
            .any(|state| state.target_id == target_id)
        || POISONS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("poison state poisoned")
            .iter()
            .any(|state| state.target_id == target_id)
        || MIASMAS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("miasma state poisoned")
            .iter()
            .any(|state| state.target_id == target_id)
        || BLEEDS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("bleed state poisoned")
            .iter()
            .any(|state| state.target_id == target_id)
        || CONFUSIONS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("confusion state poisoned")
            .iter()
            .any(|state| state.entity_id == target_id)
        || SPIRIT_SHACKLES
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("spirit shackle state poisoned")
            .iter()
            .any(|state| state.entity_id == target_id)
        || SOAKS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .expect("soak state poisoned")
            .iter()
            .any(|state| state.entity_id == target_id)
}

pub fn has_harmful_status(ctx: &GameCtx, target_id: usize) -> bool {
    has_harmful_statuses(target_id) || is_crowd_controlled(ctx, target_id)
}

pub fn cleanse_harmful_statuses(ctx: &mut GameCtx, target_id: usize) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }

    PARALYSIS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("paralysis state poisoned")
        .retain(|state| state.entity_id != target_id);
    BURNS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("burn state poisoned")
        .retain(|state| state.target_id != target_id);
    POISONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("poison state poisoned")
        .retain(|state| state.target_id != target_id);
    MIASMAS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("miasma state poisoned")
        .retain(|state| state.target_id != target_id);
    BLEEDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("bleed state poisoned")
        .retain(|state| state.target_id != target_id);
    LEECH_SEEDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("leech seed state poisoned")
        .retain(|state| state.target_id != target_id);
    CONFUSIONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("confusion state poisoned")
        .retain(|state| state.entity_id != target_id);
    SPIRIT_SHACKLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("spirit shackle state poisoned")
        .retain(|state| state.entity_id != target_id);
    SOAKS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("soak state poisoned")
        .retain(|state| state.entity_id != target_id);
    OCTILLERY_LOCK_ONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("octillery lock-on state poisoned")
        .retain(|state| state.target_id != target_id);
    KOMMOO_DUELS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("kommo-o duel state poisoned")
        .retain(|state| state.target_id != target_id);

    ctx.add_buff(
        target_id,
        BuffState {
            duration: BuffType::Time {
                tick: CLEANSE_IMMUNITY_TICKS,
            },
            cc_immune: true,
            ..Default::default()
        },
    );
}

pub fn try_begin_spiked_hide_reflect(
    ctx: &GameCtx,
    defender_id: usize,
    attacker_id: usize,
) -> bool {
    if defender_id == attacker_id {
        return false;
    }

    let tick = ctx.tick();
    let reflects = SPIKED_HIDE_REFLECTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut reflects = reflects.lock().expect("spiked hide reflect state poisoned");
    reflects.retain(|(_, _, reflect_tick)| *reflect_tick == tick);

    if reflects.iter().any(|(defender, attacker, _)| {
        (*defender == defender_id && *attacker == attacker_id)
            || (*defender == attacker_id && *attacker == defender_id)
    }) {
        return false;
    }

    reflects.push((defender_id, attacker_id, tick));
    true
}

pub fn note_pyukumuku_barb_hit(ctx: &GameCtx, defender_id: usize, attacker_id: usize) -> usize {
    if defender_id == attacker_id {
        return 0;
    }

    let tick = ctx.tick();
    let states = PYUKUMUKU_BARBS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("pyukumuku barb state poisoned");
    states.retain(|state| {
        tick.saturating_sub(state.last_hit_tick) <= PYUKUMUKU_BARB_STACK_WINDOW_TICKS
            && ctx
                .get_entity(state.defender_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
            && ctx
                .get_entity(state.attacker_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
    });

    if let Some(state) = states
        .iter_mut()
        .find(|state| state.defender_id == defender_id && state.attacker_id == attacker_id)
    {
        state.stacks = state.stacks.saturating_add(1).min(5);
        state.last_hit_tick = tick;
        return state.stacks;
    }

    states.push(PyukumukuBarbState {
        defender_id,
        attacker_id,
        stacks: 1,
        last_hit_tick: tick,
    });
    1
}

pub fn note_direct_pokemon_damage(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    damage: usize,
    attack_type: AttackType,
) {
    if attacker_id == target_id || damage == 0 || matches!(attack_type, AttackType::Dot) {
        return;
    }

    let tick = ctx.tick();
    let ledger = DIRECT_DAMAGE_LEDGER.get_or_init(|| Mutex::new(Vec::new()));
    let mut ledger = ledger.lock().expect("direct damage ledger poisoned");
    ledger.retain(|state| tick.saturating_sub(state.tick) <= 2);
    ledger.push(DirectDamageLedgerState {
        attacker_id,
        target_id,
        damage,
        tick,
    });
    note_bouffalant_retaliate_damage(ctx, target_id, attacker_id, damage);
    maybe_trigger_missingno_negative_glitch(ctx, attacker_id, target_id, attack_type);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PokemonDamageResult {
    pub before: Option<(usize, usize)>,
    pub after: Option<(usize, usize)>,
    pub applied_damage: usize,
}

#[derive(Clone, Copy, Debug)]
struct CombatPlayerDeathSnapshot {
    player_id: usize,
    team: usize,
    deaths: usize,
    champion_id: Option<&'static str>,
}

pub fn deal_tracked_damage(
    ctx: &mut GameCtx,
    attacker_id: usize,
    target_id: usize,
    ad_damage: usize,
    ap_damage: usize,
    attack_type: AttackType,
) -> PokemonDamageResult {
    let before = combat_stat_damage_snapshot(ctx, target_id);
    let attacker_info = combat_stat_source_entity_info(ctx, attacker_id);
    let target_info = combat_stat_target_entity_info(ctx, target_id);
    let death_before = combat_player_death_snapshot(ctx);
    let kill_log_before = ctx.kill_log_count();
    ctx.deal_damage(attacker_id, target_id, ad_damage, ap_damage, attack_type);
    let death_after = combat_player_death_snapshot(ctx);
    let kill_log_after = ctx.kill_log_count();
    let after = combat_stat_damage_snapshot(ctx, target_id);
    let damage_credit = pokemon_damage_credit(ad_damage, ap_damage);
    let applied_damage = damage_credit;
    note_pokemon_damage_result_with_info(
        ctx,
        attacker_id,
        target_id,
        before,
        after,
        damage_credit,
        attacker_info,
        target_info,
        &death_before,
        &death_after,
    );
    if crate::crash_probe::stat_probe_enabled()
        && (kill_log_after > kill_log_before
            || combat_death_count_increased(&death_before, &death_after))
    {
        log_stat_probe_damage_transition(
            ctx,
            attacker_id,
            target_id,
            before,
            after,
            damage_credit,
            attacker_info,
            target_info,
            &death_before,
            &death_after,
            kill_log_before,
            kill_log_after,
        );
    }
    PokemonDamageResult {
        before,
        after,
        applied_damage,
    }
}

#[allow(dead_code)]
pub fn note_pokemon_damage_result(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    before: Option<(usize, usize)>,
    after: Option<(usize, usize)>,
) {
    note_pokemon_damage_result_with_info(
        ctx,
        attacker_id,
        target_id,
        before,
        after,
        applied_damage_between(before, after),
        combat_stat_source_entity_info(ctx, attacker_id),
        combat_stat_target_entity_info(ctx, target_id),
        &[],
        &[],
    );
}

fn note_pokemon_damage_result_with_info(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    before: Option<(usize, usize)>,
    after: Option<(usize, usize)>,
    damage_credit: usize,
    attacker_info: Option<CombatStatEntityInfo>,
    target_info: Option<CombatStatEntityInfo>,
    death_before: &[CombatPlayerDeathSnapshot],
    death_after: &[CombatPlayerDeathSnapshot],
) {
    if attacker_id == target_id {
        return;
    }
    let Some((before_hp, _before_shield)) = before else {
        return;
    };
    let after_hp = after.map(|(hp, _shield)| hp).unwrap_or(0);
    let applied_damage = damage_credit;
    if applied_damage == 0 {
        return;
    }
    let lethal_candidate =
        (before_hp > 0 && after_hp == 0) || combat_death_count_increased(death_before, death_after);
    if lethal_candidate {
        log_kda_lethal_candidate(
            ctx,
            attacker_id,
            target_id,
            before,
            after,
            applied_damage,
            attacker_info,
            target_info,
        );
    }

    let Some(attacker_info) = attacker_info else {
        if lethal_candidate {
            log_kda_credit_skip(
                ctx,
                "tracked_damage",
                "missing_attacker_info",
                attacker_id,
                target_id,
                attacker_info,
                target_info,
            );
        }
        return;
    };
    if !attacker_info.is_champion {
        return;
    }

    let target_info = target_info
        .or_else(|| recover_recent_damage_target_info(ctx, target_id, attacker_info.team))
        .or_else(|| recover_owner_target_info(ctx, target_id, attacker_info.team))
        .or_else(|| {
            if lethal_candidate {
                infer_killed_player_from_death_delta(death_before, death_after, attacker_info.team)
            } else {
                None
            }
        });
    let target_is_enemy_champion = target_info
        .map(|target_info| target_info.is_champion && target_info.team != attacker_info.team)
        .unwrap_or(false);

    if let Some(attacker_player_id) = attacker_info.player_id {
        let same_team = target_info
            .map(|target_info| target_info.team == attacker_info.team)
            .unwrap_or(false);
        if !same_team {
            add_pokemon_damage_dealt_for_player(ctx, attacker_player_id, applied_damage);
            refresh_pokemon_combat_stat_identity(
                ctx,
                attacker_player_id,
                Some(attacker_id),
                attacker_info.champion_id,
            );
        }
    }

    if let Some(target_info) = target_info {
        if target_is_enemy_champion {
            if let Some(target_player_id) = target_info.player_id {
                add_pokemon_damage_taken_for_player(ctx, target_player_id, applied_damage);
                refresh_pokemon_combat_stat_identity(
                    ctx,
                    target_player_id,
                    Some(target_id),
                    target_info.champion_id,
                );
            }
            record_pokemon_damage_credit(
                ctx,
                target_id,
                applied_damage,
                attacker_info,
                target_info,
            );

            if lethal_candidate {
                record_pokemon_kill_credit(
                    ctx,
                    attacker_id,
                    target_id,
                    attacker_info,
                    target_info,
                    "tracked_damage",
                );
            }
        } else if lethal_candidate {
            log_kda_credit_skip(
                ctx,
                "tracked_damage",
                "non_champion_or_same_team",
                attacker_id,
                target_id,
                Some(attacker_info),
                Some(target_info),
            );
        }
    } else if lethal_candidate {
        log_kda_credit_skip(
            ctx,
            "tracked_damage",
            "missing_target_info",
            attacker_id,
            target_id,
            Some(attacker_info),
            target_info,
        );
    }
}

fn combat_player_death_snapshot(ctx: &GameCtx) -> Vec<CombatPlayerDeathSnapshot> {
    (0..16)
        .filter_map(|player_id| {
            let player = ctx.get_player(player_id)?;
            let entity_id = entity_for_player_in_ctx(ctx, player_id);
            Some(CombatPlayerDeathSnapshot {
                player_id,
                team: player.team(),
                deaths: player.deaths(),
                champion_id: entity_id
                    .and_then(|entity_id| champion_id_for_entity_in_ctx(ctx, entity_id)),
            })
        })
        .collect()
}

fn combat_death_count_increased(
    before: &[CombatPlayerDeathSnapshot],
    after: &[CombatPlayerDeathSnapshot],
) -> bool {
    before.iter().any(|before_state| {
        after
            .iter()
            .find(|state| state.player_id == before_state.player_id)
            .map(|after_state| after_state.deaths > before_state.deaths)
            .unwrap_or(false)
    })
}

fn stat_probe_info_value(info: Option<CombatStatEntityInfo>) -> String {
    info.map(|info| {
        format!(
            "is_champ:{} player:{} life:{} team:{} champion:{}",
            info.is_champion,
            info.player_id
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string()),
            info.generation
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string()),
            info.team,
            info.champion_id.unwrap_or("unknown")
        )
    })
    .unwrap_or_else(|| "none".to_string())
}

fn stat_probe_damage_snapshot_value(value: Option<(usize, usize)>) -> String {
    value
        .map(|(hp, shield)| format!("hp:{} shield:{}", hp, shield))
        .unwrap_or_else(|| "none".to_string())
}

fn stat_probe_recent_kill_log_value(ctx: &GameCtx) -> String {
    let count = ctx.kill_log_count();
    if count == 0 {
        return "none".to_string();
    }
    (0..count)
        .rev()
        .take(4)
        .map(|index| {
            let entry = ctx.kill_log_at(index);
            format!(
                "idx:{} killer_team:{} killer_pos:{} killed_pos:{}",
                index,
                entry.killer_team,
                entry.killer_position as usize,
                entry.killed_position as usize
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn stat_probe_death_delta_value(
    before: &[CombatPlayerDeathSnapshot],
    after: &[CombatPlayerDeathSnapshot],
) -> String {
    let entries = before
        .iter()
        .filter_map(|before_state| {
            let after_state = after
                .iter()
                .find(|state| state.player_id == before_state.player_id)?;
            if after_state.deaths <= before_state.deaths {
                return None;
            }
            Some(format!(
                "player:{} team:{} champion:{} deaths:{}->{}",
                before_state.player_id,
                before_state.team,
                before_state.champion_id.unwrap_or("unknown"),
                before_state.deaths,
                after_state.deaths
            ))
        })
        .collect::<Vec<_>>();
    if entries.is_empty() {
        "none".to_string()
    } else {
        entries.join(" | ")
    }
}

fn stat_probe_native_kda_value(ctx: &GameCtx, player_id: Option<usize>) -> String {
    let Some(player_id) = player_id else {
        return "none".to_string();
    };
    let Some(player) = ctx.get_player(player_id) else {
        return "missing_player".to_string();
    };
    format!(
        "player:{} kills:{} deaths:{} assists:{} team:{}",
        player_id,
        player.kills(),
        player.deaths(),
        player.assists(),
        player.team()
    )
}

fn stat_probe_custom_kda_value(ctx: &GameCtx, player_id: Option<usize>) -> String {
    let Some(player_id) = player_id else {
        return "none".to_string();
    };
    let ctx_id = combat_ctx_id(ctx);
    let Some(states) = POKEMON_COMBAT_STATS.get() else {
        return "none".to_string();
    };
    let Ok(states) = states.lock() else {
        return "poisoned".to_string();
    };
    states
        .iter()
        .find(|state| state.ctx_id == ctx_id && state.player_id == player_id)
        .map(|state| {
            format!(
                "player:{} entity:{} champion:{} kills:{} deaths:{} assists:{} damage_dealt:{} damage_taken:{} healing:{} tick:{}",
                state.player_id,
                state.entity_id,
                state.champion_id.unwrap_or("unknown"),
                state.kills,
                state.deaths,
                state.assists,
                state.damage_dealt,
                state.damage_taken,
                state.healing_done,
                state.last_seen_tick
            )
        })
        .unwrap_or_else(|| "none".to_string())
}

fn log_stat_probe_damage_transition(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    before: Option<(usize, usize)>,
    after: Option<(usize, usize)>,
    damage_credit: usize,
    attacker_info: Option<CombatStatEntityInfo>,
    target_info: Option<CombatStatEntityInfo>,
    death_before: &[CombatPlayerDeathSnapshot],
    death_after: &[CombatPlayerDeathSnapshot],
    kill_log_before: usize,
    kill_log_after: usize,
) {
    crate::crash_probe::log_stat_probe_event(&format!(
        "event=damage_transition tick={} ctx={} attacker={} target={} damage_credit={} before=\"{}\" after=\"{}\" attacker_info=\"{}\" target_info=\"{}\" death_delta=\"{}\" kill_log={}->{} attacker_native=\"{}\" attacker_custom=\"{}\" target_native=\"{}\" target_custom=\"{}\"",
        ctx.tick(),
        combat_ctx_id(ctx),
        attacker_id,
        target_id,
        damage_credit,
        crate::crash_probe::sanitize_log_field(&stat_probe_damage_snapshot_value(before)),
        crate::crash_probe::sanitize_log_field(&stat_probe_damage_snapshot_value(after)),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(attacker_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(target_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_death_delta_value(death_before, death_after)),
        kill_log_before,
        kill_log_after,
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            attacker_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            attacker_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            target_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            target_info.and_then(|info| info.player_id)
        )),
    ));
}

fn infer_killed_player_from_death_delta(
    before: &[CombatPlayerDeathSnapshot],
    after: &[CombatPlayerDeathSnapshot],
    attacker_team: usize,
) -> Option<CombatStatEntityInfo> {
    let mut matches = Vec::new();
    for before_state in before {
        if before_state.team == attacker_team {
            continue;
        }
        let Some(after_state) = after
            .iter()
            .find(|state| state.player_id == before_state.player_id)
        else {
            continue;
        };
        if after_state.deaths > before_state.deaths {
            matches.push(CombatStatEntityInfo {
                is_champion: true,
                player_id: Some(before_state.player_id),
                generation: Some(before_state.deaths),
                team: before_state.team,
                champion_id: before_state.champion_id,
            });
        }
    }

    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
}

fn combat_stat_damage_snapshot(ctx: &GameCtx, entity_id: usize) -> Option<(usize, usize)> {
    ctx.get_entity(entity_id)
        .map(|entity| (entity.hp().current, entity.shield()))
}

fn applied_damage_between(before: Option<(usize, usize)>, after: Option<(usize, usize)>) -> usize {
    let Some((before_hp, before_shield)) = before else {
        return 0;
    };
    let Some((after_hp, after_shield)) = after else {
        return before_hp.saturating_add(before_shield);
    };

    before_hp
        .saturating_sub(after_hp)
        .saturating_add(before_shield.saturating_sub(after_shield))
}

fn pokemon_damage_credit(ad_damage: usize, ap_damage: usize) -> usize {
    ad_damage.saturating_add(ap_damage)
}

#[derive(Clone, Copy, Debug)]
struct CombatStatEntityInfo {
    is_champion: bool,
    player_id: Option<usize>,
    generation: Option<usize>,
    team: usize,
    champion_id: Option<&'static str>,
}

fn combat_stat_source_entity_info(ctx: &GameCtx, entity_id: usize) -> Option<CombatStatEntityInfo> {
    let tick = ctx.tick();
    if let Some(entity) = ctx.get_entity(entity_id) {
        let owner = owner_for_entity_at_tick_on_team(ctx, entity_id, tick, entity.team());
        let player_life =
            player_life_for_entity_at_tick_on_team(ctx, entity_id, tick, entity.team());
        let player_id = player_life
            .map(|(player_id, _life_id)| player_id)
            .or_else(|| owner.map(|state| state.player_id));
        let generation = player_life
            .map(|(_player_id, life_id)| life_id)
            .or_else(|| {
                player_id
                    .and_then(|player_id| ctx.get_player(player_id).map(|player| player.deaths()))
            })
            .or_else(|| owner.map(|state| state.life_id));
        let champion_id = champion_id_for_entity_in_ctx(ctx, entity_id)
            .or_else(|| owner.and_then(|state| state.champion_id));
        let team = if entity.is_champion() {
            entity.team()
        } else {
            player_id
                .and_then(|player_id| ctx.get_player(player_id).map(|player| player.team()))
                .unwrap_or_else(|| entity.team())
        };
        return Some(CombatStatEntityInfo {
            is_champion: entity.is_champion() || player_id.is_some(),
            player_id,
            generation,
            team,
            champion_id,
        });
    }

    let owner = owner_for_entity_at_tick_in_ctx(ctx, entity_id, tick);
    owner.and_then(|owner| {
        let team = ctx
            .get_player(owner.player_id)
            .map(|player| player.team())?;
        Some(CombatStatEntityInfo {
            is_champion: true,
            player_id: Some(owner.player_id),
            generation: Some(owner.life_id),
            team,
            champion_id: owner.champion_id,
        })
    })
}

fn combat_stat_target_entity_info(ctx: &GameCtx, entity_id: usize) -> Option<CombatStatEntityInfo> {
    let tick = ctx.tick();
    let entity = ctx.get_entity(entity_id)?;
    if !entity.is_champion() {
        return None;
    }

    let owner = owner_for_entity_at_tick_on_team(ctx, entity_id, tick, entity.team());
    let player_life = player_life_for_entity_at_tick_on_team(ctx, entity_id, tick, entity.team());
    let player_id = player_life
        .map(|(player_id, _life_id)| player_id)
        .or_else(|| owner.map(|state| state.player_id))?;
    let generation = player_life
        .map(|(_player_id, life_id)| life_id)
        .or_else(|| ctx.get_player(player_id).map(|player| player.deaths()))
        .or_else(|| owner.map(|state| state.life_id))?;
    Some(CombatStatEntityInfo {
        is_champion: true,
        player_id: Some(player_id),
        generation: Some(generation),
        team: entity.team(),
        champion_id: champion_id_for_entity_in_ctx(ctx, entity_id)
            .or_else(|| owner.and_then(|state| state.champion_id)),
    })
}

fn record_pokemon_damage_credit(
    ctx: &GameCtx,
    target_id: usize,
    damage: usize,
    attacker_info: CombatStatEntityInfo,
    target_info: CombatStatEntityInfo,
) {
    let (Some(attacker_player_id), Some(target_player_id), Some(target_generation)) = (
        attacker_info.player_id,
        target_info.player_id,
        target_info.generation,
    ) else {
        return;
    };
    let ctx_id = combat_ctx_id(ctx);
    let tick = ctx.tick();
    let credits = POKEMON_DAMAGE_CREDITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut credits = credits
        .lock()
        .expect("pokemon damage credit state poisoned");
    credits.retain(|state| {
        state.ctx_id != ctx_id || recent_tick(tick, state.tick, POKEMON_DAMAGE_ASSIST_WINDOW_TICKS)
    });
    credits.push(PokemonDamageCreditState {
        ctx_id,
        attacker_player_id,
        attacker_team: attacker_info.team,
        target_id,
        target_team: target_info.team,
        target_player_id,
        target_generation,
        target_champion_id: target_info.champion_id,
        damage,
        tick,
    });
    while credits.len() > POKEMON_CONTEXT_LEDGER_MAX {
        credits.remove(0);
    }
    record_pokemon_enemy_participation(
        ctx,
        attacker_info,
        target_info,
        PokemonParticipationKind::EnemyDamage,
    );
}

fn record_pokemon_enemy_participation(
    ctx: &GameCtx,
    participant_info: CombatStatEntityInfo,
    subject_info: CombatStatEntityInfo,
    kind: PokemonParticipationKind,
) {
    let (Some(participant_player_id), Some(subject_player_id)) =
        (participant_info.player_id, subject_info.player_id)
    else {
        return;
    };
    if !participant_info.is_champion
        || !subject_info.is_champion
        || participant_player_id == subject_player_id
        || participant_info.team == subject_info.team
    {
        return;
    }
    record_pokemon_participation(
        ctx,
        participant_player_id,
        participant_info.team,
        subject_player_id,
        subject_info.generation,
        subject_info.team,
        kind,
    );
}

fn record_pokemon_ally_participation(
    ctx: &GameCtx,
    participant_info: CombatStatEntityInfo,
    subject_info: CombatStatEntityInfo,
    kind: PokemonParticipationKind,
) {
    let (Some(participant_player_id), Some(subject_player_id)) =
        (participant_info.player_id, subject_info.player_id)
    else {
        return;
    };
    if !participant_info.is_champion
        || !subject_info.is_champion
        || participant_player_id == subject_player_id
        || participant_info.team != subject_info.team
    {
        return;
    }
    record_pokemon_participation(
        ctx,
        participant_player_id,
        participant_info.team,
        subject_player_id,
        subject_info.generation,
        subject_info.team,
        kind,
    );
}

fn record_pokemon_participation(
    ctx: &GameCtx,
    participant_player_id: usize,
    participant_team: usize,
    subject_player_id: usize,
    subject_generation: Option<usize>,
    subject_team: usize,
    kind: PokemonParticipationKind,
) {
    let ctx_id = combat_ctx_id(ctx);
    let tick = ctx.tick();
    let participations = POKEMON_PARTICIPATIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut participations = participations
        .lock()
        .expect("pokemon participation state poisoned");
    participations.retain(|state| {
        state.ctx_id != ctx_id
            || recent_tick(tick, state.tick, POKEMON_PARTICIPATION_ASSIST_WINDOW_TICKS)
    });

    if let Some(state) = participations.iter_mut().find(|state| {
        state.ctx_id == ctx_id
            && state.participant_player_id == participant_player_id
            && state.subject_player_id == subject_player_id
            && state.kind == kind
    }) {
        state.participant_team = participant_team;
        state.subject_generation = subject_generation;
        state.subject_team = subject_team;
        state.tick = tick;
        return;
    }

    participations.push(PokemonParticipationState {
        ctx_id,
        participant_player_id,
        participant_team,
        subject_player_id,
        subject_generation,
        subject_team,
        kind,
        tick,
    });
    while participations.len() > POKEMON_PARTICIPATION_LEDGER_MAX {
        participations.remove(0);
    }
}

fn recover_recent_damage_target_info(
    ctx: &GameCtx,
    target_id: usize,
    attacker_team: usize,
) -> Option<CombatStatEntityInfo> {
    let ctx_id = combat_ctx_id(ctx);
    let tick = ctx.tick();
    let credits = POKEMON_DAMAGE_CREDITS.get_or_init(|| Mutex::new(Vec::new()));
    let credits = credits
        .lock()
        .expect("pokemon damage credit state poisoned");
    credits
        .iter()
        .rev()
        .find(|state| {
            state.ctx_id == ctx_id
                && state.target_id == target_id
                && state.target_team != attacker_team
                && recent_tick(tick, state.tick, POKEMON_DAMAGE_ASSIST_WINDOW_TICKS)
        })
        .map(|state| CombatStatEntityInfo {
            is_champion: true,
            player_id: Some(state.target_player_id),
            generation: Some(state.target_generation),
            team: state.target_team,
            champion_id: state.target_champion_id,
        })
}

fn recover_owner_target_info(
    ctx: &GameCtx,
    target_id: usize,
    attacker_team: usize,
) -> Option<CombatStatEntityInfo> {
    let owner = owner_for_entity_at_tick_in_ctx(ctx, target_id, ctx.tick())?;
    let team = ctx
        .get_player(owner.player_id)
        .map(|player| player.team())?;
    if team == attacker_team {
        return None;
    }
    Some(CombatStatEntityInfo {
        is_champion: true,
        player_id: Some(owner.player_id),
        generation: Some(owner.life_id),
        team,
        champion_id: champion_id_for_entity_in_ctx(ctx, target_id).or(owner.champion_id),
    })
}

fn combat_stat_player_info(ctx: &GameCtx, player_id: usize) -> Option<CombatStatEntityInfo> {
    let entity_id =
        entity_for_player_in_ctx(ctx, player_id).or_else(|| entity_for_player(player_id));
    if let Some(entity_id) = entity_id {
        if let Some(mut info) = combat_stat_source_entity_info(ctx, entity_id) {
            info.is_champion = true;
            info.player_id = Some(player_id);
            return Some(info);
        }
    }

    let player = ctx.get_player(player_id)?;
    Some(CombatStatEntityInfo {
        is_champion: true,
        player_id: Some(player_id),
        generation: Some(player.deaths()),
        team: player.team(),
        champion_id: entity_id.and_then(|entity_id| champion_id_for_entity_in_ctx(ctx, entity_id)),
    })
}

fn killed_entity_info_for_credit(
    ctx: &GameCtx,
    killed_entity_id: usize,
    killer_team: usize,
) -> Option<CombatStatEntityInfo> {
    let tick = ctx.tick();
    if let Some(entity) = ctx.get_entity(killed_entity_id) {
        let target_team = entity.team();
        let target_is_champion = entity.is_champion();
        drop(entity);

        if !target_is_champion || target_team == killer_team {
            return None;
        }

        let owner = owner_for_entity_at_tick_on_team(ctx, killed_entity_id, tick, target_team)
            .or_else(|| owner_for_entity_at_tick_in_ctx(ctx, killed_entity_id, tick));
        let player_life =
            player_life_for_entity_at_tick_on_team(ctx, killed_entity_id, tick, target_team);
        let player_id = player_life
            .map(|(player_id, _life_id)| player_id)
            .or_else(|| owner.map(|state| state.player_id))?;
        let generation = player_life
            .map(|(_player_id, life_id)| life_id)
            .or_else(|| owner.map(|state| state.life_id))
            .or_else(|| {
                ctx.get_player(player_id)
                    .map(|player| player.deaths().saturating_sub(1))
            })?;

        return Some(CombatStatEntityInfo {
            is_champion: true,
            player_id: Some(player_id),
            generation: Some(generation),
            team: target_team,
            champion_id: champion_id_for_entity_in_ctx(ctx, killed_entity_id)
                .or_else(|| owner.and_then(|state| state.champion_id)),
        });
    }

    recover_owner_target_info(ctx, killed_entity_id, killer_team)
}

fn record_pokemon_kill_credit(
    ctx: &GameCtx,
    killer_id: usize,
    killed_id: usize,
    killer_info: CombatStatEntityInfo,
    killed_info: CombatStatEntityInfo,
    origin: &str,
) {
    if !killer_info.is_champion || !killed_info.is_champion {
        log_kda_credit_skip(
            ctx,
            origin,
            "non_champion",
            killer_id,
            killed_id,
            Some(killer_info),
            Some(killed_info),
        );
        return;
    }

    let (Some(killer_player_id), Some(killed_player_id), Some(killed_generation)) = (
        killer_info.player_id,
        killed_info.player_id,
        killed_info.generation,
    ) else {
        log_kda_credit_skip(
            ctx,
            origin,
            "missing_player_or_life",
            killer_id,
            killed_id,
            Some(killer_info),
            Some(killed_info),
        );
        return;
    };
    if killer_player_id == killed_player_id || killer_info.team == killed_info.team {
        log_kda_credit_skip(
            ctx,
            origin,
            "same_player_or_team",
            killer_id,
            killed_id,
            Some(killer_info),
            Some(killed_info),
        );
        return;
    }
    let tick = ctx.tick();
    let ctx_id = combat_ctx_id(ctx);
    let kills = POKEMON_KILL_CREDITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut kills = kills.lock().expect("pokemon kill credit state poisoned");
    kills.retain(|state| {
        state.ctx_id != ctx_id || recent_tick(tick, state.tick, POKEMON_KILL_CREDIT_HISTORY_TICKS)
    });
    if kills.iter().any(|state| {
        state.ctx_id == ctx_id
            && state.killed_player_id == killed_player_id
            && state.killed_generation == killed_generation
    }) {
        log_kda_credit_skip(
            ctx,
            origin,
            "duplicate_life",
            killer_id,
            killed_id,
            Some(killer_info),
            Some(killed_info),
        );
        return;
    }

    add_pokemon_kill_for_player(ctx, killer_player_id);
    refresh_pokemon_combat_stat_identity(
        ctx,
        killer_player_id,
        Some(killer_id),
        killer_info.champion_id,
    );
    add_pokemon_death_for_player(ctx, killed_player_id);
    refresh_pokemon_combat_stat_identity(
        ctx,
        killed_player_id,
        Some(killed_id),
        killed_info.champion_id,
    );

    kills.push(PokemonKillCreditState {
        ctx_id,
        killer_id,
        killer_player_id,
        killed_id,
        killed_player_id,
        killed_generation,
        assist_ids: Vec::new(),
        tick,
    });
    while kills.len() > POKEMON_KILL_LEDGER_MAX {
        kills.remove(0);
    }
    drop(kills);

    let assist_ids = crate::crash_probe::catch_unwind_probe(
        "pokemon_assist_select",
        format!(
            "ctx={} killer_player={} killed_player={} killed_life={}",
            ctx_id, killer_player_id, killed_player_id, killed_generation
        ),
        Vec::new(),
        || {
            pokemon_assist_credit_ids(
                ctx,
                killer_player_id,
                killed_player_id,
                killed_generation,
                killer_info.team,
                tick,
            )
        },
    );
    for assist_player_id in &assist_ids {
        crate::crash_probe::catch_unwind_probe(
            "pokemon_assist_award",
            format!(
                "ctx={} assist_player={} killed_player={} killed_life={}",
                ctx_id, assist_player_id, killed_player_id, killed_generation
            ),
            (),
            || {
                award_pokemon_assist_once(
                    ctx,
                    *assist_player_id,
                    killed_id,
                    Some(killed_player_id),
                    Some(killed_generation),
                    tick,
                );
            },
        );
    }

    if !assist_ids.is_empty() {
        let kills = POKEMON_KILL_CREDITS.get_or_init(|| Mutex::new(Vec::new()));
        if let Ok(mut kills) = kills.lock() {
            if let Some(state) = kills.iter_mut().rev().find(|state| {
                state.ctx_id == ctx_id
                    && state.killed_player_id == killed_player_id
                    && state.killed_generation == killed_generation
            }) {
                state.assist_ids = assist_ids.clone();
            }
        }
    }

    let killer_champion = killer_info.champion_id.unwrap_or("unknown");
    let killed_champion = killed_info.champion_id.unwrap_or("unknown");
    crate::crash_probe::log_kda_probe(&format!(
        "event=kda_kill_credit origin={} tick={} ctx={} killer={} killer_player={} killer_champion=\"{}\" killed={} killed_player={} killed_life={} killed_champion=\"{}\" assists=\"{}\" killer_native=\"{}\" killer_custom=\"{}\" killed_native=\"{}\" killed_custom=\"{}\" recent_credits=\"{}\"",
        crate::crash_probe::sanitize_log_field(origin),
        tick,
        combat_ctx_id(ctx),
        killer_id,
        killer_player_id,
        crate::crash_probe::sanitize_log_field(killer_champion),
        killed_id,
        killed_player_id,
        killed_generation,
        crate::crash_probe::sanitize_log_field(killed_champion),
        assist_ids
            .iter()
            .map(|assist_id| assist_id.to_string())
            .collect::<Vec<_>>()
            .join(","),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            Some(killer_player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            Some(killer_player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            Some(killed_player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            Some(killed_player_id)
        )),
        crate::crash_probe::sanitize_log_field(&kda_probe_recent_credits(
            ctx,
            tick,
            killed_id,
            Some(killed_player_id),
            Some(killed_generation),
        )),
    ));
}

fn log_kda_lethal_candidate(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    before: Option<(usize, usize)>,
    after: Option<(usize, usize)>,
    applied_damage: usize,
    attacker_info: Option<CombatStatEntityInfo>,
    target_info: Option<CombatStatEntityInfo>,
) {
    crate::crash_probe::log_stat_probe_event(&format!(
        "event=kda_lethal_candidate tick={} ctx={} attacker={} target={} applied_damage={} before=\"{}\" after=\"{}\" attacker_info=\"{}\" target_info=\"{}\" attacker_native=\"{}\" attacker_custom=\"{}\" target_native=\"{}\" target_custom=\"{}\"",
        ctx.tick(),
        combat_ctx_id(ctx),
        attacker_id,
        target_id,
        applied_damage,
        crate::crash_probe::sanitize_log_field(&stat_probe_damage_snapshot_value(before)),
        crate::crash_probe::sanitize_log_field(&stat_probe_damage_snapshot_value(after)),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(attacker_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(target_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            attacker_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            attacker_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            target_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            target_info.and_then(|info| info.player_id)
        )),
    ));
}

fn log_kda_credit_skip(
    ctx: &GameCtx,
    origin: &str,
    reason: &str,
    killer_id: usize,
    killed_id: usize,
    killer_info: Option<CombatStatEntityInfo>,
    killed_info: Option<CombatStatEntityInfo>,
) {
    crate::crash_probe::log_stat_probe_event(&format!(
        "event=kda_credit_skip origin={} reason={} tick={} ctx={} killer={} killed={} killer_info=\"{}\" killed_info=\"{}\" killer_native=\"{}\" killer_custom=\"{}\" killed_native=\"{}\" killed_custom=\"{}\" recent_credits=\"{}\"",
        crate::crash_probe::sanitize_log_field(origin),
        crate::crash_probe::sanitize_log_field(reason),
        ctx.tick(),
        combat_ctx_id(ctx),
        killer_id,
        killed_id,
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(killer_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(killed_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            killer_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            killer_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            killed_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            killed_info.and_then(|info| info.player_id)
        )),
        crate::crash_probe::sanitize_log_field(&kda_probe_recent_credits(
            ctx,
            ctx.tick(),
            killed_id,
            killed_info.and_then(|info| info.player_id),
            killed_info.and_then(|info| info.generation),
        )),
    ));
}

fn kda_probe_recent_credits(
    ctx: &GameCtx,
    tick: usize,
    target_id: usize,
    target_player_id: Option<usize>,
    target_generation: Option<usize>,
) -> String {
    let ctx_id = combat_ctx_id(ctx);
    let Some(credits) = POKEMON_DAMAGE_CREDITS.get() else {
        return "none".to_string();
    };
    let Ok(credits) = credits.lock() else {
        return "poisoned".to_string();
    };
    let entries = credits
        .iter()
        .rev()
        .filter(|state| {
            state.ctx_id == ctx_id
                && recent_tick(tick, state.tick, POKEMON_DAMAGE_ASSIST_WINDOW_TICKS)
                && (state.target_id == target_id
                    || (target_player_id == Some(state.target_player_id)
                        && target_generation == Some(state.target_generation)))
        })
        .take(8)
        .map(|state| {
            format!(
                "attacker_player:{} team:{} target:{} target_player:{} life:{} damage:{} age:{}",
                state.attacker_player_id,
                state.attacker_team,
                state.target_id,
                state.target_player_id,
                state.target_generation,
                state.damage,
                tick.saturating_sub(state.tick)
            )
        })
        .collect::<Vec<_>>();
    if entries.is_empty() {
        "none".to_string()
    } else {
        entries.join(" | ")
    }
}

pub fn record_native_pokemon_kill_assists(
    ctx: &GameCtx,
    killer_player_id: usize,
    killed_entity_id: usize,
) {
    let Some(killer_info) = combat_stat_player_info(ctx, killer_player_id) else {
        crate::crash_probe::log_stat_probe_event(&format!(
            "event=native_on_kill_skip reason=missing_killer_info tick={} ctx={} killer_player={} killed_entity={} recent_kill_log=\"{}\"",
            ctx.tick(),
            combat_ctx_id(ctx),
            killer_player_id,
            killed_entity_id,
            crate::crash_probe::sanitize_log_field(&stat_probe_recent_kill_log_value(ctx)),
        ));
        return;
    };
    let direct_killed_info = killed_entity_info_for_credit(ctx, killed_entity_id, killer_info.team);
    let fallback_killed_info = if direct_killed_info.is_some() {
        None
    } else {
        killed_info_from_latest_kill_log(ctx, killer_player_id, killer_info)
    };
    let killed_info = direct_killed_info.or(fallback_killed_info);
    crate::crash_probe::log_stat_probe_event(&format!(
        "event=native_on_kill_seen tick={} ctx={} killer_player={} killed_entity={} killer_info=\"{}\" direct_killed_info=\"{}\" fallback_killed_info=\"{}\" recent_kill_log=\"{}\" killer_native=\"{}\" killer_custom=\"{}\"",
        ctx.tick(),
        combat_ctx_id(ctx),
        killer_player_id,
        killed_entity_id,
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(Some(killer_info))),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(direct_killed_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_info_value(fallback_killed_info)),
        crate::crash_probe::sanitize_log_field(&stat_probe_recent_kill_log_value(ctx)),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            killer_info.player_id
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            killer_info.player_id
        )),
    ));
    let Some(killed_info) = killed_info else {
        crate::crash_probe::log_stat_probe_event(&format!(
            "event=native_on_kill_skip reason=missing_killed_info tick={} ctx={} killer_player={} killed_entity={} killer_info=\"{}\" recent_kill_log=\"{}\"",
            ctx.tick(),
            combat_ctx_id(ctx),
            killer_player_id,
            killed_entity_id,
            crate::crash_probe::sanitize_log_field(&stat_probe_info_value(Some(killer_info))),
            crate::crash_probe::sanitize_log_field(&stat_probe_recent_kill_log_value(ctx)),
        ));
        return;
    };
    let killer_id = entity_for_player_in_ctx(ctx, killer_player_id)
        .or_else(|| entity_for_player(killer_player_id))
        .unwrap_or(killer_player_id);

    record_pokemon_kill_credit(
        ctx,
        killer_id,
        killed_entity_id,
        killer_info,
        killed_info,
        "native_on_kill",
    );
}

fn killed_info_from_latest_kill_log(
    ctx: &GameCtx,
    killer_player_id: usize,
    killer_info: CombatStatEntityInfo,
) -> Option<CombatStatEntityInfo> {
    let killer_position = ctx
        .get_player(killer_player_id)
        .map(|player| player.position() as usize)?;
    let kill_log_count = ctx.kill_log_count();
    if kill_log_count == 0 {
        return None;
    }

    for index in (0..kill_log_count).rev().take(8) {
        let entry = ctx.kill_log_at(index);
        if entry.killer_team != killer_info.team
            || entry.killer_position as usize != killer_position
        {
            continue;
        }

        for player_id in 0..16 {
            let Some(player) = ctx.get_player(player_id) else {
                continue;
            };
            if player.team() == killer_info.team
                || player.position() as usize != entry.killed_position as usize
            {
                continue;
            }
            let entity_id = entity_for_player_in_ctx(ctx, player_id);
            return Some(CombatStatEntityInfo {
                is_champion: true,
                player_id: Some(player_id),
                generation: Some(player.deaths().saturating_sub(1)),
                team: player.team(),
                champion_id: entity_id
                    .and_then(|entity_id| champion_id_for_entity_in_ctx(ctx, entity_id)),
            });
        }
    }

    None
}

fn pokemon_assist_credit_ids(
    ctx: &GameCtx,
    killer_player_id: usize,
    killed_player_id: usize,
    killed_generation: usize,
    killer_team: usize,
    tick: usize,
) -> Vec<usize> {
    let ctx_id = combat_ctx_id(ctx);
    let credits = POKEMON_DAMAGE_CREDITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut credits = credits
        .lock()
        .expect("pokemon damage credit state poisoned");
    credits.retain(|state| {
        state.ctx_id != ctx_id || recent_tick(tick, state.tick, POKEMON_DAMAGE_ASSIST_WINDOW_TICKS)
    });

    let mut assist_ids = Vec::new();
    let mut target_participant_ids = Vec::new();
    for state in credits.iter() {
        if state.ctx_id != ctx_id
            || state.target_player_id != killed_player_id
            || state.target_generation != killed_generation
            || state.attacker_team != killer_team
            || state.damage == 0
        {
            continue;
        }
        if !target_participant_ids
            .iter()
            .any(|id| *id == state.attacker_player_id)
        {
            target_participant_ids.push(state.attacker_player_id);
        }
        if state.attacker_player_id == killer_player_id {
            continue;
        }
        if !assist_ids.iter().any(|id| *id == state.attacker_player_id) {
            assist_ids.push(state.attacker_player_id);
        }
    }
    drop(credits);

    let participations = POKEMON_PARTICIPATIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut participations = participations
        .lock()
        .expect("pokemon participation state poisoned");
    participations.retain(|state| {
        state.ctx_id != ctx_id
            || recent_tick(tick, state.tick, POKEMON_PARTICIPATION_ASSIST_WINDOW_TICKS)
    });
    for state in participations.iter() {
        if state.ctx_id != ctx_id || state.participant_team != killer_team {
            continue;
        }

        if !matches!(
            state.kind,
            PokemonParticipationKind::EnemyDamage | PokemonParticipationKind::EnemyHarmfulBuff
        ) || state.subject_player_id != killed_player_id
            || state.subject_team == killer_team
        {
            continue;
        }

        if !target_participant_ids
            .iter()
            .any(|id| *id == state.participant_player_id)
        {
            target_participant_ids.push(state.participant_player_id);
        }
        if state.participant_player_id != killer_player_id
            && !assist_ids
                .iter()
                .any(|id| *id == state.participant_player_id)
        {
            assist_ids.push(state.participant_player_id);
        }
    }

    for state in participations.iter() {
        if state.ctx_id != ctx_id
            || state.participant_player_id == killer_player_id
            || state.participant_team != killer_team
            || state.subject_team != killer_team
            || !matches!(
                state.kind,
                PokemonParticipationKind::AllyBeneficialBuff
                    | PokemonParticipationKind::AllyHealing
            )
        {
            continue;
        }

        let helped_killer = state.subject_player_id == killer_player_id;
        let helped_target_participant = target_participant_ids
            .iter()
            .any(|id| *id == state.subject_player_id);
        if (helped_killer || helped_target_participant)
            && !assist_ids
                .iter()
                .any(|id| *id == state.participant_player_id)
        {
            assist_ids.push(state.participant_player_id);
        }
    }
    assist_ids
}

fn recent_tick(current_tick: usize, event_tick: usize, window: usize) -> bool {
    current_tick >= event_tick && current_tick.saturating_sub(event_tick) <= window
}

fn award_pokemon_assist_once(
    ctx: &GameCtx,
    assist_player_id: usize,
    killed_id: usize,
    killed_player_id: Option<usize>,
    killed_generation: Option<usize>,
    tick: usize,
) {
    let ctx_id = combat_ctx_id(ctx);
    let awards = POKEMON_ASSIST_AWARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut awards = awards.lock().expect("pokemon assist award state poisoned");
    awards.retain(|state| {
        state.ctx_id != ctx_id || recent_tick(tick, state.tick, POKEMON_KILL_CREDIT_HISTORY_TICKS)
    });
    if awards.iter().any(|state| {
        state.ctx_id == ctx_id
            && state.assist_player_id == assist_player_id
            && (state.killed_id == killed_id
                || (state.killed_player_id == killed_player_id
                    && state.killed_generation == killed_generation
                    && killed_player_id.is_some()
                    && killed_generation.is_some()))
    }) {
        crate::crash_probe::log_stat_probe_event(&format!(
            "event=kda_assist_skip reason=duplicate tick={} ctx={} assist_player={} killed={} killed_player={} killed_life={} assist_native=\"{}\" assist_custom=\"{}\"",
            tick,
            ctx_id,
            assist_player_id,
            killed_id,
            killed_player_id
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string()),
            killed_generation
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string()),
            crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
                ctx,
                Some(assist_player_id)
            )),
            crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
                ctx,
                Some(assist_player_id)
            )),
        ));
        return;
    }
    awards.push(PokemonAssistAwardState {
        ctx_id,
        assist_player_id,
        killed_id,
        killed_player_id,
        killed_generation,
        tick,
    });
    while awards.len() > POKEMON_ASSIST_LEDGER_MAX {
        awards.remove(0);
    }
    drop(awards);

    add_pokemon_assist_for_player(ctx, assist_player_id);
    refresh_pokemon_combat_stat_identity(
        ctx,
        assist_player_id,
        entity_for_player_in_ctx(ctx, assist_player_id)
            .or_else(|| entity_for_player(assist_player_id)),
        None,
    );
    crate::crash_probe::log_stat_probe_event(&format!(
        "event=kda_assist_credit tick={} ctx={} assist_player={} killed={} killed_player={} killed_life={} assist_native=\"{}\" assist_custom=\"{}\"",
        tick,
        ctx_id,
        assist_player_id,
        killed_id,
        killed_player_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        killed_generation
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        crate::crash_probe::sanitize_log_field(&stat_probe_native_kda_value(
            ctx,
            Some(assist_player_id)
        )),
        crate::crash_probe::sanitize_log_field(&stat_probe_custom_kda_value(
            ctx,
            Some(assist_player_id)
        )),
    ));
}

fn add_pokemon_damage_dealt_for_player(ctx: &GameCtx, player_id: usize, amount: usize) {
    with_pokemon_combat_stat_for_player_mut(ctx, player_id, |state| {
        state.damage_dealt = state.damage_dealt.saturating_add(amount);
    });
    sync_pokemon_player_stats_to_base_player(ctx, player_id);
}

fn add_pokemon_damage_taken_for_player(ctx: &GameCtx, player_id: usize, amount: usize) {
    with_pokemon_combat_stat_for_player_mut(ctx, player_id, |state| {
        state.damage_taken = state.damage_taken.saturating_add(amount);
    });
    sync_pokemon_player_stats_to_base_player(ctx, player_id);
}

fn add_pokemon_healing_for_player(ctx: &GameCtx, player_id: usize, amount: usize) {
    with_pokemon_combat_stat_for_player_mut(ctx, player_id, |state| {
        state.healing_done = state.healing_done.saturating_add(amount);
    });
    sync_pokemon_player_stats_to_base_player(ctx, player_id);
}

fn add_pokemon_kill_for_player(ctx: &GameCtx, player_id: usize) {
    with_pokemon_combat_stat_for_player_mut(ctx, player_id, |state| {
        state.kills = state.kills.saturating_add(1);
    });
    sync_pokemon_player_stats_to_base_player(ctx, player_id);
}

fn add_pokemon_death_for_player(ctx: &GameCtx, player_id: usize) {
    with_pokemon_combat_stat_for_player_mut(ctx, player_id, |state| {
        state.deaths = state.deaths.saturating_add(1);
    });
}

fn add_pokemon_assist_for_player(ctx: &GameCtx, player_id: usize) {
    with_pokemon_combat_stat_for_player_mut(ctx, player_id, |state| {
        state.assists = state.assists.saturating_add(1);
    });
    sync_pokemon_player_stats_to_base_player(ctx, player_id);
}

fn sync_pokemon_player_stats_to_base_player(ctx: &GameCtx, player_id: usize) {
    let ctx_id = combat_ctx_id(ctx);
    let Some(custom_stats) = pokemon_combat_stats_for_ctx_player(ctx_id, player_id) else {
        return;
    };

    let sync_states = POKEMON_BASE_STAT_SYNC.get_or_init(|| Mutex::new(Vec::new()));
    let mut sync_states = sync_states
        .lock()
        .expect("pokemon base stat sync state poisoned");
    let applied_index = sync_states
        .iter()
        .position(|state| state.ctx_id == ctx_id && state.player_id == player_id);
    let applied = applied_index
        .map(|index| sync_states[index].clone())
        .unwrap_or(PokemonBaseStatSyncState {
            ctx_id,
            player_id,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
            kills: 0,
            assists: 0,
        });

    let delta_damage_dealt = custom_stats
        .damage_dealt
        .saturating_sub(applied.damage_dealt);
    let delta_damage_taken = custom_stats
        .damage_taken
        .saturating_sub(applied.damage_taken);
    let delta_healing_done = custom_stats
        .healing_done
        .saturating_sub(applied.healing_done);
    let delta_kills = custom_stats.kills.saturating_sub(applied.kills);
    let delta_assists = custom_stats.assists.saturating_sub(applied.assists);
    if delta_damage_dealt == 0
        && delta_damage_taken == 0
        && delta_healing_done == 0
        && delta_kills == 0
        && delta_assists == 0
    {
        return;
    }

    let Some(player) = ctx.get_player(player_id) else {
        log_pokemon_base_stat_sync_event(&format!(
            "event=base_stat_sync_skip reason=missing_player tick={} ctx={} player={} custom=dd:{} dt:{} heal:{} k:{} d:{} a:{}",
            ctx.tick(),
            ctx_id,
            player_id,
            custom_stats.damage_dealt,
            custom_stats.damage_taken,
            custom_stats.healing_done,
            custom_stats.kills,
            custom_stats.deaths,
            custom_stats.assists
        ));
        return;
    };
    let expected_team = player.team();
    let expected_position = player.position();
    let handle = player.handle();
    if handle.is_null() {
        log_pokemon_base_stat_sync_event(&format!(
            "event=base_stat_sync_skip reason=null_handle tick={} ctx={} player={} team={} pos={:?}",
            ctx.tick(),
            ctx_id,
            player_id,
            expected_team,
            expected_position
        ));
        return;
    }

    let applied_successfully = unsafe {
        let player_state = &mut *(handle.as_ptr() as *mut PlayerState);
        if player_state.info.id != player_id
            || player_state.info.team != expected_team
            || player_state.info.position != expected_position
        {
            log_pokemon_base_stat_sync_event(&format!(
                "event=base_stat_sync_skip reason=identity_mismatch tick={} ctx={} player={} handle_id={} expected_team={} handle_team={} expected_pos={:?} handle_pos={:?}",
                ctx.tick(),
                ctx_id,
                player_id,
                player_state.info.id,
                expected_team,
                player_state.info.team,
                expected_position,
                player_state.info.position
            ));
            false
        } else {
            apply_pokemon_stat_deltas_to_base_player(
                player_state,
                delta_damage_dealt,
                delta_damage_taken,
                delta_healing_done,
                delta_kills,
                delta_assists,
            );
            true
        }
    };

    if !applied_successfully {
        return;
    }

    let synced = PokemonBaseStatSyncState {
        ctx_id,
        player_id,
        damage_dealt: custom_stats.damage_dealt,
        damage_taken: custom_stats.damage_taken,
        healing_done: custom_stats.healing_done,
        kills: custom_stats.kills,
        assists: custom_stats.assists,
    };
    if let Some(index) = applied_index {
        sync_states[index] = synced;
    } else {
        sync_states.push(synced);
        while sync_states.len() > POKEMON_BASE_STAT_SYNC_MAX {
            sync_states.remove(0);
        }
    }

    log_pokemon_base_stat_sync_event(&format!(
        "event=base_stat_sync_apply tick={} ctx={} player={} delta_dd={} delta_dt={} delta_heal={} delta_k={} delta_a={} total_dd={} total_dt={} total_heal={} total_k={} custom_deaths={} total_a={}",
        ctx.tick(),
        ctx_id,
        player_id,
        delta_damage_dealt,
        delta_damage_taken,
        delta_healing_done,
        delta_kills,
        delta_assists,
        custom_stats.damage_dealt,
        custom_stats.damage_taken,
        custom_stats.healing_done,
        custom_stats.kills,
        custom_stats.deaths,
        custom_stats.assists
    ));
}

fn apply_pokemon_stat_deltas_to_base_player(
    player_state: &mut PlayerState,
    damage_dealt: usize,
    damage_taken: usize,
    healing_done: usize,
    kills: usize,
    assists: usize,
) {
    player_state.info.statistics.deal = player_state
        .info
        .statistics
        .deal
        .saturating_add(damage_dealt);
    player_state.info.statistics.tank = player_state
        .info
        .statistics
        .tank
        .saturating_add(damage_taken);
    player_state.info.statistics.heal = player_state
        .info
        .statistics
        .heal
        .saturating_add(healing_done);
    player_state.info.statistics.kill = player_state.info.statistics.kill.saturating_add(kills);
    player_state.info.statistics.assist =
        player_state.info.statistics.assist.saturating_add(assists);

    player_state.info.last_statistics.deal = player_state
        .info
        .last_statistics
        .deal
        .saturating_add(damage_dealt);
    player_state.info.last_statistics.tank = player_state
        .info
        .last_statistics
        .tank
        .saturating_add(damage_taken);
    player_state.info.last_statistics.heal = player_state
        .info
        .last_statistics
        .heal
        .saturating_add(healing_done);
    player_state.info.last_statistics.kill =
        player_state.info.last_statistics.kill.saturating_add(kills);
    player_state.info.last_statistics.assist = player_state
        .info
        .last_statistics
        .assist
        .saturating_add(assists);
}

fn log_pokemon_base_stat_sync_event(message: &str) {
    let count = POKEMON_BASE_STAT_SYNC_LOG_COUNT.get_or_init(|| Mutex::new(0));
    let mut count = count.lock().expect("pokemon stat sync log count poisoned");
    if *count >= POKEMON_BASE_STAT_SYNC_LOG_LIMIT {
        return;
    }
    *count += 1;
    crate::crash_probe::log_stat_probe_event(message);
}

fn refresh_pokemon_combat_stat_identity(
    ctx: &GameCtx,
    player_id: usize,
    entity_id: Option<usize>,
    champion_id: Option<&'static str>,
) {
    let ctx_id = combat_ctx_id(ctx);
    let states = POKEMON_COMBAT_STATS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("pokemon combat stat state poisoned");
    let Some(state) = states
        .iter_mut()
        .find(|state| state.ctx_id == ctx_id && state.player_id == player_id)
    else {
        return;
    };

    if let Some(entity_id) = entity_id {
        state.entity_id = entity_id;
    }
    if let Some(champion_id) = champion_id {
        state.champion_id = Some(champion_id);
    }
    if let Some(player) = ctx.get_player(player_id) {
        state.team = player.team();
        state.position = player.position();
        if let Some(athlete_id) =
            athlete_id_for_player_identity(player_id, state.team, state.position, state.champion_id)
        {
            state.athlete_id = Some(athlete_id);
        }
    }
    state.last_seen_tick = ctx.tick();
}

fn with_pokemon_combat_stat_for_player_mut(
    ctx: &GameCtx,
    player_id: usize,
    f: impl FnOnce(&mut PokemonCombatStatState),
) {
    let ctx_id = combat_ctx_id(ctx);
    let live_entity_id = entity_for_player_in_ctx(ctx, player_id);
    let live_champion_id = live_entity_id.and_then(|entity_id| {
        champion_id_for_entity_in_ctx(ctx, entity_id).or_else(|| champion_id_for_entity(entity_id))
    });
    let states = POKEMON_COMBAT_STATS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("pokemon combat stat state poisoned");
    if let Some(state) = states
        .iter_mut()
        .find(|state| state.ctx_id == ctx_id && state.player_id == player_id)
    {
        if let Some(entity_id) = live_entity_id {
            state.entity_id = entity_id;
        }
        if let Some(champion_id) = live_champion_id {
            state.champion_id = Some(champion_id);
        }
        if let Some(player) = ctx.get_player(player_id) {
            state.team = player.team();
            state.position = player.position();
            if let Some(athlete_id) = athlete_id_for_player_identity(
                player_id,
                state.team,
                state.position,
                state.champion_id,
            ) {
                state.athlete_id = Some(athlete_id);
            }
        }
        state.last_seen_tick = ctx.tick();
        f(state);
        return;
    }
    let entity_id = live_entity_id
        .or_else(|| entity_for_player(player_id))
        .unwrap_or(player_id);
    let (team, position) = ctx
        .get_player(player_id)
        .map(|player| (player.team(), player.position()))
        .unwrap_or((usize::MAX, Position::Top));
    let athlete_id = athlete_id_for_player_identity(player_id, team, position, live_champion_id);
    let mut state = PokemonCombatStatState {
        ctx_id,
        player_id,
        athlete_id,
        entity_id,
        team,
        position,
        champion_id: live_champion_id.or_else(|| champion_id_for_entity(entity_id)),
        last_seen_tick: ctx.tick(),
        damage_dealt: 0,
        damage_taken: 0,
        healing_done: 0,
        kills: 0,
        deaths: 0,
        assists: 0,
    };
    f(&mut state);
    states.push(state);
    while states.len() > POKEMON_COMBAT_STATS_MAX {
        states.remove(0);
    }
}

#[allow(dead_code)]
pub fn custom_player_kills(player_id: usize) -> usize {
    pokemon_combat_stats_for_player(player_id)
        .map(|stats| stats.kills)
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn custom_player_deaths(player_id: usize) -> usize {
    pokemon_combat_stats_for_player(player_id)
        .map(|stats| stats.deaths)
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn custom_player_assists(player_id: usize) -> usize {
    pokemon_combat_stats_for_player(player_id)
        .map(|stats| stats.assists)
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn custom_player_damage_dealt(player_id: usize) -> usize {
    pokemon_combat_stats_for_player(player_id)
        .map(|stats| stats.damage_dealt)
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn custom_player_damage_taken(player_id: usize) -> usize {
    pokemon_combat_stats_for_player(player_id)
        .map(|stats| stats.damage_taken)
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn custom_player_healing_done(player_id: usize) -> usize {
    pokemon_combat_stats_for_player(player_id)
        .map(|stats| stats.healing_done)
        .unwrap_or(0)
}

#[allow(dead_code)]
fn pokemon_combat_stats_for_player(player_id: usize) -> Option<PokemonCombatStatState> {
    POKEMON_COMBAT_STATS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("pokemon combat stat state poisoned")
        .iter()
        .rev()
        .find(|state| state.player_id == player_id)
        .cloned()
}

fn pokemon_combat_stats_for_ctx_player(
    ctx_id: usize,
    player_id: usize,
) -> Option<PokemonCombatStatState> {
    POKEMON_COMBAT_STATS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("pokemon combat stat state poisoned")
        .iter()
        .rev()
        .find(|state| state.ctx_id == ctx_id && state.player_id == player_id)
        .cloned()
}

pub fn pokemon_combat_stat_snapshots() -> Vec<PokemonCombatStatSnapshot> {
    POKEMON_COMBAT_STATS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("pokemon combat stat state poisoned")
        .iter()
        .filter(|state| {
            state.damage_dealt > 0
                || state.damage_taken > 0
                || state.healing_done > 0
                || state.kills > 0
                || state.deaths > 0
                || state.assists > 0
        })
        .map(|state| {
            let (resolved_athlete_id, resolved_athlete_source) =
                athlete_id_for_player_identity_with_source(
                    state.player_id,
                    state.team,
                    state.position,
                    state.champion_id,
                );
            let (athlete_id, athlete_source) = if resolved_athlete_id.is_some() {
                (resolved_athlete_id, resolved_athlete_source)
            } else if let Some(athlete_id) = state.athlete_id.and_then(reliable_athlete_id) {
                (Some(athlete_id), "state")
            } else {
                (None, "none")
            };

            PokemonCombatStatSnapshot {
                ctx_id: state.ctx_id,
                player_id: state.player_id,
                athlete_id,
                athlete_source,
                team: state.team,
                position: state.position,
                champion_id: state.champion_id,
                damage_dealt: state.damage_dealt,
                damage_taken: state.damage_taken,
                healing_done: state.healing_done,
                kills: state.kills,
                deaths: state.deaths,
                assists: state.assists,
            }
        })
        .collect()
}

pub fn begin_bouffalant_retaliate(
    ctx: &GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    damage_reduce_percent: usize,
    retaliation_damage_percent: usize,
    bonus_ad_damage: usize,
    attacker_types: TypeSet,
) {
    let Some(entity) = ctx.get_entity(entity_id) else {
        return;
    };
    if !entity.is_alive() || !entity.is_champion() {
        return;
    }
    let caster_team = entity.team();
    drop(entity);

    let states = BOUFFALANT_RETALIATES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("bouffalant retaliate state poisoned");
    states.retain(|state| state.entity_id != entity_id);
    states.push(BouffalantRetaliateState {
        entity_id,
        caster_team,
        expires_at: ctx.tick().saturating_add(duration_ticks),
        damage_reduce_percent,
        retaliation_damage_percent,
        bonus_ad_damage,
        attacker_types,
        attackers: Vec::new(),
    });
}

pub fn bouffalant_retaliate_reduce_percent(ctx: &GameCtx, entity_id: usize) -> usize {
    let tick = ctx.tick();
    BOUFFALANT_RETALIATES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("bouffalant retaliate state poisoned")
        .iter()
        .find(|state| state.entity_id == entity_id && state.expires_at > tick)
        .map(|state| state.damage_reduce_percent)
        .unwrap_or(0)
}

fn note_bouffalant_retaliate_damage(
    ctx: &GameCtx,
    entity_id: usize,
    attacker_id: usize,
    damage: usize,
) {
    if entity_id == attacker_id || damage == 0 {
        return;
    }
    let tick = ctx.tick();
    let states = BOUFFALANT_RETALIATES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("bouffalant retaliate state poisoned");
    states.retain(|state| state.expires_at > tick);
    let Some(state) = states
        .iter_mut()
        .find(|state| state.entity_id == entity_id && state.expires_at > tick)
    else {
        return;
    };
    if let Some((_, stored_damage)) = state
        .attackers
        .iter_mut()
        .find(|(stored_attacker, _)| *stored_attacker == attacker_id)
    {
        *stored_damage = stored_damage.saturating_add(damage);
    } else {
        state.attackers.push((attacker_id, damage));
    }
}

pub fn note_bouffalant_cc(ctx: &GameCtx, entity_id: usize) {
    let states = BOUFFALANT_CCS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("bouffalant cc state poisoned");
    if let Some(state) = states.iter_mut().find(|state| state.entity_id == entity_id) {
        state.last_cc_at = ctx.tick();
    } else {
        states.push(BouffalantCcState {
            entity_id,
            last_cc_at: ctx.tick(),
        });
    }
}

pub fn bouffalant_recently_cced(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    BOUFFALANT_CCS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("bouffalant cc state poisoned")
        .iter()
        .any(|state| {
            state.entity_id == entity_id
                && tick.saturating_sub(state.last_cc_at) <= BOUFFALANT_RECENT_CC_WINDOW_TICKS
        })
}

pub fn begin_bouffalant_unstoppable(ctx: &GameCtx, entity_id: usize, ticks: usize) {
    let states = BOUFFALANT_UNSTOPPABLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("bouffalant unstoppable state poisoned");
    states.retain(|state| state.entity_id != entity_id && state.expires_at > ctx.tick());
    states.push(BouffalantUnstoppableState {
        entity_id,
        expires_at: ctx.tick().saturating_add(ticks),
    });
}

pub fn is_bouffalant_unstoppable(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    BOUFFALANT_UNSTOPPABLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("bouffalant unstoppable state poisoned")
        .iter()
        .any(|state| state.entity_id == entity_id && state.expires_at > tick)
}

pub fn mark_bouffalant_head_charge_target(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    ticks: usize,
    bonus_max_hp_percent: usize,
) {
    let states = BOUFFALANT_HEAD_CHARGE_MARKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("bouffalant head charge mark state poisoned");
    states.retain(|state| !(state.caster_id == caster_id && state.target_id == target_id));
    states.push(BouffalantHeadChargeMarkState {
        caster_id,
        target_id,
        expires_at: ctx.tick().saturating_add(ticks),
        bonus_max_hp_percent,
    });
}

pub fn bouffalant_head_charge_bonus_damage(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    attack_type: AttackType,
    move_type: PokemonType,
) -> usize {
    let applies_to_move =
        matches!(attack_type, AttackType::BaseAttack) || matches!(move_type, PokemonType::Ground);
    if !applies_to_move {
        return 0;
    }
    let tick = ctx.tick();
    let states = BOUFFALANT_HEAD_CHARGE_MARKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("bouffalant head charge mark state poisoned");
    states.retain(|state| state.expires_at > tick);
    let Some(state) = states
        .iter()
        .find(|state| state.caster_id == attacker_id && state.target_id == target_id)
    else {
        return 0;
    };
    ctx.get_entity(target_id)
        .map(|target| target.hp().max.saturating_mul(state.bonus_max_hp_percent) / 100)
        .unwrap_or(0)
}

pub fn consume_direct_pokemon_damage(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
) -> Option<usize> {
    let tick = ctx.tick();
    let ledger = DIRECT_DAMAGE_LEDGER.get_or_init(|| Mutex::new(Vec::new()));
    let mut ledger = ledger.lock().expect("direct damage ledger poisoned");
    let mut consumed = None;
    ledger.retain(|state| {
        if tick.saturating_sub(state.tick) > 2 {
            return false;
        }
        if consumed.is_none() && state.attacker_id == attacker_id && state.target_id == target_id {
            consumed = Some(state.damage);
            return false;
        }
        true
    });
    consumed
}

pub fn note_physical_basic_hit(ctx: &GameCtx, attacker_id: usize, target_id: usize) {
    if attacker_id == target_id {
        return;
    }

    let tick = ctx.tick();
    let hits = PHYSICAL_BASIC_HITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut hits = hits.lock().expect("physical basic hit state poisoned");
    hits.retain(|(_, _, hit_tick)| tick.saturating_sub(*hit_tick) <= 1);
    hits.push((attacker_id, target_id, tick));
}

pub fn consume_physical_basic_hit(ctx: &GameCtx, attacker_id: usize, target_id: usize) -> bool {
    let tick = ctx.tick();
    let hits = PHYSICAL_BASIC_HITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut hits = hits.lock().expect("physical basic hit state poisoned");
    let mut consumed = false;
    hits.retain(|(attacker, target, hit_tick)| {
        if tick.saturating_sub(*hit_tick) > 1 {
            return false;
        }
        if !consumed && *attacker == attacker_id && *target == target_id {
            consumed = true;
            return false;
        }
        true
    });
    consumed
}

pub fn recent_physical_basic_hit(ctx: &GameCtx, attacker_id: usize, target_id: usize) -> bool {
    let tick = ctx.tick();
    let hits = PHYSICAL_BASIC_HITS.get_or_init(|| Mutex::new(Vec::new()));
    let mut hits = hits.lock().expect("physical basic hit state poisoned");
    hits.retain(|(_, _, hit_tick)| tick.saturating_sub(*hit_tick) <= 1);
    hits.iter()
        .any(|(attacker, target, _)| *attacker == attacker_id && *target == target_id)
}

pub fn begin_hawlucha_counter(
    ctx: &GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    reduce_percent: usize,
    retaliation_damage: usize,
    radius: u64,
    slow_percent: i32,
    slow_ticks: usize,
    lifesteal_percent: i32,
    lifesteal_ticks: usize,
) {
    let tick = ctx.tick();
    let counters = HAWLUCHA_COUNTERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut counters = counters.lock().expect("hawlucha counter state poisoned");
    counters.retain(|state| state.entity_id != entity_id && state.expires_at > tick);
    counters.push(HawluchaCounterState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
        reduce_percent,
        retaliation_damage,
        radius,
        slow_percent,
        slow_ticks,
        lifesteal_percent,
        lifesteal_ticks,
    });
}

pub fn try_trigger_hawlucha_counter(
    ctx: &mut GameCtx,
    entity_id: usize,
    attacker_id: usize,
    attack_type: AttackType,
) -> usize {
    if entity_id == attacker_id || matches!(attack_type, AttackType::Dot) {
        return 0;
    }
    let tick = ctx.tick();
    let counters = HAWLUCHA_COUNTERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut counters = counters.lock().expect("hawlucha counter state poisoned");
    let Some(index) = counters
        .iter()
        .position(|state| state.entity_id == entity_id && state.expires_at > tick)
    else {
        counters.retain(|state| state.expires_at > tick);
        return 0;
    };
    let state = counters.remove(index);
    counters.retain(|state| state.expires_at > tick);
    drop(counters);

    let Some(hawlucha) = ctx.get_entity(entity_id) else {
        return state.reduce_percent;
    };
    if !hawlucha.is_alive() {
        return state.reduce_percent;
    }
    let caster_pos = hawlucha.pos();
    let caster_team = hawlucha.team();
    let attacker_is_champion = ctx
        .get_entity(attacker_id)
        .map(|attacker| attacker.is_alive() && attacker.is_champion())
        .unwrap_or(false);
    drop(hawlucha);

    let target_ids: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() != caster_team
                && entity.is_alive()
                && !entity.is_tower()
                && distance_sq(entity.pos(), caster_pos)
                    <= state.radius.saturating_mul(state.radius)
        })
        .map(|entity| entity.id())
        .collect();
    for target_id in target_ids {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            entity_id,
            target_id,
            state.retaliation_damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Fighting,
            TypeSet::dual(PokemonType::Fighting, PokemonType::Flying),
            defender_types,
        );
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: state.slow_ticks,
                },
                move_speed_mult: -state.slow_percent.abs(),
                ..Default::default()
            },
        );
    }
    if attacker_is_champion {
        ctx.add_buff(
            entity_id,
            BuffState {
                duration: BuffType::Time {
                    tick: state.lifesteal_ticks,
                },
                vamp: state.lifesteal_percent,
                ..Default::default()
            },
        );
    }
    apply_hawlucha_momentum(ctx, entity_id);
    state.reduce_percent
}

pub fn begin_zeraora_zing_zap(
    ctx: &GameCtx,
    entity_id: usize,
    duration_ticks: usize,
    damage: usize,
    blink_range: u64,
    force_move_speed: u64,
    force_move_ticks: u64,
) {
    if duration_ticks == 0 {
        return;
    }
    let tick = ctx.tick();
    let states = ZERAORA_ZING_ZAPS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("zeraora zing zap state poisoned");
    states.retain(|state| state.entity_id != entity_id && state.expires_at > tick);
    states.push(ZeraoraZingZapState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
        damage: damage.max(1),
        blink_range,
        force_move_speed: force_move_speed.max(1),
        force_move_ticks: force_move_ticks.max(1),
    });
}

pub fn try_consume_zeraora_zing_zap(
    ctx: &mut GameCtx,
    entity_id: usize,
    attacker_id: usize,
    attack_type: AttackType,
) -> bool {
    if entity_id == attacker_id || matches!(attack_type, AttackType::BaseAttack | AttackType::Dot) {
        return false;
    }
    let tick = ctx.tick();
    let states = ZERAORA_ZING_ZAPS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("zeraora zing zap state poisoned");
    let Some(index) = states
        .iter()
        .position(|state| state.entity_id == entity_id && state.expires_at > tick)
    else {
        states.retain(|state| state.expires_at > tick);
        return false;
    };
    let state = states.remove(index);
    states.retain(|state| state.expires_at > tick);
    drop(states);

    let Some(zeraora) = ctx.get_entity(entity_id) else {
        return true;
    };
    if !zeraora.is_alive() {
        return true;
    }
    let zeraora_pos = zeraora.pos();
    drop(zeraora);
    let Some(attacker) = ctx.get_entity(attacker_id) else {
        return true;
    };
    if !attacker.is_alive() {
        return true;
    }
    let attacker_pos = attacker.pos();
    drop(attacker);

    if distance_sq(zeraora_pos, attacker_pos) <= state.blink_range.saturating_mul(state.blink_range)
    {
        let dx = attacker_pos.x as i64 - zeraora_pos.x as i64;
        let dy = attacker_pos.y as i64 - zeraora_pos.y as i64;
        apply_pokemon_cc(
            ctx,
            entity_id,
            entity_id,
            CCState::ForceMove {
                tick: state.force_move_ticks,
                dx,
                dy,
                speed: state.force_move_speed,
            },
        );
    }

    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, attacker_id);
    crate::pokemon_types::deal_pokemon_damage(
        ctx,
        entity_id,
        attacker_id,
        0,
        state.damage,
        AttackType::Skill,
        PokemonType::Electric,
        TypeSet::single(PokemonType::Electric),
        defender_types,
    );
    true
}

pub fn begin_zeraora_thunder_cage(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    center: EntityPos,
    radius: u64,
    duration_ticks: usize,
    tick_interval: usize,
    tick_damage: usize,
    exit_damage: usize,
    slow_percent: i32,
) {
    if duration_ticks == 0 || tick_interval == 0 || radius == 0 {
        return;
    }
    let state = ZeraoraThunderCageState {
        caster_id,
        caster_team,
        center,
        radius,
        expires_at: ctx.tick().saturating_add(duration_ticks),
        next_tick_at: ctx.tick(),
        tick_interval,
        tick_damage: tick_damage.max(1),
        exit_damage: exit_damage.max(1),
        slow_percent: slow_percent.abs(),
        occupants: Vec::new(),
    };
    let states = ZERAORA_THUNDER_CAGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("zeraora thunder cage state poisoned");
    states.push(state);
}

pub fn update_zeraora_thunder_cages(ctx: &mut GameCtx) {
    let tick = ctx.tick();
    let states = ZERAORA_THUNDER_CAGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("zeraora thunder cage state poisoned");
    let mut tick_hits = Vec::new();
    let mut exit_hits = Vec::new();

    states.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let radius_sq = state.radius.saturating_mul(state.radius);
        let current: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), state.center) <= radius_sq
            })
            .map(|entity| entity.id())
            .collect();

        for old_id in state.occupants.iter().copied() {
            if !current.iter().any(|id| *id == old_id) {
                exit_hits.push((state.caster_id, old_id, state.exit_damage));
            }
        }
        if tick >= state.next_tick_at {
            for target_id in current.iter().copied() {
                tick_hits.push((
                    state.caster_id,
                    target_id,
                    state.tick_damage,
                    state.slow_percent,
                ));
            }
            state.next_tick_at = tick.saturating_add(state.tick_interval);
        }
        state.occupants = current;
        draw_field_circle(ctx, state.center, state.radius, VFX_ELECTRIC);
        true
    });
    drop(states);

    for (caster_id, target_id, damage) in exit_hits {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage,
            AttackType::Skill,
            PokemonType::Electric,
            TypeSet::single(PokemonType::Electric),
            defender_types,
        );
    }
    for (caster_id, target_id, damage, slow_percent) in tick_hits {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage,
            AttackType::Skill,
            PokemonType::Electric,
            TypeSet::single(PokemonType::Electric),
            defender_types,
        );
        add_harmful_buff(
            ctx,
            caster_id,
            target_id,
            BuffState {
                duration: BuffType::Time { tick: 70 },
                move_speed_mult: -slow_percent.abs(),
                ..Default::default()
            },
        );
        note_zeraora_ability_hit(ctx, caster_id, target_id, ActionSlot::Skill);
    }
}

pub fn mark_zeraora_wild_charge_target(
    ctx: &GameCtx,
    caster_id: usize,
    target_id: usize,
    bonus_percent: usize,
    ticks: usize,
) {
    if bonus_percent == 0 || ticks == 0 {
        return;
    }
    let states = ZERAORA_WILD_CHARGE_MARKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("zeraora wild charge mark state poisoned");
    states.retain(|state| {
        !(state.caster_id == caster_id && state.target_id == target_id)
            && state.expires_at > ctx.tick()
    });
    states.push(ZeraoraWildChargeMarkState {
        caster_id,
        target_id,
        bonus_percent,
        expires_at: ctx.tick().saturating_add(ticks),
    });
}

pub fn zeraora_wild_charge_basic_bonus(ctx: &GameCtx, caster_id: usize, target_id: usize) -> usize {
    let tick = ctx.tick();
    let states = ZERAORA_WILD_CHARGE_MARKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states
        .lock()
        .expect("zeraora wild charge mark state poisoned");
    states.retain(|state| state.expires_at > tick);
    states
        .iter()
        .filter(|state| state.caster_id == caster_id && state.target_id == target_id)
        .map(|state| state.bonus_percent)
        .max()
        .unwrap_or(0)
}

pub fn note_zeraora_ability_hit(
    ctx: &mut GameCtx,
    caster_id: usize,
    target_id: usize,
    slot: ActionSlot,
) {
    if matches!(slot, ActionSlot::Attack) {
        return;
    }
    let target_is_champion = ctx
        .get_entity(target_id)
        .map(|target| target.is_champion() && target.is_alive())
        .unwrap_or(false);
    if !target_is_champion {
        return;
    }

    let tick = ctx.tick();
    {
        let cooldowns = ZERAORA_MERCILESS_COOLDOWNS.get_or_init(|| Mutex::new(Vec::new()));
        let mut cooldowns = cooldowns
            .lock()
            .expect("zeraora merciless cooldown state poisoned");
        cooldowns.retain(|state| state.ready_at > tick);
        if cooldowns
            .iter()
            .any(|state| state.caster_id == caster_id && state.target_id == target_id)
        {
            let hits = ZERAORA_MERCILESS_HITS.get_or_init(|| Mutex::new(Vec::new()));
            let mut hits = hits.lock().expect("zeraora merciless hit state poisoned");
            hits.retain(|state| state.expires_at > tick);
            hits.push(ZeraoraMercilessHitState {
                caster_id,
                target_id,
                slot,
                expires_at: tick.saturating_add(2 * 60),
            });
            return;
        }
    }

    let should_trigger = {
        let hits = ZERAORA_MERCILESS_HITS.get_or_init(|| Mutex::new(Vec::new()));
        let mut hits = hits.lock().expect("zeraora merciless hit state poisoned");
        hits.retain(|state| state.expires_at > tick);
        let trigger = hits.iter().any(|state| {
            state.caster_id == caster_id && state.target_id == target_id && state.slot != slot
        });
        hits.push(ZeraoraMercilessHitState {
            caster_id,
            target_id,
            slot,
            expires_at: tick.saturating_add(2 * 60),
        });
        trigger
    };
    if !should_trigger {
        return;
    }

    let cooldowns = ZERAORA_MERCILESS_COOLDOWNS.get_or_init(|| Mutex::new(Vec::new()));
    let mut cooldowns = cooldowns
        .lock()
        .expect("zeraora merciless cooldown state poisoned");
    cooldowns.push(ZeraoraMercilessCooldownState {
        caster_id,
        target_id,
        ready_at: tick.saturating_add(5 * 60),
    });
    drop(cooldowns);

    trigger_zeraora_merciless(ctx, caster_id, target_id);
}

fn trigger_zeraora_merciless(ctx: &mut GameCtx, caster_id: usize, target_id: usize) {
    const RADIUS: u64 = 24000;
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }
    let center = target.pos();
    let target_team = target.team();
    drop(target);
    let hits: Vec<usize> = (0..ctx.entity_count())
        .filter_map(|index| ctx.entity_at(index))
        .filter(|entity| {
            entity.team() == target_team
                && entity.is_alive()
                && !entity.is_tower()
                && distance_sq(entity.pos(), center) <= RADIUS.saturating_mul(RADIUS)
        })
        .map(|entity| entity.id())
        .collect();
    let damage = ctx
        .get_entity(caster_id)
        .map(|caster| 35usize.saturating_add(caster.stat().magic_power.saturating_mul(35) / 100))
        .unwrap_or(35)
        .max(1);
    for hit_id in hits.iter().copied() {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, hit_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            hit_id,
            0,
            damage,
            AttackType::Skill,
            PokemonType::Electric,
            TypeSet::single(PokemonType::Electric),
            defender_types,
        );
    }
    let duration = hits.len().min(3).saturating_mul(60);
    if duration > 0 {
        add_beneficial_buff(
            ctx,
            caster_id,
            caster_id,
            BuffState {
                duration: BuffType::Time { tick: duration },
                skill_cooldown_mult: 25,
                ult_cooldown_mult: 25,
                ..Default::default()
            },
        );
    }
    draw_field_circle(ctx, center, RADIUS, VFX_ELECTRIC);
}

pub fn apply_hawlucha_momentum(ctx: &mut GameCtx, entity_id: usize) {
    add_beneficial_buff(
        ctx,
        entity_id,
        entity_id,
        BuffState {
            duration: BuffType::Time {
                tick: HAWLUCHA_MOMENTUM_TICKS,
            },
            attack_speed_mult: HAWLUCHA_MOMENTUM_ATTACK_SPEED,
            move_speed_mult: HAWLUCHA_MOMENTUM_MOVE_SPEED,
            ..Default::default()
        },
    );
}

pub fn apply_copied_hawlucha_momentum(ctx: &mut GameCtx, entity_id: usize) {
    if receiver_has_copied(entity_id, "pokemon_moba_hawlucha") {
        apply_hawlucha_momentum(ctx, entity_id);
    }
}

pub fn note_hawlucha_flying_press_target(ctx: &mut GameCtx, entity_id: usize, target_id: usize) {
    let target_is_champion = ctx
        .get_entity(target_id)
        .map(|target| target.is_champion())
        .unwrap_or(false);
    if !target_is_champion {
        return;
    }
    let momentum = HAWLUCHA_MOMENTUMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut momentum = momentum.lock().expect("hawlucha momentum state poisoned");
    let Some(state) = momentum
        .iter_mut()
        .find(|state| state.entity_id == entity_id)
    else {
        momentum.push(HawluchaMomentumState {
            entity_id,
            flying_press_targets: vec![target_id],
        });
        drop(momentum);
        grant_hawlucha_unique_ad(ctx, entity_id);
        return;
    };
    if state.flying_press_targets.contains(&target_id) {
        return;
    }
    state.flying_press_targets.push(target_id);
    drop(momentum);
    grant_hawlucha_unique_ad(ctx, entity_id);
}

fn grant_hawlucha_unique_ad(ctx: &mut GameCtx, entity_id: usize) {
    ctx.add_buff(
        entity_id,
        BuffState {
            attack: HAWLUCHA_FLYING_PRESS_AD_PER_UNIQUE_CHAMPION,
            ..Default::default()
        },
    );
}

fn update_sigilyph_gravities(ctx: &GameCtx, tick: usize) {
    let states = SIGILYPH_GRAVITIES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sigilyph gravity state poisoned");
    let mut restores = Vec::new();
    states.retain(|state| {
        let alive = ctx
            .get_entity(state.entity_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        if alive && state.expires_at > tick {
            true
        } else {
            restores.push((state.entity_id, state.original_types));
            false
        }
    });
    drop(states);

    for (entity_id, original_types) in restores {
        register_entity_types(entity_id, original_types);
    }
}

fn update_sigilyph_glyphs(ctx: &mut GameCtx, tick: usize) {
    let states = SIGILYPH_GLYPHS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("sigilyph glyph state poisoned");
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        ctx.get_entity(state.caster_id)
            .map(|caster| caster.is_alive())
            .unwrap_or(false)
            && ctx
                .get_entity(state.target_id)
                .map(|target| target.is_alive())
                .unwrap_or(false)
    });

    let snapshot = states.clone();
    let mut exploding_pairs: Vec<(SigilyphGlyphState, SigilyphGlyphState)> = Vec::new();
    let mut remove_marks: Vec<(usize, usize)> = Vec::new();
    for (left_index, left) in snapshot.iter().enumerate() {
        if left.target_team == left.caster_team
            || remove_marks.contains(&(left.caster_id, left.target_id))
        {
            continue;
        }
        let Some(left_pos) = ctx.get_entity(left.target_id).map(|target| target.pos()) else {
            continue;
        };
        for right in snapshot.iter().skip(left_index + 1) {
            if right.caster_id != left.caster_id
                || right.target_team == right.caster_team
                || remove_marks.contains(&(right.caster_id, right.target_id))
            {
                continue;
            }
            let Some(right_pos) = ctx.get_entity(right.target_id).map(|target| target.pos()) else {
                continue;
            };
            if distance_sq(left_pos, right_pos)
                <= SIGILYPH_GLYPH_PROXIMITY_RADIUS.saturating_mul(SIGILYPH_GLYPH_PROXIMITY_RADIUS)
            {
                exploding_pairs.push((*left, *right));
                remove_marks.push((left.caster_id, left.target_id));
                remove_marks.push((right.caster_id, right.target_id));
                break;
            }
        }
    }
    states.retain(|state| !remove_marks.contains(&(state.caster_id, state.target_id)));
    drop(states);

    for (left, right) in exploding_pairs {
        explode_sigilyph_glyph(ctx, left.caster_id, left.target_id);
        explode_sigilyph_glyph(ctx, right.caster_id, right.target_id);
    }
}

fn explode_sigilyph_glyph(ctx: &mut GameCtx, caster_id: usize, target_id: usize) {
    let Some(caster) = ctx.get_entity(caster_id) else {
        return;
    };
    if !caster.is_alive() {
        return;
    }
    let caster_team = caster.team();
    let caster_ap = caster.stat().magic_power;
    let caster_types =
        entity_types(caster_id).unwrap_or(TypeSet::dual(PokemonType::Psychic, PokemonType::Flying));
    drop(caster);

    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if !target.is_alive() {
        return;
    }
    let target_pos = target.pos();
    drop(target);

    let primary_damage = SIGILYPH_GLYPH_EXPLOSION_BASE_AP
        + caster_ap.saturating_mul(SIGILYPH_GLYPH_EXPLOSION_AP_RATIO) / 100;
    let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
    crate::pokemon_types::deal_pokemon_damage(
        ctx,
        caster_id,
        target_id,
        0,
        primary_damage.max(1),
        AttackType::Skill,
        PokemonType::Psychic,
        caster_types,
        defender_types,
    );

    let splash_damage = primary_damage.saturating_mul(SIGILYPH_GLYPH_SPLASH_PERCENT) / 100;
    if splash_damage > 0 {
        let splash_targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), target_pos)
                        <= SIGILYPH_GLYPH_EXPLOSION_RADIUS
                            .saturating_mul(SIGILYPH_GLYPH_EXPLOSION_RADIUS)
            })
            .map(|entity| entity.id())
            .collect();
        for splash_target_id in splash_targets {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, splash_target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                caster_id,
                splash_target_id,
                0,
                splash_damage.max(1),
                AttackType::Skill,
                PokemonType::Psychic,
                caster_types,
                defender_types,
            );
        }
    }
    draw_status_marker(
        ctx,
        target_pos,
        SIGILYPH_GLYPH_EXPLOSION_RADIUS,
        VFX_PSYCHIC,
    );
}

pub fn update_statuses(ctx: &mut GameCtx, rng_seed: u64) {
    let tick = ctx.tick();
    let last_update = LAST_UPDATE_TICK.get_or_init(|| Mutex::new(usize::MAX));
    {
        let mut last_update = last_update.lock().expect("status update tick poisoned");
        if *last_update == tick {
            return;
        }
        *last_update = tick;
    }

    prune_tracked_buffs(ctx);
    update_zeraora_thunder_cages(ctx);
    update_sigilyph_gravities(ctx, tick);
    update_sigilyph_glyphs(ctx, tick);
    update_swanna_cyclones(ctx, tick);
    update_swanna_sky_circuses(ctx, tick);
    update_marowak_bone_windmills(ctx, tick);
    update_garganacl_salt(ctx, tick);
    update_ampharos_gigavolts(ctx, tick);
    update_xatu_super_psys(ctx, tick);
    update_wishiwashi_massive_catches(ctx, tick);
    update_missingno_glitch_storms(ctx, tick);
    update_missingno_trick_rooms(ctx, tick);
    update_missingno_pending_debuffs(ctx, tick);
    update_yanmega_tinted_lenses(ctx, tick);
    update_yanmega_giga_drains(ctx, tick);

    let delayed_confusions = DELAYED_CONFUSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut delayed_confusions = delayed_confusions
        .lock()
        .expect("delayed confusion state poisoned");
    let mut resolving_confusions = Vec::new();
    delayed_confusions.retain(|state| {
        if state.trigger_at > tick {
            return true;
        }
        resolving_confusions.push(*state);
        false
    });
    drop(delayed_confusions);
    for state in resolving_confusions {
        apply_confusion_from(
            ctx,
            state.caster_id,
            state.target_id,
            state.stacks,
            state.ticks,
        );
    }

    let nasty_plots = NASTY_PLOTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut nasty_plots = nasty_plots.lock().expect("nasty plot state poisoned");
    let mut resolving_nasty_plots = Vec::new();
    nasty_plots.retain(|state| {
        if state.resolves_at > tick {
            return true;
        }
        resolving_nasty_plots.push(*state);
        false
    });
    drop(nasty_plots);

    for state in resolving_nasty_plots {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            continue;
        };
        let caster_alive = caster.is_alive();
        drop(caster);
        let Some(target) = ctx.get_entity(state.target_id) else {
            continue;
        };
        let target_alive = target.is_alive();
        let target_hp = target.hp();
        let target_full_hp = target_hp.current >= target_hp.max;
        drop(target);

        if !caster_alive || !target_alive {
            continue;
        }

        if target_full_hp {
            let stun_ticks = adjusted_cc_ticks(ctx, state.target_id, state.stun_ticks) as u64;
            break_kommoo_duel_on_hard_cc(ctx, state.caster_id, state.target_id);
            apply_pokemon_cc(
                ctx,
                state.caster_id,
                state.target_id,
                CCState::Stun { tick: stun_ticks },
            );
            note_steadfast_cc(ctx, state.target_id);
            if let Some(pos) = ctx.get_entity(state.target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, pos, 13000, VFX_DARK);
            }
        } else {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, state.target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                state.caster_id,
                state.target_id,
                0,
                state.damage.max(1),
                AttackType::Skill,
                PokemonType::Dark,
                state.attacker_types,
                defender_types,
            );
            if let Some(pos) = ctx.get_entity(state.target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, pos, 16000, VFX_DARK);
            }
        }
    }

    let stored_powers = STORED_POWERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut stored_powers = stored_powers.lock().expect("stored power state poisoned");
    let mut resolving_stored_powers = Vec::new();
    stored_powers.retain(|state| {
        if state.resolves_at > tick {
            return true;
        }
        resolving_stored_powers.push(*state);
        false
    });
    drop(stored_powers);

    for state in resolving_stored_powers {
        if state.stored_damage == 0 {
            continue;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            continue;
        };
        if !caster.is_alive() {
            continue;
        }
        let caster_pos = caster.pos();
        drop(caster);

        let targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), caster_pos)
                        <= state.radius.saturating_mul(state.radius)
            })
            .map(|entity| entity.id())
            .collect();

        for target_id in targets {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                state.caster_id,
                target_id,
                0,
                state.stored_damage.max(1),
                AttackType::Skill,
                PokemonType::Psychic,
                state.attacker_types,
                defender_types,
            );
        }
        draw_status_marker(ctx, caster_pos, state.radius, VFX_PSYCHIC);
    }

    let retaliates = BOUFFALANT_RETALIATES.get_or_init(|| Mutex::new(Vec::new()));
    let mut retaliates = retaliates
        .lock()
        .expect("bouffalant retaliate state poisoned");
    let mut resolving_retaliates = Vec::new();
    retaliates.retain(|state| {
        if state.expires_at > tick {
            return true;
        }
        resolving_retaliates.push(state.clone());
        false
    });
    drop(retaliates);

    for state in resolving_retaliates {
        let Some(caster) = ctx.get_entity(state.entity_id) else {
            continue;
        };
        if !caster.is_alive() {
            continue;
        }
        let caster_pos = caster.pos();
        drop(caster);

        for (attacker_id, stored_damage) in state.attackers {
            let Some(attacker) = ctx.get_entity(attacker_id) else {
                continue;
            };
            if !attacker.is_alive() || attacker.team() == state.caster_team {
                continue;
            }
            drop(attacker);
            let damage = stored_damage
                .saturating_mul(state.retaliation_damage_percent)
                .saturating_div(100)
                .saturating_add(state.bonus_ad_damage)
                .max(1);
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, attacker_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                state.entity_id,
                attacker_id,
                damage,
                0,
                AttackType::Skill,
                PokemonType::Normal,
                state.attacker_types,
                defender_types,
            );
        }
        draw_status_marker(ctx, caster_pos, 30000, VFX_NORMAL);
    }

    let miasmas = MIASMAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut miasmas = miasmas.lock().expect("miasma state poisoned");
    let mut miasma_death_spreads = Vec::new();
    miasmas.retain(|state| {
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if state.expires_at <= tick {
            return false;
        }
        if !target.is_alive() {
            miasma_death_spreads.push((
                state.caster_id,
                state.target_id,
                target.pos(),
                state.stacks,
                state.poison_damage_per_tick,
            ));
            return false;
        }
        true
    });
    drop(miasmas);

    for (caster_id, dead_target_id, dead_pos, stacks, poison_damage_per_tick) in
        miasma_death_spreads
    {
        let Some(caster) = ctx.get_entity(caster_id) else {
            continue;
        };
        let caster_team = caster.team();
        drop(caster);
        let spread_targets: Vec<usize> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.id() != dead_target_id
                    && entity.team() != caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), dead_pos)
                        <= MIASMA_DEATH_SPREAD_RADIUS.saturating_mul(MIASMA_DEATH_SPREAD_RADIUS)
            })
            .map(|entity| entity.id())
            .collect();
        for target_id in spread_targets {
            add_miasma_stacks(ctx, caster_id, target_id, stacks, poison_damage_per_tick);
        }
        draw_status_marker(ctx, dead_pos, MIASMA_DEATH_SPREAD_RADIUS, VFX_POISON);
    }

    {
        let states = ANTI_HEALS.get_or_init(|| Mutex::new(Vec::new()));
        states
            .lock()
            .expect("anti-heal state poisoned")
            .retain(|state| state.expires_at > tick);
    }

    update_thievul_stakeouts(ctx, tick);

    let states = PARALYSIS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("paralysis state poisoned");
    let mut stuns = Vec::new();

    states.retain_mut(|state| {
        let Some(entity) = ctx.get_entity(state.entity_id) else {
            return false;
        };
        if !entity.is_alive() || state.expires_at <= tick {
            return false;
        }

        while state.next_roll_at <= tick {
            if should_proc(rng_seed, state.entity_id, state.next_roll_at) {
                stuns.push((state.caster_id, state.entity_id));
            }
            state.next_roll_at += PARALYSIS_ROLL_INTERVAL_TICKS;
        }

        true
    });

    drop(states);
    for (caster_id, target_id) in stuns {
        if is_limber(ctx, target_id) {
            continue;
        }
        if let Some(target_pos) = ctx.get_entity(target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 10500, VFX_ELECTRIC);
        }
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Stun {
                tick: PARALYSIS_STUN_TICKS,
            },
        );
        note_steadfast_cc(ctx, target_id);
    }

    let burns = BURNS.get_or_init(|| Mutex::new(Vec::new()));
    let mut burns = burns.lock().expect("burn state poisoned");
    let mut burn_ticks = Vec::new();
    burns.retain_mut(|state| {
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !target.is_alive() || state.expires_at <= tick {
            return false;
        }

        while state.next_tick_at <= tick {
            burn_ticks.push((state.caster_id, state.target_id, state.damage_per_tick));
            state.next_tick_at += BURN_TICK_INTERVAL;
        }

        true
    });
    drop(burns);

    for (caster_id, target_id, damage) in burn_ticks {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        if try_wonder_guard_dot_tick(ctx, caster_id, actual_target_id, PokemonType::Fire) {
            if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, target_pos, 9000, VFX_FIRE);
            }
            continue;
        }
        if has_dot_absorb(ctx, actual_target_id) {
            log_dot_absorb_heal(
                ctx,
                "burn",
                caster_id,
                target_id,
                actual_target_id,
                damage.max(1),
            );
            heal_with_antiheal(ctx, actual_target_id, actual_target_id, damage.max(1));
        } else {
            let (ad_damage, ap_damage) = adjust_endure_damage(ctx, actual_target_id, 0, damage);
            crate::pokemon_status::deal_tracked_damage(
                ctx,
                caster_id,
                actual_target_id,
                ad_damage,
                ap_damage,
                AttackType::Dot,
            );
        }
        if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 9000, VFX_FIRE);
        }
    }

    let poisons = POISONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut poisons = poisons.lock().expect("poison state poisoned");
    let mut poison_ticks = Vec::new();
    poisons.retain_mut(|state| {
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !target.is_alive() || state.expires_at <= tick {
            return false;
        }

        while state.next_tick_at <= tick {
            poison_ticks.push((state.caster_id, state.target_id, state.damage_per_tick));
            state.next_tick_at += POISON_TICK_INTERVAL;
        }
        true
    });
    drop(poisons);

    for (caster_id, target_id, damage) in poison_ticks {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        if try_wonder_guard_dot_tick(ctx, caster_id, actual_target_id, PokemonType::Poison) {
            if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, target_pos, 9000, VFX_POISON);
            }
            continue;
        }
        if has_dot_absorb(ctx, actual_target_id) {
            log_dot_absorb_heal(
                ctx,
                "poison",
                caster_id,
                target_id,
                actual_target_id,
                damage.max(1),
            );
            heal_with_antiheal(ctx, actual_target_id, actual_target_id, damage.max(1));
        } else {
            let (ad_damage, ap_damage) = adjust_endure_damage(ctx, actual_target_id, 0, damage);
            crate::pokemon_status::deal_tracked_damage(
                ctx,
                caster_id,
                actual_target_id,
                ad_damage,
                ap_damage,
                AttackType::Dot,
            );
        }
        if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 9000, VFX_POISON);
        }
    }

    let bleeds = BLEEDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut bleeds = bleeds.lock().expect("bleed state poisoned");
    let mut bleed_ticks = Vec::new();
    bleeds.retain_mut(|state| {
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !target.is_alive() || state.expires_at <= tick {
            return false;
        }

        while state.next_tick_at <= tick {
            bleed_ticks.push((state.caster_id, state.target_id, state.damage_per_tick));
            state.next_tick_at += BLEED_TICK_INTERVAL;
        }
        true
    });
    drop(bleeds);

    FROZENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("frozen state poisoned")
        .retain(|state| {
            state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    for (caster_id, target_id, damage) in bleed_ticks {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        if try_wonder_guard_dot_tick(ctx, caster_id, actual_target_id, PokemonType::Normal) {
            if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, target_pos, 8500, VFX_BLEED);
            }
            continue;
        }
        let (ad_damage, ap_damage) = adjust_endure_damage(ctx, actual_target_id, damage, 0);
        crate::pokemon_status::deal_tracked_damage(
            ctx,
            caster_id,
            actual_target_id,
            ad_damage,
            ap_damage,
            AttackType::Dot,
        );
        if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 8500, VFX_BLEED);
        }
    }

    let infestations = INFESTATIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut infestations = infestations.lock().expect("infestation state poisoned");
    let mut infestation_ticks = Vec::new();
    infestations.retain_mut(|state| {
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !target.is_alive() || state.expires_at <= tick {
            return false;
        }

        while state.next_tick_at <= tick {
            infestation_ticks.push((
                state.caster_id,
                state.target_id,
                state.damage_per_tick,
                state.attacker_types,
            ));
            state.next_tick_at += INFESTATION_TICK_INTERVAL;
        }
        true
    });
    drop(infestations);

    for (caster_id, target_id, damage, attacker_types) in infestation_ticks {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        if try_wonder_guard_dot_tick(ctx, caster_id, actual_target_id, PokemonType::Bug) {
            if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, target_pos, 7000, VFX_BUG);
            }
            continue;
        }
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            damage.max(1),
            0,
            AttackType::Dot,
            PokemonType::Bug,
            attacker_types,
            defender_types,
        );
        if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 7000, VFX_BUG);
        }
    }

    update_ice_fields(ctx, tick, rng_seed);
    update_leech_seeds(ctx, tick);
    update_wish_channels(ctx, tick);
    update_power_up_punch_channels(ctx, tick);
    update_quick_return_dashes(ctx, tick);
    update_scheduled_force_aways(ctx, tick);
    update_orbeetle_agility_chains(ctx, tick);
    update_aqua_rings(ctx, tick);
    update_misty_terrains(ctx, tick);
    update_brine_fields(ctx, tick);
    update_grassy_terrains(ctx, tick);
    update_rillaboom_drum_auras(ctx, tick);
    update_rillaboom_grassy_surges(ctx, tick);
    update_shiftry_tornadoes(ctx, tick);
    update_quaquaval_aqua_step_segments(ctx, tick);
    update_sticky_webs(ctx, tick);
    update_whirlpools(ctx, tick);
    update_sing_auras(ctx, tick);
    update_frosmoth_sleep_circles(ctx, tick);
    update_bug_buzz_auras(ctx, tick);
    update_armarouge_mystical_fire_auras(ctx, tick);
    update_alluring_voice_auras(ctx, tick);
    update_blissey_heal_auras(ctx, tick);
    update_charm_heals(ctx, tick);
    update_stealth_rocks(ctx, tick);
    update_earthquakes(ctx, tick);
    update_blood_moons(ctx, tick);
    update_roosts(ctx, tick);
    update_flame_trails(ctx, tick);
    update_sawk_throh_forms(ctx);
    update_kommoo_duels(ctx, tick);
    update_grapploct_submissions(ctx, tick);
    update_coalossal_magma_storms(ctx, tick);

    BLAZE_CONTACTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("blaze contact state poisoned")
        .retain(|state| tick.saturating_sub(state.last_contact_tick) <= BLAZE_CHAIN_WINDOW_TICKS);

    ENTITY_TYPES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("entity type state poisoned")
        .retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    SHELL_ARMORS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("shell armor state poisoned")
        .retain(|entity_id| {
            ctx.get_entity(*entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    LIMBERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("limber state poisoned")
        .retain(|entity_id| {
            ctx.get_entity(*entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    HITMONTOPS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("hitmontop state poisoned")
        .retain(|entity_id| {
            ctx.get_entity(*entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    KILOWATTRELS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("kilowattrel state poisoned")
        .retain(|entity_id| {
            ctx.get_entity(*entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    DOT_IMMUNES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("dot immunity state poisoned")
        .retain(|entity_id| {
            ctx.get_entity(*entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    DOT_ABSORBERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("dot absorb state poisoned")
        .retain(|entity_id| {
            ctx.get_entity(*entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    HONEY_GATHERERS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("honey gatherer state poisoned")
        .retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    DIRECT_DAMAGE_LEDGER
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("direct damage ledger poisoned")
        .retain(|state| tick.saturating_sub(state.tick) <= 2);

    MEGA_LAUNCHER
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("mega launcher state poisoned")
        .retain(|state| tick.saturating_sub(state.last_hit_tick) <= MEGA_LAUNCHER_WINDOW_TICKS);

    SOFT_UNTARGETABLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("soft untargetable state poisoned")
        .retain(|state| {
            state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    WEAVILE_HUNTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("weavile hunt state poisoned")
        .retain(|state| {
            state.stealth_expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    CONFUSIONS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("confusion state poisoned")
        .retain(|state| {
            state.expires_at > tick
                && state.stacks > 0
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    FLAME_TRAILS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("flame trail state poisoned")
        .retain(|state| state.expires_at > tick);

    SPIKED_HIDE_REFLECTS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("spiked hide reflect state poisoned")
        .retain(|(_, _, reflect_tick)| *reflect_tick == tick);

    PYUKUMUKU_BARBS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("pyukumuku barb state poisoned")
        .retain(|state| {
            tick.saturating_sub(state.last_hit_tick) <= PYUKUMUKU_BARB_STACK_WINDOW_TICKS
                && ctx
                    .get_entity(state.defender_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
                && ctx
                    .get_entity(state.attacker_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    PHYSICAL_BASIC_HITS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("physical basic hit state poisoned")
        .retain(|(_, _, hit_tick)| tick.saturating_sub(*hit_tick) <= 1);

    STURDIES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("sturdy state poisoned")
        .retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    BATTLE_BONDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("battle bond state poisoned")
        .retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    SPIRIT_SHACKLES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("spirit shackle state poisoned")
        .retain(|state| {
            state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    SOAKS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("soak state poisoned")
        .retain(|state| {
            state.expires_at > tick
                && ctx
                    .get_entity(state.entity_id)
                    .map(|entity| entity.is_alive())
                    .unwrap_or(false)
        });

    BRINE_FIELDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("brine field state poisoned")
        .retain(|state| state.expires_at > tick);

    WILL_O_WISP_CHARGES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("will-o-wisp charge state poisoned")
        .retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    PORYGON_TYPES
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("porygon type state poisoned")
        .retain(|state| {
            ctx.get_entity(state.entity_id)
                .map(|entity| entity.is_alive())
                .unwrap_or(false)
        });

    WHIRLPOOLS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("whirlpool state poisoned")
        .retain(|state| state.expires_at > tick);
}

fn update_whirlpools(ctx: &mut GameCtx, tick: usize) {
    let whirlpools = WHIRLPOOLS.get_or_init(|| Mutex::new(Vec::new()));
    let mut whirlpools = whirlpools.lock().expect("whirlpool state poisoned");
    let mut hits = Vec::new();
    let mut self_ticks = Vec::new();
    let mut visuals = Vec::new();

    whirlpools.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }

        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        while state.next_tick_at <= tick {
            state.next_tick_at += WHIRLPOOL_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive() || entity.team() == state.caster_team {
                continue;
            }
            if distance_sq(entity.pos(), state.center) <= state.radius.saturating_mul(state.radius)
            {
                hits.push((
                    state.caster_id,
                    entity.id(),
                    state.damage_per_tick,
                    state.slow_percent,
                    state.slow_ticks,
                    state.attacker_types,
                ));
            }
        }

        if distance_sq(caster_pos, state.center) <= state.radius.saturating_mul(state.radius) {
            self_ticks.push((
                state.caster_id,
                state.self_heal_per_tick,
                state.self_attack_mult,
                state.self_buff_ticks,
            ));
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.id() == state.caster_id
                || !entity.is_alive()
                || !entity.is_champion()
                || !receiver_has_copied(entity.id(), "pokemon_moba_gyarados")
                || distance_sq(entity.pos(), state.center)
                    > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            self_ticks.push((
                entity.id(),
                state.self_heal_per_tick,
                state.self_attack_mult,
                state.self_buff_ticks,
            ));
        }

        true
    });
    drop(whirlpools);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_WATER);
    }

    for (caster_id, target_id, damage, slow_percent, slow_ticks, attacker_types) in hits {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage,
            AttackType::Dot,
            PokemonType::Water,
            attacker_types,
            defender_types,
        );
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: slow_ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }

    for (caster_id, heal, attack_mult, buff_ticks) in self_ticks {
        if heal > 0 {
            heal_with_antiheal(ctx, caster_id, caster_id, heal);
        }
        ctx.add_buff(
            caster_id,
            BuffState {
                duration: BuffType::Time { tick: buff_ticks },
                attack_mult,
                ..Default::default()
            },
        );
    }
}

fn update_flame_trails(ctx: &mut GameCtx, tick: usize) {
    let trails = FLAME_TRAILS.get_or_init(|| Mutex::new(Vec::new()));
    let mut trails = trails.lock().expect("flame trail state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    trails.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        visuals.push((state.start, state.end, state.width));
        if state.next_tick_at > tick {
            return true;
        }

        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        drop(caster);

        while state.next_tick_at <= tick {
            state.next_tick_at += FLAME_TRAIL_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || distance_to_segment_sq(entity.pos(), state.start, state.end)
                    > state.width.saturating_mul(state.width)
            {
                continue;
            }
            hits.push((
                state.caster_id,
                entity.id(),
                state.damage_per_tick,
                state.attacker_types,
                state.burn_chance_percent,
                state.burn_ticks,
                state.burn_damage,
            ));
        }

        true
    });
    drop(trails);

    for (start, end, width) in visuals {
        draw_line_band(ctx, start, end, width, VFX_FIRE);
    }

    for (
        caster_id,
        target_id,
        damage,
        attacker_types,
        burn_chance_percent,
        burn_ticks,
        burn_damage,
    ) in hits
    {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            damage,
            0,
            AttackType::Dot,
            PokemonType::Fire,
            attacker_types,
            defender_types,
        );
        if burn_chance_percent > 0
            && splitmix64(
                ctx.seed()
                    ^ ((caster_id as u64) << 40)
                    ^ ((target_id as u64) << 16)
                    ^ ctx.tick() as u64
                    ^ 0xf1a4e_u64,
            ) % 100
                < burn_chance_percent as u64
        {
            apply_burn_for(ctx, caster_id, target_id, burn_damage.max(1), burn_ticks);
        }
    }
}

fn update_charm_heals(ctx: &mut GameCtx, tick: usize) {
    let heals = CHARM_HEALS.get_or_init(|| Mutex::new(Vec::new()));
    let mut heals = heals.lock().expect("charm heal state poisoned");
    let mut heal_ticks = Vec::new();

    heals.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !caster.is_alive()
            || !target.is_alive()
            || caster.team() != target.team()
            || state.expires_at <= tick
        {
            return false;
        }

        while state.next_tick_at <= tick {
            heal_ticks.push((state.caster_id, state.target_id, state.heal_per_tick));
            state.next_tick_at += CHARM_HEAL_INTERVAL_TICKS;
        }
        true
    });
    drop(heals);

    for (caster_id, target_id, heal) in heal_ticks {
        heal_with_antiheal(ctx, caster_id, target_id, heal.max(1));
        after_blissey_heal(ctx, caster_id, target_id);
        if let Some(target_pos) = ctx.get_entity(target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 8500, VFX_FAIRY);
        }
    }
}

fn update_blissey_heal_auras(ctx: &mut GameCtx, tick: usize) {
    let auras = BLISSEY_HEAL_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("blissey heal aura state poisoned");
    let mut heals = Vec::new();
    let mut visuals = Vec::new();

    auras.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);
        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += MISTY_TERRAIN_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.is_alive()
                && entity.team() == state.caster_team
                && entity.is_champion()
                && distance_sq(entity.pos(), caster_pos)
                    <= state.radius.saturating_mul(state.radius)
            {
                heals.push((state.caster_id, entity.id(), state.heal_per_tick));
            }
        }
        true
    });
    drop(auras);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_PSYCHIC);
    }
    for (caster_id, target_id, heal) in heals {
        heal_with_antiheal(ctx, caster_id, target_id, heal.max(1));
        after_blissey_heal(ctx, caster_id, target_id);
    }
}

fn update_misty_terrains(ctx: &mut GameCtx, tick: usize) {
    let terrains = MISTY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut terrains = terrains.lock().expect("misty terrain state poisoned");
    let mut heals = Vec::new();
    let mut slows = Vec::new();
    let mut visuals = Vec::new();

    terrains.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }

        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        drop(caster);

        while state.next_tick_at <= tick {
            state.next_tick_at += MISTY_TERRAIN_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || distance_sq(entity.pos(), state.center)
                    > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            if entity.team() == state.caster_team {
                heals.push((state.caster_id, entity.id(), state.heal_per_tick));
            } else {
                slows.push((entity.id(), state.slow_percent, state.slow_ticks));
            }
        }

        true
    });
    drop(terrains);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_FAIRY);
    }
    for (caster_id, target_id, heal) in heals {
        heal_with_antiheal(ctx, caster_id, target_id, heal.max(1));
        after_blissey_heal(ctx, caster_id, target_id);
    }
    for (target_id, slow_percent, ticks) in slows {
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }
}

fn update_brine_fields(ctx: &mut GameCtx, tick: usize) {
    let fields = BRINE_FIELDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("brine field state poisoned");
    let mut slows = Vec::new();
    let mut visuals = Vec::new();

    fields.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }

        while state.next_tick_at <= tick {
            state.next_tick_at += BRINE_FIELD_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.is_alive()
                && distance_sq(entity.pos(), state.center)
                    <= state.radius.saturating_mul(state.radius)
            {
                slows.push((entity.id(), state.slow_percent, state.slow_ticks));
            }
        }

        true
    });
    drop(fields);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_WATER);
    }
    for (target_id, slow_percent, ticks) in slows {
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }
}

fn update_rillaboom_drum_auras(ctx: &mut GameCtx, tick: usize) {
    let auras = RILLABOOM_DRUM_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("rillaboom drum aura state poisoned");
    let mut ally_heals = Vec::new();
    let mut ally_speeds = Vec::new();
    let mut enemy_slows = Vec::new();
    let mut final_stuns = Vec::new();
    let mut visuals = Vec::new();

    auras.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        if state.expires_at <= tick {
            if state.final_stun_ticks > 0 {
                for index in 0..ctx.entity_count() {
                    let Some(entity) = ctx.entity_at(index) else {
                        continue;
                    };
                    if !entity.is_alive()
                        || entity.team() == state.caster_team
                        || distance_sq(entity.pos(), caster_pos)
                            > state.radius.saturating_mul(state.radius)
                    {
                        continue;
                    }
                    final_stuns.push((state.caster_id, entity.id(), state.final_stun_ticks));
                }
            }
            return false;
        }

        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += MISTY_TERRAIN_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || distance_sq(entity.pos(), caster_pos) > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            if entity.team() == state.caster_team && entity.is_champion() {
                ally_heals.push((state.caster_id, entity.id(), state.heal_per_tick));
                if state.ally_move_speed_mult != 0 {
                    ally_speeds.push((
                        state.caster_id,
                        entity.id(),
                        state.ally_move_speed_mult,
                        state.ally_buff_ticks,
                    ));
                }
            } else if entity.team() != state.caster_team && state.enemy_slow_percent != 0 {
                enemy_slows.push((
                    entity.id(),
                    state.enemy_slow_percent,
                    state.enemy_slow_ticks,
                ));
            }
        }

        true
    });
    drop(auras);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_GRASS);
    }
    for (caster_id, target_id, amount) in ally_heals {
        let _ = heal_with_antiheal(ctx, caster_id, target_id, amount);
    }
    for (caster_id, target_id, move_speed_mult, ticks) in ally_speeds {
        add_beneficial_buff(
            ctx,
            caster_id,
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                move_speed_mult,
                ..Default::default()
            },
        );
    }
    for (target_id, slow_percent, ticks) in enemy_slows {
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }
    for (caster_id, target_id, ticks) in final_stuns {
        let ticks = adjusted_cc_ticks(ctx, target_id, ticks);
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Stun { tick: ticks as u64 },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill { tick: ticks },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockAttack { tick: ticks },
        );
    }
}

fn update_rillaboom_grassy_surges(ctx: &mut GameCtx, tick: usize) {
    let surges = RILLABOOM_GRASSY_SURGES.get_or_init(|| Mutex::new(Vec::new()));
    let mut surges = surges
        .lock()
        .expect("rillaboom grassy surge state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    surges.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        drop(caster);

        visuals.push((state.start, state.end, state.width));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += state.tick_interval;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || distance_to_segment_sq(entity.pos(), state.start, state.end)
                    > state.width.saturating_mul(state.width)
            {
                continue;
            }
            hits.push((
                state.caster_id,
                entity.id(),
                state.damage_per_tick,
                state.slow_percent,
                state.slow_ticks,
            ));
        }

        true
    });
    drop(surges);

    for (start, end, width) in visuals {
        draw_line_band(ctx, start, end, width, VFX_GRASS);
    }
    for (caster_id, target_id, damage, slow_percent, slow_ticks) in hits {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            0,
            damage.max(1),
            AttackType::Dot,
            PokemonType::Grass,
            TypeSet::single(PokemonType::Grass),
            defender_types,
        );
        ctx.add_buff(
            actual_target_id,
            BuffState {
                duration: BuffType::Time { tick: slow_ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
    }
}

fn update_shiftry_tornadoes(ctx: &mut GameCtx, tick: usize) {
    let states = SHIFTRY_TORNADOES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("shiftry tornado state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        drop(caster);

        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            let damage = state.damage_per_tick.saturating_mul(
                100usize
                    .saturating_add(state.ticks_done.saturating_mul(state.damage_growth_percent)),
            ) / 100;
            for index in 0..ctx.entity_count() {
                let Some(entity) = ctx.entity_at(index) else {
                    continue;
                };
                if !entity.is_alive()
                    || entity.team() == state.caster_team
                    || entity.is_tower()
                    || distance_sq(entity.pos(), state.center)
                        > state.radius.saturating_mul(state.radius)
                {
                    continue;
                }
                hits.push((
                    state.caster_id,
                    entity.id(),
                    damage.max(1),
                    state.lift_ticks,
                ));
            }
            state.ticks_done = state.ticks_done.saturating_add(1);
            state.next_tick_at = state.next_tick_at.saturating_add(state.tick_interval);
        }

        true
    });
    drop(states);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_GRASS);
    }
    for (caster_id, target_id, damage, lift_ticks) in hits {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id);
        let attacker_types = entity_types(caster_id)
            .unwrap_or_else(|| TypeSet::dual(PokemonType::Grass, PokemonType::Dark));
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            damage,
            0,
            AttackType::Dot,
            PokemonType::Grass,
            attacker_types,
            defender_types,
        );
        apply_airborne_hard_cc(ctx, caster_id, actual_target_id, lift_ticks);
    }
}

fn update_grassy_terrains(ctx: &mut GameCtx, tick: usize) {
    let fields = GRASSY_TERRAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("grassy terrain state poisoned");
    let mut buffs = Vec::new();
    let mut visuals = Vec::new();

    fields.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += GRASSY_TERRAIN_INTERVAL_TICKS;
        }

        if distance_sq(caster_pos, state.center) <= state.radius.saturating_mul(state.radius) {
            buffs.push((state.caster_id, state.attack_speed_mult, state.buff_ticks));
        }

        true
    });
    drop(fields);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_GRASS);
    }
    for (target_id, attack_speed_mult, ticks) in buffs {
        add_beneficial_buff(
            ctx,
            target_id,
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                attack_speed_mult,
                ..Default::default()
            },
        );
    }
}

fn update_sticky_webs(ctx: &mut GameCtx, tick: usize) {
    let webs = STICKY_WEBS.get_or_init(|| Mutex::new(Vec::new()));
    let mut webs = webs.lock().expect("sticky web state poisoned");
    let mut buffs = Vec::new();
    let mut visuals = Vec::new();

    webs.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        drop(caster);

        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += STICKY_WEB_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || distance_sq(entity.pos(), state.center)
                    > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            let speed_mult = if entity.id() == state.caster_id {
                state.kricketune_speed_percent
            } else if receiver_has_copied(entity.id(), "pokemon_moba_kricketune") {
                state.kricketune_speed_percent
            } else if entity.team() != state.caster_team {
                -state.enemy_slow_percent
            } else {
                continue;
            };
            buffs.push((entity.id(), speed_mult, state.buff_ticks));
        }

        true
    });
    let web_snapshot = webs.clone();
    drop(webs);

    update_web_walker_spots(ctx, tick, &web_snapshot);
    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_BUG);
    }
    for (target_id, move_speed_mult, ticks) in buffs {
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                move_speed_mult,
                ..Default::default()
            },
        );
    }
}

fn update_sing_auras(ctx: &mut GameCtx, tick: usize) {
    let auras = SING_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("sing aura state poisoned");
    let mut sleeps = Vec::new();
    let mut visuals = Vec::new();

    auras.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += KRICKETUNE_AURA_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || distance_sq(entity.pos(), caster_pos) > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            sleeps.push((state.caster_id, entity.id(), state.sleep_ticks));
        }

        true
    });
    drop(auras);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_NORMAL);
    }
    for (caster_id, target_id, ticks) in sleeps {
        let ticks = adjusted_cc_ticks(ctx, target_id, ticks);
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Stun { tick: ticks as u64 },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill { tick: ticks },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockAttack { tick: ticks },
        );
        note_steadfast_cc(ctx, target_id);
    }
}

fn update_frosmoth_sleep_circles(ctx: &mut GameCtx, tick: usize) {
    let circles = FROSMOTH_SLEEP_CIRCLES.get_or_init(|| Mutex::new(Vec::new()));
    let mut circles = circles.lock().expect("frosmoth sleep state poisoned");
    let mut sleeps = Vec::new();
    let mut visuals = Vec::new();
    let mut moves = Vec::new();

    circles.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        visuals.push((state.center, state.radius));

        if state.next_move_at <= tick {
            let target_pos = if state.path_index < FROSMOTH_SLEEP_PATH_POINTS {
                frosmoth_sleep_path_point(
                    state.center,
                    state.radius,
                    state.path_index,
                    FROSMOTH_SLEEP_PATH_POINTS,
                )
            } else {
                state.origin
            };
            moves.push((
                state.caster_id,
                caster_pos,
                target_pos,
                state.force_move_speed,
                state.force_move_ticks,
            ));
            state.path_index = state.path_index.saturating_add(1);
            state.next_move_at = tick.saturating_add(FROSMOTH_SLEEP_MOVE_INTERVAL_TICKS);
        }

        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += FROSMOTH_SLEEP_INTERVAL_TICKS;
        }

        let radius_sq = state.radius.saturating_mul(state.radius);
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || distance_sq(entity.pos(), state.center) > radius_sq
            {
                continue;
            }
            sleeps.push((state.caster_id, entity.id(), state.sleep_ticks));
        }

        true
    });
    drop(circles);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_BUG);
    }
    for (entity_id, current_pos, target_pos, speed, ticks) in moves {
        let dx = target_pos.x as i64 - current_pos.x as i64;
        let dy = target_pos.y as i64 - current_pos.y as i64;
        apply_pokemon_cc(
            ctx,
            entity_id,
            entity_id,
            CCState::ForceMove {
                tick: ticks,
                dx,
                dy,
                speed,
            },
        );
        draw_line_band(ctx, current_pos, target_pos, 6000, VFX_BUG);
    }
    for (caster_id, target_id, ticks) in sleeps {
        let ticks = adjusted_cc_ticks(ctx, target_id, ticks);
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Stun { tick: ticks as u64 },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill { tick: ticks },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockAttack { tick: ticks },
        );
        note_steadfast_cc(ctx, target_id);
    }
}

fn update_bug_buzz_auras(ctx: &mut GameCtx, tick: usize) {
    let auras = BUG_BUZZ_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("bug buzz aura state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    auras.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += KRICKETUNE_AURA_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || distance_sq(entity.pos(), caster_pos) > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            hits.push((
                state.caster_id,
                entity.id(),
                state.damage_per_tick,
                state.confusion_stacks,
                state.confusion_ticks,
                state.attacker_types,
            ));
        }

        true
    });
    drop(auras);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_BUG);
    }
    for (caster_id, target_id, damage, confusion_stacks, confusion_ticks, attacker_types) in hits {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Bug,
            attacker_types,
            defender_types,
        );
        apply_confusion(ctx, target_id, confusion_stacks, confusion_ticks);
    }
}

fn update_armarouge_mystical_fire_auras(ctx: &mut GameCtx, tick: usize) {
    let auras = ARMAROUGE_MYSTICAL_FIRE_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras
        .lock()
        .expect("armarouge mystical fire state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    auras.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        visuals.push((caster_pos, state.radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at = state.next_tick_at.saturating_add(state.tick_interval);
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || entity.is_tower()
                || !entity.is_targetable()
                || distance_sq(entity.pos(), caster_pos) > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            hits.push((
                state.caster_id,
                entity.id(),
                state.damage,
                state.burn_chance_percent,
                state.burn_ticks,
                state.burn_damage,
                state.confusion_chance_percent,
                state.confusion_stacks,
                state.confusion_ticks,
                state.attacker_types,
            ));
        }

        true
    });
    drop(auras);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_FIRE);
    }

    for (
        caster_id,
        target_id,
        damage,
        burn_chance_percent,
        burn_ticks,
        burn_damage,
        confusion_chance_percent,
        confusion_stacks,
        confusion_ticks,
        attacker_types,
    ) in hits
    {
        let actual_target_id = audino_protect_redirect(ctx, target_id).unwrap_or(target_id);
        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, actual_target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            actual_target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Fire,
            attacker_types,
            defender_types,
        );
        note_armarouge_landed_attack(ctx, caster_id);

        if burn_chance_percent >= 100
            || splitmix64(
                ctx.seed()
                    ^ ((caster_id as u64) << 40)
                    ^ ((actual_target_id as u64) << 16)
                    ^ tick as u64
                    ^ 0xa661_u64,
            ) % 100
                < burn_chance_percent as u64
        {
            apply_burn_for(
                ctx,
                caster_id,
                actual_target_id,
                burn_damage.max(1),
                burn_ticks,
            );
        }
        if confusion_chance_percent >= 100
            || splitmix64(
                ctx.seed()
                    ^ ((caster_id as u64) << 40)
                    ^ ((actual_target_id as u64) << 16)
                    ^ tick as u64
                    ^ 0xc0f5_u64,
            ) % 100
                < confusion_chance_percent as u64
        {
            apply_confusion_from(
                ctx,
                caster_id,
                actual_target_id,
                confusion_stacks,
                confusion_ticks,
            );
        }
        if let Some(target_pos) = ctx.get_entity(actual_target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 9500, VFX_FIRE);
        }
    }
}

fn update_alluring_voice_auras(ctx: &mut GameCtx, tick: usize) {
    let auras = ALLURING_VOICE_AURAS.get_or_init(|| Mutex::new(Vec::new()));
    let mut auras = auras.lock().expect("alluring voice aura state poisoned");
    let mut taunts = Vec::new();
    let mut confusions = Vec::new();
    let mut visuals = Vec::new();

    auras.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }
        let caster_pos = caster.pos();
        drop(caster);

        visuals.push((caster_pos, state.outer_radius, state.inner_radius));
        if state.next_tick_at > tick {
            return true;
        }
        while state.next_tick_at <= tick {
            state.next_tick_at += KRICKETUNE_AURA_INTERVAL_TICKS;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive() || entity.team() == state.caster_team {
                continue;
            }
            let dist = distance_sq(entity.pos(), caster_pos);
            if dist <= state.outer_radius.saturating_mul(state.outer_radius) {
                taunts.push((state.caster_id, entity.id(), state.taunt_ticks));
            }
            if dist <= state.inner_radius.saturating_mul(state.inner_radius) {
                confusions.push((entity.id(), state.confusion_stacks, state.confusion_ticks));
            }
        }

        true
    });
    drop(auras);

    for (center, outer_radius, inner_radius) in visuals {
        draw_field_circle(ctx, center, outer_radius, VFX_FAIRY);
        draw_field_circle(ctx, center, inner_radius, VFX_BUG);
    }
    for (caster_id, target_id, taunt_ticks) in taunts {
        let ticks = adjusted_cc_ticks(ctx, target_id, taunt_ticks);
        break_kommoo_duel_on_hard_cc(ctx, caster_id, target_id);
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::Taunt {
                tick: ticks as u64,
                target: caster_id,
            },
        );
        note_steadfast_cc(ctx, target_id);
    }
    for (target_id, stacks, confusion_ticks) in confusions {
        apply_confusion(ctx, target_id, stacks, confusion_ticks);
    }
}

fn update_thievul_stakeouts(ctx: &mut GameCtx, tick: usize) {
    let states = THIEVUL_STAKEOUTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("thievul stakeout state poisoned");
    let mut pulses = Vec::new();

    states.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if state.next_tick_at <= tick {
            pulses.push(*state);
            state.next_tick_at = tick.saturating_add(30);
        }
        true
    });
    drop(states);

    for state in pulses {
        let targets: Vec<(usize, EntityPos)> = (0..ctx.entity_count())
            .filter_map(|index| ctx.entity_at(index))
            .filter(|entity| {
                entity.team() != state.caster_team
                    && entity.is_alive()
                    && !entity.is_tower()
                    && distance_sq(entity.pos(), state.center)
                        <= state.radius.saturating_mul(state.radius)
            })
            .map(|entity| (entity.id(), entity.pos()))
            .collect();
        for (target_id, target_pos) in targets {
            let ticks = adjusted_cc_ticks(ctx, target_id, state.root_ticks);
            if ticks > 0 {
                break_kommoo_duel_on_hard_cc(ctx, state.caster_id, target_id);
                apply_pokemon_cc(
                    ctx,
                    state.caster_id,
                    target_id,
                    CCState::Bind { tick: ticks as u64 },
                );
                note_steadfast_cc(ctx, target_id);
            }
            draw_status_marker(ctx, target_pos, 8500, VFX_DARK);
        }
        draw_field_circle(ctx, state.center, state.radius, VFX_DARK);
    }
}

fn update_aqua_rings(ctx: &mut GameCtx, tick: usize) {
    let rings = AQUA_RINGS.get_or_init(|| Mutex::new(Vec::new()));
    let mut rings = rings.lock().expect("aqua ring state poisoned");
    let mut heals = Vec::new();
    let mut debuffs = Vec::new();
    let mut visuals = Vec::new();

    rings.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            return false;
        }
        if state.next_tick_at > tick {
            return true;
        }

        let caster_pos = caster.pos();
        let caster_team = state.caster_team;
        visuals.push((caster_pos, state.radius));
        while state.next_tick_at <= tick {
            state.next_tick_at += AQUA_RING_INTERVAL_TICKS;
        }
        drop(caster);

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || distance_sq(entity.pos(), caster_pos) > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            if entity.team() == caster_team {
                heals.push((state.caster_id, entity.id(), state.heal_per_tick));
            } else {
                debuffs.push((
                    entity.id(),
                    state.enemy_attack_mult,
                    state.enemy_debuff_ticks,
                ));
            }
        }

        true
    });
    drop(rings);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_WATER);
    }
    for (caster_id, target_id, heal) in heals {
        heal_with_antiheal(ctx, caster_id, target_id, heal.max(1));
        after_blissey_heal(ctx, caster_id, target_id);
    }
    for (target_id, attack_mult, ticks) in debuffs {
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: ticks },
                attack_mult,
                ..Default::default()
            },
        );
    }
}

fn update_quick_return_dashes(ctx: &mut GameCtx, tick: usize) {
    let states = QUICK_RETURN_DASHES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("quick return dash state poisoned");
    let mut returns = Vec::new();

    states.retain(|state| {
        let Some(entity) = ctx.get_entity(state.entity_id) else {
            return false;
        };
        if !entity.is_alive() {
            return false;
        }
        if state.trigger_at > tick {
            return true;
        }
        returns.push((
            state.entity_id,
            entity.pos(),
            state.origin,
            state.speed,
            state.ticks,
        ));
        false
    });
    drop(states);

    for (entity_id, current_pos, origin, speed, ticks) in returns {
        let dx = origin.x as i64 - current_pos.x as i64;
        let dy = origin.y as i64 - current_pos.y as i64;
        apply_pokemon_cc(
            ctx,
            entity_id,
            entity_id,
            CCState::ForceMove {
                tick: ticks,
                dx,
                dy,
                speed,
            },
        );
        draw_line_band(ctx, current_pos, origin, 6000, VFX_NORMAL);
    }
}

fn update_scheduled_force_aways(ctx: &mut GameCtx, tick: usize) {
    let states = SCHEDULED_FORCE_AWAYS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("scheduled force-away state poisoned");
    let mut moves = Vec::new();

    states.retain(|state| {
        let Some(entity) = ctx.get_entity(state.entity_id) else {
            return false;
        };
        if !entity.is_alive() {
            return false;
        }
        if state.trigger_at > tick {
            return true;
        }
        moves.push((
            state.entity_id,
            entity.pos(),
            state.away_from,
            state.speed,
            state.ticks,
        ));
        false
    });
    drop(states);

    for (entity_id, current_pos, away_from, speed, ticks) in moves {
        let dx = current_pos.x as i64 - away_from.x as i64;
        let dy = current_pos.y as i64 - away_from.y as i64;
        apply_pokemon_cc(
            ctx,
            entity_id,
            entity_id,
            CCState::ForceMove {
                tick: ticks,
                dx,
                dy,
                speed,
            },
        );
        draw_line_band(ctx, away_from, current_pos, 6000, VFX_WATER);
    }
}

fn update_orbeetle_agility_chains(ctx: &mut GameCtx, tick: usize) {
    let states = ORBEETLE_AGILITY_CHAINS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("orbeetle agility state poisoned");
    let mut hops = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            return false;
        }
        let caster_team = caster.team();
        drop(caster);

        if state.next_hop_at > tick {
            return true;
        }

        while state.next_index < state.targets.len() {
            let target_id = state.targets[state.next_index];
            state.next_index += 1;

            let Some(target) = ctx.get_entity(target_id) else {
                continue;
            };
            if target.team() == caster_team
                || !target.is_alive()
                || !target.is_targetable()
                || is_soft_untargetable(ctx, target_id)
            {
                continue;
            }

            hops.push((
                state.caster_id,
                target_id,
                state.damage,
                state.attacker_types,
                state.force_move_speed,
                state.force_move_ticks,
            ));
            state.next_hop_at = tick.saturating_add(state.hop_interval_ticks);
            return state.next_index < state.targets.len();
        }

        false
    });
    drop(states);

    for (caster_id, target_id, damage, attacker_types, force_move_speed, force_move_ticks) in hops {
        let Some(caster) = ctx.get_entity(caster_id) else {
            continue;
        };
        if !caster.is_alive() {
            continue;
        }
        let caster_pos = caster.pos();
        drop(caster);

        let Some(target) = ctx.get_entity(target_id) else {
            continue;
        };
        if !target.is_alive() {
            continue;
        }
        let target_pos = target.pos();
        drop(target);

        let dx = target_pos.x as i64 - caster_pos.x as i64;
        let dy = target_pos.y as i64 - caster_pos.y as i64;
        apply_pokemon_cc(
            ctx,
            caster_id,
            caster_id,
            CCState::ForceMove {
                tick: force_move_ticks,
                dx,
                dy,
                speed: force_move_speed,
            },
        );
        draw_line_band(ctx, caster_pos, target_pos, 6500, VFX_PSYCHIC);

        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Psychic,
            attacker_types,
            defender_types,
        );
    }
}

fn update_stealth_rocks(ctx: &GameCtx, tick: usize) {
    let states = STEALTH_ROCKS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("stealth rock state poisoned");
    let mut apply_windows = Vec::new();

    states.retain_mut(|state| {
        let Some(entity) = ctx.get_entity(state.entity_id) else {
            return false;
        };
        if !entity.is_alive() || state.expires_at <= tick {
            return false;
        }
        if state.next_toggle_at <= tick {
            state.untargetable = !state.untargetable;
            state.next_toggle_at = state.next_toggle_at.saturating_add(2 * 60);
        }
        if state.untargetable {
            let window = state.next_toggle_at.saturating_sub(tick).max(1).min(2 * 60);
            apply_windows.push((state.entity_id, window));
        }
        true
    });
    drop(states);

    for (entity_id, window) in apply_windows {
        apply_soft_untargetable(ctx, entity_id, window);
    }
}

fn update_earthquakes(ctx: &mut GameCtx, tick: usize) {
    let states = EARTHQUAKES.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("earthquake state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() || state.expires_at <= tick {
            return false;
        }
        let caster_pos = caster.pos();
        visuals.push((caster_pos, state.radius));

        while state.next_tick_at <= tick {
            for index in 0..ctx.entity_count() {
                let Some(target) = ctx.entity_at(index) else {
                    continue;
                };
                if target.team() == state.caster_team || !target.is_alive() {
                    continue;
                }
                if distance_sq(target.pos(), caster_pos)
                    <= state.radius.saturating_mul(state.radius)
                {
                    hits.push((
                        state.caster_id,
                        target.id(),
                        state.damage_per_tick,
                        state.slow_percent,
                        state.slow_ticks,
                        state.attacker_types,
                    ));
                }
            }
            state.next_tick_at = state.next_tick_at.saturating_add(state.tick_interval);
        }

        true
    });
    drop(states);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_GROUND);
    }

    for (caster_id, target_id, damage, slow_percent, slow_ticks, attacker_types) in hits {
        let defender_types = crate::neutral_objectives::defender_types_for_target(ctx, target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            caster_id,
            target_id,
            damage.max(1),
            0,
            AttackType::Skill,
            PokemonType::Ground,
            attacker_types,
            defender_types,
        );
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time { tick: slow_ticks },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
        if let Some(target_pos) = ctx.get_entity(target_id).map(|target| target.pos()) {
            draw_status_marker(ctx, target_pos, 9500, VFX_GROUND);
        }
    }
}

fn update_blood_moons(ctx: &mut GameCtx, tick: usize) {
    let states = BLOOD_MOONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("blood moon state poisoned");
    let mut ticks = Vec::new();
    let mut visuals = Vec::new();

    states.retain_mut(|state| {
        let Some(entity) = ctx.get_entity(state.entity_id) else {
            return false;
        };
        if !entity.is_alive() || state.expires_at <= tick {
            return false;
        }
        let pos = entity.pos();
        let hp = entity.hp();
        drop(entity);
        visuals.push(pos);
        while state.next_tick_at <= tick {
            let damage = hp
                .max
                .saturating_mul(state.hp_loss_per_second_percent)
                .saturating_mul(BURN_TICK_INTERVAL)
                .saturating_div(100 * 60)
                .max(1);
            ticks.push((state.entity_id, damage));
            state.next_tick_at = state.next_tick_at.saturating_add(BURN_TICK_INTERVAL);
        }
        true
    });
    drop(states);

    for pos in visuals {
        draw_status_marker(ctx, pos, 14000, VFX_BLEED);
    }
    for (entity_id, damage) in ticks {
        crate::pokemon_status::deal_tracked_damage(
            ctx,
            entity_id,
            entity_id,
            damage,
            0,
            AttackType::Dot,
        );
    }
}

fn update_roosts(ctx: &mut GameCtx, tick: usize) {
    let states = ROOSTS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("roost state poisoned");
    let mut heals = Vec::new();
    let mut restores = Vec::new();

    states.retain_mut(|state| {
        let Some(entity) = ctx.get_entity(state.entity_id) else {
            restores.push((state.entity_id, state.original_types));
            return false;
        };
        if !entity.is_alive() {
            restores.push((state.entity_id, state.original_types));
            return false;
        }
        if state.expires_at <= tick {
            restores.push((state.entity_id, state.original_types));
            return false;
        }
        while state.next_tick_at <= tick {
            heals.push((state.entity_id, state.heal_per_tick));
            state.next_tick_at += 30;
        }
        true
    });
    drop(states);

    for (entity_id, heal) in heals {
        heal_with_antiheal(ctx, entity_id, entity_id, heal.max(1));
        if let Some(pos) = ctx.get_entity(entity_id).map(|entity| entity.pos()) {
            draw_status_marker(ctx, pos, 9000, VFX_STEEL);
        }
    }
    for (entity_id, types) in restores {
        register_entity_types(entity_id, types);
    }
}

fn update_ice_fields(ctx: &mut GameCtx, tick: usize, rng_seed: u64) {
    let fields = ICE_FIELDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut fields = fields.lock().expect("ice field state poisoned");
    let mut hits = Vec::new();
    let mut visuals = Vec::new();

    fields.retain_mut(|state| {
        if state.expires_at <= tick {
            return false;
        }
        visuals.push((state.center, state.radius));
        if state.next_tick_at > tick {
            return true;
        }

        while state.next_tick_at <= tick {
            state.next_tick_at += ICE_FIELD_TICK_INTERVAL;
        }

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if !entity.is_alive()
                || entity.team() == state.caster_team
                || distance_sq(entity.pos(), state.center)
                    > state.radius.saturating_mul(state.radius)
            {
                continue;
            }
            hits.push((
                state.caster_id,
                entity.id(),
                state.damage_per_tick,
                state.freeze_chance_percent,
                state.freeze_ticks,
                state.slow_percent,
                state.control_ticks,
            ));
        }

        true
    });
    drop(fields);

    for (center, radius) in visuals {
        draw_field_circle(ctx, center, radius, VFX_ICE);
    }
    for (
        caster_id,
        target_id,
        damage,
        freeze_chance_percent,
        freeze_ticks,
        slow_percent,
        control_ticks,
    ) in hits
    {
        if damage > 0 {
            crate::pokemon_status::deal_tracked_damage(
                ctx,
                caster_id,
                target_id,
                damage,
                0,
                AttackType::Dot,
            );
        }
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockAttack {
                tick: control_ticks,
            },
        );
        apply_pokemon_cc(
            ctx,
            caster_id,
            target_id,
            CCState::BlockSkill {
                tick: control_ticks,
            },
        );
        ctx.add_buff(
            target_id,
            BuffState {
                duration: BuffType::Time {
                    tick: control_ticks,
                },
                move_speed_mult: -slow_percent,
                ..Default::default()
            },
        );
        if chance_percent(rng_seed, caster_id, target_id, tick, freeze_chance_percent) {
            apply_frozen_for(ctx, target_id, freeze_ticks);
        }
    }
}

fn update_wish_channels(ctx: &mut GameCtx, tick: usize) {
    let channels = WISH_CHANNELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut channels = channels.lock().expect("wish channel state poisoned");
    let mut heals = Vec::new();
    let mut visuals = Vec::new();

    channels.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !caster.is_alive() || !target.is_alive() {
            return false;
        }

        let elapsed = tick.saturating_sub(state.started_at);
        visuals.push((caster.pos(), target.pos()));
        let moved = distance_sq(caster.pos(), state.last_pos)
            > WISH_MOVE_THRESHOLD.saturating_mul(WISH_MOVE_THRESHOLD);
        state.last_pos = caster.pos();

        if moved || elapsed >= state.max_ticks {
            let effective_ticks = elapsed.min(state.max_ticks).max(1);
            let heal = state.heal_amount.saturating_mul(effective_ticks) / state.max_ticks.max(1);
            heals.push((state.caster_id, state.target_id, heal.max(1)));
            return false;
        }

        true
    });
    drop(channels);

    for (caster_pos, target_pos) in visuals {
        draw_line_band(ctx, caster_pos, target_pos, 5000, VFX_NORMAL);
        draw_status_marker(ctx, target_pos, 9000, VFX_NORMAL);
    }
    for (caster_id, target_id, heal) in heals {
        heal_with_antiheal(ctx, caster_id, target_id, heal);
    }
}

fn update_power_up_punch_channels(ctx: &mut GameCtx, tick: usize) {
    let channels = POWER_UP_PUNCH_CHANNELS.get_or_init(|| Mutex::new(Vec::new()));
    let mut channels = channels
        .lock()
        .expect("power-up punch channel state poisoned");
    let mut resolutions = Vec::new();
    let mut visuals = Vec::new();

    channels.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        if !caster.is_alive() {
            return false;
        }

        let caster_pos = caster.pos();
        let elapsed = tick.saturating_sub(state.started_at);
        let moved = distance_sq(caster_pos, state.last_pos)
            > WISH_MOVE_THRESHOLD.saturating_mul(WISH_MOVE_THRESHOLD);
        state.last_pos = caster_pos;
        visuals.push((caster_pos, state.target_pos, state.width));

        if moved || elapsed >= state.max_ticks {
            let effective_ticks = elapsed.min(state.max_ticks).max(1);
            resolutions.push((
                state.player_id,
                state.caster_id,
                state.caster_team,
                caster_pos,
                state.target_pos,
                state.ad_damage,
                state.width,
                state.max_ticks.max(1),
                effective_ticks,
                state.full_cooldown_ticks,
                state.attacker_types,
            ));
            return false;
        }

        true
    });
    drop(channels);

    for (caster_pos, target_pos, width) in visuals {
        draw_line_band(ctx, caster_pos, target_pos, width, VFX_FIGHTING);
        draw_status_marker(ctx, caster_pos, 11000, VFX_FIRE);
    }

    for (
        player_id,
        caster_id,
        caster_team,
        caster_pos,
        target_pos,
        ad_damage,
        width,
        max_ticks,
        effective_ticks,
        full_cooldown_ticks,
        attacker_types,
    ) in resolutions
    {
        let charged_percent =
            50usize.saturating_add(effective_ticks.saturating_mul(160) / max_ticks);
        let mut damage = ad_damage
            .saturating_mul(charged_percent)
            .saturating_div(100)
            .max(1);
        damage += damage / 10;

        let hit_target = first_enemy_on_line(ctx, caster_team, caster_pos, target_pos, width);
        if let Some(target_id) = hit_target {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                caster_id,
                target_id,
                damage,
                0,
                AttackType::Skill,
                PokemonType::Fighting,
                attacker_types,
                defender_types,
            );
            set_power_up_punch_ready_at(player_id, tick);
            if let Some(target_pos) = ctx.get_entity(target_id).map(|target| target.pos()) {
                draw_status_marker(ctx, target_pos, 14000, VFX_FIGHTING);
            }
        } else {
            set_power_up_punch_ready_at(player_id, tick.saturating_add(full_cooldown_ticks));
        }
    }
}

fn update_leech_seeds(ctx: &mut GameCtx, tick: usize) {
    let seeds = LEECH_SEEDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut seeds = seeds.lock().expect("leech seed state poisoned");
    let mut leech_ticks = Vec::new();
    let mut spreads = Vec::new();
    let mut visuals = Vec::new();

    seeds.retain_mut(|state| {
        let Some(caster) = ctx.get_entity(state.caster_id) else {
            return false;
        };
        let Some(target) = ctx.get_entity(state.target_id) else {
            return false;
        };
        if !caster.is_alive()
            || !target.is_alive()
            || state.expires_at <= tick
            || distance_sq(caster.pos(), target.pos())
                > state.break_range.saturating_mul(state.break_range)
        {
            return false;
        }

        while state.next_tick_at <= tick {
            leech_ticks.push((state.caster_id, state.target_id, state.damage_per_tick));
            state.next_tick_at += LEECH_TICK_INTERVAL;
        }

        let caster_pos = caster.pos();
        let target_pos = target.pos();
        let caster_team = caster.team();
        visuals.push((caster_pos, target_pos));
        drop(caster);
        drop(target);

        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.team() == caster_team
                || !entity.is_champion()
                || !entity.is_alive()
                || entity.id() == state.target_id
            {
                continue;
            }
            if distance_to_segment_sq(entity.pos(), caster_pos, target_pos)
                <= LEECH_SPREAD_WIDTH.saturating_mul(LEECH_SPREAD_WIDTH)
            {
                spreads.push((
                    state.caster_id,
                    entity.id(),
                    state.expires_at,
                    tick + LEECH_TICK_INTERVAL,
                    state.damage_per_tick,
                    state.break_range,
                ));
            }
        }

        true
    });
    drop(seeds);

    for (caster_pos, target_pos) in visuals {
        draw_line_band(ctx, caster_pos, target_pos, LEECH_SPREAD_WIDTH, VFX_GRASS);
        draw_status_marker(ctx, target_pos, 9000, VFX_GRASS);
    }

    for (caster_id, target_id, damage) in leech_ticks {
        crate::pokemon_status::deal_tracked_damage(
            ctx,
            caster_id,
            target_id,
            0,
            damage,
            AttackType::Dot,
        );
        heal_with_antiheal(ctx, caster_id, caster_id, damage);
    }

    for (caster_id, target_id, expires_at, next_tick_at, damage_per_tick, break_range) in spreads {
        push_leech_seed(
            caster_id,
            target_id,
            expires_at,
            next_tick_at,
            damage_per_tick,
            break_range,
        );
    }
}

#[derive(Clone, Copy)]
struct CoalossalMagmaStormPulse {
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    stage: usize,
    line_damage: usize,
    center_damage: usize,
    attacker_types: TypeSet,
    line_width: u64,
    side_offset: u64,
    center_radius: u64,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
}

fn update_coalossal_magma_storms(ctx: &mut GameCtx, tick: usize) {
    let states = COALOSSAL_MAGMA_STORMS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("coalossal heat crash state poisoned");
    let mut pulses = Vec::new();
    states.retain_mut(|state| {
        let caster_alive = ctx
            .get_entity(state.caster_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        if !caster_alive || state.expires_at <= tick || state.stage > 3 {
            return false;
        }

        while state.next_stage_at <= tick && state.stage <= 3 {
            pulses.push(CoalossalMagmaStormPulse {
                caster_id: state.caster_id,
                caster_team: state.caster_team,
                start: state.start,
                end: state.end,
                stage: state.stage,
                line_damage: state.line_damage,
                center_damage: state.center_damage,
                attacker_types: state.attacker_types,
                line_width: state.line_width,
                side_offset: state.side_offset,
                center_radius: state.center_radius,
                burn_chance_percent: state.burn_chance_percent,
                burn_ticks: state.burn_ticks,
                burn_damage: state.burn_damage,
            });
            state.stage = state.stage.saturating_add(1);
            state.next_stage_at = state
                .next_stage_at
                .saturating_add(state.stage_interval_ticks.max(1));
        }

        state.stage <= 3
    });
    drop(states);

    for pulse in pulses {
        let target_ids = coalossal_magma_storm_targets(ctx, pulse);
        for target_id in target_ids {
            let defender_types =
                crate::neutral_objectives::defender_types_for_target(ctx, target_id);
            let damage = if pulse.stage < 3 {
                pulse.line_damage
            } else {
                pulse.center_damage
            };
            crate::pokemon_types::deal_pokemon_damage(
                ctx,
                pulse.caster_id,
                target_id,
                damage.max(1),
                0,
                AttackType::Skill,
                PokemonType::Fire,
                pulse.attacker_types,
                defender_types,
            );
            if chance_percent(
                ctx.seed(),
                pulse.caster_id,
                target_id,
                tick.saturating_add(pulse.stage),
                pulse.burn_chance_percent,
            ) {
                apply_burn_for(
                    ctx,
                    pulse.caster_id,
                    target_id,
                    pulse.burn_damage.max(1),
                    pulse.burn_ticks,
                );
            }
        }
    }
}

fn update_grapploct_submissions(ctx: &mut GameCtx, tick: usize) {
    let states = GRAPPLOCT_SUBMISSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("grapploct submission state poisoned");
    let mut ticks = Vec::new();
    states.retain_mut(|state| {
        let caster_alive = ctx
            .get_entity(state.caster_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        let target_alive = ctx
            .get_entity(state.target_id)
            .map(|entity| entity.is_alive())
            .unwrap_or(false);
        if !caster_alive || !target_alive || state.expires_at <= tick {
            return false;
        }
        while state.next_tick_at <= tick {
            ticks.push(*state);
            state.next_tick_at = state.next_tick_at.saturating_add(state.tick_interval);
        }
        true
    });
    drop(states);

    for state in ticks {
        let Some(target) = ctx.get_entity(state.target_id) else {
            continue;
        };
        if target.team() == state.caster_team || !target.is_alive() {
            continue;
        }
        let target_hp = target.hp();
        let target_shield = target.shield();
        let target_pos = target.pos();
        drop(target);

        if state.execute_threshold_percent > 0
            && target_hp.current.saturating_mul(100)
                <= target_hp
                    .max
                    .saturating_mul(state.execute_threshold_percent)
        {
            let before_hp = target_hp.current;
            let before_shield = target_shield;
            crate::pokemon_status::deal_tracked_damage(
                ctx,
                state.caster_id,
                state.target_id,
                target_hp.current.max(1),
                0,
                AttackType::Skill,
            );
            let (after_hp, after_shield) = ctx
                .get_entity(state.target_id)
                .map(|entity| (entity.hp().current, entity.shield()))
                .unwrap_or((0, 0));
            crate::crash_probe::log_damage_probe(&format!(
                "event=native_direct_deal label=\"grapploct_execute_pre_tick\" tick={} attacker={} target={} ad={} ap=0 total={} attack_type={:?} before_hp={} after_hp={} hp_lost={} before_shield={} after_shield={} shield_lost={}",
                ctx.tick(),
                state.caster_id,
                state.target_id,
                target_hp.current.max(1),
                target_hp.current.max(1),
                AttackType::Skill,
                before_hp,
                after_hp,
                before_hp.saturating_sub(after_hp),
                before_shield,
                after_shield,
                before_shield.saturating_sub(after_shield),
            ));
            draw_status_marker(ctx, target_pos, 14000, VFX_FIGHTING);
            continue;
        }

        let defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, state.target_id);
        crate::pokemon_types::deal_pokemon_damage(
            ctx,
            state.caster_id,
            state.target_id,
            state.damage_per_tick.max(1),
            0,
            AttackType::Skill,
            PokemonType::Fighting,
            state.attacker_types,
            defender_types,
        );
        if state.execute_threshold_percent > 0 {
            if let Some(target) = ctx.get_entity(state.target_id) {
                let hp = target.hp();
                let before_shield = target.shield();
                drop(target);
                if hp.current > 0
                    && hp.current.saturating_mul(100)
                        <= hp.max.saturating_mul(state.execute_threshold_percent)
                {
                    let before_hp = hp.current;
                    crate::pokemon_status::deal_tracked_damage(
                        ctx,
                        state.caster_id,
                        state.target_id,
                        hp.current.max(1),
                        0,
                        AttackType::Skill,
                    );
                    let (after_hp, after_shield) = ctx
                        .get_entity(state.target_id)
                        .map(|entity| (entity.hp().current, entity.shield()))
                        .unwrap_or((0, 0));
                    crate::crash_probe::log_damage_probe(&format!(
                        "event=native_direct_deal label=\"grapploct_execute_post_tick\" tick={} attacker={} target={} ad={} ap=0 total={} attack_type={:?} before_hp={} after_hp={} hp_lost={} before_shield={} after_shield={} shield_lost={}",
                        ctx.tick(),
                        state.caster_id,
                        state.target_id,
                        hp.current.max(1),
                        hp.current.max(1),
                        AttackType::Skill,
                        before_hp,
                        after_hp,
                        before_hp.saturating_sub(after_hp),
                        before_shield,
                        after_shield,
                        before_shield.saturating_sub(after_shield),
                    ));
                    continue;
                }
            }
        }
        draw_status_marker(ctx, target_pos, 11000, VFX_FIGHTING);
    }
}

fn coalossal_magma_storm_targets(ctx: &mut GameCtx, pulse: CoalossalMagmaStormPulse) -> Vec<usize> {
    let mut targets = Vec::new();
    if pulse.stage < 3 {
        let chunk_start = segment_point(pulse.start, pulse.end, pulse.stage, 3);
        let chunk_end = segment_point(pulse.start, pulse.end, pulse.stage.saturating_add(1), 3);
        let (left_start, left_end, right_start, right_end) =
            parallel_line_segments(chunk_start, chunk_end, pulse.side_offset);
        let width_sq = pulse.line_width.saturating_mul(pulse.line_width);
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.team() == pulse.caster_team
                || !entity.is_alive()
                || entity.is_tower()
                || !entity.is_targetable()
            {
                continue;
            }
            let pos = entity.pos();
            if distance_to_segment_sq(pos, left_start, left_end) <= width_sq
                || distance_to_segment_sq(pos, right_start, right_end) <= width_sq
            {
                targets.push(entity.id());
            }
        }
        draw_line_band(ctx, left_start, left_end, pulse.line_width, VFX_FIRE);
        draw_line_band(ctx, right_start, right_end, pulse.line_width, VFX_FIRE);
    } else {
        let radius_sq = pulse.center_radius.saturating_mul(pulse.center_radius);
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };
            if entity.team() == pulse.caster_team
                || !entity.is_alive()
                || entity.is_tower()
                || !entity.is_targetable()
            {
                continue;
            }
            if distance_sq(entity.pos(), pulse.end) <= radius_sq {
                targets.push(entity.id());
            }
        }
        draw_field_circle(ctx, pulse.end, pulse.center_radius, VFX_FIRE);
    }
    targets
}

fn push_leech_seed(
    caster_id: usize,
    target_id: usize,
    expires_at: usize,
    next_tick_at: usize,
    damage_per_tick: usize,
    break_range: u64,
) {
    let seeds = LEECH_SEEDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut seeds = seeds.lock().expect("leech seed state poisoned");
    if seeds
        .iter()
        .any(|state| state.caster_id == caster_id && state.target_id == target_id)
    {
        return;
    }
    seeds.push(LeechSeedState {
        caster_id,
        target_id,
        expires_at,
        next_tick_at,
        damage_per_tick,
        break_range,
    });
}

pub fn begin_light_screen(
    ctx: &mut GameCtx,
    caster_pos: EntityPos,
    target_pos: EntityPos,
    duration_ticks: usize,
    length: u64,
) {
    let (start, end) = perpendicular_segment(caster_pos, target_pos, length);
    let tick = ctx.tick();
    let mut screens = LIGHT_SCREENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("light screen state poisoned");
    screens.retain(|screen| screen.expires_at > tick);
    screens.push(LightScreenState {
        start,
        end,
        expires_at: tick.saturating_add(duration_ticks),
    });
    drop(screens);
    draw_line_band(ctx, start, end, 5500, VFX_PSYCHIC);
}

pub fn light_screen_blocks(
    ctx: &GameCtx,
    attacker_id: usize,
    target_id: usize,
    move_type: PokemonType,
) -> bool {
    if !matches!(move_type, PokemonType::Fire | PokemonType::Electric) {
        return false;
    }
    let Some(attacker_pos) = ctx.get_entity(attacker_id).map(|entity| entity.pos()) else {
        return false;
    };
    let Some(target_pos) = ctx.get_entity(target_id).map(|entity| entity.pos()) else {
        return false;
    };
    let tick = ctx.tick();
    let mut screens = LIGHT_SCREENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("light screen state poisoned");
    screens.retain(|screen| screen.expires_at > tick);
    screens
        .iter()
        .any(|screen| segments_intersect(attacker_pos, target_pos, screen.start, screen.end))
}

pub fn begin_smoke_screen(
    ctx: &mut GameCtx,
    caster_pos: EntityPos,
    target_pos: EntityPos,
    duration_ticks: usize,
    length: u64,
) {
    let (start, end) = perpendicular_segment(caster_pos, target_pos, length);
    let tick = ctx.tick();
    let mut screens = SMOKE_SCREENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("smoke screen state poisoned");
    screens.retain(|screen| screen.expires_at > tick);
    screens.push(SmokeScreenState {
        start,
        end,
        expires_at: tick.saturating_add(duration_ticks),
    });
    drop(screens);
    draw_line_band(ctx, start, end, 6500, VFX_NORMAL);
}

pub fn smoke_screen_blocks(ctx: &GameCtx, attacker_id: usize, target_id: usize) -> bool {
    let Some(attacker_pos) = ctx.get_entity(attacker_id).map(|entity| entity.pos()) else {
        return false;
    };
    let Some(target_pos) = ctx.get_entity(target_id).map(|entity| entity.pos()) else {
        return false;
    };
    smoke_screen_blocks_line(ctx, attacker_pos, target_pos)
}

pub fn smoke_screen_blocks_line(ctx: &GameCtx, start: EntityPos, end: EntityPos) -> bool {
    let tick = ctx.tick();
    let mut screens = SMOKE_SCREENS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("smoke screen state poisoned");
    screens.retain(|screen| screen.expires_at > tick);
    screens
        .iter()
        .any(|screen| segments_intersect(start, end, screen.start, screen.end))
}

pub fn begin_detect_guard(ctx: &GameCtx, entity_id: usize, duration_ticks: usize) {
    let tick = ctx.tick();
    let states = DETECT_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("detect guard state poisoned");
    states.retain(|state| state.entity_id != entity_id && state.expires_at > tick);
    states.push(DetectGuardState {
        entity_id,
        expires_at: tick.saturating_add(duration_ticks),
    });
}

pub fn consume_detect_guard(ctx: &GameCtx, entity_id: usize) -> bool {
    let tick = ctx.tick();
    let states = DETECT_GUARDS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("detect guard state poisoned");
    let mut consumed = false;
    states.retain(|state| {
        if state.expires_at <= tick {
            return false;
        }
        if state.entity_id == entity_id && !consumed {
            consumed = true;
            return false;
        }
        true
    });
    consumed
}

pub fn has_boxer(_ctx: &GameCtx, entity_id: usize) -> bool {
    champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_grapploct")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_grapploct")
}

pub fn sirfetchd_leek_damage_reduce_percent(ctx: &GameCtx, entity_id: usize) -> usize {
    let has_leek = champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_sirfetchd")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_sirfetchd");
    if !has_leek {
        return 0;
    }
    let crit_chance = ctx
        .get_entity(entity_id)
        .map(|entity| entity.stat().crit_chance)
        .unwrap_or(0)
        .saturating_add(50);
    (crit_chance.saturating_sub(100) / 2).min(45)
}

pub fn has_ice_scales(_ctx: &GameCtx, entity_id: usize) -> bool {
    champion_id_for_entity(entity_id)
        .map(|champion_id| champion_id == "pokemon_moba_frosmoth")
        .unwrap_or(false)
        || receiver_has_copied(entity_id, "pokemon_moba_frosmoth")
}

#[allow(clippy::too_many_arguments)]
pub fn begin_grapploct_submission(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    target_id: usize,
    duration_ticks: usize,
    tick_interval: usize,
    damage_per_tick: usize,
    execute_threshold_percent: usize,
    attacker_types: TypeSet,
) {
    let Some(target) = ctx.get_entity(target_id) else {
        return;
    };
    if target.team() == caster_team || !target.is_alive() || !target.is_targetable() {
        return;
    }
    drop(target);
    let tick = ctx.tick();
    let tick_interval = tick_interval.max(1);
    apply_pokemon_cc(
        ctx,
        caster_id,
        caster_id,
        CCState::Bind {
            tick: duration_ticks as u64,
        },
    );
    apply_pokemon_cc(
        ctx,
        caster_id,
        target_id,
        CCState::Bind {
            tick: duration_ticks as u64,
        },
    );
    let states = GRAPPLOCT_SUBMISSIONS.get_or_init(|| Mutex::new(Vec::new()));
    let mut states = states.lock().expect("grapploct submission state poisoned");
    states.push(GrapploctSubmissionState {
        caster_id,
        caster_team,
        target_id,
        next_tick_at: tick,
        expires_at: tick.saturating_add(duration_ticks),
        tick_interval,
        damage_per_tick,
        execute_threshold_percent,
        attacker_types,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn begin_coalossal_magma_storm(
    ctx: &mut GameCtx,
    caster_id: usize,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    line_damage: usize,
    center_damage: usize,
    attacker_types: TypeSet,
    line_width: u64,
    side_offset: u64,
    center_radius: u64,
    stage_interval_ticks: usize,
    burn_chance_percent: usize,
    burn_ticks: usize,
    burn_damage: usize,
) {
    let tick = ctx.tick();
    let stage_interval_ticks = stage_interval_ticks.max(1);
    let mut states = COALOSSAL_MAGMA_STORMS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .expect("coalossal heat crash state poisoned");
    states.push(CoalossalMagmaStormState {
        caster_id,
        caster_team,
        start,
        end,
        next_stage_at: tick,
        stage: 0,
        expires_at: tick.saturating_add(stage_interval_ticks.saturating_mul(6)),
        line_damage,
        center_damage,
        attacker_types,
        line_width,
        side_offset,
        center_radius,
        stage_interval_ticks,
        burn_chance_percent,
        burn_ticks,
        burn_damage,
    });
    drop(states);

    let (left_start, left_end, right_start, right_end) =
        parallel_line_segments(start, end, side_offset);
    draw_line_band(ctx, left_start, left_end, line_width, VFX_FIRE);
    draw_line_band(ctx, right_start, right_end, line_width, VFX_FIRE);
    draw_field_circle(ctx, end, center_radius, VFX_FIRE);
}

pub fn trigger_steam_engine_if_relevant(
    ctx: &mut GameCtx,
    target_id: usize,
    move_type: PokemonType,
) {
    if !matches!(move_type, PokemonType::Fire | PokemonType::Water) {
        return;
    }
    let has_steam_engine = champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_coalossal")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_coalossal");
    if !has_steam_engine {
        return;
    }
    ctx.add_buff(
        target_id,
        BuffState {
            duration: BuffType::Time { tick: 4 * 60 },
            move_speed_mult: 160,
            ..Default::default()
        },
    );
}

pub fn trigger_flash_fire_if_relevant(
    ctx: &mut GameCtx,
    target_id: usize,
    move_type: PokemonType,
    damage: usize,
) {
    if damage == 0 || !matches!(move_type, PokemonType::Fire) {
        return;
    }
    let has_flash_fire = champion_id_for_entity(target_id)
        .map(|champion_id| champion_id == "pokemon_moba_ceruledge")
        .unwrap_or(false)
        || receiver_has_copied(target_id, "pokemon_moba_ceruledge");
    if !has_flash_fire {
        return;
    }
    ctx.add_buff(
        target_id,
        BuffState {
            duration: BuffType::Time { tick: 6 * 60 },
            attack_mult: 12,
            magic_power_mult: 12,
            ..Default::default()
        },
    );
}

fn draw_status_marker(ctx: &mut GameCtx, pos: EntityPos, radius: u64, color: u32) {
    ctx.debug_draw_circle(pos.x, pos.y, radius, color);
    ctx.debug_draw_circle(pos.x, pos.y, radius.saturating_mul(2) / 3, color);
    ctx.debug_draw_line(
        pos.x.saturating_sub(radius),
        pos.y,
        pos.x.saturating_add(radius),
        pos.y,
        color,
    );
}

fn draw_field_circle(ctx: &mut GameCtx, pos: EntityPos, radius: u64, color: u32) {
    ctx.debug_draw_circle(pos.x, pos.y, radius, color);
    ctx.debug_draw_circle(pos.x, pos.y, radius.saturating_mul(3) / 4, color);
    ctx.debug_draw_circle(pos.x, pos.y, radius / 2, color);
}

fn draw_line_band(ctx: &mut GameCtx, start: EntityPos, end: EntityPos, width: u64, color: u32) {
    ctx.debug_draw_line(start.x, start.y, end.x, end.y, color);
    let dx = end.x as f64 - start.x as f64;
    let dy = end.y as f64 - start.y as f64;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0 {
        return;
    }

    let perp_x = -dy / len;
    let perp_y = dx / len;
    let offset = width as f64;
    let s1 = pos_from_f64(
        start.x as f64 + perp_x * offset,
        start.y as f64 + perp_y * offset,
    );
    let e1 = pos_from_f64(
        end.x as f64 + perp_x * offset,
        end.y as f64 + perp_y * offset,
    );
    let s2 = pos_from_f64(
        start.x as f64 - perp_x * offset,
        start.y as f64 - perp_y * offset,
    );
    let e2 = pos_from_f64(
        end.x as f64 - perp_x * offset,
        end.y as f64 - perp_y * offset,
    );
    ctx.debug_draw_line(s1.x, s1.y, e1.x, e1.y, color);
    ctx.debug_draw_line(s2.x, s2.y, e2.x, e2.y, color);
    ctx.debug_draw_circle(start.x, start.y, width / 2, color);
    ctx.debug_draw_circle(end.x, end.y, width / 2, color);
}

fn pos_from_f64(x: f64, y: f64) -> EntityPos {
    EntityPos {
        x: x.clamp(0.0, u64::MAX as f64) as u64,
        y: y.clamp(0.0, u64::MAX as f64) as u64,
    }
}

fn frosmoth_sleep_path_point(
    center: EntityPos,
    radius: u64,
    index: usize,
    total: usize,
) -> EntityPos {
    let total = total.max(1) as f64;
    let angle = (index as f64 / total) * std::f64::consts::TAU;
    pos_from_f64(
        center.x as f64 + angle.cos() * radius as f64,
        center.y as f64 + angle.sin() * radius as f64,
    )
}

fn perpendicular_segment(from: EntityPos, to: EntityPos, length: u64) -> (EntityPos, EntityPos) {
    let dx = to.x as f64 - from.x as f64;
    let dy = to.y as f64 - from.y as f64;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0 {
        let half = length as f64 / 2.0;
        return (
            pos_from_f64(to.x as f64 - half, to.y as f64),
            pos_from_f64(to.x as f64 + half, to.y as f64),
        );
    }
    let half = length as f64 / 2.0;
    let perp_x = -dy / len;
    let perp_y = dx / len;
    (
        pos_from_f64(to.x as f64 + perp_x * half, to.y as f64 + perp_y * half),
        pos_from_f64(to.x as f64 - perp_x * half, to.y as f64 - perp_y * half),
    )
}

fn parallel_line_segments(
    start: EntityPos,
    end: EntityPos,
    offset: u64,
) -> (EntityPos, EntityPos, EntityPos, EntityPos) {
    let dx = end.x as f64 - start.x as f64;
    let dy = end.y as f64 - start.y as f64;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0 {
        return (start, end, start, end);
    }

    let perp_x = -dy / len;
    let perp_y = dx / len;
    let offset = offset as f64;
    (
        pos_from_f64(
            start.x as f64 + perp_x * offset,
            start.y as f64 + perp_y * offset,
        ),
        pos_from_f64(
            end.x as f64 + perp_x * offset,
            end.y as f64 + perp_y * offset,
        ),
        pos_from_f64(
            start.x as f64 - perp_x * offset,
            start.y as f64 - perp_y * offset,
        ),
        pos_from_f64(
            end.x as f64 - perp_x * offset,
            end.y as f64 - perp_y * offset,
        ),
    )
}

fn segment_point(start: EntityPos, end: EntityPos, index: usize, total: usize) -> EntityPos {
    let total = total.max(1) as f64;
    let t = index as f64 / total;
    pos_from_f64(
        start.x as f64 + (end.x as f64 - start.x as f64) * t,
        start.y as f64 + (end.y as f64 - start.y as f64) * t,
    )
}

fn segments_intersect(a: EntityPos, b: EntityPos, c: EntityPos, d: EntityPos) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);

    if o1 == 0 && on_segment(a, c, b) {
        return true;
    }
    if o2 == 0 && on_segment(a, d, b) {
        return true;
    }
    if o3 == 0 && on_segment(c, a, d) {
        return true;
    }
    if o4 == 0 && on_segment(c, b, d) {
        return true;
    }

    o1 != o2 && o3 != o4
}

fn orientation(a: EntityPos, b: EntityPos, c: EntityPos) -> i32 {
    let ax = a.x as i128;
    let ay = a.y as i128;
    let bx = b.x as i128;
    let by = b.y as i128;
    let cx = c.x as i128;
    let cy = c.y as i128;
    let value = (by - ay) * (cx - bx) - (bx - ax) * (cy - by);
    value.signum() as i32
}

fn on_segment(a: EntityPos, b: EntityPos, c: EntityPos) -> bool {
    b.x >= a.x.min(c.x) && b.x <= a.x.max(c.x) && b.y >= a.y.min(c.y) && b.y <= a.y.max(c.y)
}

fn distance_sq(a: mod_api::EntityPos, b: mod_api::EntityPos) -> u64 {
    let dx = a.x as i128 - b.x as i128;
    let dy = a.y as i128 - b.y as i128;
    squared_len_i128(dx, dy).min(u64::MAX as i128) as u64
}

fn integer_sqrt(value: u64) -> u64 {
    if value <= 1 {
        return value;
    }
    let mut left = 1u64;
    let mut right = value.min(1 << 32);
    let mut result = 1u64;
    while left <= right {
        let mid = left + (right - left) / 2;
        let square = mid.saturating_mul(mid);
        if square <= value {
            result = mid;
            left = mid.saturating_add(1);
        } else {
            right = mid.saturating_sub(1);
        }
    }
    result
}

fn squared_len_i128(dx: i128, dy: i128) -> i128 {
    dx.checked_mul(dx)
        .and_then(|x2| dy.checked_mul(dy).and_then(|y2| x2.checked_add(y2)))
        .unwrap_or(i128::MAX)
}

fn distance_to_segment_sq(
    point: mod_api::EntityPos,
    start: mod_api::EntityPos,
    end: mod_api::EntityPos,
) -> u64 {
    let px = point.x as i128;
    let py = point.y as i128;
    let ax = start.x as i128;
    let ay = start.y as i128;
    let bx = end.x as i128;
    let by = end.y as i128;
    let abx = bx - ax;
    let aby = by - ay;
    let apx = px - ax;
    let apy = py - ay;
    let ab_len_sq = squared_len_i128(abx, aby);

    if ab_len_sq <= 0 {
        return distance_sq(point, start);
    }

    let t_num = (apx * abx + apy * aby).clamp(0, ab_len_sq);
    let closest_x_num = ax * ab_len_sq + abx * t_num;
    let closest_y_num = ay * ab_len_sq + aby * t_num;
    let dx_num = px * ab_len_sq - closest_x_num;
    let dy_num = py * ab_len_sq - closest_y_num;
    let Some(denominator) = ab_len_sq.checked_mul(ab_len_sq) else {
        return distance_sq(point, start);
    };
    if denominator <= 0 {
        return distance_sq(point, start);
    }

    let dist_sq = squared_len_i128(dx_num, dy_num) / denominator;
    dist_sq.min(u64::MAX as i128) as u64
}

fn first_enemy_on_line(
    ctx: &GameCtx,
    caster_team: usize,
    start: EntityPos,
    end: EntityPos,
    width: u64,
) -> Option<usize> {
    let ax = start.x as i128;
    let ay = start.y as i128;
    let bx = end.x as i128;
    let by = end.y as i128;
    let abx = bx - ax;
    let aby = by - ay;
    let ab_len_sq = abx * abx + aby * aby;
    if ab_len_sq == 0 {
        return None;
    }

    let mut best: Option<(i128, usize)> = None;
    for index in 0..ctx.entity_count() {
        let Some(entity) = ctx.entity_at(index) else {
            continue;
        };
        if entity.team() == caster_team || !entity.is_alive() || !entity.is_targetable() {
            continue;
        }

        let pos = entity.pos();
        if distance_to_segment_sq(pos, start, end) > width.saturating_mul(width) {
            continue;
        }

        let px = pos.x as i128;
        let py = pos.y as i128;
        let projection = (px - ax) * abx + (py - ay) * aby;
        if projection < 0 || projection > ab_len_sq {
            continue;
        }

        if best
            .map(|(best_projection, _)| projection < best_projection)
            .unwrap_or(true)
        {
            best = Some((projection, entity.id()));
        }
    }

    best.map(|(_, entity_id)| entity_id)
}

fn is_status_immune(target_id: usize, immune_types: &[PokemonType]) -> bool {
    entity_types(target_id)
        .map(|types| {
            types
                .iter()
                .any(|pokemon_type| immune_types.iter().any(|immune| *immune == pokemon_type))
        })
        .unwrap_or(false)
}

fn should_proc(rng_seed: u64, entity_id: usize, tick: usize) -> bool {
    splitmix64(rng_seed ^ ((entity_id as u64) << 32) ^ tick as u64) % PARALYSIS_CHANCE_DEN == 0
}

fn chance_percent(
    rng_seed: u64,
    caster_id: usize,
    target_id: usize,
    tick: usize,
    percent: usize,
) -> bool {
    if percent >= 100 {
        return true;
    }
    let roll = splitmix64(
        rng_seed
            ^ ((caster_id as u64) << 40)
            ^ ((target_id as u64) << 16)
            ^ tick as u64
            ^ 0xf20d_u64,
    ) % 100;
    roll < percent as u64
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^ (value >> 31)
}
