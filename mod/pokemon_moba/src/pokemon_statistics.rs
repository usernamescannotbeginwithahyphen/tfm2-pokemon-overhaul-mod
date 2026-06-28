use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

use mod_api::{ClientData, ModServerExtension, ServerModContext};

use crate::pokemon_status::{
    pokemon_combat_stat_snapshots, pokemon_player_identity_snapshots, PokemonCombatStatSnapshot,
};

const STAT_MERGE_LOG_PATH: &str =
    "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\stat-merge.log";

static STAT_MERGE_PROBE_COUNTS: OnceLock<Mutex<StatMergeProbeCounts>> = OnceLock::new();
static CLIENT_STAT_APPLIED: OnceLock<Mutex<Vec<ClientAppliedStatState>>> = OnceLock::new();

#[derive(Default)]
struct StatMergeProbeCounts {
    after_management_tick: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct ClientAppliedStatState {
    ctx_id: usize,
    player_id: usize,
    damage_dealt: usize,
    damage_taken: usize,
    healing_done: usize,
    kills: usize,
    deaths: usize,
}

pub struct PokemonStatisticsServerExtension;

impl ModServerExtension for PokemonStatisticsServerExtension {
    fn on_server_start(&self, _ctx: &mut ServerModContext<'_>) {
        write_stat_merge_log("event=stat_merge_server_start");
    }

