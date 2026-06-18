# Temporary Day-Advance Crash Probe

Status: active in source as of June 6, 2026. Remove this after the day-advance crash is fixed or proven unrelated. The server roster callback is disabled because both early and late server database roster reads caused access violations.

## Purpose

The game still sometimes crashes while advancing a day. The TFM2 log records `attempt to divide by zero`, and Windows has reported `pokemon_moba.dll` as the faulting module when a server management callback was active. Symbolizing those crash offsets landed in `pokemon_roster.rs` after the engine panic, so the callback is disabled while the underlying divide-by-zero is isolated.

The current soft-lock path no longer hard-crashes, but the TFM2 log still records repeated `attempt to divide by zero` panics during day advancement. Callback-level probes are now active so the next test can tell us whether the panic originates inside our mod callbacks or after control returns to the engine.

## Active Probe Files

- `mod/pokemon_moba/src/crash_probe.rs`
  - Installs a Rust panic hook during mod init.
  - Writes panic payload and file/line, when available, to `logs/panic-probe.log`.
  - Provides `catch_unwind_probe(...)` for temporary callback-level probes.
- `mod/pokemon_moba/src/pokemon_content.rs`
  - Temporarily wraps Pokemon combat effect, passive, and player-input-AI callbacks.
  - Writes callback panics to `logs/mod-panic-probe.log` with champion/action/entity context.
  - Also hardens line/rectangle/cone targeting helpers against zero denominators from degenerate or overflowed coordinates.
  - Reports action metadata to the engine with non-zero duration, cooldown, start timing, and range. The June 6 static audit found three `start_timing: 0` actions (`pokemon_moba_hawlucha` skill2, `pokemon_moba_bouffalant` skill, `pokemon_moba_delibird` skill2), which could trigger engine-side divide-by-zero before any combat callback runs.
  - Temporarily converts Delibird `skill2` from a no-target `PokemonMoveEffect::None` placeholder into `Present (Reserve)`, a long-cooldown, charge-gated copy of Present (Healing). This keeps the registered slot valid for base AI, copied/forced action paths, and engine simulation while day-advance hangs are being isolated.
  - Hardens `expected_movement_for_ai()` so effect variants with optional movement no longer report `(0 ticks, 0 distance)` as a movement expectation. The June 6 follow-up audit found Blaziken's Blaze Kick uses `SelfBuffAreaDamageKnockbackChanceBurn` with `knockback_speed: 0` and `knockback_ticks: 0`; that is valid gameplay data, but unsafe AI metadata if the engine divides by expected movement duration or distance.
  - Hardens additional SDK-facing AI metadata after the June 6 15:21 hang log still showed repeated engine-side `attempt to divide by zero` panics with no `mod-panic-probe.log`: `expected_cc_time()` no longer returns `Some(0)`, and `expected_buff()` no longer reports zero-duration or empty buff states. This preserves combat behavior and only changes base-game AI/sim valuation hints.
  - Static-audited executable `/` and `%` operations in `pokemon_content.rs` on June 6 after logs stopped giving useful origin details. Remaining variable integer denominators are guarded by `.max(1)`, `saturating_div`, explicit non-zero checks, or non-empty collection checks. The three `% candidates.len()` sites now go through `random_index(...)`, which returns `None` for zero-length collections before modulo is evaluated.
  - Temporarily clamps `ModEffect.growth_range` to at least 1 along with range/duration/cooldown/start timing. All Pokemon actions currently declare `growth_range: 0`; if the engine treats this as a denominator during management simulation, this removes that zero without changing visible range in any meaningful way.
  - Temporarily wraps `expected_damage`, `expected_heal`, `expected_shield`, `expected_cc_time`, `expected_buff`, `expected_move_distance`, and `expected_rush_effect` with panic probes. Per-call success logging to `logs/metadata-probe.log` was disabled after the June 6 16:02 test produced about 138 MB in a couple minutes and likely distorted day-advance performance. Re-enable only for short, targeted runs.
  - Temporarily floors real registered actions with computed `expected_damage == (0, 0)` to a 1-point AI-only expected damage value. The June 6 16:09 clean run still produced repeated engine-side divide-by-zero panics with no mod callback panic, so this removes the remaining all-zero damage valuation path without changing combat damage.
  - Open follow-up: end-of-game damage summary charts often show zero damage/taken for Pokemon even though Pokemon effects route through `GameCtx::deal_damage`. Grapploct showing 80 damage once suggests some paths are recorded, but not all Pokemon-layer damage is reflected in the stock summary. This is separate from the current day-advance divide-by-zero unless future logs connect them.
