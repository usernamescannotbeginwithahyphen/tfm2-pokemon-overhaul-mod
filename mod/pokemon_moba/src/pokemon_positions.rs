use crate::pokemon_content::{
    champion_for_mod_index, PokemonChampion, PokemonMove, PokemonMoveEffect,
};
use crate::pokemon_strategy::{
    has_trait, shared_trait_count, strategy_for_champion_id, StrategyTrait,
};
use crate::pokemon_types::total_modifier_ratio;
use mod_api::*;

const VANILLA_CHAMPION_COUNT: usize = 60;

const TOP: usize = Position::Top as usize;
const JUNGLE: usize = Position::Jungle as usize;
const MID: usize = Position::Mid as usize;
const BOTTOM: usize = Position::Bottom as usize;
const SUPPORT: usize = Position::Support as usize;
const POSITION_COUNT: usize = 5;
const PICK_SLOT_POSITIONS: [usize; POSITION_COUNT] = [TOP, JUNGLE, MID, BOTTOM, SUPPORT];
const DIRECT_LANE_MATCHUP_SCALE: f32 = 0.68;
const TEAM_MATCHUP_SCALE: f32 = 0.34;
const BLIND_COUNTER_RISK_SCALE: f32 = 0.28;
const ALLY_COMP_SCALE: f32 = 0.32;
const SLOT_PRIMARY_FIT_SCORE: f32 = 1_000_000.0;
const SLOT_SECONDARY_FIT_SCORE: f32 = 900_000.0;
const SLOT_MISMATCH_SCORE: f32 = -1_000_000.0;
const SLOT_OPEN_PRIMARY_SCORE: f32 = 120_000.0;
const SLOT_OPEN_SECONDARY_SCORE: f32 = 90_000.0;
const SLOT_DUPLICATE_PRIMARY_SCORE: f32 = -45_000.0;
const SLOT_DUPLICATE_COVERAGE_SCORE: f32 = -30_000.0;
const MISSING_SLOT_COVER_SCORE: f32 = 80_000.0;
const MISSING_SLOT_MISS_SCORE: f32 = -40_000.0;
const INCOMPATIBLE_ATTACH_PAIR_SCORE: f32 = -10_000.0;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct PokemonPositionInfo {
    pub id: &'static str,
    pub positions: &'static [usize],
    pub label: &'static str,
}

