# SDK 0.6.4 Capability Audit

2026-10-18. Sources: installed official SDK 0.6.4 (ABI 10), official docs commit
`064c2fa`, and the current native Pokemon implementation. These are source-level
findings, not observed in-game results; no match was run for this audit.

The [whole-mod follow-up](#whole-mod-audit-follow-up) expands this beyond the
original examples. The [roster inventory](stable-champion-migration-audit.generated.md)
accounts for all 103 champions, 412 action slots and 103 passive descriptions.

## Build 0.2.13 Attachment Correction

2026-10-19: investigates the reported Comfey/Clawitzer base-to-lane teleport loop.
The report has not been independently reproduced in a live match. Concrete code
defects and corrections:

- Both passives applied a four-tick ForceMove with speed 3,600,000 whenever
  their host was more than 500 units away. This had no endpoint bound. It is now
  an exact `entity_set_pos` synchronization once per passive refresh, retaining
  the small deadzone, short root, cast capability and existing stat transfers.
  No broad CC clear or banish is introduced.
- Partner-seeking AI ignored its base input and could replace a detached recall
  with movement. It now preserves Return and ignores recent returning hosts and
  owners. The passive uses the same return/cycle eligibility rules.
- The attached-recall blocker could fail open when retreat helpers produced no
  valid replacement. Attached recall now returns Block, translated by the stable
  adapter into an explicit hold at the caster position, not SDK None/pass-through.
- Owner death/respawn resets stale attachment state. Host death/respawn releases
  dependent attachments with their normal detach cooldown. Circular Comfey and
  Clawitzer attachment graphs are rejected to avoid position/stat feedback.

SDK 0.6.4 supplies exact position mutation in simulation callbacks and live
read-only simulation access in AI. No dedicated parent/attach operation or
authoritative in-progress recall query was found. The retained eight-second
recent-Return guard can delay reattachment after a canceled recall; do not call it
an engine recall-state query. This patch does not change detach-on-host-recall
into shared recall, nor replace the existing third-max-HP Clawitzer detach rule.

Regression cases cover exact following across ordinary and large host movements,
no repeated jumps when stationary, no tether ForceMove, host/owner recall,
return-window reattachment rejection, cycles, death/respawn and AI recall holds.
They use the SDK mock host, not the engine movement/recall implementation. Live
acceptance must include each champion alone and both together, host recall,
detached recall, canceled recall, death/respawn, continued skill casting and
Clawitzer's large-hit detach.

Validation: 91 active debug tests passed, including five attachment regression
tests; the native-only companion exporter remains intentionally ignored.
Release build 0.2.13 deployed to the installed `mods/pokemon_moba_stable` package;
all 1,112 package files matched after deployment. DLL SHA-256:
`38CCB5BBAF856F874F63E7546D22015F303DAB424AA1C47DE443CF450200C25F`.

## Build 0.2.10 Roster Review

See [the full review](vfx-roster-review.md) and its generated per-action checklist.
This corrects premature/conditional VFX, delayed Psybeam combat, target marks,
chit-state selection, duplicate reference overlays and two projectile routes.
It also identifies composite-sheet multiplicity, combined phase artwork, and
directional/mouth anchoring that remain unresolved. The 0.2.9 cone-center change
is not an exact fix for a moving breath. No live visual sign-off is claimed.

## Build 0.2.9 Visual Placement

Source-level fixes for the report that many effects appear at the wrong position
or move after casting. This does not claim that every animation is visually
correct in the live renderer.

- Generic cast events previously passed live entity targets through unchanged.
  Bursts now snapshot explicit world positions at the event, including directional
  input conversion; an invalid target does not fall back to an unrelated location.
  Existing `Animation` bindings already had `is_follow: false`; entity snapshots
  make placement explicit, but are not proof of the reported renderer drift's cause.
- Cones/lines previously reused the victim/aim position as the whole effect's
  center. The shared dispatcher now uses the segment/cone center and its gameplay
  dimensions. Self-buff AoEs use the caster center/radius; triple and bouncing
  areas reuse the exact gameplay center helpers. Dash-end cones use actual arrival
  position and the original movement direction.
- Dash impact flashes no longer fire at launch. They fire on swept contact;
  Pikachu/Greninja/Dragapult streaks instead attach to the moving unit and are
  removed on interruption, replacement or completion. Damage logic is unchanged.
- Smoke Screen, Trick Room, Grassy Surge and flame trails render from persistent
  ground geometry, not a generic cast snapshot. Flame trails retain the source
  champion's artwork for copied moves. Quaquaval trail segments use their midpoint,
  not their endpoint. Magmortar Heat Wave follows the caster, matching its gameplay.
- Attached visual refreshes start on the first state update instead of waiting
  for a globally divisible-by-30 tick, and refresh a single instance before expiry.
  Fixed fields retain their existing deduplicated thirty-tick leases.

Regression coverage includes all 412 roster slots emitting position-based generic
events, exact cone/self/triple centers, frozen impact targets, non-stacking aura
refresh, Smoke Screen remaining at its original position after both units move,
and delayed dash flashes/interruption cleanup. Debug and release suites each
passed 69 tests, with the companion-data exporter intentionally ignored. Existing
movement/damage tests remain in place. The tests inspect SDK calls; they do not
render the game.

Deployed 0.2.9 to the real Steam `mods/pokemon_moba_stable` directory. All 1088
installed files match the source package by SHA-256; no companion data champions
are installed. DLL SHA-256:
`EC55EBC437875687869A895F6D3A3CCEFB0E86F52C506B1A615A4AD4E9169BD6`.

Remaining acceptance work: run a new match and check directional artwork, frame
pivots, world-to-sprite sizing and renderer depth. The public
[visual binding schema](https://github.com/teamsamoyed/TeamfightManager2Mod/blob/main/docs/data-champion-schema/visual-bindings.md)
does not expose a rotation/pivot setter for these Animation bindings; LoopAnimation
ignores `is_follow` at registration. Supplying accurate range/radius is not proof
that an arbitrary raster animation rotates/scales to match its gameplay shape.
Consumed/expired field visuals can still linger for at most their thirty-tick
lease because the public event surface has no cancellation handle. Bespoke
multi-stage moves/channels still require individual visual acceptance; this patch
does not replace them with guessed gameplay or companion damage definitions.

## Build 0.2.8 Movement Balance

All 103 authored base speeds now lie within 624-936 (median 780, +/-20%),
compressed from 390-1480 without changing ordering or ties. This is not an engine
speed cap: conditional buffs, slows, items and ability travel speeds remain active.
Scizor's speed-based combat coefficients and travel threshold, Dedenne's charge
distance and Electrode's movement-pulse distance are compensated. See
[movement balance](balance-registry.md#movement-compression-build-028) for the
complete before/after roster and exact adjustments. Guide/tooltips were regenerated.

Debug and release suites each passed 64 tests, with one intentionally ignored
companion-data exporter. New coverage verifies the full roster band/order/median,
level-12 values, Scizor's speed-derived contributions and unbuffed crit matchup
set, plus the shortened Light Metal distance boundary. Live balance remains untested.

Build 0.2.8 was deployed to the real Steam `mods/pokemon_moba_stable` directory.
All 1088 installed files matched the source package by SHA-256, including the
regenerated HTML and PDF guides. No companion `.data_champion` files are shipped.
DLL SHA-256: `E67FE927047C20E5509D4EA35325A28D1FE81A5593C129134BBDE9FB492621B6`.
The PDF's Scizor stats/moves and Dedenne stats pages were visually reviewed.

## Build 0.2.7 Combat Semantics Fixes

2026-10-18. This implements the follow-up to the historical audit below. These
are source changes and mock-host regression tests, not live game acceptance.

- Periodic stat bonuses/debuffs now use explicitly named one-stack refreshes.
  Dedenne grid/static, Mantine water speed, Shiftry camouflage, stance bonuses,
  schooling, auras/fields and Receiver branches no longer append on each update.
  Tracked refreshes retain one ledger entry per source/name. Intentional event
  stacks still use add_buff. The host mock now models real multiple instances.
- New simulation-scoped motion delivery covers six DashDamage variants, three
  LineFirstDash variants, both LineDashPierce variants and DashEndConeDamage.
  Actual observed movement drives swept contact; first-hit stops, piercing hits
  each target once, a miss still moves, and blocked motion cannot hit the intended
  but untraveled route. One-tick ForceMove leases must be verified against host
  update ordering. Shared movement endpoints are bounded to avoid overshooting.
- All seven TravelingWave variants now advance a rectangular damage front at
  5000 world units/tick. A visual-only native projectile carries the artwork at
  the same speed. Crossings use relative target motion and one hit per target.
  These are classified by gameplay effect, not by asset filename. Genuine beams
  remain instant. Shedinja's arrival cone and the terrifying wave also had missing
  direct-damage classifications; these are now included in the resolver.
- Dedenne chains use stored node damage (14 + 28% AD before Pokemon modifiers),
  not an unrelated fraction of attack damage. Active-grid recast and the longer
  custom cooldown are now reflected in AI readiness. Node consumption/overload
  remain in the existing resolver.
- Light Metal grants a real native shield. The old custom absorber no longer
  subtracts the same damage again. Combat resets charge distance without removing
  unrelated native shields.
- Shed Tail spawns a separate timed, stationary, attack/skill-blocked unit and
  taunts toward that entity. It uses the caster's HP/defenses and attached tail
  art; there is no caster-taunt fallback if spawning fails. Live unit targeting,
  visibility and lifetime acceptance are still required.
- Copied moves keep their elemental type and source artwork. Smeargle uses its
  own species types for STAB, including delayed fields/projectiles. Receiver
  dispatch now includes Shiftry's bush updater.
- Persistent visuals begin on first observation, with per-field deduplicated
  30-tick leases refreshed after state cleanup. Consumed/expired fields stop
  refreshing, but the final lease may remain visible for up to 30 ticks. Life
  Dew ignores full-health allies and removes a consumed pickup before healing.
  Gameplay zones remain fixed world-positioned records, not terrain mesh edits.
- Sunny Day/Rain Dance forward their rectangle center, length and half-width.
  Public Animation/LoopAnimation bindings have no rotation or cancellation
  parameter. Exact rotated artwork/footprint matching remains unsupported by
  this path and must not be claimed as fixed by size metadata alone.
- Self-Destruct's successful outcome means an enemy champion was caught, not
  necessarily killed. Only an actual self-KO records the respawn outcome. The
  speed bonus/penalty waits until the caster is alive and is consumed once, with
  both spawn-hook and surviving-entity update paths. The SDK exposes read-only
  respawn_time, not a respawn-countdown setter; no timer change is implemented.

### Coverage And Live Acceptance

The shared dash conversion is not a rewrite of bespoke multi-stage movement:
Dedenne Play Rough, Arcanine handlers, ChainDashDamage, and other specialized
arrival/retreat sequences retain their existing implementations. Flame trails
still originate from successful dash impacts rather than being painted every
movement tick. Multi-hit missiles still discharge their authored hit sequence
on one impact. These are remaining code/design checks, not proven parity.

Test first-hit misses/blockers, piercing contacts, canceled dashes, returning
dashes, moving-target waves and one-tick ForceMove host ordering. Check field
rotation/scaling/overlap and the short visual tail after consumption; native
shield accounting against tower/minion damage; Shed Tail entity behavior; and
Electrode's actual respawn path. The full 103-champion roster inventory is not
103 individually simulated champions. Universal movement-speed balancing,
Noivern, tower death artwork, skill audio and full body swapping are unchanged.

Validation: `cargo test --release --offline --locked` passed 62 tests, with one
intentionally ignored companion-data exporter. New regressions exercise real
buff-instance stacking, refresh-ledger ownership, dash miss/contact/piercing and
blocked paths, arrival-cone damage, traveling-wave timing, grid damage/readiness,
native shield forwarding, decoy taunt ownership, copied STAB, immediate field
leases/rectangle metadata, Life Dew consumption and one-shot respawn outcomes.
These tests use a mock stable host; live engine movement/rendering is not proven.

Deployment: build 0.2.7 installed into the real Steam game directory's
`mods/pokemon_moba_stable` only. All 1088 installed files matched package SHA-256
hashes; zero `.data_champion` files. Installed DLL SHA-256:
`EC5C9373E7B3AB791348CDE094D82193E299233119012A7C1A4CFFC17B96CB9A`.
The unrelated `saw` mod was left untouched. No live match has been run.

## Post-0.2.6 Combat Semantics Audit

Historical pre-fix audit, 2026-10-18, against installed build 0.2.6. See the 0.2.7
implementation above for current status. This pass checked the
103-champion / 412-action inventory (264 distinct effect variants), shared
resolver/adapter, all 71 action slots with staged projectile bindings, movement
families, field lifecycle/visual consumers, passive updates/copies and engine
integration. It is not a claim that every numerical rule or all 103 champions
have been individually simulated. The generated roster table remains the full
inventory; its family tags are not completed acceptance tests.

### Confirmed Fix Candidates

1. **P1: refresh buffs are repeatedly appended rather than refreshed.**
   `GameCtx::add_buff` calls SDK `add_buff`, not the named one-stack refresh API.
   Dedenne's grid adds a 35-tick +25% speed buff every simulation tick while
   occupied (`pokemon_content.rs:update_dedenne_grids`). Mantine's Swift Swim
   has the same per-update pattern. Shiftry refreshes a 20-tick +15% buff every
   10 ticks; Houndoom adds a 45-tick -15% attack buff every 30 ticks. Receiver's
   Inteleon/Emboar branches append 45-tick stat buffs every update. These calls
   allow overlapping stacks, not one intended persistent bonus. Use scoped,
   named one-stack refreshes and preserve intentional stacking separately.
   Existing mock `add_buff` stores by `(entity, name)` in a HashMap, overwriting
   duplicates; it cannot catch the real append/overlap problem. Improve that
   test model before using its stat assertions as proof.

2. **P1: dashes are not contact-driven.** `targets_for_effect` chooses the first
   enemy or all enemies along a segment at cast time. Damage/status resolution
   and `ForceMove` scheduling occur in the same callback, not when the mover
   reaches each victim. First-hit variants (Emboar, Blaziken, Ceruledge,
   Torterra) invoke movement inside the selected-target loop, so an empty line
   produces no dash. Their bounded-speed helper only caps total travel toward
   the old target position; it does not detect moving blockers. Piercing dashes
   (Heliolisk, Dragapult) move independently of target count but still use the
   initial target snapshot. Ordinary targeted dashes use unbounded speed/ticks
   and can travel beyond the aimed position. Fix with explicit movement state,
   swept contact checks, one-hit-per-victim bookkeeping and defined miss/end
   behavior; preserve real dashes, not teleport substitutions. No public
   all-purpose dash-on-collision callback was found in the installed SDK.

3. **P1/P2: projectile timing is coupled to staged artwork.**
   `projectiles::launch` first requires a catalog projectile binding. Covered
   first-hit lines set `penetrate=false`; covered piercing lines set it true;
   their impact callback restricts resolution to the collided target. That part
   is wired. Non-covered line/wave actions still execute instantaneous segment
   queries, including Pikachu Thunder Wave, Gyarados Surf, Venusaur's poison
   wave and several freeze/infestation waves. Beam/hitscan moves may be correct
   as instant; traveling waves should be classified explicitly, not by whether
   a filename contains `projectile`. Multi-hit projectile actions currently
   discharge all hits on one contact, not independent missiles. Arbok ground
   lobs are timer-resolved, and Bonemerang has separate outbound/return legs.
   Projectile hit filters currently default to Enemy instead of consistently
   preserving action eligibility; first-hit collision with an excluded entity
   can consume a shot before resolver filtering. This is a blocker-policy risk,
   not proof that every such collision is wrong; test and specify blockers.

4. **P2: Dedenne's grid has two additional logic gaps.** It calculates and
   stores `node_damage = 14 + 28% AD`, but never reads that field. Basics instead
   chain for 35% of their hit damage; Play Rough chains for 45% of its damage,
   contrary to the configured/advertised node damage. The engine ult cooldown
   is 120 ticks for recast, while the real cooldown is 5400 ticks. The effect
   checks that custom cooldown, but `ai_can_activate` has no Dedenne grid gate:
   AI can repeatedly select a no-op ult outside its real availability window.
   The seven fixed nodes, eight-second expiry, side-node consumption/healing,
   chain callbacks, overload, multi-node silence/disarm and VFX event callers
   do exist. These gaps must be fixed before calling the entire ult correct.

5. **P2: field visuals do not consistently match gameplay geometry.** Sunny
   Day and Rain Dance query oriented rectangles (`start`, `target`, `length`,
   `width`), but their native VFX refresh uses a single endpoint and `width/2`.
   Length and orientation are not supplied. The general cast geometry helper
   also falls back to radius 10000 for many specialized effects. Fixed ground
   fields (grass, salt, webs, Life Dew, Dedenne nodes) do use stored world
   coordinates, not the caster's moving position. Their 30-tick loop segments
   allow up to 29 ticks of initial delay and about 30 ticks of cosmetic tail;
   animation restarts, scaling and overlap still require visual testing.

6. **P2: Scizor's shield is still custom-only.** Light Metal writes a private
   `shield_amount`; only the Pokemon damage resolver consumes it. It never
   creates a native shield, so engine-origin hits do not share that protection
   or visible shield accounting. Native shielding is available. Conversion must
   preserve Light Metal's movement/combat reset rules without clearing other
   sources' shields or leaving both native and custom absorption active.

7. **P2: copied identity still mixes move and caster semantics.** Sketch invokes
   `PokemonEffect` with `copied.source_champion`. The default `active_action_typing`
   returns that champion's types, and the damage layer uses those supplied types
   for STAB. Thus copied moves can receive the original species' STAB rather
   than Smeargle's actual typing. Preserve original move/art identity but separate
   caster typing and passive ownership. Receiver also still lacks a call to
   Shiftry's bush-speed updater: it has the basic-attack bonus check but not the
   full copied camouflage behavior. Do not assume all copied passives are complete.

8. **P2: Heliolisk's Shed Tail is still a decoy approximation.** It retreats and
   taunts enemies toward the caster ID, not a spawned tail. The SDK now exposes
   `spawn_unit`, but a proper tail needs its own registered unit, lifetime and
   targeting rules. This is an upgrade candidate, not something already converted.

### Family Coverage And Remaining Acceptance

| Family | Source-level result | Remaining work |
| --- | --- | --- |
| Direct damage / type / crit / stats | Pokemon resolver retained; native damage pipeline active; old injector disabled | Live stats/procs, special crits and engine-origin damage interactions |
| First-hit / piercing staged projectiles | Distinct collision flags and impact dispatch present | Dynamic blockers, eligible hit filters, repeated contacts, target death, return-leg timing |
| Other lines / traveling waves / multi-hit bursts | Often cast-time snapshots or one-contact bursts | Explicit delivery classification; don't convert genuine beams to missiles |
| Targeted / first-hit / piercing / returning dashes | Distinct target selection, but forced movement is not authoritative contact | Miss behavior, swept collision, hit timing, interruption, actual endpoints and tower safety |
| Teleports / swaps | Banette, Skarmory and Audino use native positions | Landing constraints and interrupted/dead targets |
| Fields / traps / terrain | State, positions, expiry, interaction queries and native visual consumers present | Refresh stacking, exact geometry, death lifetime, overlap/consumption and VFX timing |
| Dedenne grid | Nodes, consumption, overload and visuals connected | Buff refresh, configured node damage and custom cooldown AI gate |
| Healing / cleanse / shields | Engine heal/CC clear and generic shield calls available | Native engine acceptance; remaining custom shield conversion and source-safe removal |
| Buffs / debuffs / stance / channels | Preserved gameplay stores and native adapter | Audit every periodic refresh versus intentional stack; remaining custom recast gates |
| AI / vision / roles | Live gates, shared detection, role priors and post-draft assignment wired | Live host input acceptance; full macro strategy is not replaced |
| Copies / delayed casts | Sketch, Receiver and Instruct paths retained | Caster versus source identity, copied persistent-state cleanup and full passive coverage |
| Items / gold | Inventory-derived riders and credited-CS payout implemented | Item-proc transfer unsupported; historical item-setting replay snapshot limits |
| Runtime / replays | Simulation-scoped stores, resets and callback scopes in place | Interleaved live/background/replay acceptance, lifecycle cleanup |
| Deferred features | Noivern, tower death art, new audio and full sprite swapping unchanged | Do not treat these as completed features |

Ground terrain here means gameplay zones (damage, slows, buffs, triggers), not
runtime edits to the map's baked walls/pathfinding. That representation is
appropriate for grass, salt, webs and healing pickups; it does not create a
physical obstacle or summon automatically.

Priority: fix refresh/stack semantics and the mock model first, then movement
contact/timing and transport classification, Dedenne's damage/readiness, field
geometry, native shield ownership, copied identity, and the decoy unit. Keep
the existing combat resolver as the damage/status owner. Do not replace skills
with synthetic companion damage or discard mechanics to simplify migration.

Validation rerun: `cargo test --release --offline --locked` passed 48 tests,
with 1 intentionally ignored exporter. Those tests do not cover these newly
identified behavior gaps (and the buff mock limitation above explains one
blind spot). No gameplay code, package version or installed DLL changed during
this audit. No live match was run.

## Build 0.2.6 Shared Detection

Kricketune and Ampharos now feed the same simulation-scoped team knowledge into
all Pokemon player-AI hooks and action target constraints. This is AI awareness,
not a new fog-of-war renderer or a replacement for the engine's full strategy AI.

- Connected webs refresh enemy observations; disconnected webs do not. Ampharos
  refreshes enemy champion observations every passive update, independently of
  the existing five-second damage pulse. Teammates receive the same knowledge.
- AI snapshots use the observed position for hidden enemies, not their live
  position after leaving detection. Observations no older than eight ticks can
  authorize our target gate; the two-second remainder is last-known memory for
  movement/threat decisions. Expiry, team separation and simulation separation
  are tested. Other existing spotting effects use this same consumer.
- Newly known hidden enemies affect local champion-number comparisons. Below
  40% health, or locally outnumbered, units consider retreat instead of chasing.
  Unsafe movement routes through a detected disadvantage receive alternative
  waypoints. Candidate paths avoid enemy tower entry and must pass host input
  validation; blocked escapes fall back to holding rather than advancing.
- Healthy units can select enemy-targeted attack/skill/ultimate inputs, including
  position/direction casts. Selection runs through existing champion-specific
  charge/channel/Sketch gates. Existing safe recalls and active skill choices
  are preserved; attached partners do not independently run away from their host.
- Current detection counters our Weavile stealth bookkeeping/native invisibility
  lease. Real engine untargetability is never overridden. The host can still
  reject an unseen-target attack; then movement/threat awareness remains useful.
  Only an in-game test can establish which such inputs the host accepts.
- Unit tests cover favorable engagement, low-health and outnumbered retreats,
  unsafe routes, tower safety, actual connected-web detection, continuous aura
  detection, team/match isolation, last-known positions, expiry and target gates.
  No live match has been run for this change. Test allied responses to detections
  with both abilities, including broken web connections and enemies leaving range.
- Validation: `cargo test --release --offline --locked`: 48 passed, 0 failed,
  1 intentionally ignored companion-data exporter. A mocked host test verifies
  accepted combat input, rejected combat falling back to valid movement, and
  complete rejection producing no override. This does not establish live host
  acceptance of hidden-target attacks.

## Build 0.2.5 Map-Grid Camouflage

Correction to the earlier audit: no direct `is_in_bush` getter does NOT mean
bush geometry is unavailable. `StableMapCustomizer` exposes `bushes`, `regions`
and `region_centers` through `StableJsonDoc`; the documented grid is 30x30 with
32,000-unit cells. Build 0.2.5 registers a read-only map customizer. It never
changes terrain or baked visibility/pathfinding.

- Shiftry now checks its world-position cell, not nearby enemy visibility.
  Actual bushes can activate camouflage without a nearby observer; unseen open
  ground no longer activates it. Its existing +15% speed, 90-tick qualification,
  120-tick linger and short buff-refresh cadence remain. No new invisibility
  ability or body-swap behavior was added. The initial tick no longer grants a
  spurious bonus from the default zero linger deadline.
- Region-center coordinates validate nested-grid axis order. Nexus cells
  validate the empty-bush value (0 or -1; unsigned MAX is normalized to -1).
  Nonempty region labels, including zero when empty is -1, count as bushes.
  Malformed or ambiguous data is rejected rather than guessed.
- The map callback lacks a simulation identity. Initial typed tower/nexus
  positions associate captured geometry with a match. Matching is independent
  of entity IDs/order; multiple conflicting grids for the same layout are
  rejected. Each simulation then keeps its own immutable snapshot. Missing or
  rejected geometry disables camouflage activation, with no visibility-guess
  fallback. The first live match MUST confirm that actual host geometry passes
  these validations; unit fixtures are not proof of the host's map encoding.
- Diagnostics: `%APPDATA%/TeamSamoyed/TeamfightManager2/data/pokemon_moba_stable-map.log`.
  `capture ... result=Ok(...)` and `bind ... result=Ok(...)` confirm extraction and
  association; errors identify missing fields, ambiguous geometry or layout
  mismatch. Unrecognized geometry is included once per distinct captured layout
  for diagnosis. No SDK callback pointers are retained.
- Ampharos still suppresses Shiftry's camouflage activation within its live
  42,000-unit true-sight aura; qualified speed linger follows the existing rules.
  Weavile's native invisibility also yields to this aura. Luminous Pulse targets
  by position, without a visibility filter, so its damage/slow still reaches
  enemies hidden by bushes. This is NOT a new engine-wide team-vision override.
- Live acceptance: start a new match, confirm successful capture/bind diagnostics,
  then check Shiftry in a bush with no enemies nearby, visible inside a bush,
  hidden outside bushes, short/qualified exits, and near enemy Ampharos. Existing
  matches/replays without a captured matching map cannot be assumed supported.

Build 0.2.5 validation: 41 tests pass in both debug and release; the companion
exporter remains intentionally ignored. Tests cover grid orientation/encoding,
boundaries, malformed/ambiguous maps, frozen-map isolation, camouflage activation
and linger, Ampharos suppression and pulse hits without visibility. A live host
map capture/bind has not yet been observed.

## Build 0.2.4 Audit Fixes

The audit below records the pre-fix findings. This section supersedes its open
status for the following changes; it does not claim in-game acceptance.

- **A1/A3, AI:** all native actions now forward activation/target gates and the
  movement-while-casting gate. Read-only AI snapshots include live units, HP,
  visibility, targetability, CC, cooldowns and player ownership. Policies cover
  charged actions, attached Comfey healing/Encore, healing eligibility, stance
  switching and Sketch's learned target/range/cooldown. Sketch planning exposes
  a maximum copied-range envelope; the actual copied range is checked per target.
  Copied targeting ranks eligible units instead of taking the nearest enemy.
  Smeargle's forced-learning fallback now runs in an authoritative passive
  callback, not a read-only AI hook. Will-O-Wisp and Sticky Web readiness queries
  predict charge availability without refilling authoritative state. Other
  authored mechanics remain intact.
- **A2, towers:** use live tower positions, check dash paths and blink landings,
  permit outward escapes, and reject unsafe replacements with an explicit
  hold-position input. SDK `think() -> None` preserves the base input, so it is
  not a cancellation. This is a conservative 70,000-unit safety radius, not a
  claim to read tower aggro or intelligently tank a dive. Multi-stage movement
  and changing destinations still require live testing.
- **C2, cleanse:** existing custom harmful-status cleanup now also invokes
  native `entity_clear_cc` and removes our named native anti-heal buff. This is
  not a generic dispel of every engine stat buff.
- **C1, anti-heal:** engine healing is the sole reduction owner. Our ledger
  selects the strongest unexpired source and maintains one named native buff;
  expiration restores a weaker remaining source without stacking. Heal amounts
  are sent before HP capping so the engine caps after reduction. Ripen and
  Pokemon-specific inversion remain. Mock-host tests prove our dispatch,
  replacement/expiration and observed-heal accounting, not the closed-source
  engine's arithmetic. A real anti-heal/heal test remains required.
- **M1, teleports:** Banette Phantom Force and Skarmory Fly now use position
  updates. Audino Substitute swaps positions, with a rollback attempt if the
  second update fails. Existing heals/hits and Protect redirection remain.
  Pikachu Volt Tackle and genuine dashes remain dashes.
- **M2, Octillery:** Suction Cups refreshes native `ignore_wall` and -15% movement
  speed. The old passive had no numeric penalty; 15% is an explicit balance
  choice, not a recovered historical value. Updated description and guide.
  Wall collision permission does not prove the built-in AI selects wall routes.
- **S1, stealth:** Weavile Hunt uses native invisibility in short renewable
  leases, ends it on cast/attack and coordinates with Ampharos true sight.
  Custom targeting protections remain. Fog visibility alone is not true sight.
  Shiftry initially retained its inferred-bush speed passive; build 0.2.5 replaces
  that inference using the supported map document (see above). Attached champions are not banished, since
  that would prevent their casts. No unsupported body swapping was introduced.
- **G1, Receiver:** copied Gholdengo receives the same duplicate-guarded +10
  credited-CS payout, respecting owner validation and passive suppression.
  A thousand duplicate callbacks produce one payout in regression tests.
- **R1, state:** legacy content/status stores use public origin kind, match ID,
  replay ID, set index and seed. Callback scopes restore correctly when nested;
  match start clears only its partition, and finished partitions are reclaimed
  lazily. No engine-owned arbitrary state container was found. Identical public
  identities/seeds remain indistinguishable; a missing origin falls back to
  seed. Live match/replay lifecycle and cleanup callbacks need validation.
- **Deferred by request:** Noivern, tower death art, SFX and debug rendering are
  unchanged. Projectile and persistent-field visuals await live testing.

### Acceptance Order

Local validation: `cargo test --offline` and `cargo test --release --offline`
each passed 35 tests, with only the intentionally disabled companion exporter
ignored. The gate smoke test invokes all 412 roster actions; this is adapter
coverage, not a claim that every skill has been exercised in a real match.

1. Start a fresh match with build 0.2.4. Check log for panics and simulation speed.
2. Check Smeargle copied ally heals/offensive casts, charge limits, attachment
   healing and stance switching. Verify rejected tower dives do not freeze AI.
3. Test cleanse under both native CC and custom statuses; compare one heal with
   no anti-heal, 25%, 50%, overlapping sources, source expiry and near-full HP.
4. Check all three teleports, normal dashes, Octillery walls/speed, Weavile
   visibility and reveal/break behavior, Receiver's +10 per credited CS.
5. Advance multiple matches and open replays; check no status/gold cross-talk.
6. Then check projectile travel/impact and persistent AoE placement/lifetimes.

## Statistics

`StableSim::deal_damage` has used the full engine attack pipeline since 0.5.4:
mitigation, crit, dealt/taken statistics, kill/assist credit, item/buff procs,
lifesteal/reflect and floating damage numbers. The adapter already calls it.
`heal` remains the supported healing call; 0.6.4 adds self_heal/CS/exp replay fields.
There is no reason to revive the old opaque-pointer statistics injector.
Build 0.2.1 comments out all five sync call sites and excludes both unsafe legacy
mutation functions with `#[cfg(any())]`. The historical server module remains
undeclared, with a commented module declaration. The internal ledger remains for
Pokemon mechanics/diagnostics, not as a second writer of engine statistics.

The SDK accepts raw AD/AP. `deal_damage_raw` bypasses statistics and credit and
is not a stats fix. `deal_damage_typed(Fixed)` supplies true damage with
stats/procs, but is not a drop-in way to preserve AD/AP semantics or suppress
all engine side effects.

Build 0.2.3 gives plain basic attacks without special crit rules to the engine.
Custom critical rules stay in the Pokemon resolver: examples include Inteleon's
250% multiplier, Sirfetch'd/crit-overflow scaling, forced crits, Shell Armor and
critical-damage reduction. Endure, Sturdy, Light Metal and redirected damage also
retain pre-resolved crits so their damage clamps see the final amount.
Conservative non-plain attack paths also retain their
old rolls. The SDK has no public already-critted flag: around those synchronous
damage calls the adapter applies a uniquely named negative crit-chance buff,
then removes it, including on unwind. Attack kind, mitigation and statistics stay
on `deal_damage`; raw or true damage is not substituted. Nested proc damage does
not inherit the outer hit's engine-crit permission. Permission also matches
the original attacker/target and only applies to basic-attack damage.

This workaround needs in-game validation. Custom-resolved crits do not become
engine `is_crit=true` events, so crit-only item callbacks are not equivalent.
Same-caster synchronous procs also temporarily observe the suppressed crit stat.
Custom shields and damage reductions likewise need checks
against the new pipeline. The internal ledger now uses observed HP/shield deltas
and no longer adds extra kill gold. Engine statistics remain authoritative.
Custom support-assist rules may differ from the engine's rules.

## Gholdengo

`on_unit_kill(sim, player, owner, victim)` now reports precisely who earned a
minion/camp kill; `victim.kind()` distinguishes camps and minions. Epic/team
objectives do not fire this callback. This permits event-driven Good as Gold.

The callback does NOT contain the actual gold reward. `player.gold()` is the
spendable balance, and statistics JSON documents cumulative gold, not a per-kill
reward breakdown. Taking half of every positive balance change would also count
passive income, champion kills and our own bonus. Do not implement that feedback loop.

Build 0.2.3 pays exactly +10 gold on each credited minion/camp callback, as requested.
The player's champion must match the callback owner. Per-passive (seed, tick, victim)
deduplication is recorded before adding gold to reject duplicate/reentrant delivery.
Two different victims in one tick both pay; a camp that respawns later can pay again.
Wallet gains, passive income and champion-kill callbacks do not trigger this bonus.
Old CS polling is compiled out, not used alongside the new callback. The player-facing
description is deliberately unchanged; +10 is an approximation, not exact 50% scaling.

Base bundle inspected on 2026-10-18: `melee_minion.gold` and `range_minion.gold`
are both 20, `growth_gold` is 4, caps are 60/50 respectively. Wave growth starts
at tick 1800 and uses a 1800-tick interval in ordinary 5v5. Bottom bonus is 20%;
support gold reduction is 15%; 2v2/3v3 gold ratios are 200/150. Settings alone
do not prove the exact engine reward formula/rounding or callback ordering.
Thus +10 per credited kill (+12 for bottom) is a fixed initial-value bonus,
not an exact 50% of all minion/camp rewards. Adding 20 would double the original
early-game bonus. The historical runaway-gold cause is not conclusively diagnosed;
the old polling/reward route is no longer executable.

## Items: Symbiosis, Thief, Poltergeist

Confirmed reads:

- `StablePlayer::item_keys()` and `item_count()` expose actual owned inventory.
- Server/client `setting_get_json(SettingTargetV1::ItemSetting, path)` exposes
  the item-settings document.
- Item build hooks expose candidate keys/category/tier and can select AI builds.

Build 0.2.2 implements equipment stat riders in `src/equipment.rs`. The installed
base bundle confirms settings are an object of item definitions plus `mod_items`
array. Definitions are indexed by their `key`, not the outer setting-slot name
(`iron_blade` contains key `ironsword`). A serde_json parser rejects ambiguous or
malformed catalogs. Server startup/management hooks read current ItemSetting;
the match-start hook freezes a catalog per seed, so later management changes do
not alter running matches. Live inventory is resolved inside the current sim,
not through the old global entity/player mapping or a watched-client callback.

- Thief transfers its existing percentage of one supported positive equipment
  contribution with equal, opposite timed buffs. Base damage, duration, tracked
  buff interactions and existing per-cast stacking rules remain intact.
- Symbiosis shares 50% of supported positive equipment stats. A named buff is
  refreshed instead of stacking overlapping 45-tick copies every 30 ticks;
  changed equipment replaces the old stat payload. Its throttle is sim-scoped.
- Poltergeist uses equipment AD/AP/HP/armor/MR/crit/movement contributions,
  preserving its random category, weighting, damage percentage and Pokemon
  resolver. Percentage contributions use champion base values, not temporary
  stat surplus. Proc-only or attack-speed-only items have no eligible damage stat.

Supported transferable stats are signed buff fields: AD/AP/HP/armor/MR and their
multipliers, crit, attack/move speed, skill/ult cooldown, regen, vamp and radius.
Unsigned reductions/penetration/range, boolean flags, proc cooldowns, dynamic
stacks and triggered item abilities are not transferred. Unknown inventory keys
or unavailable settings disable the equipment rider; no fallback invents stats.
These are still stat approximations, not literal inventory theft. Repeated Thief
casts retain old stacking behavior, rather than enforcing an item-ownership cap.

Remaining integration checks: merged data/native mod item inclusion in the live
settings response; server hook ordering before match start; background matches;
and cold-reloaded historical replays after item balance changes. Per-seed
snapshots currently live only for the server session: historical settings are
not persisted by this mod. Do not claim historical replay equivalence yet.
Optional existing stat-probe logging reports catalog readiness and item count.

Returned item keys are owned read snapshots: deleting/commenting a key in our
copy cannot unequip the engine's item. Editing global ItemSetting would affect
all holders, not temporarily transfer one player's inventory or proc cooldowns.
We can emulate selected static stat contributions with named buffs/debuffs,
but percent-stacking and arbitrary item procs require separate treatment. Literal
inventory theft remains unsupported by the inspected public interface.

## Native VFX And SFX

The old blocker is resolved in the public contract:

1. Register named `view_effects`, `view_projectiles`, and/or `view_buffs` in a
   standalone file such as `view/pokemon.view_effects`.
2. Call `play_view_effect` or `play_sfx` from the actual simulation effect/passive.
3. Pass the real entity/position and lifetime. The host records the event for
   the watched match/replay. Background simulations do not draw it on screen.
4. Moving projectile art belongs to `spawn_projectile`'s exact visual name and
   resolves damage on its hit callback. A stationary effect is not a projectile.

No data champion, native passive replacement, synthetic damage, one-tick visual
buff, or manual draw_sprite synchronization is required for detached effects.
0.6.4 also fixes frame recording in passive callbacks, explicitly covering
previously invisible projectiles and heal/gold numbers.

Build 0.2.1 registers all 279 staged assets in `view/pokemon.view_effects`, generated
alongside the Rust catalog by `tools/generate-stable-vfx-bindings.py`. Native casts
select by explicit source champion and action slot, including copied Smeargle moves.
No entity display-name inference, companion gameplay, or manual client renderer.

Persistent fields (webs, grass/salt, ice, mist, Brine, Life Dew, Dedenne nodes, etc.)
read their authoritative position/state. Persistent auras use named zero-stat visual
buffs refreshed to one stack from active state; they do not replace the skill's
gameplay. Ground fields use 30-tick loop-event segments because there is no public
view-event cancellation handle. Consuming/destroying a field stops refresh, with
up to a half-second visual tail. These refreshes can restart the animation.
Conditional passives emit only at the real proc, or while their actual state exists.
Shedinja chits appear only while knocked down; Quaquaval trails use ground segments.

Build 0.2.3 routes staged projectiles through named native impact callbacks. Targeted
shots home; first-hit shots stop on collision; piercing shots resolve only the
collided target, not the whole line again. Damage, statuses and on-hit mechanics
run through the preserved Pokemon resolver at impact. Cast notifications and copied
move cooldowns stay at launch. Impact callbacks do not spawn the projectile again.
Arbok's ground lob queues its area resolution at distance/speed travel time, since
a ground destination may contain no entity. Its visual collision is intentionally
no-op. Dreepy launches on its passive trigger; Magmortar's existing shot scheduler
launches individual cannonballs. Bonemerang has separate outgoing/returning legs;
the return aims at the caster's position when that leg starts. Empowered range and
speed remain distinct. Rhythm refreshes on successful contact, not once per enemy
as a stacking bonus. No external global pending-projectile payload map is introduced.

Stats are sampled at impact, including Magmortar (previously captured at channel
start). Multi-hit bursts keep their existing hit count and resolve together on one
contact, not as independently staggered missiles. Pollen Puff chooses its ally at
launch and consumes Honey at impact. Spawn failure retains the old immediate
resolver as a fallback; a successfully spawned projectile never deals launch damage.
Default speed is the SDK's 5000 units/tick. Exact collision ordering, return/ground
timers, lost targets, buff offsets and animation timing need watched-match validation.
SFX calls exist but no new audio assets or audio wiring were added.

## Other Useful Features

- `lane_prior` seeds native lane suitability (wired from our existing table).
  Build 0.2.3 preserves every authored allowed lane across all 103 champions:
  specialists get 100; default dual-lane picks get 75/25 in authored order.
  Feraligatr, Emboar, Kleavor, Sawk/Throh, Hitmonchan, Pyukumuku, Kommo-o,
  Grapploct, Sirfetch'd, Armarouge, Goodra, Vaporeon and Smeargle get 50/50.
  Skeledirge gets Top 50 / Mid 25 / Support 25. These shares are new inferred
  balance defaults from the authored lane sets, not recovered historical numbers.
  Unsupported lanes are zero and every champion totals 100. Both seeding and the
  post-draft assignment solver use this same table.
  Blissey seeds Support=100, not the flat Top/Jungle default. This is a learned
  model prior, not an immutable UI label. Existing save/model learning can override
  it; actual new-game lane labels still need an in-game check.
- `decide_positions` now controls post-draft AI lane assignment using a complete
  optimal permutation for Pokemon-only teams. Mixed/unknown rosters retain engine
  assignment. An incompatible set of picks still cannot cover every lane; manual
  player choices are not forcibly overridden. No UI capture is needed for this.
- `can_activate`, `target_constraint`, dynamic range and state-dependent moving casts
  allow direct AI restrictions for Smeargle, attachments and tower-dash safety.
- Per-slot cooldown/charge reads and cooldown reduction are available.
- Real shields, invisibility, banish, teleport, unit spawning, stackable buffs,
  typed true damage and simulation-origin reads are available.
- Native passive registration for data champions now exists too. Hybrids are more
  capable, but no hybrid conversion is needed to obtain native VFX.
- No full champion sprite/model replacement setter was found. Attached animation
  effects are overlays, not complete stance/form animation-set swaps. The prior
  transparent-body/manual-overlay workaround remains unproven and deferred.
- Smeargle recursively executes the copied source champion/action with Smeargle's
  caster id before native VFX dispatch. This selects copied move art without
  replacing his body. Persistent/state-specific copied moves require tests;
  copied projectiles use the same native impact path while retaining Smeargle's
  caster entity; the copied move's source identity selects art and gameplay.

## Required In-Game Checks

- New Pokemon-only save; complete watched and background matches without panics.
- Compare damage/healing/KDA in live results, saved history and replay.
- Compare normal/mitigated hits, shield absorption, overkill, DOT, reflect,
  critical hits and custom lifesteal. Check duplicated crit/proc effects.
- Confirm one ordinary engine kill reward, no old fixed bonus on top.
- Test real shields and Snorlax minion/camp berries separately from champion kills.
- Check Gholdengo receives +10 per CS and no bonus from ordinary income/kill rewards.
- Check delayed hits, piercing, Bonemerang returns, cannonball cadence and copied shots.
- Before shipping VFX wiring, test fields/auras/marks/projectiles on several champions,
  including background matches/replays, and inspect scale/anchor/depth visually.

## Build Verification

Build 0.2.3: debug and release tests both passed 23 tests, with 1 intentionally
ignored companion exporter. Release DLL built without warnings. Tests cover all
69 standard projectile actions (plus the special Bonemerang schedule), launch/hit
separation, piercing target isolation, ordinary versus custom crit dispatch,
crit-stat restoration, and 1,000 duplicate CS callbacks yielding only one +10 bonus.
All 103 lane sets are checked for valid shares totaling 100. Guide regenerated.
406 package JSON documents validated; all 1088 installed files match the source
package, with zero `.data_champion` files. Deployed to Steam's existing
`mods/pokemon_moba_stable`. DLL SHA-256:
`7049C398352C6A5CC25186D70CE74D7405E5DD925F7695B2765FBCBED3A33354`.
No match was run. Tests exercise SDK dispatch with a mock host, not the
closed-source engine's actual collision, damage, item-proc or replay behavior.

### Previous Build 0.2.2

Build 0.2.2: 15 tests passed, 1 intentionally ignored companion exporter. Release
build succeeded without warnings. New tests cover inventory keys versus setting
slot names, mod items, unknown/malformed/duplicate definitions, equal timed stat
transfer, empty-item guards, item-only Poltergeist scaling, and catalog isolation
between matches. Guide and item-related tooltips were regenerated/updated.
405 package JSON documents validated. All 1088 installed files match the source
package, with zero `.data_champion` files. DLL SHA-256:
`11842301C225DE541D2D5EF013298C6D5022B67730D88D28F15A5E51086B8A31`.
Deployed to the existing Steam `mods/pokemon_moba_stable` folder. No match was run.
Projectile damage timing, Good as Gold, and crit handling were not changed.

### Previous Build 0.2.1

Build 0.2.1: 10 release tests passed, 1 intentionally ignored companion exporter.
Checks cover action tags/charges, victim forwarding, engine damage/shield dispatch,
post-draft lane assignment, unique roster-owned visual names, explicit state-event
sites, and exact VFX coordinates/full projectile names without damage calls.
All 279 animation files have sheets; 402 package JSON documents validated in the
selected JSON/fanim/mod-info/override/view-binding formats. All 1088 installed
files match the package byte-for-byte; no `.data_champion` files are installed.
DLL SHA-256: `26C5366EF923CFDA28279433DA15DEB653E9C2BD5CD3364A36F89C85ED728580`.
Installed to the existing Steam `mods/pokemon_moba_stable` folder. No match was run.
These tests validate our SDK calls, not the closed-source engine's stat arithmetic
or the renderer's appearance. Item mechanics and Good as Gold were not redesigned.

### Previous Build

Build 0.2.0 compiled against the installed SDK 0.6.4. Release tests: 6 passed,
1 intentionally ignored companion exporter. All 405 package JSON assets parse.
Deployed to the existing Steam `mods/pokemon_moba_stable` folder: 1087 source
and installed files, zero hash differences, zero `.data_champion` files.
DLL SHA-256: `9600F01471847FED3840508B0AACE7E67C3E6DA727EDB7624B405F2391654AB5`.
No in-game match, replay or persisted-results validation was performed.

References:

- https://github.com/teamsamoyed/TeamfightManager2Mod/blob/main/docs/stable-api-reference.md
- https://github.com/teamsamoyed/TeamfightManager2Mod/blob/main/docs/data-champion-schema/visual-bindings.md
- https://github.com/teamsamoyed/TeamfightManager2Mod/blob/main/docs/data-items.md

## Whole-Mod Audit Follow-Up

2026-10-18, build 0.2.3. Scope: the full roster's action/passive inventory,
shared combat/status/movement/AI adapters, draft, equipment, economy, VFX,
structures, runtime lifecycle, SDK/deployment conventions and test coverage.
No gameplay source, package metadata, DLL or installed files changed in this
audit. Noivern is explicitly deferred at the user's request.

This is a source-level compatibility and capability audit, not a claim that
every move has been executed or every interaction proven correct. The generated
matrix records every action implementation and passive, with review families.
The evidence below distinguishes confirmed integration gaps from suspected
engine interactions. Some available APIs predate 0.6.4; availability does not
mean the legacy adapter already uses them.

### Priority 1: Shared Correctness

**R1 - Runtime state is not isolated per simulation (confirmed architecture risk).**
`pokemon_status.rs:1930` onward uses process-wide state stores; many keys are
entity IDs only. `has_status_tick_rolled_back` (2111) detects a new match only
from the tick decreasing, and `pokemon_content.rs:24697` clears the global
stores on that signal. `update_statuses` (21018) skips a tick solely because that
tick number was already processed. Interleaved matches/replays with overlapping
entity IDs can share, reset or skip state. This is not evidence that such
interleaving caused the reported hangs. Test watched/unwatched matches, multiple
matches, replay restart and seeking before claiming isolation. Move authoritative
state into simulation-owned lifecycle/storage where possible; a seed alone is
not enough to distinguish concurrent original/replay instances of the same match.
The SDK exposes `sim_origin`, but its identity/lifecycle must be validated before
using it as a store key. Official documentation requires deterministic callbacks.

**C1 - Healing/anti-heal has two owners (confirmed duplicate representation;
double reduction needs an engine test).** `apply_anti_heal`
(`pokemon_status.rs:6198`) both stores a custom percentage and adds a native
`heal_reduce` buff. `heal_with_antiheal` (6262) reduces the heal first, caps it to
missing HP, then calls native `heal`. If native healing applies that buff again,
50% becomes 75% effective reduction. Repeated aura applications also append
same-name native buffs while the custom ledger takes a maximum. Establish the
engine's stacking and heal-cap order, then assign anti-heal one owner. Preserve
Ripen, Trick Room inversion, Yanmega drain denial and Pokemon-specific lifesteal
rules. Do not fix this by bypassing engine healing statistics.

**C2 - Cleanse does not clear active native CC (confirmed call-path gap).**
`cleanse_harmful_statuses` (`pokemon_status.rs:17568`) removes selected custom
status records and applies short future CC immunity. It never calls
`entity_clear_cc`, although the adapter already exposes it (1454). It also does
not remove the custom anti-heal list or native debuff buffs. Use explicit status
ownership/names and remove intended harmful effects; do not indiscriminately
remove self-roots or beneficial buffs. Tests must distinguish removing existing
CC from preventing new CC, including Blissey, Purifying Salt and self-cleanses.

**C3 - Engine damage does not automatically enter every Pokemon defense.**
`pokemon_types.rs:124` retains Protect redirection, Detect, Wonder Guard,
Sturdy/Endure-related checks, type rules and other pre-damage decisions. Native
item procs, tower damage and other engine sources do not automatically pass
through this function. The full native attack pipeline can now generate more
such effects. Verify incoming engine damage, shields, lifesteal, reflect,
death-prevention and assist credit separately from Pokemon-versus-Pokemon hits.
The SDK's post-damage passive callback is not a cancellable pre-damage hook.
Do not delete these preserved workarounds or silently redesign them.

Custom crit ownership is already split in 0.2.3, not wholly engine-owned.
Kingdra/Inteleon/overflow/forced-crit/crit-defense paths remain custom. Temporary
crit suppression prevents a second roll but does not mark the engine event as
critical; crit-only item behavior remains a known mismatch. Existing mock-host
tests do not establish engine mitigation/proc ordering.

**G1 - Receiver misses the new Good as Gold callback (confirmed gap).**
`pokemon_content.rs:25143` handles CS bonus only when `self.champion.id` is
Gholdengo. Passimian copying Gholdengo will not enter it, unlike the adjacent
Snorlax berry branch which explicitly supports Receiver. Extend copied-passive
coverage without removing owner validation, deduplication or Nullifying Gas
suppression. Native Gholdengo's +10 credited-CS route is implemented and tested;
wallet/CS polling is disabled and must stay disabled. Test multiple victims in
one tick, respawns, copied passives, death, suppression and replay reloads.

### Priority 2: AI And Movement

**A1 - New AI facilities are available but not forwarded (confirmed).**
`PlayerAiContext` (`stable_legacy_api.rs:535`) does not expose SDK `sim()` reads.
`LegacyAction` (1049) does not implement `can_activate`, `target_constraint` or
`can_use_with_move_by`; `LegacyEffect` (1101) does not forward dynamic range.
The SDK therefore sees permissive/default gates and static range. Add reusable
forwarding first, then action-specific policies across the whole roster:
healing/cleanse need, existing marks/buffs, charges, channel release, attachment,
stance, field placement, target eligibility and available cooldowns.

These gates guide AI, not enforcement of custom input. Recheck validity in effect
execution. Dynamic range adds a nonnegative amount and does not replace static
casting type/target metadata. SDK cooldown reads and fixed slot cooldown reduction
are also unused. Keep haste as haste; only replace timer approximations with
`entity_reduce_cooldown` where the intended mechanic is an immediate reduction.
Comfey's current Encore description specifies recovery rate, not an instant reset.

**A2 - Tower-safe input replacement can fail open (confirmed).**
`replace_if_valid` (`pokemon_content.rs:25491`) returns `Pass` when a replacement
is unavailable/invalid. `release_channel_input` tries two escape inputs and can
also return `Pass`; the adapter then preserves the original input. Thus rejecting
a dangerous dash does not guarantee the original dash is blocked. Snapshot-based
danger checks also accept old positions (30 ticks for entities, 180 for towers),
use a fixed tower radius and fail open when data is missing. They need current
simulation reads and an explicit safe no-action fallback. Evaluate real dashes
along their paths, blinks at their landing positions, and rescue/retreat moves
according to their distinct intent. Do not turn Pikachu's Volt Tackle into a blink.

**A3 - Smeargle can benefit substantially, but the new SDK has not fixed its AI.**
`smeargle_ready_copied_input` and related selectors (`pokemon_content.rs:25755`)
rank moves mainly by original slot and select nearby allies/enemies, rather than
the copied move's utility and target requirements. The four Sketch wrappers still
have fixed enemy-targeting/42000-range metadata (38370 onward), so validating a
wrapper input is not equivalent to validating the copied move. AI fallback also
calls `smeargle_force_learn_for_player`, mutating custom learned state during a
read-only AI query. Move learning into authoritative simulation callbacks; use
live target/HP/CC/cooldown/range information for selection. Test copied heals,
targeted shots, direction attacks, ground fields, channels and attached-only
skills. Copied VFX can still use the source move identity with Smeargle as caster.

**M1 - Intended teleports still use force movement (confirmed).**
Banette's Phantom Force (15894), Skarmory's Fly (12425), and Audino's Substitute
(12805) in `pokemon_content.rs` still call forced-movement helpers. The adapter's
`entity_set_pos` exists but has no gameplay callers. Use native teleport for the
intended blink/swap semantics, retaining destination checks, hostile-displacement
immunity, ally rescue rules, damage timing and all normal dashes. Do not broadly
replace every ForceMove: throws, pulls, charges and retreats have different rules.
Native knockback/grab/pull APIs can replace compatible approximations, but their
CC-immunity semantics must match the Pokemon movement resolver.

**M2 - Octillery's Suction Cups is still empty (confirmed).**
`update_suction_cups_placeholder` (`pokemon_status.rs:16941`) does nothing.
`ignore_wall` is already forwarded in BuffState. This enables a real terrain-
crossing implementation with the intended speed penalty, but does not prove the
base AI will choose wall-crossing routes. Test collision and navigation separately.

### Priority 2: Visibility, Identity And Persistent Effects

**S1 - Custom hidden/untargetable state is not native invisibility (confirmed).**
No active call to `entity_set_invisible` or `entity_banish` exists. Weavile's
Assaulting Hunt, Kleavor's alternating Stealth Rock, Comfey/Clawitzer attachment
and some aerial windows use `apply_soft_untargetable` (`pokemon_status.rs:6992`).
This is consulted by Pokemon selectors, not a universal engine targeting policy.
Native invisibility explicitly hides vision/targeting and is a candidate for real
stealth. Banish additionally locks input, so it is NOT an acceptable blanket
replacement for attached Pokemon that must continue casting. Establish attack/
damage break rules, reveal rules and cancellation behavior before switching.

Shiftry specifically has a different issue: `shiftry_inferred_in_bush` (15399)
infers bush cover from invisibility to nearby enemies. No nearby enemy means no
bush bonus; other causes of lost vision can resemble bush cover. Its current
passive grants movement speed and does not activate invisibility. Reading the
actual map bush grid could improve this; there is no simple per-entity is-in-bush
wrapper in the inspected API. Map customizer JSON supplies geometry, but linking
that geometry to the correct simulation needs lifecycle work. Ampharos, Starmie,
Xatu and Kricketune's custom reveals must be tested alongside any native stealth.

**E1 - Unit categories still rely on heuristics (confirmed).**
`neutral_objectives.rs` infers epic identity from HP/defenses/movement;
`is_jungle_camp` (`pokemon_content.rs:27538`) uses team 2 plus exclusions.
`EntityRef` does not expose native `kind()`. Forward the real kinds to distinguish
minions, jungle, epic/Serpen, summons, towers and nexus. This improves neutral
typing, objective bonuses, target filtering and kill-driven effects without
guessing from balance stats. Preserve intentional Pokemon neutral type mappings.

**F1 - VFX hooks work in principle; coverage is not visual acceptance.**
The catalog has 279 assets across 84 champions, including 73 projectile bindings.
The roster matrix explicitly lists the 19 champions without staged bindings;
Smeargle can use copied assets despite no own bindings. Current VFX use public
frame events, not the old companion gameplay/dummy effects. Persistent fields
refresh every 30 ticks and may have up to 30 ticks of visual tail after removal.
Check initial appearance, looping, consumed/cleansed/despawned state, world versus
entity anchors, scale, depth, spectator selection and replay. Preserve genuine
fields such as Arboliva terrain, Kricketune webs, Goodra pickups, Garganacl salt,
Dedenne grids and Quaquaval trails; a visual buff is not their gameplay state.

`play_sfx` is forwarded but has no gameplay callers, so adding VFX does not mean
custom skill audio is wired. `debug_draw_circle/line` in the adapter (1470) are
empty; legacy debug markers currently cannot draw anything. Forward them under
an explicit diagnostic switch, not as a substitute for combat art.

Stakataka/Celesteela body mappings exist, but the current tower `dead` tags are
idle-frame placeholders, not a real death animation. Do not confuse the separate
Groudon/Kyogre nexus destruction assets with tower death art. Full animated body
swapping for Sawk/Throh, Blood Moon or Ditto was not found in the public API;
the separately documented transparent-body/animation-overlay workaround remains
an experiment. `spawn_unit` supports a timed unit with supplied stats and attacks,
but its documented chase/attack AI is not an unrestricted summon behavior API.
Do not turn permanent traps or decoys into generic attacking summons by default.

**P1 - Projectile impact migration still needs engine-level verification.**
0.2.3 defers supported projectile hits, not merely visuals. Tests cover callback
dispatch and collided-target isolation, not engine collision/homing semantics.
Verify moving targets, collision widths, dead/hidden targets, caster death,
piercing once per target, multi-hit sequences, terrain/map edges, copied casts,
spawn failure, and whether proc/stat snapshots belong at launch or impact.
Arbok ground lobs use a distance/speed queued impact rather than collision;
Bonemerang schedules a return and Magmortar launches individual shots. Their
timing and fallback behavior deserve dedicated tests. SDK 0.6.4 specifically
fixes frame recording for projectiles spawned from passive callbacks, so earlier
invisible passive shots were not necessarily bad asset bindings.

### Remaining Systems And Verification

**I1 - Equipment integration is implemented, not full inventory manipulation.**
Symbiosis/Thief/Poltergeist use live item keys and supported equipment stats.
They do not transfer item identity or procs. Validate item upgrades during a buff,
expiry/death, repeated theft, percentage-stat scaling and fresh-process replays
with missing catalogs. Native item-build hooks are available but not registered;
these can select builds matching each Pokemon/stance without combat rewrites.

**D1 - Draft and metadata hooks are already present.** All 103 lane priors are
tested to total 100; post-draft assignment is registered. Test fresh saves,
existing learned history, partial/off-role teams and supported match sizes.
Do not reinstate UI capture. Tooltips/action tags are covered by export tests;
in-game localization and displayed lane labels still require a visual check.

**H1 - SDK/deployment and adapter hygiene.** Cargo points at the installed
0.6.4 SDK; build tooling checks the SDK version and resolved path. Native-only
packaging intentionally excludes `.data_champion` and the statistics injector.
Do not resurrect old source snapshots as live dependencies. Keep SDK upgrades
single-source, verify source/installed hashes and remove stale deployed payloads
only within the explicitly selected mod directory. This audit did not redeploy.
The adapter also maps unknown input codes to Move (`stable_legacy_api.rs:479`),
rather than safely ignoring them. Its raw-pointer GameCtx erases callback lifetimes
(1253); no escape was demonstrated here, but future AI integration should not
expand that unsafe pattern. Prefer callback-scoped read-only access for AI.

**Performance is an open measurement task, not a diagnosed engine limitation.**
Global vectors, repeated entity/buff snapshots and area scans occur across shared
updaters. Some loops are tick-throttled, so do not assume every passive duplicates
all work. Measure before restructuring. Test full-roster mixed matches with many
permanent fields, projectiles and statuses, including accelerated simulation.

### Recommended Work Order And Acceptance

1. Shared regression harness: anti-heal, cleanse, damage/procs, multi-simulation
   state, copied passives and safe AI cancellation. Preserve current mechanics.
2. Forward the new AI reads/gates, cooldowns and entity kinds once; then apply
   policies across all action families in the roster matrix, not one canary.
3. Replace intended teleport and terrain approximations; implement compatible
   native stealth with explicit reveal/break semantics. Leave Noivern unchanged.
4. Multi-champion live acceptance for projectile/field geometry, real shields,
   CC/cleanse, attachments, special crits, items, +10 CS, draft and statistics.
5. Repeat in accelerated/background matches and fresh-process replays. Compare
   replay/persisted damage, healing, self-heal, kills, assists, gold and CS; then
   tune performance. Add tower death art and audio only with verified assets.

Audit verification: `cargo test --offline` completed with 23 passed, 0 failed,
1 intentionally ignored companion-data exporter. The guide export was refreshed
by that suite. The inventory generator rejects incomplete/duplicate roster
coverage and unsupported action syntax, and accounts for 103 x 4 action slots.
No live match, visual acceptance or engine arithmetic was verified in this pass.