    fn after_management_tick(&self, ctx: &mut ServerModContext<'_>) {
        crate::crash_probe::catch_unwind_probe(
            "stats_after_management_tick",
            String::new(),
            (),
            || {
                summarize_custom_combat_stats(ctx);
            },
        );
    }
}

fn summarize_custom_combat_stats(ctx: &mut ServerModContext<'_>) {
    let _ = ctx;
    let snapshots = pokemon_combat_stat_snapshots();
    log_stat_merge_tick(snapshots.len(), &snapshots);
}

fn log_stat_merge_tick(snapshot_count: usize, snapshots: &[PokemonCombatStatSnapshot]) {
    let counts =
        STAT_MERGE_PROBE_COUNTS.get_or_init(|| Mutex::new(StatMergeProbeCounts::default()));
    let Ok(mut counts) = counts.lock() else {
        return;
    };
    counts.after_management_tick = counts.after_management_tick.saturating_add(1);
    let tick_count = counts.after_management_tick;
    let should_log = tick_count <= 5 || tick_count % 100 == 0;
    if !should_log {
        return;
    }

    let mut ctx_ids = Vec::<(usize, usize, usize, usize, usize, usize, usize, usize)>::new();
    for snapshot in snapshots {
        if let Some(entry) = ctx_ids.iter_mut().find(|entry| entry.0 == snapshot.ctx_id) {
            entry.1 = entry.1.saturating_add(1);
            entry.2 = entry.2.saturating_add(snapshot.damage_dealt);
            entry.3 = entry.3.saturating_add(snapshot.damage_taken);
            entry.4 = entry.4.saturating_add(snapshot.kills);
            entry.5 = entry.5.saturating_add(snapshot.deaths);
            entry.6 = entry.6.saturating_add(snapshot.assists);
            entry.7 = entry.7.saturating_add(snapshot.healing_done);
        } else if ctx_ids.len() < 8 {
            ctx_ids.push((
                snapshot.ctx_id,
                1,
                snapshot.damage_dealt,
                snapshot.damage_taken,
                snapshot.kills,
                snapshot.deaths,
                snapshot.assists,
                snapshot.healing_done,
            ));
        }
    }

    let summary = ctx_ids
        .iter()
        .map(
            |(ctx_id, rows, damage_dealt, damage_taken, kills, deaths, assists, healing)| {
                format!(
                    "ctx:{} rows:{} dealt:{} taken:{} kills:{} deaths:{} assists:{} healing:{}",
                    ctx_id, rows, damage_dealt, damage_taken, kills, deaths, assists, healing
                )
            },
        )
        .collect::<Vec<_>>()
        .join(" | ");

    let rows = snapshots
        .iter()
        .take(8)
        .map(|snapshot| {
            format!(
                "player:{} athlete:{} source:{} team:{} pos:{} champ:{}",
                snapshot.player_id,
                snapshot
                    .athlete_id
                    .map(|athlete_id| athlete_id.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                snapshot.athlete_source,
                snapshot.team,
                snapshot.position as usize,
                snapshot.champion_id.unwrap_or("unknown")
            )
        })
        .collect::<Vec<_>>()
        .join(" | ");
    let mapped_rows = snapshots
        .iter()
        .filter(|snapshot| snapshot.athlete_id.is_some())
        .count();
    let exact_rows = snapshots
        .iter()
        .filter(|snapshot| snapshot.athlete_source == "exact")
        .count();
    let slot_rows = snapshots
        .iter()
        .filter(|snapshot| snapshot.athlete_source == "slot")
        .count();
    let player_rows = snapshots
        .iter()
        .filter(|snapshot| snapshot.athlete_source == "player")
        .count();
    let state_rows = snapshots
        .iter()
        .filter(|snapshot| snapshot.athlete_source == "state")
        .count();
    let unknown_rows = snapshots
        .iter()
        .filter(|snapshot| snapshot.athlete_source == "none")
        .count();
    let identities = pokemon_player_identity_snapshots();
    let unique_athlete_count = identities
        .iter()
        .fold(Vec::<usize>::new(), |mut athlete_ids, identity| {
            if !athlete_ids.contains(&identity.athlete_id) {
                athlete_ids.push(identity.athlete_id);
            }
            athlete_ids
        })
        .len();
    let identity_rows = identities
        .iter()
        .rev()
        .take(12)
        .map(|identity| {
            format!(
                "player:{} athlete:{} team:{} pos:{} champ:{} seen:{}",
                identity.player_id,
                identity.athlete_id,
                identity.team,
                identity.position as usize,
                identity.champion_name,
                identity.seen_count
            )
        })
        .collect::<Vec<_>>()
        .join(" | ");

    write_stat_merge_log(&format!(
        "event=stat_merge_tick count={} snapshots={} athlete_mapped={} exact={} slot={} player={} state={} unknown={} identities={} unique_athletes={} database=\"server_db_probe_disabled\" summary=\"{}\" rows=\"{}\" identity_rows=\"{}\"",
        tick_count,
        snapshot_count,
        mapped_rows,
        exact_rows,
        slot_rows,
        player_rows,
        state_rows,
        unknown_rows,
        identities.len(),
        unique_athlete_count,
        crate::crash_probe::sanitize_log_field(&summary),
        crate::crash_probe::sanitize_log_field(&rows),
        crate::crash_probe::sanitize_log_field(&identity_rows),
    ));
}

fn write_stat_merge_log(line: &str) {
    if let Some(parent) = Path::new(STAT_MERGE_LOG_PATH).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(STAT_MERGE_LOG_PATH)
    else {
        return;
    };

    let _ = writeln!(file, "{line}");
}

#[allow(dead_code)]
pub fn patch_client_statistics(data: &ClientData) {
    let snapshots = pokemon_combat_stat_snapshots();
    if snapshots.is_empty() {
        return;
    }

    let applied = CLIENT_STAT_APPLIED.get_or_init(|| Mutex::new(Vec::new()));
    let Ok(mut applied) = applied.lock() else {
        return;
    };

    let mut db = data.db_mut();
    let mut applied_rows = 0usize;
    let mut applied_damage = 0usize;
    let mut applied_kills = 0usize;
    let mut applied_deaths = 0usize;

    for snapshot in snapshots {
        let Some(champion_id) = snapshot.champion_id else {
            continue;
        };

        let already = applied_state_for(&mut applied, snapshot.ctx_id, snapshot.player_id);
        let delta_damage_dealt = snapshot.damage_dealt.saturating_sub(already.damage_dealt);
        let delta_damage_taken = snapshot.damage_taken.saturating_sub(already.damage_taken);
        let delta_healing = snapshot.healing_done.saturating_sub(already.healing_done);
        let delta_kills = snapshot.kills.saturating_sub(already.kills);
        let delta_deaths = snapshot.deaths.saturating_sub(already.deaths);

        if delta_damage_dealt == 0
            && delta_damage_taken == 0
            && delta_healing == 0
            && delta_kills == 0
            && delta_deaths == 0
        {
            continue;
        }

        let Some(champion_stats) = db.champion_patch_statistics.get_mut(champion_id) else {
            continue;
        };

        let mut patched_any_version = false;
        for season_stats in champion_stats.data.values_mut() {
            let position_stats = season_stats
                .by_position
                .entry(snapshot.position)
                .or_default();
            position_stats.kills = position_stats.kills.saturating_add(delta_kills);
            position_stats.deaths = position_stats.deaths.saturating_add(delta_deaths);
            position_stats.dealing = position_stats.dealing.saturating_add(delta_damage_dealt);
            position_stats.tanking = position_stats.tanking.saturating_add(delta_damage_taken);
            position_stats.healing = position_stats.healing.saturating_add(delta_healing);
            patched_any_version = true;
        }

        if !patched_any_version {
            continue;
        }

        already.damage_dealt = snapshot.damage_dealt;
        already.damage_taken = snapshot.damage_taken;
        already.healing_done = snapshot.healing_done;
        already.kills = snapshot.kills;
        already.deaths = snapshot.deaths;
        applied_rows = applied_rows.saturating_add(1);
        applied_damage = applied_damage.saturating_add(delta_damage_dealt);
        applied_kills = applied_kills.saturating_add(delta_kills);
        applied_deaths = applied_deaths.saturating_add(delta_deaths);
    }

    while applied.len() > 2048 {
        applied.remove(0);
    }

    if applied_rows > 0 {
        write_stat_merge_log(&format!(
            "event=client_stat_patch rows={} damage={} kills={} deaths={}",
            applied_rows, applied_damage, applied_kills, applied_deaths,
        ));
    }
}

fn applied_state_for(
    states: &mut Vec<ClientAppliedStatState>,
    ctx_id: usize,
    player_id: usize,
) -> &mut ClientAppliedStatState {
    if let Some(index) = states
        .iter()
        .position(|state| state.ctx_id == ctx_id && state.player_id == player_id)
    {
        return &mut states[index];
    }

    states.push(ClientAppliedStatState {
        ctx_id,
        player_id,
        ..Default::default()
    });
    states.last_mut().expect("applied stat state just pushed")
}
