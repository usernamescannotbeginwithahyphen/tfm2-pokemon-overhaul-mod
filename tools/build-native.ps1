param(
  [string]$GameRoot = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2",
  [string]$ModId = "pokemon_moba_stable"
)

$ErrorActionPreference = "Stop"

$stableBuild = Join-Path $PSScriptRoot "build-stable-native.ps1"
& $stableBuild -GameRoot $GameRoot -ModId $ModId
exit $LASTEXITCODE