pub const POKEMON_POSITIONS: [PokemonPositionInfo; 101] = [
    PokemonPositionInfo {
        id: "pokemon_moba_pikachu",
        positions: &[MID, JUNGLE],
        label: "Mid / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_charizard",
        positions: &[MID, TOP],
        label: "Mid / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_blastoise",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_feraligatr",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_emboar",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_blaziken",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_greninja",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_decidueye",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_inteleon",
        positions: &[BOTTOM, MID],
        label: "Bottom / Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_skeledirge",
        positions: &[TOP, MID, SUPPORT],
        label: "Top / Mid / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_porygonz",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_blissey",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_kleavor",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_scizor",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_ursaluna",
        positions: &[TOP, SUPPORT],
        label: "Top / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_sawk_throh",
        positions: &[JUNGLE, TOP],
        label: "Jungle / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_hitmonchan",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_hitmonlee",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_hitmontop",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_kilowattrel",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_beeheeyem",
        positions: &[BOTTOM, MID],
        label: "Bottom / Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_gyarados",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_noivern",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_mantine",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_cryogonal",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_vanilluxe",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_skarmory",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_houndoom",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_arbok",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_clawitzer",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_octillery",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_pyukumuku",
        positions: &[TOP, SUPPORT],
        label: "Top / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_banette",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_kricketune",
        positions: &[SUPPORT, JUNGLE],
        label: "Support / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_ambipom",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_gallade",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_audino",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_pangoro",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_passimian",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_oranguru",
        positions: &[SUPPORT, MID],
        label: "Support / Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_dragalge",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_heliolisk",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_turtonator",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_ribombee",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_drampa",
        positions: &[MID, TOP],
        label: "Mid / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_kommoo",
        positions: &[JUNGLE, TOP],
        label: "Jungle / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_thievul",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_archaludon",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_appletun",
        positions: &[TOP, SUPPORT],
        label: "Top / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_goodra",
        positions: &[SUPPORT, TOP],
        label: "Support / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_dedenne",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_hawlucha",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_bouffalant",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_starmie",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_drednaw",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_orbeetle",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_coalossal",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_magmortar",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_grapploct",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_sirfetchd",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_arboliva",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_armarouge",
        positions: &[TOP, JUNGLE],
        label: "Top / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_ceruledge",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_gholdengo",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_frosmoth",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_shedinja",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_ludicolo",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_kingdra",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_delibird",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_cloyster",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_electrode",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_snorlax",
        positions: &[TOP, SUPPORT],
        label: "Top / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_zeraora",
        positions: &[MID, JUNGLE],
        label: "Mid / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_rillaboom",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_dragapult",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_shiftry",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_sigilyph",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_weavile",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_swanna",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_marowak",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_garganacl",
        positions: &[TOP],
        label: "Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_ampharos",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_xatu",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_quaquaval",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_arcanine",
        positions: &[MID, TOP],
        label: "Mid / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_missingno",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_yanmega",
        positions: &[TOP, MID],
        label: "Top / Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_wishiwashi",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_comfey",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_smeargle",
        positions: &[MID, SUPPORT],
        label: "Mid / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_torterra",
        positions: &[TOP, SUPPORT],
        label: "Top / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_venusaur",
        positions: &[SUPPORT, TOP],
        label: "Support / Top",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_eevee",
        positions: &[SUPPORT],
        label: "Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_jolteon",
        positions: &[BOTTOM, JUNGLE],
        label: "Bottom / Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_flareon",
        positions: &[MID],
        label: "Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_vaporeon",
        positions: &[TOP, SUPPORT],
        label: "Top / Support",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_leafeon",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_glaceon",
        positions: &[BOTTOM],
        label: "Bottom",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_umbreon",
        positions: &[JUNGLE],
        label: "Jungle",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_espeon",
        positions: &[SUPPORT, MID],
        label: "Support / Mid",
    },
    PokemonPositionInfo {
        id: "pokemon_moba_sylveon",
        positions: &[SUPPORT],
        label: "Support",
    },
];

#[allow(dead_code)]
pub fn lane_label(champion_id: &str) -> Option<&'static str> {
    POKEMON_POSITIONS
        .iter()
        .find(|info| info.id == champion_id)
        .map(|info| info.label)
}

pub fn info_for_champion_id(champion_id: &str) -> Option<&'static PokemonPositionInfo> {
    POKEMON_POSITIONS.iter().find(|info| info.id == champion_id)
}

pub fn position_icon_source(position: usize) -> Option<&'static str> {
    match position {
        TOP => Some("asset/base/ui/icons/top"),
        JUNGLE => Some("asset/base/ui/icons/jungle"),
        MID => Some("asset/base/ui/icons/mid"),
        BOTTOM => Some("asset/base/ui/icons/bottom"),
        SUPPORT => Some("asset/base/ui/icons/support"),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn position_name(position: usize) -> Option<&'static str> {
    match position {
        TOP => Some("Top"),
        JUNGLE => Some("Jungle"),
        MID => Some("Mid"),
        BOTTOM => Some("Bottom"),
        SUPPORT => Some("Support"),
        _ => None,
    }
}

pub fn info_for_candidate(candidate: usize) -> Option<&'static PokemonPositionInfo> {
    candidate
        .checked_sub(VANILLA_CHAMPION_COUNT)
        .and_then(|index| POKEMON_POSITIONS.get(index))
}

fn champion_for_candidate(candidate: usize) -> Option<PokemonChampion> {
    candidate
        .checked_sub(VANILLA_CHAMPION_COUNT)
        .and_then(champion_for_mod_index)
}

fn slot_position(slot: usize) -> Option<usize> {
    PICK_SLOT_POSITIONS.get(slot).copied()
}

fn position_counts(picks: &[usize]) -> [usize; POSITION_COUNT] {
    let mut counts = [0; POSITION_COUNT];
    for pick in picks {
        if let Some(info) = info_for_candidate(*pick) {
            if let Some(primary) = info.positions.first() {
                if *primary < POSITION_COUNT {
                    counts[*primary] += 1;
                }
            }
        }
    }
    counts
}

