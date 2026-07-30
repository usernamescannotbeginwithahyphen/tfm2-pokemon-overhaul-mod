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
    fn to_stable(self) -> AttackTypeV1 {
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
    Airborne { tick: u64 },
    Stun { tick: u64 },
    Bind { tick: u64 },
    BlockAttack { tick: usize },
    BlockSkill { tick: usize },
    BlockMoveSkill { tick: usize },
    ForceMove { tick: u64, dx: i64, dy: i64, speed: u64 },
    Taunt { tick: u64, target: usize },
    Fear { tick: u64, dx: i64, dy: i64 },
    Charm { tick: u64, dx: i64, dy: i64 },
    Animation { tick: u64 },
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
            Self::ForceMove { tick, dx, dy, speed } => {
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

    fn from_stable(input: InputTargetV1) -> Self {
        if input.kind == InputTargetKindV1::Target.code() {
            Self::Target { target_id: input.target_id }
        } else if input.kind == InputTargetKindV1::Pos.code() {
            Self::Pos { x: input.x, y: input.y }
        } else if input.kind == InputTargetKindV1::Dir.code() {
            Self::Dir { x: input.dir_x, y: input.dir_y }
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
            Self::Attack { target: InputTarget::from_stable(input.target) }
        } else if input.kind == InputKindV1::Skill.code() {
            Self::Skill { target: InputTarget::from_stable(input.target) }
        } else if input.kind == InputKindV1::Skill2.code() {
            Self::Skill2 { target: InputTarget::from_stable(input.target) }
        } else if input.kind == InputKindV1::Ult.code() {
            Self::Ult { target: InputTarget::from_stable(input.target) }
        } else if input.kind == InputKindV1::Return.code() {
            Self::Return
        } else {
            Self::Move { x: input.x, y: input.y }
        }
    }
}

pub enum PlayerInputDecision {
    Pass,
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
            position: init.lane.map(Position::from_stable).unwrap_or(Position::Top),
            champion_name: init.champion_name.clone(),
        }
    }
}

pub struct PlayerAiContext<'a, 'b, 'c> {
    inner: &'a mut StableAiContext<'b>,
    champion_name: String,
    _marker: std::marker::PhantomData<&'c ()>,
}

impl<'a, 'b, 'c> PlayerAiContext<'a, 'b, 'c> {
    fn from_stable(inner: &'a mut StableAiContext<'b>) -> Self {
        let champion_name = inner.champion_name().unwrap_or_default();
        Self { inner, champion_name, _marker: std::marker::PhantomData }
    }

    pub fn player_id(&self) -> usize { self.inner.player_id() }
    pub fn athlete_id(&self) -> usize { self.inner.athlete_id() }
    pub fn team(&self) -> usize { self.inner.team() }
    pub fn position(&self) -> Position {
        self.inner.lane().map(Position::from_stable).unwrap_or(Position::Top)
    }
    pub fn champion_name(&self) -> &str { &self.champion_name }
    pub fn tick(&self) -> usize { self.inner.tick() }
    pub fn hp_ratio_percent(&self) -> Option<usize> { self.inner.hp_ratio_percent() }

    pub fn is_valid_input(&self, input: &Input) -> bool {
        self.inner.is_valid_input(&input.to_stable())
    }

    pub fn get_run_away_input(&mut self) -> Option<Input> {
        self.inner.run_away_input().map(Input::from_stable)
    }

    pub fn get_run_away_without_skill_input(&mut self) -> Option<Input> {
        self.inner.run_away_without_skill_input().map(Input::from_stable)
    }
}

pub trait ModPlayerInputAi: Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn ModPlayerInputAi>;
    fn id(&self) -> &str;
    fn priority(&self) -> i32 { 0 }
    fn matches(&self, _ctx: &PlayerAiInitContext) -> bool { false }
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
        Self { inner: Box::new(inner) }
    }
}

impl StablePlayerAi for LegacyPlayerAi {
    fn clone_box(&self) -> Box<dyn StablePlayerAi> {
        Box::new(Self { inner: self.inner.clone_box() })
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

    fn think(&mut self, ctx: &mut StableAiContext<'_>, base_input: Option<InputV1>) -> Option<InputV1> {
        let mut legacy_ctx = PlayerAiContext::from_stable(ctx);
        match self.inner.think(&mut legacy_ctx, base_input.map(Input::from_stable)) {
            PlayerInputDecision::Pass => base_input,
            PlayerInputDecision::Replace(input) => Some(input.to_stable()),
        }
    }
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
    fn priority(&self) -> i32 { 0 }
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
        self.inner
            .score_ban(&DraftScoreContext::from_stable(ctx), candidate, base_score)
            .to_stable()
    }

