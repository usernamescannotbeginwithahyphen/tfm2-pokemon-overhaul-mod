# Neutral Objectives And Creeps

Initial visual replacement pass:

| Base asset | Pokemon replacement | Notes |
| --- | --- | --- |
| `asset/base/aseprite_resources/ingame/epic` | `asset/pokemon_moba/ingame/xerneas` | Big epic objective. Replaces the previous Mew visual pass; gameplay logic remains tied to the base epic objective. |
| `asset/base/aseprite_resources/ingame/bee` | `asset/pokemon_moba/ingame/beedrill` | Neutral camp visual. |
| `asset/base/aseprite_resources/ingame/mushroom` | `asset/pokemon_moba/ingame/amoonguss` | Neutral camp visual. PMDCollab has no usable Amoonguss source, so this sheet is generated from imagegen using the prior in-mod Amoonguss as reference. |
| `asset/base/aseprite_resources/ingame/rhino` | `asset/pokemon_moba/ingame/rhyperior` | Neutral camp visual. |
| `asset/base/aseprite_resources/ingame/serpen` | `asset/pokemon_moba/ingame/eternatus` | Major objective. Keeps Serpen's permanent stat buff logic, renamed to Eternal Buff. |
| `asset/base/aseprite_resources/ingame/stump` | `asset/pokemon_moba/ingame/trevenant` | Neutral camp visual. |

These are visual overrides through `mod.override_info`. Each replacement targets the concrete `#sheet` and `#anim` assets because those are the assets present in the game bundle. Gameplay values are unchanged for now.

The epic objective is now presented as Xerneas in visuals and UI text. Gameplay values remain tied to the base epic objective unless the native objective/type hooks are changed separately.

Neutral/objective sprites are staged under `assets/custom_spritework/ingame` and synced into `mod/pokemon_moba/ingame` with the existing helper:

```powershell
.\tools\convert-neutral-sprites.ps1
```

The staged sheets are packed with center/bottom-anchored profiles. Current neutral sizing:

- Beedrill: 72x72 canvas, 34px max visible content.
- Amoonguss: 96x96 canvas, imagegen-created because PMDCollab lacks a usable Amoonguss sheet.
- Rhyperior and Trevenant: 112x112 canvas.
- Eternatus: 160x160 canvas so Serpen's pit reads as a major objective.
- Xerneas: 136x136 canvas, replacing the previous Mew epic-objective visual.

The June 25 sizing pass moved Beedrill, Amoonguss, Trevenant, Rhyperior, Eternatus, and Xerneas up 10 px inside their existing fixed canvases. This adds roughly 19-22 px of bottom padding so in-game healthbars no longer cut through the sprite bodies.

Ordinary jungle camps use a transparent or short `dead` animation tag. Their base-game healthbars and combat state despawn correctly, but visible idle-style dead frames can leave the Pokemon sprite on the map after death. Xerneas and Eternatus keep visible death frames because their objective lifecycle already clears correctly in game.

Lane minions use `asset/base/aseprite_resources/UI_aseprite/minion`. The mod still overrides its `#sheet` and `#anim` with `asset/pokemon_moba/ingame/falinks_minion`, generated from a single body cropped out of the Falinks train and scaled to a 44x44 canvas with 24px max visible content. The current jungle/objective pass intentionally leaves Falinks and lane minions untouched.

```powershell
.\tools\convert-minion-sprites.ps1
```

Preview contacts for the current neutral/objective pass live under `assets/custom_spritework/previews` as `beedrill_neutral_contact.png`, `amoonguss_neutral_contact.png`, `trevenant_neutral_contact.png`, `rhyperior_neutral_contact.png`, `eternatus_neutral_contact.png`, and `xerneas_neutral_contact.png`. Previous neutral runtime assets for Mew, Rayquaza, Onix, Rhydon, and Arceus form variants have been removed from `mod/pokemon_moba/ingame`.

