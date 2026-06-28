# Custom Pokemon Spritework Workflow

Use this for Pokemon champion sprite work. The workflow is PMDCollab-source-first: use existing Pokemon Mystery Dungeon-style animation sheets when they cover a move, then add small matching sprite-attached cues only where a Pokemon's kit needs extra readability.

## Storage Rules

- Stage converted sheets and animation files under `assets\custom_spritework\champions`.
- Runtime copies live under `mod\pokemon_moba\champions_custom`; do not put new Pokemon champion sprites back into `mod\pokemon_moba\champions`.
- `mod.override_info` should map Pokemon champion `#sheet` and `#anim` assets to `asset/pokemon_moba/champions_custom/<short-id>#sheet` and `#anim`.
- Current new-Pokemon flow: new Pokemon are expected to already have `mod.override_info` mappings or to receive them when `tools\sync-custom-champion-sprites.py` runs. After making a new sprite group, generate previews and sync immediately into `mod\pokemon_moba\champions_custom`; do not wait for a separate staging/remap batch unless James asks for one.
- Stage future-ready custom projectile/spell VFX under `assets\custom_spritework\vfx`.
- Stage QC GIFs under `assets\custom_spritework\previews`.
- Store downloaded PMDCollab source material and credits under `assets\custom_spritework\references\pmdcollab\<species-id>`.
- Store ImageGen source sheets under `assets\custom_spritework\references\imagegen`.
- Preserve `credits.txt` with the downloaded source set for every Pokemon.

Expected staged files:

```text
assets\custom_spritework\champions\ambipom#sheet.png
assets\custom_spritework\champions\ambipom#anim.fanim
assets\custom_spritework\previews\ambipom_idle_preview.gif
assets\custom_spritework\previews\ambipom_run_preview.gif
assets\custom_spritework\previews\ambipom_basicattack_preview.gif
assets\custom_spritework\previews\ambipom_skill1_preview.gif
assets\custom_spritework\previews\ambipom_skill2_preview.gif
assets\custom_spritework\previews\ambipom_ult_preview.gif
assets\custom_spritework\previews\ambipom_size_compare.png
```

Optional passive/buff previews may be added when they help QC. Do not make them by default when the passive has no important persistent visual state.

## Per-Pokemon Order

Work alphabetically through the roster, starting with Ambipom.

For each Pokemon:

1. Read an existing comparable live sprite and `.fanim` from `mod\pokemon_moba\champions_custom` to preserve approximate gameplay scale, canvas size, and required animation tags.
2. Read the Pokemon's kit in `mod\pokemon_moba\src\pokemon_content.rs` and the summary in `docs\project-reference.md`.
3. Download/cache the PMDCollab source folder for the Pokemon under `assets\custom_spritework\references\pmdcollab\<species-id>`, including `AnimData.xml`, selected `*-Anim.png` files, and `credits.txt`.
4. Choose source animations for the required TFM2 tags: `idle`, `run`, `attack`, `skill`, `skill2`, `ult`, and `dead`.
5. Convert the selected PMD animation row into a staged fixed-canvas TFM2 `#sheet.png` and `#anim.fanim` with `tools\convert-pmdcollab-sprite.py`. Use right-facing source rows as the canonical staged direction, but prefer three-quarter right-facing rows for Pokemon idle/portrait readability. PMD's usual second visible row is the preferred right-facing three-quarter view (`--direction-row 1` because the converter is zero-based), the third visible row is pure right side-view (`--direction-row 2`), and the seventh visible row is pure left side-view (`--direction-row 6`).
   Some PMDCollab actions only ship one direction row; the converter clamps those actions to their available row instead of producing blank frames.
   If an action has the right source pose but too much PMDCollab body travel, use `--travel-scale tag=value` to reduce horizontal travel without fully recentering the animation.
6. All staged action readability should face left-to-right. If ImageGen creates projectiles, beams, travel effects, tail stabs, or other directional effects facing the wrong way, mirror the extracted frames before packing them into final sheets.
7. Prefer source-frame reuse or small deterministic sprite-attached overlays for missing move readability so the Pokemon does not morph between actions. Use ImageGen only for genuinely missing art that cannot be built from the source frames, and keep the generated result matched to the downloaded source style.
   If a loose generated effect looks pasted on or visibly clashes in the preview GIF, remove it from the champion sheet. For that case, either leave the PMDCollab body clean or rebuild the full affected action as an intentional ImageGen animation strip where the body and effect are generated together in one consistent style.
   When an action needs a longer bridge into another pose, use roughly four prep/transition frames as the default starting point. Four frames has been enough to make abrupt pose changes, such as standing into a low shooting pose, read as intentional without overextending the move.
8. Detached projectile/spell-area VFX can be designed and staged now, but do not rely on them for current in-game readability until the native Rust champion VFX call path exists. Keep those files under `assets\custom_spritework\vfx` and preview them separately.
9. Generate looping preview GIFs for idle/run/basic/skill1/skill2/ult and any passive or buff state that needs visual QC. Use transparent GIF backgrounds (`--background 0,0,0,0`) rather than chroma-key green so the previews match the alpha behavior expected in-game. Use 288x288 previews for standard 96x96 champion frames so every Pokemon is reviewed at the same 3x scale.
10. Generate an idle size comparison PNG for every new Pokemon before accepting scale. Use `tools\render-sprite-size-comparison.py <short-id>` so the new Pokemon appears beside the current reference spread: Shedinja, Electrode, Gallade, Venusaur, and Snorlax. The output is `assets\custom_spritework\previews\<short-id>_size_compare.png` and includes visible height, bottom anchor, and a shared ground line.
11. Verify the staged `.fanim` JSON parses, every referenced rectangle fits inside the staged sheet, preview GIFs play without neighboring-frame bleed, and the size comparison does not show the new Pokemon as an obvious scale outlier.
12. Treat the preview GIFs and size comparison as the framing check before in-game testing. Existing live Showdown-converted sprites usually put idle bottoms around y=66-70 inside the 96x96 frame, but the current custom roster has accepted exceptions by size bucket. If the Pokemon sits too low in comparison, raise it by increasing `--bottom-padding` or applying a small negative `--shift-y`; if it sits too high, lower it. Do not align to the full transparent PMD source cell.
13. When a Pokemon's staged files are accepted, run `.\.venv\Scripts\python.exe .\tools\sync-custom-champion-sprites.py` to copy staged assets into `mod\pokemon_moba\champions_custom` and normalize `mod.override_info`.

## Size QA rules

Use `assets\custom_spritework\size_audit\champion_size_audit_full_pass.png` as the current roster comparison sheet. Generate a fresh sheet after any broad sizing pass so Pokemon are judged against each other, not in isolation.

Champion size buckets are visual guidelines, measured by the first idle frame's visible height inside the fixed 96x96 frame:

- Extra Small: about 28-30 px idle height for tiny Pokemon such as Shedinja, Comfey, Porygon-Z, Pikachu, Ribombee, Dedenne, and Kilowattrel.
- Small: about 32-34 px idle height for compact Pokemon such as Eevee-line bodies, Delibird, Electrode, Clawitzer, Octillery, Orbeetle, and small flyers.
- Normal: about 36-40 px idle height for humanoid or mid-sized Pokemon such as Hitmonlee, Smeargle, Gallade, Greninja, Scizor, Zeraora, and Starmie.
- Large: about 40-44 px idle height for bulky or tall Pokemon such as Drednaw, Kleavor, Decidueye, Dragalge, Magmortar, and Skarmory.
- Extra Large: about 46-50 px idle height for deliberately huge silhouettes such as Snorlax, Torterra, Gyarados, Turtonator, Venusaur, Feraligatr, Goodra, and Ursaluna.
- Extra Large Pair: about 50 px idle height for paired bodies such as Sawk/Throh; judge width separately because two characters share the frame.

Do not enlarge already accepted Pokemon as part of a size pass; enlargement blurs source pixels and makes the roster drift upward in scale. Only downscale clear outliers, preserving the frame rectangle and bottom anchor unless the specific issue is vertical placement.

Use `tools\scale-sprite-sheet-content.py` for conservative downscales or tiny placement nudges. It scales visible pixels inside existing `.fanim` rectangles and leaves frame coordinates unchanged.

The full roster size plan lives at `assets\custom_spritework\size_audit\champion_size_plan.md`. It records every Pokemon's bucket, target idle height, original visible size for the pass, scale factor, and vertical shift.

Ambipom conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0424 `
  --source-dir assets\custom_spritework\references\pmdcollab\0424 `
  --output-base assets\custom_spritework\champions\ambipom `
  --effect skill=tickle `
  --effect skill2=thief `
  --effect ult=ult_aura `
  --max-content-size 38 `
  --bottom-padding 28
```

Preview command after a staged sheet exists:

```powershell
.\.venv\Scripts\python.exe .\tools\render-sprite-previews.py assets\custom_spritework\champions\ambipom --background 0,0,0,0
```

The renderer writes to `assets\custom_spritework\previews` by default.

Size comparison command after the standard preview GIFs exist:

```powershell
.\.venv\Scripts\python.exe .\tools\render-sprite-size-comparison.py ambipom
```

The comparison renderer writes `assets\custom_spritework\previews\<short-id>_size_compare.png` by default. Override references with `--compare shedinja electrode gallade venusaur snorlax` if a specific Pokemon needs a more targeted comparison set.

## Animation Naming

Use game-facing preview names that match the Pokemon kit:

- `basicattack` previews the `.fanim` `attack` tag.
- `skill1` previews the `.fanim` `skill` tag.
- `skill2` previews the `.fanim` `skill2` tag.
- `ult` previews the `.fanim` `ult` tag.

Keep `.fanim` tags compatible with the existing mod path unless a Pokemon genuinely needs extras:

```text
idle
run
attack
skill
skill2
ult
dead
```

## Ambipom Source Mapping

- PMDCollab source: `sprite/0424`, from `https://sprites.pmdcollab.org/#/0424?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Double Hit basic: `MultiStrike`
- `skill` / Tickle: stationary `Idle` base with animated hand-tail tickle cues
- `skill2` / Thief: `Attack` with small dark pluck/glint cues
- `ult` / Fury Swipes steroid: `RearUp` with a small yellow sprite-attached power aura
- `dead`: `Hurt`
- Technician does not need a separate Ambipom preview.

Run animations must show grounded leg motion. Do not include dust puffs, dust trails, ground clouds, or other footstep dust in run animations.

## Appletun Source Mapping

- PMDCollab source: `sprite/0842`, from `https://sprites.pmdcollab.org/#/0842?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Headbutt basic: `Shoot`
- `skill` / Apple Acid: `Shoot`; custom Apple Acid projectile is staged separately as `assets\custom_spritework\vfx\appletun_skill1_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Sweet Scent: `Charge` with a pink/green sprite-attached sweet-scent aura.
- `ult` / Fickle Beam: `Shoot`; custom Fickle Beam projectile is staged separately as `assets\custom_spritework\vfx\appletun_ult_projectile#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Ripen does not need a separate passive preview.

Appletun conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0842 `
  --source-dir assets\custom_spritework\references\pmdcollab\0842 `
  --output-base assets\custom_spritework\champions\appletun `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Charge `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --effect skill2=sweet_scent_aura `
  --max-content-size 46 `
  --bottom-padding 28
```

Appletun's ImageGen projectile sources are mirrored during VFX packing with `tools\pack-imagegen-vfx.py --flip-x` so the packed projectiles follow the mod convention of left-to-right action readability.

## Arbok Source Mapping

- PMDCollab source: `sprite/0024`, from `https://sprites.pmdcollab.org/#/0024?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Poison Sting basic: ImageGen-generated upright tail-stab strip, packed from `assets\custom_spritework\references\imagegen\arbok_tail_stab_basic_source.png` with border-flood chroma removal so the purple body stays opaque, then appended as the final `attack` tag.
- `skill` / Coil dash: `Attack`, duration-scaled to Arbok's 52-tick skill duration.
- `skill2` / Spit Up: `Shoot`; custom poison splash projectile is staged separately as `assets\custom_spritework\vfx\arbok_skill2_projectile#sheet.png` and `#anim.fanim`. The projectile must read left-to-right, with its trail behind on the left and a final splash/radius frame because the Rust move is `AreaMiasmaSlow { radius: 34000, ... }`.
- `ult` / Venoshock: `Charge` with a small purple/green poison cast ring and motes.
- `dead`: `Hurt+Sleep`
- Dripping Fangs does not need a separate passive preview.

Arbok conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0024 `
  --source-dir assets\custom_spritework\references\pmdcollab\0024 `
  --output-base assets\custom_spritework\champions\arbok `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Idle `
  --map skill=Attack `
  --map skill2=Shoot `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --effect ult=venoshock_cast `
  --tag-duration attack=0.4333 `
  --tag-duration skill=0.8667 `
  --tag-duration skill2=0.6000 `
  --tag-duration ult=0.7333 `
  --max-content-size 40 `
  --bottom-padding 28
.\.venv\Scripts\python.exe .\tools\replace-fanim-tag.py `
  --target-base assets\custom_spritework\champions\arbok `
  --source-base assets\custom_spritework\temp\arbok_basic_tail_stab `
  --tag attack
```

## Arboliva Source Mapping

- PMDCollab source: `sprite/0930`, from `https://sprites.pmdcollab.org/#/0930?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Seed Bomb basic body cast: `Eat`; the custom seed bomb projectile with final splash is staged separately as `assets\custom_spritework\vfx\arboliva_basic_seed_bomb#sheet.png` and `#anim.fanim`.
- `skill` / Energy Ball: selected/recentered early `Shoot` frames so Arboliva does not hop back and forth too much; custom energy ball projectile is staged separately as `assets\custom_spritework\vfx\arboliva_skill1_energy_ball#sheet.png` and `#anim.fanim`.
- `skill2` / Trailblaze: `Rotate`, which is the actual PMDCollab source animation copied by the `Twirl` alias. The fiery grassy terrain VFX is staged as `assets\custom_spritework\vfx\arboliva_skill2_fiery_terrain#sheet.png` and `#anim.fanim`.
- `ult` / Solar Beam: `Charge`; custom line beam VFX is staged separately as `assets\custom_spritework\vfx\arboliva_ult_solar_beam#sheet.png` and `#anim.fanim`.
- Passive / Seed Sower: persistent grassy terrain VFX is staged as `assets\custom_spritework\vfx\arboliva_passive_grassy_terrain#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`

Arboliva conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0930 `
  --source-dir assets\custom_spritework\references\pmdcollab\0930 `
  --output-base assets\custom_spritework\champions\arboliva `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Eat `
  --map skill=Shoot `
  --map skill2=Rotate `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5667 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=1.1333 `
  --tag-duration dead=1.0 `
  --frame-select skill=0-7 `
  --recenter-tag skill `
  --max-content-size 42 `
  --bottom-padding 28
```

## Archaludon Source Mapping

- PMDCollab source: `sprite/1018`, from `https://sprites.pmdcollab.org/#/1018?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Breaking Swipe basic: `RearUp`
- `skill` / Iron Defense: `Charge`
- `skill2` / Autotomize: `Double`, with `--travel-scale skill2=0.45` so it keeps a small back-and-forth motion without the full source slide.
- `ult` / Electro Shot: `Shoot`, with four PMDCollab-source-derived bend/prep frames inserted before the Shoot channel to reduce the abrupt idle/walk-to-shoot snap. ImageGen-created electric self-area aura is staged separately as `assets\custom_spritework\vfx\archaludon_ult_electro_aura#sheet.png` and `#anim.fanim`, and is also composited into the staged `ult` tag so the current native champion path still shows the damage aura.
- `dead`: `Hurt+Sleep`
- Stalwart does not need a separate passive preview.

Archaludon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 1018 `
  --source-dir assets\custom_spritework\references\pmdcollab\1018 `
  --output-base assets\custom_spritework\champions\archaludon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=RearUp `
  --map skill=Charge `
  --map skill2=Double `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5333 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=3.0 `
  --tag-duration dead=1.0 `
  --travel-scale skill2=0.45 `
  --max-content-size 44 `
  --bottom-padding 28
```

## Armarouge Source Mapping

- PMDCollab source: `sprite/0936`, from `https://sprites.pmdcollab.org/#/0936?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Fire Lash basic: `Strike`, with `--travel-scale attack=0.15` and `--recenter-tag attack` so it reads as arm swinging rather than a forward lunge or right-shifted body.
- `skill` / Mystical Fire: `Rotate`, with ImageGen-created fire self-area aura staged separately as `assets\custom_spritework\vfx\armarouge_skill1_fire_aura#sheet.png` and `#anim.fanim`, and composited into the staged `skill` tag for current in-game readability.
- `skill2` / Heart Stamp: `Attack`, with `--travel-scale skill2=0.35` so the hit pose remains but the source travel is reduced.
- `ult` / Armor Cannon: `Shoot`; custom left-to-right fire line VFX is staged separately as `assets\custom_spritework\vfx\armarouge_ult_fire_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Weak Armor does not need a separate passive preview.

Armarouge conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0936 `
  --source-dir assets\custom_spritework\references\pmdcollab\0936 `
  --output-base assets\custom_spritework\champions\armarouge `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Rotate `
  --map skill2=Attack `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5333 `
  --tag-duration skill=0.7333 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=1.0333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.15 `
  --travel-scale skill2=0.35 `
  --recenter-tag attack `
  --max-content-size 44 `
  --bottom-padding 28
```

## Audino Source Mapping

- PMDCollab source: `sprite/0531`, from `https://sprites.pmdcollab.org/#/0531?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Pound basic: `Attack`, with `--travel-scale attack=0.25` and `--recenter-tag attack` so the melee hit stays close and centered.
- `skill` / Protect: `Charge`
- `skill2` / Substitute ally dash: `Slam`, preserving its dash read for the ally swap/heal.
- `ult` / Endure: `Withdraw`
- `dead`: `Hurt+Sleep`
- Regenerator does not need a separate passive preview.
- No ImageGen assets are needed for Audino.

Audino conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0531 `
  --source-dir assets\custom_spritework\references\pmdcollab\0531 `
  --output-base assets\custom_spritework\champions\audino `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Charge `
  --map skill2=Slam `
  --map ult=Withdraw `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4333 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=0.5333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.25 `
  --recenter-tag attack `
  --max-content-size 42 `
  --bottom-padding 28
```

## Banette Source Mapping

- PMDCollab source: `sprite/0354`, from `https://sprites.pmdcollab.org/#/0354?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Shadow Claw basic: `Strike`
- `skill` / Shadow Sneak: `Attack`, preserving the source travel over the skill distance. ImageGen-created shadow smoke is staged separately as `assets\custom_spritework\vfx\banette_skill1_shadow_smoke#sheet.png` and `#anim.fanim`, and composited into the staged `skill` tag for current in-game readability.
- `skill2` / Night Shade: `SpAttack`; custom left-to-right cone VFX is staged separately as `assets\custom_spritework\vfx\banette_skill2_night_shade_cone#sheet.png` and `#anim.fanim`.
- `ult` / Phantom Force: `Hop`, with its horizontal travel shifted to match the `Attack` / Shadow Sneak travel envelope. ImageGen-created phantom/splash VFX is staged separately as `assets\custom_spritework\vfx\banette_ult_phantom_splash#sheet.png` and `#anim.fanim`, and the phantom trail is composited into the staged `ult` tag. The later splash frames represent the conditional splash-on-kill visual.
- `dead`: `Hurt+Sleep`
- Cursed Body does not need a separate passive preview.

Banette conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0354 `
  --source-dir assets\custom_spritework\references\pmdcollab\0354 `
  --output-base assets\custom_spritework\champions\banette `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Attack `
  --map skill2=SpAttack `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --max-content-size 42 `
  --bottom-padding 28
```

After conversion, reapply the ImageGen smoke/phantom overlays and the custom ult travel shift from the staged helper logic used for Banette.

## Beeheeyem Source Mapping

- PMDCollab source: `sprite/0606`, from `https://sprites.pmdcollab.org/#/0606?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Guard Split basic: `SpAttack`, using only frames `0,5-12,13` so the animation keeps the right-facing eye/hand twinkle without the full source flourish. Custom left-to-right psychic projectile VFX is staged separately as `assets\custom_spritework\vfx\beeheeyem_basic_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Imprison: `Charge`
- `skill2` / Light Screen: `Double`; custom placed Light Screen VFX is staged separately as `assets\custom_spritework\vfx\beeheeyem_skill2_light_screen#sheet.png` and `#anim.fanim`.
- `ult` / Zen Headbutt: `Attack`, with a post-pass that shifts the hit frames forward and composites the ImageGen-created psychic impact around the head/front so it reads as a headbutt rather than a hand slap. The standalone impact VFX is staged as `assets\custom_spritework\vfx\beeheeyem_ult_headbutt_effect#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Magician does not need a separate passive preview.

Beeheeyem conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0606 `
  --source-dir assets\custom_spritework\references\pmdcollab\0606 `
  --output-base assets\custom_spritework\champions\beeheeyem `
  --map idle=Idle `
  --map run=Walk `
  --map attack=SpAttack `
  --map skill=Charge `
  --map skill2=Double `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4667 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.4 `
  --tag-duration ult=0.8 `
  --tag-duration dead=1.0 `
  --frame-select attack=0,5-12,13 `
  --recenter-tag attack `
  --max-content-size 42 `
  --bottom-padding 28
```

After conversion, reapply the Beeheeyem ult post-pass and VFX chroma cleanup used in this staged build. ImageGen VFX sources are preserved under `assets\custom_spritework\references\imagegen`.

## Blastoise Source Mapping

- PMDCollab source: `sprite/0009`, from `https://sprites.pmdcollab.org/#/0009?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Hydro Pump basic: `Shoot`, with `--recenter-tag attack` so the small forward lunge is minimized and the action reads as cannon fire. Hydro Pump has two independent hits, so stage two separate left-to-right basic projectile VFX assets: `assets\custom_spritework\vfx\blastoise_basic_projectile_1#sheet.png` / `#anim.fanim` and `assets\custom_spritework\vfx\blastoise_basic_projectile_2#sheet.png` / `#anim.fanim`.
- `skill` / Rapid Spin: `Ricochet`, with `--recenter-tag skill` so Blastoise withdraws/spins in place instead of traveling across the frame.
- `skill2` / Aqua Tail: `Swing`, trimmed to frames `0,1,2,1,0` with `--recenter-tag skill2` so it reads as a short tail arc into the target instead of a full circular spin.
- `ult` / Hydro Cannon: use the PMDCollab-style ImageGen strip saved at `assets\custom_spritework\references\imagegen\blastoise_ult_pmdstyle_source.png` as the cannon reference, but keep Blastoise's actual PMDCollab `Idle` body as the stable base layer for all eight `ult` frames. The body must not shrink or grow to accommodate the cannon; only the baked-in cannon layer should extend upward/right, fire, and retract. The Mega Blastoise-style back cannon must be baked into the full 96x96 champion frames and attached behind the shell/head silhouette; do not rebuild this from a separate floating runtime overlay. The older `blastoise_ult_cannon` overlay asset and the first oversized integrated strip are reference-only and should not be mapped as the final `ult`. The piercing Hydro Cannon line VFX is staged separately as `assets\custom_spritework\vfx\blastoise_ult_hydro_cannon_line#sheet.png` and `#anim.fanim`.
- `passive_idle` / Mega Launcher idle: copied from `Idle`, with ImageGen reference art used to guide a subtle red-orange tint on the actual visible shoulder cannon rim pixels only.
- `passive_run` / Mega Launcher run: copied from `Walk`, with the same cannon-rim tint following the run frames. Also keep `passive` as an alias of `passive_idle` for simple preview naming.
- `dead`: `Hurt+Sleep`

Blastoise conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0009 `
  --source-dir assets\custom_spritework\references\pmdcollab\0009 `
  --output-base assets\custom_spritework\champions\blastoise `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Ricochet `
  --map skill2=Swing `
  --map ult=Idle `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4333 `
  --tag-duration skill=1.5 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=0.9667 `
  --tag-duration dead=1.0 `
  --frame-select skill2=0,1,2,1,0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --max-content-size 42 `
  --bottom-padding 28
```

After conversion, reapply the Blastoise PMDCollab-style integrated-ult source-strip post-pass from `blastoise_ult_pmdstyle_source.png`, append `passive_idle` and `passive_run` without resizing existing atlas regions, keep `passive` as a `passive_idle` alias, and run chroma-fringe cleanup on the water VFX and integrated ult frames.

## Blaziken Source Mapping

- PMDCollab source: `sprite/0257`, from `https://sprites.pmdcollab.org/#/0257?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Double Kick basic: `Kick`.
- `skill` / Blaze Kick: `Shoot`.
- `skill2` / Bulk Up: `Charge`.
- `ult` / Flare Blitz: `Slam`, with a post-pass left-to-right dash offset applied inside the `ult` 96x96 cells so the dash covers more distance visually. Bake small body-attached flames into the `ult` body frames so it reads as a flaming dash. Do not use a separate forward-moving fireball/projectile strip for this ult.
- `dead`: `Hurt+Sleep`
- Speed Boost passive does not need a separate animation.
- ImageGen VFX staged for future custom calls:
  - `assets\custom_spritework\vfx\blaziken_ult_flame_trail#sheet.png` / `#anim.fanim`
  - Source is saved as `assets\custom_spritework\references\imagegen\blaziken_ult_flame_trail_source.png`.
  - A generated dash-fire strip that looked like a detached projectile was removed. Flare Blitz should be body-attached fire plus a persistent ground trail, not a flying fire projectile.

Blaziken conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0257 `
  --source-dir assets\custom_spritework\references\pmdcollab\0257 `
  --output-base assets\custom_spritework\champions\blaziken `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Kick `
  --map skill=Shoot `
  --map skill2=Charge `
  --map ult=Slam `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4333 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --max-content-size 44 `
  --bottom-padding 28