fn coverage_counts(picks: &[usize]) -> [usize; POSITION_COUNT] {
    let mut counts = [0; POSITION_COUNT];
    for pick in picks {
        if let Some(info) = info_for_candidate(*pick) {
            for position in info.positions {
                if *position < POSITION_COUNT {
                    counts[*position] += 1;
                }
            }
        }
    }
    counts
}

fn available_slot_fit(ctx: &DraftScoreContext, slot_position: usize) -> (bool, bool) {
    let mut has_primary = false;
    let mut has_coverage = false;

    for candidate in ctx.available_champions {
        let Some(info) = info_for_candidate(*candidate) else {
            continue;
        };
        if info.positions.first().copied() == Some(slot_position) {
            has_primary = true;
            has_coverage = true;
        } else if info.positions.contains(&slot_position) {
            has_coverage = true;
        }
    }

    (has_primary, has_coverage)
}

fn strategic_pick_score(
    ctx: &DraftScoreContext,
    candidate: usize,
    info: &PokemonPositionInfo,
) -> f32 {
    let Some(candidate_champion) = champion_for_candidate(candidate) else {
        return 0.0;
    };
    let candidate_lane = slot_position(ctx.ally_pick.len());
    let mut score = 0.0;
    let mut has_lane_opponent = false;

    for (enemy_slot, enemy_pick) in ctx.enemy_pick.iter().enumerate() {
        let Some(enemy_champion) = champion_for_candidate(*enemy_pick) else {
            continue;
        };
        let matchup = matchup_score(candidate_champion, enemy_champion);
        if candidate_lane.is_some() && slot_position(enemy_slot) == candidate_lane {
            has_lane_opponent = true;
            score += matchup * DIRECT_LANE_MATCHUP_SCALE;
        } else if matchup > 0.0 {
            score += matchup * TEAM_MATCHUP_SCALE;
        }
    }

    if !has_lane_opponent {
        if let Some(candidate_lane) = candidate_lane {
            score += blind_counter_risk_score(ctx, candidate, candidate_champion, candidate_lane)
                * BLIND_COUNTER_RISK_SCALE;
        }
    }

    if info.positions.len() > 1 && score >= 0.0 {
        score += 18.0;
    }

    score += eeveelution_race_score(ctx, candidate_champion);
    score += ally_composition_score(ctx, candidate_champion) * ALLY_COMP_SCALE;

    score
}

fn blind_counter_risk_score(
    ctx: &DraftScoreContext,
    candidate: usize,
    candidate_champion: PokemonChampion,
    candidate_lane: usize,
) -> f32 {
    let mut worst_counter = 0.0_f32;
    let mut counter_total = 0.0;
    let mut counter_count = 0;

    for enemy_candidate in ctx.available_champions {
        if *enemy_candidate == candidate {
            continue;
        }
        let Some(enemy_info) = info_for_candidate(*enemy_candidate) else {
            continue;
        };
        if !enemy_info.positions.contains(&candidate_lane) {
            continue;
        }
        let Some(enemy_champion) = champion_for_candidate(*enemy_candidate) else {
            continue;
        };
        let threat = matchup_score(enemy_champion, candidate_champion);
        if threat <= 0.0 {
            continue;
        }
        worst_counter = worst_counter.max(threat);
        counter_total += threat;
        counter_count += 1;
    }

    if counter_count == 0 {
        return 12.0;
    }

    let average_counter = counter_total / counter_count as f32;
    -(worst_counter * 0.72 + average_counter * 0.28)
}

fn strategic_ban_score(ctx: &DraftScoreContext, candidate: usize) -> f32 {
    let Some(candidate_info) = info_for_candidate(candidate) else {
        return 0.0;
    };
    let Some(candidate_champion) = champion_for_candidate(candidate) else {
        return 0.0;
    };
    let mut score = 0.0;

    for (ally_slot, ally_pick) in ctx.ally_pick.iter().enumerate() {
        let Some(ally_lane) = slot_position(ally_slot) else {
            continue;
        };
        if !candidate_info.positions.contains(&ally_lane) {
            continue;
        }
        let Some(ally_champion) = champion_for_candidate(*ally_pick) else {
            continue;
        };
        let threat = matchup_score(candidate_champion, ally_champion);
        if threat > 0.0 {
            score += threat * 0.65;
        }
    }

    score
}