## Towers, Nexus, And Map Asset Keys

Tower and nexus visuals are shared per team, not per individual lane/tier. The game exposes one blue tower asset and one red tower asset; all blue towers use the blue tower visual and all red towers use the red tower visual. `twin_tower` appears in game settings as a separate stat/objective category, but no separate twin-tower sprite key has been found. Treat twin towers as sharing the normal team tower art unless runtime testing proves otherwise.

Tower body assets:

| Base asset | Sheet size | Required animation tags |
| --- | --- | --- |
| `asset/base/aseprite_resources/ingame/blue_tower` | 581x64 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/red_tower` | 581x64 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/blue_tower_orb` | 357x64 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/red_tower_orb` | 357x64 | `idle`, `attack`, `attack_projectile`, `hit_effect` |

Nexus/core assets:

| Base asset | Sheet size | Required animation tags |
| --- | --- | --- |
| `asset/base/aseprite_resources/ingame/blue_nexus` | 836x81 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/red_nexus` | 836x81 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/blue_nexus_orb` | 526x81 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/red_nexus_orb` | 526x81 | `idle`, `attack`, `attack_projectile`, `hit_effect` |
| `asset/base/aseprite_resources/ingame/blue_nexus_destroy_effect` | 1348x184 | `destroy` |
| `asset/base/aseprite_resources/ingame/red_nexus_destroy_effect` | 1348x184 | `destroy` |

Tower projectile effect assets also exist at `asset/base/aseprite_resources/skill_effect/blue_tower_projectile` and `asset/base/aseprite_resources/skill_effect/red_tower_projectile`. They are tiny 4x4 PNG effect resources. Transparent replacements are live-remapped to `asset/pokemon_moba/ingame/red_tower_projectile` and `asset/pokemon_moba/ingame/blue_tower_projectile` so the base projectile does not double-render over the baked tower attack animation.

The 5v5 map is not a single editable image. It is a stack of static PNG layers and small shadow sprites. The visible art can be swapped, but gameplay placement/pathing/bush behavior is still controlled by the existing map settings and engine data, so replacement art must keep lanes, walls, bushes, towers, nexus positions, jungle camp reads, and playable boundaries aligned to the original layout.

5v5 map layer assets:

| Base asset | Size | Notes |
| --- | --- | --- |
| `asset/base/aseprite_resources/ingame/5v5/background_5v5` | 1280x1280 | Main arena background. |
| `asset/base/aseprite_resources/ingame/5v5/wall_5v5` | 1280x1280 | Wall/terrain layer. |
| `asset/base/aseprite_resources/ingame/5v5/wall_5v5_front` | 1280x1280 | Foreground wall/occlusion layer. |
| `asset/base/aseprite_resources/ingame/5v5/wall_shadow_5v5` | 1280x1280 | Wall shadow layer. |
| `asset/base/aseprite_resources/ingame/5v5/bush_5v5` | 1280x1280 | Visible bush layer. |
| `asset/base/aseprite_resources/ingame/5v5/bush_shadow_5v5` | 1280x1280 | Bush shadow layer. |
| `asset/base/aseprite_resources/ingame/5v5/tower_shadow` | 23x24 | Reused tower shadow sprite. |
| `asset/base/aseprite_resources/ingame/5v5/nexus_shadow` | 54x30 | Reused nexus shadow sprite. |
| `asset/base/aseprite_resources/ingame/5v5/minimap_5v5_bg` | 320x320 | Minimap background. |
| `asset/base/aseprite_resources/ingame/minimap_5v5` | 913x193 sheet plus `#data` | Minimap icon/sheet data; update only if the map/minimap style requires it. |

For tower, nexus, and animated objective swaps, override both `#sheet` and `#anim` entries in `mod.override_info`. For static map layer swaps, override the base PNG asset path directly, without `#sheet` or `#anim`.