```

After conversion, reapply the Blaziken `ult` dash-offset post-pass currently documented by the preview output: `[-10,-8,-5,-2,2,6,10,13,16,18,18,16,14,12,9,6,3,0,0]`, bake the local body-attached flame wrap into the `ult` frames, then regenerate Blaziken previews and re-clean green chroma fringe on the ImageGen flame trail VFX sheet.

## Blissey Source Mapping

- PMDCollab source: `sprite/0242`, from `https://sprites.pmdcollab.org/#/0242?form=0`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Pound basic: `Attack`, with `--travel-scale attack=0.25` so the melee leap is much smaller than the source animation.
- `skill` / Heal Pulse: `Twirl`, with the ImageGen-created healing aura baked directly into the champion `skill` frames. This is not a separate staged VFX/projectile asset because the move is a Blissey-centered aura and needs to read in-game through the body sprite path.
- `skill2` / Softboiled: `Charge`
- `ult` / Healing Wish: `Shoot`
- `dead`: `Hurt+Sleep`
- Healer passive does not need a separate animation.

Blissey conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0242 `
  --source-dir assets\custom_spritework\references\pmdcollab\0242 `
  --output-base assets\custom_spritework\champions\blissey `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Twirl `
  --map skill2=Charge `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.35 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=3.0 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.25 `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

After conversion, pack `assets\custom_spritework\references\imagegen\blissey_skill1_heal_pulse_aura_source.png` into `assets\custom_spritework\temp\blissey_skill1_heal_pulse_aura_bake`, flood-clean the green chroma key, and composite the aura behind Blissey's `skill` frames in `assets\custom_spritework\champions\blissey#sheet.png`. Regenerate the Blissey preview GIFs and contact sheet after baking.

## Bouffalant Source Mapping

- PMDCollab source: none available for `sprite/0626` at the time this pass was created.
- Static reference sprites are preserved under `assets\custom_spritework\references\static\bouffalant`.
- Full ImageGen source sheet is preserved as `assets\custom_spritework\references\imagegen\bouffalant_full_source.png`.
- `idle`: ImageGen-generated subtle breathing bounce, 8 packed frames.
- `run`: ImageGen-generated grounded heavy hoof walk/run, 8 packed frames. No dust.
- `attack` / Stomp basic: ImageGen-generated short melee stomp/body slam, 8 packed frames with minimal travel.
- `skill` / Retaliate: ImageGen-generated rooted defensive stance with a small body-attached guard aura, 8 packed frames. The aura is baked into the champion sprite frames.
- `skill2` / Stomping Tantrum: ImageGen-generated ground stomp with a small body-attached impact ring/crack effect under Bouffalant, 8 packed frames.
- `ult` / Head Charge: ImageGen-generated left-to-right charging headbutt with body-attached speed streaks and impact motion, 10 packed frames. Do not stage this as a separate projectile. The current pack skips the generated upright recovery frame and uses the source-frame sequence `0,1,2,3,4,5,6,7,6,5` so the animation ends in charge/impact motion instead of a still standing pose.
- `dead`: ImageGen-generated hurt/slump/faint, 4 packed frames.
- Afro passive does not need a separate animation.

Bouffalant was generated as one complete multi-row ImageGen source sheet using the two static sprites as references so the model stays consistent across idle, walk, attack, Skill 1, Skill 2, ult, and death. The generated sheet was then chroma-key cleaned, component-bounded per frame, normalized into 96x96 cells, and packed into:

```text
assets\custom_spritework\champions\bouffalant#sheet.png
assets\custom_spritework\champions\bouffalant#anim.fanim
```

If Bouffalant needs to be regenerated later, use a single full-sheet ImageGen prompt with a flat `#00ff00` background, seven horizontal rows, and left-to-right action readability. After generation, do not rely on equal grid slicing alone; run a component/bounding-box extraction pass or manually verify every action row because close ImageGen spacing can otherwise pull neighboring-frame body pieces into attack and skill frames. For rows with baked effects, such as Retaliate, Stomping Tantrum, and Head Charge, scale from Bouffalant's body bbox rather than the full effect bbox so the body size stays stable while the aura/impact/charge effect extends around it.

## Ceruledge Source Mapping

- PMDCollab source: `sprite/0937`, from `https://sprites.pmdcollab.org/#/0937?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\ceruledge_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Astonish basic: `Shoot`
- `skill` / Flame Charge: `Attack`, with a post-pass that shifts the body forward across the tag so the dash reads longer. A blue-violet Flame Charge trail from ImageGen is baked into the `skill` frames and staged separately for future custom VFX as `assets\custom_spritework\vfx\ceruledge_skill1_flame_trail#sheet.png` and `#anim.fanim`.
- `skill2` / Poltergeist: `Charge`, with an ImageGen ghost/item aura baked behind the body frames for readability.
- `ult` / Bitter Blade: `Strike`, with `--travel-scale ult=0.35` to keep the melee attack from traveling too far and an ImageGen blue-violet/red slash effect baked around the body frames.
- `dead`: `Hurt+Sleep`
- Flash Fire passive does not need a separate animation.

Ceruledge conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0937 `
  --source-dir assets\custom_spritework\references\pmdcollab\0937 `
  --output-base assets\custom_spritework\champions\ceruledge `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Attack `
  --map skill2=Charge `
  --map ult=Strike `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.3667 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=0.7333 `
  --tag-duration dead=1.0 `
  --travel-scale skill=1.25 `
  --travel-scale ult=0.35 `
  --max-content-size 44 `
  --bottom-padding 28
```

After conversion, reapply the Ceruledge effect post-pass: extract the three rows from `ceruledge_effects_source.png`, pack the Flame Charge trail into `ceruledge_skill1_flame_trail`, bake the trail behind the `skill` frames while shifting the body forward, bake the Poltergeist aura behind `skill2`, and bake the Bitter Blade slash around `ult`. The converter currently only reduces travel with `--travel-scale`, so Flame Charge's longer apparent dash is handled by the post-pass offsets, not by the conversion command alone.

## Charizard Source Mapping

- PMDCollab source: `sprite/0006`, from `https://sprites.pmdcollab.org/#/0006?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\charizard_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Flamethrower basic: `Charge`; custom cone VFX is staged separately as `assets\custom_spritework\vfx\charizard_basicattack_flamethrower_cone#sheet.png` and `#anim.fanim`.
- `skill` / Dragon Breath: `Charge`; custom cone VFX is staged separately as `assets\custom_spritework\vfx\charizard_skill1_dragon_breath_cone#sheet.png` and `#anim.fanim`.
- `skill2` / Fire Blast: `Shoot`, with `--recenter-tag skill2` to reduce the source hop and `--frame-select skill2=0-9` to remove the tiny recovery pose at the end. Custom Fire Blast projectile VFX is staged separately as `assets\custom_spritework\vfx\charizard_skill2_fire_blast_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Inferno: `Hop`, with the ImageGen circular inferno ring baked into the `ult` champion frames because this is a radius effect around Charizard rather than a projectile.
- `dead`: `Hurt+Sleep`
- Blaze passive does not need a separate animation.

Charizard conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0006 `
  --source-dir assets\custom_spritework\references\pmdcollab\0006 `
  --output-base assets\custom_spritework\champions\charizard `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Charge `
  --map skill=Charge `
  --map skill2=Shoot `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.7333 `
  --tag-duration skill=0.8 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=0.9667 `
  --tag-duration dead=1.0 `
  --frame-select skill2=0-9 `
  --recenter-tag skill2 `
  --max-content-size 48 `
  --bottom-padding 28
```

After conversion, reapply the Charizard effect post-pass: extract the four rows from `charizard_effects_source.png`, pack the Flamethrower cone, Dragon Breath cone, and Fire Blast projectile as separate staged VFX assets, and bake the Inferno ring behind/around the `ult` frames. Keep all directional effects left-to-right.

## Clawitzer Source Mapping

- PMDCollab source: `sprite/0693`, from `https://sprites.pmdcollab.org/#/0693?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\clawitzer_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Aqua Jet basic: `Shoot`; Aqua Jet projectile VFX is staged separately as `assets\custom_spritework\vfx\clawitzer_basicattack_aqua_jet_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Water Sport: `Shoot`; three-circle Water Sport AoE VFX is staged separately as `assets\custom_spritework\vfx\clawitzer_skill1_water_sport_aoe#sheet.png` and `#anim.fanim`.
- `skill2` / Aqua Cutter: `Shoot`; Aqua Cutter projectile VFX is staged separately as `assets\custom_spritework\vfx\clawitzer_skill2_aqua_cutter_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Bubble Beam: `Shoot`; Bubble Beam line VFX is staged separately as `assets\custom_spritework\vfx\clawitzer_ult_bubble_beam_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Clingy passive does not need separate spritework; while attached to an ally, Clawitzer may not visibly play much of the body animation in-game.

Clawitzer conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0693 `
  --source-dir assets\custom_spritework\references\pmdcollab\0693 `
  --output-base assets\custom_spritework\champions\clawitzer `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6667 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=1.9667 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

After conversion, extract the four rows from `clawitzer_effects_source.png` using the magenta key, pack Aqua Jet, Water Sport, Aqua Cutter, and Bubble Beam as separate staged VFX assets, and regenerate 288x288 body/VFX previews. Use strict slot boundaries for this ImageGen sheet; the Water Sport row can otherwise pick up slivers from adjacent frames.

## Cloyster Source Mapping

- PMDCollab source: `sprite/0091`, from `https://sprites.pmdcollab.org/#/0091?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\cloyster_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Glaciate basic: `Attack`, with `--travel-scale attack=0.2` and `--recenter-tag attack` so the melee body lunge is much smaller. Custom icy basic projectile VFX is staged separately as `assets\custom_spritework\vfx\cloyster_basicattack_glaciate_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Icicle Crash: `Shoot`; one reusable left-to-right icicle projectile VFX is staged as `assets\custom_spritework\vfx\cloyster_skill1_icicle_crash_projectile#sheet.png` and `#anim.fanim`. The Rust move fires 4/5/6/7 separate hits by rank, so the eventual custom VFX call should spawn this same projectile once per hit rather than requiring separate art for each shard.
- `skill2` / Clamp: `Attack`, preserving source travel so Cloyster lunges/grabs onto the enemy.
- `ult` / Aurora Beam: `Charge`; custom horizontal Aurora Beam line VFX is staged separately as `assets\custom_spritework\vfx\cloyster_ult_aurora_beam_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Overcoat passive does not need separate spritework.

Cloyster conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0091 `
  --source-dir assets\custom_spritework\references\pmdcollab\0091 `
  --output-base assets\custom_spritework\champions\cloyster `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Shoot `
  --map skill2=Attack `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.75 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.2 `
  --recenter-tag attack `
  --max-content-size 40 `
  --bottom-padding 28
```

After conversion, extract the three rows from `cloyster_effects_source.png` using the magenta key: Glaciate projectile, Icicle Crash reusable projectile, and Aurora Beam line. Use strict slot boundaries and remove detached ahead-fragments if ImageGen creates small spark components in the next projectile's space. Regenerate 288x288 body/VFX previews and confirm Clamp remains the only travel-heavy action.

## Coalossal Source Mapping

- PMDCollab source: none available for `sprite/0839` at the time this pass was created.
- External visual reference requested by James: `https://www.deviantart.com/magna-ryunoid/art/839-coalossal-sprite-animation-846454694`. The staged sheet was generated with ImageGen rather than downloaded from that page.
- Initial ImageGen body source sheet: `assets\custom_spritework\references\imagegen\coalossal_full_source.png`.
- Corrective body-only action source sheet: `assets\custom_spritework\references\imagegen\coalossal_body_fix_source.png`.
- ImageGen VFX source sheet: `assets\custom_spritework\references\imagegen\coalossal_vfx_source.png`.
- Corrective Magma Storm floor VFX source sheet: `assets\custom_spritework\references\imagegen\coalossal_ult_magma_storm_floor_source.png`.
- `idle`: ImageGen-generated subtle ember/breathing loop, 8 packed frames. ImageGen produced seven unique idle poses, so the pack duplicates one midpoint frame to keep an 8-frame loop.
- `run`: ImageGen-generated heavy grounded walk cycle, 8 packed frames. No dust.
- `attack` / Rock Blast basic: ImageGen-generated rooted firing body-only animation, 8 packed frames. Do not include the Rock Blast projectile in the champion preview; the separate future VFX projectile is staged as `assets\custom_spritework\vfx\coalossal_basicattack_rock_blast_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Tar Shot: ImageGen-generated rooted body-only lean/launch animation, 8 packed frames. Do not include the traveling tar wave in the champion preview; the wave is staged separately as `assets\custom_spritework\vfx\coalossal_skill1_tar_shot_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Smokescreen: ImageGen-generated body-only exhale/cast animation, 8 packed frames. Do not include the placed smoke wall/cloud in the champion preview; the larger field is staged separately as `assets\custom_spritework\vfx\coalossal_skill2_smokescreen_field#sheet.png` and `#anim.fanim`.
- `ult` / Magma Storm: ImageGen-generated rooted brace/stomp body-only animation, 10 packed frames. The champion frames intentionally avoid mouth-shot or projectile-only frames. The correct VFX is an arena-floor effect: two parallel rectangular magma/fire lanes appear first, then the center erupts after the skill delay for the higher-damage center hit. This is staged as `assets\custom_spritework\vfx\coalossal_ult_magma_storm_line#sheet.png` and `#anim.fanim`.
- `dead`: ImageGen-generated hurt/collapse/faint, 5 packed frames.
- Steam Engine passive does not need separate spritework.

Coalossal was generated as ImageGen-only because there was no PMDCollab source. The body sheet was manually windowed from the generated source rather than component-auto-packed because the full generated sheet had close frame spacing and large effects that could otherwise make body animations inherit neighboring slices or disappear into projectile-only frames. If Coalossal is regenerated later, use at least two ImageGen passes: one body-only action sheet and one VFX-only sheet. Keep the body tags readable on Coalossal himself and stage the long Rock Blast, Tar Shot, Smokescreen, and Magma Storm visuals as separate VFX assets. Magma Storm should be packed from floor-effect frames, not from a mouth projectile or horizontal fireball strip.

## Comfey Source Mapping

- PMDCollab source: `sprite/0764`, from `https://sprites.pmdcollab.org/#/0764?form=0`.
- ImageGen Petal Dance source: `assets\custom_spritework\references\imagegen\comfey_petal_dance_aura_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Growth: `Charge`, recentered.
- `skill` / Floral Healing: `Charge`, recentered.
- `skill2` / Petal Dance: `Charge`, recentered. The placed Grass/Fairy floor aura is staged separately as `assets\custom_spritework\vfx\comfey_skill2_petal_dance_aura#sheet.png` and `#anim.fanim`.
- `ult` / Encore: `Charge`, recentered.
- `dead`: `Hurt+Sleep`
- Flower Veil passive does not need separate spritework. Comfey is usually attached to an ally, so many body animations may rarely be visible in-game.

Comfey conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0764 `
  --source-dir assets\custom_spritework\references\pmdcollab\0764 `
  --output-base assets\custom_spritework\champions\comfey `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Charge `
  --map skill=Charge `
  --map skill2=Charge `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4333 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 38 `
  --bottom-padding 28
```

After conversion, pack the ImageGen Petal Dance source using a magenta key so the green aura pixels stay intact. Keep the Petal Dance aura as a separate staged VFX asset instead of baking it into Comfey's body frames.

## Cryogonal Source Mapping

- PMDCollab source: `sprite/0615`, from `https://sprites.pmdcollab.org/#/0615?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\cryogonal_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Ice Shard basic: `Shoot`, with `--travel-scale attack=0.25` and `--recenter-tag attack` so the source movement is reduced. The Ice Shard projectile is staged separately as `assets\custom_spritework\vfx\cryogonal_basicattack_ice_shard_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Icy Wind: `Charge`; the traveling freeze wave is staged separately as `assets\custom_spritework\vfx\cryogonal_skill1_icy_wind_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Freeze-Dry: `SpAttack`; the crystalline freeze burst is staged separately as `assets\custom_spritework\vfx\cryogonal_skill2_freeze_dry_burst#sheet.png` and `#anim.fanim`.
- `ult` / Ice Beam: `Charge`; the piercing line beam is staged separately as `assets\custom_spritework\vfx\cryogonal_ult_ice_beam_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Immunity passive does not need separate spritework.

Cryogonal conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0615 `
  --source-dir assets\custom_spritework\references\pmdcollab\0615 `
  --output-base assets\custom_spritework\champions\cryogonal `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=SpAttack `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4333 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.75 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.25 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

After conversion, pack the four ImageGen rows with a magenta key so blue/cyan ice pixels stay intact. The generated Ice Shard and Freeze-Dry rows may have fewer than eight unique source groups; duplicate a nearby frame when needed to keep stable 8-frame VFX timing.

## Decidueye Source Mapping

- PMDCollab source: `sprite/0724`, from `https://sprites.pmdcollab.org/#/0724?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\decidueye_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Leafage: `Shoot`; the triple-arc leaf projectile VFX is staged separately as `assets\custom_spritework\vfx\decidueye_basicattack_leafage_triple_arc#sheet.png` and `#anim.fanim`.
- `skill` / Phantom Force: `Charge`; the ghostly impact/dash VFX is staged separately as `assets\custom_spritework\vfx\decidueye_skill1_phantom_force_impact#sheet.png` and `#anim.fanim`.
- `skill2` / Leaf Storm: `Shake`, with a post-pass that shifts the body backward across the tag to sell the self-retreat. The wide leaf storm line VFX is staged separately as `assets\custom_spritework\vfx\decidueye_skill2_leaf_storm_line#sheet.png` and `#anim.fanim`.
- `ult` / Spirit Shackle: `Double`; the spectral binding arrow/projectile is staged separately as `assets\custom_spritework\vfx\decidueye_ult_spirit_shackle_projectile#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Long Reach passive does not need separate spritework.

Decidueye conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0724 `
  --source-dir assets\custom_spritework\references\pmdcollab\0724 `
  --output-base assets\custom_spritework\champions\decidueye `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=Shake `
  --map ult=Double `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4333 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.75 `
  --tag-duration ult=0.9 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag ult `
  --max-content-size 48 `
  --bottom-padding 28
```

After conversion, reapply the Leaf Storm post-pass to `skill2`: shift the Shake frames left over the tag with offsets roughly `0, -2, -4, -7, -9, -11, -13, -16, -14, -12, -10` so Decidueye visibly leaps backward while casting. Pack VFX from the ImageGen source with a magenta key and use equal source-frame columns when necessary to avoid small leaves/spirit particles bleeding across frame boundaries.

## Dedenne Source Mapping

- PMDCollab source: `sprite/0702`, from `https://sprites.pmdcollab.org/#/0702?form=0`.
- ImageGen general effect source: `assets\custom_spritework\references\imagegen\dedenne_effects_source.png`.
- ImageGen corrected Spark Grid source: `assets\custom_spritework\references\imagegen\dedenne_ult_spark_grid_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Nuzzle: `Attack`, with reduced travel because this is a melee basic. The contact zap VFX is staged separately as `assets\custom_spritework\vfx\dedenne_basicattack_nuzzle_spark#sheet.png` and `#anim.fanim`.
- `skill` / Play Rough: `QuickStrike`, recentered. Do not preserve PMDCollab's full QuickStrike offset in the sprite frames because it leaves the 96x96 canvas; the Rust move already force-moves Dedenne for the dash. The Fairy/Electric dash streak is staged separately as `assets\custom_spritework\vfx\dedenne_skill1_play_rough_dash#sheet.png` and `#anim.fanim`.
- `skill2` / Double Shock: `Charge`; the local electric ring VFX is staged separately as `assets\custom_spritework\vfx\dedenne_skill2_double_shock_ring#sheet.png` and `#anim.fanim`.
- `ult` / Electric Terrain: `Hop`; Spark Grid needs multiple future-ready VFX assets because the Rust move has separate placement, chain damage, overload, and consume/heal states:
  - `assets\custom_spritework\vfx\dedenne_ult_spark_grid_nodes#sheet.png`
  - `assets\custom_spritework\vfx\dedenne_ult_spark_grid_chain#sheet.png`
  - `assets\custom_spritework\vfx\dedenne_ult_spark_grid_overload#sheet.png`
  - `assets\custom_spritework\vfx\dedenne_ult_spark_grid_consume_heal#sheet.png`
- `dead`: `Hurt+Sleep`
- Static passive does not need separate spritework.

Dedenne conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0702 `
  --source-dir assets\custom_spritework\references\pmdcollab\0702 `
  --output-base assets\custom_spritework\champions\dedenne `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=QuickStrike `
  --map skill2=Charge `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.3333 `
  --tag-duration skill=0.4667 `
  --tag-duration skill2=1.0667 `
  --tag-duration ult=0.4 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 38 `
  --bottom-padding 28
```

When packing Dedenne's ult VFX, use the corrected Spark Grid source rather than the first general ImageGen sheet for the terrain rows. The corrected source preserves the required topology: one center node plus six surrounding nodes in a flower/hex ring. The overload source had a thin generated row-separator line near the bottom of the frames; strip wide bottom separator rows after packing.

## Delibird Source Mapping

- PMDCollab source: `sprite/0225`, from `https://sprites.pmdcollab.org/#/0225?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\delibird_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Present: `Shoot`, with reduced source travel. The thrown present is staged separately as `assets\custom_spritework\vfx\delibird_present_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Present (Healing): `Shoot`, with reduced source travel; reuse the present projectile VFX unless a later pass wants a separate healing-colored present.
- `skill2` / Present (Reserve): `Shoot`, with reduced source travel; reuse the present projectile VFX.
- `ult` / Drill Peck: `Nod`, using Delibird's right-facing row (`--direction-row 2`), then bake in the ImageGen `delibird_ult_drill_peck_flourish` icy peck burst so the ult reads as an attack instead of a simple nod. Do not use the front-facing Nod rows for this attack. The baked champion ult uses a clean blank final fade frame to avoid the generated edge fragments.
- `dead`: `Hurt+Sleep`
- Hustle passive does not need separate spritework.

Delibird conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0225 `
  --source-dir assets\custom_spritework\references\pmdcollab\0225 `
  --output-base assets\custom_spritework\champions\delibird `
  --direction-row 2 `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Nod `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.4333 `
  --tag-duration skill2=0.4333 `
  --tag-duration ult=0.6667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.35 `
  --travel-scale skill2=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 46 `
  --bottom-padding 28
```

After conversion, repack `delibird_effects_source.png` into a present projectile row and a Drill Peck flourish row. Rebuild the champion `ult` tag by appending eight Nod-plus-flourish composite frames to `delibird#sheet.png` and redirecting the `ult` `.fanim` frames to those appended frames. Keep the separate Drill Peck flourish VFX source staged for future use, but current readability comes from the baked champion ult.

## Dragalge Source Mapping

- PMDCollab source: `sprite/0691`, from `https://sprites.pmdcollab.org/#/0691?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\dragalge_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Acid Spit: `Shoot`, with reduced source travel. The corrosive projectile is staged separately as `assets\custom_spritework\vfx\dragalge_basicattack_acid_spit_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Poison Tail: `Rotate`. The self-centered poison spin aura is staged separately as `assets\custom_spritework\vfx\dragalge_skill1_poison_tail_aura#sheet.png` and `#anim.fanim`.
- `skill2` / Sludge Bomb: `Shoot`, with reduced source travel. The triple poison splash/bounce read is staged separately as `assets\custom_spritework\vfx\dragalge_skill2_sludge_bomb_splashes#sheet.png` and `#anim.fanim`.
- `ult` / Toxic Chain: `Attack`, with reduced travel because this is a targeted status ult, not a full dash. The tether/execute marker is staged separately as `assets\custom_spritework\vfx\dragalge_ult_toxic_chain_tether#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Poison Sipper passive does not need separate spritework.

Dragalge conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0691 `
  --source-dir assets\custom_spritework\references\pmdcollab\0691 `
  --output-base assets\custom_spritework\champions\dragalge `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Rotate `
  --map skill2=Shoot `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.6 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill2=0.35 `
  --travel-scale ult=0.25 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 54 `
  --bottom-padding 28
```

When packing `dragalge_effects_source.png`, use detected per-row effect groups rather than equal-width fixed columns. The ImageGen source places the effects unevenly, and equal slicing causes vertical fragments from neighboring frames in the Poison Tail, Sludge Bomb, and Toxic Chain previews. The current staged VFX frame counts are 7/7/7/8 for Acid Spit, Poison Tail, Sludge Bomb, and Toxic Chain respectively.

## Drampa Source Mapping

- PMDCollab source: `sprite/0780`, from `https://sprites.pmdcollab.org/#/0780?form=0`.
- ImageGen general effect source: `assets\custom_spritework\references\imagegen\drampa_effects_source.png`.
- ImageGen replacement Draco Meteor source: `assets\custom_spritework\references\imagegen\drampa_draco_meteor_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Twister: `Shoot`, with reduced source travel. The narrow dragon wind line projectile is staged separately as `assets\custom_spritework\vfx\drampa_basicattack_twister_line#sheet.png` and `#anim.fanim`.
- `skill` / Outrage: `Double`. The PMDCollab body motion is subtle, so the self-buff readability comes from the separate caster-centered aura staged as `assets\custom_spritework\vfx\drampa_skill1_outrage_aura#sheet.png` and `#anim.fanim`.
- `skill2` / Dragon Energy: `RearUp`, with reduced source travel. The targeted dragon-energy burst is staged separately as `assets\custom_spritework\vfx\drampa_skill2_dragon_energy_burst#sheet.png` and `#anim.fanim`.
- `ult` / Draco Meteor: `Hop`. The large meteor AoE is staged separately as `assets\custom_spritework\vfx\drampa_ult_draco_meteor_aoe#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Berserk passive does not need separate spritework.

Drampa conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0780 `
  --source-dir assets\custom_spritework\references\pmdcollab\0780 `
  --output-base assets\custom_spritework\champions\drampa `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Double `
  --map skill2=RearUp `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4667 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=1.1667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill2=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 58 `
  --bottom-padding 28
```

