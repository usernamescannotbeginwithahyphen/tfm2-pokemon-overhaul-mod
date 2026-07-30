param(
  [string]$GameRoot = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2",
  [string]$ModId = "pokemon_moba_stable"
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path "$PSScriptRoot\..").Path
$modRoot = Join-Path $root "mod\$ModId"
$stableSdk = Join-Path $GameRoot "mod-sdk-stable"
$stableCrate = Join-Path $stableSdk "mod-api-stable"

if (-not (Test-Path -LiteralPath $stableCrate)) {
  throw "Stable SDK crate not found: $stableCrate"
}

$baseVersion = Join-Path $stableSdk "base_version.txt"
if (Test-Path -LiteralPath $baseVersion) {
  $version = (Get-Content -LiteralPath $baseVersion -Raw).Trim()
  Write-Host "Using stable SDK $version from $stableSdk"
} else {
  Write-Host "Using stable SDK from $stableSdk"
}

Push-Location $modRoot
try {
  cargo build --release
  if ($LASTEXITCODE -ne 0) {
    throw "Stable native build failed."
  }

  $builtDll = Join-Path (Join-Path $modRoot "target\release") "$ModId.dll"
  if (-not (Test-Path -LiteralPath $builtDll)) {
    throw "Cargo build finished, but expected DLL was not found: $builtDll"
  }

  $outDll = Join-Path $modRoot "$ModId.dll"
  Copy-Item -LiteralPath $builtDll -Destination $outDll -Force
  Write-Host "Build successful: $outDll"
} finally {
  Pop-Location
}