The local SDK/docs do not expose a supported map geometry API for adding new permanent bushes, walls, collision, pathing, spawn points, or static interactables. Treat the 5v5 map assets above as visual layers unless a later SDK update exposes map-object registration. Custom Pokemon combat fields can still create gameplay at runtime through Rust champion logic, but drawing a new feature onto the static map will not make it functional by itself.

Pokemon map background work-in-progress:

- The current water/fire map files are reference and mask material only, not a live replacement.
- The current mask follows James's guide: red owns the upper/right triangle, blue owns the lower/left triangle, and a thin neutral diagonal strip covers the river and the Morgard/Serpen objective pits. Use this mask as ImageGen reference when generating the actual full-detail `background_5v5` replacement.
- Do not remap `asset/base/aseprite_resources/ingame/5v5/background_5v5` until a full generated background is accepted. Any final replacement must stay exactly 1280x1280 and preserve the base map's lane, tower pad, objective pit, jungle camp, wall, bush, and base geometry.

Live Pokemon tower replacement pass:

- Red-side tower body: Stakataka, remapped from `asset/base/aseprite_resources/ingame/red_tower` to `asset/pokemon_moba/ingame/red_tower`.
- Blue-side tower body: Celesteela, remapped from `asset/base/aseprite_resources/ingame/blue_tower` to `asset/pokemon_moba/ingame/blue_tower`.
- The live tower body assets keep the required `idle`, `attack`, `attack_projectile`, and `hit_effect` tags. They use larger custom frame rectangles with gutters rather than the base tower's tight 31x63 body frames, so the Pokemon towers can read as objective-scale structures without neighboring-frame bleed.
- The body animation carries the visible tower activation and apparent shot origin. The `attack_projectile` tag is still present on the body sheet for compatibility, and the separate tiny tower projectile resources are remapped as transparent placeholders so the renderer cannot show both the base projectile and the baked tower shot.
- The live `red_tower_orb` and `blue_tower_orb` assets are intentionally transparent. The orb layer appears to be a separate renderer component; blanking it avoids a floating detached overlay while keeping the expected animation tags available. If in-game tower shots need the orb layer as their visible origin, replace the transparent orb with a small aligned eye/core/cannon-mouth sprite instead of a second full tower.
- No separate tower destroy animation asset has been found. Tower body/orb sheets expose `idle`, `attack`, `attack_projectile`, and `hit_effect`; only nexus/core exposes explicit `*_nexus_destroy_effect` assets with a `destroy` tag. Do not generate Stakataka/Celesteela crumble animations unless runtime testing or further asset extraction reveals a callable tower destroy key.

Live Pokemon nexus replacement pass:

- Blue-side nexus body: Kyogre, remapped from `asset/base/aseprite_resources/ingame/blue_nexus` to `asset/pokemon_moba/ingame/blue_nexus`.
- Red-side nexus body: Groudon, remapped from `asset/base/aseprite_resources/ingame/red_nexus` to `asset/pokemon_moba/ingame/red_nexus`.
- The June 25 follow-up pass reduced both live nexus body sheets by an additional 6% inside their existing 220x150 frames so Kyogre/Groudon overlap the forward towers less.
- Destroy effects are remapped separately as `blue_nexus_destroy_effect#sheet.png` / `#anim.fanim` and `red_nexus_destroy_effect#sheet.png` / `#anim.fanim`, each with a `destroy` tag.
- The live `blue_nexus_orb` and `red_nexus_orb` assets are intentionally transparent, matching the tower strategy. The main nexus body sheet carries the visible idle/attack read, while the `attack_projectile` and `hit_effect` tags remain present as transparent placeholders for compatibility.
- QC previews live under `assets/custom_spritework/previews` as `blue_nexus_contact.png`, `red_nexus_contact.png`, `*_nexus_attack_preview.gif`, `*_nexus_destroy_preview.gif`, and `*_nexus_cell_debug.png`.