fn matchup_score(attacker: PokemonChampion, defender: PokemonChampion) -> f32 {
    let offensive_edge = offensive_type_edge(attacker, defender);
    let defensive_liability = offensive_type_edge(defender, attacker);
    let type_score = (offensive_edge - defensive_liability * 0.75) * 420.0;

    type_score + strategy_matchup_score(attacker, defender)
}

fn strategy_matchup_score(attacker: PokemonChampion, defender: PokemonChampion) -> f32 {
    let Some(attacker_strategy) = strategy_for_champion_id(attacker.id) else {
        return 0.0;
    };
    let Some(defender_strategy) = strategy_for_champion_id(defender.id) else {
        return 0.0;
    };

    let attacker_threats =
        shared_trait_count(attacker_strategy.threatens, defender_strategy.traits);
    let defender_vulnerabilities =
        shared_trait_count(defender_strategy.vulnerable_to, attacker_strategy.traits);
    let defender_threats =
        shared_trait_count(defender_strategy.threatens, attacker_strategy.traits);
    let attacker_vulnerabilities =
        shared_trait_count(attacker_strategy.vulnerable_to, defender_strategy.traits);

    let mut score = 0.0;
    score += attacker_threats as f32 * 34.0;
    score += defender_vulnerabilities as f32 * 42.0;
    score -= defender_threats as f32 * 22.0;
    score -= attacker_vulnerabilities as f32 * 28.0;

    if has_trait(attacker_strategy, StrategyTrait::TankBuster)
        && has_trait(defender_strategy, StrategyTrait::HighHealth)
    {
        score += 32.0;
    }
    if has_trait(attacker_strategy, StrategyTrait::PercentHealthDamage)
        && has_trait(defender_strategy, StrategyTrait::HighHealth)
    {
        score += 48.0;
    }
    if has_trait(attacker_strategy, StrategyTrait::AntiHeal)
        && has_trait(defender_strategy, StrategyTrait::HealReliant)
    {
        score += 48.0;
    }
    if has_trait(attacker_strategy, StrategyTrait::Disabler)
        && has_trait(defender_strategy, StrategyTrait::HealReliant)
    {
        score += 26.0;
    }
    if has_trait(attacker_strategy, StrategyTrait::Dive)
        && has_trait(defender_strategy, StrategyTrait::BacklineCarry)
    {
        score += 24.0;
    }
    if has_trait(attacker_strategy, StrategyTrait::HardCc)
        && has_trait(defender_strategy, StrategyTrait::HighMobility)
    {
        score += 20.0;
    }

    score
}

fn eeveelution_race_score(ctx: &DraftScoreContext, candidate: PokemonChampion) -> f32 {
    let Some(candidate_strategy) = strategy_for_champion_id(candidate.id) else {
        return 0.0;
    };
    if !has_trait(candidate_strategy, StrategyTrait::Eeveelution) {
        return 0.0;
    }

    let ally_count = eeveelution_count(ctx.ally_pick);
    let enemy_count = eeveelution_count(ctx.enemy_pick);
    let next_ally_count = ally_count + 1;
    let count_delta = next_ally_count as isize - enemy_count as isize;

    let mut score = ally_count as f32 * 8.0;
    if count_delta > 0 {
        score += count_delta as f32 * 12.0;
    } else if count_delta == 0 {
        score += 4.0;
    } else {
        score -= (-count_delta) as f32 * 18.0;
        if enemy_count > 0 {
            score -= 8.0;
        }
    }

    score
}

fn eeveelution_count(picks: &[usize]) -> usize {
    picks
        .iter()
        .filter_map(|pick| champion_for_candidate(*pick))
        .filter(|champion| is_eeveelution_champion(*champion))
        .count()
}

fn is_eeveelution_champion(champion: PokemonChampion) -> bool {
    strategy_for_champion_id(champion.id)
        .is_some_and(|strategy| has_trait(strategy, StrategyTrait::Eeveelution))
}