When packing Drampa VFX, use detected groups for Twister and Outrage. The Dragon Energy row is wide and needs manual/fixed slicing. The first general ImageGen source produced connected Draco Meteor crater frames with hard vertical split artifacts, so replace the ult VFX with the cleaner eight-frame `drampa_draco_meteor_source.png` strip.

## Drednaw Source Mapping

- PMDCollab source: `sprite/0834`, from `https://sprites.pmdcollab.org/#/0834?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\drednaw_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Head Smash: `Attack`, with reduced travel because this is a melee basic. No separate VFX is staged for the basic attack.
- `skill` / Rock Tomb: `Shoot`. The target bind/cage VFX is staged separately as `assets\custom_spritework\vfx\drednaw_skill1_rock_tomb_bind#sheet.png` and `#anim.fanim`.
- `skill2` / Rock Polish: `Charge`. Only a light self-buff shimmer is staged separately as `assets\custom_spritework\vfx\drednaw_skill2_rock_polish_aura#sheet.png` and `#anim.fanim`.
- `ult` / Razor Shell: `Swing`, trimmed to a tight right/front arc with `--frame-select ult=0,1,0,8` instead of the full Swing rotation. The water spin-to-retreat trail VFX is staged separately as `assets\custom_spritework\vfx\drednaw_ult_razor_shell_trail#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Wide Guard passive does not need separate spritework.

Drednaw conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0834 `
  --source-dir assets\custom_spritework\references\pmdcollab\0834 `
  --output-base assets\custom_spritework\champions\drednaw `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Shoot `
  --map skill2=Charge `
  --map ult=Swing `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --frame-select ult=0,1,0,8 `
  --travel-scale attack=0.2 `
  --travel-scale ult=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 52 `
  --bottom-padding 28
```

When packing `drednaw_effects_source.png`, use detected left-to-right groups for all rows. Razor Shell should read as a tight water spin followed by a narrow retreat trail; do not replace it with a broad full-circle Swing body rotation.

## Eevee Source Mapping

- PMDCollab source: `sprite/0133`, from `https://sprites.pmdcollab.org/#/0133?form=0`.
- ImageGen effect source: `assets\custom_spritework\references\imagegen\eevee_effects_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Tackle: `Attack`, with lightly reduced travel. No separate VFX is staged for the basic attack.
- `skill` / Baby-Doll Eyes: `Wake`. The charm/debuff/protect cue is staged separately as `assets\custom_spritework\vfx\eevee_skill1_babydoll_eyes_charm#sheet.png` and `#anim.fanim`.
- `skill2` / Wish: PMDCollab `Appeal`, which is defined as a copy of `TailWhip` in `AnimData.xml`, so the converter uses `TailWhip`. The healing channel ring is staged separately as `assets\custom_spritework\vfx\eevee_skill2_wish_heal_ring#sheet.png` and `#anim.fanim`.
- `ult` / Baton Pass: `Charge`. The stat-transfer tether/ribbon is staged separately as `assets\custom_spritework\vfx\eevee_ult_baton_pass_tether#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Helping Hand passive does not need separate spritework.

Eevee conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0133 `
  --source-dir assets\custom_spritework\references\pmdcollab\0133 `
  --output-base assets\custom_spritework\champions\eevee `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Wake `
  --map skill2=TailWhip `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.3667 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=2.0 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 38 `
  --bottom-padding 28
```

When packing `eevee_effects_source.png`, use border-flood chroma removal instead of broad magenta threshold removal. The Baby-Doll Eyes row contains pink effects close to the chroma key, and aggressive thresholding removes useful heart/eye pixels.

## Electrode Source Mapping

- PMDCollab source: `sprite/0101`, from `https://sprites.pmdcollab.org/#/0101?form=0`.
- PMDCollab credits: `CHUNSOFT`, current as of the cached `credits.txt`.
- ImageGen explosion source: `assets\custom_spritework\references\imagegen\electrode_explosions_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Rollout: `Attack`, with reduced travel because this is a melee basic. No separate VFX is staged for the basic attack.
- `skill` / Magnet Rise: PMDCollab `RearUp`, which is defined as a copy of `Twirl` in `AnimData.xml`, so the converter uses `Twirl`.
- `skill2` / Magnetic Flux: `Hop`
- `ult` / Self-Destruct: `Charge`, followed by a post-pass that bakes a gradual red tint/flashing charge into Electrode's body and then replaces the final frames with explosion-only ImageGen frames so Electrode vanishes into the blast. The large Self-Destruct radius explosion is also staged separately as `assets\custom_spritework\vfx\electrode_ult_self_destruct_explosion_aoe#sheet.png` and `#anim.fanim`.
- `dead`: ImageGen-generated small explosion frames baked directly into the champion `dead` tag. Do not stage a separate normal-death VFX asset; non-ult deaths should be fully represented by the champion animation.
- Aftermath passive does not need a separate standing/buff animation.

Electrode conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0101 `
  --source-dir assets\custom_spritework\references\pmdcollab\0101 `
  --output-base assets\custom_spritework\champions\electrode `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Twirl `
  --map skill2=Hop `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.2333 `
  --tag-duration skill=0.4667 `
  --tag-duration skill2=0.4 `
  --tag-duration ult=2.6667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.3 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 40 `
  --bottom-padding 28
```

After conversion, pack `electrode_explosions_source.png` into one callable large AOE VFX asset and one temporary small-death bake source. Bake the ult red/flash charge and both explosion rows into `assets\custom_spritework\champions\electrode#sheet.png`, then remove the temporary small-death VFX files so only `electrode_ult_self_destruct_explosion_aoe` remains under `assets\custom_spritework\vfx`. Regenerate the Electrode preview GIFs and contact sheets after the bake. After size comparison QA, the accepted champion sheet was downscaled in place with `tools\scale-sprite-sheet-content.py --scale 0.90 --shift-y 0`, making the first idle frame 29 px tall with bottom anchor 68.

## Emboar Source Mapping

- PMDCollab source: `sprite/0500`, from `https://sprites.pmdcollab.org/#/0500?form=0`.
- PMDCollab credits are preserved in `assets\custom_spritework\references\pmdcollab\0500\credits.txt`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Arm Thrust: first five frames of `Charge` (`--frame-select attack=0-4`) with very low travel so the startup reads as a short punch rather than a full cast.
- `skill` / Hammer Arm: `Shoot`, left clean. A prior loose ImageGen impact-overlay pass looked pasted on in GIF previews, so no baked effect or standalone staged VFX is kept for this move.
- `skill2` / Heat Crash: PMDCollab `Strike`, which is defined as a copy of `Attack` in `AnimData.xml`, so the converter uses `Attack` and preserves most of the source dash travel. A prior loose ImageGen flame-trail overlay pass looked pasted on in GIF previews, so no baked effect or standalone staged VFX is kept for this move.
- `ult` / Power-up Punch: `Charge`, left clean for now. If this needs more readability later, rebuild the whole Power-up Punch animation intentionally with ImageGen using the PMDCollab Emboar body as reference, rather than compositing separate fist glow/impact sprites onto the existing frames.
- `dead`: `Hurt+Sleep`
- Reckless passive does not need separate spritework.

Emboar conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0500 `
  --source-dir assets\custom_spritework\references\pmdcollab\0500 `
  --output-base assets\custom_spritework\champions\emboar `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Charge `
  --map skill=Shoot `
  --map skill2=Attack `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4167 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.6667 `
  --tag-duration ult=1.5 `
  --tag-duration dead=1.0 `
  --frame-select attack=0-4 `
  --travel-scale attack=0.15 `
  --travel-scale skill2=0.8 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag ult `
  --max-content-size 54 `
  --bottom-padding 28
```

After conversion, keep the Emboar champion sheet clean and regenerate `assets\custom_spritework\previews\emboar_full_contact.png`. Do not keep the rejected loose ImageGen overlay source or VFX assets in the staged set.

## Espeon Source Mapping

- PMDCollab source: `sprite/0196`, from `https://sprites.pmdcollab.org/#/0196?form=0`.
- PMDCollab credits: `CHUNSOFT`, current as of the cached `credits.txt`.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\espeon_vfx_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\espeon_vfx_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Tackle: `Attack`, with reduced travel because this is a short melee/basic body action. No separate VFX is staged for the basic attack.
- `skill` / Confusion: `Double`. No separate VFX is staged for skill1 per the current art direction.
- `skill2` / Confuse Ray: `Shoot`. The PMDCollab body frames already include small star accents; keep the body clean otherwise. A separate future-ready violet psychic projectile/ring VFX is staged as `assets\custom_spritework\vfx\espeon_skill2_confuse_ray_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Future Sight: `Charge`. A separate future-ready centered warning sigil into psychic burst VFX is staged as `assets\custom_spritework\vfx\espeon_ult_future_sight_marker#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Eeveelution passive does not need separate spritework.

Espeon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0196 `
  --source-dir assets\custom_spritework\references\pmdcollab\0196 `
  --output-base assets\custom_spritework\champions\espeon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Double `
  --map skill2=Shoot `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.5667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.25 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

When packing `espeon_vfx_source.png`, use green-dominance chroma cleanup rather than exact-key matching. The first row packs to `espeon_skill2_confuse_ray_projectile` as a compact 96x96 left-to-right projectile. The second row packs to `espeon_ult_future_sight_marker` as a 128x128 centered target/impact VFX. Keep both as separate staged VFX; do not bake them onto the Espeon champion body unless a later full-action ImageGen rebuild is requested.

## Feraligatr Source Mapping

- PMDCollab source: `sprite/0160`, from `https://sprites.pmdcollab.org/#/0160?form=0`.
- PMDCollab credits: `CHUNSOFT`, current as of the cached `credits.txt`.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\feraligatr_muddy_water_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\feraligatr_muddy_water_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Scratch: `Strike`, with reduced travel because this is a melee basic. The PMDCollab Strike source includes its own small slash cue; no separate VFX is staged for the basic attack.
- `skill` / Aqua Tail: `Swing`, trimmed to frames `0,1,2,1,0` and reduced travel so it reads as a short tail/body arc instead of a full broad rotation.
- `skill2` / Thrash: `Double`. No separate VFX is staged for the self-offense buff.
- `ult` / Muddy Water: `Shoot`. A separate future-ready left-to-right muddy water line VFX is staged as `assets\custom_spritework\vfx\feraligatr_ult_muddy_water_line#sheet.png` and `#anim.fanim`. The Rust move is a 72k range, 22k-wide piercing line slow, so this VFX should read as a horizontal muddy wave rather than a round projectile.
- `dead`: `Hurt+Sleep`
- Spiked Hide passive does not need separate spritework.

Feraligatr conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0160 `
  --source-dir assets\custom_spritework\references\pmdcollab\0160 `
  --output-base assets\custom_spritework\champions\feraligatr `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Swing `
  --map skill2=Double `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0 `
  --frame-select skill=0,1,2,1,0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 54 `
  --bottom-padding 28
```

When packing `feraligatr_muddy_water_source.png`, use green-dominance chroma cleanup and pack the generated six usable left-to-right wave frames into 192x96 frames under the `line` tag. Keep Muddy Water as a separate staged VFX; do not bake it onto Feraligatr's champion body.

## Flareon Source Mapping

- PMDCollab source: `sprite/0136`, from `https://sprites.pmdcollab.org/#/0136?form=0`.
- PMDCollab credits: `CHUNSOFT` for Walk/Attack/Strike/Shoot/Appeal/TailWhip/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate, plus CC-BY-NC 4.0 contributor rows in `credits.txt` for Pain and other supplemental animations.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\flareon_vfx_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\flareon_vfx_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Ember: `Shoot`. A separate future-ready left-to-right Ember projectile is staged as `assets\custom_spritework\vfx\flareon_basic_ember_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Fire Spin: `Rotate`. A separate future-ready caster-centered fire aura/ring is staged as `assets\custom_spritework\vfx\flareon_skill1_fire_spin_aura#sheet.png` and `#anim.fanim`.
- `skill2` / Flame Wheel: `Pain`. A separate future-ready left-to-right Flame Wheel line projectile/travel VFX is staged as `assets\custom_spritework\vfx\flareon_skill2_flame_wheel_line#sheet.png` and `#anim.fanim`.
- `ult` / Fire Fang: `Attack`, with reduced travel because the Rust move already handles the dash. A separate future-ready Fire Fang impact is staged as `assets\custom_spritework\vfx\flareon_ult_fire_fang_impact#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Eeveelution passive does not need separate spritework.

Flareon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0136 `
  --source-dir assets\custom_spritework\references\pmdcollab\0136 `
  --output-base assets\custom_spritework\champions\flareon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Rotate `
  --map skill2=Pain `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.9 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=0.5667 `
  --tag-duration dead=1.0 `
  --travel-scale ult=0.45 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 40 `
  --bottom-padding 28
```

When packing `flareon_vfx_source.png`, split the ImageGen rows into Ember projectile, Fire Spin aura, Flame Wheel line, and Fire Fang impact. Ember packs to five 96x96 `projectile` frames, Fire Spin to eight 128x128 `aura` frames, Flame Wheel to five 160x96 `line` frames, and Fire Fang to six 96x96 `impact` frames. The Fire Fang row needs manual paired crops for the opening crescent/fang frames; component-based auto-detection splits the left and right fang arcs into separate frames.

## Frosmoth Source Mapping

- PMDCollab source: `sprite/0873`, from `https://sprites.pmdcollab.org/#/0873?form=0`.
- PMDCollab credits: `JHONY_REX` / PMDCollab_1, another PMDCollab_1 contributor row, and a CC-BY-NC 4.0 contributor row in `credits.txt` for Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Swing/Double/Rotate/Hop.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\frosmoth_vfx_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\frosmoth_vfx_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Powder Snow: `Shoot`. A separate future-ready compact icy impact/AoE is staged as `assets\custom_spritework\vfx\frosmoth_basic_powder_snow_impact#sheet.png` and `#anim.fanim`.
- `skill` / Avalanche: `Shoot`. A separate future-ready left-to-right jagged ice wave is staged as `assets\custom_spritework\vfx\frosmoth_skill1_avalanche_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Sleep Powder: `Swing`, because the Rust skill force-moves Frosmoth around a fixed field. The placed field VFX is staged separately as `assets\custom_spritework\vfx\frosmoth_skill2_sleep_powder_field#sheet.png` and `#anim.fanim`; do not treat this as a caster-following aura.
- `ult` / Silver Wind: `Charge`. A separate future-ready left-to-right silver bug-wing wind wave is staged as `assets\custom_spritework\vfx\frosmoth_ult_silver_wind_wave#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Ice Scales passive does not need separate spritework.

Frosmoth conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0873 `
  --source-dir assets\custom_spritework\references\pmdcollab\0873 `
  --output-base assets\custom_spritework\champions\frosmoth `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Swing `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5 `
  --tag-duration skill=0.6667 `
  --tag-duration skill2=0.8667 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 46 `
  --bottom-padding 28
```

When packing `frosmoth_vfx_source.png`, use flood chroma cleanup so the pale green Sleep Powder pixels are preserved while only the connected green background becomes alpha. Powder Snow packs to five 96x96 `impact` frames, Avalanche to six 192x96 `line` frames, Sleep Powder to a clean six-frame 160x160 pulsing `field` loop using the non-bleeding generated field frames, and Silver Wind to six 192x96 `line` frames. Keep all VFX separate from the Frosmoth champion body.

## Gallade Source Mapping

- PMDCollab source: `sprite/0475`, from `https://sprites.pmdcollab.org/#/0475?form=0`.
- PMDCollab credits: `CHUNSOFT`, current as of the cached `credits.txt`, for Walk/Attack/Strike/Shoot/RearUp/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\gallade_psycho_cut_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\gallade_psycho_cut_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Sacred Sword: `Strike`, with reduced travel because this is a melee basic.
- `skill` / Future Sight: `Charge`. No separate VFX is staged in this pass.
- `skill2` / Calm Mind: `RearUp`. No separate VFX is staged in this pass.
- `ult` / Psycho Cut: `Shoot`. A separate future-ready left-to-right psychic arc slash is staged as `assets\custom_spritework\vfx\gallade_ult_psycho_cut_cone#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Justified passive does not need separate spritework.

Gallade conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0475 `
  --source-dir assets\custom_spritework\references\pmdcollab\0475 `
  --output-base assets\custom_spritework\champions\gallade `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Charge `
  --map skill2=RearUp `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.4667 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 48 `
  --bottom-padding 28
```

When packing `gallade_psycho_cut_source.png`, use flood chroma cleanup, detected six frame groups, and pack to 192x96 arc-slash frames. ImageGen left some lime-green antialias/trail pixels after chroma cleanup; recolor/remove green-dominant remaining pixels so Psycho Cut reads as cyan/magenta psychic energy rather than a chroma-key artifact. After size comparison QA, the accepted champion sheet was downscaled in place with `tools\scale-sprite-sheet-content.py --scale 0.92 --shift-y 0`, making the first idle frame 35 px tall with bottom anchor 68.

## Gholdengo Source Mapping

- PMDCollab source: `sprite/1000`, from `https://sprites.pmdcollab.org/#/1000?form=0`.
- PMDCollab credits: PMDCollab_2 contributor rows in `credits.txt` for Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/SpAttack/Swing/Double/Rotate/Hop, with later Attack/Shoot/SpAttack updates.
- ImageGen VFX sources: `assets\custom_spritework\references\imagegen\gholdengo_vfx_source.png` and `assets\custom_spritework\references\imagegen\gholdengo_make_it_rain_source.png`, with cleaned alpha sources preserved beside them.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Flash Cannon: `Shoot`. A separate future-ready steel projectile is staged as `assets\custom_spritework\vfx\gholdengo_basic_flash_cannon_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Ominous Wind: `Shoot`. A separate future-ready left-to-right ghost wind wave is staged as `assets\custom_spritework\vfx\gholdengo_skill1_ominous_wind_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Metal Sound: `Charge`. A separate future-ready caster-centered sound aura is staged as `assets\custom_spritework\vfx\gholdengo_skill2_metal_sound_aura#sheet.png` and `#anim.fanim`.
- `ult` / Make It Rain: `SpAttack`. A separate future-ready octagonal coin-rain AoE is staged as `assets\custom_spritework\vfx\gholdengo_ult_make_it_rain_aoe#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Good as Gold passive does not need separate spritework.

Gholdengo conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 1000 `
  --source-dir assets\custom_spritework\references\pmdcollab\1000 `
  --output-base assets\custom_spritework\champions\gholdengo `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Charge `
  --map ult=SpAttack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.6667 `
  --tag-duration ult=0.9 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 48 `
  --bottom-padding 28
```

When packing `gholdengo_vfx_source.png`, use flood chroma cleanup, then recolor remaining green-dominant effect pixels instead of treating them as valid art. The first Flash Cannon generation was overdesigned and the first Metal Sound pass did not read as sound, so the final staged basic/skill2 assets use the magenta-key replacement `gholdengo_flash_metal_sound_source_v2.png`: Flash Cannon is simplified to only the main steel projectile component, and Metal Sound is packed from the waveform/concentric-ring row. The first Make It Rain generation had connected frame bleed and green fill inside the octagons, so the final staged ult uses the separate magenta-key `gholdengo_make_it_rain_source.png`; after cleanup, recolor the generated internal magenta floor fill to warm gold/orange. Final VFX counts are 5 Flash Cannon `projectile` frames, 6 Ominous Wind `line` frames, 6 Metal Sound `aura` frames, and 8 Make It Rain `aoe` frames.

## Glaceon Source Mapping

- PMDCollab source: `sprite/0471`, from `https://sprites.pmdcollab.org/#/0471?form=0`.
- PMDCollab credits: `CHUNSOFT`, a PMDCollab_1 contributor row, and a later `CC_BY-NC_4` contributor row are preserved in `assets\custom_spritework\references\pmdcollab\0471\credits.txt`.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\glaceon_vfx_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\glaceon_vfx_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Ice Shard: `Shoot`. A separate future-ready left-to-right ice shard projectile is staged as `assets\custom_spritework\vfx\glaceon_basic_ice_shard_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Snowscape: `Shoot`. A separate centered ice field VFX is staged as `assets\custom_spritework\vfx\glaceon_skill1_snowscape_field#sheet.png` and `#anim.fanim`.
- `skill2` / Ice Fang: `QuickStrike`, with reduced travel because the Rust move already handles the leap to target. A separate future-ready fang impact VFX is staged as `assets\custom_spritework\vfx\glaceon_skill2_ice_fang_impact#sheet.png` and `#anim.fanim`.
- `ult` / Blizzard: `Charge`. A separate larger centered blizzard field VFX is staged as `assets\custom_spritework\vfx\glaceon_ult_blizzard_field#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Eeveelution passive does not need separate spritework.

Glaceon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0471 `
  --source-dir assets\custom_spritework\references\pmdcollab\0471 `
  --output-base assets\custom_spritework\champions\glaceon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=QuickStrike `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=1.1667 `
  --tag-duration dead=1.0 `
  --travel-scale skill2=0.65 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 40 `
  --bottom-padding 28
```

When packing `glaceon_vfx_source.png`, use border-flood cleanup tuned for the generated magenta gradient background, not exact `255,0,255`, then split the four rows into separate assets. Final VFX counts are 5 Ice Shard `projectile` frames, 6 Snowscape `field` frames, 6 Ice Fang `impact` frames, and 8 Blizzard `field` frames. Keep these VFX separate from the Glaceon champion body.

## Goodra Source Mapping

- PMDCollab source: `sprite/0706`, from `https://sprites.pmdcollab.org/#/0706?form=0`.
- PMDCollab credits: one PMDCollab_1 contributor row in `assets\custom_spritework\references\pmdcollab\0706\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Swing/Double/Rotate/Hop.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\goodra_vfx_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\goodra_vfx_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Dizzy Punch: `Attack`, with reduced travel because this is a short melee basic. A separate future-ready confusion impact VFX is staged as `assets\custom_spritework\vfx\goodra_basic_dizzy_punch_impact#sheet.png` and `#anim.fanim`.
- `skill` / Rain Dance: `Double`, with reduced horizontal travel. A separate future-ready wide rain field VFX is staged as `assets\custom_spritework\vfx\goodra_skill1_rain_dance_field#sheet.png` and `#anim.fanim`.
- `skill2` / Dragon Cheer: `Hop`. A separate future-ready caster-centered dragon cheer aura is staged as `assets\custom_spritework\vfx\goodra_skill2_dragon_cheer_aura#sheet.png` and `#anim.fanim`.
- `ult` / Life Dew: `Charge`. Separate future-ready Life Dew pickup and trigger visuals are staged as `assets\custom_spritework\vfx\goodra_ult_life_dew_pickup#sheet.png` / `#anim.fanim` and `assets\custom_spritework\vfx\goodra_ult_life_dew_heal_burst#sheet.png` / `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Sap Sipper passive does not need separate spritework.

Goodra conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0706 `
  --source-dir assets\custom_spritework\references\pmdcollab\0706 `
  --output-base assets\custom_spritework\champions\goodra `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Double `
  --map skill2=Hop `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5667 `
  --tag-duration skill=0.6 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=0.9 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.45 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 52 `
  --bottom-padding 28
```

When packing `goodra_vfx_source.png`, use border-flood cleanup tuned for the generated magenta background so the pink/violet effect pixels remain while only the connected background becomes alpha. Final VFX counts are 5 Dizzy Punch `impact` frames, 6 Rain Dance `field` frames, 6 Dragon Cheer `aura` frames, 6 Life Dew `pickup` frames, and 6 Life Dew heal `burst` frames. Keep these VFX separate from the Goodra champion body.

## Grapploct Source Mapping

- PMDCollab source: `sprite/0853`, from `https://sprites.pmdcollab.org/#/0853?form=0`.
- PMDCollab credits: one `CC_BY-NC_4` contributor row in `assets\custom_spritework\references\pmdcollab\0853\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/SpAttack/RearUp/Swing/Double/Rotate/Hop.
- `idle`: `Idle`
- `run`: first five frames of `Attack` (`--frame-select run=0-4`, inclusive), not PMDCollab `Walk`, because the source Walk is a crawl that does not fit the champion run read. Keep this as a compact repeated pre-punch fake walk, slowed to a 0.7s loop so it does not flicker like a two-frame warp.
- `attack` / Comet Punch: `RearUp`, which reads as a multi-arm punch.
- `skill` / Detect: `Charge`
- `skill2` / Submission: `Hop`, with reduced travel so the landing/grab continues slightly into the target.
- `ult` / Octolock: late `Hop` frames only (`5-9` source frame range via `--frame-select ult=5-10`) with travel removed, so Grapploct stays grounded and reads as a grab/lock instead of a jump.
- `dead`: `Hurt+Sleep`
- Boxer passive does not need separate spritework.
- No separate VFX are staged for this pass.

Grapploct conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0853 `
  --source-dir assets\custom_spritework\references\pmdcollab\0853 `
  --output-base assets\custom_spritework\champions\grapploct `
  --map idle=Idle `
  --map run=Attack `
  --map attack=RearUp `
  --map skill=Charge `
  --map skill2=Hop `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --frame-select run=0-4 `
  --frame-select ult=5-10 `
  --tag-duration run=0.7 `
  --tag-duration attack=0.6 `
  --tag-duration skill=0.6 `
  --tag-duration skill2=0.75 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale run=0.2 `
  --travel-scale skill2=0.65 `
  --travel-scale ult=0.0 `
  --recenter-tag run `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 52 `
  --bottom-padding 28
```

