use mod_api::{EntityRef, GameCtx};

use crate::pokemon_types::{PokemonType, TypeSet};

pub fn defender_types_for_target(ctx: &GameCtx, target_id: usize) -> TypeSet {
    let Some(target) = ctx.get_entity(target_id) else {
        return TypeSet::single(PokemonType::Normal);
    };

    if is_epic_objective(&target) {
        return TypeSet::single(PokemonType::Psychic);
    }

    if let Some(types) = crate::pokemon_status::entity_types(target_id) {
        return types;
    }

    TypeSet::single(PokemonType::Normal)
}

fn is_epic_objective(entity: &EntityRef<'_>) -> bool {
    if entity.is_champion() || entity.is_tower() || entity.is_minion() || !entity.is_alive() {
        return false;
    }

    let stat = entity.stat();
    let hp = entity.hp();

    hp.max >= 10_000
        && stat.hp >= 10_000
        && stat.defence >= 100
        && stat.magic_resistance >= 100
        && stat.move_speed == 0
}
