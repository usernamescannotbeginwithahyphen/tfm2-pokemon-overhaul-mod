//! Swept contact delivery for dashes and rectangular waves. Damage remains in
//! the Pokemon resolver; movement never pre-applies damage to the whole route.
use crate::legacy::pokemon_content::{PokemonChampion, PokemonMove, PokemonMoveEffect as E};
use crate::{
    runtime_state::SimOnceLock,
    stable_legacy_api::{CCState, EntityPos, GameCtx, InputTarget},
};
use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
};
const WAVE_SPEED: u64 = 10000;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Dash {
        speed: u64,
        ticks: usize,
        width: u64,
        pierce: bool,
        targeted: bool,
    },
    Arrival {
        speed: u64,
        ticks: usize,
        range: u64,
    },
    Wave {
        range: u64,
        width: u64,
        thickness: u64,
    },
}

fn shape(effect: E, move_speed: usize) -> Option<Shape> {
    Some(match effect {
        E::DashEndConeDamage {
            force_move_speed,
            force_move_ticks,
            dash_range,
            ..
        } => Shape::Arrival {
            speed: force_move_speed,
            ticks: force_move_ticks as usize,
            range: dash_range,
        },
        E::LineFirstDashDamageFlameTrail {
            force_move_speed: speed,
            force_move_ticks: ticks,
            width,
            ..
        }
        | E::LineFirstDashDamageMissingHp {
            force_move_speed: speed,
            force_move_ticks: ticks,
            width,
            ..
        } => Shape::Dash {
            speed,
            ticks: ticks as usize,
            width,
            pierce: false,
            targeted: false,
        },
        E::LineFirstDashDamageDistanceSelfHp {
            force_move_speed,
            force_move_ticks,
            width,
            ..
        } => Shape::Dash {
            speed: force_move_speed.saturating_add(move_speed as u64 * 3),
            ticks: force_move_ticks as usize,
            width,
            pierce: false,
            targeted: false,
        },
        E::LineDashPierceDamage {
            force_move_speed: speed,
            force_move_ticks: ticks,
            width,
            ..
        }
        | E::LineDashPierceContactDrain {
            force_move_speed: speed,
            force_move_ticks: ticks,
            width,
            ..
        } => Shape::Dash {
            speed,
            ticks: ticks as usize,
            width,
            pierce: true,
            targeted: false,
        },
        E::StarmieFlipTurn { force_move_speed: speed, force_move_ticks: ticks, .. }
        | E::DashDamage {
            force_move_speed: speed,
            force_move_ticks: ticks,
            ..
        }
        | E::DashDamageReturn {
            force_move_speed: speed,
            force_move_ticks: ticks,
            ..
        }
        | E::DashDamageChanceParalysis {
            force_move_speed: speed,
            force_move_ticks: ticks,
            ..
        }
        | E::DashDamageChanceFreeze {
            force_move_speed: speed,
            force_move_ticks: ticks,
            ..
        }
        | E::DashDamageChanceDisarm {
            force_move_speed: speed,
            force_move_ticks: ticks,
            ..
        }
        | E::DashDamageChanceBurn {
            force_move_speed: speed,
            force_move_ticks: ticks,
            ..
        } => Shape::Dash {
            speed,
            ticks: ticks as usize,
            width: 10000,
            pierce: false,
            targeted: true,
        },
        E::QuaquavalUpTempo { dash_speed, dash_ticks, .. } => Shape::Dash {
            speed: dash_speed, ticks: dash_ticks, width: 6000, pierce: false, targeted: true,
        },
        E::RescueAllyDash { force_move_speed, force_move_ticks, .. } => Shape::Dash {
            speed: force_move_speed, ticks: force_move_ticks as usize, width: 6000, pierce: false, targeted: true,
        },
        E::TravelingWaveDamage {
            travel_range,
            wall_length,
            thickness,
            ..
        }
        | E::TravelingWaveDamageInfestation {
            travel_range,
            wall_length,
            thickness,
            ..
        }
        | E::TravelingWaveDamageSlowBurn {
            travel_range,
            wall_length,
            thickness,
            ..
        }
        | E::TravelingWaveDamageParalysis {
            travel_range,
            wall_length,
            thickness,
            ..
        }
        | E::TravelingWaveDamageChanceFreeze {
            travel_range,
            wall_length,
            thickness,
            ..
        }
        | E::TravelingWaveDamageTerrify {
            travel_range,
            wall_length,
            thickness,
            ..
        }
        | E::TravelingWavePoisonSlow {
            travel_range,
            wall_length,
            thickness,
            ..
        } => Shape::Wave {
            range: travel_range,
            width: wall_length,
            thickness,
        },
        _ => return None,
    })
}

