# Pokemon MOBA Balance Surface

## Movement Compression, Build 0.2.8

2026-10-18. The authoritative base speeds in the stable Rust roster were edited
directly. No runtime clamp or hidden normalization is applied. The guide and SDK
champion registration therefore read the same values.

- Old range: 390-1480; median: 780; fastest/slowest ratio: 3.79.
- New range: 624-936; median: 780; fastest/slowest ratio: 1.50.
- Each side of the median is linearly compressed independently to +/-156, rounded
  to the nearest integer. Below median: 780 + (old - 780) * 156 / 390.
  Above median: 780 + (old - 780) * 156 / 700.
- Every old speed tie and strict relative ordering is preserved. All 103 champions
  remain at zero movement-speed growth. The median itself is unchanged.
- Haste, slows, items, conditional passives and dash/projectile travel speeds are
  not clamped to this base-speed band. This pass narrows baseline rotations and
  recovery, without deleting mobility abilities or changing minion/tower stats.
- Scizor stays fastest at 936. Metal Claw speed coefficient: 4% -> 6%;
  Swords Dance speed-to-crit: 50% -> 79%; Light Metal shield: 20% -> 32% of speed.
  These keep unbuffed speed-derived contributions close to their previous values.
  Light Metal travel threshold: 120000 -> 76000, preserving roughly 81 straight-line
  movement ticks per recharge before combat resets and the existing shield timer.
- Scizor's speed-check crit threshold: target below 50% -> below 81% of his speed.
  Tests preserve the exact eligible set of unbuffed roster opponents; the previous
  50% rule would become impossible within the compressed base-speed band.
- Dedenne remains near the fast end at 871. Static distance per charge: 15000 ->
  11000, preserving roughly 12.6 straight-line movement ticks per charge.
  Charge damage/cap, grid bonuses and out-of-combat haste are unchanged.
- Electrode Discharge distance: 90000 -> 66000, maintaining roughly 76 movement
  ticks per pulse at the new 869 base speed. Its haste and respawn outcome are unchanged.
- Other combat coefficients, health/defenses, ranges, cooldowns and movement
  ability speeds are unchanged. The compensations are not claims of final balance;
  actual routing, combat uptime and item interactions need live matches.

The roster-order test freezes these before/after values and checks registration
data, level-12 values, bounds, median, all pairwise adjacent speed rankings and
Scizor's unbuffed speed-crit matchup set. This is a one-time rebalance, not a
formula to silently reapply on each build.

Guide maintenance: run the Rust guide-export test, then
`python tools/generate_pokemon_guide.py`. Render the PDF with
`node tools/render-pokemon-guide.cjs` (Playwright on NODE_PATH, Edge installed),
visually check the changed pages, and copy the verified candidate from `tmp/pdfs`
to both guide locations before deployment. Do not ship an old PDF beside a new HTML guide.

| Pokemon | Before | After |
| --- | ---: | ---: |
| Comfey | 390 | 624 |
| Garganacl | 420 | 636 |
| Snorlax | 420 | 636 |
| Coalossal | 430 | 640 |
| Archaludon | 470 | 656 |
| Weezing | 500 | 668 |
| Xatu | 500 | 668 |
| Appletun | 510 | 672 |
| Wishiwashi | 515 | 674 |
| Ampharos | 520 | 676 |
| Bouffalant | 520 | 676 |
| Drampa | 520 | 676 |
| Magmortar | 520 | 676 |
| Turtonator | 520 | 676 |
| Yanmega | 545 | 686 |
| Quaquaval | 555 | 690 |
| Cloyster | 560 | 692 |
| Drednaw | 560 | 692 |
| Goodra | 560 | 692 |
| MissingNo. | 560 | 692 |
| Rillaboom | 570 | 696 |
| Kecleon | 575 | 698 |
| Arcanine | 600 | 708 |
| Armarouge | 600 | 708 |
| Grapploct | 610 | 712 |
| Arboliva | 620 | 716 |
| Beeheeyem | 620 | 716 |
| Delibird | 620 | 716 |
| Kingdra | 620 | 716 |
| Sigilyph | 630 | 720 |
| Blissey | 650 | 728 |
| Sirfetch'd | 650 | 728 |
| Pangoro | 660 | 732 |
| Smeargle | 660 | 732 |
| Frosmoth | 690 | 744 |
| Shiftry | 690 | 744 |
| Ludicolo | 700 | 748 |
| Gholdengo | 720 | 756 |
| Passimian | 720 | 756 |
| Pyukumuku | 720 | 756 |
| Ribombee | 720 | 756 |
| Shedinja | 720 | 756 |
| Ursaluna | 720 | 756 |
| Dragalge | 740 | 764 |
| Audino | 760 | 772 |
| Dragapult | 760 | 772 |
| Kommo-o | 760 | 772 |
| Marowak | 760 | 772 |
| Skeledirge | 760 | 772 |
| Gyarados | 780 | 780 |
| Oranguru | 780 | 780 |
| Starmie | 780 | 780 |
| Torterra | 780 | 780 |
| Zeraora | 820 | 789 |
| Vanilluxe | 840 | 793 |
| Inteleon | 850 | 796 |
| Kleavor | 850 | 796 |
| Clawitzer | 860 | 798 |
| Hitmonlee | 860 | 798 |
| Vaporeon | 870 | 800 |
| Flareon | 900 | 807 |
| Gallade | 900 | 807 |
| Octillery | 900 | 807 |
| Orbeetle | 930 | 813 |
| Skarmory | 930 | 813 |
| Venusaur | 930 | 813 |
| Thievul | 940 | 816 |
| Cryogonal | 980 | 825 |
| Decidueye | 980 | 825 |
| Emboar | 980 | 825 |
| Hawlucha | 980 | 825 |
| Heliolisk | 980 | 825 |
| Blastoise | 990 | 827 |
| Mantine | 1000 | 829 |
| Kricketune | 1010 | 831 |
| Feraligatr | 1040 | 838 |
| Hitmonchan | 1040 | 838 |
| Noivern | 1040 | 838 |
| Charizard | 1050 | 840 |
| Ambipom | 1060 | 842 |
| Kilowattrel | 1060 | 842 |
| Sawk & Throh | 1060 | 842 |
| Swanna | 1060 | 842 |
| Houndoom | 1110 | 854 |
| Blaziken | 1120 | 856 |
| Ceruledge | 1120 | 856 |
| Hitmontop | 1120 | 856 |
| Porygon-Z | 1120 | 856 |
| Espeon | 1150 | 862 |
| Arbok | 1160 | 865 |
| Weavile | 1160 | 865 |
| Banette | 1180 | 869 |
| Electrode | 1180 | 869 |
| Pikachu | 1180 | 869 |
| Sylveon | 1180 | 869 |
| Dedenne | 1190 | 871 |
| Eevee | 1210 | 876 |
| Glaceon | 1220 | 878 |
| Umbreon | 1260 | 887 |
| Greninja | 1280 | 891 |
| Leafeon | 1280 | 891 |
| Jolteon | 1310 | 898 |
| Scizor | 1480 | 936 |