fn ally_composition_score(ctx: &DraftScoreContext, candidate: PokemonChampion) -> f32 {
    let Some(candidate_strategy) = strategy_for_champion_id(candidate.id) else {
        return 0.0;
    };

    let mut score = 0.0;
    let mut same_primary_type = 0;
    let mut ad_count = usize::from(has_trait(candidate_strategy, StrategyTrait::AdDamage));
    let mut ap_count = usize::from(has_trait(candidate_strategy, StrategyTrait::ApDamage));
    let mut front_count = usize::from(has_trait(candidate_strategy, StrategyTrait::Frontline));
    let mut sustain_count = usize::from(has_trait(candidate_strategy, StrategyTrait::Healer));
    let mut shared_vulnerability_count = 0;

    for ally_pick in ctx.ally_pick {
        let Some(ally) = champion_for_candidate(*ally_pick) else {
            continue;
        };
        if ally.types.primary == candidate.types.primary {
            same_primary_type += 1;
        }
        let Some(ally_strategy) = strategy_for_champion_id(ally.id) else {
            continue;
        };

        ad_count += usize::from(has_trait(ally_strategy, StrategyTrait::AdDamage));
        ap_count += usize::from(has_trait(ally_strategy, StrategyTrait::ApDamage));
        front_count += usize::from(has_trait(ally_strategy, StrategyTrait::Frontline));
        sustain_count += usize::from(has_trait(ally_strategy, StrategyTrait::Healer));
        shared_vulnerability_count += shared_trait_count(
            candidate_strategy.vulnerable_to,
            ally_strategy.vulnerable_to,
        );
    }

    if same_primary_type >= 2 {
        score -= 36.0 * (same_primary_type - 1) as f32;
    }
    if shared_vulnerability_count >= 2 {
        score -= 12.0 * shared_vulnerability_count as f32;
    }
    if ad_count >= 4 || ap_count >= 4 {
        score -= 42.0;
    } else if ad_count > 0 && ap_count > 0 {
        score += 18.0;
    }
    if ctx.ally_pick.len() >= 2 && front_count == 0 {
        score -= 34.0;
    }
    if ctx.ally_pick.len() >= 3 && sustain_count == 0 {
        score -= 22.0;
    }

    score
}

fn incompatible_attach_pair_score(ctx: &DraftScoreContext, candidate: PokemonChampion) -> f32 {
    let incompatible_id = match candidate.id {
        "pokemon_moba_clawitzer" => "pokemon_moba_comfey",
        "pokemon_moba_comfey" => "pokemon_moba_clawitzer",
        _ => return 0.0,
    };

    if ctx
        .ally_pick
        .iter()
        .filter_map(|pick| champion_for_candidate(*pick))
        .any(|ally| ally.id == incompatible_id)
    {
        INCOMPATIBLE_ATTACH_PAIR_SCORE
    } else {
        0.0
    }
}

fn offensive_type_edge(attacker: PokemonChampion, defender: PokemonChampion) -> f32 {
    let moves = [
        attacker.attack,
        attacker.skill,
        attacker.skill2,
        attacker.ult.unwrap_or(attacker.skill2),
    ];
    let mut weighted_edge = 0.0;
    let mut total_weight = 0.0;

    for pokemon_move in moves {
        if !move_deals_enemy_damage(pokemon_move) {
            continue;
        }
        let weight = move_weight(pokemon_move);
        let (num, den) =
            total_modifier_ratio(pokemon_move.move_type, attacker.types, defender.types);
        let modifier = num as f32 / den as f32;
        weighted_edge += (modifier - 1.0) * weight;
        total_weight += weight;
    }

    if total_weight == 0.0 {
        0.0
    } else {
        weighted_edge / total_weight
    }
}

fn move_weight(pokemon_move: PokemonMove) -> f32 {
    match pokemon_move.slot {
        crate::pokemon_content::ActionSlot::Attack => 0.55,
        crate::pokemon_content::ActionSlot::Skill => 1.0,
        crate::pokemon_content::ActionSlot::Skill2 => 0.9,
        crate::pokemon_content::ActionSlot::Ult => 0.75,
    }
}

