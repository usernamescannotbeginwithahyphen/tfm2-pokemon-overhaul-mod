param(
  [string]$GameRoot = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2",
  [string]$SdkRoot = "",
  [string]$ModId = "pokemon_moba",
  [string]$WorkshopId = "3744254195"
)

$ErrorActionPreference = "Stop"

function Write-WorkshopIdFile {
  param(
    [string]$ModRoot,
    [string]$PublishedFileId
  )

  if ([string]::IsNullOrWhiteSpace($PublishedFileId)) {
    return
  }

  $workshopIdPath = Join-Path $ModRoot "mod.workshop_id"
  $content = @"
{
  "published_file_id": $PublishedFileId
}
"@
  $utf8NoBom = [System.Text.UTF8Encoding]::new($false)
  [System.IO.File]::WriteAllText($workshopIdPath, $content, $utf8NoBom)
}

$root = (Resolve-Path "$PSScriptRoot\..").Path
$modRoot = Join-Path $root "mod\$ModId"
Write-WorkshopIdFile -ModRoot $modRoot -PublishedFileId $WorkshopId
if ([string]::IsNullOrWhiteSpace($SdkRoot)) {
  $localSdkRoot = Join-Path $root "downloads\tfm2-sdk\0.4.13"
  if (Test-Path -LiteralPath (Join-Path $localSdkRoot "mod-sdk")) {
    $SdkRoot = $localSdkRoot
  } else {
    $SdkRoot = $GameRoot
  }
}
$sdkDir = Join-Path $SdkRoot "mod-sdk"
$buildScript = Join-Path $sdkDir "build_mod_cargo.ps1"
$depsDir = Join-Path $sdkDir "deps"
$nativeDir = Join-Path $sdkDir "native"

if (-not (Test-Path -LiteralPath $buildScript)) {
  throw "SDK build script not found: $buildScript"
}

$modApi = Get-ChildItem -LiteralPath $depsDir -Filter "libmod_api-*.rlib" | Select-Object -First 1
if (-not $modApi) {
  throw "libmod_api .rlib not found in $depsDir"
}
$engineUi = Get-ChildItem -LiteralPath $depsDir -Filter "libengine_ui-*.rlib" | Select-Object -First 1
if (-not $engineUi) {
  throw "libengine_ui .rlib not found in $depsDir"
}
$engineCore = Get-ChildItem -LiteralPath $depsDir -Filter "libengine_core-*.rlib" | Select-Object -First 1
if (-not $engineCore) {
  throw "libengine_core .rlib not found in $depsDir"
}
$gameCore = Get-ChildItem -LiteralPath $depsDir -Filter "libgame_core-*.rlib" | Select-Object -First 1
if (-not $gameCore) {
  throw "libgame_core .rlib not found in $depsDir"
}
$targetDir = Join-Path $modRoot "target"

$flags = @(
  "-L",
  "dependency=$depsDir",
  "--extern",
  "mod_api=$($modApi.FullName)",
  "--extern",
  "engine_ui=$($engineUi.FullName)",
  "--extern",
  "engine_core=$($engineCore.FullName)",
  "--extern",
  "game_core=$($gameCore.FullName)"
)
if (Test-Path -LiteralPath $nativeDir) {
  $flags += @("-L", "native=$nativeDir")
}

Push-Location $modRoot
try {
  $env:CARGO_ENCODED_RUSTFLAGS = $flags -join [char]31
  cargo rustc --release --manifest-path (Join-Path $modRoot "Cargo.toml") --target-dir $targetDir --lib -- --crate-type cdylib
  if ($LASTEXITCODE -ne 0) {
    throw "Native build failed."
  }

  $builtDll = Join-Path (Join-Path $targetDir "release") "$ModId.dll"
  if (-not (Test-Path -LiteralPath $builtDll)) {
    throw "Cargo build finished, but expected DLL was not found: $builtDll"
  }

  $outDll = Join-Path $modRoot "$ModId.dll"
  Copy-Item -LiteralPath $builtDll -Destination $outDll -Force
  Write-Host "Build successful: $outDll"
} finally {
  Remove-Item Env:\CARGO_ENCODED_RUSTFLAGS -ErrorAction SilentlyContinue
  Pop-Location
}
