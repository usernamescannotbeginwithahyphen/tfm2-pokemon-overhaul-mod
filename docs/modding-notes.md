# Teamfight Manager 2 Modding Notes

Source docs: https://github.com/teamsamoyed/TeamfightManager2Mod/tree/main

## Package Rules

- A mod is a folder under the game's `mods/` directory.
- The folder name is the mod id. This project uses `pokemon_moba`.
- Only `mod.mod_info` is required for the mod to appear in the Mods menu.
- Files inside the mod are referenced as `asset/<mod_id>/<relative_path_without_extension>`.
- Example: `mods/pokemon_moba/icons/skills/pikachu_skill.png` is referenced as `asset/pokemon_moba/icons/skills/pikachu_skill`.

## Champion Rules

- Data-only champions are JSON files ending in `.data_champion`.
- Required fields are `id`, `attack`, `skill`, and `skill2`; `ult` is optional but we will include it for MOBA-style kits.
- Use stable ids like `pokemon_moba_pikachu`. Saves and patches may refer to these ids.
- Valid categories are `Melee`, `Range`, `Magician`, `Util`, and `Assassin`.
- Useful tags are `AD`, `AP`, `Heal`, `Shield`, `Dot`, `CC`, `Range`, `Melee`, `Tank`, and `Magic`.
- Explicitly set `casting_target`, `applied_target`, and `target` for offensive effects because Rust defaults often point at allies.

## Text And i18n

- Native champion names are returned directly from the DLL.
- Champion action descriptions should reference mod-owned text directly, such as `#asset/pokemon_moba/text/champion?description.pokemon_moba_pikachu.skill`.
- Do not merge text into `asset/base/text/champion`, `asset/base/text/object`, or `asset/base/text/ui` for now. The current installed build logs those base text assets as non-mergeable and reports them as failed overrides.

## Asset Rules

- Direct skill icons use three extensionless paths in `skill_icons`: skill, skill2, ult.
- Static PNG champion sprites can be used while testing and do not need `anim_prefix`.
- Animated Aseprite or manual animation sheets should use a shared base path and set `anim_prefix`, usually `""` when tags already match `idle`, `run`, `attack`, `skill`, `skill2`, `ult`, and `dead`.
- Aseprite user data must declare `sheet_type` for the game to create `#sheet`, `#anim`, or `#data` assets.

## Native Code Boundary

Start data-only. Move to native Rust only for behaviors the documented JSON effect list cannot express, such as bespoke simulation logic, custom item callbacks, save hooks, AI hooks, or runtime services.