fn move_deals_enemy_damage(pokemon_move: PokemonMove) -> bool {
    !matches!(
        pokemon_move.effect,
        PokemonMoveEffect::None
            | PokemonMoveEffect::TargetSkillBlock { .. }
            | PokemonMoveEffect::TargetTerrify { .. }
            | PokemonMoveEffect::SelfDefenseBuff { .. }
            | PokemonMoveEffect::SelfProtectDefenseBuff { .. }
            | PokemonMoveEffect::SelfHeal { .. }
            | PokemonMoveEffect::SelfSleepHeal { .. }
            | PokemonMoveEffect::SelfHealResetPorygonType { .. }
            | PokemonMoveEffect::BlisseyHealAura { .. }
            | PokemonMoveEffect::GlobalAllyHealPercent { .. }
            | PokemonMoveEffect::TargetAttackDebuffProtect { .. }
            | PokemonMoveEffect::TargetAttackDebuffByDefense { .. }
            | PokemonMoveEffect::TargetOffenseDebuff { .. }
            | PokemonMoveEffect::TargetDefenseDebuffStun { .. }
            | PokemonMoveEffect::TargetLockOn { .. }
            | PokemonMoveEffect::TargetPoison { .. }
            | PokemonMoveEffect::PainSplit
            | PokemonMoveEffect::AreaMiasmaSlow { .. }
            | PokemonMoveEffect::SelfOffenseBuff { .. }
            | PokemonMoveEffect::StealthRockBuff { .. }
            | PokemonMoveEffect::SelfOffenseCritBuff { .. }
            | PokemonMoveEffect::SelfSpeedCritBuff { .. }
            | PokemonMoveEffect::BloodMoon { .. }
            | PokemonMoveEffect::AreaSoak { .. }
            | PokemonMoveEffect::ApplyConfusion { .. }
            | PokemonMoveEffect::ForceSelfCrit { .. }
            | PokemonMoveEffect::FutureSight { .. }
            | PokemonMoveEffect::WishChannel { .. }
            | PokemonMoveEffect::BatonPass { .. }
            | PokemonMoveEffect::AquaRing { .. }
            | PokemonMoveEffect::MistyTerrain { .. }
            | PokemonMoveEffect::BrineField { .. }
            | PokemonMoveEffect::CharmHeal { .. }
            | PokemonMoveEffect::SawkThrohSwitch
    )
}

fn pick_score(ctx: &DraftScoreContext, candidate: usize) -> f32 {
    let Some(info) = info_for_candidate(candidate) else {
        return 0.0;
    };
    let candidate_champion = champion_for_candidate(candidate);
    let primary_counts = position_counts(ctx.ally_pick);
    let coverage_counts = coverage_counts(ctx.ally_pick);
    let mut score = 0.0;

    if let Some(slot_position) = slot_position(ctx.ally_pick.len()) {
        let (slot_has_primary, slot_has_coverage) = available_slot_fit(ctx, slot_position);
        let primary_fit = info.positions.first().copied() == Some(slot_position);
        let secondary_fit = !primary_fit && info.positions.contains(&slot_position);

        score += if primary_fit {
            SLOT_PRIMARY_FIT_SCORE
        } else if secondary_fit {
            if slot_has_primary {
                SLOT_SECONDARY_FIT_SCORE - 75_000.0
            } else {
                SLOT_SECONDARY_FIT_SCORE
            }
        } else if slot_has_coverage {
            SLOT_MISMATCH_SCORE
        } else {
            -250_000.0
        };
    }

    for (index, position) in info.positions.iter().enumerate() {
        if *position >= POSITION_COUNT {
            continue;
        }
        if coverage_counts[*position] == 0 {
            score += if index == 0 {
                SLOT_OPEN_PRIMARY_SCORE
            } else {
                SLOT_OPEN_SECONDARY_SCORE
            };
        } else if index == 0 {
            score += SLOT_DUPLICATE_PRIMARY_SCORE * primary_counts[*position] as f32;
        } else {
            score += SLOT_DUPLICATE_COVERAGE_SCORE * coverage_counts[*position] as f32;
        }
    }

    let missing_support = coverage_counts[SUPPORT] == 0;
    let missing_jungle = coverage_counts[JUNGLE] == 0;
    let covers_support = info.positions.contains(&SUPPORT);
    let covers_jungle = info.positions.contains(&JUNGLE);

    if ctx.ally_pick.len() >= 3 && missing_support {
        score += if covers_support {
            MISSING_SLOT_COVER_SCORE
        } else {
            MISSING_SLOT_MISS_SCORE
        };
    }
    if ctx.ally_pick.len() >= 3 && missing_jungle {
        score += if covers_jungle {
            MISSING_SLOT_COVER_SCORE
        } else {
            MISSING_SLOT_MISS_SCORE
        };
    }

    if ctx.ally_pick.len() >= 4 {
        let mut covers_any_missing = false;
        for position in 0..POSITION_COUNT {
            if coverage_counts[position] == 0 {
                if info.positions.contains(&position) {
                    covers_any_missing = true;
                    score += MISSING_SLOT_COVER_SCORE;
                } else {
                    score += MISSING_SLOT_MISS_SCORE;
                }
            }
        }
        if !covers_any_missing {
            score += MISSING_SLOT_MISS_SCORE;
        }
    }

    score += strategic_pick_score(ctx, candidate, info);
    if let Some(candidate_champion) = candidate_champion {
        score += incompatible_attach_pair_score(ctx, candidate_champion);
    }

    score
}