    fn score_pick(
        &self,
        ctx: &StableDraftContext<'_>,
        candidate: usize,
        base_score: f32,
    ) -> StableDraftDecision {
        self.inner
            .score_pick(&DraftScoreContext::from_stable(ctx), candidate, base_score)
            .to_stable()
    }
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
    fn description(&self) -> String {
        String::new()
    }
}

pub trait ModEffectType: Send + Sync + 'static {
    fn apply(&self, ctx: &mut GameCtx, rng_seed: u64, caster_id: usize, input: InputTarget);
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
    fn on_update(&mut self, _ctx: &mut GameCtx, _rng_seed: u64, _player: usize, _entity: usize) {}
    fn on_base_attack(&mut self, _ctx: &mut GameCtx, _rng_seed: u64, _player: usize, _entity: usize) {}
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
        self.champions.push(LegacyChampion { inner: Box::new(champion) });
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

impl StableChampion for LegacyChampion {
    fn id(&self) -> String {
        self.inner.id().to_string()
    }

    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    fn skill_icon(&self, skill_index: usize) -> (String, String) {
        let (source, tag) = self.inner.skill_icon(skill_index);
        (source.replace("asset/pokemon_moba/", "asset/pokemon_moba_stable/"), tag)
    }

    fn category(&self) -> ChampionCategoryV1 {
        self.inner.category().to_stable()
    }

    fn tags(&self) -> Vec<ChampionTagV1> {
        self.inner.tags().into_iter().map(ChampionTag::to_stable).collect()
    }

    fn stat(&self) -> StatV1 {
        self.inner.stat()
    }

    fn growth(&self) -> StatV1 {
        self.inner.growth()
    }

    fn attack(&self) -> Box<dyn StableAction> {
        Box::new(LegacyAction { inner: self.inner.attack() })
    }

    fn skill(&self) -> Box<dyn StableAction> {
        Box::new(LegacyAction { inner: self.inner.skill() })
    }

    fn skill2(&self) -> Box<dyn StableAction> {
        Box::new(LegacyAction { inner: self.inner.skill2() })
    }

    fn ult(&self) -> Option<Box<dyn StableAction>> {
        self.inner.ult().map(|inner| Box::new(LegacyAction { inner }) as Box<dyn StableAction>)
    }

    fn passive(&self) -> Option<Box<dyn StablePassive>> {
        self.inner.passive().map(|inner| Box::new(LegacyPassive { inner }) as Box<dyn StablePassive>)
    }
}

struct LegacyAction {
    inner: Box<dyn ModAction>,
}

