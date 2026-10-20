param(
  [string]$GameRoot = "C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2",
  [string]$ModId = "pokemon_moba_stable",
  [string]$WorkshopId = ""
)

$ErrorActionPreference = "Stop"

function ConvertTo-WorkshopId {
  param([string]$Value)
  $id = [UInt64]0
  if ($Value -notmatch '^[0-9]+$' -or -not [UInt64]::TryParse($Value, [ref]$id) -or $id -eq 0) {
    throw "Invalid Workshop published_file_id: $Value"
  }
  return $id
}

function Read-WorkshopIdFile {
  param([string]$Path)
  $info = Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
  return ConvertTo-WorkshopId -Value ([string]$info.published_file_id)
}

function Write-WorkshopIdFile {
  param(
    [string]$ModRoot,
    [string]$PublishedFileId
  )

  if ([string]::IsNullOrWhiteSpace($PublishedFileId)) {
    return
  }

  $workshopIdPath = Join-Path $ModRoot "mod.workshop_id"
  $content = @{ published_file_id = (ConvertTo-WorkshopId -Value $PublishedFileId) } | ConvertTo-Json
  $utf8NoBom = [System.Text.UTF8Encoding]::new($false)
  [System.IO.File]::WriteAllText($workshopIdPath, $content, $utf8NoBom)
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

$vfxRoot = Join-Path $source 'vfx'
if (Test-Path -LiteralPath $vfxRoot) {
  $intermediates = @(Get-ChildItem -LiteralPath $vfxRoot -File -Recurse -Filter '*_cleaned.png')
  if ($intermediates.Count -gt 0) {
    throw "Package contains $($intermediates.Count) development-only *_cleaned.png images. Remove them from vfx before deployment."
  }
}

# Capture the selected identity before clearing the installed package. A source
# ID is authoritative; otherwise preserve an existing installation's ID.
$sourceWorkshopPath = Join-Path $source "mod.workshop_id"
$targetWorkshopPath = Join-Path $target "mod.workshop_id"
$effectiveWorkshopId = $null
if (-not [string]::IsNullOrWhiteSpace($WorkshopId)) {
  $effectiveWorkshopId = ConvertTo-WorkshopId -Value $WorkshopId
  Write-WorkshopIdFile -ModRoot $source -PublishedFileId ([string]$effectiveWorkshopId)
} elseif (Test-Path -LiteralPath $sourceWorkshopPath) {
  $effectiveWorkshopId = Read-WorkshopIdFile -Path $sourceWorkshopPath
} elseif (Test-Path -LiteralPath $targetWorkshopPath) {
  $effectiveWorkshopId = Read-WorkshopIdFile -Path $targetWorkshopPath
}

New-Item -ItemType Directory -Force -Path $target | Out-Null

$resolvedTarget = (Resolve-Path -LiteralPath $target).Path
$resolvedModsRoot = (Resolve-Path -LiteralPath $modsRoot).Path
if (-not $resolvedTarget.StartsWith($resolvedModsRoot.TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase)) {
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

if ($null -ne $effectiveWorkshopId) {
  Write-WorkshopIdFile -ModRoot $target -PublishedFileId ([string]$effectiveWorkshopId)
}

Write-Host "Copied $source to $target"
