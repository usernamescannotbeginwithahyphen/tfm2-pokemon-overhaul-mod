#![allow(dead_code)]

use mod_api::{AttackType, GameCtx};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    Bird,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeSet {
    pub primary: PokemonType,
    pub secondary: Option<PokemonType>,
}

impl TypeSet {
    pub const fn single(primary: PokemonType) -> Self {
        Self {
            primary,
            secondary: None,
        }
    }

    pub const fn dual(primary: PokemonType, secondary: PokemonType) -> Self {
        Self {
            primary,
            secondary: Some(secondary),
        }
    }

    pub fn iter(self) -> impl Iterator<Item = PokemonType> {
        [Some(self.primary), self.secondary].into_iter().flatten()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Matchup {
    SuperEffective,
    NotVeryEffective,
    Neutral,
}

const SUPER_NUM: usize = 6;
const SUPER_DEN: usize = 5;
const RESIST_NUM: usize = 4;
const RESIST_DEN: usize = 5;
const STAB_NUM: usize = 11;
const STAB_DEN: usize = 10;

pub fn type_modifier_ratio(move_type: PokemonType, defender_types: TypeSet) -> (usize, usize) {
    let mut num = 1;
    let mut den = 1;

    for defender_type in defender_types.iter() {
        match matchup(move_type, defender_type) {
            Matchup::SuperEffective => {
                num *= SUPER_NUM;
                den *= SUPER_DEN;
            }
            Matchup::NotVeryEffective => {
                num *= RESIST_NUM;
                den *= RESIST_DEN;
            }
            Matchup::Neutral => {}
        }
    }

    reduce_ratio(num, den)
}

pub fn stab_modifier_ratio(move_type: PokemonType, attacker_types: TypeSet) -> (usize, usize) {
    if attacker_types
        .iter()
        .any(|attacker_type| attacker_type == move_type)
    {
        (STAB_NUM, STAB_DEN)
    } else {
        (1, 1)
    }
}

pub fn total_modifier_ratio(
    move_type: PokemonType,
    attacker_types: TypeSet,
    defender_types: TypeSet,
) -> (usize, usize) {
    let (type_num, type_den) = type_modifier_ratio(move_type, defender_types);
    let (stab_num, stab_den) = stab_modifier_ratio(move_type, attacker_types);
    reduce_ratio(type_num * stab_num, type_den * stab_den)
}

pub fn apply_damage_modifier(
    damage: usize,
    move_type: PokemonType,
    attacker_types: TypeSet,
    defender_types: TypeSet,
) -> usize {
    if damage == 0 {
        return 0;
    }
    let (num, den) = total_modifier_ratio(move_type, attacker_types, defender_types);
    ((damage.saturating_mul(num) + (den / 2)) / den).max(1)
}

#[allow(dead_code)]
pub fn deal_pokemon_damage(
    ctx: &mut GameCtx,
    attacker: usize,
    target: usize,
    ad_damage: usize,
    ap_damage: usize,
    attack_type: AttackType,
    move_type: PokemonType,
    attacker_types: TypeSet,
    defender_types: TypeSet,
) -> crate::pokemon_status::PokemonDamageResult {
    let (target, defender_types) =
        if let Some(audino_id) = crate::pokemon_status::audino_protect_redirect(ctx, target) {
            (
                audino_id,
                crate::neutral_objectives::defender_types_for_target(ctx, audino_id),
            )
        } else {
            (target, defender_types)
        };

    if crate::pokemon_status::telepathy_blocks_ally_harm(ctx, attacker, target) {
        return crate::pokemon_status::PokemonDamageResult::default();
    }

    if crate::pokemon_status::consume_detect_guard(ctx, target) {
        return crate::pokemon_status::PokemonDamageResult::default();
    }
    if crate::pokemon_status::try_consume_zeraora_zing_zap(ctx, target, attacker, attack_type) {
        return crate::pokemon_status::PokemonDamageResult::default();
    }
    if crate::pokemon_status::is_bouffalant_unstoppable(ctx, target) {
        return crate::pokemon_status::PokemonDamageResult::default();
    }

    if let Some(retaliation_damage) =
        crate::pokemon_status::try_consume_reversal(ctx, target, attacker)
    {
        let retaliation_attacker_types =
            crate::neutral_objectives::defender_types_for_target(ctx, target);
        let retaliation_defender_types =
            crate::neutral_objectives::defender_types_for_target(ctx, attacker);
        deal_pokemon_damage(
            ctx,
            target,
            attacker,
            retaliation_damage,
            0,
            AttackType::Skill,
            PokemonType::Fighting,
            retaliation_attacker_types,
            retaliation_defender_types,
        );
        return crate::pokemon_status::PokemonDamageResult::default();
    }

    if (ad_damage > 0 || ap_damage > 0)
        && crate::pokemon_status::try_wonder_guard_damage(
            ctx,
            attacker,
            target,
            attack_type,
            move_type,
            defender_types,
        )
    {
        return crate::pokemon_status::PokemonDamageResult::default();
    }

    let modified_ad_damage =
        apply_damage_modifier(ad_damage, move_type, attacker_types, defender_types);
    let modified_ap_damage =
        apply_damage_modifier(ap_damage, move_type, attacker_types, defender_types);
    let bypass_resistance = crate::pokemon_status::is_blood_moon_active(ctx, attacker)
        || crate::pokemon_status::has_scrappy(ctx, attacker)
        || crate::pokemon_status::thievul_stakeout_bypasses_resistance(ctx, attacker, target);
    let (ad_damage, ap_damage) = if bypass_resistance {
        (
            modified_ad_damage.max(ad_damage),
            modified_ap_damage.max(ap_damage),
        )
    } else {
        (modified_ad_damage, modified_ap_damage)
    };
    let aqua_step_bonus = if matches!(move_type, PokemonType::Water) {
        crate::pokemon_status::quaquaval_water_damage_bonus_percent(ctx, attacker)
    } else {
        0
    };
    let (ad_damage, ap_damage) = if aqua_step_bonus > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_add(aqua_step_bonus)) / 100,
            ap_damage.saturating_mul(100usize.saturating_add(aqua_step_bonus)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let reduce_percent =
        crate::pokemon_status::gallade_prediction_reduce_percent(ctx, target, attacker);
    let (ad_damage, ap_damage) = if reduce_percent > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
            ap_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let reduce_percent = crate::pokemon_status::wide_guard_reduce_percent(ctx, target, attacker);
    let (ad_damage, ap_damage) = if reduce_percent > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
            ap_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let reduce_percent = crate::pokemon_status::afro_reduce_percent(ctx, target, attacker);
    let (ad_damage, ap_damage) = if reduce_percent > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
            ap_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let reduce_percent = crate::pokemon_status::bouffalant_retaliate_reduce_percent(ctx, target);
    let (ad_damage, ap_damage) = if reduce_percent > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
            ap_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let (ad_damage, ap_damage) = if matches!(move_type, PokemonType::Fighting)
        && crate::pokemon_status::has_boxer(ctx, target)
    {
        (
            ad_damage.saturating_mul(75) / 100,
            ap_damage.saturating_mul(75) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let reduce_percent = crate::pokemon_status::sirfetchd_leek_damage_reduce_percent(ctx, target);
    let (ad_damage, ap_damage) = if reduce_percent > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
            ap_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let reduce_percent =
        crate::pokemon_status::try_trigger_hawlucha_counter(ctx, target, attacker, attack_type);
    let (ad_damage, ap_damage) = if reduce_percent > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
            ap_damage.saturating_mul(100usize.saturating_sub(reduce_percent)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    let ap_damage = if crate::pokemon_status::has_ice_scales(ctx, target) {
        ap_damage / 2
    } else {
        ap_damage
    };
    let ad_damage =
        ad_damage.saturating_add(crate::pokemon_status::bouffalant_head_charge_bonus_damage(
            ctx,
            attacker,
            target,
            attack_type,
            move_type,
        ));
    let (ad_damage, ap_damage) =
        crate::pokemon_status::adjust_endure_damage(ctx, target, ad_damage, ap_damage);
    let (ad_damage, ap_damage) =
        crate::pokemon_status::adjust_sturdy_damage(ctx, target, attack_type, ad_damage, ap_damage);
    let pain_amp_bonus = crate::pokemon_status::xatu_pain_amplifier_bonus_percent(ctx, target);
    let (ad_damage, ap_damage) = if pain_amp_bonus > 0 {
        (
            ad_damage.saturating_mul(100usize.saturating_add(pain_amp_bonus)) / 100,
            ap_damage.saturating_mul(100usize.saturating_add(pain_amp_bonus)) / 100,
        )
    } else {
        (ad_damage, ap_damage)
    };
    if ad_damage > 0 || ap_damage > 0 {
        crate::pokemon_status::note_light_metal_dealt_damage(ctx, attacker);
        crate::pokemon_status::note_direct_pokemon_damage(
            ctx,
            attacker,
            target,
            ad_damage.saturating_add(ap_damage),
            attack_type,
        );
        crate::pokemon_status::note_starmie_damage_hit(ctx, attacker, target);
        crate::pokemon_status::trigger_justified_if_dark_damage(
            ctx,
            target,
            move_type,
            ad_damage.saturating_add(ap_damage),
        );
        crate::pokemon_status::trigger_steam_engine_if_relevant(ctx, target, move_type);
        crate::pokemon_status::trigger_flash_fire_if_relevant(
            ctx,
            target,
            move_type,
            ad_damage.saturating_add(ap_damage),
        );
    }
    let attacker_player = crate::pokemon_status::player_for_entity(attacker);
    let target_player = crate::pokemon_status::player_for_entity(target);
    let attacker_before_stats = player_probe_stats(ctx, attacker_player);
    let target_before_stats = player_probe_stats(ctx, target_player);
    let kill_log_before = ctx.kill_log_count();
    let damage_result = crate::pokemon_status::deal_tracked_damage(
        ctx,
        attacker,
        target,
        ad_damage,
        ap_damage,
        attack_type,
    );
    if damage_result.applied_damage > 0 && ad_damage > 0 {
        crate::pokemon_status::trigger_garganacl_salt_cure_from_physical_damage(
            ctx, target, ad_damage,
        );
    }
    let before = damage_result.before;
    let after = damage_result.after;
    log_pokemon_stat_probe(
        ctx,
        attacker,
        target,
        ad_damage,
        ap_damage,
        attack_type,
        move_type,
        before,
        after,
        damage_result.applied_damage,
        attacker_player,
        target_player,
        attacker_before_stats,
        target_before_stats,
        kill_log_before,
    );
    log_pokemon_damage_probe(
        ctx,
        attacker,
        target,
        ad_damage,
        ap_damage,
        attack_type,
        move_type,
        before,
        after,
    );

    damage_result
}

fn damage_probe_snapshot(ctx: &GameCtx, entity_id: usize) -> Option<(usize, usize)> {
    ctx.get_entity(entity_id)
        .map(|entity| (entity.hp().current, entity.shield()))
}

#[allow(clippy::too_many_arguments)]
fn log_pokemon_damage_probe(
    ctx: &GameCtx,
    attacker: usize,
    target: usize,
    ad_damage: usize,
    ap_damage: usize,
    attack_type: AttackType,
    move_type: PokemonType,
    before: Option<(usize, usize)>,
    after: Option<(usize, usize)>,
) {
    let (before_hp, before_shield) = before.unwrap_or((0, 0));
    let (after_hp, after_shield) = after.unwrap_or((0, 0));
    let hp_lost = before_hp.saturating_sub(after_hp);
    let shield_lost = before_shield.saturating_sub(after_shield);
    let attacker_player = crate::pokemon_status::player_for_entity(attacker).unwrap_or(attacker);
    let target_player = crate::pokemon_status::player_for_entity(target).unwrap_or(target);
    let attacker_champion =
        crate::pokemon_status::champion_id_for_entity(attacker).unwrap_or("unknown");
    let target_champion =
        crate::pokemon_status::champion_id_for_entity(target).unwrap_or("unknown");
    crate::crash_probe::log_damage_probe(&format!(
        "event=pokemon_resolver_deal tick={} attacker={} attacker_player={} attacker_champion=\"{}\" target={} target_player={} target_champion=\"{}\" ad={} ap={} total={} attack_type={:?} move_type={:?} before_hp={} after_hp={} hp_lost={} before_shield={} after_shield={} shield_lost={}",
        ctx.tick(),
        attacker,
        attacker_player,
        crate::crash_probe::sanitize_log_field(attacker_champion),
        target,
        target_player,
        crate::crash_probe::sanitize_log_field(target_champion),
        ad_damage,
        ap_damage,
        ad_damage.saturating_add(ap_damage),
        attack_type,
        move_type,
        before_hp,
        after_hp,
        hp_lost,
        before_shield,
        after_shield,
        shield_lost,
    ));
}

#[allow(clippy::too_many_arguments)]
fn log_pokemon_stat_probe(
    ctx: &GameCtx,
    attacker: usize,
    target: usize,
    ad_damage: usize,
    ap_damage: usize,
    attack_type: AttackType,
    move_type: PokemonType,
    before: Option<(usize, usize)>,
    after: Option<(usize, usize)>,
    applied: usize,
    attacker_player: Option<usize>,
    target_player: Option<usize>,
    attacker_before_stats: PlayerProbeStats,
    target_before_stats: PlayerProbeStats,
    kill_log_before: usize,
) {
    let (before_hp, before_shield) = before.unwrap_or((0, 0));
    let (after_hp, after_shield) = after.unwrap_or((0, 0));
    let hp_lost = before_hp.saturating_sub(after_hp);
    let shield_lost = before_shield.saturating_sub(after_shield);
    let attacker_after_stats = player_probe_stats(ctx, attacker_player);
    let target_after_stats = player_probe_stats(ctx, target_player);
    let kill_log_after = ctx.kill_log_count();
    let attacker_champion =
        crate::pokemon_status::champion_id_for_entity(attacker).unwrap_or("unknown");
    let target_champion =
        crate::pokemon_status::champion_id_for_entity(target).unwrap_or("unknown");
    let latest_kill_log = if kill_log_after > kill_log_before && kill_log_after > 0 {
        let entry = ctx.kill_log_at(kill_log_after - 1);
        format!(
            "latest_kill_tick={} latest_killer_team={} latest_killer_position={} latest_killed_position={} latest_assist_count={} latest_assist_positions=\"{}\"",
            entry.tick,
            entry.killer_team,
            entry.killer_position,
            entry.killed_position,
            entry.assist_count,
            entry
                .assist_positions
                .iter()
                .take(entry.assist_count as usize)
                .map(|position| position.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    } else {
        "latest_kill_tick=none latest_killer_team=none latest_killer_position=none latest_killed_position=none latest_assist_count=0 latest_assist_positions=\"\"".to_string()
    };

    crate::crash_probe::log_stat_probe(&format!(
        "event=resolver_damage tick={} attacker={} attacker_player={} attacker_champion=\"{}\" target={} target_player={} target_champion=\"{}\" request_ad={} request_ap={} request_total={} applied={} hp_lost={} shield_lost={} attack_type={:?} move_type={:?} attacker_before={} attacker_after={} target_before={} target_after={} kill_log_before={} kill_log_after={} {}",
        ctx.tick(),
        attacker,
        optional_usize(attacker_player),
        crate::crash_probe::sanitize_log_field(attacker_champion),
        target,
        optional_usize(target_player),
        crate::crash_probe::sanitize_log_field(target_champion),
        ad_damage,
        ap_damage,
        ad_damage.saturating_add(ap_damage),
        applied,
        hp_lost,
        shield_lost,
        attack_type,
        move_type,
        attacker_before_stats.log_value(),
        attacker_after_stats.log_value(),
        target_before_stats.log_value(),
        target_after_stats.log_value(),
        kill_log_before,
        kill_log_after,
        latest_kill_log,
    ));
}

fn player_probe_stats(ctx: &GameCtx, player_id: Option<usize>) -> PlayerProbeStats {
    let Some(player_id) = player_id else {
        return PlayerProbeStats::missing();
    };
    let Some(player) = ctx.get_player(player_id) else {
        return PlayerProbeStats::missing_with_id(player_id);
    };

    PlayerProbeStats {
        player_id,
        found: true,
        team: player.team(),
        position: player.position() as usize,
        is_alive: player.is_alive(),
        kills: player.kills(),
        deaths: player.deaths(),
        assists: player.assists(),
        cs: player.cs(),
        gold: player.gold(),
    }
}

fn optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "none".to_string())
}

#[derive(Clone, Copy)]
struct PlayerProbeStats {
    player_id: usize,
    found: bool,
    team: usize,
    position: usize,
    is_alive: bool,
    kills: usize,
    deaths: usize,
    assists: usize,
    cs: usize,
    gold: usize,
}

impl PlayerProbeStats {
    fn missing() -> Self {
        Self {
            player_id: usize::MAX,
            found: false,
            team: usize::MAX,
            position: usize::MAX,
            is_alive: false,
            kills: 0,
            deaths: 0,
            assists: 0,
            cs: 0,
            gold: 0,
        }
    }

    fn missing_with_id(player_id: usize) -> Self {
        Self {
            player_id,
            ..Self::missing()
        }
    }

    fn log_value(self) -> String {
        format!(
            "{{player_id:{} found:{} team:{} position:{} alive:{} kills:{} deaths:{} assists:{} cs:{} gold:{}}}",
            self.player_id,
            self.found,
            self.team,
            self.position,
            self.is_alive,
            self.kills,
            self.deaths,
            self.assists,
            self.cs,
            self.gold,
        )
    }
}

fn reduce_ratio(num: usize, den: usize) -> (usize, usize) {
    let divisor = gcd(num, den);
    (num / divisor, den / divisor)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a.max(1)
}

fn matchup(move_type: PokemonType, defender_type: PokemonType) -> Matchup {
    use Matchup::{Neutral, NotVeryEffective as Resist, SuperEffective as Super};
    use PokemonType::*;

    match (move_type, defender_type) {
        (Normal, Rock | Steel) => Resist,
        (Normal, Ghost) => Resist,

        (Fire, Grass | Ice | Bug | Steel) => Super,
        (Fire, Fire | Water | Rock | Dragon) => Resist,

        (Water, Fire | Ground | Rock) => Super,
        (Water, Water | Grass | Dragon) => Resist,

        (Electric, Water | Flying) => Super,
        (Electric, Electric | Grass | Dragon) => Resist,
        (Electric, Ground) => Resist,

        (Grass, Water | Ground | Rock) => Super,
        (Grass, Fire | Grass | Poison | Flying | Bug | Dragon | Steel) => Resist,

        (Ice, Grass | Ground | Flying | Dragon) => Super,
        (Ice, Fire | Water | Ice | Steel) => Resist,

        (Fighting, Normal | Ice | Rock | Dark | Steel) => Super,
        (Fighting, Poison | Flying | Psychic | Bug | Fairy) => Resist,
        (Fighting, Ghost) => Resist,

        (Poison, Grass | Fairy) => Super,
        (Poison, Poison | Ground | Rock | Ghost) => Resist,
        (Poison, Steel) => Resist,

        (Ground, Fire | Electric | Poison | Rock | Steel) => Super,
        (Ground, Grass | Bug) => Resist,
        (Ground, Flying) => Resist,

        (Flying, Grass | Fighting | Bug) => Super,
        (Flying, Electric | Rock | Steel) => Resist,

        (Psychic, Fighting | Poison) => Super,
        (Psychic, Psychic | Steel) => Resist,
        (Psychic, Dark) => Resist,

        (Bug, Grass | Psychic | Dark) => Super,
        (Bug, Fire | Fighting | Poison | Flying | Ghost | Steel | Fairy) => Resist,

        (Rock, Fire | Ice | Flying | Bug) => Super,
        (Rock, Fighting | Ground | Steel) => Resist,

        (Ghost, Psychic | Ghost) => Super,
        (Ghost, Dark) => Resist,
        (Ghost, Normal) => Resist,

        (Dragon, Dragon) => Super,
        (Dragon, Steel) => Resist,
        (Dragon, Fairy) => Resist,

        (Dark, Psychic | Ghost) => Super,
        (Dark, Fighting | Dark | Fairy) => Resist,

        (Steel, Ice | Rock | Fairy) => Super,
        (Steel, Fire | Water | Electric | Steel) => Resist,

        (Fairy, Fighting | Dragon | Dark) => Super,
        (Fairy, Fire | Poison | Steel) => Resist,

        _ => Neutral,
    }
}
