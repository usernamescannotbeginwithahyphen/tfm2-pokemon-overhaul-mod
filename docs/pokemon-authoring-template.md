# Pokemon Champion Authoring Template

Use this shape when giving me a Pokemon kit.

```text
Pokemon:
Display name:
Pokemon types:
Role/category: Melee | Range | Magician | Util | Assassin
Tags: AD, AP, Heal, Shield, Dot, CC, Range, Melee, Tank, Magic
Playstyle:

Draft strategy:
- Traits: AdDamage, ApDamage, HighHealth, HighDefense, HighMobility, LowMobility, BasicAttacker, CritCarry, BacklineCarry, Frontline, MeleeThreat, SingleTargetAssassin, Dive, Poke, Zoner, Disabler, HardCc, Dot, Healer, HealReliant, Sustain, Shielding, Cleanse, TeamBuff, TankBuster, PercentHealthDamage, AntiHeal, Execute, ObjectiveBurn, TypeFlexible, Untargetable, Squishy, ChannelReliant
- Threatens:
- Vulnerable to:

Stats:
- HP:
- Attack:
- Magic power:
- Defence:
- Magic resistance:
- Move speed:
- HP regen:
- Growth notes:

Passive:
- Name:
- Effect:
- Passimian Receiver compatibility: compatible | incompatible | move-specific translation needed

Basic attack:
- Name:
- Move type:
- Category: Physical | Special | Status
- Scaling: base + %AD or base + %AP
- Range:
- Cooldown/attack speed feel:
- Effect:

Skill 1:
- Name:
- Move type:
- Category: Physical | Special | Status
- Targeting: Targeting | Position | Direction | None
- Scaling:
- Range:
- Cooldown:
- Damage/heal/shield/CC:
- Effect notes:

Skill 2:
- Name:
- Move type:
- Category: Physical | Special | Status
- Targeting:
- Scaling:
- Range:
- Cooldown:
- Damage/heal/shield/CC:
- Effect notes:

Ultimate:
- Name:
- Move type:
- Category: Physical | Special | Status
- Targeting:
- Scaling:
- Range:
- Cooldown:
- Damage/heal/shield/CC:
- Effect notes:

Assets:
- Champion sprite asset id / short-id for staged PMD/custom files:
- Skill icon override needed? yes/no
- Projectile/effect notes:
```

## Defaults

If you leave numbers open, I will choose a conservative baseline:

- Squishy ranged: `580-680 HP`, low defence, medium move speed
- Bruiser melee: `720-850 HP`, medium defence, medium move speed
- Tank: `900+ HP`, high defence/resistance, slower move speed
- Physical move: AD scaling
- Special move: AP scaling
- Status move: no direct damage, may heal, shield, buff, debuff, DOT, or CC
- Draft strategy traits should describe gameplay capabilities and weaknesses, not exact pair counters. Use explicit one-off counters only when broad traits cannot model the matchup.
- Use `AdDamage` or `ApDamage` for a Pokemon's meaningful team damage profile. Pure healers or utility tanks with only token damage can omit both so the draft scorer does not treat them as real damage sources.
- Be explicit about area shape. "Cone" means a forward cone/wedge. "Arc" means a melee cleave/fan around the attacker, usually basic-attack-like, and should not be implemented or described as a projectile cone unless the design says so.
- Invisibility/stealth currently means Pokemon-layer hidden targeting/AI awareness plus soft untargetable. It does not change native renderer visibility unless a later SDK visibility hook is found.
