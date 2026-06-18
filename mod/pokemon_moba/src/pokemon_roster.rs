use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

use mod_api::{ModServerExtension, ServerModContext};

use crate::crash_probe;
use crate::pokemon_content;

const POKEMON_CHAMPION_PREFIX: &str = "pokemon_moba_";
const ROSTER_PROBE_LOG_PATH: &str =
    "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\roster-probe.log";

static ROSTER_PROBE_COUNTS: OnceLock<Mutex<RosterProbeCounts>> = OnceLock::new();

pub struct PokemonRosterServerExtension;

impl ModServerExtension for PokemonRosterServerExtension {
    fn on_server_start(&self, ctx: &mut ServerModContext<'_>) {
        crash_probe::catch_unwind_probe("roster_on_server_start", String::new(), (), || {
            sanitize_runtime_rosters(ctx, "on_server_start");
        });
    }

    fn before_management_tick(&self, ctx: &mut ServerModContext<'_>) {
        crash_probe::catch_unwind_probe("roster_before_management_tick", String::new(), (), || {
            sanitize_runtime_rosters(ctx, "before_management_tick");
        });
    }
}

fn sanitize_runtime_rosters(ctx: &mut ServerModContext<'_>, hook: &str) {
    let fallback = pokemon_content::champion_ids();
    let mut summary = RosterFilterSummary::default();

    summary.add(
        "available",
        retain_known_pokemon_only_and_complete(&mut ctx.database.available_champions, &fallback),
    );

    summary.add(
        "custom",
        retain_known_pokemon_only_and_complete(
            &mut ctx.database.game_play_option.custom_champions,
            &fallback,
        ),
    );

    let mut patch_roster_count = 0usize;
    for patch_state in ctx.database.pre_patch_data.values_mut() {
        patch_roster_count += 1;
        summary.add(
            "patch",
            retain_known_pokemon_only_and_complete(&mut patch_state.available_champions, &fallback),
        );
    }

    if should_log_roster_probe(hook, summary.changed) {
        write_line(
            ROSTER_PROBE_LOG_PATH,
            &format!(
                "event=roster_filter hook={} rosters={} patches={} changed_rosters={} removed={} deduped={} completed={} before_total={} after_total={}",
                hook,
                summary.rosters,
                patch_roster_count,
                summary.changed_rosters,
                summary.removed_non_pokemon_or_unknown,
                summary.removed_duplicates,
                summary.added_missing_pokemon,
                summary.before_total,
                summary.after_total,
            ),
        );
    }
}

fn retain_known_pokemon_only_and_complete(
    roster: &mut Vec<String>,
    fallback: &[String],
) -> RosterFilterStats {
    let known_pokemon: HashSet<&str> = fallback.iter().map(String::as_str).collect();
    let before_total = roster.len();
    let before_known = roster
        .iter()
        .filter(|id| is_known_pokemon_champion_id(id, &known_pokemon))
        .count();

    let mut seen = HashSet::new();
    let mut filtered = Vec::with_capacity(before_total.max(fallback.len()));
    let mut removed_duplicates = 0usize;

    for id in roster.drain(..) {
        if !is_known_pokemon_champion_id(&id, &known_pokemon) {
            continue;
        }

        if seen.insert(id.clone()) {
            filtered.push(id);
        } else {
            removed_duplicates += 1;
        }
    }

    let mut added_missing_pokemon = 0usize;
    for id in fallback {
        if seen.insert(id.clone()) {
            filtered.push(id.clone());
            added_missing_pokemon += 1;
        }
    }

    let after_total = filtered.len();
    *roster = filtered;

    RosterFilterStats {
        before_total,
        after_total,
        removed_non_pokemon_or_unknown: before_total.saturating_sub(before_known),
        removed_duplicates,
        added_missing_pokemon,
    }
}

fn is_known_pokemon_champion_id(id: &str, known_pokemon: &HashSet<&str>) -> bool {
    id.starts_with(POKEMON_CHAMPION_PREFIX) && known_pokemon.contains(id)
}

fn should_log_roster_probe(hook: &str, changed: bool) -> bool {
    let counts = ROSTER_PROBE_COUNTS.get_or_init(|| Mutex::new(RosterProbeCounts::default()));
    let Ok(mut counts) = counts.lock() else {
        return changed;
    };

    let hook_count = match hook {
        "on_server_start" => {
            counts.on_server_start += 1;
            counts.on_server_start
        }
        "before_management_tick" => {
            counts.before_management_tick += 1;
            counts.before_management_tick
        }
        _ => {
            counts.other += 1;
            counts.other
        }
    };

    if changed {
        counts.changed += 1;
        return counts.changed <= 100 || counts.changed % 1000 == 0;
    }

    hook_count <= 5 || hook_count == 10 || hook_count == 50 || hook_count % 500 == 0
}

fn write_line(path: &str, line: &str) {
    if let Some(parent) = Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };

    let _ = writeln!(file, "{line}");
}

#[derive(Default)]
struct RosterProbeCounts {
    on_server_start: usize,
    before_management_tick: usize,
    other: usize,
    changed: usize,
}

#[derive(Default)]
struct RosterFilterSummary {
    rosters: usize,
    changed_rosters: usize,
    removed_non_pokemon_or_unknown: usize,
    removed_duplicates: usize,
    added_missing_pokemon: usize,
    before_total: usize,
    after_total: usize,
    changed: bool,
}

impl RosterFilterSummary {
    fn add(&mut self, _label: &str, stats: RosterFilterStats) {
        self.rosters += 1;
        self.before_total += stats.before_total;
        self.after_total += stats.after_total;
        self.removed_non_pokemon_or_unknown += stats.removed_non_pokemon_or_unknown;
        self.removed_duplicates += stats.removed_duplicates;
        self.added_missing_pokemon += stats.added_missing_pokemon;

        if stats.changed() {
            self.changed_rosters += 1;
            self.changed = true;
        }
    }
}

struct RosterFilterStats {
    before_total: usize,
    after_total: usize,
    removed_non_pokemon_or_unknown: usize,
    removed_duplicates: usize,
    added_missing_pokemon: usize,
}

impl RosterFilterStats {
    fn changed(&self) -> bool {
        self.removed_non_pokemon_or_unknown > 0
            || self.removed_duplicates > 0
            || self.added_missing_pokemon > 0
            || self.before_total != self.after_total
    }
}
