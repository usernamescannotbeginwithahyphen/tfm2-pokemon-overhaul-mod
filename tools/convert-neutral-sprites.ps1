param(
  [string]$SourceDir = "assets\custom_spritework\ingame",
  [string]$OutputDir = "mod\pokemon_moba\ingame"
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path "$PSScriptRoot\..").Path
$sourceRoot = Join-Path $root $SourceDir
$outputRoot = Join-Path $root $OutputDir

New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null

foreach ($source in Get-ChildItem -LiteralPath $sourceRoot -File) {
  if (
    $source.Name.EndsWith("#sheet.png") -or
    $source.Name.EndsWith("#anim.fanim") -or
    $source.Name.EndsWith("_projectile.png")
  ) {
    Copy-Item -LiteralPath $source.FullName -Destination (Join-Path $outputRoot $source.Name) -Force
  }
}

Write-Host "Synced staged ingame sprite assets from $SourceDir to $OutputDir."
