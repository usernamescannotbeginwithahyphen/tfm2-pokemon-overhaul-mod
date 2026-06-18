param(
  [string]$Manifest = "data\showdown_ani_manifest.json",
  [string]$SourceDir = "assets\source\showdown\ani",
  [string]$OutputDir = "mod\pokemon_moba\ingame",
  [int]$CanvasSize = 96,
  [int]$MaxContentSize = 58
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path "$PSScriptRoot\..").Path
$python = Join-Path $root ".venv\Scripts\python.exe"
$neutralProfiles = @(
  @{ Name = "mew"; CanvasSize = 72; MaxContentSize = 38; BlankDead = $false },
  @{ Name = "beedrill"; CanvasSize = 72; MaxContentSize = 34; BlankDead = $true },
  @{ Name = "amoonguss"; CanvasSize = 80; MaxContentSize = 44; BlankDead = $true },
  @{ Name = "rhyperior"; CanvasSize = $CanvasSize; MaxContentSize = $MaxContentSize; BlankDead = $true },
  @{ Name = "eternatus"; CanvasSize = 128; MaxContentSize = 96; BlankDead = $false },
  @{ Name = "trevenant"; CanvasSize = $CanvasSize; MaxContentSize = $MaxContentSize; BlankDead = $true }
)

Push-Location $root
try {
  foreach ($profile in $neutralProfiles) {
    & $python .\tools\showdown_sprites.py batch-convert `
      --manifest $Manifest `
      --input-dir $SourceDir `
      --output-dir $OutputDir `
      --names $profile.Name `
      --action-tags idle,run,attack,dead,base,attack_left,attack_right,attack_target_effect `
      --canvas-size $profile.CanvasSize `
      --max-content-size $profile.MaxContentSize `
      --anchor center `
      --overwrite
    if ($LASTEXITCODE -ne 0) { throw "Neutral sprite conversion failed for $($profile.Name)." }

    if ($profile.BlankDead) {
      & $python .\tools\blank-animation-tags.py `
        (Join-Path $OutputDir $profile.Name) `
        --tags dead
      if ($LASTEXITCODE -ne 0) { throw "Failed to blank $($profile.Name) dead animation tag." }
    }
  }

  & $python .\tools\blank-animation-tags.py `
    (Join-Path $OutputDir "mew") `
    --tags attack_left attack_right attack_target_effect
  if ($LASTEXITCODE -ne 0) { throw "Failed to blank auxiliary Mew animation tags." }
} finally {
  Pop-Location
}