## Champion categories

The Teamfight Manager 2 SDK exposes these champion categories:

- `Melee`
- `Range`
- `Magician`
- `Util`
- `Assassin`

For this mod, user-facing "caster" and "mage" map to `ChampionCategory::Magician`. The UI text can still say Caster; the code has to use the SDK enum name.

There is no `Tank` category in the SDK. Tanky Pokemon should use the closest gameplay category and add `ChampionTag::Tank`.

## Current Pokemon category mapping

- Pikachu: `Magician`, with AP/Magic tags. This favors midlane caster recommendations while his mobility and burst still make him play like a caster-assassin.
- Charizard: `Magician`.
- Blastoise: `Range`, with mixed AD/AP tags.
- Venusaur: `Util`, with Tank/CC/DOT tags.
- Eevee: `Util`.
- Jolteon: `Range`.
- Flareon: `Magician`.
- Vaporeon: `Util`, with Tank/AP tags.
- Leafeon: `Assassin`, with AD/Range/DOT tags.
- Glaceon: `Range`, with AD/Range/CC tags.
- Umbreon: `Assassin`, with AD/Melee/CC tags.

## Patchability limitation

The base game has patchable champion and action fields, but the SDK's `ModChampionInfo` interface for native mod-added champions does not expose those patchable methods directly. That means the stock in-save patch system should not be assumed to understand custom native fields such as:

- Burn chance or duration
- Paralysis chance or duration
- Pokemon type damage multipliers
- STAB values
- Wish channel scaling
- Baton Pass stat percent
- Aqua Ring heal ticks
- Custom movement, bind, poison, leech, and aura values
- True invisibility/stealth rules; Umbreon's Shadow Veil is currently implemented as a short move speed and damage-reduction buff because the SDK does not expose an invisibility setter

Those values are currently implemented by our native Rust effects. They work in-game, but they are not automatically visible as individual patch knobs to the stock patch generator.

## AI valuation hooks

`PokemonEffect` now reports custom move value through the SDK's AI hooks:

- `expected_damage`: direct damage, multi-hit damage, expected Burn/Poison/Leech damage
- `expected_heal`: Wish, Leech Seed, Aqua Ring
- `expected_cc_time`: Paralysis, bind/root, slows
- `expected_buff`: self buffs, enemy debuffs, Baton Pass, Aqua Ring debuff
- `expected_move_distance`: dashes and knockbacks
- `expected_rush_effect`: dash engage skills
- `auto_target` / `on_caster`: self-centered effects such as Rapid Spin, Fire Spin, Aqua Ring
- `can_move`: moves intentionally usable while moving

This is the important immediate bridge for match AI: it can reason about our custom effects instead of treating them as blank or direct-damage-only moves.

## Next balance-registry step

If we want save-era balance patches to mutate Pokemon-specific numbers, the next step is to move every custom numeric field into a mod-owned balance registry keyed by champion and move, for example:

- `pokemon_moba.pikachu.thunder_wave.paralysis_chance`
- `pokemon_moba.charizard.blaze.burn_duration`
- `pokemon_moba.eevee.baton_pass.stat_percent`
- `pokemon_moba.vaporeon.aqua_ring.heal_per_tick`
- `pokemon_moba.leafeon.razor_leaf.bleed_chance`
- `pokemon_moba.glaceon.snowscape.freeze_chance`
- `pokemon_moba.umbreon.dark_pulse.shadow_veil_duration`

Then all move application and AI expected-value code should read from that registry instead of storing numbers directly in `PokemonMoveEffect` variants. That will give us one place to add our own patch/import/save mechanism if the SDK does not later expose mod champion patch fields.