The converter may need the full PMDCollab `0853` cache present because its scaling pass inspects all animation metadata, even if a tag is not mapped. Keep the cached Walk/Shoot/Swing/Double/Rotate files under `assets\custom_spritework\references\pmdcollab\0853` even though Grapploct's staged run deliberately avoids Walk.

## Greninja Source Mapping

- PMDCollab source: `sprite/0658`, from `https://sprites.pmdcollab.org/#/0658?form=0`.
- PMDCollab credits: JHONY_REX and later `CC_BY-NC_4` contributor rows in `assets\custom_spritework\references\pmdcollab\0658\credits.txt` cover Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/RearUp/Swing/Double/Rotate/Hop/QuickStrike.
- ImageGen VFX sources: `assets\custom_spritework\references\imagegen\greninja_vfx_source.png` and replacement `assets\custom_spritework\references\imagegen\greninja_quickattack_shuriken_v2_source.png`, with cleaned alpha sources preserved beside them.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Night Slash: `Attack`, with reduced travel because this is a melee basic. A separate future-ready dark slash impact VFX is staged as `assets\custom_spritework\vfx\greninja_basic_night_slash_impact#sheet.png` and `#anim.fanim`.
- `skill` / Quick Attack: `QuickStrike`, preserving most of its dash read. A separate future-ready left-to-right afterimage dash VFX is staged as `assets\custom_spritework\vfx\greninja_skill1_quick_attack_streak#sheet.png` and `#anim.fanim`; use the v2 ImageGen source so this reads as Greninja moving fast, not as a launched projectile.
- `skill2` / Water Pledge: `Rotate`. A separate future-ready centered water AoE burst is staged as `assets\custom_spritework\vfx\greninja_skill2_water_pledge_aoe#sheet.png` and `#anim.fanim`.
- `ult` / Water Shuriken: `Shoot`. A separate future-ready left-to-right spinning water shuriken projectile is staged as `assets\custom_spritework\vfx\greninja_ult_water_shuriken_projectile#sheet.png` and `#anim.fanim`; use the v2 ImageGen source so the shuriken visibly rotates as it travels.
- `dead`: `Hurt+Sleep`
- Battle Bond passive does not need separate spritework.

Greninja conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0658 `
  --source-dir assets\custom_spritework\references\pmdcollab\0658 `
  --output-base assets\custom_spritework\champions\greninja `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=QuickStrike `
  --map skill2=Rotate `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4667 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.6 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.45 `
  --travel-scale skill=0.75 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 46 `
  --bottom-padding 28
```

When packing `greninja_vfx_source.png`, use border-flood cleanup tuned for the generated magenta background, not exact chroma replacement. The first Quick Attack pass read too much like a projectile and the first Water Shuriken pass did not spin enough, so final staged skill/ult VFX use `greninja_quickattack_shuriken_v2_source.png`. Final VFX counts are 5 Night Slash `impact` frames, 6 Quick Attack afterimage `streak` frames, 6 Water Pledge `aoe` frames, and 6 spinning Water Shuriken `projectile` frames. Keep these VFX separate from the Greninja champion body.

## Gyarados Source Mapping

- PMDCollab source: `sprite/0130`, from `https://sprites.pmdcollab.org/#/0130?form=0`.
- PMDCollab credits: `CHUNSOFT` in `assets\custom_spritework\references\pmdcollab\0130\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Rumble/RearUp/Swing/Double/Rotate/Hop.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\gyarados_vfx_source.png`, with cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\gyarados_vfx_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Whirlpool: `RearUp`. A separate future-ready centered whirlpool field VFX is staged as `assets\custom_spritework\vfx\gyarados_basic_whirlpool_field#sheet.png` and `#anim.fanim`.
- `skill` / Scald: `Shoot`. A separate future-ready left-to-right boiling water projectile/splash VFX is staged as `assets\custom_spritework\vfx\gyarados_skill1_scald_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Hurricane: `Hop`. A separate future-ready left-to-right wind wall VFX is staged as `assets\custom_spritework\vfx\gyarados_skill2_hurricane_wave#sheet.png` and `#anim.fanim`.
- `ult` / Hyper Beam: `Charge`. A separate future-ready left-to-right beam line VFX is staged as `assets\custom_spritework\vfx\gyarados_ult_hyper_beam_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Lake of Rage passive does not need separate spritework.

Gyarados conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0130 `
  --source-dir assets\custom_spritework\references\pmdcollab\0130 `
  --output-base assets\custom_spritework\champions\gyarados `
  --map idle=Idle `
  --map run=Walk `
  --map attack=RearUp `
  --map skill=Shoot `
  --map skill2=Hop `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.6333 `
  --tag-duration skill=0.6 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=1.5333 `
  --tag-duration dead=1.0 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 62 `
  --bottom-padding 26
```

When packing `gyarados_vfx_source.png`, use border-flood cleanup tuned for the generated magenta background. Final VFX counts are 6 Whirlpool `field` frames, 6 Scald `projectile` frames, 6 Hurricane `wave` frames, and 8 Hyper Beam `line` frames. The current Hyper Beam row was generated as a continuous charge/full-beam/fade strip, so it is packed as equal slices; if more polish is needed later, regenerate that row with visibly separated individual frames.

## Hawlucha Source Mapping

- PMDCollab source: `sprite/0701`, from `https://sprites.pmdcollab.org/#/0701?form=0`.
- PMDCollab credits: `<@!356635814668664832>` under `CC_BY-NC_4` in `assets\custom_spritework\references\pmdcollab\0701\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Kick/Swing/Double/Rotate/Hop/Cringe.
- PMDCollab `AnimData.xml` defines `Shoot` as `CopyOf Charge` and `Strike` as `CopyOf Attack`, and the copied sheet files are not present upstream. Use the underlying source sheets (`Charge` and `Attack`) in converter commands instead of trying to download `Shoot-Anim.png` or `Strike-Anim.png`.
- ImageGen VFX source: `assets\custom_spritework\references\imagegen\hawlucha_air_slash_source.png`, with the cleaned alpha source preserved as `assets\custom_spritework\references\imagegen\hawlucha_air_slash_source_cleaned.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Wing Attack: `Attack`, with reduced travel because this is a melee basic and PMDCollab's `Strike` tag is an alias of `Attack`.
- `skill` / Air Slash: `Charge`, because PMDCollab's `Shoot` tag is an alias of `Charge`. A separate future-ready left-to-right blue-white air slash cone/wave VFX is staged as `assets\custom_spritework\vfx\hawlucha_skill1_air_slash_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Counter: `Charge`
- `ult` / Flying Press: `Attack`, preserving most of the source travel so the action reads as an airborne flying dash. The initial `Hop` pass was rejected because it barely changed pose and did not read as Flying Press in the GIF preview.
- `dead`: `Hurt+Sleep`
- Momentum passive does not need separate spritework.

Hawlucha conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0701 `
  --source-dir assets\custom_spritework\references\pmdcollab\0701 `
  --output-base assets\custom_spritework\champions\hawlucha `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Charge `
  --map skill2=Charge `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.45 `
  --tag-duration skill=0.4667 `
  --tag-duration skill2=1.0 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.4 `
  --travel-scale ult=0.85 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --max-content-size 46 `
  --bottom-padding 28
```

When packing `hawlucha_air_slash_source.png`, use magenta border-flood cleanup, then recolor any generated purple/magenta wind accents to blue-white before preview validation. Final VFX count is 6 Air Slash `wave` frames. Keep this VFX separate from the Hawlucha champion body.

## Heliolisk Source Mapping

- PMDCollab source: `sprite/0695`, from `https://sprites.pmdcollab.org/#/0695?form=0`.
- PMDCollab credits: `<@!356635814668664832>` under `PMDCollab_2` in `assets\custom_spritework\references\pmdcollab\0695\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Shock/Swing/Double/Rotate/Hop/Cringe.
- ImageGen VFX sources: `assets\custom_spritework\references\imagegen\heliolisk_shed_tail_decoy_source.png` and `assets\custom_spritework\references\imagegen\heliolisk_discharge_aura_source.png`, with cleaned alpha sources preserved beside them.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Scratch: `Attack`, with reduced travel because this is a melee basic.
- `skill` / Swift: `Attack`, with more travel preserved because Rust implements it as a line dash pierce.
- `skill2` / Shed Tail: `Walk`. A separate future-ready dropped tail decoy VFX is staged as `assets\custom_spritework\vfx\heliolisk_skill2_shed_tail_decoy#sheet.png` and `#anim.fanim`.
- `ult` / Parabolic Charge: `Shock`; no additional ult VFX is staged because PMDCollab's Shock animation already reads as a self electric burst.
- `dead`: `Hurt+Sleep`
- Discharge passive has a separate future-ready small electric pulse aura staged as `assets\custom_spritework\vfx\heliolisk_passive_discharge_aura#sheet.png` and `#anim.fanim`.

Heliolisk conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0695 `
  --source-dir assets\custom_spritework\references\pmdcollab\0695 `
  --output-base assets\custom_spritework\champions\heliolisk `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Attack `
  --map skill2=Walk `
  --map ult=Shock `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.3 `
  --tag-duration skill=0.3 `
  --tag-duration skill2=0.3333 `
  --tag-duration ult=0.7333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.4 `
  --travel-scale skill=0.85 `
  --recenter-tag attack `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 48 `
  --bottom-padding 28
```

When packing Heliolisk ImageGen VFX, use magenta border-flood cleanup. Final VFX counts are 5 Shed Tail `decoy` frames and 6 Discharge passive `aura` frames. If the generated Shed Tail taunt accents drift toward magenta, recolor those accents to pale yellow-white before preview validation.

## Hitmonchan Source Mapping

- PMDCollab source: `sprite/0107`, from `https://sprites.pmdcollab.org/#/0107?form=0`.
- PMDCollab credits: `CHUNSOFT` in `assets\custom_spritework\references\pmdcollab\0107\credits.txt` covers Walk/Attack/Punch/Shoot/Uppercut/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen reference source: `assets\custom_spritework\references\imagegen\hitmonchan_typed_punch_accents_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Drain Punch: `Punch`, kept clean with no baked VFX.
- `skill` / Ice Punch: `Punch`, with subtle sprite-baked cyan frost/glove accents on the active punch frames and a lightly recolored icy impact slash.
- `skill2` / Fire Punch: `Punch`, with subtle sprite-baked orange flame/glove accents on the active punch frames and a lightly recolored fiery impact slash.
- `ult` / Thunder Punch: `Uppercut`, with subtle sprite-baked yellow electric glove sparks on the active uppercut frames.
- `dead`: `Hurt+Sleep`
- Iron Fist passive does not need separate spritework.

Hitmonchan conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0107 `
  --source-dir assets\custom_spritework\references\pmdcollab\0107 `
  --output-base assets\custom_spritework\champions\hitmonchan `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Punch `
  --map skill=Punch `
  --map skill2=Punch `
  --map ult=Uppercut `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=0.6333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.35 `
  --travel-scale skill2=0.35 `
  --travel-scale ult=0.45 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 44 `
  --bottom-padding 28
```

The ImageGen pass for Hitmonchan should stay as a style/reference pass for glove-local typed cues, not as a full body replacement. Apply the final baked effects to the PMDCollab frames with a tight red-glove color mask so the tan body and hair do not tint. Keep all typed effects inside `assets\custom_spritework\champions\hitmonchan#sheet.png`; these are not projectile or area VFX.

## Hitmonlee Source Mapping

- PMDCollab source: `sprite/0106`, from `https://sprites.pmdcollab.org/#/0106?form=0`.
- PMDCollab credits: `CHUNSOFT` in `assets\custom_spritework\references\pmdcollab\0106\credits.txt` covers Walk/Attack/Kick/Shoot/Withdraw/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Double Kick: custom recombination of `Kick` frames. Use only the early prep frames plus the later clean outward kick frame, then reverse toward standing and repeat for the second hit. Avoid the full spin/sweep section from the source Kick action.
- `skill` / Low Sweep: full `Kick` animation.
- `skill2` / High Jump Kick: custom recombination of `Hop` and `Kick`. Use Hop for takeoff/landing, then splice the clean outward Kick frame into the apex, shifted upward so it reads as an airborne kick.
- `ult` / Axe Kick: custom recombination of `Attack` and `Kick`. Use Attack windup, Kick's outward strike/recovery, then Attack recovery so it reads as a committed outward kick instead of the generic Attack body motion.
- `dead`: `Hurt+Sleep`
- Reckless passive does not need separate spritework.
- No ImageGen assets or detached VFX are staged for Hitmonlee in this pass.

Hitmonlee baseline conversion command before the custom repack:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0106 `
  --source-dir assets\custom_spritework\references\pmdcollab\0106 `
  --output-base assets\custom_spritework\champions\hitmonlee `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Kick `
  --map skill=Kick `
  --map skill2=Hop `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.45 `
  --travel-scale skill2=0.55 `
  --travel-scale ult=0.45 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 48 `
  --bottom-padding 28
```

After conversion, repack `assets\custom_spritework\champions\hitmonlee#sheet.png` and `#anim.fanim` from the normalized frames. The staged pass uses Double Kick frame indices `[0,1,2,3,9,9,10,11,12,13,1,2,3,9,9,10,11,12,13]` from Kick so both hits include a brief held kick and recovery instead of clipping off immediately after impact; High Jump Kick uses Hop frames with shifted Kick frame `9` at the apex; Axe Kick uses Attack frames `[0,1,2,3]`, Kick frames `[7,8,9,10]`, then Attack frames `[5,6,7,9]`.

## Hitmontop Source Mapping

- PMDCollab source: `sprite/0237`, from `https://sprites.pmdcollab.org/#/0237?form=0`.
- PMDCollab credits: `CHUNSOFT` in `assets\custom_spritework\references\pmdcollab\0237\credits.txt` covers Walk/Attack/QuickStrike/Shoot/Twirl/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Body Press: `Charge`
- `skill` / Low Kick: `Shoot`
- `skill2` / Feint: `Double`
- `ult` / Triple Kick: `Attack`
- `dead`: `Hurt+Sleep`
- No ImageGen assets or detached VFX are staged for Hitmontop in this pass.

Hitmontop conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0237 `
  --source-dir assets\custom_spritework\references\pmdcollab\0237 `
  --output-base assets\custom_spritework\champions\hitmontop `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Charge `
  --map skill=Shoot `
  --map skill2=Double `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.3667 `
  --tag-duration skill=0.4333 `
  --tag-duration skill2=0.4333 `
  --tag-duration ult=0.7 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.35 `
  --travel-scale skill2=0.35 `
  --travel-scale ult=0.65 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

## Houndoom Source Mapping

- PMDCollab source: `sprite/0229`, from `https://sprites.pmdcollab.org/#/0229?form=0`.
- PMDCollab credits: `CHUNSOFT` in `assets\custom_spritework\references\pmdcollab\0229\credits.txt` covers Walk/Attack/Strike/Shoot/RearUp/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen VFX row sources are preserved as `assets\custom_spritework\references\imagegen\houndoom_ember_row.png`, `houndoom_foul_play_row.png`, `houndoom_howl_row.png`, and `houndoom_inferno_row.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Ember: `Shoot`. A separate future-ready left-to-right fire ember projectile is staged as `assets\custom_spritework\vfx\houndoom_basic_ember_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Foul Play: `Shoot`. A separate future-ready left-to-right dark shadow-fire projectile is staged as `assets\custom_spritework\vfx\houndoom_skill1_foul_play_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Howl: `RearUp`. A separate future-ready left-to-right jagged sound-wave debuff VFX is staged as `assets\custom_spritework\vfx\houndoom_skill2_howl_debuff_pulse#sheet.png` and `#anim.fanim`.
- `ult` / Inferno: `Charge`. A separate future-ready circular ground fire field is staged as `assets\custom_spritework\vfx\houndoom_ult_inferno_field#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Intimidate: no separate VFX is staged; the passive aura is unnecessary for this pass.

Houndoom conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0229 `
  --source-dir assets\custom_spritework\references\pmdcollab\0229 `
  --output-base assets\custom_spritework\champions\houndoom `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=RearUp `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.5 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=0.8333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.35 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 50 `
  --bottom-padding 28
```

Final VFX counts are 6 Ember `projectile` frames, 6 Foul Play `projectile` frames, 6 Howl `wave` frames, and 8 Inferno `field` frames. The generated field/wave rows may include internal chroma green holes; after packing, remove all green-dominant pixels from the final Houndoom VFX sheets because none of these effects use green as intentional art. Generate per-effect GIF previews for Houndoom VFX because the generic champion preview renderer only handles champion tags.

## Inteleon Source Mapping

- PMDCollab source: `sprite/0818`, from `https://sprites.pmdcollab.org/#/0818?form=0`.
- PMDCollab credits: `<@!216640380408430592>` under `PMDCollab_2` in `assets\custom_spritework\references\pmdcollab\0818\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Swing/Double/Rotate/Hop.
- ImageGen VFX sources: `assets\custom_spritework\references\imagegen\inteleon_vfx_source.png` for Rain Dance/Soak and `assets\custom_spritework\references\imagegen\inteleon_watergun_snipeshot_v2_source.png` for the corrected Water Gun/Snipe Shot strips. Cleaned alpha sources are preserved beside them.
- ImageGen VFX row sources are preserved as `assets\custom_spritework\references\imagegen\inteleon_water_gun_row.png`, `inteleon_rain_dance_row.png`, `inteleon_soak_row.png`, and `inteleon_snipe_shot_row.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Water Gun: `Shoot`, with reduced travel. A separate future-ready left-to-right narrow water stream projectile is staged as `assets\custom_spritework\vfx\inteleon_basic_water_gun_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Rain Dance: `Double`, with reduced horizontal travel. A separate future-ready caster-centered rain aura is staged as `assets\custom_spritework\vfx\inteleon_skill1_rain_dance_aura#sheet.png` and `#anim.fanim`.
- `skill2` / Soak: `Charge`, with reduced travel. A separate future-ready water field/puddle VFX is staged as `assets\custom_spritework\vfx\inteleon_skill2_soak_field#sheet.png` and `#anim.fanim`.
- `ult` / Snipe Shot: `Shoot`, with reduced travel. A separate future-ready left-to-right very thin piercing line VFX is staged as `assets\custom_spritework\vfx\inteleon_ult_snipe_shot_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Sniper: no separate spritework or VFX is staged.

Inteleon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0818 `
  --source-dir assets\custom_spritework\references\pmdcollab\0818 `
  --output-base assets\custom_spritework\champions\inteleon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Double `
  --map skill2=Charge `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.45 `
  --tag-duration skill=0.6 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=0.9667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.35 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 50 `
  --bottom-padding 28
```

Final VFX counts are 6 Water Gun `projectile` frames, 6 Rain Dance `aura` frames, 6 Soak `field` frames, and 7 Snipe Shot `line` frames. Keep Water Gun and Snipe Shot left-to-right. Water Gun should read as a pressurized stream, not a large water blob; Snipe Shot should stay a map-wide, sniper-thin line and is packed at 192x32 to avoid a bulky projectile read. Generate per-effect GIF previews for Inteleon VFX because the generic champion preview renderer only handles champion tags.

## Jolteon Source Mapping

- PMDCollab source: `sprite/0135`, from `https://sprites.pmdcollab.org/#/0135?form=0`.
- PMDCollab credits: `CHUNSOFT` plus `<@!474262233442942995>` under `PMDCollab_1` in `assets\custom_spritework\references\pmdcollab\0135\credits.txt` cover Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Shock/Wake and related action tags.
- ImageGen VFX sources: corrected v2 sources are preserved as `assets\custom_spritework\references\imagegen\jolteon_pin_missile_v2_source.png`, `jolteon_sand_attack_v2_source.png`, `jolteon_thunder_shock_v2_source.png`, and `jolteon_thunder_fang_v2_source.png`. The first Pin Missile/Sand Attack/Thunder Fang pass was replaced because Pin Missile read as a thick energy bolt, Sand Attack read as a dirt clump, and Thunder Fang cropped the upper bite with white artifacts.
- Older ImageGen VFX row sources from the discarded pass are still present as `assets\custom_spritework\references\imagegen\jolteon_pin_missile_row.png`, `jolteon_sand_attack_row.png`, `jolteon_thunder_shock_row.png`, and `jolteon_thunder_fang_row.png`; do not use them for future packing.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Pin Missile: `Shoot`, with reduced travel. A separate future-ready reusable left-to-right Pin Missile projectile is staged as `assets\custom_spritework\vfx\jolteon_basic_pin_missile_projectile#sheet.png` and `#anim.fanim`; the Rust move is three independent hits, so this one projectile can be fired repeatedly later.
- `skill` / Sand Attack: `Wake`, using only source frames `3-5` from the preferred right-facing row, as requested. A separate future-ready left-to-right sand cloud/debuff puff is staged as `assets\custom_spritework\vfx\jolteon_skill1_sand_attack_cloud#sheet.png` and `#anim.fanim`.
- `skill2` / Thunder Shock: `Shock`. A separate future-ready left-to-right electric bolt projectile is staged as `assets\custom_spritework\vfx\jolteon_skill2_thunder_shock_projectile#sheet.png` and `#anim.fanim`. The first ImageGen row packed too weak/thin, so the final staged asset uses the green-key v2 replacement source.
- `ult` / Thunder Fang: `Attack`, with reduced travel because the Rust move already handles dash movement. A separate future-ready centered electric fang impact is staged as `assets\custom_spritework\vfx\jolteon_ult_thunder_fang_impact#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Eeveelution: no separate spritework or VFX is staged.

Jolteon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0135 `
  --source-dir assets\custom_spritework\references\pmdcollab\0135 `
  --output-base assets\custom_spritework\champions\jolteon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Wake `
  --map skill2=Shock `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.4667 `
  --tag-duration skill2=0.4333 `
  --tag-duration ult=0.5667 `
  --tag-duration dead=1.0 `
  --frame-select skill=3-5 `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.1 `
  --travel-scale skill2=0.15 `
  --travel-scale ult=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 46 `
  --bottom-padding 28
```

After conversion, normalize Jolteon's action durations to the Rust move timings because Shock and Attack can exceed the requested totals under the converter minimum frame duration. Final VFX counts are 6 Pin Missile `projectile` frames, 6 Sand Attack `cloud` frames, 6 Thunder Shock `projectile` frames, and 6 Thunder Fang `impact` frames. Use the v2 VFX pass as the visual target: Pin Missile should be thin cream quills with small electric accents, Sand Attack should be loose tan particle spray rather than a dirt ball, and Thunder Fang should show the complete upper and lower electric bite without cropping. Validate packed sheets and GIF previews for green/magenta chroma residue before treating the assets as final.

## Kilowattrel Source Mapping

- PMDCollab source: `sprite/0941`, from `https://sprites.pmdcollab.org/#/0941?form=0`.
- PMDCollab credits: `<@!229131140903075840>` and `<@!642311628557385738>` under `CC_BY-NC_4` in `assets\custom_spritework\references\pmdcollab\0941\credits.txt` cover Idle/Walk/Attack/Strike/FlapAround/Hop plus Sleep/Hurt/Charge/Shoot/Swing/Double/Rotate.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\kilowattrel_spark_vfx_source.png`, `kilowattrel_electro_ball_vfx_source.png`, `kilowattrel_gust_vfx_source.png`, and `kilowattrel_charge_aura_vfx_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Spark: `Shoot`, with reduced travel. A separate future-ready left-to-right compact electric projectile is staged as `assets\custom_spritework\vfx\kilowattrel_basicattack_spark_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Electro Ball: `Shoot`, with reduced travel. A separate future-ready left-to-right electric orb projectile is staged as `assets\custom_spritework\vfx\kilowattrel_skill1_electro_ball_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Gust: `FlapAround`, with reduced travel. A separate future-ready left-to-right wind line is staged as `assets\custom_spritework\vfx\kilowattrel_skill2_gust_line#sheet.png` and `#anim.fanim`; keep this visually Flying-type, not electric.
- `ult` / Charge: `Charge`, with reduced travel. A separate future-ready caster-centered electric self-area aura is staged as `assets\custom_spritework\vfx\kilowattrel_ult_charge_aura#sheet.png` and `#anim.fanim`; this is not a projectile.
- `dead`: `Hurt+Sleep`
- Passive / Volt Absorb: no separate spritework or VFX is staged.
- Spark uses only Shoot frames `0-10` so the body animation can fit the 0.4s Rust timing under the converter minimum frame duration.