#[derive(Clone)]
struct Delivery {
    champion: PokemonChampion,
    action: PokemonMove,
    original_action: PokemonMove,
    caster: usize,
    team: usize,
    origin: EntityPos,
    end: EntityPos,
    previous: EntityPos,
    started: usize,
    shape: Shape,
    target: Option<usize>,
    hit: BTreeSet<usize>,
    positions: BTreeMap<usize, EntityPos>,
    dedenne_empowered: Option<bool>,
    side_offset: u64,
}
static ACTIVE: SimOnceLock<Mutex<Vec<Delivery>>> = SimOnceLock::new();
thread_local! { static CONTACT: Cell<Option<(usize, EntityPos, EntityPos)>> = const { Cell::new(None) }; }
thread_local! { static TARGET: Cell<Option<(usize, usize)>> = const { Cell::new(None) }; }
struct TargetScope(Option<(usize, usize)>);
impl Drop for TargetScope { fn drop(&mut self) { TARGET.with(|v| v.set(self.0)); } }
pub fn contact_target(caster: usize) -> Option<usize> {
    TARGET.with(|v| v.get().filter(|v| v.0 == caster).map(|v| v.1))
}
struct ContactScope(Option<(usize, EntityPos, EntityPos)>);
impl Drop for ContactScope {
    fn drop(&mut self) {
        CONTACT.with(|s| s.set(self.0));
    }
}
pub fn resolving(caster: usize) -> bool {
    CONTACT.with(|s| s.get().is_some_and(|v| v.0 == caster))
}
pub fn origin(caster: usize) -> Option<EntityPos> {
    CONTACT.with(|s| s.get().filter(|v| v.0 == caster).map(|v| v.1))
}
pub fn endpoint(caster: usize) -> Option<EntityPos> {
    CONTACT.with(|s| s.get().filter(|v| v.0 == caster).map(|v| v.2))
}

pub fn reset() {
    ACTIVE.get_or_init(Default::default).lock().unwrap().clear();
}

fn distance(a: EntityPos, b: EntityPos) -> u64 {
    let x = a.x as i128 - b.x as i128;
    let y = a.y as i128 - b.y as i128;
    (x * x + y * y).isqrt() as u64
}
fn point_at(a: EntityPos, b: EntityPos, length: u64) -> EntityPos {
    let d = distance(a, b).max(1) as i128;
    EntityPos {
        x: (a.x as i128 + (b.x as i128 - a.x as i128) * length as i128 / d).max(0) as u64,
        y: (a.y as i128 + (b.y as i128 - a.y as i128) * length as i128 / d).max(0) as u64,
    }
}

