# Temporary Combat Probe

Status: removed from active source as of June 6, 2026. The first spawn-plus-apply probe was rolled back after a startup crash, and the later apply-only probe was removed after it produced enough evidence to answer the native binding question. The startup crash was later traced to missing sprite override metadata, not the probe.

This note records the temporary instrumentation attempt for the TFM2 native multi-champion binding issue described in `C:\Users\james\Downloads\tfm2_multichampion_binding_bug.pdf`.

## Purpose

The PDF reports two symptoms worth checking in this mod:

- Native `ModEffectType::apply` may only fire for 1-2 in-match entity slots when a native mod registers many champions.
- Runtime reads from `ctx.get_entity(...).stat()` and `ctx.get_entity(...).hp()` may return wrong values for modded entities.

This mod is native-only, so a full `apply` binding failure should be obvious in game. The temporary probe verified that assumption by writing a log line whenever a Pokemon native effect applied.

## Removed Apply-Only Probe

- `mod/pokemon_moba/src/combat_probe.rs`
  - Temporary file logger, now deleted.
  - Gated by `logs/enable-combat-probe.txt`.
  - Writes `logs/combat-probe.log`.
  - Throttles `apply` lines to the first 3 lines per champion/action/entity key.
- `mod/pokemon_moba/src/lib.rs`
  - Temporarily added `mod combat_probe;`, now removed.
- `mod/pokemon_moba/src/pokemon_content.rs`
  - Temporarily wrote `event=apply` from `PokemonEffect::apply` with champion id, action slot, entity id, level, team, HP, runtime stat, and expected base stat. The call site is now removed.

There is no combat probe in the active DLL.

## Enable And Test

Build and deploy the mod:

```powershell
.\tools\build-native.ps1
.\tools\deploy.ps1
```

Enable the probe and clear the previous probe log:

```powershell
New-Item -ItemType File -Force .\logs\enable-combat-probe.txt
Remove-Item .\logs\combat-probe.log -ErrorAction SilentlyContinue
```

Then start TFM2 and play 1-2 matches with several different Pokemon selected across both teams.

After a match, inspect the log:

```powershell
Get-Content .\logs\combat-probe.log -Tail 200
```

## Rolled-Back Probe Files And Call Sites

- `mod/pokemon_moba/src/combat_probe.rs` was added, then deleted.
  - Temporary file logger.
  - Gated by `logs/enable-combat-probe.txt`.
  - Writes `logs/combat-probe.log`.
  - Throttles `apply` lines to the first 3 lines per champion/action/entity key.
- `mod/pokemon_moba/src/lib.rs` temporarily added `mod combat_probe;`, then removed it.
- `mod/pokemon_moba/src/pokemon_content.rs` temporarily added:
  - `PokemonEffect::apply`: `event=apply` with champion id, action slot, entity id, level, team, HP, runtime stat, and expected base stat.
  - `PokemonPassive::on_spawn`: `event=spawn` with champion id, entity id, and player id.

## Startup Failure

With `logs/enable-combat-probe.txt` enabled, TFM2 loaded the main menu briefly, then went black and hung until Windows closed it. The game log at `C:\Users\james\AppData\Roaming\TeamSamoyed\TeamfightManager2\data\log.log` recorded:

```text
panic occurred: "called `Option::unwrap()` on a `None` value"
```

The probe code did not call `unwrap()`, so the immediate conclusion is only that this instrumentation build is not safe enough to use as-is.

## Current State

Both the spawn hook and the apply hook are removed. The archived result remains in `logs/combat-probe-20260606-124729.log`.

The old probe flag and old probe log were removed during rollback; recreate the flag when testing:

- `logs/enable-combat-probe.txt`

After the rollback, the same startup panic still occurred, so the probe was not the direct cause.

## Startup Crash Follow-Up

The deployed package matched the workspace byte-for-byte, which ruled out a partial deploy. Validation then found that three registered native champions had sprite files and text entries but were missing `#sheet` and `#anim` mappings in `mod/pokemon_moba/mod.override_info`:

- `pokemon_moba_arboliva`
- `pokemon_moba_armarouge`
- `pokemon_moba_ceruledge`

The missing mappings were added on June 6, 2026. After the patch, every registered native Pokemon has a `#sheet` and `#anim` override entry, and every remapped champion sprite file exists. Only `mod.override_info` was copied into the installed mod folder; no old DLL fallback was deployed.

## Safer Apply Probe Notes

If this probe ever needs to be recreated, keep it apply-only, throttled, and gated by `logs/enable-combat-probe.txt`.

## Original Test Goal

Once a safe probe exists, the log should answer these checks:

- If `event=apply` appears for more than 1-2 distinct entity ids and multiple champion ids during a match, the native `apply` binding is probably working for this mod.
- If many Pokemon spawn but `event=apply` only appears for 1-2 entity ids, the mod may be hitting the native binding issue from the PDF.
- Compare `runtime_stat` to `expected_base_stat`. Exact matches are not required because buffs, passives, items, or levels can change runtime stats. Severe default-looking values or mismatches on every modded entity would support the stat-read concern from the PDF.
- Compare `hp=current/max` against the expected base HP and what you see in game. Bad HP values on most Pokemon would also support the stat-read concern.

## June 6 Probe Result

The apply-only probe produced `logs/combat-probe-20260606-124729.log` before a separate day-advance crash. Summary:

- `event=apply` rows: 6,884
- Distinct champion ids: 77
- Distinct entity ids: 580
- Distinct champion/action/entity keys: 2,635

This strongly argues against the specific PDF binding failure where native `apply` only fires for 1-2 entity slots. Runtime stat reads are not default-looking; many differ from expected base stats in ways consistent with buffs, passives, items, level, or match state.

The day-advance crash was logged separately as `attempt to divide by zero`. Windows reported the faulting module as `pokemon_moba.dll`, and symbolizing the crash offset pointed at `PokemonMobaServerExtension::after_management_tick`, not `PokemonEffect::apply`. The probe flag was disabled, and `mod/pokemon_moba/src/pokemon_roster.rs` was hardened so runtime champion rosters are always Pokemon-only and are repopulated with the registered Pokemon list if they are empty or base-only.
