# Stable Champion Migration Audit

SDK 0.6.4 / build 0.2.8 source inventory. 103 champions, 412 actions, 103 passive descriptions.
Run `cargo test --offline` in the stable package before regenerating to refresh the guide export.
Content SHA-256 (normalized text): `6255edaac759cfe0ebc5bfd73247ceb38023b68efd481cd56862cb6a6e239073`.

This is an exhaustive inventory and review checklist, NOT 103 live-tested champions.
All rows inherit the shared runtime, damage/statistics, AI, draft and deployment checks.
Family tags are mechanically assigned review prompts, not confirmed per-champion defects.
Finding codes and evidence: [full SDK audit](sdk-0.6.4-findings.md#whole-mod-audit-follow-up).
Shared fixes and remaining limits: [0.2.4 implementation](sdk-0.6.4-findings.md#build-024-audit-fixes).
Current fixes and coverage limits: [0.2.7 combat semantics](sdk-0.6.4-findings.md#build-027-combat-semantics-fixes).
VFX counts are staged asset bindings, not proof of correct runtime playback. P = projectile assets.
Actions are listed in attack / skill / skill2 / ultimate order; repeated variants are intentional.

| Champion | Four action implementations | Passive | VFX / P | Review families | Specific follow-up |
| --- | --- | --- | ---: | --- | --- |
| Pikachu (`pokemon_moba_pikachu`) | `DirectDamage` / `DashDamage` / `TravelingWaveDamageParalysis` / `AreaDamage` | Static Shock | 4 / 1 | M1/A2, F1 | Shared checks + family-specific regression tests. |
| Charizard (`pokemon_moba_charizard`) | `ConeDamage` / `ConeDamage` / `LineFirstDamage` / `AreaDamage` | Blaze | 3 / 1 | M1/A2, F1 | Shared checks + family-specific regression tests. |
| Blastoise (`pokemon_moba_blastoise`) | `MultiHitDamage` / `SelfBuffAreaDamage` / `DirectDamageKnockback` / `LinePierceDamage` | Mega Launcher | 4 / 2 | C3, M1/A2, F1 | Shared checks + family-specific regression tests. |
| Feraligatr (`pokemon_moba_feraligatr`) | `DirectDamage` / `DirectDamageKnockback` / `SelfOffenseBuff` / `LinePierceDamageSlowApToAd` | Spiked Hide | 1 / 0 | C3, M1/A2 | Shared checks + family-specific regression tests. |
| Emboar (`pokemon_moba_emboar`) | `DirectDamage` / `DirectDamageStun` / `LineFirstDashDamageDistanceSelfHp` / `PowerUpPunchChannel` | Reckless | 0 / 0 | M1/A2, A1 | Shared checks + family-specific regression tests. |
| Blaziken (`pokemon_moba_blaziken`) | `MultiHitDamage` / `SelfBuffAreaDamageKnockbackChanceBurn` / `SelfOffenseBuff` / `LineFirstDashDamageFlameTrail` | Speed Boost | 1 / 0 | M1/A2, F1 | Shared checks + family-specific regression tests. |
| Greninja (`pokemon_moba_greninja`) | `DirectDamage` / `DashDamageReturn` / `AreaDamage` / `LinePierceDamage` | Battle Bond | 4 / 1 | M1/A2, A1, F1 | Shared checks + family-specific regression tests. |
| Decidueye (`pokemon_moba_decidueye`) | `TripleArcDamage` / `DirectDamage` / `LinePierceDamageSelfRetreat` / `DirectDamageSpiritShackle` | Long Reach | 4 / 1 | M1/A2, A1 | Shared checks + family-specific regression tests. |
| Inteleon (`pokemon_moba_inteleon`) | `DirectDamage` / `SelfOffenseCritBuff` / `AreaSoak` / `LineExecute` | Sniper | 4 / 1 | C3, F1 | Shared checks + family-specific regression tests. |
| Skeledirge (`pokemon_moba_skeledirge`) | `DirectDamage` / `TargetSkillBlock` / `DirectBurn` / `SelfAreaDamageBurn` | Where There's Smoke There's Fire | 4 / 2 | F1 | Shared checks + family-specific regression tests. |
| Porygon-Z (`pokemon_moba_porygonz`) | `ConversionDamage` / `Conversion2Damage` / `SelfHealResetPorygonType` / `TriAttack` | Adaptability | 4 / 2 | C1/C2 | Shared checks + family-specific regression tests. |
| Blissey (`pokemon_moba_blissey`) | `DirectDamage` / `BlisseyHealAura` / `CharmHeal` / `GlobalAllyHealPercent` | Healer | 0 / 0 | C1/C2, F1 | C1/C2 fixed 0.2.4: one anti-heal owner and native/custom cleanse; live engine verification. |
| Kleavor (`pokemon_moba_kleavor`) | `DirectDamageEveryThirdBleed` / `MultiHitDamage` / `StealthRockBuff` / `DirectDamageBindRecoilCrit` | Sharpness | 0 / 0 | C3, S1 | S1: Stealth Rock uses custom alternating untargetability; do not blindly replace with banish. |
| Scizor (`pokemon_moba_scizor`) | `DirectDamageSpeedScale` / `SelfSpeedCritBuff` / `DirectDamageSpeedCheckCrit` / `DirectDamage` | Light Metal | 1 / 0 | C3, A1 | 0.2.7 Light Metal uses native shield without custom double absorption; live engine-origin hits need validation. |
| Ursaluna (`pokemon_moba_ursaluna`) | `DirectDamage` / `TargetTerrify` / `EarthquakeAura` / `BloodMoon` | Guts | 3 / 0 | F1 | C3/F1: Blood Moon combat rules preserved; no public full-model swapping API found. |
| Sawk & Throh (`pokemon_moba_sawk_throh`) | `SawkThrohBasic` / `SawkThrohSwitch` / `SawkThrohSkill2` / `SawkThrohUlt` | Inner Focus | 0 / 0 | M1/A2 | A1/F1: state-dependent cast gates available; full-model swapping not exposed. |
| Hitmonchan (`pokemon_moba_hitmonchan`) | `DirectDamageDrain` / `DirectDamageChanceFreeze` / `DirectDamageChanceBurn` / `DirectDamageParalysisFor` | Iron Fist | 0 / 0 | Shared baseline | Shared checks + family-specific regression tests. |
| Hitmonlee (`pokemon_moba_hitmonlee`) | `MultiHitDamage` / `DirectDamageStun` / `MeleeArcDamage` / `DirectDamageSelfSlow` | Limber | 0 / 0 | C3 | Shared checks + family-specific regression tests. |
| Hitmontop (`pokemon_moba_hitmontop`) | `DirectDamage` / `DirectDamage` / `TargetDefenseDebuff` / `MultiHitDamage` | Steadfast | 0 / 0 | Shared baseline | Shared checks + family-specific regression tests. |
| Kilowattrel (`pokemon_moba_kilowattrel`) | `DirectDamage` / `LineFirstDamage` / `LinePierceDamage` / `SelfBuffAreaDamage` | Volt Absorb | 4 / 2 | C1/C2, F1 | Shared checks + family-specific regression tests. |
| Beeheeyem (`pokemon_moba_beeheeyem`) | `GuardSplitDamage` / `DirectBindBoth` / `LightScreen` / `DirectDamageConfusion` | Synchronize | 3 / 1 | C3 | Shared checks + family-specific regression tests. |
| Gyarados (`pokemon_moba_gyarados`) | `WhirlpoolField` / `DirectDamageChanceBurnBypassImmune` / `TravelingWaveDamage` / `LinePierceDamage` | Lake of Rage | 4 / 1 | C1/C2, C3, F1 | Shared checks + family-specific regression tests. |
| Noivern (`pokemon_moba_noivern`) | `DirectDamageDefenseDebuff` / `LinePierceDamageStun` / `LinePierceDamage` / `DirectDamage` | Infiltrator | 4 / 1 | Shared baseline | Explicitly deferred: Infiltrator unchanged; no tower-aggro interception API found. |
| Mantine (`pokemon_moba_mantine`) | `LineFirstDamage` / `BrineField` / `DirectDamageStun` / `TravelingWaveDamage` | Swift Swim | 4 / 1 | F1 | 0.2.7 Swift Swim refreshes one named speed buff; live water entry/exit validation remains. |
| Cryogonal (`pokemon_moba_cryogonal`) | `DirectDamage` / `TravelingWaveDamageChanceFreeze` / `DirectDamage` / `LinePierceDamageChanceFreeze` | Immunity | 4 / 1 | C3 | Shared checks + family-specific regression tests. |
| Vanilluxe (`pokemon_moba_vanilluxe`) | `DirectDamage` / `TravelingWaveDamageChanceFreeze` / `ConeDamageFreeze` / `SelfAreaDamageFreezeAll` | Snow Patrol | 4 / 1 | C3, F1 | Shared checks + family-specific regression tests. |
| Skarmory (`pokemon_moba_skarmory`) | `DirectDamage` / `Roost` / `MultiHitDamage` / `FlyAllySplash` | Sturdy | 0 / 0 | M1/A2 | M1 fixed 0.2.4: native Fly teleport; Sturdy covers Pokemon resolver hits only (C3). |
| Houndoom (`pokemon_moba_houndoom`) | `DirectDamage` / `DirectDamage` / `TargetDefenseDebuffStun` / `AreaDamageBurn` | Intimidate | 4 / 2 | F1 | Shared checks + family-specific regression tests. |
| Arbok (`pokemon_moba_arbok`) | `DirectDamageEverySecondMiasma` / `LinePierceDamageMiasmaCoil` / `AreaMiasmaSlow` / `SelfAreaMiasmaDetonate` | Dripping Fangs | 1 / 1 | C1/C2, A1, F1 | Shared checks + family-specific regression tests. |
| Clawitzer (`pokemon_moba_clawitzer`) | `DirectDamage` / `TripleAreaDamage` / `DirectDamage` / `LinePierceDamage` | Clingy | 4 / 2 | A1, F1, S1 | A1/S1: attachment target/recall gates; soft untargetability not engine-wide immunity. |
| Octillery (`pokemon_moba_octillery`) | `MultiHitDamage` / `TargetLockOn` / `BouncingAreaDamage` / `LineFirstDamage` | Suction Cups | 4 / 2 | F1 | M2 fixed 0.2.4: ignore_wall and -15% speed; route selection needs testing. |
| Pyukumuku (`pokemon_moba_pyukumuku`) | `SelfProtectDefenseBuff` / `TargetPoison` / `SelfSleepHeal` / `PainSplit` | Innards Out | 0 / 0 | C1/C2, C3, A1 | Shared checks + family-specific regression tests. |
| Banette (`pokemon_moba_banette`) | `DirectDamage` / `TargetSwapDamage` / `ConeDamageBlind` / `DirectDamageSplashOnKill` | Cursed Body | 3 / 0 | M1/A2 | M1 fixed 0.2.4: native Phantom Force teleport; live landing test. |
| Kricketune (`pokemon_moba_kricketune`) | `DirectDamage` / `StickyWebField` / `SingAura` / `BugBuzzAuraConfusion` | Web Walker | 1 / 0 | M1/A2, F1 | Shared detection wired; 0.2.7 named web refresh and immediate field leases. Live overlap/lifecycle validation remains. |
| Ambipom (`pokemon_moba_ambipom`) | `MultiHitDamage` / `ApplyConfusion` / `TargetStatSteal` / `SelfOffenseCritBuff` | Technician | 0 / 0 | C3 | I1: equipment Thief implemented; Technician still Pokemon buff ledger. |
| Gallade (`pokemon_moba_gallade`) | `DirectDamage` / `TargetDamageReduction` / `SelfCleanseOffenseBuff` / `MeleeArcDamage` | Justified | 1 / 0 | C1/C2 | Shared checks + family-specific regression tests. |
| Audino (`pokemon_moba_audino`) | `DirectDamage` / `AllyDamageRedirect` / `AllySwapHeal` / `EndureBuff` | Regenerator | 0 / 0 | C1/C2, C3, M1/A2 | M1 fixed 0.2.4: native Substitute position swap; Protect redirection preserved (C3). |
| Pangoro (`pokemon_moba_pangoro`) | `DirectDamage` / `TargetTauntSelfDefenseBuff` / `DirectDamageSelfRetreat` / `SelfAreaDamageThrow` | Scrappy | 0 / 0 | M1/A2, A1, F1 | Shared checks + family-specific regression tests. |
| Passimian (`pokemon_moba_passimian`) | `DirectDamage` / `SelfAttackDefenseBuff` / `ReversalGuard` / `DirectDamage` | Receiver | 2 / 2 | C3 | Copied CS payout, Inteleon/Emboar named refresh and Shiftry updater wired; full copied-passive live acceptance remains. |
| Oranguru (`pokemon_moba_oranguru`) | `DirectDamage` / `NastyPlot` / `StoredPower` / `Instruct` | Symbiosis | 4 / 1 | A1 | I1: equipment-derived Symbiosis implemented; no item-proc sharing; copied Instruct targeting needs A1. |
| Dragalge (`pokemon_moba_dragalge`) | `DirectDamageChancePoison` / `SelfAreaDamageKnockbackChancePoison` / `TripleAreaDamagePoison` / `TargetPoisonExecute` | Poison Sipper | 4 / 1 | C1/C2, M1/A2, F1 | Shared checks + family-specific regression tests. |
| Heliolisk (`pokemon_moba_heliolisk`) | `DirectDamage` / `LineDashPierceDamage` / `ShedTailTauntDash` / `SelfAreaDamageParalysis` | Discharge | 2 / 0 | M1/A2, A1, F1 | 0.2.7 separate Shed Tail unit/taunt and swept piercing dash; live summon/body/lifetime validation remains. |
| Turtonator (`pokemon_moba_turtonator`) | `DirectDamage` / `ShellTrap` / `SunnyDay` / `SelfAreaDamage` | Burning Jealousy | 5 / 1 | F1 | Shared checks + family-specific regression tests. |
| Ribombee (`pokemon_moba_ribombee`) | `PollenPuff` / `LinePierceDamageDrain` / `RescueAllyDash` / `AlluringVoiceAura` | Honey Gatherer | 4 / 1 | C1/C2, M1/A2, A1, F1 | Shared checks + family-specific regression tests. |
| Drampa (`pokemon_moba_drampa`) | `LinePierceDamageStun` / `SelfAttackDualDefenseBuff` / `HpCostDirectDamage` / `AreaDamage` | Berserk | 4 / 0 | C3, A1, F1 | Shared checks + family-specific regression tests. |
| Kommo-o (`pokemon_moba_kommoo`) | `DirectDamageTargetMaxHp` / `AreaAttackDefenseDebuff` / `SelfHpCostOffenseSpeedBuff` / `KommooCloseCombatDuel` | Overcoat | 0 / 0 | C3, F1 | Shared checks + family-specific regression tests. |
| Thievul (`pokemon_moba_thievul`) | `DirectDamage` / `DirectDamageAntiHeal` / `SelfCritAntiHealBuff` / `StakeoutField` | Heal Block | 2 / 0 | C1/C2, C3, F1 | C1 fixed 0.2.4: strongest-source named anti-heal with expiry tests; live engine verification. |
| Archaludon (`pokemon_moba_archaludon`) | `DirectDamage` / `SelfDefenseBuff` / `SelfAttackDualDefenseBuff` / `SelfAreaDamage` | Stalwart | 1 / 0 | C3, M1/A2, F1 | Shared checks + family-specific regression tests. |
| Appletun (`pokemon_moba_appletun`) | `DirectDamage` / `TargetDefenseDebuff` / `AreaAttackDefenseDebuff` / `LinePierceDamage` | Ripen | 2 / 2 | C1/C2, F1 | C1: Ripen amplification, HP cap and anti-heal order need engine tests. |
| Goodra (`pokemon_moba_goodra`) | `DirectDamageChanceConfusion` / `RainDance` / `DragonCheerAura` / `LifeDewPickups` | Sap Sipper | 5 / 0 | F1 | Shared checks + family-specific regression tests. |
| Dedenne (`pokemon_moba_dedenne`) | `DirectDamageChanceParalysis` / `DedennePlayRough` / `DedenneParabolicDischarge` / `DedenneElectricTerrain` | Static | 7 / 0 | A1, F1 | 0.2.7 grid refresh, configured node damage and cooldown gate fixed. Bespoke Play Rough timing remains separate; live grid acceptance pending. |
| Hawlucha (`pokemon_moba_hawlucha`) | `DirectDamage` / `ConeDamage` / `HawluchaCounter` / `HawluchaFlyingPress` | Momentum | 1 / 0 | M1/A2 | Shared checks + family-specific regression tests. |
| Bouffalant (`pokemon_moba_bouffalant`) | `DirectDamage` / `BouffalantRetaliate` / `BouffalantStompingTantrum` / `BouffalantHeadCharge` | Afro | 0 / 0 | A1 | Shared checks + family-specific regression tests. |
| Starmie (`pokemon_moba_starmie`) | `DirectDamage` / `StarmieFlipTurn` / `StarmieHeartSwap` / `StarmiePsybeam` | Illuminate | 4 / 1 | M1/A2, A1, S1 | S1/F1: Illuminate reveal, mark size/anchor/lifetime verification. |
| Drednaw (`pokemon_moba_drednaw`) | `DirectDamage` / `DirectDamageBind` / `SelfAttackDualDefenseBuff` / `SelfAreaDamageRetreatLine` | Wide Guard | 3 / 0 | C3, M1/A2, F1 | Shared checks + family-specific regression tests. |
| Orbeetle (`pokemon_moba_orbeetle`) | `DirectDamage` / `ChainDashDamage` / `TravelingWaveDamageInfestation` / `ConeDamageConfusion` | Telepathy | 2 / 0 | M1/A2 | Shared checks + family-specific regression tests. |
| Coalossal (`pokemon_moba_coalossal`) | `DirectDamage` / `TravelingWaveDamageSlowBurn` / `SmokeScreen` / `CoalossalMagmaStorm` | Steam Engine | 4 / 1 | Shared baseline | Shared checks + family-specific regression tests. |
| Magmortar (`pokemon_moba_magmortar`) | `DirectDamage` / `DirectDamageSplash` / `MagmortarHeatWave` / `MagmortarEruption` | Overheat | 4 / 3 | Shared baseline | P1: per-shot impacts implemented; launch-vs-impact stat sampling changed. |
| Grapploct (`pokemon_moba_grapploct`) | `MultiHitDamage` / `SelfDetectGuard` / `SubmissionBindDot` / `SubmissionBindDot` | Boxer | 0 / 0 | C3 | Shared checks + family-specific regression tests. |
| Sirfetch'd (`pokemon_moba_sirfetchd`) | `DirectDamage` / `DirectDamage` / `LineFirstDamage` / `DirectDamageSpeedCheckCrit` | Leek | 1 / 1 | C3 | Shared checks + family-specific regression tests. |
| Arboliva (`pokemon_moba_arboliva`) | `DirectDamageSplash` / `DirectDamage` / `TrailblazeGrassyTerrain` / `LinePierceDamageChanceBurn` | Seed Sower | 5 / 1 | A1, F1 | Shared checks + family-specific regression tests. |
| Armarouge (`pokemon_moba_armarouge`) | `DirectDamage` / `SelfAreaDamageBurnConfusion` / `DirectDamageStun` / `LinePierceDamageSelfDefenseDebuff` | Weak Armor | 2 / 0 | A1, F1 | Shared checks + family-specific regression tests. |
| Ceruledge (`pokemon_moba_ceruledge`) | `DirectDamageChanceStun` / `LineFirstDashDamageFlameTrail` / `TargetItemScaledDamage` / `DirectDamageExecuteThreshold` | Flash Fire | 1 / 0 | M1/A2, F1 | I1: equipment-derived Poltergeist implemented; validate equipment changes/expiry. |
| Gholdengo (`pokemon_moba_gholdengo`) | `DirectDamage` / `TravelingWaveDamage` / `AreaDamageDefenseDebuff` / `AreaDamage` | Good as Gold | 4 / 1 | F1 | G1: +10 credited-CS callback implemented; duplicate/death/replay tests remain. |
| Frosmoth (`pokemon_moba_frosmoth`) | `AreaDamage` / `TravelingWaveDamageChanceFreeze` / `SleepPowderCircle` / `TravelingWaveDamage` | Ice Scales | 4 / 0 | A1, F1 | Shared checks + family-specific regression tests. |
| Shedinja (`pokemon_moba_shedinja`) | `DirectDamage` / `DashEndConeDamage` / `GrudgeDamage` / `TravelingWaveDamageTerrify` | Wonder Guard | 5 / 2 | C3, M1/A2 | 0.2.7 dash-cone resolves on arrival; missing damage classification fixed. Wonder Guard remains custom pre-damage logic. |
| Ludicolo (`pokemon_moba_ludicolo`) | `DirectDamage` / `ConeTauntThenConfuse` / `SingAura` / `DirectDamageStun` | Rain Dish | 5 / 0 | C1/C2, F1 | Shared checks + family-specific regression tests. |
| Kingdra (`pokemon_moba_kingdra`) | `DirectDamage` / `AreaDamage` / `KingdraDragonDance` / `DirectDamage` | King's Rock | 4 / 2 | C3, A1, F1 | Shared checks + family-specific regression tests. |
| Delibird (`pokemon_moba_delibird`) | `PresentDamage` / `PresentHealPercent` / `PresentHealPercent` / `DirectDamageChanceExecute` | Hustle | 2 / 1 | C1/C2, C3 | Shared checks + family-specific regression tests. |
| Cloyster (`pokemon_moba_cloyster`) | `DirectDamageChanceFreeze` / `CloysterIcicleCrash` / `CloysterClamp` / `DirectDamageChanceFreeze` | Overcoat | 3 / 2 | C3 | Shared checks + family-specific regression tests. |
| Electrode (`pokemon_moba_electrode`) | `DirectDamage` / `ElectrodeMagnetRise` / `ElectrodeMagneticFlux` / `ElectrodeSelfDestruct` | Aftermath | 1 / 0 | Shared baseline | 0.2.7 post-respawn speed outcome fixed and one-shot tested; no public respawn-countdown setter. |
| Snorlax (`pokemon_moba_snorlax`) | `DirectDamage` / `SnorlaxSwallow` / `SnorlaxBellyDrum` / `SnorlaxSnore` | Gluttony | 0 / 0 | C1/C2, M1/A2, A1 | Shared checks + family-specific regression tests. |
| Zeraora (`pokemon_moba_zeraora`) | `MultiHitDamage` / `ZeraoraThunderCage` / `ZeraoraZingZap` / `ZeraoraWildCharge` | Merciless | 4 / 0 | A1 | Shared checks + family-specific regression tests. |
| Rillaboom (`pokemon_moba_rillaboom`) | `DirectDamage` / `RillaboomDrumBeating` / `RillaboomDrumRoll` / `RillaboomGrassySurge` | Drum Solo | 3 / 0 | C1/C2 | Shared checks + family-specific regression tests. |
| Dragapult (`pokemon_moba_dragapult`) | `DirectDamage` / `MultiHitDamage` / `LineDashPierceContactDrain` / `DirectDamageTerrify` | Dragon Launcher | 5 / 4 | C1/C2, M1/A2 | P1: Dreepy callback flight, spawn origin and source/copy ownership tests. |
| Shiftry (`pokemon_moba_shiftry`) | `DirectDamage` / `ConeForceAwaySlow` / `DirectDamage` / `ShiftryTornado` | Forest Camouflage | 2 / 0 | M1/A2, F1, S1 | Map-grid occupancy, named speed refresh and Receiver camouflage updater wired. Live map validation pending. |
| Sigilyph (`pokemon_moba_sigilyph`) | `DirectDamage` / `ConeDamage` / `SigilyphGravity` / `DirectDamageConfusion` | Glypher | 5 / 2 | Shared baseline | Shared checks + family-specific regression tests. |
| Weavile (`pokemon_moba_weavile`) | `DirectDamageChanceFreeze` / `WeavileScratchAndShuffle` / `WeavilePursuitClaw` / `WeavileAssaultingHunt` | Lone Predator | 0 / 0 | Shared baseline | S1 improved 0.2.4: native Hunt invisibility plus custom protection; reveal/break live tests. |
| Swanna (`pokemon_moba_swanna`) | `DirectDamage` / `DirectDamageBleed` / `SwannaFeatheryCyclone` / `SwannaSkyCircus` | Tailwind | 3 / 1 | A1 | Shared checks + family-specific regression tests. |
| Marowak (`pokemon_moba_marowak`) | `MarowakBonemerang` / `MarowakBoneRush` / `DirectDamageStun` / `MarowakBoneWindmill` | Rhythm | 2 / 1 | A1 | P1: outbound/return collisions, catch timing and Rhythm refresh require live tests. |
| Garganacl (`pokemon_moba_garganacl`) | `DirectDamage` / `GarganaclLandCrush` / `GarganaclPurifyingSalt` / `GarganaclBlessedSalt` | Salt Cure | 3 / 0 | F1 | Shared checks + family-specific regression tests. |
| Ampharos (`pokemon_moba_ampharos`) | `DirectDamage` / `AmpharosFlash` / `AmpharosSearchlightTail` / `AmpharosGigavolt` | Luminous Pulse | 5 / 1 | M1/A2, F1, S1 | 0.2.6 continuous shared team detection and Weavile reveal wired; live host attack acceptance pending. |
| Xatu (`pokemon_moba_xatu`) | `DirectDamage` / `XatuMindBend` / `XatuPainAmplifier` / `XatuSuperPsy` | Prophecy | 4 / 1 | M1/A2, A1, S1 | S1: custom reveal must coordinate with any native invisibility migration. |
| Quaquaval (`pokemon_moba_quaquaval`) | `QuaquavalHydroKick` / `QuaquavalSpiralShot` / `QuaquavalUpTempo` / `QuaquavalExcitingDance` | Aqua Step | 5 / 1 | F1 | Shared checks + family-specific regression tests. |
| Arcanine (`pokemon_moba_arcanine`) | `DirectDamage` / `ArcanineExtremespeed` / `ArcanineWhiteFlames` / `ArcanineFlamesOfRage` | Blazing Mane | 3 / 1 | C1/C2, A1 | Shared checks + family-specific regression tests. |
| MissingNo. (`pokemon_moba_missingno`) | `MissingNoGlitchBasic` / `MissingNoRandomStatus` / `MissingNoGlitchStorm` / `MissingNoTrickRoom` | Old man glitch | 6 / 1 | A1 | Shared checks + family-specific regression tests. |
| Kecleon (`pokemon_moba_kecleon`) | `DirectDamage` / `KecleonDoubleScratch` / `KecleonChromashift` / `KecleonColorfulWhip` | Energy Predator | 2 / 0 | Shared baseline | Shared checks + family-specific regression tests. |
| Yanmega (`pokemon_moba_yanmega`) | `LinePierceDamage` / `YanmegaBuzzingBoost` / `YanmegaTintedLens` / `YanmegaGigaDrain` | Ogre Darner's Bug Blood | 3 / 0 | C1/C2, C3 | C3: drain immunity is Pokemon-layer-only; engine item lifesteal requires verification. |
| Weezing (`pokemon_moba_weezing`) | `LinePierceDamageChancePoison` / `LinePierceDamageStun` / `WeezingBodyOdor` / `WeezingSludgeWhirlpool` | Nullifying Gas | 5 / 0 | F1 | C3: Nullifying Gas suppression must cover copied passives and all new callbacks. |
| Wishiwashi (`pokemon_moba_wishiwashi`) | `DirectDamage` / `WishiwashiWaveSplash` / `WishiwashiCowardice` / `WishiwashiMassiveCatch` | Schooling | 4 / 1 | A1 | C3/F1: native shields supported; Schooling state and overlays are not body swaps. |
| Comfey (`pokemon_moba_comfey`) | `SelfAllStatsBuff` / `HealAttachedAllyPercent` / `GrassPowerAura` / `EncoreAttachedAlly` | Flower Veil | 1 / 0 | C1/C2, C3, A1, F1, S1 | A1/S1: attachment target/recall gates; retain casting while attached; Encore is currently haste. |
| Smeargle (`pokemon_moba_smeargle`) | `Sketch` / `Sketch` / `Sketch` / `Sketch` | Sketch | 0 / 0 | A1 | 0.2.7 preserves copied move element/art with actual caster STAB; persistent copies still need live tests. |
| Torterra (`pokemon_moba_torterra`) | `DirectDamageDrain` / `SelfDefenseBuff` / `SelfHeal` / `LineFirstDashDamageMissingHp` | Shell Armor | 0 / 0 | C1/C2, C3, M1/A2 | Shared checks + family-specific regression tests. |
| Venusaur (`pokemon_moba_venusaur`) | `DirectDamage` / `LineFirstBindBoth` / `LineFirstLeechSeed` / `TravelingWavePoisonSlow` | Tangling Vines | 3 / 1 | Shared baseline | Shared checks + family-specific regression tests. |
| Eevee (`pokemon_moba_eevee`) | `DirectDamage` / `TargetAttackDebuffProtect` / `WishChannel` / `BatonPass` | Helping Hand | 3 / 0 | A1 | Shared checks + family-specific regression tests. |
| Jolteon (`pokemon_moba_jolteon`) | `MultiHitDamage` / `TargetOffenseDebuff` / `DirectDamage` / `DashDamageChanceParalysis` | Eeveelution | 4 / 2 | M1/A2, A1 | Shared checks + family-specific regression tests. |
| Flareon (`pokemon_moba_flareon`) | `DirectDamageChanceBurn` / `SelfBuffAreaDamageKnockbackChanceBurn` / `LinePierceDamageChanceBurn` / `DashDamageChanceBurn` | Eeveelution | 4 / 1 | M1/A2, A1, F1 | Shared checks + family-specific regression tests. |
| Vaporeon (`pokemon_moba_vaporeon`) | `LinePierceDamage` / `TargetAttackDebuffByDefense` / `LineFirstDamage` / `AquaRing` | Eeveelution | 4 / 2 | A1 | Shared checks + family-specific regression tests. |
| Leafeon (`pokemon_moba_leafeon`) | `DirectDamageChanceBleed` / `DirectDamageIgnoreType` / `DirectDamageBleed` / `AreaDamageSelfSlow` | Eeveelution | 4 / 2 | A1, F1 | Shared checks + family-specific regression tests. |
| Glaceon (`pokemon_moba_glaceon`) | `DirectDamage` / `IceField` / `DashDamageChanceFreeze` / `IceField` | Eeveelution | 4 / 1 | M1/A2, A1, F1 | Shared checks + family-specific regression tests. |
| Umbreon (`pokemon_moba_umbreon`) | `DirectDamage` / `AreaDamageDefenseDebuff` / `LineFirstDamageSelfBuff` / `DashDamageChanceDisarm` | Eeveelution | 3 / 0 | M1/A2, A1, F1 | Shared checks + family-specific regression tests. |
| Espeon (`pokemon_moba_espeon`) | `DirectDamage` / `ApplyConfusion` / `ForceSelfCrit` / `FutureSight` | Eeveelution | 2 / 1 | C3, M1/A2, A1 | Shared checks + family-specific regression tests. |
| Sylveon (`pokemon_moba_sylveon`) | `AreaDamageAttackDebuff` / `MistyTerrain` / `CharmHeal` / `DazzlingGleam` | Eeveelution | 4 / 0 | C1/C2, A1, F1 | Shared checks + family-specific regression tests. |

## Shared Effect Coverage

Every action variant below inherits the shared resolver/adapter audit; occurrence counts are not test counts.

| Effect | Roster actions |
| --- | ---: |
| `AlluringVoiceAura` | 1 |
| `AllyDamageRedirect` | 1 |
| `AllySwapHeal` | 1 |
| `AmpharosFlash` | 1 |
| `AmpharosGigavolt` | 1 |
| `AmpharosSearchlightTail` | 1 |
| `ApplyConfusion` | 2 |
| `AquaRing` | 1 |
| `ArcanineExtremespeed` | 1 |
| `ArcanineFlamesOfRage` | 1 |
| `ArcanineWhiteFlames` | 1 |
| `AreaAttackDefenseDebuff` | 2 |
| `AreaDamage` | 7 |
| `AreaDamageAttackDebuff` | 1 |
| `AreaDamageBurn` | 1 |
| `AreaDamageDefenseDebuff` | 2 |
| `AreaDamageSelfSlow` | 1 |
| `AreaMiasmaSlow` | 1 |
| `AreaSoak` | 1 |
| `BatonPass` | 1 |
| `BlisseyHealAura` | 1 |
| `BloodMoon` | 1 |
| `BouffalantHeadCharge` | 1 |
| `BouffalantRetaliate` | 1 |
| `BouffalantStompingTantrum` | 1 |
| `BouncingAreaDamage` | 1 |
| `BrineField` | 1 |
| `BugBuzzAuraConfusion` | 1 |
| `ChainDashDamage` | 1 |
| `CharmHeal` | 2 |
| `CloysterClamp` | 1 |
| `CloysterIcicleCrash` | 1 |
| `CoalossalMagmaStorm` | 1 |
| `ConeDamage` | 4 |
| `ConeDamageBlind` | 1 |
| `ConeDamageConfusion` | 1 |
| `ConeDamageFreeze` | 1 |
| `ConeForceAwaySlow` | 1 |
| `ConeTauntThenConfuse` | 1 |
| `Conversion2Damage` | 1 |
| `ConversionDamage` | 1 |
| `DashDamage` | 1 |
| `DashDamageChanceBurn` | 1 |
| `DashDamageChanceDisarm` | 1 |
| `DashDamageChanceFreeze` | 1 |
| `DashDamageChanceParalysis` | 1 |
| `DashDamageReturn` | 1 |
| `DashEndConeDamage` | 1 |
| `DazzlingGleam` | 1 |
| `DedenneElectricTerrain` | 1 |
| `DedenneParabolicDischarge` | 1 |
| `DedennePlayRough` | 1 |
| `DirectBindBoth` | 1 |
| `DirectBurn` | 1 |
| `DirectDamage` | 71 |
| `DirectDamageAntiHeal` | 1 |
| `DirectDamageBind` | 1 |
| `DirectDamageBindRecoilCrit` | 1 |
| `DirectDamageBleed` | 2 |
| `DirectDamageChanceBleed` | 1 |
| `DirectDamageChanceBurn` | 2 |
| `DirectDamageChanceBurnBypassImmune` | 1 |
| `DirectDamageChanceConfusion` | 1 |
| `DirectDamageChanceExecute` | 1 |
| `DirectDamageChanceFreeze` | 4 |
| `DirectDamageChanceParalysis` | 1 |
| `DirectDamageChancePoison` | 1 |
| `DirectDamageChanceStun` | 1 |
| `DirectDamageConfusion` | 2 |
| `DirectDamageDefenseDebuff` | 1 |
| `DirectDamageDrain` | 2 |
| `DirectDamageEverySecondMiasma` | 1 |
| `DirectDamageEveryThirdBleed` | 1 |
| `DirectDamageExecuteThreshold` | 1 |
| `DirectDamageIgnoreType` | 1 |
| `DirectDamageKnockback` | 2 |
| `DirectDamageParalysisFor` | 1 |
| `DirectDamageSelfRetreat` | 1 |
| `DirectDamageSelfSlow` | 1 |
| `DirectDamageSpeedCheckCrit` | 2 |
| `DirectDamageSpeedScale` | 1 |
| `DirectDamageSpiritShackle` | 1 |
| `DirectDamageSplash` | 2 |
| `DirectDamageSplashOnKill` | 1 |
| `DirectDamageStun` | 6 |
| `DirectDamageTargetMaxHp` | 1 |
| `DirectDamageTerrify` | 1 |
| `DragonCheerAura` | 1 |
| `EarthquakeAura` | 1 |
| `ElectrodeMagnetRise` | 1 |
| `ElectrodeMagneticFlux` | 1 |
| `ElectrodeSelfDestruct` | 1 |
| `EncoreAttachedAlly` | 1 |
| `EndureBuff` | 1 |
| `FlyAllySplash` | 1 |
| `ForceSelfCrit` | 1 |
| `FutureSight` | 1 |
| `GarganaclBlessedSalt` | 1 |
| `GarganaclLandCrush` | 1 |
| `GarganaclPurifyingSalt` | 1 |
| `GlobalAllyHealPercent` | 1 |
| `GrassPowerAura` | 1 |
| `GrudgeDamage` | 1 |
| `GuardSplitDamage` | 1 |
| `HawluchaCounter` | 1 |
| `HawluchaFlyingPress` | 1 |
| `HealAttachedAllyPercent` | 1 |
| `HpCostDirectDamage` | 1 |
| `IceField` | 2 |
| `Instruct` | 1 |
| `KecleonChromashift` | 1 |
| `KecleonColorfulWhip` | 1 |
| `KecleonDoubleScratch` | 1 |
| `KingdraDragonDance` | 1 |
| `KommooCloseCombatDuel` | 1 |
| `LifeDewPickups` | 1 |
| `LightScreen` | 1 |
| `LineDashPierceContactDrain` | 1 |
| `LineDashPierceDamage` | 1 |
| `LineExecute` | 1 |
| `LineFirstBindBoth` | 1 |
| `LineFirstDamage` | 6 |
| `LineFirstDamageSelfBuff` | 1 |
| `LineFirstDashDamageDistanceSelfHp` | 1 |
| `LineFirstDashDamageFlameTrail` | 2 |
| `LineFirstDashDamageMissingHp` | 1 |
| `LineFirstLeechSeed` | 1 |
| `LinePierceDamage` | 9 |
| `LinePierceDamageChanceBurn` | 2 |
| `LinePierceDamageChanceFreeze` | 1 |
| `LinePierceDamageChancePoison` | 1 |
| `LinePierceDamageDrain` | 1 |
| `LinePierceDamageMiasmaCoil` | 1 |
| `LinePierceDamageSelfDefenseDebuff` | 1 |
| `LinePierceDamageSelfRetreat` | 1 |
| `LinePierceDamageSlowApToAd` | 1 |
| `LinePierceDamageStun` | 3 |
| `MagmortarEruption` | 1 |
| `MagmortarHeatWave` | 1 |
| `MarowakBoneRush` | 1 |
| `MarowakBoneWindmill` | 1 |
| `MarowakBonemerang` | 1 |
| `MeleeArcDamage` | 2 |
| `MissingNoGlitchBasic` | 1 |
| `MissingNoGlitchStorm` | 1 |
| `MissingNoRandomStatus` | 1 |
| `MissingNoTrickRoom` | 1 |
| `MistyTerrain` | 1 |
| `MultiHitDamage` | 12 |
| `NastyPlot` | 1 |
| `PainSplit` | 1 |
| `PollenPuff` | 1 |
| `PowerUpPunchChannel` | 1 |
| `PresentDamage` | 1 |
| `PresentHealPercent` | 2 |
| `QuaquavalExcitingDance` | 1 |
| `QuaquavalHydroKick` | 1 |
| `QuaquavalSpiralShot` | 1 |
| `QuaquavalUpTempo` | 1 |
| `RainDance` | 1 |
| `RescueAllyDash` | 1 |
| `ReversalGuard` | 1 |
| `RillaboomDrumBeating` | 1 |
| `RillaboomDrumRoll` | 1 |
| `RillaboomGrassySurge` | 1 |
| `Roost` | 1 |
| `SawkThrohBasic` | 1 |
| `SawkThrohSkill2` | 1 |
| `SawkThrohSwitch` | 1 |
| `SawkThrohUlt` | 1 |
| `SelfAllStatsBuff` | 1 |
| `SelfAreaDamage` | 2 |
| `SelfAreaDamageBurn` | 1 |
| `SelfAreaDamageBurnConfusion` | 1 |
| `SelfAreaDamageFreezeAll` | 1 |
| `SelfAreaDamageKnockbackChancePoison` | 1 |
| `SelfAreaDamageParalysis` | 1 |
| `SelfAreaDamageRetreatLine` | 1 |
| `SelfAreaDamageThrow` | 1 |
| `SelfAreaMiasmaDetonate` | 1 |
| `SelfAttackDefenseBuff` | 1 |
| `SelfAttackDualDefenseBuff` | 3 |
| `SelfBuffAreaDamage` | 2 |
| `SelfBuffAreaDamageKnockbackChanceBurn` | 2 |
| `SelfCleanseOffenseBuff` | 1 |
| `SelfCritAntiHealBuff` | 1 |
| `SelfDefenseBuff` | 2 |
| `SelfDetectGuard` | 1 |
| `SelfHeal` | 1 |
| `SelfHealResetPorygonType` | 1 |
| `SelfHpCostOffenseSpeedBuff` | 1 |
| `SelfOffenseBuff` | 2 |
| `SelfOffenseCritBuff` | 2 |
| `SelfProtectDefenseBuff` | 1 |
| `SelfSleepHeal` | 1 |
| `SelfSpeedCritBuff` | 1 |
| `ShedTailTauntDash` | 1 |
| `ShellTrap` | 1 |
| `ShiftryTornado` | 1 |
| `SigilyphGravity` | 1 |
| `SingAura` | 2 |
| `Sketch` | 4 |
| `SleepPowderCircle` | 1 |
| `SmokeScreen` | 1 |
| `SnorlaxBellyDrum` | 1 |
| `SnorlaxSnore` | 1 |
| `SnorlaxSwallow` | 1 |
| `StakeoutField` | 1 |
| `StarmieFlipTurn` | 1 |
| `StarmieHeartSwap` | 1 |
| `StarmiePsybeam` | 1 |
| `StealthRockBuff` | 1 |
| `StickyWebField` | 1 |
| `StoredPower` | 1 |
| `SubmissionBindDot` | 2 |
| `SunnyDay` | 1 |
| `SwannaFeatheryCyclone` | 1 |
| `SwannaSkyCircus` | 1 |
| `TargetAttackDebuffByDefense` | 1 |
| `TargetAttackDebuffProtect` | 1 |
| `TargetDamageReduction` | 1 |
| `TargetDefenseDebuff` | 2 |
| `TargetDefenseDebuffStun` | 1 |
| `TargetItemScaledDamage` | 1 |
| `TargetLockOn` | 1 |
| `TargetOffenseDebuff` | 1 |
| `TargetPoison` | 1 |
| `TargetPoisonExecute` | 1 |
| `TargetSkillBlock` | 1 |
| `TargetStatSteal` | 1 |
| `TargetSwapDamage` | 1 |
| `TargetTauntSelfDefenseBuff` | 1 |
| `TargetTerrify` | 1 |
| `TrailblazeGrassyTerrain` | 1 |
| `TravelingWaveDamage` | 4 |
| `TravelingWaveDamageChanceFreeze` | 3 |
| `TravelingWaveDamageInfestation` | 1 |
| `TravelingWaveDamageParalysis` | 1 |
| `TravelingWaveDamageSlowBurn` | 1 |
| `TravelingWaveDamageTerrify` | 1 |
| `TravelingWavePoisonSlow` | 1 |
| `TriAttack` | 1 |
| `TripleArcDamage` | 1 |
| `TripleAreaDamage` | 1 |
| `TripleAreaDamagePoison` | 1 |
| `WeavileAssaultingHunt` | 1 |
| `WeavilePursuitClaw` | 1 |
| `WeavileScratchAndShuffle` | 1 |
| `WeezingBodyOdor` | 1 |
| `WeezingSludgeWhirlpool` | 1 |
| `WhirlpoolField` | 1 |
| `WishChannel` | 1 |
| `WishiwashiCowardice` | 1 |
| `WishiwashiMassiveCatch` | 1 |
| `WishiwashiWaveSplash` | 1 |
| `XatuMindBend` | 1 |
| `XatuPainAmplifier` | 1 |
| `XatuSuperPsy` | 1 |
| `YanmegaBuzzingBoost` | 1 |
| `YanmegaGigaDrain` | 1 |
| `YanmegaTintedLens` | 1 |
| `ZeraoraThunderCage` | 1 |
| `ZeraoraWildCharge` | 1 |
| `ZeraoraZingZap` | 1 |

## No Staged VFX Bindings

Emboar, Blissey, Kleavor, Sawk & Throh, Hitmonchan, Hitmonlee, Hitmontop, Skarmory, Pyukumuku, Ambipom, Audino, Pangoro, Kommo-o, Bouffalant, Grapploct, Snorlax, Weavile, Smeargle, Torterra

No staged binding does not imply a broken skill: some moves have no separate VFX artwork. Smeargle reuses the copied move's assets.