pub fn launch(
    ctx: &mut GameCtx,
    champion: PokemonChampion,
    action: PokemonMove,
    caster: usize,
    input: InputTarget,
) -> bool {
    let Some(entity) = ctx.get_entity(caster) else {
        return false;
    };
    let extra = match action.effect {
        E::LinePierceDamageSlowApToAd { width, .. }
        | E::LinePierceDamageSelfRetreat { width, .. }
        | E::LinePierceDamageChanceBurn { width, .. }
        | E::LinePierceDamageChancePoison { width, .. }
        | E::LinePierceDamageStun { width, .. } if matches!(champion.id,
            "pokemon_moba_feraligatr" | "pokemon_moba_decidueye" | "pokemon_moba_flareon" | "pokemon_moba_weezing") =>
            Some(Shape::Wave { range: action.range, width: width * 2, thickness: 6000 }),
        E::WishiwashiWaveSplash { travel_range, schooling_travel_bonus, wall_length, .. } =>
            Some(Shape::Wave { range: travel_range + if crate::legacy::pokemon_status::wishiwashi_is_schooling(ctx,caster) {schooling_travel_bonus} else {0},
                width: wall_length, thickness: 6000 }),
        E::ArcanineWhiteFlames{travel_range,line_width,..} => Some(Shape::Wave {
            range:travel_range,width:line_width*2,thickness:6000}),
        _ => None,
    };
    let Some(mut shape) = extra.or_else(|| shape(action.effect, entity.stat().move_speed)) else {
        return false;
    };
    let origin = entity.pos();
    let team = entity.team();
    let aim = match input {
        InputTarget::Target { target_id } => ctx.get_entity(target_id).map(|t| t.pos()),
        InputTarget::Pos { x, y } => Some(EntityPos { x, y }),
        InputTarget::Dir { x, y } => Some(EntityPos {
            x: (origin.x as i128 + x as i128).max(0) as u64,
            y: (origin.y as i128 + y as i128).max(0) as u64,
        }),
        InputTarget::None => None,
    };
    // Recognized delivery must not fall back to instant damage on invalid input.
    let Some(aim) = aim.filter(|p| *p != origin) else {
        return true;
    };
    let range = match shape {
        Shape::Wave { range, .. } => range,
        Shape::Arrival {
            speed,
            ticks,
            range,
        } => range.min(speed.saturating_mul(ticks as u64)),
        Shape::Dash {
            speed,
            ticks,
            targeted,
            ..
        } => {
            let max = action.range.min(speed.saturating_mul(ticks as u64));
            if targeted || !matches!(input, InputTarget::Dir { .. }) {
                max.min(distance(origin, aim))
            } else {
                max
            }
        }
    };
    if range == 0 {
        return true;
    }
    let end = point_at(origin, aim, range);
    if matches!(action.effect,E::QuaquavalUpTempo{..}|E::StarmieFlipTurn{..}|E::RescueAllyDash{..}) {
        if let Shape::Dash {ref mut speed,ticks,..}=shape {
            *speed=(*speed).min(range.div_ceil(ticks.max(1) as u64));
        }
    }
    if !matches!(shape, Shape::Wave { .. })
        && crate::legacy::pokemon_status::smoke_screen_blocks_line(ctx, origin, end)
    {
        return true;
    }
    if let E::LinePierceDamageSelfRetreat { retreat_speed, retreat_ticks, .. } = action.effect {
        crate::legacy::pokemon_status::schedule_force_away_from_pos(ctx, caster, aim, 0, retreat_speed, retreat_ticks);
    }
    let positions = (0..ctx.entity_count())
        .filter_map(|i| ctx.entity_at(i).map(|e| (e.id(), e.pos())))
        .collect();
    let state = Delivery {
        champion,
        action,
        original_action: action,
        caster,
        team,
        origin,
        end,
        previous: origin,
        started: ctx.tick(),
        shape,
        target: if let InputTarget::Target { target_id } = input {
            Some(target_id)
        } else {
            None
        },
        hit: BTreeSet::new(),
        positions,
        dedenne_empowered: None,
        side_offset: if let E::ArcanineWhiteFlames{side_offset,..}=action.effect {side_offset} else {0},
    };
    if !matches!(shape, Shape::Wave { .. }) {
        step_dash(ctx, &state, origin);
    } else {
        if state.side_offset>0 {
            let dx=end.x as f64-origin.x as f64;let dy=end.y as f64-origin.y as f64;
            let distance=dx.hypot(dy).max(1.0);
            for sign in [-1.0,1.0] {
                let shift=|p:EntityPos|EntityPos{x:(p.x as f64-dy/distance*state.side_offset as f64*sign).max(0.0) as u64,
                    y:(p.y as f64+dx/distance*state.side_offset as f64*sign).max(0.0) as u64};
                crate::native_vfx::traveling_wave(ctx,champion.id,action,caster,shift(origin),shift(end),WAVE_SPEED);
            }
        } else {crate::native_vfx::traveling_wave(ctx, champion.id, action, caster, origin, end, WAVE_SPEED);}
    }
    let mut states = ACTIVE.get_or_init(Default::default).lock().unwrap();
    let mut replaced = Vec::new();
    // A new dash supersedes the old movement, but independently fired waves coexist.
    if !matches!(shape, Shape::Wave { .. }) {
        for old in states
            .iter()
            .filter(|s| s.caster == caster && !matches!(s.shape, Shape::Wave { .. }))
        {
            replaced.push((old.champion.id, old.action));
        }
        states.retain(|s| s.caster != caster || matches!(s.shape, Shape::Wave { .. }));
    }
    states.push(state);
    drop(states);
    for (id, old_action) in replaced {
        crate::native_vfx::dash_end(ctx, id, old_action, caster);
    }
    if let Shape::Dash { ticks, .. } = shape {
        crate::native_vfx::dash_start(ctx, champion.id, action, caster, ticks);
    }
    true
}