Kilowattrel conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0941 `
  --source-dir assets\custom_spritework\references\pmdcollab\0941 `
  --output-base assets\custom_spritework\champions\kilowattrel `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=FlapAround `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.6333 `
  --tag-duration ult=1.1667 `
  --tag-duration dead=1.0 `
  --frame-select attack=0-10 `
  --travel-scale attack=0.2 `
  --travel-scale skill=0.2 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.15 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

Final VFX counts are 6 Spark `projectile` frames, 6 Electro Ball `projectile` frames, 6 Gust `line` frames, and 6 Charge `aura` frames. Spark should stay small and fast-looking, Electro Ball should be a distinct orb with a trail, Gust should remain a slim pale wind line, and Charge should be a centered ground/aura ring. Validate packed sheets and GIF previews for green/magenta chroma residue before treating the assets as final.

## Kingdra Source Mapping

- PMDCollab source: `sprite/0230`, from `https://sprites.pmdcollab.org/#/0230?form=0`.
- PMDCollab credits: `CHUNSOFT` in `assets\custom_spritework\references\pmdcollab\0230\credits.txt` covers Walk/Attack/Strike/Shoot/RearUp/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\kingdra_scale_shot_single_scale_v2_source.png`, `kingdra_waterfall_vfx_source.png`, `kingdra_dragon_dance_vfx_source.png`, and `kingdra_dragon_energy_vfx_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Scale Shot: `Shoot`, with reduced travel and source frames `0-9`. A separate future-ready left-to-right single dragon-scale projectile is staged as `assets\custom_spritework\vfx\kingdra_basicattack_scale_shot_projectile#sheet.png` and `#anim.fanim`; keep it as one fired scale per frame, not a pin missile, quill volley, or multi-shard cluster.
- `skill` / Waterfall: `RearUp`, with reduced travel and source frames `0-8`. A separate future-ready target-centered Waterfall AoE is staged as `assets\custom_spritework\vfx\kingdra_skill1_waterfall_aoe#sheet.png` and `#anim.fanim`.
- `skill2` / Dragon Dance: `Double`, with reduced travel and source frames `0-12`. A separate future-ready caster-centered Dragon Dance aura is staged as `assets\custom_spritework\vfx\kingdra_skill2_dragon_dance_aura#sheet.png` and `#anim.fanim`.
- `ult` / Dragon Energy: `Shoot`, with reduced travel and source frames `0-9`. A separate future-ready left-to-right Dragon Energy projectile is staged as `assets\custom_spritework\vfx\kingdra_ult_dragon_energy_projectile#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / King's Rock: no separate spritework or VFX is staged.

Kingdra conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0230 `
  --source-dir assets\custom_spritework\references\pmdcollab\0230 `
  --output-base assets\custom_spritework\champions\kingdra `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=RearUp `
  --map skill2=Double `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6 `
  --tag-duration skill2=0.5 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0 `
  --frame-select attack=0-9 `
  --frame-select skill=0-8 `
  --frame-select skill2=0-12 `
  --frame-select ult=0-9 `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.15 `
  --travel-scale skill2=0.3 `
  --travel-scale ult=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 28
```

Final VFX counts are 6 Scale Shot `projectile` frames, 6 Waterfall `aoe` frames, 6 Dragon Dance `aura` frames, and 6 Dragon Energy `projectile` frames. Dragon Energy's ImageGen source put neighboring frames close together, so the final sheet was repacked from equal source cells and had detached right-edge components stripped from the packed frames. Scale Shot was regenerated from `kingdra_scale_shot_single_scale_v2_source.png` because the first version read like a recolored blue pin missile; keep the final as a small blue-purple single dragon scale with no volley or heavy trail. Keep Waterfall as a target-position AoE rather than a projectile, Dragon Dance as a self-buff swirl, and Dragon Energy as the larger Dragon attack. Validate packed sheets and GIF previews for green/magenta chroma residue before treating the assets as final.

## Kleavor Source Mapping

- PMDCollab source: `sprite/0900`, from `https://sprites.pmdcollab.org/#/0900?form=0`.
- PMDCollab credits: current `CC_BY-NC_4` in `assets\custom_spritework\references\pmdcollab\0900\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Slice/Swing/Double/Rotate/Hop/QuickStrike.
- ImageGen baked accent source is preserved as `assets\custom_spritework\references\imagegen\kleavor_ult_slash_accent_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / basic attack: `Attack`, trimmed to source frames `0-6,12` so it lands as a short axe hit and recovery.
- `skill` / skill 1: `Attack`, using source frames `0-12` so it plays more of the axe-swinging attack sequence than the basic.
- `skill2`: `Charge`, reduced travel and recentered.
- `ult`: `QuickStrike`, not recentered so dash motion survives, but with travel scaled to `0.4` so all frames remain visible inside the 96x96 sprite cell. A small ImageGen stone-gold slash accent is baked directly into the `ult` frames in `assets\custom_spritework\champions\kleavor#sheet.png`; do not treat this as a separate projectile or VFX asset.
- `dead`: `Hurt+Sleep`

Kleavor conversion command before the baked ult accent step:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0900 `
  --source-dir assets\custom_spritework\references\pmdcollab\0900 `
  --output-base assets\custom_spritework\champions\kleavor `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Attack `
  --map skill2=Charge `
  --map ult=QuickStrike `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0 `
  --frame-select attack=0-6,12 `
  --frame-select skill=0-12 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.45 `
  --travel-scale skill2=0.2 `
  --travel-scale ult=0.4 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --max-content-size 42 `
  --bottom-padding 28
```

Kleavor has no separate staged VFX files. The ult accent was generated on chroma green, locally keyed, scaled down, and composited into the champion sheet's `ult` frames. Validate the body sheet and GIF previews for green/magenta chroma residue after any rebuild.

## Kommo-o Source Mapping

- PMDCollab source: `sprite/0784`, from `https://sprites.pmdcollab.org/#/0784?form=0`.
- PMDCollab credits: current `PMDCollab_1` in `assets\custom_spritework\references\pmdcollab\0784\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Swing/Double/Rotate/Hop.
- ImageGen baked accent source is preserved as `assets\custom_spritework\references\imagegen\kommo-o_noble_roar_baked_accent_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Dragon Claw: `Strike`, with reduced travel and recentering.
- `skill` / Noble Roar: `Shoot`, with reduced travel and recentering. A small ImageGen pale-gold roar/sound-wave accent is baked directly into the `skill` frames in `assets\custom_spritework\champions\kommo-o#sheet.png`; do not treat this as a separate projectile or VFX asset.
- `skill2` / Clangorous Soul: `Charge`, with reduced travel and recentering.
- `ult` / Close Combat: `Shoot`, with reduced travel and recentering.
- `dead`: `Hurt+Sleep`
- Passive / Overcoat: no separate spritework or VFX is staged.

Kommo-o conversion command before the baked Noble Roar accent step:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0784 `
  --source-dir assets\custom_spritework\references\pmdcollab\0784 `
  --output-base assets\custom_spritework\champions\kommo-o `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Shoot `
  --map skill2=Charge `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.2 `
  --travel-scale skill2=0.2 `
  --travel-scale ult=0.25 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 44 `
  --bottom-padding 26
```

Kommo-o has no separate staged VFX files. The Noble Roar accent was generated on chroma green, locally keyed, scaled down, and composited into the champion sheet's `skill` frames. Validate the body sheet and GIF previews for green/magenta chroma residue after any rebuild.

## Kricketune Source Mapping

- PMDCollab source: `sprite/0402`, from `https://sprites.pmdcollab.org/#/0402?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` in `assets\custom_spritework\references\pmdcollab\0402\credits.txt` covers Walk/Attack/MultiStrike/Shoot/Strike/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen Sticky Web deployable source is preserved as `assets\custom_spritework\references\imagegen\kricketune_sticky_web_deployable_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Lunge: `Attack`, with reduced melee travel and recentering.
- `skill` / Sticky Web: `Shoot`, with reduced travel and recentering. A separate future-ready Sticky Web ground deployable is staged as `assets\custom_spritework\vfx\kricketune_skill1_sticky_web_deployable#sheet.png` and `#anim.fanim`; keep it as an arena-floor field/deployable, not a projectile and not baked into Kricketune's body.
- `skill2` / Sing: `Double`, with reduced travel and recentering.
- `ult` / Bug Buzz: `Charge`, with reduced travel and recentering.
- `dead`: `Hurt+Sleep`
- Passive / Web Walker: no separate spritework is staged. It should visually rely on the Sticky Web deployable where needed.

Kricketune conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0402 `
  --source-dir assets\custom_spritework\references\pmdcollab\0402 `
  --output-base assets\custom_spritework\champions\kricketune `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Shoot `
  --map skill2=Double `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.8667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.2 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 26
```

Final VFX count is 6 Sticky Web `field` frames. The first chroma pass left green panels inside the packed frames, so the final pack uses direct green-key removal with higher tolerance. Validate the packed body/VFX sheets and GIF previews for green/magenta chroma residue before treating the assets as final.

## Leafeon Source Mapping

- PMDCollab source: `sprite/0470`, from `https://sprites.pmdcollab.org/#/0470?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` and `PMDCollab_1` in `assets\custom_spritework\references\pmdcollab\0470\credits.txt` cover Idle/Walk/Shoot/Charge/Hurt/Sleep and related action sets.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\leafeon_basicattack_razor_leaf_source.png`, `leafeon_skill1_magical_leaf_source.png`, `leafeon_skill2_leaf_blade_source.png`, and `leafeon_ult_leaf_storm_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Razor Leaf: `Shoot`, with reduced travel and recentering. A separate future-ready twin-leaf projectile is staged as `assets\custom_spritework\vfx\leafeon_basicattack_razor_leaf_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Magical Leaf: `Shoot`, with reduced travel and recentering. A separate future-ready single glowing leaf projectile is staged as `assets\custom_spritework\vfx\leafeon_skill1_magical_leaf_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Leaf Blade: `Shoot`, with reduced travel and recentering. A separate future-ready close-range leaf crescent impact is staged as `assets\custom_spritework\vfx\leafeon_skill2_leaf_blade_slash#sheet.png` and `#anim.fanim`.
- `ult` / Leaf Storm: `Charge`, with reduced travel and recentering. A separate future-ready circular leaf storm AoE is staged as `assets\custom_spritework\vfx\leafeon_ult_leaf_storm_aoe#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Eeveelution: no separate spritework or VFX is staged.

Leafeon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0470 `
  --source-dir assets\custom_spritework\references\pmdcollab\0470 `
  --output-base assets\custom_spritework\champions\leafeon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.5333 `
  --tag-duration ult=0.8667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.2 `
  --travel-scale skill=0.2 `
  --travel-scale skill2=0.2 `
  --travel-scale ult=0.2 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 38 `
  --bottom-padding 28
```

Final VFX counts are 6 Razor Leaf `projectile` frames, 6 Magical Leaf `projectile` frames, 6 Leaf Blade `impact` frames, and 6 Leaf Storm `aoe` frames. Because Leafeon's VFX are green leaf effects, ImageGen sources used magenta chroma key. The final packed sheets include a cleanup pass that remaps remaining magenta/purple key-fringe hues into yellow-green leaf highlights; keep that cleanup if the VFX are repacked from the original ImageGen sources. Validate packed sheets and GIF previews for green/magenta chroma residue before treating the assets as final.

## Ludicolo Source Mapping

- PMDCollab source: `sprite/0272`, from `https://sprites.pmdcollab.org/#/0272?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` in `assets\custom_spritework\references\pmdcollab\0272\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Emit/Swing/Double/Rotate/Hop/QuickStrike.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\ludicolo_basicattack_facade_impact_source.png`, `ludicolo_skill1_teeter_dance_cone_source.png`, `ludicolo_skill2_grass_whistle_aura_source.png`, `ludicolo_ult_ivy_cudgel_impact_source.png`, and `ludicolo_passive_rain_dish_aura_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Facade: `Attack`, with reduced melee travel and recentering. A separate future-ready normal-type impact is staged as `assets\custom_spritework\vfx\ludicolo_basicattack_facade_impact#sheet.png` and `#anim.fanim`.
- `skill` / Teeter Dance: `Rotate`, with reduced travel and recentering. A separate future-ready right-opening dance/confusion cone is staged as `assets\custom_spritework\vfx\ludicolo_skill1_teeter_dance_cone#sheet.png` and `#anim.fanim`.
- `skill2` / Grass Whistle: `Emit`, with reduced travel and recentering. A separate future-ready caster-centered sleep/music aura is staged as `assets\custom_spritework\vfx\ludicolo_skill2_grass_whistle_aura#sheet.png` and `#anim.fanim`.
- `ult` / Ivy Cudgel: `Hop`, with reduced travel and recentering. A separate future-ready water/grass cudgel impact is staged as `assets\custom_spritework\vfx\ludicolo_ult_ivy_cudgel_impact#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Rain Dish: no body animation is staged, but a separate follower rain aura is staged as `assets\custom_spritework\vfx\ludicolo_passive_rain_dish_aura#sheet.png` and `#anim.fanim`. This should follow Ludicolo as a compact self-centered rain cloud/ring, not behave like a map-wide storm.

Ludicolo conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0272 `
  --source-dir assets\custom_spritework\references\pmdcollab\0272 `
  --output-base assets\custom_spritework\champions\ludicolo `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Rotate `
  --map skill2=Emit `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.7333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.7333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.2 `
  --travel-scale skill2=0.15 `
  --travel-scale ult=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 42 `
  --bottom-padding 26
```

Final VFX counts are 6 Facade `impact` frames, 6 Teeter Dance `cone` frames, 6 Grass Whistle `aura` frames, 6 Ivy Cudgel `impact` frames, and 6 Rain Dish `aura` frames. Ludicolo's VFX sources use magenta chroma key because several effects use grass-green or water-blue color ranges. Validate packed sheets and GIF previews for green/magenta chroma residue before treating the assets as final.

## Magmortar Source Mapping

- PMDCollab source: `sprite/0467`, from `https://sprites.pmdcollab.org/#/0467?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` in `assets\custom_spritework\references\pmdcollab\0467\credits.txt` covers Walk/Attack/Strike/Emit/SpAttack/Shoot/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\magmortar_basicattack_lava_plume_source.png`, `magmortar_skill1_temper_flare_source.png`, `magmortar_skill2_heat_wave_ring_source.png`, and `magmortar_ult_eruption_cannonball_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Lava Plume: `Shoot`, with reduced travel and recentering. A separate future-ready heavy molten projectile is staged as `assets\custom_spritework\vfx\magmortar_basicattack_lava_plume_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Temper Flare: `Shoot`, with reduced travel and recentering. A separate future-ready fire projectile plus compact splash/explosion is staged as `assets\custom_spritework\vfx\magmortar_skill1_temper_flare_projectile_splash#sheet.png` and `#anim.fanim`.
- `skill2` / Heat Wave: `Rotate`, with reduced travel and recentering. A separate future-ready caster-following fire ring aura is staged as `assets\custom_spritework\vfx\magmortar_skill2_heat_wave_ring#sheet.png` and `#anim.fanim`.
- `ult` / Eruption: `Charge`, with reduced travel and recentering. A separate future-ready lava cannonball salvo projectile is staged as `assets\custom_spritework\vfx\magmortar_ult_eruption_cannonball#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Overheat: no separate spritework or VFX is staged.

Magmortar conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0467 `
  --source-dir assets\custom_spritework\references\pmdcollab\0467 `
  --output-base assets\custom_spritework\champions\magmortar `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Rotate `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.7333 `
  --tag-duration ult=0.8667 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.2 `
  --travel-scale skill=0.2 `
  --travel-scale skill2=0.2 `
  --travel-scale ult=0.15 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 44 `
  --bottom-padding 26
```

Final VFX counts are 6 Lava Plume `projectile` frames, 6 Temper Flare `projectile` frames, 6 Heat Wave `aura` frames, and 6 Eruption `projectile` frames. The sources used green chroma key, and the final packed sheets include a cleanup pass that remaps remaining green/yellow-green key-fringe pixels into orange/yellow ember highlights. Keep that cleanup if these VFX are repacked from source, and validate packed sheets/GIF previews for green/magenta chroma residue before treating the assets as final.

## Mantine Source Mapping

- PMDCollab source: `sprite/0226`, from `https://sprites.pmdcollab.org/#/0226?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` in `assets\custom_spritework\references\pmdcollab\0226\credits.txt` covers Walk/Attack/Strike/Shoot/Hover/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\mantine_basicattack_bubble_source.png`, `mantine_skill1_brine_field_source.png`, `mantine_skill2_dive_splash_source.png`, and `mantine_ult_surf_wave_source.png`.
- `idle`: `Idle`
- `run`: `Hover`, not `Walk`, per the requested floating run animation.
- `attack` / Bubble: `Shoot`, with reduced travel and recentering. A separate future-ready bubble projectile is staged as `assets\custom_spritework\vfx\mantine_basicattack_bubble_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Brine: requested `Strike`; PMDCollab defines `Strike` as `CopyOf Attack`, so the staged body uses `Attack` with reduced travel and recentering. A separate future-ready water slow field is staged as `assets\custom_spritework\vfx\mantine_skill1_brine_water_field#sheet.png` and `#anim.fanim`.
- `skill2` / Dive: `Shoot`, with reduced travel and recentering, then a post-pass blanks source frames 4-8 in the champion sheet so Mantine disappears for the middle of the cast. A separate future-ready dive splash/ripple VFX is staged as `assets\custom_spritework\vfx\mantine_skill2_dive_splash#sheet.png` and `#anim.fanim`.
- `ult` / Surf: `Hop`, with reduced travel and recentering. A separate future-ready left-to-right Surf wave is staged as `assets\custom_spritework\vfx\mantine_ult_surf_wave#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Swift Swim: no separate spritework or VFX is staged.

Mantine conversion command before the Dive blanking post-pass:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0226 `
  --source-dir assets\custom_spritework\references\pmdcollab\0226 `
  --output-base assets\custom_spritework\champions\mantine `
  --map idle=Idle `
  --map run=Hover `
  --map attack=Shoot `
  --map skill=Attack `
  --map skill2=Shoot `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.7 `
  --tag-duration ult=0.7333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.15 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.15 `
  --travel-scale ult=0.35 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 44 `
  --bottom-padding 26
```

Final VFX counts are 6 Bubble `projectile` frames, 6 Brine `field` frames, 6 Dive `splash` frames, and 6 Surf `wave` frames. The sources used green chroma key, and the final packed sheets include a cleanup pass that remaps remaining green key-fringe pixels into blue/cyan water highlights. Keep that cleanup if these VFX are repacked from source, and validate packed sheets/GIF previews for green/magenta chroma residue before treating the assets as final.

## Noivern Source Mapping

- PMDCollab source: `sprite/0715`, from `https://sprites.pmdcollab.org/#/0715?form=0`.
- PMDCollab credits: current `CC_BY-NC_4` in `assets\custom_spritework\references\pmdcollab\0715\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Swing/Double/Rotate/Hop/Hover.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\noivern_basicattack_screech_v2_source.png`, `noivern_skill1_whirlwind_source.png`, `noivern_skill2_tailwind_source.png`, and `noivern_ult_dragon_pulse_source.png`. Use the v2 Screech source; the first Screech pass was rejected because it read like debris/impact chunks instead of sound. Noivern's former Tailwind move is now named Fighting Wings; keep the existing file stem unless the VFX asset is repacked.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Screech: `Shoot`, with reduced travel and recentering. A separate future-ready sonic debuff wave is staged as `assets\custom_spritework\vfx\noivern_basicattack_screech_soundwave#sheet.png` and `#anim.fanim`; it should read as expanding waveform arcs/concentric audio rings, not a projectile, debris wall, or impact burst.
- `skill` / Whirlwind: `Shoot`, with reduced travel and recentering. A separate future-ready heavier wind/debris line is staged as `assets\custom_spritework\vfx\noivern_skill1_whirlwind_line#sheet.png` and `#anim.fanim`.
- `skill2` / Fighting Wings: `Shoot`, with reduced travel and recentering. A separate future-ready faster, cleaner wind-speed line is staged as `assets\custom_spritework\vfx\noivern_skill2_tailwind_line#sheet.png` and `#anim.fanim`.
- `ult` / Dragon Pulse: `Shoot`, with reduced travel and recentering. A separate future-ready purple-blue dragon projectile is staged as `assets\custom_spritework\vfx\noivern_ult_dragon_pulse_projectile#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Infiltrator: no separate spritework or VFX is staged.

Noivern conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0715 `
  --source-dir assets\custom_spritework\references\pmdcollab\0715 `
  --output-base assets\custom_spritework\champions\noivern `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --tag-duration attack=0.4 `
  --tag-duration skill=0.5333 `
  --tag-duration skill2=0.5333 `
  --tag-duration ult=0.6333 `
  --tag-duration dead=1.0 `
  --travel-scale attack=0.15 `
  --travel-scale skill=0.15 `
  --travel-scale skill2=0.15 `
  --travel-scale ult=0.15 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --max-content-size 44 `
  --bottom-padding 26
```

Final VFX counts are 6 Screech `wave` frames, 6 Whirlwind `line` frames, 6 Fighting Wings `line` frames, and 6 Dragon Pulse `projectile` frames. The sources used green chroma key, and the final packed sheets include a cleanup pass that remaps remaining green key-fringe pixels into cool wind/sound/dragon highlights. Keep that cleanup if these VFX are repacked from source, and validate packed sheets/GIF previews for green/magenta chroma residue before treating the assets as final.

## Octillery Source Mapping

- PMDCollab source: `sprite/0224`, from `https://sprites.pmdcollab.org/#/0224?form=0`.
- PMDCollab credits: current `Unspecified` in `assets\custom_spritework\references\pmdcollab\0224\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Withdraw/RearUp/Swing/Double/Rotate/Hop.
- ImageGen VFX sources are preserved as `assets\custom_spritework\references\imagegen\octillery_basicattack_bullet_seed_source.png`, `octillery_skill1_lock_on_source.png`, `octillery_skill2_bouncy_bubble_source.png`, and `octillery_ult_octazooka_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Bullet Seed: `Shoot`. A separate future-ready single-seed projectile is staged as `assets\custom_spritework\vfx\octillery_basicattack_bullet_seed_projectile#sheet.png` and `#anim.fanim`; call this same projectile three times for Bullet Seed rather than baking three seeds into one sprite.
- `skill` / Lock-On: `Charge`. A separate future-ready enemy crosshair/focusing reticle is staged as `assets\custom_spritework\vfx\octillery_skill1_lock_on_reticle#sheet.png` and `#anim.fanim`.
- `skill2` / Bouncy Bubble: `Hop`. A separate future-ready three-landing path VFX is staged as `assets\custom_spritework\vfx\octillery_skill2_bouncy_bubble_path#sheet.png` and `#anim.fanim`; it should read as bounce splash, travel, bounce splash, travel, then final bounce/pop splash.
- `ult` / Octazooka: `RearUp`. A separate future-ready compact ink-water projectile is staged as `assets\custom_spritework\vfx\octillery_ult_octazooka_projectile#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Suction Cups: no separate spritework or VFX is staged.

Octillery conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0224 `
  --source-dir assets\custom_spritework\references\pmdcollab\0224 `
  --output-base assets\custom_spritework\champions\octillery `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=Hop `
  --map ult=RearUp `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 50 `
  --bottom-padding 26 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult
```

Final VFX counts are 6 Bullet Seed `projectile` frames, 6 Lock-On `reticle` frames, 9 Bouncy Bubble `bounce` frames, and 6 Octazooka `projectile` frames. Bullet Seed used magenta chroma key so the green seed highlight survives; the other VFX used green chroma key. Validate packed sheets/GIF previews for green/magenta chroma residue before treating repacked assets as final.

## Oranguru Source Mapping

- PMDCollab source: none available for `sprite/0765` at the time this pass was created.
- Static reference sprite is preserved as `assets\custom_spritework\references\static\oranguru\oranguru_reference.png`.
- Full ImageGen body source sheet is preserved as `assets\custom_spritework\references\imagegen\oranguru_full_source.png`.
- Corrective Stored Power body source row is preserved as `assets\custom_spritework\references\imagegen\oranguru_skill2_stored_power_body_source.png`.
- ImageGen VFX source sheet is preserved as `assets\custom_spritework\references\imagegen\oranguru_vfx_source.png`.
- `idle`: ImageGen-generated seated breathing/fan loop, 8 packed frames. The smallest generated first frame is skipped so the idle size stays steadier, and the later undersized idle slot is replaced with the previous normal-sized pose to avoid a visible size pop.
- `run`: ImageGen-generated slow grounded staff/fan shuffle, 8 packed frames. No dust.
- `attack` / Simple Beam: ImageGen-generated body-only fan/hand casting gesture, 8 packed frames. Detached neighbor-frame flecks from the generated row are removed; the separate future VFX projectile is staged as `assets\custom_spritework\vfx\oranguru_basicattack_simple_beam_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Nasty Plot: ImageGen-generated plotting pose with small body-attached dark thought glow, 8 packed frames. A separate future-ready enemy mark is staged as `assets\custom_spritework\vfx\oranguru_skill1_nasty_plot_mark#sheet.png` and `#anim.fanim`.
- `skill2` / Stored Power: ImageGen-generated corrected body row, 10 packed frames. A separate future-ready psychic radius release is staged as `assets\custom_spritework\vfx\oranguru_skill2_stored_power_burst#sheet.png` and `#anim.fanim`.
- `ult` / Instruct: ImageGen-generated command gesture, 8 packed frames. A separate future-ready ally command marker is staged as `assets\custom_spritework\vfx\oranguru_ult_instruct_command#sheet.png` and `#anim.fanim`.
- `dead`: ImageGen-generated hurt/slump/faint, 5 packed frames.
- Passive / Symbiosis: no separate spritework or VFX is staged.

Oranguru was generated as ImageGen-only because there was no PMDCollab source. Use a single full-sheet body generation pass for consistency, but validate every row with a frame contact sheet before accepting it. The first body source had a usable full character set, but Stored Power's generated aura row bled across frame slots, so it was replaced with a separate `oranguru_skill2_stored_power_body_source.png` row and repacked into the champion sheet. Keep long spell visuals separate from the body sheet; Simple Beam, Nasty Plot, Stored Power, and Instruct are staged under `assets\custom_spritework\vfx`.

After comparing the scratch-built Oranguru sheet against Leafeon and against Oranguru's own attack/Stored Power/Instruct rows, the larger idle/run/Nasty Plot/dead rows were rebuilt from the clean ImageGen source components at 85% scale while preserving their bottom anchors. This keeps the ImageGen-only rows from reading like Oranguru randomly grows and shrinks between normal gameplay animations. Do not resize already-packed atlas cells in place for this kind of correction; rebuild the affected cells from clean source frames so old silhouettes cannot remain underneath the resized sprites.

Final VFX counts are 6 Simple Beam `projectile` frames, 6 Nasty Plot `mark` frames, 9 Stored Power `burst` frames, and 7 Instruct `command` frames. The body source used magenta chroma key to preserve the green fan, while the VFX source used green chroma key. Validate packed sheets/GIF previews for green/magenta chroma residue and frame-edge alpha before treating repacked assets as final.

