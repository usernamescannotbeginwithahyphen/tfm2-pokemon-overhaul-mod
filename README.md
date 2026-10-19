# Teamfight Manager 2 Pokemon MOBA Mod

Workspace for a full Teamfight Manager 2 overhaul mod using `pokemon_moba_stable` as the package id and stable SDK 0.6.4.

## Layout

- `mod/pokemon_moba_stable/` is the actual mod package that can be copied into the game's `mods/` folder.
- `templates/` contains authoring templates that should not be copied into the active mod package.
- `schemas/` contains lightweight JSON schemas for editor validation.
- `tools/` contains local authoring and deployment scripts.
- `data/` is for planning/source-of-truth files before content is generated into the mod package.
- `docs/` contains project notes derived from the developer modding docs.

## Workflow

Read `docs/project-reference.md` first when resuming work, especially after context compaction.

1. Add or update native champion code under `mod/pokemon_moba_stable/src/legacy/pokemon_content.rs`.
2. Add or update custom status/persistent effect code under `mod/pokemon_moba_stable/src/legacy/pokemon_status.rs`.
3. Stage supplied custom sprites in `assets/custom_spritework/champions/`, then sync them into `mod/pokemon_moba_stable/champions_custom/` with `tools/sync-custom-champion-sprites.py`.
4. Run `tools/validate-json.ps1` if JSON assets changed.
5. Run `tools/build-native.ps1` after Rust/native code changes.
6. Run `tools/deploy.ps1` to copy the mod into the installed game's `mods/pokemon_moba_stable` folder.

Workshop publication is separate from deployment. Pass `-WorkshopId 3744254195` explicitly to the deploy script when preparing the existing Workshop item.

## Legacy Data-Only Champion Stub

This is kept for reference only. Current Pokemon roster work uses native Rust champion entries, not `.data_champion` stubs.

```powershell
.\tools\new-champion.ps1 -ShortId pikachu -Name "Pikachu" -Category Range -Tags AP,Range
```

This creates a loadable `.data_champion` file with placeholder stats/effects and inserts matching English i18n keys.