fn ban_score(ctx: &DraftScoreContext, candidate: usize) -> f32 {
    let Some(info) = info_for_candidate(candidate) else {
        return 0.0;
    };
    if ctx.enemy_pick.is_empty() {
        return 0.0;
    }

    let enemy_coverage = coverage_counts(ctx.enemy_pick);
    let mut score = 0.0;

    for (index, position) in info.positions.iter().enumerate() {
        if *position >= POSITION_COUNT {
            continue;
        }
        if enemy_coverage[*position] == 0 {
            score += if index == 0 { 45.0 } else { 28.0 };
        }
    }

    if ctx.enemy_pick.len() >= 2
        && enemy_coverage[SUPPORT] == 0
        && info.positions.contains(&SUPPORT)
    {
        score += 60.0;
    }
    if ctx.enemy_pick.len() >= 2 && enemy_coverage[JUNGLE] == 0 && info.positions.contains(&JUNGLE)
    {
        score += 45.0;
    }

    score += eeveelution_ban_score(ctx, candidate);
    score += strategic_ban_score(ctx, candidate);

    score
}

fn eeveelution_ban_score(ctx: &DraftScoreContext, candidate: usize) -> f32 {
    let Some(candidate_champion) = champion_for_candidate(candidate) else {
        return 0.0;
    };
    if !is_eeveelution_champion(candidate_champion) {
        return 0.0;
    }

    let enemy_count = eeveelution_count(ctx.enemy_pick);
    if enemy_count == 0 {
        return 0.0;
    }

    let Some(candidate_strategy) = strategy_for_champion_id(candidate_champion.id) else {
        return 0.0;
    };

    let mut score = enemy_count as f32 * 10.0;
    if has_trait(candidate_strategy, StrategyTrait::BacklineCarry)
        || has_trait(candidate_strategy, StrategyTrait::SingleTargetAssassin)
        || has_trait(candidate_strategy, StrategyTrait::Healer)
    {
        score += 8.0;
    }

    score
}

#[derive(Debug)]
pub struct PokemonDraftScoreHook;

impl ModDraftScoreHook for PokemonDraftScoreHook {
    fn id(&self) -> &str {
        "pokemon_moba_position_draft_score"
    }

    fn priority(&self) -> i32 {
        100
    }

    fn score_pick(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        crate::crash_probe::catch_unwind_probe(
            "draft_score_pick",
            format!("candidate={candidate}"),
            DraftScoreDecision::Pass,
            || {
                let score = pick_score(ctx, candidate);
                if score == 0.0 {
                    DraftScoreDecision::Pass
                } else {
                    DraftScoreDecision::Replace(score)
                }
            },
        )
    }

    fn score_ban(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        crate::crash_probe::catch_unwind_probe(
            "draft_score_ban",
            format!("candidate={candidate}"),
            DraftScoreDecision::Pass,
            || {
                let score = ban_score(ctx, candidate);
                if score == 0.0 {
                    DraftScoreDecision::Pass
                } else {
                    DraftScoreDecision::Add(score)
                }
            },
        )
    }
}