## Orbeetle Source Mapping

- PMDCollab source: `sprite/0826`, from `https://sprites.pmdcollab.org/#/0826?form=0`.
- PMDCollab credits: current `CC_BY-NC_4` in `assets\custom_spritework\references\pmdcollab\0826\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Emit/SpAttack/Swing/Double/Rotate/Hop.
- ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\orbeetle_vfx_source.png`, with row crops `orbeetle_vfx_row_1.png` and `orbeetle_vfx_row_2.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Struggle Bug: `Attack`, recentered because the source Attack offsets clipped the right edge in the first 96x96 pack.
- `skill` / Agility: `Double`
- `skill2` / Infestation: `Shoot`. A separate future-ready left-to-right Bug swarm/wave VFX is staged as `assets\custom_spritework\vfx\orbeetle_skill2_infestation_wave#sheet.png` and `#anim.fanim`.
- `ult` / Psyshock: `Emit`. A separate future-ready left-to-right psychic cone VFX is staged as `assets\custom_spritework\vfx\orbeetle_ult_psyshock_cone#sheet.png` and `#anim.fanim`; the generated source row has seven clean frames, so pack it as seven frames rather than forcing an eighth clipped frame.
- `dead`: `Hurt+Sleep`
- Passive / Telepathy: no separate spritework or VFX is staged.

Orbeetle conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0826 `
  --source-dir assets\custom_spritework\references\pmdcollab\0826 `
  --output-base assets\custom_spritework\champions\orbeetle `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Double `
  --map skill2=Shoot `
  --map ult=Emit `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --bottom-padding 22 `
  --recenter-tag attack
```

Final VFX counts are 6 Infestation `wave` frames and 7 Psyshock `cone` frames. The original ImageGen VFX rows had green chroma residue that read as artifacting in the previews, so the final packed VFX sheets remap all saturated green/green-fringe pixels into the intended purple/lavender bug palette for Infestation and violet/cyan psychic palette for Psyshock. Keep that cleanup if these sheets are regenerated from source. Orbeetle is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Pangoro Source Mapping

- PMDCollab source: `sprite/0675`, from `https://sprites.pmdcollab.org/#/0675?form=0`.
- PMDCollab credits: current `CC_BY-NC_4` rows in `assets\custom_spritework\references\pmdcollab\0675\credits.txt` cover Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/Twirl/Swing/Double/Rotate/Hop.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Night Slash: `Strike`
- `skill` / Taunt: `Charge`
- `skill2` / Parting Shot: `Strike`
- `ult` / Brutal Swing: `Swing`
- `dead`: `Hurt+Sleep`
- Passive / Scrappy: no separate spritework or VFX is staged.

Pangoro conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0675 `
  --source-dir assets\custom_spritework\references\pmdcollab\0675 `
  --output-base assets\custom_spritework\champions\pangoro `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Charge `
  --map skill2=Strike `
  --map ult=Swing `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 46 `
  --bottom-padding 24
```

Pangoro is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch. No ImageGen or detached VFX assets are staged for this pass.

## Passimian Source Mapping

- PMDCollab source: `sprite/0766`, from `https://sprites.pmdcollab.org/#/0766?form=0`.
- PMDCollab credits: current `CC_BY-NC_4` rows in `assets\custom_spritework\references\pmdcollab\0766\credits.txt` cover Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Strike/SpAttack/Swing/Double/Rotate/Hop.
- ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\passimian_vfx_source.png`, with row crops `passimian_vfx_row_1.png` and `passimian_vfx_row_2.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Fling: `SpAttack`. A separate future-ready melon projectile is staged as `assets\custom_spritework\vfx\passimian_basicattack_fling_melon_projectile#sheet.png` and `#anim.fanim`; it is based on the same green-yellow melon Passimian carries in the body sprite.
- `skill` / Coaching: `Charge`
- `skill2` / Reversal: `Hop`
- `ult` / Focus Blast: `Shoot`. A separate future-ready left-to-right Focus Blast projectile is staged as `assets\custom_spritework\vfx\passimian_ult_focus_blast_projectile#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Receiver: no separate spritework or VFX is staged.

Passimian conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0766 `
  --source-dir assets\custom_spritework\references\pmdcollab\0766 `
  --output-base assets\custom_spritework\champions\passimian `
  --map idle=Idle `
  --map run=Walk `
  --map attack=SpAttack `
  --map skill=Charge `
  --map skill2=Hop `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 42 `
  --bottom-padding 24
```

Final VFX counts are 6 Fling melon `projectile` frames and 7 Focus Blast `projectile` frames. The ImageGen source used magenta chroma key to preserve the green melon; the final packed VFX sheets remove remaining magenta fringe after packing. Passimian is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Pikachu Source Mapping

- PMDCollab source: `sprite/0025`, from `https://sprites.pmdcollab.org/#/0025?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` in `assets\custom_spritework\references\pmdcollab\0025\credits.txt` covers Walk/Attack/QuickStrike/Shoot/Shock/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate and related extra actions.
- ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\pikachu_vfx_source.png`, with row crops `pikachu_vfx_row_1.png` through `pikachu_vfx_row_4.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Spark: `Shoot`, recentered. A separate future-ready compact Electric projectile is staged as `assets\custom_spritework\vfx\pikachu_basicattack_spark_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Volt Tackle: `QuickStrike`, recentered because the original PMD dash offsets caused partial/vanishing body frames inside the 96x96 cell. A separate future-ready dash trail is staged as `assets\custom_spritework\vfx\pikachu_skill1_volt_tackle_dash_trail#sheet.png` and `#anim.fanim`.
- `skill2` / Thunder Wave: `Shoot`, recentered. A separate future-ready long horizontal traveling wave/wall is staged as `assets\custom_spritework\vfx\pikachu_skill2_thunder_wave_wall#sheet.png` and `#anim.fanim`.
- `ult` / Thunder: `Shock`. A separate future-ready centered lightning strike/ground impact AOE is staged as `assets\custom_spritework\vfx\pikachu_ult_thunder_strike_aoe#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Static Shock: no separate spritework or VFX is staged.

Pikachu conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0025 `
  --source-dir assets\custom_spritework\references\pmdcollab\0025 `
  --output-base assets\custom_spritework\champions\pikachu `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=QuickStrike `
  --map skill2=Shoot `
  --map ult=Shock `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 34 `
  --bottom-padding 30 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2
```

Final VFX counts are 6 Spark `projectile` frames, 8 Volt Tackle `trail` frames, 8 Thunder Wave `wave` frames, and 7 Thunder `strike` frames. The ImageGen source used magenta chroma key; the final packed VFX sheets remove magenta fringe after packing. Pikachu is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Porygon-Z Source Mapping

- PMDCollab source: `sprite/0474`, from `https://sprites.pmdcollab.org/#/0474?form=0`.
- PMDCollab credits: current `CHUNSOFT CUR Unspecified` and `PMDCollab_1` rows in `assets\custom_spritework\references\pmdcollab\0474\credits.txt` cover Walk/Attack/Strike/Shoot/SpAttack/RearUp/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate and related extra actions.
- ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\porygonz_vfx_source.png`, with row crops `porygonz_vfx_row_1.png` through `porygonz_vfx_row_4.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Conversion: `Shoot`, recentered. A separate future-ready digital conversion projectile is staged as `assets\custom_spritework\vfx\porygonz_basicattack_conversion_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Conversion 2: `Shoot`, recentered. A separate future-ready stronger prismatic digital projectile is staged as `assets\custom_spritework\vfx\porygonz_skill1_conversion2_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Recover: `RearUp`, recentered. A separate future-ready centered digital heal/reset aura is staged as `assets\custom_spritework\vfx\porygonz_skill2_recover_aura#sheet.png` and `#anim.fanim`.
- `ult` / Tri Attack: `Shoot`, recentered. A separate future-ready three-color line attack is staged as `assets\custom_spritework\vfx\porygonz_ult_tri_attack_line#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Adaptability: no separate spritework or VFX is staged.

Porygon-Z conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0474 `
  --source-dir assets\custom_spritework\references\pmdcollab\0474 `
  --output-base assets\custom_spritework\champions\porygonz `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=RearUp `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 36 `
  --bottom-padding 30 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult
```

Final VFX counts are 6 Conversion `projectile` frames, 6 Conversion 2 `projectile` frames, 8 Recover `aura` frames, and 7 Tri Attack `line` frames. The ImageGen source used magenta chroma key; the projectile and aura rows use broad magenta cleanup after packing, while Tri Attack uses stricter key cleanup so the red beam channel is not accidentally removed. Porygon-Z is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Pyukumuku Source Mapping

- PMDCollab source: `sprite/0771`, from `https://sprites.pmdcollab.org/#/0771?form=0`.
- PMDCollab credits: current `PMDCollab_2` and `CC_BY-NC_4` rows in `assets\custom_spritework\references\pmdcollab\0771\credits.txt` cover Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/Rumble/Emit/Swing/Double/Rotate/Hop.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Harden: `Charge`, recentered.
- `skill` / Toxic: `Shoot`, recentered.
- `skill2` / Rest: `Sleep`, recentered.
- `ult` / Pain Split: `Attack`, recentered.
- `dead`: `Emit`, recentered. This is intentionally not the normal `Hurt+Sleep` death because Pyukumuku's innards-out style death read is clearer with `Emit`.
- Passive / Innards Out: no separate spritework or VFX is staged.

Pyukumuku conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0771 `
  --source-dir assets\custom_spritework\references\pmdcollab\0771 `
  --output-base assets\custom_spritework\champions\pyukumuku `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Charge `
  --map skill=Shoot `
  --map skill2=Sleep `
  --map ult=Attack `
  --map dead=Emit `
  --direction-row 1 `
  --max-content-size 34 `
  --bottom-padding 30 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --recenter-tag dead
```

Pyukumuku is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch. No ImageGen or detached VFX assets are staged for this pass.

## Ribombee Source Mapping

- PMDCollab source: `sprite/0743`, from `https://sprites.pmdcollab.org/#/0743?form=0`.
- PMDCollab credits: current `PMDCollab_1` rows in `assets\custom_spritework\references\pmdcollab\0743\credits.txt` cover Attack/Charge/Double/Hop/Hover/Hurt/Idle/Rotate/Shoot/Sleep/Strike/Swing/Walk.
- ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\ribombee_vfx_source.png`, with row crops `ribombee_vfx_row_1.png` through `ribombee_vfx_row_4.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Pollen Puff: `Shoot`, recentered. A separate future-ready yellow/pink pollen projectile is staged as `assets\custom_spritework\vfx\ribombee_basicattack_pollen_puff_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Draining Kiss: `Shoot`, recentered. A separate future-ready thin Fairy kiss line is staged as `assets\custom_spritework\vfx\ribombee_skill1_draining_kiss_line#sheet.png` and `#anim.fanim`.
- `skill2` / U-Turn: `Swing`, recentered. A separate future-ready compact bug/fairy crescent dash trail is staged as `assets\custom_spritework\vfx\ribombee_skill2_u_turn_trail#sheet.png` and `#anim.fanim`.
- `ult` / Alluring Voice: `Rotate`, recentered. A separate future-ready caster-centered singing/charm aura is staged as `assets\custom_spritework\vfx\ribombee_ult_alluring_voice_aura#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`
- Passive / Honey Gatherer: no separate spritework or VFX is staged.

Ribombee conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0743 `
  --source-dir assets\custom_spritework\references\pmdcollab\0743 `
  --output-base assets\custom_spritework\champions\ribombee `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Swing `
  --map ult=Rotate `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 36 `
  --bottom-padding 30 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult
```

Final VFX counts are 6 Pollen Puff `projectile` frames, 7 Draining Kiss `line` frames, 8 U-Turn `trail` frames, and 6 Alluring Voice `aura` frames. The ImageGen source used green chroma key and also produced some green-tinted sparkle/ring pixels inside the effects; final packed VFX sheets remap green-dominant pixels into the intended yellow/pink Fairy palette after packing. Ribombee is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Sawk & Throh Source Mapping

- PMDCollab source: none used for this staged pass. The shared pair sheet was generated with ImageGen and preserved under `assets\custom_spritework\references\imagegen`.
- Current engine limitation: Sawk & Throh is a native Rust `PokemonChampion`, and the native path currently exposes one standard animation tag per action slot. The earlier `SwitchByBuff + view_buffs` stance-routing probe only applies to data champion view-buff routing, not to the current native Rust champion path. For now, use Option 1: one shared tag-team body sheet where both partners are visible in each standard action.
- ImageGen sources: `sawk_throh_source_v2.png` for idle/run/basic/switch base rows, plus corrected replacement rows `sawk_throh_skill2_row_source.png`, `sawk_throh_ult_row_source.png`, and `sawk_throh_dead_row_source.png`.
- `idle`: shared pair idle, both visible.
- `run`: shared pair run, both visible.
- `attack` / Brick Break / Rock Smash: shared forward pair attack aimed toward the enemy.
- `skill` / Sawk & Throh stance switch: shared tag-team handoff pose, both visible through the full row.
- `skill2` / Dynamic Punch / Circle Throw: corrected replacement row where both partners attack forward as teammates. Do not use rows where they appear to fight each other, duplicate Sawk, or drop one partner.
- `ult` / Mach Punch / Storm Throw: corrected replacement row where both partners remain visible through the full action.
- `dead`: corrected replacement row ending with both partners down.
- Passive / Inner Focus: no separate spritework or VFX is staged.

Sawk & Throh has no detached VFX assets in this pass. It is staged only in `assets\custom_spritework` for now; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch. If native view-buff/form routing is added later, revisit this as a proper Sawk-default/Throh-buffed stance swap rather than this shared-pair fallback.

## Scizor Source Mapping

- PMDCollab source: `sprite/0212`, from `https://sprites.pmdcollab.org/#/0212?form=0`.
- PMDCollab credits: current `CHUNSOFT` row in `assets\custom_spritework\references\pmdcollab\0212\credits.txt` covers Idle/Walk/Sleep/Hurt/Attack/Charge/Shoot/MultiScratch/SpAttack/Swing/Double/Rotate/Hop.
- `SpAttack` is a `CopyOf Charge` alias in Scizor's `AnimData.xml`, so the staged basic attack maps to `Charge` while documenting the user-facing request as SpAttack.
- ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\scizor_skill1_swords_dance_vfx_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Metal Claw: `Charge` (`SpAttack` alias), reduced travel and recentered.
- `skill` / Swords Dance: `Idle` body only, plus a separate future-ready caster-centered red/silver Swords Dance aura staged as `assets\custom_spritework\vfx\scizor_skill1_swords_dance_aura#sheet.png` and `#anim.fanim`. The QC preview `assets\custom_spritework\previews\scizor_skill1_preview.gif` composites this aura over the idle body because the skill is primarily VFX.
- `skill2` / Bullet Punch: `Shoot`, reduced travel and recentered.
- `ult` / X-Scissor: `MultiScratch`, reduced travel and recentered.
- `dead`: `Hurt+Sleep`
- Passive / Light Metal: no separate spritework or VFX is staged.

Scizor conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0212 `
  --source-dir assets\custom_spritework\references\pmdcollab\0212 `
  --output-base assets\custom_spritework\champions\scizor `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Charge `
  --map skill=Idle `
  --map skill2=Shoot `
  --map ult=MultiScratch `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 54 `
  --bottom-padding 22 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --recenter-tag dead `
  --travel-scale attack=0.35 `
  --travel-scale skill2=0.55 `
  --travel-scale ult=0.55
```

The Swords Dance aura was generated on magenta chroma, packed as an `aura` tag, then recolored away from hot magenta into red/orange/silver highlights. Scizor is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Shedinja Source Mapping

- PMDCollab source: `sprite/0292`, from `https://sprites.pmdcollab.org/#/0292?form=0`.
- PMDCollab credits: current `CHUNSOFT` row in `assets\custom_spritework\references\pmdcollab\0292\credits.txt` covers Walk/Attack/Scratch/Shoot/SpAttack/Twirl/Sleep/Hurt/Idle/Swing/Double/Hop/Charge/Rotate.
- ImageGen body/VFX sources are preserved as `assets\custom_spritework\references\imagegen\shedinja_downed_death_source.png` and `assets\custom_spritework\references\imagegen\shedinja_vfx_source.png`.
- `idle`: `Idle`
- `run`: `Walk`
- `attack` / Shadow Force: `Shoot`, reduced travel and recentered. A separate future-ready dark ghost projectile is staged as `assets\custom_spritework\vfx\shedinja_basicattack_shadow_force_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Skitter Smack: `Charge`, recentered. A separate future-ready bug/dark cone slash is staged as `assets\custom_spritework\vfx\shedinja_skill1_skitter_smack_cone#sheet.png` and `#anim.fanim`.
- `skill2` / Grudge: `Shoot`, reduced travel and recentered. A separate future-ready curse projectile/mark is staged as `assets\custom_spritework\vfx\shedinja_skill2_grudge_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Astral Barrage: `Shoot`, reduced travel and recentered. A separate future-ready left-to-right astral barrage line/wave is staged as `assets\custom_spritework\vfx\shedinja_ult_astral_barrage_wave#sheet.png` and `#anim.fanim`.
- `dead`: ImageGen-generated greyed-out downed body row packed into the champion `dead` tag, with Shedinja lying on the ground. This is baked into the current body sheet because the native game can already call `dead`.
- Passive / Wonder Guard: no normal standing body animation. Future-ready rectangular hit-chit overlay states are staged as `assets\custom_spritework\vfx\shedinja_passive_wonder_guard_chits#sheet.png` and `#anim.fanim`, with fixed state tags `chits_5`, `chits_4`, `chits_3`, `chits_2`, `chits_1`, and `chits_0`. These are not a playback animation; treat them like a health bar applied only after the grey downed/death body state is reached. Chits decrement by removing fixed slots left-to-right without resizing or recentering. Keep these as generic pips, rectangles, or squares; do not use shell/clamshell/fish/water iconography. The QC preview `assets\custom_spritework\previews\shedinja_passive_wonder_guard_downed_chits_preview.gif` composites the state frames over the final downed body, but this overlay is not currently callable by the game.

Shedinja conversion command before replacing the `dead` tag with the ImageGen downed row:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0292 `
  --source-dir assets\custom_spritework\references\pmdcollab\0292 `
  --output-base assets\custom_spritework\champions\shedinja `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=Shoot `
  --map ult=Shoot `
  --map dead=Sleep `
  --direction-row 1 `
  --max-content-size 44 `
  --bottom-padding 24 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --recenter-tag dead `
  --travel-scale attack=0.35 `
  --travel-scale skill2=0.35 `
  --travel-scale ult=0.35
```

Final VFX counts are 6 Shadow Force `projectile` frames, 6 Skitter Smack `cone` frames, 6 Grudge `projectile` frames, 6 Astral Barrage `line` frames, and 6 Wonder Guard fixed state frames. Shedinja is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Sirfetch'd Source Mapping

PMDCollab `sprite/0865` provides `Idle`, `Walk`, `Swing`, `Double`, and `Rotate`, but the action rows rotate through facing directions and the PMD walk is mostly an idle shuffle rather than a readable run. Sirfetch'd is therefore staged from ImageGen source instead of a normal PMDCollab conversion. The usable generated source is `assets\custom_spritework\references\imagegen\sirfetchd_full_source_v3.png`; the earlier green-key and v2 sources are reference-only and should not be repacked because they caused green deletion, weak walking motion, crowded rows, or body-size pops.

- `idle`: ImageGen upright right-facing 3/4 idle, 4 frames.
- `run`: ImageGen walk cycle, 8 frames, no dust. Feet must visibly alternate; do not accept a squash/stretch idle shuffle as Sirfetch'd's run.
- `attack` / Skull Bash: ImageGen short melee leek thrust/slash, 6 frames, little body travel.
- `skill` / Solar Blade: ImageGen charge-into-leek slash, 8 frames. The green slash cue is baked into the body/action frame; do not use a detached Solar Blade projectile.
- `skill2` / Vacuum Wave: ImageGen body-only cast/swing, 6 frames. The actual projectile is staged separately as `assets\custom_spritework\vfx\sirfetchd_skill2_vacuum_wave_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Brave Bird: ImageGen forward dash/contact strike, 7 frames. The fixed-scale v3 pack keeps the body full-size in every frame and avoids clipping from neighboring rows.
- `dead`: ImageGen greyed/downed row, 8 frames.
- Passive / Leek: no separate animation or VFX.

Final VFX count is 6 Vacuum Wave `projectile` frames. The v3 pack uses a fixed character scale instead of per-frame bounding-box fitting, so the long leek does not make the body shrink or grow between actions. Sirfetch'd is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Skarmory Source Mapping

PMDCollab `sprite/0227` supplies the full body source needed for this pass. `Strike` is a `CopyOf Attack` in `AnimData.xml`; do not try to download a separate `Strike-Anim.png`.

- `idle`: `Idle`, 7 frames.
- `run`: `Walk`, 9 frames.
- `attack` / Peck: `Attack`, recentered and timed to the authored 28-tick basic attack.
- `skill` / Roost: `Charge`, recentered and timed to the authored 32-tick self-cast. No separate VFX staged.
- `skill2` / Dual Wingbeat: `Attack`, recentered, with `--frame-select skill2=0-6,3-6,7-12` so the attack motion produces two visible hits inside the authored 34-tick skill window.
- `ult` / Fly: `Hop`, recentered and timed to the authored 58-tick ultimate. The in-game force movement should carry Skarmory while this airborne row is playing.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Sturdy: no separate animation or VFX.

Skarmory has no detached VFX in this pass. Skarmory is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Skeledirge Source Mapping

PMDCollab `sprite/0911` supplies the full body source needed for this pass. Skeledirge also has four future-ready ImageGen VFX sheets because all active slots need readable spell effects, but these are staged separately until custom VFX routing can call them.

- `idle`: `Idle`, 12 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Shadow Ball: `Shoot`, reduced travel and recentered. A separate future-ready dark ghost projectile is staged as `assets\custom_spritework\vfx\skeledirge_basicattack_shadow_ball_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Hex: `Charge`, recentered. A separate future-ready purple curse/lock target effect is staged as `assets\custom_spritework\vfx\skeledirge_skill1_hex_curse#sheet.png` and `#anim.fanim`; this is a target skill-block cue, not a projectile.
- `skill2` / Will-O-Wisp: `Shoot`, reduced travel and recentered. A separate future-ready blue-purple ghost flame projectile is staged as `assets\custom_spritework\vfx\skeledirge_skill2_will_o_wisp_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Torch Song: `RearUp`, recentered and timed to the authored self-area burn. A separate future-ready circular fire/music floor AoE is staged as `assets\custom_spritework\vfx\skeledirge_ult_torch_song_aoe#sheet.png` and `#anim.fanim`; keep it as an in-place self-area effect, not a travel projectile.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Where There's Smoke There's Fire: no separate animation or VFX.

Skeledirge conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0911 `
  --source-dir assets\custom_spritework\references\pmdcollab `
  --output-base assets\custom_spritework\champions\skeledirge `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=Shoot `
  --map ult=RearUp `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 58 `
  --bottom-padding 16 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill2=0.25
```

Final VFX counts are 6 Shadow Ball `projectile` frames, 6 Hex `curse` frames, 6 Will-O-Wisp `projectile` frames, and 6 Torch Song `aoe` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\skeledirge_vfx_source.png`. Skeledirge is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Smeargle Source Mapping

PMDCollab `sprite/0235` supplies the body source needed for this pass. Smeargle's Rust implementation uses static `Sketch` wrapper slots that dispatch learned move data at runtime, so the staged body sheet should not shape-shift into copied Pokemon. Keep Smeargle's own body animation for every slot, and later route copied move VFX separately when custom VFX calls become available.

- `idle`: `Idle`, 2 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Sketch: `Shoot`, reduced travel and recentered.
- `skill` / Sketch: `Shoot`, reduced travel and recentered.
- `skill2` / Sketch: `Shoot`, reduced travel and recentered.
- `ult` / Sketch: `Shoot`, reduced travel and recentered.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Sketch memory: no separate animation or VFX.

Smeargle conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0235 `
  --source-dir assets\custom_spritework\references\pmdcollab `
  --output-base assets\custom_spritework\champions\smeargle `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 50 `
  --bottom-padding 20 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

No detached Smeargle VFX are staged in this pass. Future copied-move visuals should inherit or reference the copied opponent move's VFX while preserving Smeargle's own body sheet. Smeargle is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Snorlax Source Mapping

PMDCollab `sprite/0143` supplies the full body source needed for this pass. Snorlax is body-animation driven only; no detached VFX are staged.

- `idle`: `Idle`, 6 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Strength: `Stomp`, recentered and lightly travel-reduced for a melee basic.
- `skill` / Swallow: `Charge`, recentered and travel-reduced.
- `skill2` / Belly Drum: `Double`, recentered and travel-reduced.
- `ult` / Snore: `Sleep`, 2 frames.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Gluttony: no separate animation or VFX.

Snorlax conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0143 `
  --source-dir assets\custom_spritework\references\pmdcollab `
  --output-base assets\custom_spritework\champions\snorlax `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Stomp `
  --map skill=Charge `
  --map skill2=Double `
  --map ult=Sleep `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 62 `
  --bottom-padding 14 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25
