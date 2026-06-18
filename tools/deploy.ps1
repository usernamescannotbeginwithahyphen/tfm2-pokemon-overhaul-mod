param(
  [string]$GameRoot = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2",
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
  Set-Content -LiteralPath $workshopIdPath -Value $content -Encoding UTF8
}

$workspaceRoot = (Resolve-Path "$PSScriptRoot\..").Path
$source = Join-Path $workspaceRoot "mod\$ModId"
$target = Join-Path $GameRoot "mods\$ModId"
$modsRoot = Join-Path $GameRoot "mods"

if (-not (Test-Path -LiteralPath $source)) {
  throw "Mod source does not exist: $source"
}

if (-not (Test-Path -LiteralPath $GameRoot)) {
  throw "Game root does not exist: $GameRoot"
}

Write-WorkshopIdFile -ModRoot $source -PublishedFileId $WorkshopId

New-Item -ItemType Directory -Force -Path $target | Out-Null

$resolvedTarget = (Resolve-Path -LiteralPath $target).Path
$resolvedModsRoot = (Resolve-Path -LiteralPath $modsRoot).Path
if (-not $resolvedTarget.StartsWith($resolvedModsRoot, [StringComparison]::OrdinalIgnoreCase)) {
  throw "Refusing to clear unexpected deploy target: $resolvedTarget"
}

Get-ChildItem -LiteralPath $target -Force | Remove-Item -Recurse -Force

$excludeNames = @(
  "src",
  "target",
  "Cargo.toml",
  "Cargo.lock",
  "rust-toolchain.toml"
)

Get-ChildItem -LiteralPath $source -Force |
  Where-Object { $excludeNames -notcontains $_.Name } |
  Copy-Item -Destination $target -Recurse -Force

Write-Host "Copied $source to $target"
