# Neutral Objectives And Creeps

Initial visual replacement pass:

| Base asset | Pokemon replacement | Notes |
| --- | --- | --- |
| `asset/base/aseprite_resources/ingame/epic` | `asset/pokemon_moba/ingame/mew` | Big epic objective. Keeps Morgard's minion buff logic, renamed to Mew. |
| `asset/base/aseprite_resources/ingame/bee` | `asset/pokemon_moba/ingame/beedrill` | Neutral camp visual. |
| `asset/base/aseprite_resources/ingame/mushroom` | `asset/pokemon_moba/ingame/amoonguss` | Neutral camp visual. |
| `asset/base/aseprite_resources/ingame/rhino` | `asset/pokemon_moba/ingame/rhyperior` | Neutral camp visual. |
| `asset/base/aseprite_resources/ingame/serpen` | `asset/pokemon_moba/ingame/eternatus` | Major objective. Keeps Serpen's permanent stat buff logic, renamed to Eternal Buff. |
| `asset/base/aseprite_resources/ingame/stump` | `asset/pokemon_moba/ingame/trevenant` | Neutral camp visual. |

These are visual overrides through `mod.override_info`. Each replacement targets the concrete `#sheet` and `#anim` assets because those are the assets present in the game bundle. Gameplay values are unchanged for now.

Native Pokemon damage code recognizes the epic objective by its base-game stats and treats Mew as a Psychic-type defender for type-effectiveness calculations. That applies to damage routed through our native Pokemon move framework; it does not affect base-game champion damage, item damage, or Mew's outgoing attacks.

Neutral sprites are converted with center-anchored profiles. Some camps use smaller per-Pokemon sizing so multi-unit camps do not become visually crowded:

```powershell
.\tools\convert-neutral-sprites.ps1
```

Current neutral sizing:

- Mew: 72x72 canvas, 38px max visible content. The epic boss renderer uses separate attack/effect tags, so Mew's `attack_left`, `attack_right`, and `attack_target_effect` tags are blanked to prevent multiple full Mew bodies from rendering at once.
- Beedrill: 72x72 canvas, 34px max visible content.
- Amoonguss: 80x80 canvas, 44px max visible content.
- Rhyperior and Trevenant: 96x96 canvas, 58px max visible content.
- Eternatus: 128x128 canvas, 96px max visible content so Serpen's pit reads as a major objective.

Ordinary jungle camps use a transparent `dead` animation tag. Their base-game healthbars and combat state despawn correctly, but visible idle-style dead frames can leave the Pokemon sprite on the map after death. Mew and Eternatus keep visible death frames because their objective lifecycle already clears correctly in game.

Lane minions use `asset/base/aseprite_resources/UI_aseprite/minion`. The mod overrides its `#sheet` and `#anim` with `asset/pokemon_moba/ingame/falinks_minion`, generated from a single body cropped out of the Falinks train and scaled to a 44x44 canvas with 24px max visible content. The base game still controls wave timing, minion count, health bars, melee/ranged roles, projectiles, and Mew-buffed variants.

```powershell
.\tools\convert-minion-sprites.ps1
```

Towers and nexus/core are also overrideable as asset work, but they are intentionally left alone until the neutral replacements are confirmed.