impl StableAction for LegacyAction {
    fn clone_box(&self) -> Box<dyn StableAction> {
        Box::new(Self { inner: self.inner.clone_box() })
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
            effect: Box::new(LegacyEffect { inner: effect.effect_type }),
        })
    }

    fn cooltime_use_count(&self, caster_stat: &StatV1) -> usize {
        self.inner.cooltime_use_count(caster_stat)
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
    fn apply(&self, sim: &mut StableSim<'_>, rng_seed: u64, caster_id: usize, input: InputTargetV1) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.apply(&mut ctx, rng_seed, caster_id, InputTarget::from_stable(input));
    }

    fn expected_damage(&self, caster_stat: &StatV1) -> (usize, usize) {
        self.inner.expected_damage(caster_stat)
    }

    fn expected_heal(&self, caster_stat: &StatV1) -> usize {
        self.inner.expected_heal(caster_stat)
    }

    fn expected_shield(&self, caster_stat: &StatV1) -> usize {
        self.inner.expected_shield(caster_stat)
    }

    fn expected_cc_time(&self) -> Option<usize> {
        self.inner.expected_cc_time()
    }

    fn expected_buff(&self, caster_stat: &StatV1) -> Option<BuffV1> {
        self.inner.expected_buff(caster_stat).map(|buff| buff.to_stable())
    }

    fn expected_move_distance(&self) -> Option<(usize, u64)> {
        self.inner.expected_move_distance()
    }

    fn expected_rush_effect(&self) -> bool {
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
        Box::new(Self { inner: self.inner.clone_box() })
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
        self.inner.on_attack(&mut ctx, player, entity, target, damage);
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
        self.inner.on_damaged(&mut ctx, player, entity, attacker, damage);
    }

    fn on_kill(&mut self, sim: &mut StableSim<'_>, player: usize, entity: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_kill(&mut ctx, player, entity);
    }

    fn on_update(&mut self, sim: &mut StableSim<'_>, rng_seed: u64, player: usize, entity: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_update(&mut ctx, rng_seed, player, entity);
    }

    fn on_base_attack(&mut self, sim: &mut StableSim<'_>, rng_seed: u64, player: usize, entity: usize) {
        let mut ctx = GameCtx::from_stable(sim);
        self.inner.on_base_attack(&mut ctx, rng_seed, player, entity);
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
}

impl GameCtx {
    pub fn from_stable(sim: &mut StableSim<'_>) -> Self {
        Self { sim: sim as *mut StableSim<'_> as *mut StableSim<'static> }
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
        self.sim_mut().add_buff(target, &buff.to_stable());
    }

    pub fn apply_cc(&mut self, target: usize, cc: CCState) {
        self.sim_mut().apply_cc(target, &cc.to_stable());
    }

    pub fn deal_damage(&mut self, attacker: usize, target: usize, ad: usize, ap: usize, attack_type: AttackType) {
        self.sim_mut().deal_damage(attacker, target, ad, ap, attack_type.to_stable());
    }

    pub fn heal(&mut self, caster: usize, target: usize, amount: usize) {
        self.sim_mut().heal(caster, target, amount);
    }

    pub fn entity_set_hp(&self, entity_id: usize, hp: usize) -> bool {
        self.sim_mut().entity_set_hp(entity_id, hp)
    }

    pub fn entity_set_pos(&self, entity_id: usize, x: u64, y: u64) -> bool {
        self.sim_mut().entity_set_pos(entity_id, x, y)
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

    pub fn id(&self) -> usize { self.id }
    pub fn stat(&self) -> EntityStat { self.stat }
    pub fn pos(&self) -> EntityPos { self.pos }
    pub fn hp(&self) -> EntityHp { self.hp }
    pub fn team(&self) -> usize { self.team }
    pub fn level(&self) -> usize { self.level }
    pub fn is_alive(&self) -> bool { self.is_alive }
    pub fn is_champion(&self) -> bool { self.is_champion }
    pub fn is_tower(&self) -> bool { self.is_tower }
    pub fn is_minion(&self) -> bool { self.is_minion }
    pub fn shield(&self) -> usize { self.shield }
    pub fn radius(&self) -> usize { self.radius }
    pub fn is_targetable(&self) -> bool { self.is_targetable }
    pub fn buff_count(&self) -> usize { self.buffs.len() }
    pub fn buff_at(&self, index: usize) -> Option<BuffState> { self.buffs.get(index).cloned() }
    pub fn cc_count(&self) -> usize { self.ccs.len() }
    pub fn cc_at(&self, index: usize) -> EntityCc {
        self.ccs.get(index).copied().unwrap_or_default()
    }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCc {
    pub cc_type: u32,
    pub state: Option<CCState>,
}

impl Default for EntityCc {
    fn default() -> Self {
        Self { cc_type: 255, state: None }
    }
}

impl EntityCc {
    fn from_stable(cc: CcV1) -> Self {
        Self { cc_type: cc.kind, state: Some(CCState::from_stable(cc)) }
    }
}

impl BuffState {
    fn from_stable(buff: BuffV1) -> Self {
        let duration = if buff.duration_kind == BuffDurationV1::Time.code() {
            BuffType::Time { tick: buff.duration_tick }
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
            Self::BlockAttack { tick: cc.tick as usize }
        } else if cc.kind == CcKindV1::BlockSkill.code() {
            Self::BlockSkill { tick: cc.tick as usize }
        } else if cc.kind == CcKindV1::BlockMoveSkill.code() {
            Self::BlockMoveSkill { tick: cc.tick as usize }
        } else if cc.kind == CcKindV1::ForceMove.code() {
            Self::ForceMove { tick: cc.tick, dx: cc.dx, dy: cc.dy, speed: cc.speed }
        } else if cc.kind == CcKindV1::Taunt.code() {
            Self::Taunt { tick: cc.tick, target: cc.target }
        } else if cc.kind == CcKindV1::Fear.code() {
            Self::Fear { tick: cc.tick, dx: cc.dx, dy: cc.dy }
        } else if cc.kind == CcKindV1::Charm.code() {
            Self::Charm { tick: cc.tick, dx: cc.dx, dy: cc.dy }
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
        Self { ptr: std::ptr::null_mut() }
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

    pub fn id(&self) -> usize { self.id }
    pub fn team(&self) -> usize { self.team }
    pub fn level(&self) -> usize { self.level }
    pub fn gold(&self) -> usize { self.gold }
    pub fn kills(&self) -> usize { self.kills }
    pub fn deaths(&self) -> usize { self.deaths }
    pub fn assists(&self) -> usize { self.assists }
    pub fn cs(&self) -> usize { self.cs }
    pub fn position(&self) -> Position { self.position }
    pub fn champion(&self) -> Option<EntityRef<'static>> { self.champion.clone() }
    pub fn is_alive(&self) -> bool {
        self.champion.as_ref().map_or(false, EntityRef::is_alive)
    }
    pub fn handle(&self) -> PlayerHandle { PlayerHandle::null() }
}
