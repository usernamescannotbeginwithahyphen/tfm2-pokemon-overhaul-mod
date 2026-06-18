param(
  [string]$Root = (Resolve-Path "$PSScriptRoot\..").Path
)

$ErrorActionPreference = "Stop"

$patterns = @(
  "*.json",
  "*.mod_info",
  "*.override_info",
  "*.i18n",
  "*.data_champion",
  "*.sprite_sheet",
  "*.fanim"
)

$files = foreach ($pattern in $patterns) {
  Get-ChildItem -Path $Root -Recurse -File -Filter $pattern |
    Where-Object {
      $_.FullName -notmatch "\\.mod-docs\\" -and
      $_.FullName -notmatch "\\logs\\" -and
      $_.FullName -notmatch "\\target\\"
    }
}

$failed = @()
foreach ($file in $files | Sort-Object FullName -Unique) {
  try {
    $null = Get-Content -LiteralPath $file.FullName -Raw | ConvertFrom-Json
    Write-Host "OK  $($file.FullName)"
  } catch {
    $failed += $file.FullName
    Write-Error "Invalid JSON: $($file.FullName)`n$($_.Exception.Message)" -ErrorAction Continue
  }
}

if ($failed.Count -gt 0) {
  throw "$($failed.Count) JSON file(s) failed validation."
}

Write-Host "Validated $($files.Count) JSON file(s)."
