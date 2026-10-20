#![allow(dead_code)]

use mod_api_stable::{
    AttackTypeV1, BuffDurationV1, BuffV1, CastingTargetV1, CastingTypeV1, CcKindV1, CcV1,
    ChampionCategoryV1, ChampionTagV1, InputKindV1, InputTargetKindV1, InputTargetV1, InputV1,
    KillLogV1, LaneV1, StableAction, StableAiContext, StableAiInit, StableChampion,
    StableDraftContext, StableDraftDecision, StableEffectSpec, StableEffectType, StableMod,
    StablePassive, StablePlayerAi, StableSim, StatV1,
};

pub type EntityStat = StatV1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EntityPos {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EntityHp {
    pub current: usize,
    pub max: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackType {
    BaseAttack,
    Skill,
    Dot,
    DotIgnoreShield,
    Item,
    Well,
}

impl AttackType {
    pub(crate) fn to_stable(self) -> AttackTypeV1 {
        match self {
            Self::BaseAttack => AttackTypeV1::BaseAttack,
            Self::Skill => AttackTypeV1::Skill,
            Self::Dot => AttackTypeV1::Dot,
            Self::DotIgnoreShield => AttackTypeV1::DotIgnoreShield,
            Self::Item => AttackTypeV1::Item,
            Self::Well => AttackTypeV1::Well,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastingType {
    Targeting,
    Position,
    Direction,
    None,
}

impl CastingType {
    fn to_stable(self) -> CastingTypeV1 {
        match self {
            Self::Targeting => CastingTypeV1::Targeting,
            Self::Position => CastingTypeV1::Position,
            Self::Direction => CastingTypeV1::Direction,
            Self::None => CastingTypeV1::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastingTarget {
    Ally,
    AllyChampion,
    AllyChampionInCC,
    AllyNotSelf,
    AllyOnlySelf,
    Enemy,
    EnemyWithoutTower,
    EnemyChampion,
    EnemyChampionInCC,
    EnemyChampionRecentlyAttacked,
    Both,
    BothWithoutTower,
    BothChampion,
    None,
}

impl CastingTarget {
    fn to_stable(self) -> CastingTargetV1 {
        match self {
            Self::Ally => CastingTargetV1::Ally,
            Self::AllyChampion => CastingTargetV1::AllyChampion,
            Self::AllyChampionInCC => CastingTargetV1::AllyChampionInCc,
            Self::AllyNotSelf => CastingTargetV1::AllyNotSelf,
            Self::AllyOnlySelf => CastingTargetV1::AllyOnlySelf,
            Self::Enemy => CastingTargetV1::Enemy,
            Self::EnemyWithoutTower => CastingTargetV1::EnemyWithoutTower,
            Self::EnemyChampion => CastingTargetV1::EnemyChampion,
            Self::EnemyChampionInCC => CastingTargetV1::EnemyChampionInCc,
            Self::EnemyChampionRecentlyAttacked => CastingTargetV1::EnemyChampionRecentlyAttacked,
            Self::Both => CastingTargetV1::Both,
            Self::BothWithoutTower => CastingTargetV1::BothWithoutTower,
            Self::BothChampion => CastingTargetV1::BothChampion,
            Self::None => CastingTargetV1::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChampionCategory {
    Melee,
    Range,
    Magician,
    Util,
    Assassin,
}

impl ChampionCategory {
    fn to_stable(self) -> ChampionCategoryV1 {
        match self {
            Self::Melee => ChampionCategoryV1::Melee,
            Self::Range => ChampionCategoryV1::Range,
            Self::Magician => ChampionCategoryV1::Magician,
            Self::Util => ChampionCategoryV1::Util,
            Self::Assassin => ChampionCategoryV1::Assassin,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChampionTag {
    AD,
    AP,
    Heal,
    Shield,
    Dot,
    DOT,
    CC,
    Range,
    Melee,
    Tank,
    Magic,
}

impl ChampionTag {
    fn to_stable(self) -> ChampionTagV1 {
        match self {
            Self::AD => ChampionTagV1::Ad,
            Self::AP => ChampionTagV1::Ap,
            Self::Heal => ChampionTagV1::Heal,
            Self::Shield => ChampionTagV1::Shield,
            Self::Dot => ChampionTagV1::Dot,
            Self::DOT => ChampionTagV1::Dot,
            Self::CC => ChampionTagV1::Cc,
            Self::Range => ChampionTagV1::Range,
            Self::Melee => ChampionTagV1::Melee,
            Self::Tank => ChampionTagV1::Tank,
            Self::Magic => ChampionTagV1::Magic,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Top = 0,
    Jungle = 1,
    Mid = 2,
    Bottom = 3,
    Support = 4,
}

impl Position {
    fn from_stable(lane: LaneV1) -> Self {
        match lane {
            LaneV1::Jungle => Self::Jungle,
            LaneV1::Mid => Self::Mid,
            LaneV1::Bottom => Self::Bottom,
            LaneV1::Support => Self::Support,
            _ => Self::Top,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuffType {
    Permanent,
    Time { tick: usize },
    WithShield { tick: usize, shield: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuffState {
    pub duration: BuffType,
    pub attack: i32,
    pub attack_mult: i32,
    pub magic_power: i32,
    pub magic_power_mult: i32,
    pub defence: i32,
    pub defence_mult: i32,
    pub hp: i32,
    pub hp_regen: i32,
    pub magic_resistance: i32,
    pub magic_resistance_mult: i32,
    pub vamp: i32,
    pub hp_mult: i32,
    pub move_speed_mult: i32,
    pub attack_speed_mult: i32,
    pub skill_cooldown_mult: i32,
    pub ult_cooldown_mult: i32,
    pub radius_mult: i32,
    pub crit_chance: i32,
    pub damage_reflect: usize,
    pub damaged_amplify: usize,
    pub damaged_reduce: usize,
    pub defence_penetration: usize,
    pub magic_resistance_penetration: usize,
    pub toughness: usize,
    pub heal_reduce: usize,
    pub range: usize,
    pub base_attack_enemy_max_hp_damage: usize,
    pub self_max_hp_damage: usize,
    pub skill_enemy_max_hp_damage: usize,
    pub dot_amplify: usize,
    pub base_attack_damaged_reduce: usize,
    pub skill_damaged_reduce: usize,
    pub cc_immune: bool,
    pub undying: bool,
    pub ignore_wall: bool,
}

impl Default for BuffState {
    fn default() -> Self {
        Self {
            duration: BuffType::Permanent,
            attack: 0,
            attack_mult: 0,
            magic_power: 0,
            magic_power_mult: 0,
            defence: 0,
            defence_mult: 0,
            hp: 0,
            hp_regen: 0,
            magic_resistance: 0,
            magic_resistance_mult: 0,
            vamp: 0,
            hp_mult: 0,
            move_speed_mult: 0,
            attack_speed_mult: 0,
            skill_cooldown_mult: 0,
            ult_cooldown_mult: 0,
            radius_mult: 0,
            crit_chance: 0,
            damage_reflect: 0,
            damaged_amplify: 0,
            damaged_reduce: 0,
            defence_penetration: 0,
            magic_resistance_penetration: 0,
            toughness: 0,
            heal_reduce: 0,
            range: 0,
            base_attack_enemy_max_hp_damage: 0,
            self_max_hp_damage: 0,
            skill_enemy_max_hp_damage: 0,
            dot_amplify: 0,
            base_attack_damaged_reduce: 0,
            skill_damaged_reduce: 0,
            cc_immune: false,
            undying: false,
            ignore_wall: false,
        }
    }
}

impl BuffState {
    fn to_stable(&self) -> BuffV1 {
        let mut buff = match self.duration {
            BuffType::Permanent => BuffV1::named("pokemon_moba_legacy_buff"),
            BuffType::Time { tick } => BuffV1::timed("pokemon_moba_legacy_buff", tick),
            BuffType::WithShield { tick, .. } => {
                let mut buff = BuffV1::timed("pokemon_moba_legacy_shield_buff", tick);
                buff.duration_kind = BuffDurationV1::WithShield.code();
                buff
            }
        };
        buff.attack = self.attack;
        buff.attack_mult = self.attack_mult;
        buff.magic_power = self.magic_power;
        buff.magic_power_mult = self.magic_power_mult;
        buff.defence = self.defence;
        buff.defence_mult = self.defence_mult;
        buff.hp = self.hp;
        buff.hp_regen = self.hp_regen;
        buff.magic_resistance = self.magic_resistance;
        buff.magic_resistance_mult = self.magic_resistance_mult;
        buff.vamp = self.vamp;
        buff.hp_mult = self.hp_mult;
        buff.move_speed_mult = self.move_speed_mult;
        buff.attack_speed_mult = self.attack_speed_mult;
        buff.skill_cooldown_mult = self.skill_cooldown_mult;
        buff.ult_cooldown_mult = self.ult_cooldown_mult;
        buff.radius_mult = self.radius_mult;
        buff.crit_chance = self.crit_chance;
        buff.damage_reflect = self.damage_reflect;
        buff.damaged_amplify = self.damaged_amplify;
        buff.damaged_reduce = self.damaged_reduce;
        buff.defence_penetration = self.defence_penetration;
        buff.magic_resistance_penetration = self.magic_resistance_penetration;
        buff.toughness = self.toughness;
        buff.heal_reduce = self.heal_reduce;
        buff.range = self.range;
        buff.base_attack_enemy_max_hp_damage = self.base_attack_enemy_max_hp_damage;
        buff.self_max_hp_damage = self.self_max_hp_damage;
        buff.skill_enemy_max_hp_damage = self.skill_enemy_max_hp_damage;
        buff.dot_amplify = self.dot_amplify;
        buff.base_attack_damaged_reduce = self.base_attack_damaged_reduce;
        buff.skill_damaged_reduce = self.skill_damaged_reduce;
        buff.cc_immune = self.cc_immune;
        buff.undying = self.undying;
        buff.ignore_wall = self.ignore_wall;
        buff
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCState {
    Airborne {
        tick: u64,
    },
    Stun {
        tick: u64,
    },
    Bind {
        tick: u64,
    },
    BlockAttack {
        tick: usize,
    },
    BlockSkill {
        tick: usize,
    },
    BlockMoveSkill {
        tick: usize,
    },
    ForceMove {
        tick: u64,
        dx: i64,
        dy: i64,
        speed: u64,
    },
    Taunt {
        tick: u64,
        target: usize,
    },
    Fear {
        tick: u64,
        dx: i64,
        dy: i64,
    },
    Charm {
        tick: u64,
        dx: i64,
        dy: i64,
    },
    Animation {
        tick: u64,
    },
}

impl CCState {
    fn to_stable(self) -> CcV1 {
        match self {
            Self::Airborne { tick } => CcV1::of_kind(CcKindV1::Airborne, tick),
            Self::Stun { tick } => CcV1::of_kind(CcKindV1::Stun, tick),
            Self::Bind { tick } => CcV1::of_kind(CcKindV1::Bind, tick),
            Self::BlockAttack { tick } => CcV1::of_kind(CcKindV1::BlockAttack, tick as u64),
            Self::BlockSkill { tick } => CcV1::of_kind(CcKindV1::BlockSkill, tick as u64),
            Self::BlockMoveSkill { tick } => CcV1::of_kind(CcKindV1::BlockMoveSkill, tick as u64),
            Self::ForceMove {
                tick,
                dx,
                dy,
                speed,
            } => {
                let mut cc = CcV1::of_kind(CcKindV1::ForceMove, tick);
                cc.dx = dx;
                cc.dy = dy;
                cc.speed = speed;
                cc
            }
            Self::Taunt { tick, target } => {
                let mut cc = CcV1::of_kind(CcKindV1::Taunt, tick);
                cc.target = target;
                cc
            }
            Self::Fear { tick, dx, dy } => {
                let mut cc = CcV1::of_kind(CcKindV1::Fear, tick);
                cc.dx = dx;
                cc.dy = dy;
                cc
            }
            Self::Charm { tick, dx, dy } => {
                let mut cc = CcV1::of_kind(CcKindV1::Charm, tick);
                cc.dx = dx;
                cc.dy = dy;
                cc
            }
            Self::Animation { tick } => CcV1::of_kind(CcKindV1::Animation, tick),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputTarget {
    None,
    Target { target_id: usize },
    Pos { x: u64, y: u64 },
    Dir { x: i64, y: i64 },
}

impl InputTarget {
    fn to_stable(self) -> InputTargetV1 {
        match self {
            Self::None => InputTargetV1::NONE,
            Self::Target { target_id } => InputTargetV1::target(target_id),
            Self::Pos { x, y } => InputTargetV1::pos(x, y),
            Self::Dir { x, y } => InputTargetV1::dir(x, y),
        }
    }

    pub(crate) fn from_stable(input: InputTargetV1) -> Self {
        if input.kind == InputTargetKindV1::Target.code() {
            Self::Target {
                target_id: input.target_id,
            }
        } else if input.kind == InputTargetKindV1::Pos.code() {
            Self::Pos {
                x: input.x,
                y: input.y,
            }
        } else if input.kind == InputTargetKindV1::Dir.code() {
            Self::Dir {
                x: input.dir_x,
                y: input.dir_y,
            }
        } else {
            Self::None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    Attack { target: InputTarget },
    Skill { target: InputTarget },
    Skill2 { target: InputTarget },
    Ult { target: InputTarget },
    Move { x: u64, y: u64 },
    Return,
}

impl Input {
    fn to_stable(self) -> InputV1 {
        match self {
            Self::Attack { target } => InputV1::action(InputKindV1::Attack, target.to_stable()),
            Self::Skill { target } => InputV1::action(InputKindV1::Skill, target.to_stable()),
            Self::Skill2 { target } => InputV1::action(InputKindV1::Skill2, target.to_stable()),
            Self::Ult { target } => InputV1::action(InputKindV1::Ult, target.to_stable()),
            Self::Move { x, y } => InputV1::move_to(x, y),
            Self::Return => InputV1::return_home(),
        }
    }

    fn from_stable(input: InputV1) -> Self {
        if input.kind == InputKindV1::Attack.code() {
            Self::Attack {
                target: InputTarget::from_stable(input.target),
            }
        } else if input.kind == InputKindV1::Skill.code() {
            Self::Skill {
                target: InputTarget::from_stable(input.target),
            }
        } else if input.kind == InputKindV1::Skill2.code() {
            Self::Skill2 {
                target: InputTarget::from_stable(input.target),
            }
        } else if input.kind == InputKindV1::Ult.code() {
            Self::Ult {
                target: InputTarget::from_stable(input.target),
            }
        } else if input.kind == InputKindV1::Return.code() {
            Self::Return
        } else {
            Self::Move {
                x: input.x,
                y: input.y,
            }
        }
    }
}

pub enum PlayerInputDecision {
    Pass,
    Block,
    Replace(Input),
}

pub struct PlayerAiInitContext {
    pub player_id: usize,
    pub athlete_id: usize,
    pub team: usize,
    pub position: Position,
    pub champion_name: String,
}

impl PlayerAiInitContext {
    fn from_stable(init: &StableAiInit) -> Self {
        Self {
            player_id: init.player_id,
            athlete_id: init.athlete_id,
            team: init.team,
            position: init
                .lane
                .map(Position::from_stable)
                .unwrap_or(Position::Top),
            champion_name: init.champion_name.clone(),
        }
    }
}

pub struct PlayerAiContext<'a, 'b, 'c> {
    inner: &'a mut StableAiContext<'b>,
    champion_name: String,
    world: Option<crate::ai_world::World>,
    _scope: Option<crate::runtime_state::Scope>,
    _marker: std::marker::PhantomData<&'c ()>,
}

impl<'a, 'b, 'c> PlayerAiContext<'a, 'b, 'c> {
    fn from_stable(inner: &'a mut StableAiContext<'b>) -> Self {
        let champion_name = inner.champion_name().unwrap_or_default();
        let player = inner.player_id();
        let (world, scope) = if let Some(sim) = inner.sim() {
            let scope = crate::runtime_state::Scope::enter(&sim);
            let world = sim.get_player(player).and_then(|player| player.champion())
                .map(|caster| crate::ai_world::World::for_player(&sim, caster.id(), player));
            (world, Some(scope))
        } else { (None, None) };
        Self {
            inner,
            champion_name,
            world,
            _scope: scope,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn player_id(&self) -> usize {
        self.inner.player_id()
    }
    pub fn world(&self) -> Option<&crate::ai_world::World> { self.world.as_ref() }
    pub fn athlete_id(&self) -> usize {
        self.inner.athlete_id()
    }
    pub fn team(&self) -> usize {
        self.inner.team()
    }
    pub fn position(&self) -> Position {
        self.inner
            .lane()
            .map(Position::from_stable)
            .unwrap_or(Position::Top)
    }
    pub fn champion_name(&self) -> &str {
        &self.champion_name
    }
    pub fn tick(&self) -> usize {
        self.inner.tick()
    }
    pub fn hp_ratio_percent(&self) -> Option<usize> {
        self.inner.hp_ratio_percent()
    }

    pub fn is_valid_input(&self, input: &Input) -> bool {
        self.inner.is_valid_input(&input.to_stable())
    }

    pub fn get_run_away_input(&mut self) -> Option<Input> {
        self.inner.run_away_input().map(Input::from_stable)
    }

    pub fn get_run_away_without_skill_input(&mut self) -> Option<Input> {
        self.inner
            .run_away_without_skill_input()
            .map(Input::from_stable)
    }
}

pub trait ModPlayerInputAi: Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn ModPlayerInputAi>;
    fn id(&self) -> &str;
    fn priority(&self) -> i32 {
        0
    }
    fn matches(&self, _ctx: &PlayerAiInitContext) -> bool {
        false
    }
    fn think(
        &mut self,
        _ctx: &mut PlayerAiContext<'_, '_, '_>,
        _base_input: Option<Input>,
    ) -> PlayerInputDecision {
        PlayerInputDecision::Pass
    }
}

pub struct LegacyPlayerAi {
    inner: Box<dyn ModPlayerInputAi>,
}

impl LegacyPlayerAi {
    pub fn new<T>(inner: T) -> Self
    where
        T: ModPlayerInputAi,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl StablePlayerAi for LegacyPlayerAi {
    fn clone_box(&self) -> Box<dyn StablePlayerAi> {
        Box::new(Self {
            inner: self.inner.clone_box(),
        })
    }

    fn id(&self) -> String {
        self.inner.id().to_string()
    }

    fn priority(&self) -> i32 {
        self.inner.priority()
    }

    fn matches(&self, init: &StableAiInit) -> bool {
        self.inner.matches(&PlayerAiInitContext::from_stable(init))
    }

    fn think(
        &mut self,
        ctx: &mut StableAiContext<'_>,
        base_input: Option<InputV1>,
    ) -> Option<InputV1> {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::AiThink);
        let mut legacy_ctx = PlayerAiContext::from_stable(ctx);
        let decision = self
            .inner
            .think(&mut legacy_ctx, base_input.map(Input::from_stable));
        finish_ai_decision(legacy_ctx.world(), legacy_ctx.champion_name(), base_input, decision)
    }
}

fn finish_ai_decision(world: Option<&crate::ai_world::World>, name: &str,
    base: Option<InputV1>, decision: PlayerInputDecision) -> Option<InputV1> {
    // SDK None keeps the built-in input. Use an explicit hold command to cancel.
    let hold = world.and_then(|world| world.caster())
        .map(|caster| InputV1::move_to(caster.pos.x, caster.pos.y));
    let disposition = match &decision {
        PlayerInputDecision::Pass => "pass",
        PlayerInputDecision::Block => "block",
        PlayerInputDecision::Replace(_) => "replace",
    };
    let selected = match decision {
        PlayerInputDecision::Pass => base,
        PlayerInputDecision::Block => hold,
        PlayerInputDecision::Replace(input) => Some(input.to_stable()),
    };
    let final_input = match selected {
        Some(input) if world.map_or(false, |world|
            !crate::legacy::pokemon_content::ai_input_allowed(world, name, &Input::from_stable(input))) => hold,
        _ => selected,
    };
    crate::ai_trace::decision(world, name, base, selected, final_input, disposition);
    final_input
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DraftScoreDecision {
    Pass,
    Add(f32),
    Replace(f32),
}

impl DraftScoreDecision {
    fn to_stable(self) -> StableDraftDecision {
        match self {
            Self::Pass => StableDraftDecision::Pass,
            Self::Add(score) => StableDraftDecision::Add(score),
            Self::Replace(score) => StableDraftDecision::Replace(score),
        }
    }
}

pub struct DraftScoreContext<'a> {
    pub available_champions: &'a [usize],
    pub ally_ban: &'a [usize],
    pub enemy_ban: &'a [usize],
    pub ally_pick: &'a [usize],
    pub enemy_pick: &'a [usize],
}

impl<'a> DraftScoreContext<'a> {
    fn from_stable(ctx: &'a StableDraftContext<'a>) -> Self {
        Self {
            available_champions: ctx.available_champions(),
            ally_ban: ctx.ally_bans(),
            enemy_ban: ctx.enemy_bans(),
            ally_pick: ctx.ally_picks(),
            enemy_pick: ctx.enemy_picks(),
        }
    }
}

pub trait ModDraftScoreHook: Send + Sync + 'static {
    fn id(&self) -> &str;
    fn priority(&self) -> i32 {
        0
    }
    fn score_ban(
        &self,
        _ctx: &DraftScoreContext,
        _candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        DraftScoreDecision::Pass
    }
    fn score_pick(
        &self,
        _ctx: &DraftScoreContext,
        _candidate: usize,
        _base_score: f32,
    ) -> DraftScoreDecision {
        DraftScoreDecision::Pass
    }
}

pub struct LegacyDraftHook<T> {
    pub inner: T,
}

impl<T: ModDraftScoreHook> mod_api_stable::StableDraftHook for LegacyDraftHook<T> {
    fn decide_positions(&self, ctx: &StableDraftContext<'_>) -> Option<Vec<usize>> {
        let lanes = ctx.slot_lanes();
        let picks: Option<Vec<&str>> = ctx.ally_picks().iter()
            .map(|&id| ctx.champion_name(id)).collect();
        pokemon_position_assignment(&picks?, &lanes)
    }

    fn id(&self) -> String {
        self.inner.id().to_string()
    }

    fn priority(&self) -> i32 {
        self.inner.priority()
    }

    fn score_ban(
        &self,
        ctx: &StableDraftContext<'_>,
        candidate: usize,
        base_score: f32,
    ) -> StableDraftDecision {
        with_normalized_draft(ctx, candidate, |legacy, candidate| {
            self.inner.score_ban(legacy, candidate, base_score).to_stable()
        })
    }

    fn score_pick(
        &self,
        ctx: &StableDraftContext<'_>,
        candidate: usize,
        base_score: f32,
    ) -> StableDraftDecision {
        with_normalized_draft(ctx, candidate, |legacy, candidate| {
            self.inner.score_pick(legacy, candidate, base_score).to_stable()
        })
    }
}

fn with_normalized_draft<R>(ctx: &StableDraftContext<'_>, candidate: usize,
    f: impl FnOnce(&DraftScoreContext<'_>, usize) -> R) -> R {
    let normalize = |id: usize| ctx.champion_name(id)
        .and_then(crate::legacy::pokemon_positions::legacy_score_id)
        .unwrap_or(usize::MAX);
    let map = |ids: &[usize]| ids.iter().copied().map(normalize).collect::<Vec<_>>();
    let available = map(ctx.available_champions());
    let ally_ban = map(ctx.ally_bans());
    let enemy_ban = map(ctx.enemy_bans());
    let ally_pick = map(ctx.ally_picks());
    let enemy_pick = map(ctx.enemy_picks());
    f(&DraftScoreContext { available_champions: &available, ally_ban: &ally_ban,
        enemy_ban: &enemy_ban, ally_pick: &ally_pick, enemy_pick: &enemy_pick }, normalize(candidate))
}

pub trait ModChampionInfo: Send + Sync + 'static {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn skill_icon(&self, skill_index: usize) -> (String, String);
    fn category(&self) -> ChampionCategory;
    fn tags(&self) -> Vec<ChampionTag>;
    fn stat(&self) -> EntityStat;
    fn growth(&self) -> EntityStat;
    fn attack(&self) -> Box<dyn ModAction>;
    fn skill(&self) -> Box<dyn ModAction>;
    fn skill2(&self) -> Box<dyn ModAction>;
    fn ult(&self) -> Option<Box<dyn ModAction>>;
    fn passive(&self) -> Option<Box<dyn ModPassive>>;
}

pub struct ModEffect {
    pub range: u64,
    pub growth_range: u64,
    pub start_timing: usize,
    pub casting: CastingType,
    pub target: CastingTarget,
    pub attack_type: AttackType,
    pub effect_type: Box<dyn ModEffectType>,
}

pub trait ModAction: Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn ModAction>;
    fn action_name(&self) -> &str;
    fn duration(&self) -> usize;
    fn cancelable(&self) -> bool {
        false
    }
    fn cooltime(&self, caster_stat: &EntityStat, caster_level: usize) -> usize;
    fn casting_target(&self) -> CastingTarget;
    fn effect(&self) -> Option<ModEffect>;
    fn cooltime_use_count(&self, _caster_stat: &EntityStat) -> usize {
        1
    }
    fn can_use_with_move(&self) -> bool {
        false
    }
    fn can_activate(&self, _world: &crate::ai_world::World) -> bool { true }
    fn needs_ai_neighbours(&self) -> bool { true }
    fn target_constraint(&self, _world: &crate::ai_world::World, _target: usize) -> bool { true }
    fn can_use_with_move_by(&self, _caster: &mod_api_stable::StableEntityView<'_>) -> bool {
        self.can_use_with_move()
    }
    fn description(&self) -> String {
        String::new()
    }
}

pub trait ModEffectType: Send + Sync + 'static {
    fn apply(&self, ctx: &mut GameCtx, rng_seed: u64, caster_id: usize, input: InputTarget);
    fn may_adjust_range(&self) -> bool { false }
    fn range_adjust(&self, _caster: &mod_api_stable::StableEntityView<'_>, _target: &mod_api_stable::StableEntityView<'_>) -> u64 { 0 }
    fn expected_damage(&self, _caster_stat: &EntityStat) -> (usize, usize) {
        (0, 0)
    }
    fn expected_heal(&self, _caster_stat: &EntityStat) -> usize {
        0
    }
    fn expected_shield(&self, _caster_stat: &EntityStat) -> usize {
        0
    }
    fn expected_cc_time(&self) -> Option<usize> {
        None
    }
    fn expected_buff(&self, _caster_stat: &EntityStat) -> Option<BuffState> {
        None
    }
    fn expected_move_distance(&self) -> Option<(usize, u64)> {
        None
    }
    fn expected_rush_effect(&self) -> bool {
        false
    }
    fn auto_target(&self) -> bool {
        false
    }
    fn on_caster(&self) -> bool {
        false
    }
    fn can_move(&self) -> bool {
        false
    }
    fn linear_move_speed(&self) -> Option<usize> {
        None
    }
}

pub trait ModPassive: Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn ModPassive>;
    fn on_spawn(&mut self, _ctx: &mut GameCtx, _player: usize, _entity: usize) {}
    fn on_attack(
        &mut self,
        _ctx: &mut GameCtx,
        _player: usize,
        _entity: usize,
        _target: usize,
        _damage: &mut usize,
    ) {
    }
    fn on_damaged(
        &mut self,
        _ctx: &mut GameCtx,
        _player: usize,
        _entity: usize,
        _attacker: usize,
        _damage: usize,
    ) {
    }
    fn on_kill(&mut self, _ctx: &mut GameCtx, _player: usize, _entity: usize) {}
    fn on_unit_kill(&mut self, _ctx: &mut GameCtx, _player: usize, _entity: usize, _victim: usize) {
    }
    fn on_update(&mut self, _ctx: &mut GameCtx, _rng_seed: u64, _player: usize, _entity: usize) {}
    fn on_base_attack(
        &mut self,
        _ctx: &mut GameCtx,
        _rng_seed: u64,
        _player: usize,
        _entity: usize,
    ) {
    }
    fn on_assist(&mut self, _ctx: &mut GameCtx, _player: usize, _entity: usize) {}
    fn on_dead(&mut self, _ctx: &mut GameCtx, _player: usize) {}
}

#[derive(Default)]
pub struct ModRegistration {
    champions: Vec<LegacyChampion>,
}

impl ModRegistration {
    pub fn new(_mod_id: &str) -> Self {
        Self::default()
    }

    pub fn add_champion<T>(&mut self, champion: T)
    where
        T: ModChampionInfo,
    {
        self.champions.push(LegacyChampion {
            inner: Box::new(champion),
        });
    }

    pub fn into_stable(self, stable: &mut StableMod) {
        for champion in self.champions {
            stable.add_champion(champion);
        }
    }
}

struct LegacyChampion {
    inner: Box<dyn ModChampionInfo>,
}

fn pokemon_position_assignment(picks: &[&str], lanes: &[LaneV1]) -> Option<Vec<usize>> {
    // Five players: exhaust all 120 permutations instead of greedy swaps that strand a role.
    if picks.len() != 5 || lanes.len() != 5 {
        return None;
    }
    let priors: Option<Vec<_>> = picks.iter().map(|id| pokemon_lane_prior(id)).collect();
    let priors = priors?; // Leave mixed/base rosters to the engine rather than guess their roles.
    let mut best = Vec::new();
    let mut best_score = -1;
    fn visit(priors: &[Vec<(LaneV1, usize)>], lanes: &[LaneV1], order: &mut Vec<usize>,
             best: &mut Vec<usize>, best_score: &mut i32, score: i32) {
        if order.len() == lanes.len() {
            if score > *best_score { *best_score = score; *best = order.clone(); }
            return;
        }
        for pick in 0..priors.len() {
            if order.contains(&pick) { continue; }
            let weight = priors[pick].iter().find(|(lane, _)| *lane == lanes[order.len()])
                .map(|(_, value)| *value).unwrap_or(0);
            // Coverage first, then primary over secondary. One off-role cannot be
            // compensated by promoting the other four players to their primary role.
            let fit = if weight > 0 { 1000 + weight as i32 } else { 0 };
            order.push(pick);
            visit(priors, lanes, order, best, best_score, score + fit);
            order.pop();
        }
    }
    visit(&priors, lanes, &mut Vec::new(), &mut best, &mut best_score, 0);
    Some(best)
}

fn pokemon_lane_prior(id: &str) -> Option<Vec<(LaneV1, usize)>> {
    let weights = crate::legacy::pokemon_positions::lane_weights(id)?;
    Some(
        [
            LaneV1::Top,
            LaneV1::Jungle,
            LaneV1::Mid,
            LaneV1::Bottom,
            LaneV1::Support,
        ]
        .into_iter()
        .enumerate()
        .map(|(lane, value)| {
            (value, weights[lane])
        })
        .collect(),
    )
}

impl StableChampion for LegacyChampion {
    fn lane_prior(&self) -> Option<Vec<(LaneV1, usize)>> {
        pokemon_lane_prior(self.inner.id())
    }

    fn id(&self) -> String {
        self.inner.id().to_string()
    }

    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    fn skill_icon(&self, skill_index: usize) -> (String, String) {
        let (source, tag) = self.inner.skill_icon(skill_index);
        (
            source.replace("asset/pokemon_moba/", "asset/pokemon_moba_stable/"),
            tag,
        )
    }

    fn category(&self) -> ChampionCategoryV1 {
        self.inner.category().to_stable()
    }

    fn tags(&self) -> Vec<ChampionTagV1> {
        self.inner
            .tags()
            .into_iter()
            .map(ChampionTag::to_stable)
            .collect()
    }

    fn stat(&self) -> StatV1 {
        self.inner.stat()
    }

    fn growth(&self) -> StatV1 {
        self.inner.growth()
    }

    fn attack(&self) -> Box<dyn StableAction> {
        Box::new(LegacyAction {
            inner: self.inner.attack(),
        })
    }

    fn skill(&self) -> Box<dyn StableAction> {
        Box::new(LegacyAction {
            inner: self.inner.skill(),
        })
    }

    fn skill2(&self) -> Box<dyn StableAction> {
        Box::new(LegacyAction {
            inner: self.inner.skill2(),
        })
    }

    fn ult(&self) -> Option<Box<dyn StableAction>> {
        self.inner
            .ult()
            .map(|inner| Box::new(LegacyAction { inner }) as Box<dyn StableAction>)
    }

    fn passive(&self) -> Option<Box<dyn StablePassive>> {
        self.inner
            .passive()
            .map(|inner| Box::new(LegacyPassive { inner }) as Box<dyn StablePassive>)
    }
}

struct LegacyAction {
    inner: Box<dyn ModAction>,
}

impl StableAction for LegacyAction {
    fn can_activate(&self, sim: &StableSim<'_>, caster: usize) -> bool {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Activation);
        let _scope = crate::runtime_state::Scope::enter(sim);
        let world = if self.inner.needs_ai_neighbours() { crate::ai_world::World::capture(sim, caster) }
            else { crate::ai_world::World::for_caster(sim, caster) };
        let allowed = self.inner.can_activate(&world);
        crate::ai_trace::gate(&world, self.inner.action_name(), None, allowed);
        allowed
    }
    fn target_constraint(&self, sim: &StableSim<'_>, caster: usize, target: usize) -> bool {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Target);
        let _scope = crate::runtime_state::Scope::enter(sim);
        let world = crate::ai_world::World::for_target(sim, caster, target);
        let allowed = self.inner.target_constraint(&world, target);
        crate::ai_trace::gate(&world, self.inner.action_name(), Some(target), allowed);
        allowed
    }
    fn can_use_with_move_by(&self, caster: &mod_api_stable::StableEntityView<'_>) -> bool {
        self.inner.can_use_with_move_by(caster)
    }
    fn clone_box(&self) -> Box<dyn StableAction> {
        Box::new(Self {
            inner: self.inner.clone_box(),
        })
    }

    fn action_name(&self) -> String {
        self.inner.action_name().to_string()
    }

    fn duration(&self) -> usize {
        self.inner.duration()
    }

    fn cancelable(&self) -> bool {
        self.inner.cancelable()
    }

    fn cooltime(&self, caster_stat: &StatV1, caster_level: usize) -> usize {
        self.inner.cooltime(caster_stat, caster_level)
    }

    fn casting_target(&self) -> CastingTargetV1 {
        self.inner.casting_target().to_stable()
    }

    fn effect(&self) -> Option<StableEffectSpec> {
        let effect = self.inner.effect()?;
        Some(StableEffectSpec {
            range: effect.range,
            growth_range: effect.growth_range,
            start_timing: effect.start_timing,
            casting: effect.casting.to_stable(),
            target: effect.target.to_stable(),
            attack_type: effect.attack_type.to_stable(),
            effect: Box::new(LegacyEffect {
                inner: effect.effect_type,
            }),
        })
    }

    fn cooltime_use_count(&self, caster_stat: &StatV1) -> usize {
        self.inner.cooltime_use_count(caster_stat).max(1)
    }

    fn can_use_with_move(&self) -> bool {
        self.inner.can_use_with_move()
    }

    fn description(&self) -> String {
        self.inner.description()
    }
}

struct LegacyEffect {
    inner: Box<dyn ModEffectType>,
}

impl StableEffectType for LegacyEffect {
    fn may_adjust_range(&self) -> bool { self.inner.may_adjust_range() }
    fn range_adjust(&self, caster: &mod_api_stable::StableEntityView<'_>, target: &mod_api_stable::StableEntityView<'_>) -> u64 {
        self.inner.range_adjust(caster, target)
    }
    fn apply(
        &self,
        sim: &mut StableSim<'_>,
        rng_seed: u64,
        caster_id: usize,
        input: InputTargetV1,
    ) {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Effect);
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.apply(
            &mut ctx,
            rng_seed,
            caster_id,
            InputTarget::from_stable(input),
        );
    }

    fn expected_damage(&self, caster_stat: &StatV1) -> (usize, usize) {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner.expected_damage(caster_stat)
    }

    fn expected_heal(&self, caster_stat: &StatV1) -> usize {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner.expected_heal(caster_stat)
    }

    fn expected_shield(&self, caster_stat: &StatV1) -> usize {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner.expected_shield(caster_stat)
    }

    fn expected_cc_time(&self) -> Option<usize> {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner.expected_cc_time()
    }

    fn expected_buff(&self, caster_stat: &StatV1) -> Option<BuffV1> {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner
            .expected_buff(caster_stat)
            .map(|buff| buff.to_stable())
    }

    fn expected_move_distance(&self) -> Option<(usize, u64)> {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner.expected_move_distance()
    }

    fn expected_rush_effect(&self) -> bool {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Metadata);
        self.inner.expected_rush_effect()
    }

    fn auto_target(&self) -> bool {
        self.inner.auto_target()
    }

    fn on_caster(&self) -> bool {
        self.inner.on_caster()
    }

    fn can_move(&self) -> bool {
        self.inner.can_move()
    }

    fn linear_move_speed(&self) -> Option<usize> {
        self.inner.linear_move_speed()
    }
}

struct LegacyPassive {
    inner: Box<dyn ModPassive>,
}

impl StablePassive for LegacyPassive {
    fn clone_box(&self) -> Box<dyn StablePassive> {
        Box::new(Self {
            inner: self.inner.clone_box(),
        })
    }

    fn on_spawn(&mut self, sim: &mut StableSim<'_>, player: usize, entity: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_spawn(&mut ctx, player, entity);
    }

    fn on_attack(
        &mut self,
        sim: &mut StableSim<'_>,
        player: usize,
        entity: usize,
        target: usize,
        damage: &mut usize,
    ) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner
            .on_attack(&mut ctx, player, entity, target, damage);
    }

    fn on_damaged(
        &mut self,
        sim: &mut StableSim<'_>,
        player: usize,
        entity: usize,
        attacker: usize,
        damage: usize,
    ) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner
            .on_damaged(&mut ctx, player, entity, attacker, damage);
    }

    fn on_kill(&mut self, sim: &mut StableSim<'_>, player: usize, _entity: usize, victim: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        // The preserved Pokemon callback expects the victim, not the owner.
        self.inner.on_kill(&mut ctx, player, victim);
    }

    fn on_unit_kill(
        &mut self,
        sim: &mut StableSim<'_>,
        player: usize,
        entity: usize,
        victim: usize,
    ) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_unit_kill(&mut ctx, player, entity, victim);
    }

    fn on_update(&mut self, sim: &mut StableSim<'_>, rng_seed: u64, player: usize, entity: usize) {
        let _timing = crate::performance_probe::measure(crate::performance_probe::Stage::Passive);
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_update(&mut ctx, rng_seed, player, entity);
    }

    fn on_base_attack(
        &mut self,
        sim: &mut StableSim<'_>,
        rng_seed: u64,
        player: usize,
        entity: usize,
    ) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner
            .on_base_attack(&mut ctx, rng_seed, player, entity);
    }

    fn on_assist(&mut self, sim: &mut StableSim<'_>, player: usize, entity: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_assist(&mut ctx, player, entity);
    }

    fn on_dead(&mut self, sim: &mut StableSim<'_>, player: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_dead(&mut ctx, player);
    }
}

pub struct GameCtx {
    sim: *mut StableSim<'static>,
    _scope: crate::runtime_state::Scope,
}

impl GameCtx {
    pub fn from_stable(sim: &mut StableSim<'_>) -> Self {
        Self {
            sim: sim as *mut StableSim<'_> as *mut StableSim<'static>,
            _scope: crate::runtime_state::Scope::enter(sim),
        }
    }

    fn sim(&self) -> &StableSim<'static> {
        unsafe { &*self.sim }
    }

    fn sim_mut(&self) -> &mut StableSim<'static> {
        unsafe { &mut *self.sim }
    }

    pub fn tick(&self) -> usize {
        self.sim().tick()
    }

    pub fn seed(&self) -> u64 {
        self.sim().seed()
    }

    pub fn entity_count(&self) -> usize {
        self.sim().entity_count()
    }

    pub fn entity_at(&self, index: usize) -> Option<EntityRef<'_>> {
        self.sim().entity_at(index).map(EntityRef::from_stable)
    }

    pub fn get_entity(&self, entity_id: usize) -> Option<EntityRef<'_>> {
        self.sim().get_entity(entity_id).map(EntityRef::from_stable)
    }

    // Read-only scans do not need the owned legacy snapshot's stats/buff/CC/name copies.
    // These views borrow this context; do not retain them across a simulation mutation.
    pub(crate) fn live_entity(&self, entity_id: usize) -> Option<mod_api_stable::StableEntity<'_, 'static>> {
        self.sim().get_entity(entity_id)
    }

    pub(crate) fn live_entity_at(&self, index: usize) -> Option<mod_api_stable::StableEntity<'_, 'static>> {
        self.sim().entity_at(index)
    }

    pub(crate) fn live_player(&self, player_id: usize) -> Option<mod_api_stable::StablePlayer<'_, 'static>> {
        self.sim().get_player(player_id)
    }

    pub(crate) fn player_team_position(&self, player_id: usize) -> Option<(usize, Position)> {
        self.live_player(player_id)
            .map(|player| (player.team(), player.lane().map(Position::from_stable).unwrap_or(Position::Top)))
    }

    pub fn player_count(&self) -> usize {
        self.sim().player_count()
    }

    pub fn player_at(&self, index: usize) -> Option<PlayerRef> {
        self.sim().player_at(index).map(PlayerRef::from_stable)
    }

    pub fn get_player(&self, player_id: usize) -> Option<PlayerRef> {
        self.sim().get_player(player_id).map(PlayerRef::from_stable)
    }

    pub fn add_buff(&mut self, target: usize, buff: BuffState) {
        if let BuffType::WithShield { tick, shield } = buff.duration {
            // BuffDurationV1 carries no shield amount; shields are separate engine layers.
            self.sim_mut().entity_add_shield(target, shield, tick);
        }
        self.sim_mut().add_buff(target, &buff.to_stable());
    }

    pub fn refresh_periodic_buff(&self, name: &str, target: usize, buff: BuffState) {
        debug_assert!(matches!(buff.duration, BuffType::Time { .. }));
        self.refresh_stat_buff(target, name, buff);
    }

    pub fn add_shield(&self, target: usize, amount: usize, ticks: usize) -> bool {
        self.sim_mut().entity_add_shield(target, amount, ticks)
    }

    pub fn spawn_shed_tail(&mut self, caster: usize, ticks: usize) -> Option<usize> {
        let owner = self.get_entity(caster)?;
        let pos = owner.pos();
        let mut stat = owner.stat();
        stat.hp = stat.hp.max(1);
        stat.attack = 0;
        stat.magic_power = 0;
        stat.hp_regen = 0;
        let unit = self.sim_mut().spawn_unit("pokemon_moba_stable_shed_tail", caster,
            owner.team(), pos.x, pos.y, ticks.max(1) as u64, &stat,
            &mod_api_stable::UnitAttackV1 { attack_ratio: 0, attack: 0, ..Default::default() })?;
        self.apply_cc(unit, CCState::Bind { tick: ticks as u64 });
        self.apply_cc(unit, CCState::BlockAttack { tick: ticks });
        self.apply_cc(unit, CCState::BlockSkill { tick: ticks });
        self.visual_buff(unit, "pokemon_moba_stable_heliolisk_skill2_shed_tail_decoy", ticks);
        Some(unit)
    }

    pub fn player_item_keys(&self, player_id: usize) -> Option<Vec<String>> {
        Some(self.sim().get_player(player_id)?.item_keys())
    }

    pub fn entity_item_keys(&self, entity_id: usize) -> Option<Vec<String>> {
        let sim = self.sim();
        for index in 0..sim.player_count() {
            let Some(player) = sim.player_at(index) else { continue; };
            if player.champion().map(|entity| entity.id()) == Some(entity_id) {
                return Some(player.item_keys());
            }
        }
        None
    }

    pub fn player_statistics_json(&self, player_id: usize, path: &str) -> Option<String> {
        self.sim().get_player(player_id)?.statistics_json(path)
    }

    pub fn play_view_effect(
        &self,
        name: &str,
        caster: usize,
        input: InputTarget,
        range: u64,
        radius: u64,
        ticks: u64,
    ) -> bool {
        self.sim_mut()
            .play_view_effect(name, caster, &input.to_stable(), range, radius, ticks)
    }

    pub fn play_sfx(&mut self, name: &str, caster: usize, input: InputTarget) -> bool {
        self.sim_mut().play_sfx(name, caster, &input.to_stable())
    }

    pub fn spawn_visual_projectile(&self, name: &str, spec: &mod_api_stable::ProjectileSpawnV1) -> bool {
        self.sim_mut().spawn_projectile(name, crate::native_vfx::VISUAL_HIT, spec)
    }

    pub fn spawn_projectile(&self, name: &str, hit: &str, spec: &mod_api_stable::ProjectileSpawnV1) -> bool {
        self.sim_mut().spawn_projectile(name, hit, spec)
    }

    pub fn queue_native_effect(&self, name: &str, caster: usize, input: InputTarget, ticks: usize) -> bool {
        self.sim_mut().queue_effect(name, AttackTypeV1::Skill, caster, &input.to_stable(), ticks)
    }

    pub fn visual_buff(&self, target: usize, name: &str, ticks: usize) {
        if name.len() > mod_api_stable::BUFF_NAME_CAP { return; }
        let buff = BuffV1::timed(name, ticks);
        self.sim_mut().entity_stack_buff(target, &buff, 1, true);
    }

    pub fn sync_sketch_ranges(&self, target: usize, ranges: crate::sketch_range::Ranges) {
        crate::sketch_range::sync(self.sim_mut(), target, ranges);
    }

    pub fn refresh_stat_buff(&self, target: usize, name: &str, buff: BuffState) {
        if name.len() > mod_api_stable::BUFF_NAME_CAP { return; }
        let mut desired = buff.to_stable();
        desired.set_name(name);
        let changed = self.sim().get_entity(target).map_or(false, |entity| {
            (0..entity.buff_count()).filter_map(|index| entity.buff_at(index)).any(|mut old| {
                if old.name() != name { return false; }
                old.duration_tick = desired.duration_tick;
                old != desired
            })
        });
        // A refresh only updates duration, not stats. Replace when equipment changes.
        if changed { self.sim_mut().entity_remove_buff(target, name); }
        self.sim_mut().entity_stack_buff(target, &desired, 1, true);
    }

    pub fn apply_cc(&mut self, target: usize, cc: CCState) {
        self.sim_mut().apply_cc(target, &cc.to_stable());
    }

    pub fn deal_damage(
        &mut self,
        attacker: usize,
        target: usize,
        ad: usize,
        ap: usize,
        attack_type: AttackType,
    ) {
        let allow_engine_crit = matches!(attack_type, AttackType::BaseAttack)
            && crate::combat_crit::engine_owns_hit(attacker, target);
        // Ownership applies to this hit, never to reentrant proc damage from the host.
        crate::combat_crit::with_engine_roll(false, || {
            self.deal_damage_with_crit_policy(attacker, target, ad, ap, attack_type, allow_engine_crit);
        });
    }

    fn deal_damage_with_crit_policy(&mut self, attacker: usize, target: usize,
        ad: usize, ap: usize, attack_type: AttackType, allow_engine_crit: bool) {
        let crit = if allow_engine_crit { 0 } else {
            self.sim().get_entity(attacker).map_or(0, |entity| entity.stat().crit_chance)
        };
        if crit == 0 {
            self.sim_mut().deal_damage(attacker, target, ad, ap, attack_type.to_stable());
            return;
        }
        // No public already-critted flag exists. Suppress only this synchronous engine roll,
        // preserving attack kind, mitigation, statistics, KDA and normal item callbacks.
        static NEXT_SCOPE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let name = format!("pokemon_crit_scope_{}", NEXT_SCOPE.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let mut buff = BuffV1::timed(&name, 1);
        buff.crit_chance = -(crit.min(i32::MAX as usize) as i32);
        self.sim_mut().add_buff(attacker, &buff);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.sim_mut().deal_damage(attacker, target, ad, ap, attack_type.to_stable());
        }));
        self.sim_mut().entity_remove_buff(attacker, &name);
        if let Err(payload) = result { std::panic::resume_unwind(payload); }
    }

    pub fn heal(&mut self, caster: usize, target: usize, amount: usize) {
        self.sim_mut().heal(caster, target, amount);
    }

    pub fn entity_set_hp(&self, entity_id: usize, hp: usize) -> bool {
        self.sim_mut().entity_set_hp(entity_id, hp)
    }

    pub fn entity_set_pos(&self, entity_id: usize, x: u64, y: u64) -> bool {
        let accepted = self.sim_mut().entity_set_pos(entity_id, x, y);
        crate::ai_trace::teleport(self.tick(), entity_id, x, y, accepted);
        accepted
    }

    pub fn entity_set_invisible(&self, entity_id: usize, ticks: usize) -> bool {
        self.sim_mut().entity_set_invisible(entity_id, ticks)
    }

    pub fn entity_set_base_stat(&self, entity_id: usize, stat: &StatV1) -> bool {
        self.sim_mut().entity_set_base_stat(entity_id, stat)
    }

    pub fn player_set_gold(&self, player_id: usize, gold: usize) -> bool {
        self.sim_mut().player_set_gold(player_id, gold)
    }

    pub fn player_add_gold(&self, player_id: usize, delta: i64) -> bool {
        self.sim_mut().player_add_gold(player_id, delta)
    }

    pub fn entity_remove_buff(&self, entity_id: usize, name: &str) -> usize {
        self.sim_mut().entity_remove_buff(entity_id, name)
    }

    pub fn entity_clear_cc(&self, entity_id: usize) -> usize {
        self.sim_mut().entity_clear_cc(entity_id)
    }

    pub fn is_visible(&self, team: usize, entity_id: usize) -> bool {
        self.sim().is_visible(team, entity_id)
    }

    pub fn kill_log_count(&self) -> usize {
        self.sim().kill_log_count()
    }

    pub fn kill_log_at(&self, index: usize) -> KillLogV1 {
        self.sim().kill_log_at(index).unwrap_or_default()
    }

    pub fn debug_draw_circle(&mut self, _x: u64, _y: u64, _radius: u64, _color: u32) {}

    pub fn debug_draw_line(&mut self, _x1: u64, _y1: u64, _x2: u64, _y2: u64, _color: u32) {}
}

#[derive(Clone, Debug)]
pub struct EntityRef<'a> {
    id: usize,
    stat: EntityStat,
    pos: EntityPos,
    hp: EntityHp,
    team: usize,
    level: usize,
    is_alive: bool,
    is_champion: bool,
    is_tower: bool,
    is_minion: bool,
    shield: usize,
    radius: usize,
    is_targetable: bool,
    buffs: Vec<BuffState>,
    ccs: Vec<EntityCc>,
    name: Option<String>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> EntityRef<'a> {
    fn from_stable(entity: mod_api_stable::StableEntity<'_, '_>) -> Self {
        let buffs = (0..entity.buff_count())
            .filter_map(|index| entity.buff_at(index))
            .map(BuffState::from_stable)
            .collect();
        let ccs = (0..entity.cc_count())
            .filter_map(|index| entity.cc_at(index))
            .map(EntityCc::from_stable)
            .collect();
        let (x, y) = entity.pos();
        let (current, max) = entity.hp();
        Self {
            id: entity.id(),
            stat: entity.stat(),
            pos: EntityPos { x, y },
            hp: EntityHp { current, max },
            team: entity.team(),
            level: entity.level(),
            is_alive: entity.is_alive(),
            is_champion: entity.is_champion(),
            is_tower: entity.is_tower(),
            is_minion: entity.is_minion(),
            shield: entity.shield(),
            radius: entity.radius(),
            is_targetable: entity.is_targetable(),
            buffs,
            ccs,
            name: entity.name(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn stat(&self) -> EntityStat {
        self.stat
    }
    pub fn pos(&self) -> EntityPos {
        self.pos
    }
    pub fn hp(&self) -> EntityHp {
        self.hp
    }
    pub fn team(&self) -> usize {
        self.team
    }
    pub fn level(&self) -> usize {
        self.level
    }
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
    pub fn is_champion(&self) -> bool {
        self.is_champion
    }
    pub fn is_tower(&self) -> bool {
        self.is_tower
    }
    pub fn is_minion(&self) -> bool {
        self.is_minion
    }
    pub fn shield(&self) -> usize {
        self.shield
    }
    pub fn radius(&self) -> usize {
        self.radius
    }
    pub fn is_targetable(&self) -> bool {
        self.is_targetable
    }
    pub fn buff_count(&self) -> usize {
        self.buffs.len()
    }
    pub fn buff_at(&self, index: usize) -> Option<BuffState> {
        self.buffs.get(index).cloned()
    }
    pub fn cc_count(&self) -> usize {
        self.ccs.len()
    }
    pub fn cc_at(&self, index: usize) -> EntityCc {
        self.ccs.get(index).copied().unwrap_or_default()
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCc {
    pub cc_type: u32,
    pub state: Option<CCState>,
}

impl Default for EntityCc {
    fn default() -> Self {
        Self {
            cc_type: 255,
            state: None,
        }
    }
}

impl EntityCc {
    fn from_stable(cc: CcV1) -> Self {
        Self {
            cc_type: cc.kind,
            state: Some(CCState::from_stable(cc)),
        }
    }
}

impl BuffState {
    fn from_stable(buff: BuffV1) -> Self {
        let duration = if buff.duration_kind == BuffDurationV1::Time.code() {
            BuffType::Time {
                tick: buff.duration_tick,
            }
        } else {
            BuffType::Permanent
        };
        Self {
            duration,
            attack: buff.attack,
            attack_mult: buff.attack_mult,
            magic_power: buff.magic_power,
            magic_power_mult: buff.magic_power_mult,
            defence: buff.defence,
            defence_mult: buff.defence_mult,
            hp: buff.hp,
            hp_regen: buff.hp_regen,
            magic_resistance: buff.magic_resistance,
            magic_resistance_mult: buff.magic_resistance_mult,
            vamp: buff.vamp,
            hp_mult: buff.hp_mult,
            move_speed_mult: buff.move_speed_mult,
            attack_speed_mult: buff.attack_speed_mult,
            skill_cooldown_mult: buff.skill_cooldown_mult,
            ult_cooldown_mult: buff.ult_cooldown_mult,
            radius_mult: buff.radius_mult,
            crit_chance: buff.crit_chance,
            damage_reflect: buff.damage_reflect,
            damaged_amplify: buff.damaged_amplify,
            damaged_reduce: buff.damaged_reduce,
            defence_penetration: buff.defence_penetration,
            magic_resistance_penetration: buff.magic_resistance_penetration,
            toughness: buff.toughness,
            heal_reduce: buff.heal_reduce,
            range: buff.range,
            base_attack_enemy_max_hp_damage: buff.base_attack_enemy_max_hp_damage,
            self_max_hp_damage: buff.self_max_hp_damage,
            skill_enemy_max_hp_damage: buff.skill_enemy_max_hp_damage,
            dot_amplify: buff.dot_amplify,
            base_attack_damaged_reduce: buff.base_attack_damaged_reduce,
            skill_damaged_reduce: buff.skill_damaged_reduce,
            cc_immune: buff.cc_immune,
            undying: buff.undying,
            ignore_wall: buff.ignore_wall,
        }
    }
}

impl CCState {
    fn from_stable(cc: CcV1) -> Self {
        if cc.kind == CcKindV1::Airborne.code() {
            Self::Airborne { tick: cc.tick }
        } else if cc.kind == CcKindV1::Bind.code() {
            Self::Bind { tick: cc.tick }
        } else if cc.kind == CcKindV1::BlockAttack.code() {
            Self::BlockAttack {
                tick: cc.tick as usize,
            }
        } else if cc.kind == CcKindV1::BlockSkill.code() {
            Self::BlockSkill {
                tick: cc.tick as usize,
            }
        } else if cc.kind == CcKindV1::BlockMoveSkill.code() {
            Self::BlockMoveSkill {
                tick: cc.tick as usize,
            }
        } else if cc.kind == CcKindV1::ForceMove.code() {
            Self::ForceMove {
                tick: cc.tick,
                dx: cc.dx,
                dy: cc.dy,
                speed: cc.speed,
            }
        } else if cc.kind == CcKindV1::Taunt.code() {
            Self::Taunt {
                tick: cc.tick,
                target: cc.target,
            }
        } else if cc.kind == CcKindV1::Fear.code() {
            Self::Fear {
                tick: cc.tick,
                dx: cc.dx,
                dy: cc.dy,
            }
        } else if cc.kind == CcKindV1::Charm.code() {
            Self::Charm {
                tick: cc.tick,
                dx: cc.dx,
                dy: cc.dy,
            }
        } else if cc.kind == CcKindV1::Animation.code() {
            Self::Animation { tick: cc.tick }
        } else {
            Self::Stun { tick: cc.tick }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerStatistics {
    pub gold: usize,
    pub deal: usize,
    pub tank: usize,
    pub heal: usize,
    pub kill: usize,
    pub assist: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerInfo {
    pub id: usize,
    pub team: usize,
    pub position: Position,
    pub gold: usize,
    pub statistics: PlayerStatistics,
    pub last_statistics: PlayerStatistics,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerState {
    pub info: PlayerInfo,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerHandle {
    ptr: *mut std::ffi::c_void,
}

impl PlayerHandle {
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn is_null(self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(self) -> *mut std::ffi::c_void {
        self.ptr
    }
}

#[derive(Clone, Debug)]
pub struct PlayerRef {
    id: usize,
    team: usize,
    level: usize,
    gold: usize,
    kills: usize,
    deaths: usize,
    assists: usize,
    cs: usize,
    position: Position,
    champion: Option<EntityRef<'static>>,
}

impl PlayerRef {
    fn from_stable(player: mod_api_stable::StablePlayer<'_, '_>) -> Self {
        let position = match player.lane() {
            Some(mod_api_stable::LaneV1::Jungle) => Position::Jungle,
            Some(mod_api_stable::LaneV1::Mid) => Position::Mid,
            Some(mod_api_stable::LaneV1::Bottom) => Position::Bottom,
            Some(mod_api_stable::LaneV1::Support) => Position::Support,
            _ => Position::Top,
        };
        Self {
            id: player.id(),
            team: player.team(),
            level: player.level(),
            gold: player.gold(),
            kills: player.kills(),
            deaths: player.deaths(),
            assists: player.assists(),
            cs: player.cs(),
            position,
            champion: player.champion().map(EntityRef::from_stable),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn team(&self) -> usize {
        self.team
    }
    pub fn level(&self) -> usize {
        self.level
    }
    pub fn gold(&self) -> usize {
        self.gold
    }
    pub fn kills(&self) -> usize {
        self.kills
    }
    pub fn deaths(&self) -> usize {
        self.deaths
    }
    pub fn assists(&self) -> usize {
        self.assists
    }
    pub fn cs(&self) -> usize {
        self.cs
    }
    pub fn position(&self) -> Position {
        self.position
    }
    pub fn champion(&self) -> Option<EntityRef<'static>> {
        self.champion.clone()
    }
    pub fn is_alive(&self) -> bool {
        self.champion.as_ref().map_or(false, EntityRef::is_alive)
    }
    pub fn handle(&self) -> PlayerHandle {
        PlayerHandle::null()
    }
}

#[cfg(test)]
mod sdk_upgrade_tests {
    use super::*;
    use mod_api_stable::{EntityHandleV1, PlayerHandleV1, SimCtxV1, SimVtableV1};
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::sync::{Arc, Mutex};

    #[test]
    fn post_draft_assignment_respects_lane_slots_not_pick_order() {
        let lanes = [LaneV1::Top, LaneV1::Jungle, LaneV1::Mid, LaneV1::Bottom, LaneV1::Support];
        let picks = ["pokemon_moba_blissey", "pokemon_moba_pikachu", "pokemon_moba_gyarados",
            "pokemon_moba_charizard", "pokemon_moba_snorlax"];
        let assignment = pokemon_position_assignment(&picks, &lanes).unwrap();
        let mut sorted = assignment.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
        assert_eq!(picks[assignment[4]], "pokemon_moba_blissey");
        assert_eq!(pokemon_position_assignment(&picks[..4], &lanes), None);
        assert_eq!(pokemon_position_assignment(&["base_unknown"; 5], &lanes), None);
        let mut reversed = lanes;
        reversed.reverse();
        let other = pokemon_position_assignment(&picks, &reversed).unwrap();
        assert_eq!(picks[other[0]], "pokemon_moba_blissey");
    }

    #[test]
    fn roster_keeps_action_tags_charges_and_lane_preferences() {
        let mut registration = ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        assert_eq!(registration.champions.len(), 103);
        for champion in registration.champions {
            let stats = champion.stat();
            for (tag, action) in [
                ("attack", Some(champion.attack())),
                ("skill", Some(champion.skill())),
                ("skill2", Some(champion.skill2())),
                ("ult", champion.ult()),
            ] {
                let action = action.expect("roster must keep all four moves");
                assert_eq!(action.action_name(), tag, "{}", champion.id());
                assert!(
                    action.cooltime_use_count(&stats) > 0,
                    "{} {tag}",
                    champion.id()
                );
            }
            let prior = champion.lane_prior().expect("missing lane preferences");
            assert_eq!(prior.len(), 5);
            assert_eq!(prior.iter().map(|(_, weight)| weight).sum::<usize>(), 100);
            let info = crate::legacy::pokemon_positions::info_for_champion_id(&champion.id()).unwrap();
            for (lane, (_, weight)) in prior.iter().enumerate() {
                assert_eq!(*weight > 0, info.positions.contains(&lane));
            }
        }
        let blissey = pokemon_lane_prior("pokemon_moba_blissey").unwrap();
        assert_eq!(blissey[1].1, 0, "Blissey must not seed as jungle");
        use crate::legacy::pokemon_positions::lane_weights;
        assert_eq!(lane_weights("pokemon_moba_blissey"), Some([0, 0, 0, 0, 100]));
        for (id, expected) in [("pokemon_moba_feraligatr", vec![50, 50]),
            ("pokemon_moba_skeledirge", vec![25, 25, 50])] {
            let mut weights: Vec<_> = lane_weights(id).unwrap().into_iter().filter(|w| *w > 0).collect();
            weights.sort_unstable();
            assert_eq!(weights, expected);
        }
    }

    #[derive(Default)]
    struct Calls {
        world: bool,
        entity_enumerations: std::cell::Cell<usize>,
        player_enumerations: std::cell::Cell<usize>,
        stat_reads: std::cell::Cell<usize>,
        buff_reads: std::cell::Cell<usize>,
        towers: std::collections::HashSet<usize>,
        tick: usize,
        seed: u64,
        hp_current: Option<usize>,
        heals: Vec<(usize, usize, usize)>,
        clear_cc: Vec<usize>,
        teleports: Vec<(usize, u64, u64)>,
        invisible: Vec<(usize, usize)>,
        ally_nine: bool,
        hidden: bool,
        positions: std::collections::HashMap<usize, (u64, u64)>,
        dead: std::collections::HashSet<usize>,
        lethal_self_damage: bool,
        enemy_champions: bool,
        spawned: Vec<(String, usize, u64, u64, u64, StatV1)>,
        cc: Vec<(usize, CcV1)>,
        crit: usize,
        active_buffs: std::collections::HashMap<(usize, String), Vec<BuffV1>>,
        crit_at_damage: Vec<(usize, bool)>,
        gold: Vec<(usize, i64)>,
        damage: Vec<(usize, usize, usize, usize, u32)>,
        shields: Vec<(usize, usize, usize)>,
        buffs: usize,
        visuals: Vec<(String, usize, InputTargetV1, u64, u64, u64)>,
        reject_visuals: bool,
        projectiles: Vec<(String, String, mod_api_stable::ProjectileSpawnV1)>,
        queued: Vec<(String, usize, InputTargetV1, usize)>,
        visual_buffs: Vec<(usize, String, usize, u32)>,
    }

    unsafe extern "C" fn damage(
        state: *mut c_void,
        attacker: usize,
        target: usize,
        ad: usize,
        ap: usize,
        kind: u32,
    ) {
        let calls = &mut *(state as *mut Calls);
        calls.crit_at_damage.push((current_crit(calls, attacker), crate::combat_crit::engine_owns_roll()));
        if calls.lethal_self_damage && attacker == target { calls.dead.insert(target); }
        (*(state as *mut Calls))
            .damage
            .push((attacker, target, ad, ap, kind));
    }

    unsafe extern "C" fn shield(
        state: *mut c_void,
        target: EntityHandleV1,
        amount: usize,
        ticks: usize,
    ) -> bool {
        (*(state as *mut Calls))
            .shields
            .push((target.id().unwrap(), amount, ticks));
        true
    }

    unsafe extern "C" fn buff(state: *mut c_void, target: usize, buff: *const BuffV1) {
        let calls = &mut *(state as *mut Calls);
        calls.buffs += 1;
        calls.active_buffs.entry((target, (*buff).name().to_owned())).or_default().push(*buff);
    }

    fn current_crit(calls: &Calls, target: usize) -> usize {
        (calls.crit as i64 + calls.active_buffs.iter().filter(|((id, _), _)| *id == target)
            .flat_map(|(_, buffs)| buffs).map(|buff| buff.crit_chance as i64).sum::<i64>()).max(0) as usize
    }
    unsafe extern "C" fn remove_buff(state: *mut c_void, target: EntityHandleV1,
        name: *const u8, len: usize) -> usize {
        let name = std::str::from_utf8(std::slice::from_raw_parts(name, len)).unwrap();
        (*(state as *mut Calls)).active_buffs.remove(&(target.id().unwrap(), name.to_owned())).map_or(0, |buffs| buffs.len())
    }
    unsafe extern "C" fn buff_count(state: *const c_void, target: EntityHandleV1) -> usize {
        let reads = &(*(state as *const Calls)).buff_reads;
        reads.set(reads.get() + 1);
        (*(state as *const Calls)).active_buffs.iter().filter(|((id, _), _)| Some(*id) == target.id()).map(|(_, buffs)| buffs.len()).sum()
    }
    unsafe extern "C" fn buff_at(state: *const c_void, target: EntityHandleV1, index: usize, out: *mut BuffV1) -> bool {
        if let Some(buff) = (*(state as *const Calls)).active_buffs.iter().filter(|((id, _), _)| Some(*id) == target.id()).flat_map(|(_, buffs)| buffs).nth(index) {
            *out = *buff; true
        } else { false }
    }
    unsafe extern "C" fn valid(state: *const c_void, entity: EntityHandleV1) -> bool {
        (*(state as *const Calls)).world && (matches!(entity.id(), Some(7..=9))
            || (entity.id() == Some(10) && !(*(state as *const Calls)).spawned.is_empty()))
    }
    unsafe extern "C" fn alive(state: *const c_void, entity: EntityHandleV1) -> bool {
        valid(state, entity) && !(*(state as *const Calls)).dead.contains(&entity.id().unwrap())
    }
    unsafe extern "C" fn cc(state: *mut c_void, target: usize, value: *const CcV1) {
        (*(state as *mut Calls)).cc.push((target,*value));
    }
    unsafe extern "C" fn spawn(state: *mut c_void, name: *const u8, len: usize, caster: usize,
        _: usize, x: u64, y: u64, ticks: u64, stat: *const StatV1, _: *const mod_api_stable::UnitAttackV1) -> usize {
        let calls=&mut *(state as *mut Calls);
        calls.spawned.push((std::str::from_utf8(std::slice::from_raw_parts(name,len)).unwrap().to_owned(),caster,x,y,ticks,*stat));
        calls.positions.insert(10,(x,y));
        10
    }
    unsafe extern "C" fn entity_count(state: *const c_void) -> usize {
        if (*(state as *const Calls)).world { 3 } else { 0 }
    }
    unsafe extern "C" fn entity_at(state: *const c_void, index: usize) -> EntityHandleV1 {
        let count = &(*(state as *const Calls)).entity_enumerations;
        count.set(count.get() + 1);
        EntityHandleV1::from_id(7 + index)
    }
    unsafe extern "C" fn stat(state: *const c_void, entity: EntityHandleV1, out: *mut StatV1) -> bool {
        let reads = &(*(state as *const Calls)).stat_reads;
        reads.set(reads.get() + 1);
        *out = StatV1 { attack: 100, magic_power: 100, hp: 10000, move_speed: 5000,
            crit_chance: current_crit(&*(state as *const Calls), entity.id().unwrap()), ..Default::default() };
        true
    }
    unsafe extern "C" fn pos(state: *const c_void, entity: EntityHandleV1, x: *mut u64, y: *mut u64) -> bool {
        let id = entity.id().unwrap();
        (*x, *y) = (*(state as *const Calls)).positions.get(&id).copied()
            .unwrap_or((100000 + (id - 7) as u64 * 20000, 100000));
        true
    }
    unsafe extern "C" fn hp(state: *const c_void, _: EntityHandleV1, current: *mut usize, max: *mut usize) -> bool {
        *current = (*(state as *const Calls)).hp_current.unwrap_or(10000); *max = 10000; true
    }
    unsafe extern "C" fn team(state: *const c_void, entity: EntityHandleV1) -> usize {
        (!(entity.id() == Some(7) || ((*(state as *const Calls)).ally_nine && entity.id() == Some(9)))) as usize
    }
    unsafe extern "C" fn champion(state: *const c_void, entity: EntityHandleV1) -> bool { entity.id() == Some(7) || ((*(state as *const Calls)).ally_nine && entity.id() == Some(9)) || ((*(state as *const Calls)).enemy_champions && entity.id() == Some(8)) }
    unsafe extern "C" fn minion(_: *const c_void, entity: EntityHandleV1) -> bool { entity.id() != Some(7) }
    unsafe extern "C" fn tower(state: *const c_void, entity: EntityHandleV1) -> bool {
        (*(state as *const Calls)).towers.contains(&entity.id().unwrap())
    }
    unsafe extern "C" fn player_valid(state: *const c_void, player: PlayerHandleV1) -> bool {
        (*(state as *const Calls)).world && player.id() == Some(1)
    }
    unsafe extern "C" fn player_champion(_: *const c_void, _: PlayerHandleV1) -> EntityHandleV1 { EntityHandleV1::from_id(7) }
    unsafe extern "C" fn player_count(_: *const c_void) -> usize { 1 }
    unsafe extern "C" fn player_at(state: *const c_void, _: usize) -> PlayerHandleV1 {
        let count = &(*(state as *const Calls)).player_enumerations;
        count.set(count.get() + 1);
        PlayerHandleV1::from_id(1)
    }
    unsafe extern "C" fn visible(state: *const c_void, _: usize, _: usize) -> bool { !(*(state as *const Calls)).hidden }
    unsafe extern "C" fn gold(state: *mut c_void, player: PlayerHandleV1, delta: i64) -> bool {
        (*(state as *mut Calls)).gold.push((player.id().unwrap(), delta)); true
    }
    unsafe extern "C" fn tick(state: *const c_void) -> usize { (*(state as *const Calls)).tick }
    unsafe extern "C" fn seed(state: *const c_void) -> u64 { (*(state as *const Calls)).seed }
    unsafe extern "C" fn heal(state: *mut c_void, caster: usize, target: usize, amount: usize) {
        let calls = &mut *(state as *mut Calls);
        calls.heals.push((caster, target, amount));
        let reduction = calls.active_buffs.iter().filter(|((id, _), _)| *id == target)
            .flat_map(|(_, buffs)| buffs).map(|buff| buff.heal_reduce).sum::<usize>().min(100);
        calls.hp_current = Some(calls.hp_current.unwrap_or(10000).saturating_add(amount * (100 - reduction) / 100).min(10000));
    }
    unsafe extern "C" fn clear_cc(state: *mut c_void, target: EntityHandleV1) -> usize {
        (*(state as *mut Calls)).clear_cc.push(target.id().unwrap()); 1
    }
    unsafe extern "C" fn set_pos(state: *mut c_void, target: EntityHandleV1, x: u64, y: u64) -> bool {
        let calls = &mut *(state as *mut Calls);
        calls.teleports.push((target.id().unwrap(), x, y));
        calls.positions.insert(target.id().unwrap(), (x,y)); true
    }
    unsafe extern "C" fn invisible(state: *mut c_void, target: EntityHandleV1, ticks: usize) -> bool {
        (*(state as *mut Calls)).invisible.push((target.id().unwrap(), ticks)); true
    }

    unsafe extern "C" fn visual(state: *mut c_void, name: *const u8, len: usize,
        caster: usize, input: *const InputTargetV1, range: u64, radius: u64, ticks: u64) -> bool {
        let name = std::str::from_utf8(std::slice::from_raw_parts(name, len)).unwrap().to_owned();
        (*(state as *mut Calls)).visuals.push((name, caster, *input, range, radius, ticks));
        !(*(state as *mut Calls)).reject_visuals
    }

    unsafe extern "C" fn projectile(state: *mut c_void, name: *const u8, len: usize,
        effect: *const u8, effect_len: usize, spec: *const mod_api_stable::ProjectileSpawnV1) -> bool {
        let name = std::str::from_utf8(std::slice::from_raw_parts(name, len)).unwrap().to_owned();
        let effect = std::str::from_utf8(std::slice::from_raw_parts(effect, effect_len)).unwrap().to_owned();
        (*(state as *mut Calls)).projectiles.push((name, effect, *spec));
        true
    }

    unsafe extern "C" fn queue(state: *mut c_void, name: *const u8, len: usize,
        _: u32, caster: usize, input: *const InputTargetV1, ticks: usize) -> bool {
        let name = std::str::from_utf8(std::slice::from_raw_parts(name, len)).unwrap().to_owned();
        (*(state as *mut Calls)).queued.push((name, caster, *input, ticks));
        true
    }

    unsafe extern "C" fn visual_buff(state: *mut c_void, target: EntityHandleV1,
        buff: *const BuffV1, max: usize, refresh: u32) -> usize {
        (*(state as *mut Calls)).visual_buffs.push((target.id().unwrap(), (*buff).name().to_owned(), max, refresh));
        let buffs = (*(state as *mut Calls)).active_buffs.entry((target.id().unwrap(), (*buff).name().to_owned())).or_default();
        if buffs.len() < max { buffs.push(*buff); }
        if refresh != 0 { for old in buffs.iter_mut() { old.duration_tick = (*buff).duration_tick; } }
        buffs.len()
    }

    #[test]
    fn lightweight_reads_match_owned_snapshots_without_copying_combat_payloads() {
        let mut expected = Vec::new();
        let mut expected_player = None;
        let calls = with_world(Calls { world: true, towers: [9].into(), dead: [8].into(), ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            for index in 0..ctx.entity_count() {
                let unit = ctx.entity_at(index).unwrap();
                expected.push((unit.id(), unit.team(), unit.pos(), unit.hp().current, unit.hp().max,
                    unit.is_alive(), unit.is_champion(), unit.is_tower()));
            }
            expected_player = ctx.get_player(1).map(|p| (p.deaths(), p.team(), p.position()));
        });
        assert!(calls.stat_reads.get() >= 4);
        assert!(calls.buff_reads.get() >= 4);
        calls.stat_reads.set(0);
        calls.buff_reads.set(0);
        let calls = with_world(calls, |sim| {
            let ctx = GameCtx::from_stable(sim);
            let actual: Vec<_> = (0..ctx.entity_count()).map(|index| {
                let unit = ctx.live_entity_at(index).unwrap();
                let (x, y) = unit.pos();
                let (hp, max) = unit.hp();
                (unit.id(), unit.team(), EntityPos { x, y }, hp, max,
                    unit.is_alive(), unit.is_champion(), unit.is_tower())
            }).collect();
            assert_eq!(actual, expected);
            assert_eq!(ctx.live_player(1).map(|p| {
                let (team, position) = ctx.player_team_position(1).unwrap();
                (p.deaths(), team, position)
            }), expected_player);
            assert!(ctx.live_entity(999).is_none());
            assert!(ctx.live_player(999).is_none());
        });
        assert_eq!(calls.stat_reads.get(), 0);
        assert_eq!(calls.buff_reads.get(), 0);
    }

    fn gas_reference(ctx: &GameCtx, id: usize, champion: &str) -> bool {
        use crate::legacy::pokemon_status as status;
        if matches!(champion, "pokemon_moba_clawitzer" | "pokemon_moba_comfey") { return false; }
        let Some(unit) = ctx.get_entity(id) else { return false; };
        if !unit.is_alive() || unit.is_tower() { return false; }
        let radius = 42_000u64;
        (0..ctx.entity_count()).filter_map(|i| ctx.entity_at(i)).any(|source| {
            source.team() != unit.team() && source.is_alive()
                && (status::champion_id_for_entity(source.id()) == Some("pokemon_moba_weezing")
                    || status::receiver_has_copied(source.id(), "pokemon_moba_weezing"))
                && crate::ai_world::distance_sq(source.pos(), unit.pos()) <= u128::from(radius).pow(2)
        })
    }

    #[test]
    fn lightweight_ai_snapshots_preserve_order_expiry_death_and_respawn() {
        let mut calls = Calls { world: true, ..Default::default() };
        for tick in [1, 2, 3, 4, 200, 201] {
            calls.tick = tick;
            calls.positions.insert(7, (100000 + tick as u64 * 10, 100000));
            calls.hp_current = Some(10000 - tick);
            calls.towers = if tick < 4 { [9].into() } else { [8].into() };
            calls.dead = if tick == 3 { [7, 8, 9].into() } else { Default::default() };
            calls = with_world(calls, |sim| {
                crate::legacy::pokemon_content::verify_lightweight_snapshot_refresh(&GameCtx::from_stable(sim));
            });
        }
    }

    #[test]
    fn gas_without_sources_skips_all_entity_snapshots_and_enumeration() {
        use crate::legacy::pokemon_status as status;
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            status::register_entity_champion(&ctx, 7, "pokemon_moba_pikachu");
            assert!(!status::passive_suppressed_by_nullifying_gas(&ctx, 7, "pokemon_moba_pikachu"));
        });
        assert_eq!(calls.entity_enumerations.get(), 0);
        assert_eq!(calls.stat_reads.get(), 0);
        assert_eq!(calls.buff_reads.get(), 0);
    }

    #[test]
    fn gas_matches_original_across_copies_deaths_movement_and_match_isolation() {
        use crate::legacy::pokemon_status as status;
        let check = |ctx: &GameCtx| {
            for id in [7, 8, 9, 999] {
                for name in ["pokemon_moba_pikachu", "pokemon_moba_comfey", "pokemon_moba_clawitzer"] {
                    assert_eq!(status::passive_suppressed_by_nullifying_gas(ctx, id, name),
                        gas_reference(ctx, id, name), "entity={id} champion={name}");
                }
            }
        };
        let mut calls = with_world(Calls { world: true, ally_nine: true, enemy_champions: true, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            check(&ctx);
            status::register_entity_champion(&ctx, 8, "pokemon_moba_weezing");
            check(&ctx);
            status::register_receiver(9);
            status::copy_receiver_passive(&mut ctx, 9, 8);
            status::register_entity_champion(&ctx, 8, "pokemon_moba_pikachu");
            assert!(status::passive_suppressed_by_nullifying_gas(&ctx, 8, "pokemon_moba_pikachu"));
            check(&ctx);
        });
        for (position, dead, tower) in [((200000, 100000), false, false),
            ((161999, 100000), false, false), ((162000, 100000), false, false), ((162001, 100000), false, false),
            ((120000, 100000), true, false),
            ((120000, 100000), false, true), ((120000, 100000), false, false)] {
            calls.positions.insert(9, position);
            calls.dead = if dead { [9].into() } else { Default::default() };
            calls.towers = if tower { [8].into() } else { Default::default() };
            calls = with_world(calls, |sim| check(&GameCtx::from_stable(sim)));
        }
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            assert!(!status::passive_suppressed_by_nullifying_gas(&GameCtx::from_stable(sim), 7, "pokemon_moba_pikachu"));
        });
    }

    fn with_sim(f: impl FnOnce(&mut StableSim<'_>)) -> Calls {
        with_world(Calls::default(), f)
    }

    fn with_world(mut calls: Calls, f: impl FnOnce(&mut StableSim<'_>)) -> Calls {
        static NEXT_SEED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        if calls.seed == 0 { calls.seed = NEXT_SEED.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
        // A zeroed table contains a zero size and null optional function pointers.
        let mut table: SimVtableV1 = unsafe { std::mem::zeroed() };
        table.size = size_of::<SimVtableV1>();
        table.deal_damage = Some(damage);
        table.entity_add_shield = Some(shield);
        table.apply_cc = Some(cc);
        table.spawn_unit = Some(spawn);
        table.add_buff = Some(buff);
        table.play_view_effect = Some(visual);
        table.spawn_projectile = Some(projectile);
        table.queue_effect = Some(queue);
        table.entity_stack_buff = Some(visual_buff);
        table.tick = Some(tick);
        table.seed = Some(seed);
        table.heal = Some(heal);
        table.entity_clear_cc = Some(clear_cc);
        table.entity_set_pos = Some(set_pos);
        table.entity_set_invisible = Some(invisible);
        table.entity_is_valid = Some(valid);
        table.entity_is_alive = Some(alive);
        table.entity_is_targetable = Some(valid);
        table.entity_count = Some(entity_count);
        table.entity_at = Some(entity_at);
        table.entity_stat = Some(stat);
        table.entity_buff_count = Some(buff_count);
        table.entity_buff_at = Some(buff_at);
        table.entity_pos = Some(pos);
        table.entity_hp = Some(hp);
        table.entity_team = Some(team);
        table.entity_is_champion = Some(champion);
        table.entity_is_tower = Some(tower);
        table.entity_is_minion = Some(minion);
        table.player_is_valid = Some(player_valid);
        table.player_champion = Some(player_champion);
        table.player_count = Some(player_count);
        table.player_at = Some(player_at);
        table.is_visible = Some(visible);
        table.player_add_gold = Some(gold);
        table.entity_remove_buff = Some(remove_buff);
        let mut raw = SimCtxV1 {
            size: size_of::<SimCtxV1>(),
            sim: &table,
            frame: std::ptr::null(),
            state: &mut calls as *mut Calls as *mut c_void,
        };
        let mut sim = unsafe { StableSim::from_raw(&mut raw) }.unwrap();
        let _scope = crate::runtime_state::Scope::enter(&sim);
        f(&mut sim);
        calls
    }

    #[test]
    fn custom_crits_suppress_only_the_engine_roll_and_restore_stats() {
        let calls = with_world(Calls { world: true, crit: 100, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            ctx.deal_damage(7, 8, 200, 0, AttackType::BaseAttack);
            assert_eq!(ctx.get_entity(7).unwrap().stat().crit_chance, 100);
            crate::combat_crit::with_engine_roll(true, || ctx.deal_damage(7, 8, 100, 0, AttackType::BaseAttack));
            assert_eq!(ctx.get_entity(7).unwrap().stat().crit_chance, 100);
        });
        assert_eq!(calls.crit_at_damage, vec![(0, false), (100, false)]);
        assert_eq!(calls.buffs, 1);
        assert!(calls.active_buffs.is_empty());
        assert_eq!(calls.damage.len(), 2);
    }

    #[test]
    fn independent_missiles_resolve_one_hit_each_without_relaunching_the_volley() {
        use crate::legacy::pokemon_content::champion_by_id;
        for (id,slot,count) in [("pokemon_moba_blastoise",0,2),("pokemon_moba_octillery",0,3),
            ("pokemon_moba_dragapult",1,2),("pokemon_moba_jolteon",0,3),("pokemon_moba_cloyster",1,4)] {
            let champion=champion_by_id(id).unwrap();
            let action=if slot==0 {champion.attack} else {champion.skill};
            let mut calls=with_world(Calls{world:true,..Default::default()},|sim|{
                assert!(crate::projectiles::launch(&GameCtx::from_stable(sim),champion,action,7,InputTarget::Target{target_id:8}));
            });
            assert!(calls.damage.is_empty());
            assert_eq!(calls.queued.len(),count-1);
            for index in 0..count {
                calls.tick=10+index*5;
                calls=with_world(calls,|sim|crate::projectiles::test_missile(sim,champion,action,index,false,InputTargetV1::target(8)));
                assert_eq!(calls.damage.len(),index+1,"{id}");
                assert_eq!(calls.queued.len(),count-1,"impact must not enqueue another volley");
            }
        }
    }

    #[test]
    fn bouncy_bubble_lands_in_three_nonoverlapping_circles_in_sequence() {
        use crate::legacy::pokemon_content::champion_by_id;
        let champion=champion_by_id("pokemon_moba_octillery").unwrap();
        let mut initial=Calls{world:true,..Default::default()};
        initial.positions.insert(9,(156000,100000));
        let mut calls=with_world(initial,|sim|{
            assert!(crate::projectiles::launch_bounces(&GameCtx::from_stable(sim),champion,champion.skill2,7,InputTarget::Pos{x:120000,y:100000}));
        });
        assert!(calls.damage.is_empty());
        for tick in 1..=36 {
            calls.tick=tick;
            if tick==25 {calls.positions.insert(9,(192000,100000));}
            calls=with_world(calls,|sim|crate::projectiles::update(&mut GameCtx::from_stable(sim),42));
            assert_eq!(calls.damage.len(),tick/12,"tick {tick}");
        }
        assert_eq!(calls.projectiles.len(),3);
        assert_eq!(calls.visuals.iter().map(|v|(v.2.x,v.2.y)).collect::<Vec<_>>(),
            vec![(120000,100000),(156000,100000),(192000,100000)]);
        assert!(calls.visuals.last().unwrap().0.ends_with("bouncy_bubble_path"));
    }

    #[test]
    fn flip_turn_and_up_tempo_wait_for_real_contact() {
        use crate::legacy::pokemon_content::champion_by_id;
        for id in ["pokemon_moba_starmie","pokemon_moba_quaquaval"] {
            let champion=champion_by_id(id).unwrap();
            let action=if id.ends_with("starmie") {champion.skill} else {champion.skill2};
            let mut initial=Calls{world:true,enemy_champions:true,..Default::default()};
            initial.positions.insert(8,(140000,100000));
            initial.positions.insert(9,(600000,600000));
            let mut calls=with_world(initial,|sim|assert!(crate::motion::launch(&mut GameCtx::from_stable(sim),champion,action,7,InputTarget::Target{target_id:8})));
            assert!(calls.damage.is_empty());
            for (tick,x) in [(1,110000),(2,120000),(3,130000),(4,140000)] {
                calls.tick=tick;calls.positions.insert(7,(x,100000));
                calls=with_world(calls,|sim|crate::motion::update(&mut GameCtx::from_stable(sim),42));
                if tick<3 {assert!(calls.damage.is_empty(),"{id}");}
            }
            assert_eq!(calls.damage.len(),1,"{id}");
        }
    }

    #[test]
    fn rescue_does_not_pull_the_ally_until_ribombee_arrives() {
        use crate::legacy::pokemon_content::champion_by_id;
        let champion=champion_by_id("pokemon_moba_ribombee").unwrap();
        let mut initial=Calls{world:true,ally_nine:true,..Default::default()};
        initial.positions.insert(9,(130000,100000));
        let mut calls=with_world(initial,|sim|assert!(crate::motion::launch(&mut GameCtx::from_stable(sim),champion,champion.skill2,7,InputTarget::Target{target_id:9})));
        assert!(!calls.cc.iter().any(|(id,_)|*id==9));
        for tick in 1..=15 {
            calls.tick=tick;calls.positions.insert(7,(100000+1667*tick as u64,100000));
            calls=with_world(calls,|sim|crate::motion::update(&mut GameCtx::from_stable(sim),42));
            if tick<15 {assert!(!calls.cc.iter().any(|(id,_)|*id==9));}
        }
        assert!(calls.cc.iter().any(|(id,cc)|*id==9 && cc.kind==CcKindV1::ForceMove.code()));
        assert!(calls.damage.is_empty());
    }

    #[test]
    fn play_rough_and_razor_shell_do_not_damage_untraveled_dash_routes() {
        let mut registration=ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        for (id,target_x,steps) in [
            ("pokemon_moba_dedenne",140000,vec![110000,120000,130000,140000]),
            ("pokemon_moba_drednaw",65000,vec![91000,82000,73000,64000]),
        ] {
            let champion=registration.champions.iter().find(|c|c.id()==id).unwrap();
            let action=if id.ends_with("dedenne"){champion.skill()}else{champion.ult().unwrap()};
            let mut initial=Calls{world:true,enemy_champions:true,..Default::default()};
            initial.positions.insert(8,(target_x,100000));
            initial.positions.insert(9,(600000,600000));
            let mut calls=with_world(initial,|sim|action.effect().unwrap().effect.apply(sim,42,7,InputTargetV1::pos(160000,100000)));
            assert!(calls.damage.is_empty(),"{id} hit before moving");
            for (i,x) in steps.into_iter().enumerate() {
                calls.tick=i+1;calls.positions.insert(7,(x,100000));
                calls=with_world(calls,|sim|crate::motion::update(&mut GameCtx::from_stable(sim),42));
            }
            assert_eq!(calls.damage.len(),1,"{id}");
        }
    }

    #[test]
    fn colorful_whip_has_three_timed_pulses_not_three_instant_hits() {
        use crate::legacy::pokemon_content as content;
        let data=content::champion_by_id("pokemon_moba_kecleon").unwrap();
        let mut registration=ModRegistration::new("pokemon_moba_stable");
        content::register_champions(&mut registration);
        let champion=registration.champions.iter().find(|c|c.id()==data.id).unwrap();
        let mut initial=Calls{world:true,..Default::default()};
        initial.positions.insert(9,(600000,600000));
        let mut calls=with_world(initial,|sim|champion.ult().unwrap().effect().unwrap().effect.apply(sim,42,7,InputTargetV1::pos(140000,100000)));
        assert_eq!(calls.damage.len(),1);
        assert_eq!(calls.queued.iter().map(|q|q.3).collect::<Vec<_>>(),vec![30,60]);
        for (i,tick) in [30,60].into_iter().enumerate() {
            calls.tick=tick;
            calls=with_world(calls,|sim|content::resolve_projectile_impact(&mut GameCtx::from_stable(sim),data,data.ult.unwrap(),42,7,InputTarget::Pos{x:140000,y:100000}));
            assert_eq!(calls.damage.len(),i+2);
            assert_eq!(calls.queued.len(),2);
        }
    }

    #[test]
    fn blocked_sky_circus_does_not_hit_its_untraveled_waypoints() {
        let mut registration=ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        let champion=registration.champions.iter().find(|c|c.id()=="pokemon_moba_swanna").unwrap();
        let mut initial=Calls{world:true,enemy_champions:true,..Default::default()};
        initial.positions.insert(8,(160000,100000));initial.positions.insert(9,(600000,600000));
        let mut calls=with_world(initial,|sim|champion.ult().unwrap().effect().unwrap().effect.apply(sim,42,7,InputTargetV1::pos(160000,100000)));
        for tick in 1..=150 {
            calls.tick=tick;
            calls=with_world(calls,|sim|crate::legacy::pokemon_status::update_statuses(&mut GameCtx::from_stable(sim),42));
        }
        assert!(calls.damage.is_empty());
        assert!(!calls.visuals.iter().any(|v|v.0.ends_with("sky_circus_landing")));
    }

    #[test]
    fn periodic_refreshes_replace_payload_but_intentional_buffs_stack() {
        let calls=with_world(Calls { world:true, ..Default::default() }, |sim| {
            let mut ctx=GameCtx::from_stable(sim);
            let buff=BuffState { duration:BuffType::Time {tick:35},move_speed_mult:25,..Default::default() };
            for _ in 0..100 { ctx.refresh_stat_buff(7,"test_periodic",buff.clone()); }
            ctx.refresh_stat_buff(7,"test_periodic",BuffState {move_speed_mult:35,..buff.clone()});
            for _ in 0..3 { ctx.add_buff(7,buff.clone()); }
            for _ in 0..100 { crate::legacy::pokemon_status::refresh_beneficial_buff("test_tracked",&mut ctx,7,7,buff.clone()); }
            assert_eq!(crate::legacy::pokemon_status::tracked_bonus_move_speed_mult(&ctx,7),25);
        });
        assert_eq!(calls.active_buffs[&(7,"test_periodic".into())].len(),1);
        assert_eq!(calls.active_buffs[&(7,"test_periodic".into())][0].move_speed_mult,35);
        assert_eq!(calls.active_buffs[&(7,"pokemon_moba_legacy_buff".into())].len(),3);
        assert_eq!(calls.active_buffs[&(7,"test_tracked_7".into())].len(),1);
    }

    #[test]
    fn first_hit_and_piercing_dashes_hit_only_after_actual_movement() {
        use crate::legacy::pokemon_content::{champion_by_id,PokemonMoveEffect as E};
        for pierce in [false,true] {
            let champion=champion_by_id("pokemon_moba_torterra").unwrap();
            let mut action=champion.skill;
            action.range=100000;
            action.effect=if pierce { E::LineDashPierceDamage {base_ad:100,ad_ratio:0,width:1000,force_move_speed:20000,force_move_ticks:10} }
                else { E::LineFirstDashDamageMissingHp {base_ad:100,ad_ratio:0,missing_hp_ratio:0,width:1000,force_move_speed:20000,force_move_ticks:10} };
            let mut initial=Calls {world:true,..Default::default()};
            initial.positions.insert(9,(150000,100000));
            let mut calls=with_world(initial,|sim| {
                assert!(crate::motion::launch(&mut GameCtx::from_stable(sim),champion,action,7,InputTarget::Pos{x:200000,y:100000}));
            });
            assert!(calls.damage.is_empty());
            assert!(calls.cc.iter().any(|(id,cc)| *id==7 && cc.kind==CcKindV1::ForceMove.code()),"a miss must still begin movement");
            for (tick,x) in [(1,110000),(2,130000),(3,160000),(4,170000)] {
                calls.tick=tick; calls.positions.insert(7,(x,100000));
                calls=with_world(calls,|sim| crate::motion::update(&mut GameCtx::from_stable(sim),1));
                if tick==1 {assert!(calls.damage.is_empty(),"damage before contact");}
            }
            let targets:Vec<_>=calls.damage.iter().map(|hit|hit.1).collect();
            assert_eq!(targets,if pierce {vec![8,9]} else {vec![8]});
        }
    }

    #[test]
    fn blocked_dash_does_not_hit_along_untraveled_route() {
        use crate::legacy::pokemon_content::champion_by_id;
        let champion=champion_by_id("pokemon_moba_pikachu").unwrap();
        let mut calls=with_world(Calls {world:true,..Default::default()},|sim| {
            assert!(crate::motion::launch(&mut GameCtx::from_stable(sim),champion,champion.skill,7,InputTarget::Target{target_id:8}));
        });
        calls.tick=1;
        let calls=with_world(calls,|sim| crate::motion::update(&mut GameCtx::from_stable(sim),1));
        assert!(calls.damage.is_empty());
        assert!(calls.teleports.is_empty());
    }

    #[test]
    fn dash_end_cone_resolves_forward_only_after_arrival() {
        use crate::legacy::pokemon_content::champion_by_id;
        let champion=champion_by_id("pokemon_moba_shedinja").unwrap();
        let mut initial=Calls {world:true,..Default::default()};
        initial.positions.insert(8,(160000,100000)); initial.positions.insert(9,(600000,600000));
        let mut calls=with_world(initial,|sim| {
            assert!(crate::motion::launch(&mut GameCtx::from_stable(sim),champion,champion.skill,7,InputTarget::Pos{x:150000,y:100000}));
        });
        for tick in 1..=6 {
            calls.tick=tick; calls.positions.insert(7,((100000+9000*tick as u64).min(150000),100000));
            calls=with_world(calls,|sim| crate::motion::update(&mut GameCtx::from_stable(sim),1));
            if tick<6 { assert!(calls.damage.is_empty()); }
        }
        assert_eq!(calls.damage.iter().map(|h|h.1).collect::<Vec<_>>(),vec![8]);
    }

    #[test]
    fn parallel_white_flames_preserve_the_gap_and_hit_each_victim_once() {
        use crate::legacy::pokemon_content::champion_by_id;
        let champion = champion_by_id("pokemon_moba_arcanine").unwrap();
        let mut initial = Calls { world: true, ..Default::default() };
        initial.positions.insert(8, (140000, 109000));
        initial.positions.insert(9, (140000, 100000));
        let mut calls = with_world(initial, |sim| {
            assert!(crate::motion::launch(&mut GameCtx::from_stable(sim), champion,
                champion.skill2, 7, InputTarget::Pos { x: 200000, y: 100000 }));
        });
        assert!(calls.damage.is_empty());
        assert_eq!(calls.projectiles.len(), 2);
        for tick in 1..=20 {
            calls.tick = tick;
            calls = with_world(calls, |sim| crate::motion::update(&mut GameCtx::from_stable(sim), 1));
        }
        assert_eq!(calls.damage.iter().map(|hit| hit.1).collect::<Vec<_>>(), vec![8]);
    }

    #[test]
    fn leafage_launches_three_distinct_piercing_paths() {
        let champion = crate::legacy::pokemon_content::champion_by_id("pokemon_moba_decidueye").unwrap();
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            assert!(crate::projectiles::launch(&GameCtx::from_stable(sim), champion,
                champion.attack, 7, InputTarget::Pos { x: 200000, y: 100000 }));
        });
        assert!(calls.damage.is_empty());
        assert_eq!(calls.projectiles.len(), 3);
        assert!(calls.projectiles.iter().all(|(_, _, spec)| spec.penetrate));
        let endpoints: std::collections::BTreeSet<_> = calls.projectiles.iter()
            .map(|(_, _, spec)| (spec.target_x, spec.target_y)).collect();
        assert_eq!(endpoints.len(), 3);
    }

    #[test]
    fn waves_hit_at_arrival_once_and_keep_traveling() {
        use crate::legacy::pokemon_content::{champion_by_id,PokemonMoveEffect as E};
        let champion=champion_by_id("pokemon_moba_gyarados").unwrap();
        let mut action=champion.skill;
        action.effect=E::TravelingWaveDamage {base_ad:100,ad_ratio:0,base_ap:0,ap_ratio:0,travel_range:100000,wall_length:10000,thickness:2000};
        let mut initial=Calls {world:true,..Default::default()};
        initial.positions.insert(8,(140000,100000)); initial.positions.insert(9,(160000,100000));
        let mut calls=with_world(initial,|sim| {assert!(crate::motion::launch(&mut GameCtx::from_stable(sim),champion,action,7,InputTarget::Pos{x:200000,y:100000}));});
        assert!(calls.damage.is_empty());
        for tick in 1..=20 {
            calls.tick=tick;
            calls=with_world(calls,|sim| crate::motion::update(&mut GameCtx::from_stable(sim),1));
            if tick<4 { assert!(calls.damage.is_empty()); }
            if tick==4 { assert_eq!(calls.damage.iter().map(|h|h.1).collect::<Vec<_>>(),vec![8]); }
        }
        assert_eq!(calls.damage.iter().map(|h|h.1).collect::<Vec<_>>(),vec![8,9]);
    }

    #[test]
    fn shed_tail_taunts_to_the_stationary_summon_not_the_caster() {
        let mut registration=ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        let champion=registration.champions.iter().find(|c|c.id()=="pokemon_moba_heliolisk").unwrap();
        let mut initial=Calls {world:true,..Default::default()};
        initial.positions.insert(8,(110000,100000));
        let calls=with_world(initial,|sim| {
            champion.skill2().effect().unwrap().effect.apply(sim,1,7,InputTargetV1::target(8));
        });
        assert_eq!(calls.spawned.len(),1);
        assert_eq!((calls.spawned[0].2,calls.spawned[0].3),(100000,100000));
        assert!(calls.cc.iter().any(|(id,cc)|*id==10 && cc.kind==CcKindV1::BlockAttack.code()));
        let taunts:Vec<_>=calls.cc.iter().filter(|(_,cc)|cc.kind==CcKindV1::Taunt.code()).collect();
        assert!(!taunts.is_empty());
        assert!(taunts.iter().all(|(_,cc)|cc.target==10));
    }

    #[test]
    fn light_metal_grants_one_native_shield_and_does_not_absorb_twice() {
        use crate::legacy::pokemon_status as status;
        let mut calls=with_world(Calls {world:true,tick:1,..Default::default()},|sim| status::update_light_metal_passive(&mut GameCtx::from_stable(sim),7));
        calls.positions.insert(7,(175999,100000)); calls.tick=2;
        calls=with_world(calls,|sim| status::update_light_metal_passive(&mut GameCtx::from_stable(sim),7));
        assert!(calls.shields.is_empty(),"Light Metal must require the full travel distance");
        calls.positions.insert(7,(176001,100000)); calls.tick=3;
        let calls=with_world(calls,|sim| {
            let mut ctx=GameCtx::from_stable(sim);
            status::update_light_metal_passive(&mut ctx,7);
            assert_eq!(status::absorb_light_metal_damage(&ctx,7,200,100),(200,100));
            status::update_light_metal_passive(&mut ctx,7);
        });
        assert_eq!(calls.shields,vec![(7,1600,360)]);
    }

    #[test]
    fn dedenne_grid_uses_configured_node_damage_and_recast_readiness() {
        use crate::legacy::pokemon_content as content;
        let mut reg=ModRegistration::new("pokemon_moba_stable"); content::register_champions(&mut reg);
        let champion=reg.champions.iter().find(|c|c.id()=="pokemon_moba_dedenne").unwrap();
        let data=content::champion_by_id("pokemon_moba_dedenne").unwrap();
        let mut initial=Calls {world:true,tick:1,..Default::default()};
        initial.positions.insert(8,(130000,100000)); initial.positions.insert(9,(600000,600000));
        let calls=with_world(initial,|sim| {
            champion.ult().unwrap().effect().unwrap().effect.apply(sim,1,7,InputTargetV1::default());
            assert!(content::ai_can_activate(&crate::ai_world::World::capture(sim,7),data,data.ult.unwrap()));
            content::resolve_projectile_impact(&mut GameCtx::from_stable(sim),data,data.attack,1,7,InputTarget::Target{target_id:8});
        });
        use crate::legacy::pokemon_types::{apply_damage_modifier,PokemonType,TypeSet};
        let expected=apply_damage_modifier(42,PokemonType::Electric,data.types,TypeSet::single(PokemonType::Normal));
        assert!(calls.damage.iter().any(|hit|hit.1==8 && hit.2==expected),"node must use configured 14+28% AD: {:?}",calls.damage);
        let calls=with_world(calls,|sim| {
            champion.ult().unwrap().effect().unwrap().effect.apply(sim,1,7,InputTargetV1::default());
            assert!(!content::ai_can_activate(&crate::ai_world::World::capture(sim,7),data,data.ult.unwrap()));
        });
        let mut expired=calls; expired.tick=5401;
        with_world(expired,|sim| assert!(content::ai_can_activate(&crate::ai_world::World::capture(sim,7),data,data.ult.unwrap())));
    }

    #[test]
    fn electrode_outcome_waits_for_respawn_and_is_consumed_once_without_a_kill() {
        use crate::legacy::pokemon_content as content;
        let mut reg=ModRegistration::new("pokemon_moba_stable"); content::register_champions(&mut reg);
        let champion=reg.champions.iter().find(|c|c.id()=="pokemon_moba_electrode").unwrap();
        let data=content::champion_by_id("pokemon_moba_electrode").unwrap();
        let content::PokemonMoveEffect::ElectrodeSelfDestruct {charge_ticks,success_move_speed_mult,success_ticks,..}=data.ult.unwrap().effect else {panic!()};
        let mut passive=champion.passive().unwrap();
        let mut calls=with_world(Calls {world:true,tick:1,lethal_self_damage:true,enemy_champions:true,..Default::default()},|sim| {
            passive.on_spawn(sim,1,7);
            champion.ult().unwrap().effect().unwrap().effect.apply(sim,1,7,InputTargetV1::default());
        });
        calls.tick=charge_ticks+1;
        let mut calls=with_world(calls,|sim| passive.on_update(sim,1,1,7));
        assert!(calls.dead.contains(&7));
        assert!(!calls.dead.contains(&8),"enemy deliberately survives; catching it is enough");
        assert!(!calls.active_buffs.contains_key(&(7,"pm_self_destruct_respawn".into())));
        calls.dead.remove(&7); calls.tick+=100;
        let mut calls=with_world(calls,|sim| passive.on_update(sim,1,1,7));
        let buff=&calls.active_buffs[&(7,"pm_self_destruct_respawn".into())][0];
        assert_eq!(buff.move_speed_mult,success_move_speed_mult);
        assert_eq!(buff.duration_tick,success_ticks);
        let count=calls.visual_buffs.iter().filter(|b|b.1=="pm_self_destruct_respawn").count();
        calls.tick+=1;
        let calls=with_world(calls,|sim| passive.on_update(sim,1,1,7));
        assert_eq!(calls.visual_buffs.iter().filter(|b|b.1=="pm_self_destruct_respawn").count(),count);
    }

    #[test]
    fn field_leases_cover_every_tick_complete_cycles_and_retry_rejected_events() {
        for stem in ["kricketune_skill1_sticky_web_deployable", "arboliva_passive_grassy_terrain",
            "goodra_ult_life_dew_pickup", "garganacl_passive_salt_cure_patch",
            "sylveon_skill1_misty_terrain_field", "dedenne_ult_spark_grid_nodes"] {
            let mut calls = Calls { world: true, ..Default::default() };
            let cycle = crate::native_vfx::field_cycle_ticks(stem);
            let mut ends_at = 0;
            for tick in 0..180 {
                calls.tick = tick;
                calls.positions.insert(7, (100000 + tick as u64 * 1000, 100000));
                let before = calls.visuals.len();
                calls = with_world(calls, |sim| {
                    let ctx = GameCtx::from_stable(sim);
                    for _ in 0..3 { crate::native_vfx::field(&ctx, stem, 7, EntityPos { x: 140000, y: 100000 }, 32000); }
                });
                if calls.visuals.len() != before {
                    assert_eq!(calls.visuals.len(), before + 1, "deduplicate {stem}");
                    let v = calls.visuals.last().unwrap();
                    assert_eq!((v.2.x, v.2.y), (140000, 100000), "fixed map position");
                    assert_eq!((v.5 - 2) % cycle, 0, "do not cut {stem} mid-cycle");
                    if tick > 0 { assert!(ends_at > tick, "handoff gap in {stem}"); }
                    ends_at = tick + v.5 as usize;
                }
                assert!(ends_at > tick, "uncovered tick in {stem}");
            }
        }
        let calls = with_world(Calls { world: true, reject_visuals: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            for _ in 0..2 { crate::native_vfx::field(&ctx, "goodra_ult_life_dew_pickup", 7,
                EntityPos { x: 120000, y: 100000 }, 8500); }
        });
        assert_eq!(calls.visuals.len(), 1, "rejected events must not retry in the same tick");
        let mut calls = calls;
        calls.tick += 1;
        calls.reject_visuals = false;
        let calls = with_world(calls, |sim| {
            crate::native_vfx::field(&GameCtx::from_stable(sim), "goodra_ult_life_dew_pickup", 7,
                EntityPos { x: 120000, y: 100000 }, 8500);
        });
        assert_eq!(calls.visuals.len(), 2, "retry next tick, not after a full animation lease");
    }

    #[test]
    fn ai_known_player_snapshot_skips_owner_scan_without_changing_facts() {
        let baseline = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let world = crate::ai_world::World::capture(sim, 7);
            assert_eq!(world.player, Some(1));
        });
        assert_eq!(baseline.player_enumerations.get(), 1);
        let mut snapshot = None;
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            snapshot = Some(crate::ai_world::World::for_player(sim, 7, 1));
        });
        assert_eq!(calls.player_enumerations.get(), 0);
        assert_eq!(calls.entity_enumerations.get(), 3);
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            let old = crate::ai_world::World::capture(sim, 7);
            assert_eq!(format!("{old:?}"), format!("{:?}", snapshot.unwrap()));
        });
    }

    #[test]
    fn caster_only_ai_reads_do_not_enumerate_the_match() {
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            for _ in 0..1000 {
                let world = crate::ai_world::World::for_caster(sim, 7);
                assert_eq!(world.units.len(), 1);
                assert_eq!(world.caster().unwrap().id, 7);
                assert_eq!(world.caster().unwrap().hp, 10000);
            }
        });
        assert_eq!(calls.entity_enumerations.get(), 0);
        let calls = with_world(calls, |sim| {
            assert_eq!(crate::ai_world::World::capture(sim, 7).units.len(), 3);
        });
        assert_eq!(calls.entity_enumerations.get(), 3, "full world still reads all entities");
        with_world(Calls { world: true, dead: [7].into(), ..Default::default() }, |sim| {
            assert!(crate::ai_world::World::for_caster(sim, 7).caster().is_none());
            assert!(crate::ai_world::World::for_caster(sim, 99).caster().is_none());
        });
    }

    #[test]
    fn permanent_webs_and_grass_survive_death_and_stop_refreshing_after_destruction() {
        use crate::legacy::pokemon_status as status;
        let center = EntityPos { x: 100000, y: 100000 };
        let mut calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            status::begin_sticky_web(&ctx, 7, center, 64000, 5, 18, 8, 45);
            status::maybe_spawn_grassy_terrain(&ctx, 7, 3, 32000, 300, 12, 18, 35);
        });
        assert_eq!(calls.visuals.len(), 2, "render on creation");
        calls.dead.insert(7);
        calls.positions.insert(7, (800000, 800000));
        calls.tick = 120;
        calls = with_world(calls, |sim| status::update_statuses(&mut GameCtx::from_stable(sim), 1));
        assert_eq!(calls.visuals.len(), 4, "death must not delete permanent fields");
        assert!(calls.visuals.iter().all(|v| (v.2.x, v.2.y) == (100000, 100000)));
        calls.tick = 240;
        calls = with_world(calls, |sim| {
            status::burn_sticky_webs_at(center, 32000);
            status::clear_grassy_terrains_at(center, 32000);
            status::update_statuses(&mut GameCtx::from_stable(sim), 1);
        });
        assert_eq!(calls.visuals.len(), 4, "removed fields stop renewing");
    }

    #[test]
    fn respawn_handles_keep_permanent_field_caps_and_life_dew_ownership() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        let mut calls = with_world(Calls { world: true, ally_nine: true, tick: 1, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::register_player_entity_life_at_tick(&ctx, 1, 7, 1, Some(0));
            for x in [100000, 130000, 160000, 190000, 220000] {
                status::begin_sticky_web(&ctx, 7, EntityPos { x, y: 100000 }, 64000, 5, 18, 8, 45);
            }
            for x in [100000, 130000, 160000] {
                status::spawn_grassy_terrain_at(&ctx, 7, EntityPos { x, y: 100000 }, 3, 32000, usize::MAX, 12, 18, 35);
            }
            let champion = content::champion_by_id("pokemon_moba_goodra").unwrap();
            content::resolve_projectile_impact(&mut ctx, champion, champion.ult.unwrap(), 1, 7, InputTarget::None);
        });
        calls.visuals.clear();
        calls.tick = 120;
        calls = with_world(calls, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::register_player_entity_life_at_tick(&ctx, 1, 9, 120, Some(1));
            status::begin_sticky_web(&ctx, 9, EntityPos { x: 250000, y: 100000 }, 64000, 5, 18, 8, 45);
            status::spawn_grassy_terrain_at(&ctx, 9, EntityPos { x: 250000, y: 100000 }, 3, 32000, usize::MAX, 12, 18, 35);
            status::update_statuses(&mut ctx, 1);
            content::update_native_field_visuals(&ctx);
        });
        assert!(calls.visuals.iter().all(|v| v.1 == 9));
        assert_eq!(calls.visuals.iter().filter(|v| v.0.ends_with("sticky_web_deployable_loop")).count(), 5);
        assert_eq!(calls.visuals.iter().filter(|v| v.0.ends_with("grassy_terrain_loop")).count(), 3);
        assert_eq!(calls.visuals.iter().filter(|v| v.0.ends_with("life_dew_pickup_loop")).count(), 14);
    }

    #[test]
    fn grassy_overlap_aggregates_once_and_does_not_give_foreign_or_expired_damage_bonuses() {
        use crate::legacy::{pokemon_status as status, pokemon_types::PokemonType};
        let mut calls = with_world(Calls { world: true, tick: 1, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            for x in [95000, 100000, 105000] {
                status::spawn_grassy_terrain_at(&ctx, 7, EntityPos { x, y: 100000 }, 3, 32000, 60, 12, 18, 35);
            }
            status::update_statuses(&mut ctx, 1);
            assert_eq!(status::grassy_terrain_damage_bonus_for_caster(&ctx, 7, PokemonType::Grass), 36);
            assert_eq!(status::grassy_terrain_damage_bonus_for_caster(&ctx, 8, PokemonType::Grass), 0);
        });
        let key = (7, "pm_grassy_terrains_0_7".to_owned());
        assert_eq!(calls.active_buffs[&key].len(), 1);
        assert_eq!(calls.active_buffs[&key][0].attack_speed_mult, 54);
        calls.tick = 31;
        calls = with_world(calls, |sim| status::update_statuses(&mut GameCtx::from_stable(sim), 1));
        assert_eq!(calls.active_buffs[&key].len(), 1);
        assert_eq!(calls.active_buffs[&key][0].attack_speed_mult, 54);
        calls.tick = 61;
        with_world(calls, |sim| assert_eq!(status::grassy_terrain_damage_bonus_for_caster(
            &GameCtx::from_stable(sim), 7, PokemonType::Grass), 0));
    }

    #[test]
    fn sticky_web_ai_avoids_duplicate_centers_but_allows_overlapping_chains() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            status::begin_sticky_web(&GameCtx::from_stable(sim), 7,
                EntityPos { x: 120000, y: 100000 }, 64000, 5, 18, 8, 45);
            let world = crate::ai_world::World::capture(sim, 7);
            let champion = content::champion_by_id("pokemon_moba_kricketune").unwrap();
            assert!(!content::ai_target_allowed(&world, champion, champion.skill, 8));
            assert!(!content::ai_input_allowed(&world, "Kricketune",
                &Input::Skill { target: InputTarget::Pos { x: 121000, y: 100000 } }));
            assert!(content::ai_input_allowed(&world, "Kricketune",
                &Input::Skill { target: InputTarget::Pos { x: 145000, y: 100000 } }));
        });
    }

    #[test]
    fn life_dew_has_healing_metadata_and_creates_bubbles_before_passive_updates() {
        use crate::legacy::pokemon_content as content;
        let mut reg = ModRegistration::new("pokemon_moba_stable"); content::register_champions(&mut reg);
        let champion = reg.champions.iter().find(|c| c.id() == "pokemon_moba_goodra").unwrap();
        let mut passive = champion.passive().unwrap();
        let effect = champion.ult().unwrap().effect().unwrap();
        assert_eq!(effect.effect.expected_heal(&StatV1::default()), 14 * 175);
        let mut calls = with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
            effect.effect.apply(sim, 1, 7, InputTarget::None.to_stable());
        });
        assert_eq!(calls.visuals.len(), 14);
        assert!(calls.visuals.iter().all(|v| v.0.ends_with("life_dew_pickup_loop")));
        calls.tick = 120;
        calls.dead.insert(7);
        calls = with_world(calls, |sim| passive.on_update(sim, 1, 1, 7));
        assert_eq!(calls.visuals.len(), 28, "bubbles retain their lifetime after Goodra dies");
        calls.tick = 3600;
        calls = with_world(calls, |sim| passive.on_update(sim, 1, 1, 7));
        assert_eq!(calls.visuals.len(), 28, "expired bubbles stop rendering");
        assert!(calls.heals.is_empty(), "full-health allies must not consume bubbles");
    }

    #[test]
    fn salt_has_creation_and_contact_visuals_and_permanent_stage_waits_for_spire() {
        use crate::legacy::pokemon_status as status;
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::garganacl_spawn_passive_salt(&ctx, 7, 6, 22000, 300, 40, 20, 90, 50);
            status::update_statuses(&mut ctx, 1);
        });
        assert!(calls.visuals.iter().any(|v| v.0.ends_with("salt_cure_patch_loop")));
        assert!(calls.visuals.iter().any(|v| v.0.ends_with("salt_cure_patch")));
        assert_eq!(calls.damage.len(), 1, "visuals must not duplicate salt damage");
        let mut calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::begin_garganacl_blessed_salt(&ctx, 7, EntityPos { x: 120000, y: 100000 },
                60, 30000, 10000, 30, 10, 20, 30, 25, 30, 30, 5, 50);
            status::update_statuses(&mut ctx, 1);
            assert!(!status::garganacl_entity_in_salt(&ctx, 7));
        });
        assert!(!calls.visuals.iter().any(|v| v.0.ends_with("salt_cure_patch_loop")));
        calls.tick = 60;
        calls = with_world(calls, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::update_statuses(&mut ctx, 1);
            assert!(status::garganacl_entity_in_salt(&ctx, 7));
        });
        assert!(calls.visuals.iter().any(|v| v.0.ends_with("salt_cure_patch_loop")));
    }

    #[test]
    fn fields_render_immediately_refresh_once_and_rectangles_use_their_center() {
        let calls=with_world(Calls {world:true,tick:7,..Default::default()},|sim| {
            let ctx=GameCtx::from_stable(sim);
            for _ in 0..10 { crate::native_vfx::rectangle(&ctx,"goodra_skill1_rain_dance_field",7,
                EntityPos{x:100000,y:100000},EntityPos{x:100000,y:110000},80000,20000); }
        });
        assert_eq!(calls.visuals.len(),1);
        assert_eq!((calls.visuals[0].2.x,calls.visuals[0].2.y),(100000,140000));
        assert_eq!((calls.visuals[0].3,calls.visuals[0].4),(80000,10000));
        let cycle = crate::native_vfx::field_cycle_ticks("goodra_skill1_rain_dance_field");
        assert_eq!(calls.visuals[0].5, cycle * 30u64.div_ceil(cycle) + 2);
    }

    #[test]
    fn toxic_chain_uses_resolved_endpoints_without_changing_poison_or_execute() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        let champion = content::champion_by_id("pokemon_moba_dragalge").unwrap();
        let action = champion.ult.unwrap();
        for (caster, start, end, direction) in [
            (7, (100000, 100000), (140000, 100000), 0),
            (9, (100000, 100000), (100000, 140000), 4),
            (7, (100000, 100000), (140000, 140000), 2),
            (7, (140000, 100000), (100000, 100000), 8),
        ] {
            let mut initial = Calls { world: true, enemy_champions: true, ally_nine: true, ..Default::default() };
            initial.positions.insert(caster, start);
            initial.positions.insert(8, end);
            let calls = with_world(initial, |sim| {
                let mut ctx = GameCtx::from_stable(sim);
                crate::native_vfx::on_cast(&ctx, champion.id, action, caster, InputTarget::Target { target_id: 8 });
                content::resolve_projectile_impact(&mut ctx, champion, action, 42, caster, InputTarget::Target { target_id: 8 });
                assert_eq!(status::poison_stack_count(&ctx, 8), 50);
            });
            assert!(calls.damage.is_empty());
            let distance = ((end.0 as f64-start.0 as f64).hypot(end.1 as f64-start.1 as f64)) as u64;
            let links = distance.div_ceil(8000).clamp(1,32) as usize;
            assert_eq!(calls.visuals.len(), links + 1, "no duplicate generic cast visual");
            assert_eq!(calls.visuals[0].0, crate::native_vfx::test_direction_name("dragalge_ult_toxic_chain_tether", direction));
            assert_eq!(calls.visuals[0].1, caster);
            for (i, visual) in calls.visuals[..links].iter().enumerate() {
                let fraction = |a:u64,z:u64| (a as i128+(z as i128-a as i128)*(2*i+1) as i128/(2*links) as i128) as u64;
                assert_eq!((visual.2.x,visual.2.y),(fraction(start.0,end.0),fraction(start.1,end.1)));
                assert_eq!((visual.3,visual.4),(0,0), "link placement does not depend on renderer scaling");
            }
            assert_eq!((calls.visuals[links].2.x, calls.visuals[links].2.y), end);
            assert!(calls.visuals[links].0.ends_with("toxic_chain_impact"));
            let calls = with_world(calls, |sim| {
                content::resolve_projectile_impact(&mut GameCtx::from_stable(sim), champion, action, 42, caster, InputTarget::Target { target_id: 8 });
            });
            assert_eq!(calls.damage.len(), 1, "existing poison threshold still executes once");
            assert_eq!(calls.visuals.len(), 2*(links+1));
        }
    }

    #[test]
    fn toxic_chain_overlap_uses_only_impact_and_missing_target_emits_nothing() {
        let mut initial = Calls { world: true, ..Default::default() };
        initial.positions.insert(8, (100000, 100000));
        let calls = with_world(initial, |sim| {
            let ctx = GameCtx::from_stable(sim);
            crate::native_vfx::toxic_chain(&ctx, 7, 8);
            crate::native_vfx::toxic_chain(&ctx, 7, 999);
        });
        assert_eq!(calls.visuals.len(), 1);
        assert!(calls.visuals[0].0.ends_with("toxic_chain_impact"));
        assert!(calls.damage.is_empty());
    }

    #[test]
    fn roster_cast_visuals_use_world_positions_not_live_entity_handles() {
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            for champion in crate::legacy::pokemon_content::POKEMON_CHAMPIONS {
                for action in [Some(champion.attack), Some(champion.skill), Some(champion.skill2), champion.ult].into_iter().flatten() {
                    crate::native_vfx::on_cast(&ctx, champion.id, action, 7, InputTarget::Target { target_id: 8 });
                }
            }
        });
        assert!(calls.visuals.len() > 100);
        assert!(calls.visuals.iter().all(|v| v.2.kind == InputTargetV1::pos(0, 0).kind));
        assert!(calls.damage.is_empty());
    }

    #[test]
    fn cast_visual_geometry_matches_cones_self_bursts_and_multiple_areas() {
        use crate::legacy::pokemon_content::{champion_by_id, PokemonMoveEffect as E};
        let champion = champion_by_id("pokemon_moba_charizard").unwrap();
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            let mut action = champion.attack;
            action.effect = E::ConeDamage { base_ad: 0, ad_ratio: 0, base_ap: 10, ap_ratio: 0, length: 60000, width_at_end: 40000,
                blaze_contact: false, burn_base_ap: 0, burn_ap_ratio: 0 };
            crate::native_vfx::on_cast(&ctx, champion.id, action, 7, InputTarget::Pos { x: 100000, y: 120000 });
            action.effect = E::SelfAreaDamage { base_ad: 0, ad_ratio: 0, base_ap: 10, ap_ratio: 0, radius: 42000 };
            crate::native_vfx::on_cast(&ctx, champion.id, action, 7, InputTarget::Target { target_id: 8 });
            action.effect = E::TripleAreaDamage { base_ad: 0, ad_ratio: 0, base_ap: 10, ap_ratio: 0, radius: 6000, spread: 10000 };
            crate::native_vfx::on_cast(&ctx, champion.id, action, 7, InputTarget::Target { target_id: 8 });
        });
        assert_eq!(calls.visuals.len(), 4);
        assert_eq!(calls.visual_buffs.len(),1);
        assert!(calls.active_buffs.contains_key(&(7,crate::native_vfx::test_direction_name("charizard_basicattack_flamethrower_cone",4))));
        assert_eq!((calls.visuals[0].2.x, calls.visuals[0].2.y, calls.visuals[0].4), (100000,100000,42000));
        assert_eq!(calls.visuals[1..].iter().map(|v| (v.2.x,v.2.y)).collect::<Vec<_>>(),
            vec![(130000,100000),(115000,110000),(115000,90000)]);
    }

    #[test]
    fn bursts_freeze_targets_and_auras_refresh_immediately_without_stacking() {
        let stem = "magmortar_skill2_heat_wave_ring";
        let mut calls = with_world(Calls { world: true, tick: 7, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            crate::native_vfx::burst(&ctx, "pikachu_ult_thunder_strike_aoe", 7, InputTarget::Target { target_id: 8 }, 30000);
            for _ in 0..5 { crate::native_vfx::attached(&ctx, stem, 7); }
        });
        assert_eq!(calls.visual_buffs.len(), 1);
        calls.positions.insert(8, (500000,500000));
        calls.positions.insert(7, (300000,300000));
        calls.tick = 22;
        let calls = with_world(calls, |sim| crate::native_vfx::attached(&GameCtx::from_stable(sim), stem, 7));
        assert_eq!((calls.visuals[0].2.x,calls.visuals[0].2.y), (120000,100000));
        assert_eq!(calls.visuals[0].2.kind, InputTargetV1::pos(0,0).kind);
        assert_eq!(calls.visual_buffs.len(), 2);
        assert_eq!(calls.active_buffs[&(7,"pokemon_moba_stable_magmortar_skill2_heat_wave_ring".into())].len(), 1);
    }

    #[test]
    fn smoke_visual_uses_stored_ground_segment_after_caster_and_target_move() {
        use crate::legacy::pokemon_status as status;
        let mut calls = with_world(Calls { world: true, tick: 7, ..Default::default() }, |sim| {
            status::begin_smoke_screen(&mut GameCtx::from_stable(sim), 7,
                EntityPos { x:100000,y:100000 }, EntityPos { x:140000,y:100000 }, 100, 60000);
            status::update_statuses(&mut GameCtx::from_stable(sim), 1);
        });
        calls.tick = 7 + (calls.visuals[0].5 - 2) as usize;
        calls.positions.insert(7,(500000,500000)); calls.positions.insert(8,(600000,600000));
        let calls = with_world(calls, |sim| status::update_statuses(&mut GameCtx::from_stable(sim),1));
        let smoke: Vec<_> = calls.visuals.iter().filter(|v| v.0.contains("smokescreen")).collect();
        assert_eq!(smoke.len(), 2);
        assert!(smoke.iter().all(|v| (v.2.x,v.2.y,v.3,v.4) == (140000,100000,60000,6500)));
    }

    #[test]
    fn delayed_and_conditional_art_is_not_emitted_by_generic_casts() {
        use crate::legacy::pokemon_content::champion_by_id;
        for (id, slot) in [("oranguru",2), ("starmie",2), ("starmie",3),
            ("ampharos",3), ("zeraora",2), ("zeraora",3), ("eevee",2),
            ("banette",3), ("gholdengo",3), ("xatu",3), ("yanmega",3), ("delibird",3)] {
            let c = champion_by_id(&format!("pokemon_moba_{id}")).unwrap();
            let a = [Some(c.attack),Some(c.skill),Some(c.skill2),c.ult][slot].unwrap();
            let calls = with_world(Calls { world:true,..Default::default() }, |sim| {
                crate::native_vfx::on_cast(&GameCtx::from_stable(sim), c.id,a,7,InputTarget::Target { target_id:8 });
            });
            assert!(calls.visuals.is_empty(), "{id} {slot}: premature or duplicated artwork");
        }
    }

    #[test]
    fn stored_power_art_waits_for_release_and_uses_current_caster_position() {
        use crate::legacy::{pokemon_status as status,pokemon_types::{TypeSet,PokemonType}};
        let mut calls = with_world(Calls { world:true,tick:1,..Default::default() }, |sim| {
            let mut ctx=GameCtx::from_stable(sim);
            status::begin_stored_power(&ctx,7,10,42000,TypeSet::single(PokemonType::Psychic));
            status::note_stored_power_damage(&ctx,7,100);
            status::update_statuses(&mut ctx,1);
        });
        assert!(calls.visuals.is_empty());
        calls.tick=11; calls.positions.insert(7,(130000,110000));
        let calls=with_world(calls,|sim| {
            let mut ctx=GameCtx::from_stable(sim);
            status::note_stored_power_damage(&ctx,7,999); // Must not erase the due release.
            status::update_statuses(&mut ctx,1);
        });
        let v=calls.visuals.iter().find(|v|v.0.ends_with("oranguru_skill2_stored_power_burst")).unwrap();
        assert_eq!((v.2.x,v.2.y,v.4),(130000,110000,42000));
    }

    #[test]
    fn psybeam_damage_and_art_wait_for_charge_and_do_not_queue_again() {
        use crate::legacy::pokemon_content as content;
        let mut registration=ModRegistration::new("pokemon_moba_stable");
        content::register_champions(&mut registration);
        let champ=registration.champions.iter().find(|c|c.id()=="pokemon_moba_starmie").unwrap();
        let effect=champ.ult().unwrap().effect().unwrap();
        let mut calls=with_world(Calls {world:true,..Default::default()},|sim| {
            effect.effect.apply(sim,1,7,InputTargetV1::target(8));
        });
        assert!(calls.damage.is_empty());
        assert!(calls.visuals.is_empty());
        assert_eq!(calls.queued.len(),1);
        assert_eq!(calls.queued[0].3,90);
        let input=InputTarget::from_stable(calls.queued[0].2);
        calls.tick=90;
        let data=content::champion_by_id("pokemon_moba_starmie").unwrap();
        let calls=with_world(calls,|sim| content::resolve_projectile_impact(
            &mut GameCtx::from_stable(sim),data,data.ult.unwrap(),1,7,input));
        assert!(!calls.damage.is_empty());
        assert_eq!(calls.queued.len(),1);
        assert_eq!(calls.visuals.iter().filter(|v|v.0==crate::native_vfx::test_direction_name("starmie_ult_psybeam_line",0)).count(),1);
    }

    #[test]
    fn chit_visuals_replace_the_count_and_clear_when_not_downed() {
        let calls=with_world(Calls {world:true,..Default::default()},|sim| {
            let ctx=GameCtx::from_stable(sim);
            crate::native_vfx::wonder_guard_chits(&ctx,7,Some(5));
            crate::native_vfx::wonder_guard_chits(&ctx,7,Some(3));
        });
        assert!(!calls.active_buffs.contains_key(&(7,"pm_wonder_guard_5".into())));
        assert_eq!(calls.active_buffs[&(7,"pm_wonder_guard_3".into())].len(),1);
        let calls=with_world(calls,|sim| crate::native_vfx::wonder_guard_chits(&GameCtx::from_stable(sim),7,None));
        assert!(!calls.active_buffs.keys().any(|(_,name)|name.starts_with("pm_wonder_guard_")));
    }

    #[test]
    fn light_screen_visual_uses_persistent_wall_geometry() {
        use crate::legacy::pokemon_status as status;
        let calls=with_world(Calls {world:true,tick:1,..Default::default()},|sim| {
            let mut ctx=GameCtx::from_stable(sim);
            status::begin_light_screen(&mut ctx,7,EntityPos{x:100000,y:100000},EntityPos{x:140000,y:100000},480,90000);
            status::update_statuses(&mut ctx,1);
        });
        let v=calls.visuals.iter().find(|v|v.0==format!("{}_loop",crate::native_vfx::test_direction_name("beeheeyem_skill2_light_screen",12))).unwrap();
        assert_eq!((v.2.x,v.2.y,v.3,v.4),(140000,100000,90000,5500));
    }

    #[test]
    fn dash_impact_art_waits_for_contact_and_blocked_streaks_are_removed() {
        use crate::legacy::pokemon_content::champion_by_id;
        let champion = champion_by_id("pokemon_moba_flareon").unwrap();
        let action = champion.ult.unwrap();
        let mut calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            assert!(crate::motion::launch(&mut GameCtx::from_stable(sim), champion, action, 7, InputTarget::Target { target_id:8 }));
        });
        assert!(calls.visuals.is_empty(), "impact art at launch");
        calls.tick = 1; calls.positions.insert(7,(110000,100000));
        let calls = with_world(calls, |sim| crate::motion::update(&mut GameCtx::from_stable(sim),1));
        let impact = calls.visuals.iter().find(|v| v.0.ends_with("flareon_ult_fire_fang_impact")).unwrap();
        assert_eq!((impact.2.x,impact.2.y),(120000,100000));

        let champion = champion_by_id("pokemon_moba_pikachu").unwrap();
        let mut calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            assert!(crate::motion::launch(&mut GameCtx::from_stable(sim), champion, champion.skill, 7, InputTarget::Target { target_id:8 }));
        });
        let key = (7,"pokemon_moba_stable_pikachu_skill1_volt_tackle_dash_trail".to_owned());
        assert!(calls.active_buffs.contains_key(&key));
        calls.tick = 1;
        let calls = with_world(calls, |sim| crate::motion::update(&mut GameCtx::from_stable(sim),1));
        assert!(!calls.active_buffs.contains_key(&key));
    }

    #[test]
    fn copied_damage_keeps_move_element_without_borrowing_species_stab() {
        use crate::legacy::{pokemon_status as status,pokemon_types as types};
        use types::{PokemonType as T,TypeSet};
        let calls=with_world(Calls {world:true,..Default::default()},|sim| {
            let mut ctx=GameCtx::from_stable(sim);
            status::register_entity_champion(&ctx,7,"pokemon_moba_smeargle");
            status::register_entity_types(7,TypeSet::single(T::Normal));
            types::deal_pokemon_damage(&mut ctx,7,8,100,0,AttackType::Skill,T::Fire,TypeSet::single(T::Fire),TypeSet::single(T::Grass));
        });
        assert_eq!(calls.damage[0].2,types::apply_damage_modifier(100,T::Fire,TypeSet::single(T::Normal),TypeSet::single(T::Grass)));
    }

    #[test]
    fn life_dew_does_not_consume_on_full_health_and_heals_only_once() {
        use crate::legacy::pokemon_content as content;
        let mut reg=ModRegistration::new("pokemon_moba_stable"); content::register_champions(&mut reg);
        let champion=reg.champions.iter().find(|c|c.id()=="pokemon_moba_goodra").unwrap();
        let mut passive=champion.passive().unwrap();
        let data=content::champion_by_id("pokemon_moba_goodra").unwrap();
        let mut action=data.ult.unwrap();
        action.effect=content::PokemonMoveEffect::LifeDewPickups {pickup_count:1,duration_ticks:600,radius:5000,heal_amount:200,spread:30000};
        let mut initial=Calls {world:true,ally_nine:true,tick:1,..Default::default()};
        initial.positions.insert(9,(110000,100000));
        let mut calls=with_world(initial,|sim| {
            content::resolve_projectile_impact(&mut GameCtx::from_stable(sim),data,action,1,7,InputTarget::None);
            passive.on_update(sim,1,1,7);
        });
        assert!(calls.heals.is_empty());
        calls.tick=2; calls.hp_current=Some(9000);
        let mut calls=with_world(calls,|sim| passive.on_update(sim,1,1,7));
        assert_eq!(calls.heals,vec![(7,9,200)]);
        calls.tick=3;
        let calls=with_world(calls,|sim| passive.on_update(sim,1,1,7));
        assert_eq!(calls.heals.len(),1);
    }

    #[test]
    fn healing_has_one_antiheal_owner_and_cleanse_removes_both_layers() {
        use crate::legacy::pokemon_status as status;
        let calls = with_world(Calls { world: true, hp_current: Some(9000), ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::apply_anti_heal(&mut ctx, 8, 7, 25, 120);
            status::apply_anti_heal(&mut ctx, 9, 7, 50, 60);
            assert_eq!(status::heal_with_antiheal(&mut ctx, 7, 7, 400), 200);
            status::cleanse_harmful_statuses(&mut ctx, 7);
            assert_eq!(status::anti_heal_percent(&ctx, 7), 0);
            assert_eq!(status::heal_with_antiheal(&mut ctx, 7, 7, 400), 400);
        });
        assert_eq!(calls.heals, vec![(7, 7, 400), (7, 7, 400)]);
        assert_eq!(calls.clear_cc, vec![7]);
        assert!(!calls.active_buffs.contains_key(&(7, "pokemon_moba_anti_heal".into())));
        let capped = with_world(Calls { world: true, hp_current: Some(9900), ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::apply_anti_heal(&mut ctx, 8, 7, 50, 60);
            assert_eq!(status::heal_with_antiheal(&mut ctx, 7, 7, 400), 100);
        });
        assert_eq!(capped.heals[0].2, 400, "engine must cap after reduction, not before");
    }

    #[test]
    fn native_stealth_and_terrain_buffs_are_real_engine_calls() {
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            crate::legacy::pokemon_status::begin_weavile_hunt_stealth(&ctx, 7, 120);
            crate::legacy::pokemon_status::break_weavile_hunt_stealth(&ctx, 7);
            crate::legacy::pokemon_status::update_suction_cups(&mut ctx, 7);
        });
        assert_eq!(calls.invisible, vec![(7, 2), (7, 0)]);
        let buff = calls.active_buffs.get(&(7, "pokemon_moba_suction_cups".into())).unwrap().last().unwrap();
        assert!(buff.ignore_wall);
        assert_eq!(buff.move_speed_mult, -15);
    }

    #[test]
    fn shiftry_uses_bush_cells_not_visibility_and_preserves_qualified_linger() {
        use crate::legacy::pokemon_status as status;
        let mut calls = with_world(Calls { world: true, tick: 1, ..Default::default() }, |sim| {
            crate::map_terrain::install_test_bush(100000, 100000);
            let mut ctx = GameCtx::from_stable(sim);
            assert!(ctx.is_visible(1, 7));
            status::update_shiftry_forest_camouflage(&mut ctx, 7);
        });
        assert_eq!(calls.visual_buffs.len(), 1, "bush occupancy must work even when visible");
        assert!(calls.active_buffs.iter().any(|((id,_), buffs)| *id == 7 && buffs.iter().any(|buff| buff.move_speed_mult == 15)));
        calls.tick = 91;
        let mut calls = with_world(calls, |sim| status::update_shiftry_forest_camouflage(&mut GameCtx::from_stable(sim), 7));
        calls.tick = 92;
        calls.positions.insert(7, (300000,100000));
        let mut calls = with_world(calls, |sim| status::update_shiftry_forest_camouflage(&mut GameCtx::from_stable(sim), 7));
        calls.tick = 202;
        let mut calls = with_world(calls, |sim| status::update_shiftry_forest_camouflage(&mut GameCtx::from_stable(sim), 7));
        let during_linger = calls.visual_buffs.len();
        assert!(during_linger >= 3);
        calls.tick = 212;
        let calls = with_world(calls, |sim| status::update_shiftry_forest_camouflage(&mut GameCtx::from_stable(sim), 7));
        assert_eq!(calls.visual_buffs.len(), during_linger, "no further refresh at the two-second linger boundary");
        let calls = with_world(Calls { world: true, hidden: true, tick: 1, ..Default::default() }, |sim| {
            crate::map_terrain::install_test_bush(600000, 600000);
            status::update_shiftry_forest_camouflage(&mut GameCtx::from_stable(sim), 7);
        });
        assert_eq!(calls.buffs, 0, "unseen open ground must not count as a bush");
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            status::update_shiftry_forest_camouflage(&mut GameCtx::from_stable(sim), 7);
        });
        assert_eq!(calls.buffs, 0, "no grid or initial tick must not grant camouflage");
    }

    #[test]
    fn ampharos_reveals_camouflage_and_pulses_without_visibility() {
        use crate::legacy::pokemon_status as status;
        let mut calls = with_world(Calls { world: true, hidden: true, tick: 1, ..Default::default() }, |sim| {
            crate::map_terrain::install_test_bush(100000, 100000);
            let mut ctx = GameCtx::from_stable(sim);
            status::register_entity_champion(&ctx, 8, "pokemon_moba_ampharos");
            status::update_ampharos_luminous_pulse(&mut ctx, 8);
            assert!(status::ampharos_true_sight_reveals(&ctx, 7));
            status::update_shiftry_forest_camouflage(&mut ctx, 7);
            status::begin_weavile_hunt_stealth(&ctx, 7, 120);
        });
        assert_eq!(calls.buffs, 0, "true sight denies new camouflage");
        assert_eq!(calls.invisible, vec![(7, 0)], "true sight overrides native Hunt stealth");
        calls.tick = 301;
        let mut calls = with_world(calls, |sim| status::update_ampharos_luminous_pulse(&mut GameCtx::from_stable(sim), 8));
        assert!(calls.damage.iter().any(|(attacker,target,_,_,_)| *attacker == 8 && *target == 7));
        calls.positions.insert(7, (500000,100000));
        with_world(calls, |sim| assert!(!status::ampharos_true_sight_reveals(&GameCtx::from_stable(sim), 7)));
    }

    #[test]
    fn bush_maps_remain_separate_across_interleaved_simulations() {
        let a = with_world(Calls { world: true, ..Default::default() }, |_| {
            crate::map_terrain::install_test_bush(100000, 100000);
        });
        with_world(Calls { world: true, ..Default::default() }, |_| {
            assert_eq!(crate::map_terrain::in_bush(100000, 100000), None);
            crate::map_terrain::install_test_bush(500000, 500000);
            assert_eq!(crate::map_terrain::in_bush(100000, 100000), Some(false));
        });
        with_world(a, |_| assert_eq!(crate::map_terrain::in_bush(100000, 100000), Some(true)));
    }

    #[test]
    fn shared_detection_is_team_scoped_expires_and_does_not_track_unseen_movement() {
        use crate::legacy::{pokemon_status as status, pokemon_content as content};
        let mut calls = with_world(Calls { world: true, hidden: true, tick: 1, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::register_entity_champion(&ctx, 8, "pokemon_moba_ampharos");
            status::update_ampharos_luminous_pulse(&mut ctx, 8);
            assert_eq!(status::team_spotted_targets(1, 1).len(), 1);
            assert!(status::team_spotted_targets(0, 1).is_empty());
            let world = crate::ai_world::World::capture(sim, 9);
            assert!(world.get(7).unwrap().detected, "Ampharos shares with teammates before its first damage pulse");
            let pikachu = content::champion_by_id("pokemon_moba_pikachu").unwrap();
            assert!(content::ai_target_allowed(&world, pikachu, pikachu.attack, 7));
            let mut untargetable = world.clone();
            untargetable.units.iter_mut().find(|unit| unit.id == 7).unwrap().targetable = false;
            assert!(!content::ai_target_allowed(&untargetable, pikachu, pikachu.attack, 7));
        });
        calls.tick = 20;
        calls.positions.insert(7, (500000, 500000));
        let mut calls = with_world(calls, |sim| {
            let world = crate::ai_world::World::capture(sim, 9);
            let enemy = world.get(7).unwrap();
            assert!(!enemy.detected);
            assert!(enemy.remembered);
            assert_eq!((enemy.pos.x, enemy.pos.y), (100000, 100000));
            let pikachu = content::champion_by_id("pokemon_moba_pikachu").unwrap();
            assert!(!content::ai_target_allowed(&world, pikachu, pikachu.attack, 7));
        });
        with_world(Calls { world: true, hidden: true, tick: 20, ..Default::default() }, |sim| {
            assert!(!crate::ai_world::World::capture(sim, 9).get(7).unwrap().remembered);
        });
        calls.tick = 121;
        with_world(calls, |sim| {
            assert!(!crate::ai_world::World::capture(sim, 9).get(7).unwrap().remembered);
        });
    }

    #[test]
    fn connected_webs_share_hidden_enemies_and_disconnected_webs_do_not() {
        use crate::legacy::pokemon_status as status;
        for connected in [false, true] {
            with_world(Calls { world: true, hidden: true, tick: 1, ..Default::default() }, |sim| {
                let mut ctx = GameCtx::from_stable(sim);
                status::begin_sticky_web(&ctx, 8, EntityPos { x: 100000, y: 100000 },
                    if connected { 30000 } else { 1000 }, 5, 15, 10, 30);
                status::update_statuses(&mut ctx, 1);
                assert_eq!(!status::team_spotted_targets(1, 1).is_empty(), connected);
                status::begin_weavile_hunt_stealth(&ctx, 7, 120);
                assert_eq!(status::is_weavile_hidden_from_pokemon_ai(&ctx, 7), !connected);
                let world = crate::ai_world::World::capture(sim, 9);
                assert_eq!(world.get(7).unwrap().detected, connected);
            });
        }
    }

    #[test]
    fn detection_ai_selects_combat_but_respects_host_rejection() {
        unsafe extern "C" fn validate(state: *const c_void, input: *const InputV1) -> bool {
            match *(state as *const u8) {
                0 => true,
                1 => (*input).kind == InputKindV1::Move.code(),
                _ => false,
            }
        }
        with_world(Calls { world: true, hidden: true, ..Default::default() }, |sim| {
            let mut world = crate::ai_world::World::capture(sim, 7);
            world.units.retain(|unit| unit.id != 9);
            let enemy = world.units.iter_mut().find(|unit| unit.id == 8).unwrap();
            enemy.champion = true; enemy.detected = true; enemy.remembered = true;
            for mut mode in 0u8..3 {
                let mut table: mod_api_stable::AiVtableV1 = unsafe { std::mem::zeroed() };
                table.size = size_of::<mod_api_stable::AiVtableV1>();
                table.is_valid_input = Some(validate);
                let mut raw: mod_api_stable::AiCtxV1 = unsafe { std::mem::zeroed() };
                raw.size = size_of::<mod_api_stable::AiCtxV1>();
                raw.vtable = &table;
                raw.state = &mut mode as *mut u8 as *mut c_void;
                let mut inner = unsafe { StableAiContext::from_raw(&mut raw) }.unwrap();
                let mut ctx = PlayerAiContext { inner: &mut inner, champion_name: "Pikachu".into(),
                    world: Some(world.clone()), _scope: None, _marker: std::marker::PhantomData };
                let decision = crate::legacy::pokemon_content::PokemonMobaInputAi.think(&mut ctx, None);
                match mode {
                    0 => assert!(matches!(decision, PlayerInputDecision::Replace(Input::Attack { .. }
                        | Input::Skill { .. } | Input::Skill2 { .. } | Input::Ult { .. }))),
                    1 => assert!(matches!(decision, PlayerInputDecision::Replace(Input::Move { .. }))),
                    _ => assert!(matches!(decision, PlayerInputDecision::Pass)),
                }
            }
        });
    }

    #[test]
    fn ai_readiness_queries_never_refill_authoritative_charges() {
        use crate::legacy::pokemon_status as status;
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            status::register_player_entity_life_at_tick(&ctx, 1, 7, 0, Some(0));
            status::register_will_o_wisp_charges(7);
            status::register_sticky_web_charges(7);
            for _ in 0..3 { assert!(status::consume_will_o_wisp_charge(&ctx, 7, 3, 420)); }
            for _ in 0..5 { assert!(status::consume_sticky_web_charge(&ctx, 7, 5, 1800)); }
            assert!(!status::will_o_wisp_ready_for_player(1, 419, 3, 420));
            assert!(status::will_o_wisp_ready_for_player(1, 420, 3, 420));
            assert!(!status::will_o_wisp_ready_for_player(1, 0, 3, 420));
            assert!(!status::consume_will_o_wisp_charge(&ctx, 7, 3, 420));
            assert!(!status::sticky_web_ready_for_player(1, 1799, 5, 1800));
            assert!(status::sticky_web_ready_for_player(1, 1800, 5, 1800));
            assert!(!status::sticky_web_ready_for_player(1, 0, 5, 1800));
            assert!(!status::consume_sticky_web_charge(&ctx, 7, 5, 1800));
        });
    }

    #[test]
    fn fog_visibility_does_not_bypass_hunt_and_stance_gates_only_the_dash() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            {
                let ctx = GameCtx::from_stable(sim);
                status::begin_weavile_hunt_stealth(&ctx, 8, 120);
            }
            let mut world = crate::ai_world::World::capture(sim, 7);
            assert!(world.get(8).unwrap().visible, "mock fog visibility is deliberately true");
            assert!(status::ai_soft_untargetable(&world, 8));
            {
                let ctx = GameCtx::from_stable(sim);
                status::break_weavile_hunt_stealth(&ctx, 8);
            }
            assert!(!status::ai_soft_untargetable(&world, 8));
            world.units.iter_mut().find(|unit| unit.id == 8).unwrap().champion = true;
            let stance = content::champion_by_id("pokemon_moba_sawk_throh").unwrap();
            assert!(content::ai_target_allowed(&world, stance, stance.ult.unwrap(), 8));
            let mut tower = world.get(8).unwrap().clone();
            tower.id = 10; tower.tower = true; tower.champion = false; tower.pos.x = 190000;
            world.units.push(tower);
            assert!(!content::ai_target_allowed(&world, stance, stance.ult.unwrap(), 8));
            {
                let mut ctx = GameCtx::from_stable(sim);
                status::toggle_sawk_throh_form(&mut ctx, 7);
            }
            assert!(content::ai_target_allowed(&world, stance, stance.ult.unwrap(), 8), "Throh's throw does not dash the caster");
        });
    }

    #[test]
    fn attachments_follow_exact_positions_without_forced_movement_or_oscillation() {
        use crate::legacy::pokemon_status as status;
        for update in [status::update_flower_veil_passive, status::update_clingy_passive] {
            let mut calls = with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
                update(&mut GameCtx::from_stable(sim), 7);
            });
            assert!(calls.teleports.is_empty());
            calls.tick = 90;
            calls = with_world(calls, |sim| update(&mut GameCtx::from_stable(sim), 7));
            assert_eq!(calls.teleports, vec![(7, 140000, 100000)]);
            // An ordinary step, followed by an instantaneous host relocation to base.
            for (tick, position) in [(91, (145000, 100000)), (92, (900000, 900000))] {
                calls.tick = tick;
                calls.positions.insert(9, position);
                calls = with_world(calls, |sim| update(&mut GameCtx::from_stable(sim), 7));
                assert_eq!(calls.positions[&7], position);
            }
            for tick in 93..105 {
                calls.tick = tick;
                calls = with_world(calls, |sim| update(&mut GameCtx::from_stable(sim), 7));
            }
            assert_eq!(calls.teleports.len(), 3, "stationary hosts must not cause repeated jumps");
            assert!(calls.cc.iter().all(|(_, cc)| cc.kind == CcKindV1::Bind as u32));
            assert!(calls.clear_cc.is_empty(), "do not erase enemy CC to implement a tether");
        }
    }

    #[test]
    fn attachments_detach_for_recall_and_cannot_reattach_during_return_window() {
        use crate::legacy::pokemon_status as status;
        for update in [status::update_flower_veil_passive, status::update_clingy_passive] {
            for returning_entity in [7, 9] {
                let mut calls = with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
                    let mut ctx = GameCtx::from_stable(sim);
                    status::register_player_entity_life_at_tick(&ctx, 1, 7, 0, Some(0));
                    status::register_player_entity_life_at_tick(&ctx, 2, 9, 0, Some(0));
                    update(&mut ctx, 7);
                });
                calls.tick = 90;
                calls = with_world(calls, |sim| update(&mut GameCtx::from_stable(sim), 7));
                calls.tick = 91;
                calls = with_world(calls, |sim| {
                    let mut ctx = GameCtx::from_stable(sim);
                    status::note_player_return_input(if returning_entity == 7 { 1 } else { 2 }, 91);
                    update(&mut ctx, 7);
                    assert!(!status::is_comfey_attached_at(7, 91));
                    assert!(!status::is_clawitzer_attached_at(7, 91));
                });
                // Comfey's five-second reattach delay expires before the recall guard.
                calls.tick = 392;
                calls = with_world(calls, |sim| {
                    update(&mut GameCtx::from_stable(sim), 7);
                    assert!(!status::is_comfey_attached_at(7, 392));
                    assert!(!status::attachment_candidate_allowed_at(7, 9, 392));
                });
                assert_eq!(calls.teleports.len(), 1);
                calls.tick = 992;
                with_world(calls, |sim| {
                    update(&mut GameCtx::from_stable(sim), 7);
                    assert!(status::is_comfey_attached_at(7, 992) || status::is_clawitzer_attached_at(7, 992));
                });
            }
        }
    }

    #[test]
    fn attachments_reject_cycles_and_clear_links_between_lives() {
        use crate::legacy::pokemon_status as status;
        let mut calls = with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::update_flower_veil_passive(&mut ctx, 7);
            status::update_clingy_passive(&mut ctx, 9);
        });
        calls.tick = 90;
        calls = with_world(calls, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::update_flower_veil_passive(&mut ctx, 7);
            status::update_clingy_passive(&mut ctx, 9);
            assert_eq!(status::comfey_attached_ally(&ctx, 7), Some(9));
            assert!(!status::is_clawitzer_attached_at(9, 90));
            assert!(!status::attachment_candidate_allowed_at(9, 7, 90));
            status::reset_attachment_life(&ctx, 9);
            assert_eq!(status::comfey_attached_ally(&ctx, 7), None);
            assert!(!status::is_comfey_attach_ready_at(7, 90));
        });
        calls.tick = 400;
        calls = with_world(calls, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::update_flower_veil_passive(&mut ctx, 7);
            assert_eq!(status::comfey_attached_ally(&ctx, 7), Some(9));
        });
        calls.tick = 401;
        calls.dead.insert(7);
        calls = with_world(calls, |sim| {
            status::update_flower_veil_passive(&mut GameCtx::from_stable(sim), 7);
            assert!(!status::is_comfey_attached_at(7, 401));
        });
        calls.dead.clear();
        calls.positions.insert(7, (900000, 900000));
        calls.tick = 500;
        let before = calls.teleports.len();
        calls = with_world(calls, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::reset_attachment_life(&ctx, 7);
            status::update_flower_veil_passive(&mut ctx, 7);
            assert!(!status::is_comfey_attached_at(7, 500));
        });
        assert_eq!(calls.teleports.len(), before, "respawn must not jump back to the old host");
    }

    #[test]
    fn attachment_partner_ai_preserves_recall_and_skips_returning_hosts() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        for name in ["Comfey", "Clawitzer"] {
            let mut initial = Calls { world: true, ally_nine: true, ..Default::default() };
            initial.positions.insert(9, (400000, 100000));
            let mut calls = with_world(initial, |sim| {
                let mut ctx = GameCtx::from_stable(sim);
                status::register_player_entity_life_at_tick(&ctx, 1, 7, 0, Some(0));
                status::register_player_entity_life_at_tick(&ctx, 2, 9, 0, Some(0));
                status::update_flower_veil_passive(&mut ctx, 7);
                status::update_clingy_passive(&mut ctx, 7);
            });
            calls.tick = 90;
            with_world(calls, |sim| {
                let ctx = GameCtx::from_stable(sim);
                content::update_ai_entity_snapshot(&ctx, 1, 7);
                content::update_ai_entity_snapshot(&ctx, 2, 9);
                assert!(matches!(content::attach_partner_move_input_for_player(1, 90, name, None), Some(Input::Move { .. })));
                assert!(content::attach_partner_move_input_for_player(1, 90, name, Some(&Input::Return)).is_none());
                status::note_player_return_input(2, 90);
                assert!(content::attach_partner_move_input_for_player(1, 90, name, None).is_none());
            });
        }
    }

    #[test]
    fn attached_recall_is_blocked_even_when_retreat_helpers_have_no_valid_input() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        unsafe extern "C" fn player(_: *const c_void) -> usize { 1 }
        unsafe extern "C" fn tick(_: *const c_void) -> usize { 90 }
        for (name, update) in [("Comfey", status::update_flower_veil_passive as fn(&mut GameCtx, usize)),
            ("Clawitzer", status::update_clingy_passive as fn(&mut GameCtx, usize))] {
            let mut calls = with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
                let mut ctx = GameCtx::from_stable(sim);
                status::register_player_entity_life_at_tick(&ctx, 1, 7, 0, Some(0));
                update(&mut ctx, 7);
            });
            calls.tick = 90;
            with_world(calls, |sim| {
                update(&mut GameCtx::from_stable(sim), 7);
                let world = crate::ai_world::World::capture(sim, 7);
                let mut table: mod_api_stable::AiVtableV1 = unsafe { std::mem::zeroed() };
                table.size = size_of::<mod_api_stable::AiVtableV1>();
                table.player_id = Some(player);
                table.tick = Some(tick);
                let mut raw: mod_api_stable::AiCtxV1 = unsafe { std::mem::zeroed() };
                raw.size = size_of::<mod_api_stable::AiCtxV1>();
                raw.vtable = &table;
                let mut inner = unsafe { StableAiContext::from_raw(&mut raw) }.unwrap();
                let mut ctx = PlayerAiContext { inner: &mut inner, champion_name: name.into(),
                    world: Some(world.clone()), _scope: None, _marker: std::marker::PhantomData };
                let decision = content::PokemonMobaInputAi.think(&mut ctx, Some(Input::Return));
                assert!(matches!(decision, PlayerInputDecision::Block));
                let output = finish_ai_decision(Some(&world), name, Some(Input::Return.to_stable()), decision).unwrap();
                assert_eq!(output.kind, InputKindV1::Move.code(), "None would preserve the built-in recall");
            });
        }
    }

    #[test]
    fn intended_teleports_use_positions_and_normal_dashes_do_not() {
        let mut registration = ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        for (id, slot, target, expected) in [("pokemon_moba_banette", 3, 8, 1),
            ("pokemon_moba_skarmory", 3, 9, 1), ("pokemon_moba_audino", 2, 9, 2),
            ("pokemon_moba_pikachu", 1, 8, 0)] {
            let champion = registration.champions.iter().find(|champion| champion.id() == id).unwrap();
            let action = match slot { 1 => champion.skill(), 2 => champion.skill2(), _ => champion.ult().unwrap() };
            let calls = with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
                action.effect().unwrap().effect.apply(sim, 1, 7, InputTargetV1::target(target));
            });
            assert_eq!(calls.teleports.len(), expected, "{id}");
        }
    }

    #[test]
    fn receiver_copies_credited_cs_gold_without_duplicate_payouts() {
        use crate::legacy::pokemon_status as status;
        let mut registration = ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        let mut passive = registration.champions.iter().find(|c| c.id() == "pokemon_moba_passimian").unwrap().passive().unwrap();
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            {
                let mut ctx = GameCtx::from_stable(sim);
                status::register_receiver(7);
                status::register_entity_champion(&ctx, 8, "pokemon_moba_gholdengo");
                status::copy_receiver_passive(&mut ctx, 7, 8);
                assert!(status::receiver_has_copied(7, "pokemon_moba_gholdengo"));
            }
            for _ in 0..1000 { passive.on_unit_kill(sim, 1, 7, 9); }
        });
        assert_eq!(calls.gold, vec![(1, 10)]);
    }

    #[test]
    fn stronger_antiheal_expiry_restores_the_weaker_source_without_stacking() {
        use crate::legacy::pokemon_status as status;
        let mut calls = with_world(Calls { world: true, hp_current: Some(9000), ..Default::default() }, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            status::apply_anti_heal(&mut ctx, 8, 7, 25, 120);
            status::apply_anti_heal(&mut ctx, 9, 7, 50, 60);
        });
        calls.tick = 61;
        let calls = with_world(calls, |sim| {
            let mut ctx = GameCtx::from_stable(sim);
            assert_eq!(status::heal_with_antiheal(&mut ctx, 7, 7, 400), 300);
        });
        assert_eq!(calls.active_buffs.get(&(7, "pokemon_moba_anti_heal".into())).unwrap().last().unwrap().heal_reduce, 25);
    }

    #[test]
    fn roster_ai_gates_handle_all_actions_and_reject_tower_dives_and_empty_charges() {
        use crate::legacy::pokemon_content as content;
        with_world(Calls { world: true, ally_nine: true, ..Default::default() }, |sim| {
            let mut world = crate::ai_world::World::capture(sim, 7);
            let mut registration = ModRegistration::new("pokemon_moba_stable");
            content::register_champions(&mut registration);
            for champion in registration.champions {
                for action in [Some(champion.attack()), Some(champion.skill()), Some(champion.skill2()), champion.ult()].into_iter().flatten() {
                    action.can_activate(sim, 7);
                    action.target_constraint(sim, 7, 8);
                    assert_eq!(action.effect().unwrap().effect.may_adjust_range(), champion.id() == "pokemon_moba_smeargle");
                }
            }
            let pikachu = content::champion_by_id("pokemon_moba_pikachu").unwrap();
            assert!(content::ai_target_allowed(&world, pikachu, pikachu.skill, 8));
            let mut tower = world.get(8).unwrap().clone();
            tower.id = 10; tower.tower = true; tower.champion = false;
            // Start outside the tower, destination inside it.
            tower.pos.x = 190000;
            world.units.push(tower);
            assert!(!content::ai_target_allowed(&world, pikachu, pikachu.skill, 8));
            let base = Some(InputV1::action(InputKindV1::Skill, InputTargetV1::target(8)));
            for decision in [PlayerInputDecision::Pass, PlayerInputDecision::Block] {
                let held = finish_ai_decision(Some(&world), "Pikachu", base, decision).expect("None would keep the dangerous cast");
                assert_eq!(held.kind, InputKindV1::Move.code());
                assert_eq!((held.x, held.y), (100000, 100000));
            }
            world.units.retain(|unit| unit.id != 10);
            world.units.iter_mut().find(|unit| unit.id == 7).unwrap().cooldowns[1] = Some(mod_api_stable::CooldownV1 {
                unlocked: true, charges: 0, ..Default::default()
            });
            assert!(!content::ai_can_activate(&world, pikachu, pikachu.skill));
        });
    }

    #[test]
    fn sketch_memory_and_cooldowns_follow_respawn_but_not_other_players_or_matches() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        with_world(Calls { world: true, tick: 100, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            status::register_player_entity_life_at_tick(&ctx, 1, 7, 100, Some(0));
            status::register_smeargle(&ctx, 7);
            let source = content::champion_by_id("pokemon_moba_xatu").unwrap();
            assert!(status::smeargle_force_learn_for_player(&ctx, 1, source, source.ult.unwrap()));
            status::smeargle_start_copied_slot_cooldown(&ctx, 7, content::ActionSlot::Attack);
            status::register_player_entity_life_at_tick(&ctx, 1, 9, 200, Some(1));
            status::register_smeargle(&ctx, 9);
            assert_eq!(status::smeargle_learned_count_for_player(1), 1);
            assert_eq!(status::smeargle_learned_action(&ctx, 9, content::ActionSlot::Attack).unwrap().source_champion.id, source.id);
            assert!(status::smeargle_learned_action(&ctx, 7, content::ActionSlot::Attack).is_none());
            assert!(!status::smeargle_copied_slot_ready(&ctx, 9, content::ActionSlot::Attack));
            assert_eq!(ctx.get_entity(9).unwrap().buff_count(), 1);
            status::register_player_entity_life_at_tick(&ctx, 2, 8, 200, Some(0));
            status::register_smeargle(&ctx, 8);
            assert_eq!(status::smeargle_learned_count_for_player(2), 0);
        });
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            let ctx = GameCtx::from_stable(sim);
            status::register_player_entity_life_at_tick(&ctx, 1, 9, 0, Some(0));
            status::register_smeargle(&ctx, 9);
            assert_eq!(status::smeargle_learned_count_for_player(1), 0);
        });
    }

    #[test]
    fn sketch_seeks_contact_after_fallback_without_interrupting_recalls_or_diving() {
        use crate::legacy::pokemon_content::smeargle_learning_approach as approach;
        with_world(Calls { world: true, ..Default::default() }, |sim| {
            let mut world = crate::ai_world::World::capture(sim, 7);
            world.units.retain(|unit| unit.id != 9);
            let enemy = world.units.iter_mut().find(|unit| unit.id == 8).unwrap();
            enemy.champion = true;
            enemy.pos = EntityPos { x: 180000, y: 100000 };
            assert!(matches!(approach(&world, None, 1), Some(Input::Move { x: 132000, y: 100000 })));
            assert!(approach(&world, Some(&Input::Return), 1).is_none());
            assert!(approach(&world, Some(&Input::Attack { target: InputTarget::Target { target_id: 8 } }), 1).is_none());
            assert!(approach(&world, None, 4).is_none());
            world.units.iter_mut().find(|unit| unit.id == 8).unwrap().visible = false;
            assert!(approach(&world, None, 1).is_none());
            world.units.iter_mut().find(|unit| unit.id == 8).unwrap().visible = true;
            let mut extra = world.get(8).unwrap().clone();
            extra.id = 10;
            world.units.push(extra);
            assert!(approach(&world, None, 1).is_none());
            let extra = world.units.last_mut().unwrap();
            extra.champion = false; extra.tower = true;
            assert!(approach(&world, None, 1).is_none());
            world.units.pop();
            world.units.iter_mut().find(|unit| unit.id == 7).unwrap().hp = 1;
            assert!(approach(&world, None, 1).is_none());
        });
    }

    #[test]
    fn sketch_range_tracks_learning_and_restores_metadata_without_stacking() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            let xatu = content::champion_by_id("pokemon_moba_xatu").unwrap();
            let move_data = xatu.ult.unwrap();
            assert_eq!(move_data.range, 900000);
            {
                let ctx = GameCtx::from_stable(sim);
                status::register_player_entity_life_at_tick(&ctx, 1, 7, 0, Some(0));
                status::register_smeargle(&ctx, 7);
                assert!(status::smeargle_force_learn_for_player(&ctx, 1, xatu, move_data));
                assert!(!status::smeargle_force_learn_for_player(&ctx, 1, xatu, move_data));
            }
            let marker = sim.get_entity(7).unwrap().buff_at(0).unwrap();
            assert_eq!(sim.get_entity(7).unwrap().buff_count(), 1);
            crate::sketch_range::tests::with_view(vec![marker], 1, |view| {
                let mut registration = ModRegistration::new("pokemon_moba_stable");
                content::register_champions(&mut registration);
                for champion in &registration.champions {
                    for (slot, action) in [Some(champion.attack()), Some(champion.skill()), Some(champion.skill2()), champion.ult()].into_iter().enumerate() {
                        let Some(action) = action else { continue; };
                        let spec = action.effect().unwrap();
                        let expected = if champion.id() == "pokemon_moba_smeargle" && slot == 0 { 858000 } else { 0 };
                        assert_eq!(spec.effect.range_adjust(view, view), expected, "{} slot {slot}", champion.id());
                    }
                }
            });
            // Respawn/cleanse can remove engine buffs; learned moves survive and restore them.
            sim.entity_remove_buff(7, marker.name());
            assert_eq!(sim.get_entity(7).unwrap().buff_count(), 0);
            {
                let ctx = GameCtx::from_stable(sim);
                status::register_smeargle(&ctx, 7);
                for _ in 0..100 { status::smeargle_sync_ranges(&ctx, 7); }
                let source = content::champion_by_id("pokemon_moba_audino").unwrap();
                status::note_smeargle_affected(&ctx, 7, source, source.skill2);
                assert!(status::smeargle_try_sketch(&ctx, 7, content::ActionSlot::Skill));
            }
            assert_eq!(sim.get_entity(7).unwrap().buff_count(), 2);
            assert_eq!(sim.get_entity(8).unwrap().buff_count(), 0);
        });
        // First learning, restoring a removed marker, second learning. No tick-refresh writes.
        assert_eq!(calls.visual_buffs.len(), 3);
        assert!(calls.damage.is_empty());
        assert!(calls.visuals.is_empty());
    }

    #[test]
    fn sketch_range_metadata_replaces_stale_entries_and_keeps_other_buffs() {
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            sim.add_buff(7, &BuffV1::named("unrelated_passive"));
            crate::sketch_range::sync(sim, 7, [Some((80000, 0)), None, None, None]);
            crate::sketch_range::sync(sim, 7, [Some((60000, 0)), None, None, None]);
            assert_eq!(sim.get_entity(7).unwrap().buff_count(), 2);
            crate::sketch_range::sync(sim, 7, [None; 4]);
            let entity = sim.get_entity(7).unwrap();
            assert_eq!(entity.buff_count(), 1);
            assert_eq!(entity.buff_at(0).unwrap().name(), "unrelated_passive");
        });
        assert_eq!(calls.visual_buffs.len(), 2);
    }

    #[test]
    fn copied_heals_use_ally_eligibility_and_real_range_without_changing_smeargles_body() {
        use crate::legacy::{pokemon_content as content, pokemon_status as status};
        with_world(Calls { world: true, ally_nine: true, hp_current: Some(9000), ..Default::default() }, |sim| {
            {
                let ctx = GameCtx::from_stable(sim);
                status::register_player_entity_life_at_tick(&ctx, 1, 7, 0, Some(0));
                status::register_smeargle(&ctx, 7);
                let source = content::champion_by_id("pokemon_moba_audino").unwrap();
                assert!(status::smeargle_force_learn_for_player(&ctx, 1, source, source.skill2));
            }
            let mut world = crate::ai_world::World::capture(sim, 7);
            let smeargle = content::champion_by_id("pokemon_moba_smeargle").unwrap();
            assert!(content::ai_target_allowed(&world, smeargle, smeargle.attack, 9));
            assert!(!content::ai_target_allowed(&world, smeargle, smeargle.attack, 8));
            world.units.iter_mut().find(|unit| unit.id == 9).unwrap().pos.x += 300000;
            assert!(!content::ai_target_allowed(&world, smeargle, smeargle.attack, 9));
        });
    }

    #[test]
    fn gholdengo_rewards_credited_cs_once_without_wallet_polling() {
        let mut registration = ModRegistration::new("pokemon_moba_stable");
        crate::legacy::pokemon_content::register_champions(&mut registration);
        let champion = registration.champions.iter().find(|c| c.id() == "pokemon_moba_gholdengo").unwrap();
        let mut passive = champion.passive().unwrap();
        let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
            passive.on_unit_kill(sim, 1, 9, 8); // Wrong owner cannot consume the event.
            for _ in 0..1000 { passive.on_unit_kill(sim, 1, 7, 8); }
            passive.on_unit_kill(sim, 1, 7, 9); // Two CS in the same tick.
            passive.on_kill(sim, 1, 7, 8); // Not a second CS callback.
        });
        assert_eq!(calls.gold, vec![(1, 10), (1, 10)]);
    }

    #[test]
    fn projectiles_defer_damage_and_piercing_hits_resolve_only_the_collided_target() {
        use crate::legacy::pokemon_content::{POKEMON_CHAMPIONS, resolve_projectile_impact};
        for (id, expected_pierce) in [("pokemon_moba_pikachu", false), ("pokemon_moba_vaporeon", true)] {
            let mut registration = ModRegistration::new("pokemon_moba_stable");
            crate::legacy::pokemon_content::register_champions(&mut registration);
            let champion = registration.champions.iter().find(|c| c.id() == id).unwrap();
            let action = champion.attack().effect().unwrap();
            let data = *POKEMON_CHAMPIONS.iter().find(|c| c.id == id).unwrap();
            let launched = with_world(Calls { world: true, ..Default::default() }, |sim| {
                action.effect.apply(sim, 42, 7, InputTarget::Target { target_id: 8 }.to_stable());
            });
            assert!(launched.damage.is_empty(), "{id} dealt damage before impact");
            assert_eq!(launched.projectiles.len(), 1, "{id}");
            assert_eq!(launched.projectiles[0].2.penetrate, expected_pierce);
            assert_eq!(launched.projectiles[0].2.caster_id, 7);
            assert!(launched.projectiles[0].1.contains(":impact:"));
            let landed = with_world(Calls { world: true, ..Default::default() }, |sim| {
                resolve_projectile_impact(&mut GameCtx::from_stable(sim), data, data.attack,
                    42, 7, InputTarget::Target { target_id: 8 });
            });
            assert_eq!(landed.damage.len(), 1, "{id}: {:?}", landed.damage);
            assert_eq!(landed.damage[0].1, 8);
            assert!(landed.projectiles.is_empty(), "impact must not launch again");
        }
    }

    #[test]
    fn all_staged_action_projectiles_defer_resolution() {
        use crate::legacy::pokemon_content::{POKEMON_CHAMPIONS, PokemonMoveEffect as E};
        let mut tested = 0;
        for data in POKEMON_CHAMPIONS {
            for action in [Some(data.attack), Some(data.skill), Some(data.skill2), data.ult].into_iter().flatten() {
                if crate::native_vfx::projectile_name(data.id, action.slot.index(), 0).is_none()
                    || matches!(action.effect, E::MarowakBonemerang { .. } | E::MagmortarEruption { .. }) { continue; }
                let calls = with_world(Calls { world: true, ..Default::default() }, |sim| {
                    assert!(crate::projectiles::launch(&GameCtx::from_stable(sim), data, action, 7,
                        InputTarget::Target { target_id: 8 }), "{} {:?}", data.id, action.slot);
                });
                assert_eq!(calls.projectiles.len(), if matches!(action.effect,E::TripleArcDamage{..}) {3} else {1});
                assert!(calls.damage.is_empty());
                if matches!(action.effect, E::AreaMiasmaSlow { .. }) {
                    assert_eq!(calls.queued.len(), 1);
                    assert_eq!(calls.queued[0].3, 4); // 20,000 units / 5,000 per tick.
                    assert_eq!(calls.queued[0].2.x, 120000);
                } else {
                    let count=match action.effect {
                        E::MultiHitDamage{hits,..}=>hits,
                        E::CloysterIcicleCrash{hits_1,..}=>hits_1,
                        _=>1,
                    };
                    assert_eq!(calls.queued.len(),count-1,"{} {:?}",data.id,action.slot);
                    assert!(calls.queued.windows(2).all(|w|w[0].3<w[1].3));
                    assert_ne!(calls.projectiles[0].1, crate::native_vfx::VISUAL_HIT);
                }
                tested += 1;
            }
        }
        assert_eq!(tested, 72); // Independent Leafage fan added to the previous projectile roster.
        let bone = with_world(Calls { world: true, ..Default::default() }, |sim| {
            assert!(crate::projectiles::launch_bonemerang(&GameCtx::from_stable(sim), 7,
                EntityPos { x: 100000, y: 100000 }, EntityPos { x: 130000, y: 100000 }, true));
        });
        assert_eq!(bone.projectiles[0].2.speed, 7500);
        assert!(bone.projectiles[0].2.penetrate);
        assert_eq!(bone.queued[0].3, 4);
        assert!(bone.damage.is_empty());
    }

    #[test]
    fn ordinary_and_special_crit_paths_have_distinct_owners() {
        use crate::legacy::pokemon_content::{POKEMON_CHAMPIONS, resolve_projectile_impact};
        for (id, expected_engine_chance) in [("pokemon_moba_pikachu", 100), ("pokemon_moba_inteleon", 0)] {
            let data = *POKEMON_CHAMPIONS.iter().find(|c| c.id == id).unwrap();
            let calls = with_world(Calls { world: true, crit: 100, ..Default::default() }, |sim| {
                resolve_projectile_impact(&mut GameCtx::from_stable(sim), data, data.attack,
                    42, 7, InputTarget::Target { target_id: 8 });
            });
            assert_eq!(calls.crit_at_damage, vec![(expected_engine_chance, false)], "{id}");
            assert!(calls.active_buffs.is_empty(), "crit suppression must not persist");
        }
    }

    #[test]
    fn visuals_preserve_world_coordinates_names_and_do_not_damage() {
        let long_name = "pokemon_moba_stable_magmortar_skill1_temper_flare_projectile_splash";
        let calls = with_sim(|sim| {
            let ctx = GameCtx::from_stable(sim);
            assert!(ctx.play_view_effect("web", 17, InputTarget::Pos { x: 123000, y: 456000 }, 15000, 12000, 30));
            let spec = mod_api_stable::ProjectileSpawnV1 { caster_id: 17, x: 12000, y: 34000, target_id: 29,
                ..Default::default() };
            assert!(ctx.spawn_visual_projectile(long_name, &spec));
            ctx.visual_buff(29, "pokemon_moba_stable_sigilyph_passive_glypher_mark", 30);
        });
        assert!(calls.damage.is_empty());
        assert_eq!(calls.visuals[0].1, 17);
        assert_eq!((calls.visuals[0].2.x, calls.visuals[0].2.y), (123000, 456000));
        assert_eq!(calls.visuals[0].5, 30);
        assert_eq!(calls.projectiles[0].0, long_name);
        assert_eq!(calls.projectiles[0].1, crate::native_vfx::VISUAL_HIT);
        assert_eq!(calls.projectiles[0].2.target_id, 29);
        assert_eq!(calls.visual_buffs[0].2, 1);
        assert_eq!(calls.visual_buffs[0].3, 1);
    }

    #[test]
    fn combat_uses_engine_pipeline_once_and_forwards_real_shields() {
        let calls = with_sim(|sim| {
            let mut ctx = GameCtx::from_stable(sim);
            ctx.deal_damage(7, 8, 100, 40, AttackType::Skill);
            ctx.add_buff(
                8,
                BuffState {
                    duration: BuffType::WithShield {
                        tick: 180,
                        shield: 250,
                    },
                    ..Default::default()
                },
            );
            ctx.add_buff(8, BuffState::default());
        });
        assert_eq!(
            calls.damage,
            vec![(7, 8, 100, 40, AttackTypeV1::Skill.code())]
        );
        assert_eq!(calls.shields, vec![(8, 250, 180)]);
        assert_eq!(calls.buffs, 2);
    }

    #[derive(Clone)]
    struct VictimProbe(Arc<Mutex<Vec<usize>>>);
    impl ModPassive for VictimProbe {
        fn clone_box(&self) -> Box<dyn ModPassive> {
            Box::new(self.clone())
        }
        fn on_kill(&mut self, _ctx: &mut GameCtx, _player: usize, victim: usize) {
            self.0.lock().unwrap().push(victim);
        }
        fn on_unit_kill(
            &mut self,
            _ctx: &mut GameCtx,
            _player: usize,
            _owner: usize,
            victim: usize,
        ) {
            self.0.lock().unwrap().push(victim);
        }
    }

    #[test]
    fn kill_callbacks_preserve_victim_instead_of_using_owner() {
        let victims = Arc::new(Mutex::new(Vec::new()));
        let mut passive = LegacyPassive {
            inner: Box::new(VictimProbe(victims.clone())),
        };
        with_sim(|sim| {
            passive.on_kill(sim, 1, 12, 55);
            passive.on_unit_kill(sim, 1, 12, 89);
        });
        assert_eq!(*victims.lock().unwrap(), vec![55, 89]);
    }
}