```

Snorlax is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Starmie Source Mapping

PMDCollab `sprite/0121` supplies the full body source needed for this pass. Starmie has four future-ready ImageGen VFX sheets staged separately for its water/psychic moves.

- `idle`: `Idle`, 8 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Water Spout: `Shoot`, reduced travel and recentered. A separate future-ready water projectile is staged as `assets\custom_spritework\vfx\starmie_basicattack_water_spout_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Flip Turn: `Attack`, recentered with moderate travel retained so native movement can read as a dash/contact turn. A separate future-ready water slash/splash contact effect is staged as `assets\custom_spritework\vfx\starmie_skill1_flip_turn_splash#sheet.png` and `#anim.fanim`.
- `skill2` / Heart Swap: `Rotate`, recentered and travel-reduced. A separate future-ready psychic heart/star shockwave is staged as `assets\custom_spritework\vfx\starmie_skill2_heart_swap_shockwave#sheet.png` and `#anim.fanim`.
- `ult` / Psybeam: `Charge`, recentered and travel-reduced. A separate future-ready thin psychic line beam is staged as `assets\custom_spritework\vfx\starmie_ult_psybeam_line#sheet.png` and `#anim.fanim`; keep it thin and line-like, not a large blast.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Illuminate: no separate animation or VFX.

Starmie conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0121 `
  --source-dir assets\custom_spritework\references\pmdcollab `
  --output-base assets\custom_spritework\champions\starmie `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Attack `
  --map skill2=Rotate `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 54 `
  --bottom-padding 20 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.55 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Water Spout `projectile` frames, 6 Flip Turn `splash` frames, 6 Heart Swap `shockwave` frames, and 6 Psybeam `line` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\starmie_vfx_source.png`. Starmie is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Sylveon Source Mapping

PMDCollab `sprite/0700` supplies the full body source needed for this pass. Sylveon has four future-ready ImageGen VFX sheets staged separately for its fairy move effects.

- `idle`: `Idle`, 8 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Disarming Voice: `Shoot`, reduced travel and recentered. A separate future-ready pink fairy soundwave/radius effect is staged as `assets\custom_spritework\vfx\sylveon_basicattack_disarming_voice_aoe#sheet.png` and `#anim.fanim`.
- `skill` / Misty Terrain: `Charge`, recentered and travel-reduced. A separate future-ready pastel mist field is staged as `assets\custom_spritework\vfx\sylveon_skill1_misty_terrain_field#sheet.png` and `#anim.fanim`. The original multi-row VFX source produced bad cell bleed on this row, so the final terrain asset uses the cleaner replacement source `assets\custom_spritework\references\imagegen\sylveon_misty_terrain_source.png`.
- `skill2` / Charm: `Double`, recentered and travel-reduced. A separate future-ready ally heal/charm heart-ribbon effect is staged as `assets\custom_spritework\vfx\sylveon_skill2_charm_heal#sheet.png` and `#anim.fanim`.
- `ult` / Dazzling Gleam: `Shoot`, reduced travel and recentered. A separate future-ready fairy starburst AoE is staged as `assets\custom_spritework\vfx\sylveon_ult_dazzling_gleam_aoe#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Eeveelution: no separate animation or VFX.

Sylveon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0700 `
  --source-dir assets\custom_spritework\references\pmdcollab `
  --output-base assets\custom_spritework\champions\sylveon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=Double `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 50 `
  --bottom-padding 20 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Disarming Voice `aoe` frames, 6 Misty Terrain `field` frames, 6 Charm `heal` frames, and 6 Dazzling Gleam `aoe` frames. The first ImageGen VFX source is preserved as `assets\custom_spritework\references\imagegen\sylveon_vfx_source.png`, with the final replacement Misty Terrain source preserved separately. Sylveon is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Thievul Source Mapping

PMDCollab `sprite/0828` supplies the full body source needed for this pass. Thievul has a baked skill2 self-buff aura for current in-game readability, plus separate future-ready VFX for Hone Claws and Stakeout.

- `idle`: `Idle`, 8 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Feint Attack: `Attack`, recentered with reduced melee travel.
- `skill` / Baddy Bad: `Attack`, recentered with moderate retained melee travel.
- `skill2` / Hone Claws: `Charge`, recentered and travel-reduced, with a compact dark claw/crit aura baked behind Thievul in the `skill2` body frames. The separate future-ready aura is staged as `assets\custom_spritework\vfx\thievul_skill2_hone_claws_aura#sheet.png` and `#anim.fanim`.
- `ult` / Stakeout: `Charge`, recentered and travel-reduced. The separate future-ready placed field is staged as `assets\custom_spritework\vfx\thievul_ult_stakeout_field#sheet.png` and `#anim.fanim`; keep this as a dark circular trap field, not a projectile.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Heal Block: no separate animation or VFX.

Thievul conversion command before baking the Hone Claws aura:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0828 `
  --source-dir assets\custom_spritework\references\pmdcollab\0828 `
  --output-base assets\custom_spritework\champions\thievul `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Attack `
  --map skill2=Charge `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 52 `
  --bottom-padding 20 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.45 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Hone Claws `aura` frames and 6 Stakeout `field` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\thievul_vfx_source.png`. Thievul is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Torterra Source Mapping

PMDCollab `sprite/0389` supplies the full body source needed for this pass. Torterra is body-animation driven only; no detached VFX are staged.

- `idle`: `Idle`, 5 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Absorb: `Shoot`, recentered and travel-reduced.
- `skill` / Withdraw: `Withdraw`, recentered and travel-reduced.
- `skill2` / Synthesis: `Charge`, recentered and travel-reduced.
- `ult` / Headlong Rush: `Attack`, recentered with moderate retained travel so it reads as a heavier rush.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Shell Armor: no separate animation or VFX.

Torterra conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0389 `
  --source-dir assets\custom_spritework\references\pmdcollab `
  --output-base assets\custom_spritework\champions\torterra `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Withdraw `
  --map skill2=Charge `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 62 `
  --bottom-padding 14 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.55
```

Torterra is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Turtonator Source Mapping

PMDCollab `sprite/0776` supplies the body source for Turtonator. The requested action labels exist directly in PMDCollab, including `Rumble`; keep all body animations left-to-right and stage Turtonator only in `assets\custom_spritework` until James asks for the next remap/recompile/redeploy batch.

- `idle`: `Idle`, 9 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Ember: `Shoot`, recentered and travel-reduced. A separate future-ready Ember projectile VFX is staged as `assets\custom_spritework\vfx\turtonator_basicattack_ember_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Shell Trap: `Rumble`, recentered and travel-reduced. A separate future-ready three-trap field VFX is staged as `assets\custom_spritework\vfx\turtonator_skill1_shell_trap_field#sheet.png` and `#anim.fanim`; it should read as three persistent small circular traps in a triangle, not one large projectile.
- `skill2` / Sunny Day: `Charge`, recentered and travel-reduced. A separate future-ready rectangular sunlight field VFX is staged as `assets\custom_spritework\vfx\turtonator_skill2_sunny_day_field#sheet.png` and `#anim.fanim`.
- `ult` / Clanging Scales: `Rotate`, recentered and travel-reduced. A separate future-ready self-radius scale burst is staged as `assets\custom_spritework\vfx\turtonator_ult_clanging_scales_burst#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Burning Jealousy: no body animation change. A separate future-ready brief enemy overlay is staged as `assets\custom_spritework\vfx\turtonator_passive_burning_jealousy_overlay#sheet.png` and `#anim.fanim`; it should be applied over enemies hit by the passive once custom VFX calls are supported.

Turtonator conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0776 `
  --source-dir assets\custom_spritework\references\pmdcollab\0776 `
  --output-base assets\custom_spritework\champions\turtonator `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Rumble `
  --map skill2=Charge `
  --map ult=Rotate `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 58 `
  --bottom-padding 16 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Ember `projectile` frames, 6 Shell Trap `field` frames, 6 Sunny Day `field` frames, 6 Clanging Scales `burst` frames, and 6 Burning Jealousy `overlay` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\turtonator_vfx_source.png`; the packed sheets were cleaned to transparent alpha with no exact or near-green chroma residue and no edge alpha.

## Umbreon Source Mapping

PMDCollab `sprite/0197` supplies the full body source needed for this pass. Keep Umbreon scaled with the small Eeveelution conversions and keep all projectile/line VFX left-to-right. Umbreon is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

- `idle`: `Idle`, 15 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Bite: `Attack`, recentered with reduced melee travel. No detached VFX are staged for the basic attack.
- `skill` / Snarl: `DeepBreath`, recentered and travel-reduced. A separate future-ready self-area dark soundwave/debuff VFX is staged as `assets\custom_spritework\vfx\umbreon_skill1_snarl_aoe#sheet.png` and `#anim.fanim`.
- `skill2` / Dark Pulse: `Shoot`, recentered and travel-reduced. A separate future-ready thin left-to-right dark pulse line is staged as `assets\custom_spritework\vfx\umbreon_skill2_dark_pulse_line#sheet.png` and `#anim.fanim`.
- `ult` / Crunch: `Attack`, recentered with moderate retained travel for the dash. A separate future-ready heavier Crunch bite impact is staged as `assets\custom_spritework\vfx\umbreon_ult_crunch_impact#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Eeveelution: no separate animation or VFX.

Umbreon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0197 `
  --source-dir assets\custom_spritework\references\pmdcollab\0197 `
  --output-base assets\custom_spritework\champions\umbreon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=DeepBreath `
  --map skill2=Shoot `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 42 `
  --bottom-padding 28 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.65
```

Final VFX counts are 6 Snarl `aoe` frames, 6 Dark Pulse `line` frames, and 6 Crunch `impact` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\umbreon_vfx_source.png`; the packed sheets received an extra dark green/cyan fringe cleanup pass after chroma removal and validate with no edge alpha or green/cyan residue. Do not stage a separate basic Bite VFX for Umbreon unless James explicitly asks for one later.

## Ursaluna Source Mapping

PMDCollab `sprite/0901` supplies regular Ursaluna. PMDCollab `sprite/0901/0001` supplies the normal brown Blood Moon form used in the `ult` tag; do not use the red shiny Blood Moon form. The DeviantArt Bloodmoon Ursaluna full sprite set reference from Anarlaurendil/Dracoyan was checked as a visual reference for normal-vs-shiny color, but the staged body frames use PMDCollab sources plus ImageGen transformation aura work.

- `idle`: `Idle`, 8 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Scratch: `Strike`, recentered with moderate retained melee travel. No detached VFX are staged for the basic attack.
- `skill` / Scary Face: `Charge`, recentered and travel-reduced. A separate future-ready targeted terrify overlay is staged as `assets\custom_spritework\vfx\ursaluna_skill1_scary_face_terrify#sheet.png` and `#anim.fanim`.
- `skill2` / Earthquake: PMDCollab labels this requested `Rumble` action as a `CopyOf RearUp` alias, so the conversion maps `skill2=RearUp` to avoid missing alias-image downloads. A separate future-ready cracked-ground radius aura is staged as `assets\custom_spritework\vfx\ursaluna_skill2_earthquake_aura#sheet.png` and `#anim.fanim`.
- `ult` / Blood Moon: custom 16-frame sequence. It starts with regular Ursaluna `Charge` frames, bakes in the ImageGen Blood Moon transformation aura, then reveals and holds the normal brown Blood Moon form from PMDCollab `0901/0001`. A separate future-ready transformation aura VFX is staged as `assets\custom_spritework\vfx\ursaluna_ult_blood_moon_transformation#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Guts: no separate animation or VFX.

Ursaluna base conversion command before replacing the `ult` tag:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0901 `
  --source-dir assets\custom_spritework\references\pmdcollab\0901 `
  --output-base assets\custom_spritework\champions\ursaluna `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=Charge `
  --map skill2=RearUp `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 60 `
  --bottom-padding 16 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.45 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Blood Moon temporary source conversion used `--species 0901/0001` with the same scale and bottom-padding settings. Final VFX counts are 6 Scary Face `overlay` frames, 6 Earthquake `aura` frames, and 6 Blood Moon transformation `aura` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\ursaluna_vfx_source.png`; the packed sheets and baked champion ult validate with no edge alpha or green/magenta chroma residue. Ursaluna is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

## Vanilluxe Source Mapping

PMDCollab `sprite/0584` supplies the full body source needed for this pass. All active move body tags use reduced-travel `Shoot` as requested. Vanilluxe is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

- `idle`: `Idle`, 26 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Icicle Spear: `Shoot`, recentered and travel-reduced. A separate future-ready single icicle spear projectile is staged as `assets\custom_spritework\vfx\vanilluxe_basicattack_icicle_spear_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Chilling Water: `Shoot`, recentered and travel-reduced. A separate future-ready icy water traveling wave is staged as `assets\custom_spritework\vfx\vanilluxe_skill1_chilling_water_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Frost Breath: `Shoot`, recentered and travel-reduced. A separate future-ready right-facing cone blast is staged as `assets\custom_spritework\vfx\vanilluxe_skill2_frost_breath_cone#sheet.png` and `#anim.fanim`.
- `ult` / Sheer Cold: `Shoot`, recentered and travel-reduced. A separate future-ready self-centered freeze AoE is staged as `assets\custom_spritework\vfx\vanilluxe_ult_sheer_cold_aoe#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 8 frames.
- Passive / Snow Patrol: no separate animation or VFX.

Vanilluxe conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0584 `
  --source-dir assets\custom_spritework\references\pmdcollab\0584 `
  --output-base assets\custom_spritework\champions\vanilluxe `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 48 `
  --bottom-padding 24 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Icicle Spear `projectile` frames, 6 Chilling Water `wave` frames, 6 Frost Breath `cone` frames, and 6 Sheer Cold `aoe` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\vanilluxe_vfx_source.png`; it used a magenta chroma key so blue/cyan ice pixels were preserved. Frost Breath and Sheer Cold were manually repacked from tighter source crops after the first pass showed crop-boundary slivers. Final packed sheets validate with no edge alpha or magenta residue.

## Vaporeon Source Mapping

PMDCollab `sprite/0134` supplies the full body source needed for this pass. `SpAttack` is a `CopyOf RearUp` alias in `AnimData.xml`, so map Skill 2 to the concrete `RearUp` files while treating it as the requested SpAttack action. Vaporeon is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

- `idle`: `Idle`, 2 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Water Gun: `Shoot`, recentered and travel-reduced. A separate future-ready narrow water stream projectile is staged as `assets\custom_spritework\vfx\vaporeon_basicattack_water_gun_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Growl: `Charge`, recentered and travel-reduced. A separate future-ready pale soundwave/debuff VFX is staged as `assets\custom_spritework\vfx\vaporeon_skill1_growl_wave#sheet.png` and `#anim.fanim`.
- `skill2` / Water Pulse: `RearUp`, recentered and travel-reduced. A separate future-ready pulsing water projectile with final splash is staged as `assets\custom_spritework\vfx\vaporeon_skill2_water_pulse_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Aqua Ring: `Rotate`, recentered and travel-reduced. A separate future-ready self-centered water/healing aura is staged as `assets\custom_spritework\vfx\vaporeon_ult_aqua_ring_aura#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Eeveelution: no separate animation or VFX.

Vaporeon conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0134 `
  --source-dir assets\custom_spritework\references\pmdcollab\0134 `
  --output-base assets\custom_spritework\champions\vaporeon `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Charge `
  --map skill2=RearUp `
  --map ult=Rotate `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 44 `
  --bottom-padding 26 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Water Gun `projectile` frames, 6 Growl `wave` frames, 6 Water Pulse `projectile` frames, and 6 Aqua Ring `aura` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\vaporeon_vfx_source.png`; it used a magenta chroma key so the blue/cyan water effects were preserved. The final VFX were repacked from fixed six-cell row slots rather than automatic connected-component grouping because Water Gun's detached droplets and Aqua Ring's wide ring could otherwise be misidentified as separate neighboring frames. Final packed sheets validate with no edge alpha, magenta residue, or green residue.

## Venusaur Source Mapping

PMDCollab `sprite/0003` supplies the full body source needed for this pass. The active body tags follow James's requested mapping, with light travel reduction on Tackle and the two Shoot casts so Venusaur stays anchored while the future VFX carry the range/read. Venusaur is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

- `idle`: `Idle`, 4 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Tackle: `Attack`, recentered and travel-reduced because this is a melee basic attack.
- `skill` / Vine Whip: `Shoot`, recentered and travel-reduced. A separate future-ready line bind VFX is staged as `assets\custom_spritework\vfx\venusaur_skill1_vine_whip_line#sheet.png` and `#anim.fanim`.
- `skill2` / Leech Seed: `Shoot`, recentered and travel-reduced. A separate future-ready seed/tether projectile VFX is staged as `assets\custom_spritework\vfx\venusaur_skill2_leech_seed_projectile#sheet.png` and `#anim.fanim`.
- `ult` / Sludge Wave: `Shake`, recentered and travel-reduced. A separate future-ready left-to-right poison traveling wave VFX is staged as `assets\custom_spritework\vfx\venusaur_ult_sludge_wave#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Tangling Vines: no separate animation or VFX.

Venusaur conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0003 `
  --source-dir assets\custom_spritework\references\pmdcollab\0003 `
  --output-base assets\custom_spritework\champions\venusaur `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Shoot `
  --map skill2=Shoot `
  --map ult=Shake `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 56 `
  --bottom-padding 18 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.25
```

Final VFX counts are 6 Vine Whip `line` frames, 6 Leech Seed `projectile` frames, and 6 Sludge Wave `wave` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\venusaur_vfx_source.png`; it used a magenta chroma key so green vine/seed pixels were preserved. The final VFX were packed from detected whole source groups rather than fixed six-cell row slices because Leech Seed and Sludge Wave had fewer than six distinct separated source groups and fixed slicing caused clipped fragments. The final pack duplicates the last usable group where needed for six-frame timing, recolors vine/seed magenta fringe into dark plant outline pixels, and validates with no edge alpha, magenta residue, or pure green chroma residue.

## Zeraora Source Mapping

PMDCollab `sprite/0807` supplies the full body source needed for this pass. `Strike` is a real animation for this species, so Plasma Fists uses `Strike` directly with a custom frame selection to show two quick hits in the same basic attack. Zeraora is staged only in `assets\custom_spritework` for this pass; do not add it to `champions_custom` or `mod.override_info` until James asks for the next remap/recompile/redeploy batch.

- `idle`: `Idle`, 6 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Plasma Fists: `Strike`, recentered, travel-reduced, and frame-selected as `0-5,2-8` so the body visibly strikes twice for the Rust two-hit basic attack.
- `skill` / Thunder Cage: `RearUp`, recentered and travel-reduced. A separate future-ready electric field VFX is staged as `assets\custom_spritework\vfx\zeraora_skill1_thunder_cage_field#sheet.png` and `#anim.fanim`.
- `skill2` / Zing Zap: `Charge`, recentered and travel-reduced. A separate future-ready parry/counter flash VFX is staged as `assets\custom_spritework\vfx\zeraora_skill2_zing_zap_counter#sheet.png` and `#anim.fanim`.
- `ult` / Wild Charge: `Attack`, recentered and travel-reduced. A separate future-ready multi-bolt burst VFX is staged as `assets\custom_spritework\vfx\zeraora_ult_wild_charge_bolts#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Merciless: no body animation. A separate future-ready enemy-applied trigger overlay is staged as `assets\custom_spritework\vfx\zeraora_passive_merciless_overlay#sheet.png` and `#anim.fanim`.

Zeraora conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0807 `
  --source-dir assets\custom_spritework\references\pmdcollab\0807 `
  --output-base assets\custom_spritework\champions\zeraora `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Strike `
  --map skill=RearUp `
  --map skill2=Charge `
  --map ult=Attack `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 52 `
  --bottom-padding 20 `
  --frame-select attack=0-5,2-8 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.30 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.35 `
  --tag-duration attack=0.4667 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.4333 `
  --tag-duration ult=0.8667
```

Final VFX counts are 6 Thunder Cage `field` frames, 6 Zing Zap `counter` frames, 6 Wild Charge `bolts` frames, and 6 Merciless `overlay` frames. The ImageGen source is preserved as `assets\custom_spritework\references\imagegen\zeraora_vfx_source.png`; it used a magenta chroma key so yellow/blue electric pixels were preserved. Pack from detected whole source groups because the lightning varies heavily in width, then run a hot magenta/pink fringe cleanup pass. Wild Charge also needs a tiny-component cleanup for detached high-frame specks after packing. Final packed sheets validate with no edge alpha, magenta residue, or pure green chroma residue.

## Rillaboom Source Mapping

PMDCollab `sprite/0812` supplies the body source. `Strike` is a `CopyOf Attack` alias in `AnimData.xml`, so the staged basic attack maps to the concrete `Attack` source while still representing the requested Strike animation. Rillaboom is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom`.

- `idle`: `Idle`, 2 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Drum Stick: `Attack` as the concrete PMDCollab source for the `Strike` alias, recentered with reduced travel.
- `skill` / Drum Beating: `Sing`, recentered with reduced travel, with a small ImageGen-created green/yellow drum aura baked behind the body for current readability. Separate future-ready VFX is staged as `assets\custom_spritework\vfx\rillaboom_skill1_drum_beating_aura#sheet.png` and `#anim.fanim`.
- `skill2` / Drum Roll: `Sing`, recentered with reduced travel, with a distinct ImageGen-created amber/green rolling drum aura baked behind the body. Separate future-ready VFX is staged as `assets\custom_spritework\vfx\rillaboom_skill2_drum_roll_aura#sheet.png` and `#anim.fanim`.
- `ult` / Grassy Surge: `Shoot`, recentered with reduced travel. Separate future-ready left-to-right grass wave VFX is staged as `assets\custom_spritework\vfx\rillaboom_ult_grassy_surge_wave#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Drum Solo: no separate animation or VFX.

Rillaboom body conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0812 `
  --source-dir assets\custom_spritework\references\pmdcollab\0812 `
  --output-base assets\custom_spritework\champions\rillaboom `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Sing `
  --map skill2=Sing `
  --map ult=Shoot `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 50 `
  --bottom-padding 18 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.35 `
  --travel-scale skill=0.20 `
  --travel-scale skill2=0.20 `
  --travel-scale ult=0.25 `
  --tag-duration attack=0.5667 `
  --tag-duration skill=0.7333 `
  --tag-duration skill2=0.7333 `
  --tag-duration ult=0.7333 `
  --tag-duration dead=1.0
```

ImageGen source is preserved as `assets\custom_spritework\references\imagegen\rillaboom_vfx_source.png`, with cleaned source at `rillaboom_vfx_source_cleaned.png`. The source used a magenta chroma key; generated hot pink/magenta accent pixels were recolored into yellow/amber/green before packing so the final previews do not look like chroma-key residue. Final Rillaboom idle body is 50 px tall inside the 96x96 frame, which matches the current Extra Large cap without making him larger than accepted roster scale.

## Dragapult Source Mapping

PMDCollab `sprite/0887` supplies the full body source. Dragapult is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom`.

- `idle`: `Idle`, 2 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Shred: `Shoot`, recentered with reduced travel. Separate future-ready VFX is staged as `assets\custom_spritework\vfx\dragapult_basicattack_shred_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Dragon Darts: `Shoot`, recentered with reduced travel. Separate future-ready paired-dart VFX is staged as `assets\custom_spritework\vfx\dragapult_skill1_dragon_darts_projectile#sheet.png` and `#anim.fanim`.
- `skill2` / Diving Swipe: `Attack`, with moderate retained travel. The staged champion sheet bakes in a pale ghost-tinted body plus a white/cyan dash trail so the dash reads in the current no-custom-VFX runtime. Separate future-ready dash VFX is staged as `assets\custom_spritework\vfx\dragapult_skill2_diving_swipe_ghost_dash#sheet.png` and `#anim.fanim`.
- `ult` / Spooky Shot: `Charge`, recentered with reduced travel. Separate future-ready spooky projectile VFX is staged as `assets\custom_spritework\vfx\dragapult_ult_spooky_shot_projectile#sheet.png` and `#anim.fanim`.
- Passive / Dragon Launcher: no body animation. A separate future-ready ghostly Dreepy follow-up projectile is staged as `assets\custom_spritework\vfx\dragapult_passive_dragon_launcher_dreepy#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.

Dragapult body conversion command before baking the skill2 ghost tint:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0887 `
  --source-dir assets\custom_spritework\references\pmdcollab\0887 `
  --output-base assets\custom_spritework\champions\dragapult `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Shoot `
  --map skill2=Attack `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 48 `
  --bottom-padding 20 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.55 `
  --travel-scale ult=0.25 `
  --tag-duration attack=0.6333 `
  --tag-duration skill=0.7333 `
  --tag-duration skill2=0.6333 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0
```

ImageGen source is preserved as `assets\custom_spritework\references\imagegen\dragapult_vfx_source.png`, with cleaned source at `dragapult_vfx_source_cleaned.png`. The source used a magenta chroma key; key removal was intentionally strict so darker purple ghost smoke survived, then remaining hot magenta was recolored to dark purple before packing. Final VFX counts are 6 frames each for Shred, Dragon Darts, Diving Swipe, Spooky Shot, and the passive Dreepy. After size comparison QA, the accepted champion sheet was downscaled in place first with `tools\scale-sprite-sheet-content.py --scale 0.875 --shift-y -6`, then again with `--scale 0.905 --shift-y 0`, making the first idle frame 38 px tall with bottom anchor 70.

## Shiftry Source Mapping

PMDCollab `sprite/0275` supplies the full body source. Shiftry is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom`.

- `idle`: `Idle`, 2 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Thwack: `MultiStrike`, recentered with reduced travel.
- `skill` / Fan Away: `Shoot`, recentered with reduced travel. Separate future-ready cone gust VFX is staged as `assets\custom_spritework\vfx\shiftry_skill1_fan_away_cone#sheet.png` and `#anim.fanim`.
- `skill2` / Shiftadieu: `MultiStrike`, recentered with reduced travel, with a small baked dark/leaf slash cue so it is visually distinct from the basic attack while still using the same PMDCollab action source.
- `ult` / Fan Tornado: `Charge`, recentered with reduced travel. Separate future-ready tornado field VFX is staged as `assets\custom_spritework\vfx\shiftry_ult_fan_tornado_field#sheet.png` and `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Forest Camouflage: no separate animation or VFX.

Shiftry body conversion command before baking the skill2 cue:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0275 `
  --source-dir assets\custom_spritework\references\pmdcollab\0275 `
  --output-base assets\custom_spritework\champions\shiftry `
  --map idle=Idle `
  --map run=Walk `
  --map attack=MultiStrike `
  --map skill=Shoot `
  --map skill2=MultiStrike `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 46 `
  --bottom-padding 22 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.30 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.35 `
  --travel-scale ult=0.25 `
  --tag-duration attack=0.4667 `
  --tag-duration skill=0.6333 `
  --tag-duration skill2=0.6333 `
  --tag-duration ult=0.7667 `
  --tag-duration dead=1.0
