# Pokemon MOBA Balance Surface

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
