# Stable SDK Migration

This project now keeps the released classic package at `mod/pokemon_moba` and the 0.5.3 stable-ABI port at `mod/pokemon_moba_stable`.

## Rules For SDK Updates

- Keep only the latest SDK version in active use.
- Do not keep stale downloaded SDK folders around as build inputs.
- Stable builds should use `mod-sdk-stable` from the current Teamfight Manager 2 install unless we deliberately vendor the latest stable SDK for a release build.
- `C:\Users\james\Downloads\mod-sdk-stable.rar` was checked on 2026-07-30 and is byte-for-byte identical to the installed `Teamfight Manager2\mod-sdk-stable` folder.
- When a new SDK lands, update this note, the build scripts, and the active mod package together. Remove or ignore untracked/stale SDK copies before making migration decisions so old APIs and assets do not get mistaken for current support.

## Migration Targets

1. Port native registration from classic `mod_api` to `mod-api-stable`.
2. Register every Pokemon through `StableChampion`, `StableAction`, `StableEffectType`, and `StablePassive`.
3. Generate companion `.data_champion` files for view bindings while Rust owns runtime behavior.
4. Package staged VFX under `asset/pokemon_moba_stable/vfx`.
5. Bind projectile VFX through `StableSim::spawn_projectile(name, effect_name, spec)` plus matching `view_projectiles`.
6. Bind caster, target, buff, and field VFX through companion data actions, `view_effects`, and `view_buffs`.
7. Replace forced-dash teleport approximations with `StableSim::entity_set_pos` where the intended behavior is an instant teleport.
8. Rework tower-danger and champion-specific input decisions with `StablePlayerAi`, using read-only live sim data for towers, entities, cooldowns, buffs, CC, and projectiles.
9. Recode Gholdengo gold through stable player gold mutation instead of internal classic state edits.
10. Use `StableSim::spawn_unit` for summons where built-in chase/attack behavior is sufficient; use match hooks for custom summon orchestration.
11. Revisit custom kill-credit and kill-log handling last, after the stable damage and gold pathways are proven.

## Position Status In 0.5.3

The installed 0.5.3 stable SDK improves positions but does not expose a `StableChampion` preferred-position field.

- Draft hooks can read candidate champion IDs and current ally/enemy picks through `StableDraftContext`, but no explicit per-pick target lane is visible in the stable score call.
- The stable port infers the current lane slot from ally pick count using the same lane order the classic scorer used (`Top`, `Jungle`, `Mid`, `Bottom`, `Support`). This lets the hook strongly prefer lane-appropriate Pokemon, but it still cannot directly control any separate post-draft lane swap routine the engine may run.
- Player AI can read the assigned lane through `StableAiInit::lane`, `StableAiContext::lane`, and live `StablePlayer::lane`.
- Live sim and kill logs now expose lane data (`player_lane`, `KillLogV1.killer_position`, `killed_position`, and assist positions).
- The stable port now uses the classic `PokemonDraftScoreHook` through the stable adapter, preserving the old 103-Pokemon position/scoring behavior while running on `StableDraftContext`.

Practical result: position-aware draft/AI is now viable, but exact slot-forcing is still limited by what the stable draft hook receives. Guide/export metadata should continue to carry the user-facing position labels.

## Revisit Old Workarounds While Porting

Do not blindly carry old approximations forward. For each champion, check the guide text, `docs/project-reference.md`, and the move/passive implementation before porting.

Stable API features that should replace old compromises where the installed stable crate exposes the needed mutation:

- Shields: revisit every shield promise. The local 0.5.3 `BuffV1` exposes shield reads/metadata but not a direct shield amount field, so shield application may need data companion effects or a newer stable API surface.
- Real gold mutation: use stable player gold APIs for Gholdengo instead of internal player-state edits.
- Real teleports: use `entity_set_pos` only for moves documented as teleports/blinks, not ordinary dashes.
- Real summon/deployable units: consider `spawn_unit` for summons and match-hook state for deployables/fields.
- Better AI: use `StablePlayerAi` plus read-only `StableSim` for tower-aware dashes, copied Smeargle actions, Comfey/Clawitzer attachment behavior, and objective/tower positioning.
- Better kill inspection: use stable kill-log reads after damage/gold behavior is otherwise stable.

Known champions/mechanics to revisit:

- Banette: Phantom Force should be evaluated as a true teleport-to-target style move.
- Skarmory: Fly/global ally travel should be evaluated separately from normal dashes; do not convert ordinary movement to teleport unless the intended move is instant relocation.
- Audino: Substitute/ally rescue displacement should be evaluated for true swap/teleport semantics.
- Kricketune: Sticky Web has staged deployable VFX and should become a proper persistent web/deployable control zone.
- Octillery: Suction Cups was waiting on terrain/pathing support; stable `ignore_wall`/map/sim data may unlock a better version.
- Noivern: Infiltrator tower-aggro bypass was waiting on support; stable live tower/entity access and AI hooks may allow a partial or full implementation.
- Oranguru: Symbiosis should be revisited now that stable player/entity stat reads are cleaner.
- Ambipom: Technician should react to ally-provided Pokemon buffs through the buff ledger/stable buff reads.
- Ceruledge: Poltergeist should be revisited for item/stat-surplus behavior and VFX now that stable access is broader.
- Wishiwashi: Cowardice shield text should become a real shield instead of only metadata/description.

## Current Checkpoint

- `mod/pokemon_moba_stable` exists as a parallel package.
- Existing package assets, text, guide, icons, UI, sprites, and ingame replacements were copied.
- Asset paths in the stable package metadata now point at `asset/pokemon_moba_stable/...`.
- Staged VFX were copied into `mod/pokemon_moba_stable/vfx`.
- The stable package builds against installed SDK 0.5.3 through `tools/build-stable-native.ps1`.
- The classic gameplay layer is copied into `src/legacy` and compiled through `stable_legacy_api`, so the full native roster, Pokemon combat resolver, Pokemon status ledger, old draft scorer, and old player input AI are preserved as the migration baseline.
- All 103 runtime Pokemon are registered through the stable adapter.
- All 103 companion `.data_champion` files are generated from the Rust roster via `export_stable_data_champions`, with sprites, icons, stats, actions, and staged VFX declarations.
- Gholdengo's old unsafe live-gold mutation helpers now route through stable `player_add_gold`, preserving the old trigger logic while using the new SDK endpoint.
- Projectile VFX are declared in companion data. Traveling projectile visuals still need targeted runtime work to replace direct-hit paths with stable `spawn_projectile` calls and matching names.

The stable package is now a systematic full-roster port baseline, not the earlier champion-by-champion spike. Next migration passes should be narrowly scoped upgrades over the preserved legacy behavior: true teleports where documented, stable projectile spawning for projectile VFX, stable summon/deployable primitives, and selected AI fixes.
