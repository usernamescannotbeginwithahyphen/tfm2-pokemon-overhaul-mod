use mod_api_stable::{declare_stable_mod, LogLevel, StableHost, StableMod};

mod legacy;
mod stable_legacy_api;
mod native_vfx;
mod equipment;
mod projectiles;
mod motion;
mod combat_crit;
mod runtime_state;
mod ai_world;
mod map_terrain;
mod shared_detection;

const MOD_ID: &str = "pokemon_moba_stable";

fn init(host: &StableHost) -> StableMod {
    legacy::crash_probe::install_panic_hook();

    let version = host.game_version();
    host.log(
        LogLevel::Info,
        &format!(
            "Pokemon MOBA {} (SDK 0.6.4, native VFX event routing) loaded on game {}.{}.{}",
            env!("CARGO_PKG_VERSION"),
            version.major, version.minor, version.patch
        ),
    );

    let mut decl = StableMod::new(MOD_ID);
    decl.set_server_extension(equipment::EquipmentSettings);
    decl.set_match_hook(equipment::EquipmentSettings);
    decl.set_map_customizer(map_terrain::TerrainReader);
    native_vfx::register(&mut decl);
    projectiles::register(&mut decl);
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

declare_stable_mod!(init, requires = 10);