pub fn launch_dedenne(ctx: &mut GameCtx, champion: PokemonChampion, action: PokemonMove,
    caster: usize, end: EntityPos, empowered: bool) {
    let E::DedennePlayRough { width, force_move_speed, force_move_ticks, .. } = action.effect else { return; };
    let Some(source) = ctx.get_entity(caster) else { return; };
    let mut transport = action;
    transport.range = distance(source.pos(),end);
    transport.effect = E::LineDashPierceDamage { base_ad: 0, ad_ratio: 0, width,
        force_move_speed, force_move_ticks };
    launch(ctx,champion,transport,caster,InputTarget::Pos{x:end.x,y:end.y});
    let mut states=ACTIVE.get_or_init(Default::default).lock().unwrap();
    if let Some(s)=states.iter_mut().rev().find(|s|s.caster==caster && s.started==ctx.tick()) {
        s.action=action; s.original_action=action; s.dedenne_empowered=Some(empowered);
    }
}

pub fn launch_secondary(ctx: &mut GameCtx, champion: PokemonChampion, original: PokemonMove,
    transport: PokemonMove, caster: usize, input: InputTarget) {
    launch(ctx,champion,transport,caster,input);
    let mut states=ACTIVE.get_or_init(Default::default).lock().unwrap();
    if let Some(s)=states.iter_mut().rev().find(|s|s.caster==caster && s.started==ctx.tick()) {
        s.original_action=original;
    }
}

fn step_dash(ctx: &mut GameCtx, state: &Delivery, pos: EntityPos) {
    let (Shape::Dash { speed, .. } | Shape::Arrival { speed, .. }) = state.shape else {
        return;
    };
    let speed = speed.min(distance(pos, state.end));
    if speed == 0 {
        return;
    }
    // One-tick leases allow stopping on contact without clearing unrelated CC.
    crate::legacy::pokemon_status::apply_pokemon_cc(
        ctx,
        state.caster,
        state.caster,
        CCState::ForceMove {
            tick: 1,
            dx: state.end.x as i64 - pos.x as i64,
            dy: state.end.y as i64 - pos.y as i64,
            speed,
        },
    );
}

