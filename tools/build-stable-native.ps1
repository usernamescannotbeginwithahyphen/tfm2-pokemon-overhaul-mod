param(
  [string]$GameRoot = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2",
  [string]$ModId = "pokemon_moba_stable"
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path "$PSScriptRoot\..").Path
$modRoot = Join-Path $root "mod\$ModId"
$stableSdk = Join-Path $GameRoot "mod-sdk-stable"
$stableCrate = Join-Path $stableSdk "mod-api-stable"
$requiredVersion = [version]"0.6.4"

if (-not (Test-Path -LiteralPath $stableCrate)) {
  throw "Stable SDK crate not found: $stableCrate"
}

$baseVersion = Join-Path $stableSdk "base_version.txt"
if (Test-Path -LiteralPath $baseVersion) {
  $version = (Get-Content -LiteralPath $baseVersion -Raw).Trim()
  if ([version]$version -lt $requiredVersion) {
    throw "SDK $version is too old. This package requires SDK $requiredVersion or newer. Update the game through Steam."
  }
  Write-Host "Using stable SDK $version from $stableSdk"
} else {
  throw "Missing SDK version marker: $baseVersion"
}

Push-Location $modRoot
try {
  python (Join-Path $root "tools\generate-stable-vfx-bindings.py")
  if ($LASTEXITCODE -ne 0) { throw "Native VFX catalog generation failed." }
  $metadataText = cargo metadata --format-version 1 --offline
  if ($LASTEXITCODE -ne 0) { throw "Cannot resolve the SDK dependency." }
  $metadata = $metadataText | ConvertFrom-Json
  $dependency = $metadata.packages | Where-Object { $_.name -eq "mod-api-stable" }
  $actualCrate = Split-Path -Parent $dependency.manifest_path
  if ((Resolve-Path -LiteralPath $actualCrate).Path -ne (Resolve-Path -LiteralPath $stableCrate).Path) {
    throw "Cargo uses $actualCrate but GameRoot selects $stableCrate. Update the one Cargo SDK path before building."
  }
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