```

ImageGen source is preserved as `assets\custom_spritework\references\imagegen\shiftry_vfx_source.png`, with cleaned source at `shiftry_vfx_source_cleaned.png`. The source used a magenta chroma key; hot magenta residue was recolored into dark forest-purple pixels before packing. The generated Fan Away row's final frame was tight against the source edge, so the packed cone uses clean frames `0,1,2,3,4,4` rather than the clipped sixth source frame. Final VFX counts are 6 frames each for Fan Away and Fan Tornado.

## Sigilyph Source Mapping

PMDCollab `sprite/0561` supplies the full body source. Sigilyph is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom`.

- `idle`: `Idle`, 8 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Psychic Sphere: `Shoot`, recentered and travel-reduced. A separate future-ready projectile is staged as `assets\custom_spritework\vfx\sigilyph_basicattack_psychic_sphere_projectile#sheet.png` and `#anim.fanim`.
- `skill` / Sonic Wing: `Double`, recentered and travel-reduced. A separate future-ready cone is staged as `assets\custom_spritework\vfx\sigilyph_skill1_sonic_wing_cone#sheet.png` and `#anim.fanim`.
- `skill2` / Gravity: `Hop`, using only source frames `4-9` so Sigilyph starts at normal hover height and descends until touching the ground.
- `ult` / Psychic Assault: `SpAttack`, recentered and travel-reduced. A separate future-ready projectile is staged as `assets\custom_spritework\vfx\sigilyph_ult_psychic_assault_projectile#sheet.png` and `#anim.fanim`.
- Passive / Glypher: no body animation. Future-ready enemy mark and mark explosion VFX are staged as `assets\custom_spritework\vfx\sigilyph_passive_glypher_mark#sheet.png` / `#anim.fanim` and `assets\custom_spritework\vfx\sigilyph_passive_glypher_explosion_aoe#sheet.png` / `#anim.fanim`.
- `dead`: `Hurt+Sleep`, 8 frames.

Sigilyph body conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0561 `
  --source-dir assets\custom_spritework\references\pmdcollab\0561 `
  --output-base assets\custom_spritework\champions\sigilyph `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Double `
  --map skill2=Hop `
  --map ult=SpAttack `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 40 `
  --bottom-padding 24 `
  --frame-select skill2=4-9 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.25 `
  --travel-scale skill2=0.20 `
  --travel-scale ult=0.25 `
  --tag-duration attack=0.4667 `
  --tag-duration skill=0.5667 `
  --tag-duration skill2=0.5000 `
  --tag-duration ult=0.7333 `
  --tag-duration dead=1.0
```

ImageGen source is preserved as `assets\custom_spritework\references\imagegen\sigilyph_vfx_source.png`, with cleaned source at `sigilyph_vfx_source_cleaned.png`. The source uses a magenta chroma key to preserve cyan/purple psychic pixels. Pack the five rows deterministically as Psychic Sphere projectile, Sonic Wing cone, Psychic Assault projectile, Glypher mark, and Glypher explosion AoE. Recolor hot magenta residue to psychic purple after packing. The Glypher explosion row needed tighter cell cleanup because the generated contact sheet placed some adjacent-frame fragments near the cell edges; keep the final component/edge cleanup if repacking from the source image. Final VFX counts are 6 frames each. After size comparison QA, the accepted champion sheet was downscaled in place with `tools\scale-sprite-sheet-content.py --scale 0.92 --shift-y 0`, making the first idle frame 35 px tall with bottom anchor 72.

## Weavile Source Mapping

PMDCollab `sprite/0461` supplies the full body source. Weavile is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom`.

- `idle`: `Idle`, 2 frames.
- `run`: `Walk`, 4 frames.
- `attack` / Hail Claw: `Attack`, recentered and travel-reduced.
- `skill` / Scratch and Shuffle: `Attack`, recentered with a little more retained travel than the basic so the two-hit dash action reads differently while native Rust movement handles the actual displacement.
- `skill2` / Pursuit Claw: `Attack`, recentered and travel-reduced, with a subtle ImageGen-guided icy/dark claw arc baked into the champion frames. This cue is baked into the body sheet, not staged as a separate VFX asset.
- `ult` / Assaulting Hunt: `Charge`, recentered and travel-reduced, with ImageGen-guided stealth shimmer plus a progressive body fade/desaturation baked into the champion frames so Weavile looks like she is becoming invisible. This cue is baked into the body sheet, not staged as a separate VFX asset.
- `dead`: `Hurt+Sleep`, 4 frames.
- Passive / Lone Predator: no separate animation or VFX.

Weavile body conversion command before baking the skill2 and ult cues:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0461 `
  --source-dir assets\custom_spritework\references\pmdcollab\0461 `
  --output-base assets\custom_spritework\champions\weavile `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=Attack `
  --map skill2=Attack `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --canvas-size 96 `
  --max-content-size 38 `
  --bottom-padding 26 `
  --recenter-tag attack `
  --recenter-tag skill `
  --recenter-tag skill2 `
  --recenter-tag ult `
  --travel-scale attack=0.25 `
  --travel-scale skill=0.35 `
  --travel-scale skill2=0.25 `
  --travel-scale ult=0.20 `
  --tag-duration attack=0.3000 `
  --tag-duration skill=0.4000 `
  --tag-duration skill2=0.4333 `
  --tag-duration ult=0.5000 `
  --tag-duration dead=1.0
```

ImageGen source for the baked cues is preserved as `assets\custom_spritework\references\imagegen\weavile_baked_effects_source.png`, with cleaned source at `weavile_baked_effects_source_cleaned.png`. The first row is a light claw arc for Pursuit Claw; scale it down and keep opacity modest so it distinguishes `skill2` without reading as a projectile. The second row is a stealth shimmer overlay for Assaulting Hunt; combine it with a body fade/desaturation rather than replacing Weavile with a separate silhouette. After baking, the accepted champion sheet was downscaled in place with `tools\scale-sprite-sheet-content.py --scale 0.92 --shift-y 0`, making the first idle frame 35 px tall with bottom anchor 70.

## Swanna Source Mapping

PMDCollab `sprite/0581` supplies the full body source. Swanna is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\swanna#sheet.png` and `swanna#anim.fanim`.

- `idle`: `Idle`.
- `run`: `FlapAround` frames `0,1,0,1`, not `Walk` and not the full FlapAround row. The full row rotates through too many facing angles; this compact loop keeps the three-quarter right-facing flying read.
- `attack` / Flap: `Shoot`, with separate future-ready flying gust projectile VFX.
- `skill` / Feather Slice: `Attack`, because `Strike` is a `CopyOf Attack` alias in `AnimData.xml` and does not have its own `Strike-Anim.png`.
- `skill2` / Feathery Cyclone: `Charge`, with separate future-ready water/wind aura VFX.
- `ult` / Sky Circus: `Hop`; PMDCollab's right-facing Hop row already starts grounded, lifts off, holds an airborne travel pose, and returns to ground, so keep the full 0-9 frame sequence. Native Rust handles the actual x/y capture, Airborne, forced movement, pass-through damage, and return movement.
- Passive / Tailwind needs no separate spritework unless a subtle movement-buff overlay is added later.

Swanna body conversion command before final scale correction:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0581 `
  --source-dir .\assets\custom_spritework\references\pmdcollab\0581 `
  --output-base .\assets\custom_spritework\champions\swanna `
  --map idle=Idle `
  --map run=FlapAround `
  --map attack=Shoot `
  --map skill=Attack `
  --map skill2=Charge `
  --map ult=Hop `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 28 `
  --bottom-padding 24 `
  --frame-select run=0,1,0,1 `
  --tag-duration run=0.5333 `
  --tag-duration attack=0.3667 `
  --tag-duration skill=0.4000 `
  --tag-duration skill2=0.5667 `
  --tag-duration ult=0.9667 `
  --tag-duration dead=1.0
```

After preview comparison, downscale and lift the accepted champion sheet:

```powershell
.\.venv\Scripts\python.exe .\tools\scale-sprite-sheet-content.py .\assets\custom_spritework\champions\swanna --scale 0.92 --shift-y -6
```

This makes Swanna's first idle frame 35 px tall with bottom anchor 72, matching Sigilyph's current size band and avoiding the earlier Venusaur-sized read.

ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\swanna_vfx_source.png`, with cleaned source at `swanna_vfx_source_cleaned.png`. Packed VFX assets are:

- `assets\custom_spritework\vfx\swanna_basicattack_flap_gust_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\swanna_skill2_feathery_cyclone_aura#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\swanna_ult_sky_circus_aoe#sheet.png` / `#anim.fanim`

## Marowak Source Mapping

PMDCollab `sprite/0105` supplies the full body source. Marowak is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\marowak#sheet.png` and `marowak#anim.fanim`. Credits are preserved in `assets\custom_spritework\references\pmdcollab\0105\credits.txt`.

- `idle`: `Idle`.
- `run`: `Walk`.
- `attack` / Bonemerang: `Shoot` frames `0,1,2,1`, not the full Shoot row. The later Shoot frames rotate Marowak too far away; the compact loop keeps the three-quarter facing while the detached Bonemerang VFX sells the projectile.
- `skill` / Bone Rush: `Attack`.
- `skill2` / Bone Club: `Strike`.
- `ult` / Bone Windmill: `Charge`; native Rust handles the defensive aura, knockback, and next-Bonemerang empower state.
- `dead`: `Hurt+Sleep`.

Marowak body conversion command before final lift correction:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0105 `
  --source-dir .\assets\custom_spritework\references\pmdcollab\0105 `
  --output-base .\assets\custom_spritework\champions\marowak `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Attack `
  --map skill2=Strike `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 40 `
  --bottom-padding 18 `
  --frame-select attack=0,1,2,1 `
  --tag-duration attack=0.5667 `
  --tag-duration skill=0.7333 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.6333 `
  --tag-duration dead=1.0
```

This makes Marowak's first idle frame 28 px tall with bottom anchor 72, matching Shedinja's small size band. ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\marowak_vfx_source.png`, with cleaned source at `marowak_vfx_source_cleaned.png`; however, the accepted Bonemerang projectile was rebuilt deterministically after review so the bone stays straight and rotates as a rigid object instead of bending between frames. Packed VFX assets are:

- `assets\custom_spritework\vfx\marowak_basicattack_bonemerang_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\marowak_ult_bone_windmill_aura#sheet.png` / `#anim.fanim`

## Garganacl Source Mapping

PMDCollab `sprite/0934` supplies the full body source. Garganacl is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\garganacl#sheet.png` and `garganacl#anim.fanim`. Credits are preserved in `assets\custom_spritework\references\pmdcollab\0934\credits.txt`.

- `idle`: `Idle`.
- `run`: `Walk`.
- `attack` / Block Hammer: `Attack`.
- `skill` / Land Crush: `RearUp`, with separate future-ready ground shockwave VFX.
- `skill2` / Purifying Salt: `Double`, per the accepted request for this pass.
- `ult` / Blessed Salt: `Charge`, with separate future-ready salt spire and salt ring VFX.
- `dead`: `Hurt+Sleep`.

Garganacl body conversion command before final lift correction:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0934 `
  --source-dir .\assets\custom_spritework\references\pmdcollab\0934 `
  --output-base .\assets\custom_spritework\champions\garganacl `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Attack `
  --map skill=RearUp `
  --map skill2=Double `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 44 `
  --bottom-padding 18 `
  --tag-duration attack=0.7333 `
  --tag-duration skill=0.7000 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.6667 `
  --tag-duration dead=1.0
```

After preview comparison, lift Garganacl without changing scale:

```powershell
.\.venv\Scripts\python.exe .\tools\scale-sprite-sheet-content.py .\assets\custom_spritework\champions\garganacl --scale 1 --shift-y -6
```

This makes Garganacl's first idle frame 44 px tall with bottom anchor 72, keeping him in the bulky tank size band near Venusaur/Pangoro but below Rillaboom/Snorlax. ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\garganacl_vfx_source.png`, with cleaned source at `garganacl_vfx_source_cleaned.png`. Packed VFX assets are:

- `assets\custom_spritework\vfx\garganacl_skill1_land_crush_shockwave#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\garganacl_ult_blessed_salt_spire_ring#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\garganacl_passive_salt_cure_patch#sheet.png` / `#anim.fanim`

## Ampharos Source Mapping

PMDCollab `sprite/0181` supplies the full body source. Ampharos is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\ampharos#sheet.png` and `ampharos#anim.fanim`. Credits are preserved in `assets\custom_spritework\references\pmdcollab\0181\credits.txt`.

- `idle`: `Idle`.
- `run`: `Walk`.
- `attack` / Cluster Bolt: `Shoot`, with separate future-ready Electric projectile VFX.
- `skill` / Flash: `Attack`, with travel removed by `--travel-scale skill=0` and `--recenter-tag skill`; separate future-ready Electric self-AoE VFX.
- `skill2` / Searchlight Tail: `Charge`, with separate future-ready searchlight aura/buff VFX.
- `ult` / Gigavolt: `Charge`, with separate future-ready lightning strike and Amped Zone VFX.
- `dead`: `Hurt+Sleep`.

Ampharos body conversion command:

```powershell
.\.venv\Scripts\python.exe .\tools\convert-pmdcollab-sprite.py `
  --species 0181 `
  --source-dir .\assets\custom_spritework\references\pmdcollab\0181 `
  --output-base .\assets\custom_spritework\champions\ampharos `
  --map idle=Idle `
  --map run=Walk `
  --map attack=Shoot `
  --map skill=Attack `
  --map skill2=Charge `
  --map ult=Charge `
  --map dead=Hurt+Sleep `
  --direction-row 1 `
  --max-content-size 48 `
  --bottom-padding 24 `
  --travel-scale skill=0 `
  --recenter-tag skill `
  --tag-duration attack=0.5333 `
  --tag-duration skill=0.5000 `
  --tag-duration skill2=0.4667 `
  --tag-duration ult=0.6667 `
  --tag-duration dead=1.0
```

This makes Ampharos's first idle frame 34 px tall with bottom anchor 72, aligned with the normal-medium size band near Gallade/Sigilyph/Swanna. ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\ampharos_vfx_source.png`, with cleaned source at `ampharos_vfx_source_cleaned.png`. Packed VFX assets are:

- `assets\custom_spritework\vfx\ampharos_basicattack_cluster_bolt_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\ampharos_skill1_flash_aoe#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\ampharos_skill2_searchlight_tail_aura#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\ampharos_ult_gigavolt_strike_zone#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\ampharos_passive_luminous_pulse_aura#sheet.png` / `#anim.fanim`

## Xatu Source Mapping

PMDCollab `sprite/0178` supplies the full body source. Xatu is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\xatu#sheet.png` and `xatu#anim.fanim`. Credits are preserved in `assets\custom_spritework\references\pmdcollab\0178\credits.txt`.

- `idle`: `Idle`.
- `run`: `Hop` frames `0,2,5,6,7,8,5,2`, using the stable three-quarter start/end plus the raised-wing frames so the loop reads as flying rather than face/body bobbing.
- `attack` / Energy Singe: `Shoot`, with separate future-ready Psychic projectile VFX.
- `skill` / Mind Bend: `Rotate`, because PMDCollab `SpAttack` is a `CopyOf Rotate` alias; ImageGen psychic ring aura is baked into the champion frames.
- `skill2` / Pain Amplifier: `Rotate`, again standing in for `SpAttack`; ImageGen pressure/debuff aura is baked into the champion frames.
- `ult` / Super Psy: `Charge`, with separate future-ready thin global Psychic beam VFX.
- `dead`: `Hurt+Sleep`.

The accepted champion sheet is downscaled to 34 px idle height with bottom anchor 72, just under Sigilyph/Swanna's 35 px flyer/mage band. Prophecy needs no separate spritework or VFX. ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\xatu_vfx_source.png`, with row crops preserved beside it. Packed VFX assets are:

- `assets\custom_spritework\vfx\xatu_basicattack_energy_singe_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\xatu_skill1_mind_bend_aura#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\xatu_skill2_pain_amplifier_aura#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\xatu_ult_super_psy_beam#sheet.png` / `#anim.fanim`

## Quaquaval Source Mapping

PMDCollab `sprite/0914` supplies the full body source. Use source row 2 for the right-facing three-quarter read; row 1 is too front-facing for Quaquaval. Quaquaval is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\quaquaval#sheet.png` and `quaquaval#anim.fanim`. Credits are preserved in `assets\custom_spritework\references\pmdcollab\0914\credits.txt`.

- `idle`: `Idle`.
- `run`: `Walk`.
- `attack` / Hydro Kick: `Attack`, with separate future-ready water-splash kick VFX.
- `skill` / Spiral Shot: `Shoot`, with separate future-ready Water projectile VFX.
- `skill2` / Up-Tempo: `Charge`, with dash movement supplied by Rust and separate water/kick impact VFX.
- `ult` / Exciting Dance: `Shoot`, because PMDCollab `RearUp` is a `CopyOf Shoot` alias; no extra VFX staged for the ult itself.
- `dead`: `Hurt+Sleep`.

The accepted champion sheet uses source row 2 and is downscaled to 39-40 px idle height with bottom anchor 72, keeping Quaquaval in the medium ADC band rather than tank scale. Aqua Step needs trail VFX only; the passive body animation remains ordinary run/walk. ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\quaquaval_vfx_source.png`, with row crops preserved beside it. Packed VFX assets are:

- `assets\custom_spritework\vfx\quaquaval_basicattack_hydro_kick_splash#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\quaquaval_skill1_spiral_shot_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\quaquaval_skill2_up_tempo_impact#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\quaquaval_passive_aqua_step_trail#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\quaquaval_passive_aqua_step_empowered_trail#sheet.png` / `#anim.fanim`

## Arcanine Source Mapping

PMDCollab `sprite/0059` supplies the body source for this pass. Arcanine is mapped through `mod.override_info`; sync staged assets directly into `mod\pokemon_moba\champions_custom\arcanine#sheet.png` and `arcanine#anim.fanim`. Preserve credits in `assets\custom_spritework\references\pmdcollab\0059\credits.txt`.

- `idle`: `Idle`.
- `run`: `Walk`.
- `attack` / Flare: `Shoot`, with separate future-ready short Fire flare VFX.
- `skill` / Extremespeed: `Attack`; Rust supplies the actual dash path and shield state.
- `skill2` / White Flames: `Rumble`, the concrete source for PMDCollab's `SpAttack` alias, with separate future-ready single-line white-fire AoE VFX. Rust computes two offset line segments; future custom VFX should call the same single-line asset once for each segment rather than using a paired two-line asset.
- `ult` / Flames of Rage: `Charge`, with separate future-ready self-centered inferno aura VFX.
- `dead`: `Hurt+Sleep`.

The accepted champion sheet uses source row 1 and a 46 px idle height with bottom anchor 72, placing Arcanine in the large mage/bruiser visual band near Venusaur but below Snorlax/Torterra scale. Keep the body grounded and readable because Extremespeed movement is handled by native forced movement, not by large source-frame travel.

ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\arcanine_vfx_source.png`, with row crops preserved beside it. Packed VFX assets are:

- `assets\custom_spritework\vfx\arcanine_basicattack_flare_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\arcanine_skill2_white_flames_line_aoe#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\arcanine_ult_flames_of_rage_inferno_aura#sheet.png` / `#anim.fanim`

## Wishiwashi Source Mapping

PMDCollab `sprite/0746` supplies regular Solo Form Wishiwashi. PMDCollab `sprite/0746/0001` supplies School Form for the custom Massive Catch `ult` tag. Wishiwashi is pre-mapped through `mod.override_info`; after staging, sync directly into `mod\pokemon_moba\champions_custom\wishiwashi#sheet.png` and `wishiwashi#anim.fanim`.

- `idle`: regular `Idle`.
- `run`: regular `Walk`.
- `attack` / Splash: regular `Shoot`, with separate future-ready water droplet VFX.
- `skill` / Wave Splash: regular `Charge`, with separate future-ready traveling water wave VFX.
- `skill2` / Cowardice: regular `Double`; no separate VFX staged for this pass.
- `ult` / Massive Catch: custom spliced sequence using Solo Form startup, School Form body frames, baked schooling-channel fish/water aura, dash/swallow water wake, spit burst, and return to Solo Form.
- `dead`: regular `Hurt+Sleep`.

Wishiwashi sprite implementation notes:

- Keep Solo Form the smallest staged Pokemon while preserving legibility. Current first idle frame is 22 px tall with bottom anchor 72, smaller than Marowak/Shedinja at 28 px.
- Keep School Form isolated to the `ult` tag. Current School Form frames peak around 65 px tall, making the ult form larger than Snorlax/Rillaboom without scaling the normal champion.
- Massive Catch should visually read as School Form taking over for the ult duration. The Rust move handles the channel, dash, tether, chew ticks, return, and spit displacement; the sprite tag provides a readable 0.5s school-up channel, forward surge, swallow/splash, return/spit, and collapse back to Solo.
- ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\wishiwashi_vfx_source.png`, with row crops preserved beside it. Packed VFX assets are:

- `assets\custom_spritework\vfx\wishiwashi_basicattack_splash_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\wishiwashi_skill1_wave_splash_wave#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\wishiwashi_ult_schooling_channel_aura#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\wishiwashi_ult_massive_catch_water_burst#sheet.png` / `#anim.fanim`

## MissingNo. Source Mapping

MissingNo. is mapped through `mod.override_info` and intentionally does not use PMDCollab MissingNo. source forms. The accepted body source was generated from ImageGen using the original Red/Blue-style backward-L glitch-block sprite reference and a separate fan-sprite mood reference, then packed into:

- `mod\pokemon_moba\champions_custom\missingno#sheet.png`
- `mod\pokemon_moba\champions_custom\missingno#anim.fanim`

Use the display name `MissingNo.` with the dot. Tags:

- `idle`: glitch/static idle.
- `run`: right-facing teleport-step/glitch reassembly. The lower L-block must stay on the right side in every frame, the body should remain about 40 px tall with bottom anchor 72, and the motion should read as left-to-right by breaking apart and reforming several pixels farther forward rather than using a backward speed trail.
- `attack` / ???: glitch data cast, with no detached projectile baked into the body.
- `skill` / --: corrupted status burst cast.
- `skill2` / 'M (00): erratic glitch aura/cast, with a small body-attached glitch aura.
- `ult` / Trick Room: warped reality cast, with no large arena field baked into the body.
- `dead`: static collapse.

MissingNo. uses the custom Bird type. Bird has no standard PMD type source requirement; choose glitch/monochrome/VHS-like VFX language over a natural flying-bird treatment.

The accepted champion sheet uses a 40 px idle height with bottom anchor 72, so MissingNo. is taller than normal mages but no longer towers over Venusaur. Keep the footprint narrow and vertical; the shape should read as a corrupted backward-L block rather than a conventional creature.

ImageGen sources are preserved as:

- `assets\custom_spritework\references\imagegen\missingno_body_source.png`
- `assets\custom_spritework\references\imagegen\missingno_run_glitchstep_source.png`
- `assets\custom_spritework\references\imagegen\missingno_vfx_source.png`

Packed future-ready VFX assets are:

- `assets\custom_spritework\vfx\missingno_basicattack_glitch_data_projectile#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\missingno_basicattack_stray_glitch_line#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\missingno_skill1_random_status_corruption#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\missingno_skill2_glitch_storm_aura#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\missingno_skill2_chain_spark_arc#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\missingno_ult_trick_room_field#sheet.png` / `#anim.fanim`

Green-dominant generated pixels were recolored into cyan/magenta/lavender glitch pixels instead of left as chroma-key residue. If repacking from source, validate both the champion and VFX sheets for strict green and green-dominant residue.

## Yanmega Source Mapping

PMDCollab `sprite/0469` supplies `Idle`, `Walk`, `Shoot`, `Double`, `Charge`, `Hurt`, and `Sleep`. Yanmega's body was converted with a conservative `max-content-size` of 34 because the wing span can otherwise make him read too large compared to other medium ranged Pokemon.

Tags:

- `idle`: `Idle`.
- `run`: `Walk`.
- `attack` / Linear Beam: `Shoot`, with separate future-ready thin Bug beam VFX.
- `skill` / Buzzing Boost: `Shoot`, with separate future-ready buzzing Bug line VFX.
- `skill2` / Tinted Lens: `Double`; no separate VFX staged for this pass.
- `ult` / Giga Drain: `Charge`, with separate future-ready green drain tether/channel VFX.
- `dead`: `Hurt+Sleep`.

ImageGen source for the VFX is preserved as `assets\custom_spritework\references\imagegen\yanmega_vfx_source.png`, with row crops preserved beside it. Packed VFX assets are:

- `assets\custom_spritework\vfx\yanmega_basicattack_linear_beam#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\yanmega_skill1_buzzing_boost_line#sheet.png` / `#anim.fanim`
- `assets\custom_spritework\vfx\yanmega_ult_giga_drain_tether#sheet.png` / `#anim.fanim`

The VFX were generated on a magenta chroma key to preserve green/yellow Bug and Grass pixels, then cleaned for strict magenta residue. Do not use a green chroma key for future Yanmega VFX because both the body and Giga Drain effects legitimately contain green.