// Entry time into a box, using relative motion. This also catches targets crossing
// through a moving wave between ticks instead of checking only their endpoint.
fn box_entry(a: (f64, f64), b: (f64, f64), half: (f64, f64)) -> Option<f64> {
    let (mut enter, mut leave) = (0.0_f64, 1.0_f64);
    for (a, b, h) in [(a.0, b.0, half.0), (a.1, b.1, half.1)] {
        let delta = b - a;
        if delta.abs() < 0.000001 {
            if a.abs() > h {
                return None;
            }
        } else {
            let u = (-h - a) / delta;
            let v = (h - a) / delta;
            enter = enter.max(u.min(v));
            leave = leave.min(u.max(v));
        }
    }
    (enter <= leave).then_some(enter)
}

pub fn update(ctx: &mut GameCtx, seed: u64) {
    let states = {
        let mut active = ACTIVE.get_or_init(Default::default).lock().unwrap();
        std::mem::take(&mut *active)
    };
    let mut remaining = Vec::new();
    for mut s in states {
        if ctx.tick() <= s.started {
            remaining.push(s);
            continue;
        }
        let Some(caster) = ctx.get_entity(s.caster) else {
            crate::native_vfx::dash_end(ctx, s.champion.id, s.action, s.caster);
            continue;
        };
        let elapsed = ctx.tick().saturating_sub(s.started);
        let (pos, half, pierce, targeted, done) = match s.shape {
            Shape::Arrival { speed, ticks, .. } => {
                if !caster.is_alive() {
                    continue;
                }
                let pos = caster.pos();
                if pos == s.previous
                    || distance(pos, s.previous) > speed.saturating_mul(2).max(10000)
                {
                    continue;
                }
                (
                    pos,
                    (0.0, 0.0),
                    true,
                    false,
                    elapsed >= ticks || distance(pos, s.end) <= 1,
                )
            }
            Shape::Wave {
                range,
                width,
                thickness,
            } => {
                let traveled = (elapsed as u64).saturating_mul(WAVE_SPEED).min(range);
                (
                    point_at(s.origin, s.end, traveled),
                    (thickness as f64 / 2.0, width as f64 / 2.0),
                    true,
                    false,
                    traveled >= range,
                )
            }
            Shape::Dash {
                ticks,
                width,
                pierce,
                targeted,
                speed,
            } => {
                if !caster.is_alive() {
                    crate::native_vfx::dash_end(ctx, s.champion.id, s.action, s.caster);
                    continue;
                }
                let pos = caster.pos();
                // Walls/CC may prevent a step. Never hit targets along intended but untraveled movement.
                let interrupted = pos == s.previous
                    || distance(pos, s.previous) > speed.saturating_mul(2).max(10000);
                if interrupted {
                    crate::native_vfx::dash_end(ctx, s.champion.id, s.action, s.caster);
                    continue;
                }
                (
                    pos,
                    (width as f64, width as f64),
                    pierce,
                    targeted,
                    elapsed >= ticks || distance(pos, s.end) <= 1,
                )
            }
        };
        let d = distance(s.origin, s.end).max(1) as f64;
        let ux = (s.end.x as f64 - s.origin.x as f64) / d;
        let uy = (s.end.y as f64 - s.origin.y as f64) / d;
        let relative = |p: EntityPos, c: EntityPos| {
            let x = p.x as f64 - c.x as f64;
            let y = p.y as f64 - c.y as f64;
            (x * ux + y * uy, -x * uy + y * ux)
        };
        let mut contacts = Vec::new();
        for i in 0..ctx.entity_count() {
            if matches!(s.shape, Shape::Arrival { .. }) {
                break;
            }
            let Some(target) = ctx.entity_at(i) else {
                continue;
            };
            let id = target.id();
            let target_pos = target.pos();
            let prev = s.positions.insert(id, target_pos).unwrap_or(target_pos);
            let ally = target.team() == s.team;
            let allowed_team = if matches!(s.action.effect, E::RescueAllyDash { .. }) { ally }
                else if matches!(s.action.effect, E::WishiwashiWaveSplash { .. }) { !ally || target.is_champion() }
                else { !ally };
            if !allowed_team
                || !target.is_alive()
                || !target.is_targetable()
                || s.hit.contains(&id)
                || (targeted && Some(id) != s.target)
                || (!ally && !crate::legacy::pokemon_content::move_damage_target_allowed(
                    ctx, s.champion, s.action, id,
                ))
            {
                continue;
            }
            let a=relative(prev,s.previous);let b=relative(target_pos,pos);
            let contact=if s.side_offset==0 {box_entry(a,b,half)} else {
                [-1.0,1.0].into_iter().filter_map(|sign|box_entry((a.0,a.1-sign*s.side_offset as f64),
                    (b.0,b.1-sign*s.side_offset as f64),half)).min_by(f64::total_cmp)
            };
            if let Some(t) = contact
            {
                contacts.push((t, id));
            }
        }
        contacts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        if !pierce {
            contacts.truncate(1);
        }
        let stopped = !pierce && !contacts.is_empty();
        if stopped {
            let t = contacts[0].0;
            ctx.entity_set_pos(
                s.caster,
                (s.previous.x as f64 + (pos.x as f64 - s.previous.x as f64) * t).max(0.0) as u64,
                (s.previous.y as f64 + (pos.y as f64 - s.previous.y as f64) * t).max(0.0) as u64,
            );
        }
        for (_, id) in contacts {
            s.hit.insert(id);
            let _target = TargetScope(TARGET.with(|v| v.replace(Some((s.caster, id)))));
            let _guard =
                ContactScope(CONTACT.with(|v| v.replace(Some((s.caster, s.origin, s.end)))));
            if matches!(s.shape, Shape::Dash { .. }) {
                crate::native_vfx::on_cast(
                    ctx,
                    s.champion.id,
                    s.action,
                    s.caster,
                    InputTarget::Target { target_id: id },
                );
            }
            if let Some(empowered) = s.dedenne_empowered {
                crate::legacy::pokemon_content::resolve_dedenne_contact(ctx,s.champion,s.action,s.caster,id,empowered);
            } else {
                crate::projectiles::with_original_action(s.original_action,|| crate::legacy::pokemon_content::resolve_projectile_impact(
                ctx,
                s.champion,
                s.action,
                seed,
                s.caster,
                InputTarget::Target { target_id: id },
                ));
            }
        }
        if done && matches!(s.shape, Shape::Arrival { .. }) {
            let _guard = ContactScope(CONTACT.with(|v| v.replace(Some((s.caster, s.origin, pos)))));
            let input = InputTarget::Pos { x: pos.x, y: pos.y };
            crate::native_vfx::on_cast(ctx, s.champion.id, s.action, s.caster, input);
            crate::legacy::pokemon_content::resolve_projectile_impact(
                ctx, s.champion, s.action, seed, s.caster, input,
            );
        }
        if !done && !stopped {
            s.previous = pos;
            if !matches!(s.shape, Shape::Wave { .. }) {
                step_dash(ctx, &s, pos);
            }
            remaining.push(s);
        } else if !matches!(s.shape, Shape::Wave { .. }) {
            crate::native_vfx::dash_end(ctx, s.champion.id, s.action, s.caster);
        }
    }
    ACTIVE
        .get_or_init(Default::default)
        .lock()
        .unwrap()
        .extend(remaining);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swept_contact_does_not_hit_ahead_and_catches_crossings() {
        assert_eq!(box_entry((30.0, 0.0), (20.0, 0.0), (2.0, 10.0)), None);
        assert!(box_entry((5.0, 0.0), (-5.0, 0.0), (2.0, 10.0)).is_some());
        assert!(box_entry((0.0, 20.0), (0.0, -20.0), (2.0, 10.0)).is_some());
        assert_eq!(box_entry((5.0, 11.0), (-5.0, 11.0), (2.0, 10.0)), None);
    }
    #[test]
    fn motion_endpoint_is_bounded() {
        let a = EntityPos { x: 100, y: 100 };
        let b = EntityPos { x: 103, y: 104 };
        assert_eq!(point_at(a, b, 5), b);
        assert_eq!(point_at(a, b, 0), a);
    }
}
