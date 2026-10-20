$ErrorActionPreference = 'Stop'

# Isolated fixtures: never point this test at the real game or mod package.
$root = Join-Path (Split-Path $PSScriptRoot -Parent) ('tmp/deploy-test-' + [Guid]::NewGuid().ToString('N'))
$scriptRoot = Join-Path $root 'tools'
$source = Join-Path $root 'mod/test_mod'
$game = Join-Path $root 'game'
$target = Join-Path $game 'mods/test_mod'
New-Item -ItemType Directory -Force -Path $scriptRoot, $source, $target | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'deploy.ps1') -Destination $scriptRoot
$deploy = Join-Path $scriptRoot 'deploy.ps1'
$sourceId = Join-Path $source 'mod.workshop_id'
$targetId = Join-Path $target 'mod.workshop_id'

function Write-Id([string]$Path, [UInt64]$Id) {
  @{ published_file_id = $Id } | ConvertTo-Json | Set-Content -LiteralPath $Path
}
function Assert-Id([string]$Path, [UInt64]$Expected) {
  $actual = (Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json).published_file_id
  if ($actual -ne $Expected) { throw "Expected $Expected at $Path, got $actual" }
}

Write-Id $sourceId 3744254195
Write-Id $targetId 3823239864
& $deploy -GameRoot $game -ModId test_mod
Assert-Id $targetId 3744254195

& $deploy -GameRoot $game -ModId test_mod -WorkshopId 123456789
Assert-Id $sourceId 123456789
Assert-Id $targetId 123456789

Remove-Item -LiteralPath $sourceId
& $deploy -GameRoot $game -ModId test_mod
Assert-Id $targetId 123456789
if (Test-Path -LiteralPath $sourceId) { throw 'Installed fallback unexpectedly changed source package' }

Remove-Item -LiteralPath $targetId
& $deploy -GameRoot $game -ModId test_mod
if (Test-Path -LiteralPath $targetId) { throw 'Unpublished package acquired a Workshop ID' }

Write-Id $targetId 3744254195
$vfx = Join-Path $source 'vfx'
New-Item -ItemType Directory -Path $vfx | Out-Null
$intermediate = Join-Path $vfx 'fixture_cleaned.png'
Set-Content -LiteralPath $intermediate -Value 'Development-only fixture; must never be deployed.'
$rejected = $false
try { & $deploy -GameRoot $game -ModId test_mod }
catch { $rejected = $true }
if (-not $rejected) { throw 'Accepted development-only VFX images' }
Assert-Id $targetId 3744254195
Remove-Item -LiteralPath $intermediate

foreach ($invalid in @('0', 'not-an-id', '18446744073709551616')) {
  $rejected = $false
  try { & $deploy -GameRoot $game -ModId test_mod -WorkshopId $invalid }
  catch { $rejected = $true }
  if (-not $rejected) { throw "Accepted invalid Workshop ID: $invalid" }
  Assert-Id $targetId 3744254195
}
Write-Id $sourceId 0
$rejected = $false
try { & $deploy -GameRoot $game -ModId test_mod }
catch { $rejected = $true }
if (-not $rejected) { throw 'Accepted invalid source Workshop ID' }
Assert-Id $targetId 3744254195
Write-Host "Deployment Workshop ID tests passed. Fixtures: $root"
