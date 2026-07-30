use mod_api_stable::{declare_stable_mod, LogLevel, StableHost, StableMod};

mod legacy;
mod stable_legacy_api;

const MOD_ID: &str = "pokemon_moba_stable";

fn init(host: &StableHost) -> StableMod {
    let version = host.game_version();
    host.log(
        LogLevel::Info,
        &format!(
            "Pokemon MOBA stable port loaded on game {}.{}.{}",
            version.major, version.minor, version.patch
        ),
    );

    let mut decl = StableMod::new(MOD_ID);
    let mut legacy_registration = stable_legacy_api::ModRegistration::new(MOD_ID);
    legacy::pokemon_content::register_champions(&mut legacy_registration);
    legacy_registration.into_stable(&mut decl);
    decl.add_draft_score_hook(stable_legacy_api::LegacyDraftHook {
        inner: legacy::pokemon_positions::PokemonDraftScoreHook,
    });
    decl.add_player_input_ai(stable_legacy_api::LegacyPlayerAi::new(
        legacy::pokemon_content::PokemonMobaInputAi,
    ));

    decl
}

declare_stable_mod!(init);
