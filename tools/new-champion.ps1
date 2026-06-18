param(
  [Parameter(Mandatory = $true)]
  [ValidatePattern("^[a-z0-9_]+$")]
  [string]$ShortId,

  [Parameter(Mandatory = $true)]
  [string]$Name,

  [ValidateSet("Melee", "Range", "Magician", "Util", "Assassin")]
  [string]$Category = "Range",

  [string[]]$Tags = @("AP", "Range"),

  [string]$Sprite = "asset/base/aseprite_resources/champions/pyromancer",

  [string]$ModId = "pokemon_moba"
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path "$PSScriptRoot\..").Path
$championId = "$($ModId)_$ShortId"
$championPath = Join-Path $root "mod\$ModId\champion\$ShortId.data_champion"
$i18nPath = Join-Path $root "mod\$ModId\text\champion.i18n"

if (Test-Path -LiteralPath $championPath) {
  throw "Champion file already exists: $championPath"
}

$iconBase = "asset/$ModId/icons/skills/$ShortId"

$champion = [ordered]@{
  id = $championId
  category = $Category
  tags = $Tags
  sprite = $Sprite
  anim_prefix = ""
  skill_icons = @(
    "$($iconBase)_skill",
    "$($iconBase)_skill2",
    "$($iconBase)_ult"
  )
  stat = [ordered]@{
    attack = 40
    magic_power = 60
    hp = 620
    defence = 20
    magic_resistance = 30
    move_speed = 1050
    hp_regen = 2
    stack = 0
    crit_chance = 0
  }
  growth = [ordered]@{
    attack = 3
    magic_power = 6
    hp = 75
    defence = 3
    magic_resistance = 3
    move_speed = 0
    hp_regen = 1
    stack = 0
    crit_chance = 0
  }
  attack = [ordered]@{
    action_name = "attack"
    duration = 18
    cooltime = 60
    start_timing = 10
    cancelable = $true
    range = 52000
    casting_type = "Targeting"
    casting_target = "Enemy"
    attack_type = "BaseAttack"
    effect = [ordered]@{
      type = "TargetProjectile"
      speed = 4500
      name = "$($ShortId)_attack"
      applied_target = "Enemy"
      applied_effects = @(
        [ordered]@{
          effect = [ordered]@{
            type = "Attack"
            damage = 0
            attack_ratio = 100
          }
          casting_type = "Targeting"
        }
      )
    }
  }
  skill = [ordered]@{
    action_name = "skill"
    description = "#asset/base/text/champion?description.$championId.skill"
    duration = 20
    cooltime = 240
    start_timing = 10
    range = 65000
    casting_type = "Direction"
    casting_target = "Enemy"
    attack_type = "Skill"
    effect = [ordered]@{
      type = "LinearProjectile"
      penetrate = $true
      speed = 4200
      range = 65000
      name = "$($ShortId)_skill"
      shape = [ordered]@{ Circle = [ordered]@{ radius = 8000 } }
      applied_target = "Enemy"
      applied_effects = @(
        [ordered]@{
          effect = [ordered]@{
            type = "ApAttack"
            damage = 50
            attack_ratio = 80
          }
          casting_type = "Targeting"
        }
      )
    }
  }
  skill2 = [ordered]@{
    action_name = "skill2"
    description = "#asset/base/text/champion?description.$championId.skill2"
    duration = 16
    cooltime = 360
    start_timing = 8
    range = 0
    casting_type = "None"
    casting_target = "AllyOnlySelf"
    attack_type = "Skill"
    effect = [ordered]@{
      type = "AddCasterBuff"
      buff_state = [ordered]@{
        name = "$($ShortId)_focus"
        duration = [ordered]@{ Time = [ordered]@{ tick = 180 } }
        magic_power = 20
      }
    }
  }
  ult = [ordered]@{
    action_name = "ult"
    description = "#asset/base/text/champion?description.$championId.ult"
    duration = 25
    cooltime = 900
    start_timing = 12
    range = 42000
    casting_type = "None"
    casting_target = "Enemy"
    attack_type = "Skill"
    effect = [ordered]@{
      type = "RangeEffect"
      shape = [ordered]@{ Circle = [ordered]@{ radius = 42000 } }
      target = "Enemy"
      apply_type = "AroundCaster"
      effects = @(
        [ordered]@{
          type = "Combine"
          effects = @(
            [ordered]@{
              type = "ApAttack"
              damage = 120
              attack_ratio = 100
            },
            [ordered]@{
              type = "Stun"
              duration = 45
            }
          )
        }
      )
    }
  }
  view_projectiles = @(
    [ordered]@{
      type = "Sprite"
      name = "$($ShortId)_attack"
      sprite = "asset/base/sprite/arrow"
    },
    [ordered]@{
      type = "Sprite"
      name = "$($ShortId)_skill"
      sprite = "asset/base/sprite/arrow"
    }
  )
}

$champion | ConvertTo-Json -Depth 32 | Set-Content -LiteralPath $championPath -Encoding UTF8

$i18n = Get-Content -LiteralPath $i18nPath -Raw | ConvertFrom-Json
if (-not $i18n.en) {
  $i18n | Add-Member -NotePropertyName en -NotePropertyValue ([pscustomobject]@{})
}
if (-not $i18n.en.description) {
  $i18n.en | Add-Member -NotePropertyName description -NotePropertyValue ([pscustomobject]@{})
}

$entry = [pscustomobject]@{
  name = $Name
  skill = "Skill 1: placeholder description."
  skill2 = "Skill 2: placeholder description."
  ult = "Ultimate: placeholder description."
}

$i18n.en.description | Add-Member -NotePropertyName $championId -NotePropertyValue $entry
$i18n | ConvertTo-Json -Depth 32 | Set-Content -LiteralPath $i18nPath -Encoding UTF8

Write-Host "Created $championPath"
Write-Host "Updated $i18nPath"

