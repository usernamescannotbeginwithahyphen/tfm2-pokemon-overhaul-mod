# Roster Visual Review - Build 0.2.12

## Toxic Chain Follow-Up

Build 0.2.12 splits Dragalge's combined sheet into small directional chain links
and a separate victim impact. Successful TargetPoisonExecute resolution places
links at evenly spaced world positions along the actual caster/victim segment,
with one of 16 aim tags. Spacing is at most 8,000 units within the authored range;
an explicit 32-link cap bounds abnormal inputs. The ring is submitted at the
victim. Generic cast emission is suppressed to avoid duplicates. Copied casts
use the actual copying caster.
Zero separation emits only the ring. Missing entities emit nothing.

Poison stacks, execution threshold and damage timing are unchanged. The spell
applies immediately, so this is a brief cast connection, not a maintained tether
for the poison's 15-second duration. Animation documents no explicit stretch
control, so this route does not rely on stretching one sprite with range/radius.
Link spacing, perceived height and 16-direction rotation still need live testing.

Verification: 86 tests passed in debug and optimized builds (one intentional
skip), including cardinal/diagonal/reverse links, alternate caster, zero length,
missing target, poison stacks and execution. Validated 291 sheets / 6,108 frames
and 417 package JSON assets. Build 0.2.12 is deployed; all 1,112 installed files
match. DLL SHA-256:
`47060BAFC51BD7D3AF850988A5FE5468DB2275005C2813D47EAAB694935B4146`.
No in-game renderer test has been performed for this follow-up.

## Scope And Evidence

The roster inventory covers 103 Pokemon, 412 actions and 103 passives. This is
a source/asset review plus simulated host-callback regression testing, NOT an
in-game renderer sign-off. The per-move checklist is
[roster-vfx-audit.generated.md](roster-vfx-audit.generated.md).

## Changes

- Moving cones use one attached animation per cast with transparent mouth offset,
  not an animation restarted every tick. Sixteen pre-rotated tags also support
  static cones, beams, rectangles and tethers. They approximate arbitrary aim;
  exact renderer scaling and the mouth pivot require live acceptance.
- Charizard Inferno retains targeted circle damage and now uses a detached ring.
  Its old body-baked ring is removed. Snarl and Disarming Voice remain
  caster-centered by explicit user decision; descriptions now agree.
- Octillery Bouncy Bubble has three consecutive 12-tick flight legs and three
  separate damage circles, 36,000 units apart for an 18,000 radius. The same
  bubble artwork rises/falls during each leg and bursts on the third landing.
  Known-map bounds clamp the centers; edge-clamped circles can overlap.
- MultiHitDamage and Cloyster volleys launch separately and resolve one hit per
  impact. Blastoise alternates cannon art. Dragapult/Jolteon composite missiles
  were cropped to one missile. Leafage launches three independent piercing leaves.
- Traveling waves use 10,000 units/tick and swept contacts. Added fast contact
  transport for Muddy Water, Leaf Storm, Flame Wheel, Weezing and Wave Splash.
  Wishiwashi retains ally healing/cleanse and Schooling range. Arcanine White
  Flames uses two offset fronts with a shared once-per-victim hit set.
- Starmie Flip Turn, Quaquaval Up-Tempo and Ribombee U-Turn wait for actual
  movement contact. Dedenne spends Play Rough charges once at launch, then applies
  damage/silence at contact. Drednaw keeps the initial spin and resolves retreat
  hits along actual movement. Copied Razor Shell retains the full move identity.
- Clawitzer/Dragalge now emit one splash per gameplay center. Dedenne artwork
  contains one node per frame. Turtonator idle traps and triggered bursts are
  separate assets. Ampharos's persistent zone no longer replays the strike.
- Coalossal emits advancing paired segments and final burst separately. Kecleon
  pulses occur at their configured interval instead of three hits immediately.
  Swanna uses separate boundary/slash/landing art and actual movement contacts;
  an interrupted stationary cast cannot hit untraveled waypoints or land remotely.
- Blissey's ten-second Heal Pulse has a standalone state-driven aura. Historical
  duplicate body overlays were removed only from selected tags for Charizard,
  Banette, Rillaboom, Armarouge, Archaludon, Blissey, Thievul and Ceruledge.
- Earlier fixes remain: delayed Psybeam/Wish/Gigavolt, conditional Phantom Force,
  Nasty Plot target attachment, Stored Power release, fixed Shedinja chit counts,
  actual Light Screen geometry and gold-expanded Make It Rain radius.

## Remaining Limits And Acceptance

- The user accepted finite 30-tick field leases. Consumed/expired fields can leave
  a visual tail of up to half a second. No supported cancellation handle is wired.
- Projectile renderer rotation is not established by the public binding contract.
  Test diagonal travel and footprints; submitted coordinates alone do not prove
  the renderer displays the right orientation/scale.
- Zeraora's counter still uses its older forced-movement/damage sequence. Eevee's
  Baton Pass visual is a burst rather than a sustained connection. Sylveon's
  Charm heal marker is a cast pulse. Toxic Chain's new directional submission is
  implemented but still needs live visual acceptance. These are not silently
  marked visually complete.
- Body-only attacks cannot be inherited as detached art by Smeargle. Some heals
  (Comfey, Delibird) have no standalone staged art. Full body-set transformations
  for Sawk/Throh and Ursaluna are not implemented.
- Test original and copied casts, moving casters/targets, diagonals, interruptions,
  deaths, empty ground, map edges, overlapping fields and consumed fields in-game.
  The manually assembled Charizard preview is not an engine capture.

## Reproduction

Previous build (0.2.11) verification: debug and optimized suites each passed 84 tests, with one
data-companion exporter intentionally ignored. Validated 290 VFX sheets / 6,028
frames, all 103 body sheets, and 416 package JSON assets. All 580 staged/runtime
VFX files match. Release 0.2.11 was deployed only to the Steam installation's
`mods/pokemon_moba_stable`; all 1,110 installed files match the package. No
`.data_champion` files ship. DLL SHA-256:
`7729E3B692BF140C830ABDC05E71FC0A76FBF9A5DFA0ADA420C081A69FB78DFC`.
The refreshed guide PDF was checked on the Octillery and Umbreon moveset pages.
No in-game match was run for this build.

1. Run `tools/repair-stable-vfx-assets.py` with Pillow. Original sources are kept
   in `assets/custom_spritework/references/vfx-pre-repair`.
2. Run `tools/generate-stable-vfx-bindings.py`.
3. Run `cargo test --manifest-path mod/pokemon_moba_stable/Cargo.toml --lib --offline`
   to refresh guide/inventory exports and exercise the mock host.
4. Run `tools/audit-stable-vfx.py` with Pillow for all animation rectangles,
   durations and contact sheets; run `tools/generate_pokemon_guide.py`.
5. Run the optimized suite and `tools/build-stable-native.ps1`, then deploy only
   `pokemon_moba_stable` with the game closed. No `.data_champion` companions ship.

Generated replacement art used the built-in ImageGen tool. Accepted source files
and exact prompts are retained in
`assets/custom_spritework/references/imagegen/vfx-repair-prompts.json`.

Official binding reference:
[Visual Bindings](https://github.com/teamsamoyed/TeamfightManager2Mod/blob/main/docs/data-champion-schema/visual-bindings.md).