- `mod/pokemon_moba/src/pokemon_positions.rs`
  - Temporarily wraps Pokemon draft score callbacks.
  - Writes callback panics to `logs/mod-panic-probe.log` with candidate context.
- `mod/pokemon_moba/src/pokemon_status.rs`
  - Hardens the duplicate segment-distance helper against zero denominators from degenerate or overflowed coordinates.
- `mod/pokemon_moba/src/lib.rs`
  - Adds `mod crash_probe;`.
  - Calls `crash_probe::install_panic_hook()` in `init`.
- `mod/pokemon_moba/src/pokemon_roster.rs`
  - Kept in source for reference, but not compiled while `set_server_extension(...)` is disabled.
  - Do not re-register `PokemonRosterServerExtension` without a substantially different lifecycle strategy.
  - `after_management_tick` crashed after the engine logged `attempt to divide by zero`: first while iterating `Database.pre_patch_data`, then while reading `Database.available_champions`.
  - `on_server_start` / `before_management_tick` also failed during Start New Game on June 6, 2026. Windows reported `pokemon_moba.dll+0xe27e4`, symbolized to `pokemon_roster.rs:88` inside `retain_known_pokemon_only_and_complete` while counting/filtering a roster vector.
  - Conclusion: direct server-side roster vector reads are unsafe in the currently tested lifecycle hooks.
- `mod/pokemon_moba/Cargo.toml`
  - Temporarily enables release debug info so local crash-offset symbolization has a better chance of producing source lines.

## Expected Logs

- `logs/panic-probe.log`
  - Should contain `event=panic` with a `location="file:line:column"` field if the divide-by-zero panic originates in Rust code with location metadata.
- `logs/mod-panic-probe.log`
  - Should contain `event=effect_apply`, `event=passive_on_update`, `event=passive_on_damaged`, `event=passive_on_kill`, `event=passive_on_spawn`, `event=input_ai_think`, `event=draft_score_pick`, or `event=draft_score_ban` if our callback panics.
  - If the TFM2 log still shows `attempt to divide by zero` and this file stays absent, the panic is probably not unwinding through one of our wrapped callbacks.
- `logs/roster-probe.log`
  - No longer expected while the server roster callback is disabled.

## Removal Checklist

- Delete `mod/pokemon_moba/src/crash_probe.rs`.
- Remove `mod crash_probe;` and `crash_probe::install_panic_hook()` from `mod/pokemon_moba/src/lib.rs`.
- Leave `mod/pokemon_moba/src/pokemon_roster.rs` disabled unless a safer lifecycle hook or non-server data path is found.
- Remove the `catch_unwind_probe(...)` wrappers from `mod/pokemon_moba/src/pokemon_content.rs` and `mod/pokemon_moba/src/pokemon_positions.rs`.
- Revert Delibird `skill2` if we confirm the no-op placeholder was unrelated, or keep a real second action if engine simulation proves unstable with registered no-op slots.
- Keep the zero-movement expected-value guard unless the SDK documents that `(0, 0)` movement expectations are valid. This guard should be harmless for gameplay because it only changes AI valuation metadata.
- Keep the zero-CC and zero/empty expected-buff guards unless the SDK documents that zero-valued `Some(...)` AI expectations are valid. These guards should be harmless for gameplay because they only change AI valuation metadata.
- Remove the metadata panic wrappers and decide whether to keep or revert the `growth_range.max(1)` clamp and 1-point AI expected-damage floor after the advance-day hang is isolated.
- Delete or archive `mod/pokemon_moba/src/pokemon_roster.rs` if the disabled server callback notes are no longer useful.
- Remove the temporary `[profile.release] debug = 2` setting from `mod/pokemon_moba/Cargo.toml` if smaller release artifacts are preferred after debugging.
- Delete `logs/panic-probe.log`, `logs/mod-panic-probe.log`, `logs/metadata-probe.log`, and `logs/roster-probe.log` if they are no longer needed.
