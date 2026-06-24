# Asset Pipeline

Pokemon champion sprites now use the custom spritework pipeline. Do not use the old Pokemon Showdown/Smogon animated GIF conversion path for roster sprites.

## Champion Sprites

Source and QC assets live outside the runtime mod package:

```text
assets/custom_spritework/champions/<short-id>#sheet.png
assets/custom_spritework/champions/<short-id>#anim.fanim
assets/custom_spritework/previews/<short-id>_<animation>_preview.gif
assets/custom_spritework/references/...
assets/custom_spritework/vfx/...
```

Runtime assets live here:

```text
mod/pokemon_moba/champions_custom/<short-id>#sheet.png
mod/pokemon_moba/champions_custom/<short-id>#anim.fanim
```

`mod/pokemon_moba/mod.override_info` maps base champion sprite keys to the runtime custom assets:

```text
asset/base/aseprite_resources/champions/pokemon_moba_<short-id>#sheet
  -> asset/pokemon_moba/champions_custom/<short-id>#sheet
asset/base/aseprite_resources/champions/pokemon_moba_<short-id>#anim
  -> asset/pokemon_moba/champions_custom/<short-id>#anim
```

After new champion sprite assets are supplied, run:

```powershell
.\.venv\Scripts\python.exe .\tools\sync-custom-champion-sprites.py
```

That script audits the Rust roster, requires a staged `#sheet.png` and `#anim.fanim` for every Pokemon champion, copies staged files into `champions_custom`, and normalizes the override mappings.

## Sprite Creation

Use `docs/custom-spritework-workflow.md` for the detailed per-Pokemon process. In short:

- Prefer PMDCollab source animations when available.
- Use ImageGen only for missing actions, sprite-attached aura/readability states, or future VFX assets.
- Keep champion action readability left-to-right.
- Use transparent GIF previews as the QC source of truth before in-game testing.
- Keep future-ready detached projectile, beam, terrain, and area VFX under `assets/custom_spritework/vfx`; current native champions will ignore custom VFX unless a Rust call path is implemented for that effect.

## Deprecated Showdown Path

The former workflow converted animated GIFs from `assets/source/showdown/ani` into `mod/pokemon_moba/champions`. That output has been replaced by `champions_custom`, and the old Smogon/Showdown sprites are no longer part of the live Pokemon champion asset pipeline.

Do not run `tools/convert-pokemon-sprite.ps1` for Pokemon champion roster work.
