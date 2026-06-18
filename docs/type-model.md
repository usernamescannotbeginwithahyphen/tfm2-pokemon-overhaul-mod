# Pokemon Type Model

Damage is modified by the move's Pokemon type against the defender's Pokemon type.

This is modeled as a final damage dealt modifier, not a permanent stat change:

- Super effective: `1.2x`
- Not very effective: `0.8x`
- Immunities from the source Pokemon chart are treated as not very effective, also `0.8x`
- Same-type attack bonus, or STAB: `1.1x`
- Neutral: `1.0x`
- Minimum modified damage: `1`

For dual-type defenders, type effectiveness stacks:

- Double super effective: `1.44x`
- Super effective plus resisted: `0.96x`
- Double resisted: `0.64x`

STAB applies separately from effectiveness, so a Fire Pokemon using a Fire move against a Grass/Ice defender deals `1.1 * 1.2 * 1.2 = 1.584x` final damage.

Native move implementations should route damage through `deal_pokemon_damage(...)` in `src/pokemon_types.rs` once champion actions are implemented.
