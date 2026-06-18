param(
  [string]$Source = "assets\source\showdown\ani\falinks.gif",
  [string]$Output = "mod\pokemon_moba\ingame\falinks_minion",
  [int]$CanvasSize = 44,
  [int]$MaxContentSize = 24,
  [string]$SourceCrop = "0,0,36,49"
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path "$PSScriptRoot\..").Path
$python = Join-Path $root ".venv\Scripts\python.exe"

Push-Location $root
try {
  & $python .\tools\generate-falinks-minions.py `
    --gif $Source `
    --output $Output `
    --canvas-size $CanvasSize `
    --max-content-size $MaxContentSize `
    --source-crop $SourceCrop
  if ($LASTEXITCODE -ne 0) { throw "Falinks minion sprite conversion failed." }
} finally {
  Pop-Location
}
