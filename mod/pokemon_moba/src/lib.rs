use mod_api::*;
mod crash_probe;
mod neutral_objectives;
mod pokemon_content;
mod pokemon_positions;
mod pokemon_statistics;
mod pokemon_status;
mod pokemon_strategy;
mod pokemon_types;
mod pokemon_ui;

const MOD_ID: &str = "pokemon_moba";

struct PokemonMobaExtension;
impl ModExtension for PokemonMobaExtension {
    fn post_update(&self, _scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        pokemon_ui::patch_position_ui(ui);
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    crash_probe::install_panic_hook();

    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(PokemonMobaExtension);
    reg.add_draft_score_hook(pokemon_positions::PokemonDraftScoreHook);
    reg.add_player_input_ai(pokemon_content::PokemonMobaInputAi);
    reg.set_server_extension(pokemon_statistics::PokemonStatisticsServerExtension);
    pokemon_content::register_champions(&mut reg);
    reg
}

declare_mod!(init);
