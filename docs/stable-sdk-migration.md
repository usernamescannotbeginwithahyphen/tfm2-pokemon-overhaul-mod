# Stable SDK Migration

## Current Build: 0.2.10 / SDK 0.6.4

- Build 0.2.10 adds the complete roster VFX checklist, corrects delayed and
  conditional visuals, mark/chit states and missing projectile routes. Read
  `vfx-roster-review.md`: directional anchoring, composite artwork and several
  animation phases remain unresolved. No live visual sign-off is claimed.

- Build 0.2.6 replaces the move-toward-only web/pulse spotting consumer with
  shared team detection used by target gates, cast selection, local threat
  assessment, retreat and unsafe-route avoidance. Ampharos detects continuously.
  Last-known positions expire after two seconds; stale memory cannot authorize
  targeted attacks. Host input validation and real untargetability remain intact.
  See `sdk-0.6.4-findings.md` for behavior, safeguards and live-test limits.

Updated 2026-10-18. The single active package is `mod/pokemon_moba_stable`.
Cargo uses the official Steam installation's `mod-sdk-stable/mod-api-stable`.
Its `base_version.txt` reports 0.6.4 and its ABI level is 10. The package now
requires game >=0.6.4 and ABI 10. Do not downgrade to the 0.5.8 download merely
because GitHub's releases/latest endpoint still selects that older release.

Official docs were refreshed to commit `064c2fa` in `.mod-docs`.
See [SDK 0.6.4 findings](sdk-0.6.4-findings.md) for the capabilities audit,
remaining work, and in-game acceptance checks.

## Runtime Ownership

- All 103 champions remain native registrations with the existing Pokemon
  actions, passives, type/STAB resolver, status logic, draft scorer, and AI.
- `action_name()` returns attack, skill, skill2, or ult. Never display names.
- Do not ship same-ID companion `.data_champion` files or synthetic damage nodes.
  The companion exporter remains ignored in ordinary tests.
- New standalone `.view_effects` assets are supported by the SDK and can supply
  visuals without replacing native champion ownership. Build 0.2.1 registers all
  279 staged assets and emits cast/projectile/state events across the roster.
  See the capability audit for timing and visual-validation limitations.
- No custom statistics server injector or manual client VFX renderer is registered.
  Engine damage/heal calls remain the persistence path.
- The equipment server extension is read-only: it snapshots ItemSetting for
  equipment-aware skills. It is NOT the retired statistics injector.

## Changes In This Upgrade

- Build 0.2.5 reads the actual bush grid through `StableMapCustomizer` and freezes
  it per matched simulation for Shiftry. True-sight overrides are preserved.
  Check the capability audit's capture/bind diagnostics on the first live match.
- Build 0.2.4 addresses the whole-roster AI adapter, dash rejection, engine CC
  cleanse, anti-heal ownership, intended teleports, terrain passage, native Hunt
  stealth, copied Gholdengo rewards and simulation-state partitioning. Read the
  [implementation notes](sdk-0.6.4-findings.md#build-024-audit-fixes) before testing.
- Build 0.2.3 adds native projectile impact resolution, special Dreepy/cannonball/
  Bonemerang paths, flat +10 credited-CS rewards, weighted lane priors and explicit
  engine-versus-resolver crit ownership. See the capability audit for preserved
  behavior, deliberate timing changes and the custom crit item-proc limitation.
- `StablePassive::on_kill` now forwards its victim argument to the legacy callback;
  the owner argument must never be mistaken for the killed entity.
- `on_unit_kill` is forwarded separately. Snorlax (and Receiver copying it) gains
  one berry on credited minion/camp kills, respecting passive suppression.
- Existing WithShield requests now create actual engine shield layers; the old
  adapter transmitted the duration kind but discarded the shield amount.
- Action charge counts are clamped to at least one at the SDK boundary.
- Native lane priors now use the roster's existing primary/secondary positions.
  Post-draft `decide_positions` now maximizes supported-lane coverage, then primary
  preference, over all five-player permutations. Draft scoring resolves live names
  instead of assuming the host has exactly 60 base champions. No UI capture is used.
- The internal combat ledger measures HP/shield loss rather than crediting the raw
  damage request. This is an observation of the net delta, not a replacement for
  engine damage events when concurrent procs heal or shield.
- Removed the extra fallback kill-gold award because the engine now awards it.
- Commented out the five old statistics-injector call sites and compile-excluded
  both private-layout mutation functions. Historical source is retained, not enabled.
- On-demand adapter methods expose item keys, statistics JSON, and view-effect/SFX
  calls. Adding these methods does not itself wire every move's visuals.

## Compatibility And Validation

Source compilation and adapter tests do not execute the game's damage pipeline.
Full match, statistics persistence, crit/proc behavior, and replay checks remain
required. In particular, the Pokemon resolver has bespoke crit/passive logic that
must be checked against the engine pipeline introduced in 0.5.4.

Use a new test save. Earlier experimental saves may retain deleted data-champion
definitions and missing native effect references. The August log showed such a
version mismatch, but it did not prove the exact source of its divide-by-zero panic.
The current upstream docs separately report a fix for zero charge counts.

## SDK Update Procedure

1. Inspect both the official release and the installed SDK version. Use the newest
   official SDK compatible with the installed game; retain one active build path.
2. Refresh `.mod-docs`, record its commit and the SDK/game/ABI versions.
3. Remove obsolete downloaded SDK trees and archives after confirming the active
   build uses the current installation. Keep project code history in Git.
4. Read changed API contracts, including behavior changes to existing calls.
5. Build and test native roster/action/passive compatibility first.
6. Implement supported improvements with focused tests. Preserve existing
   workarounds where the full replacement contract is still absent.
7. Validate with a new save, watched match, background simulation and replay.
   Never call compilation alone proof of combat or statistics parity.

The build script checks the SDK version and verifies that Cargo's resolved crate
matches the requested GameRoot; it fails on mismatched SDK paths.
