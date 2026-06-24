param(
  [Parameter(Mandatory = $true)]
  [string]$Name,

  [string]$Manifest = "data\showdown_ani_manifest.json",
  [string]$SourceDir = "assets\source\showdown\ani",
  [string]$OutputDir = "mod\pokemon_moba\champions",
  [int]$CanvasSize = 96,
  [int]$MaxContentSize = 58,
  [int]$BottomPadding = 24,
  [string]$Anchor = "center",
  [switch]$NoOverlay
)

$ErrorActionPreference = "Stop"

throw "Deprecated: Pokemon champion sprites now come from assets\\custom_spritework\\champions and sync to mod\\pokemon_moba\\champions_custom. See docs\\asset-pipeline.md and run tools\\sync-custom-champion-sprites.py instead."

$root = (Resolve-Path "$PSScriptRoot\..").Path
$python = Join-Path $root ".venv\Scripts\python.exe"

if (-not (Test-Path -LiteralPath $python)) {
  throw "Missing tool Python at $python. Create it with: python -m venv .venv; .\.venv\Scripts\python.exe -m pip install -r requirements-tools.txt"
}

Push-Location $root
try {
  $profiles = @{
    pikachu = @{ CanvasSize = 96; MaxContentSize = 30 }
    eevee = @{ CanvasSize = 96; MaxContentSize = 31 }
    porygonz = @{ CanvasSize = 96; MaxContentSize = 34 }
    jolteon = @{ CanvasSize = 96; MaxContentSize = 34 }
    flareon = @{ CanvasSize = 96; MaxContentSize = 34 }
    espeon = @{ CanvasSize = 96; MaxContentSize = 34 }
    umbreon = @{ CanvasSize = 96; MaxContentSize = 33 }
    leafeon = @{ CanvasSize = 96; MaxContentSize = 36 }
    glaceon = @{ CanvasSize = 96; MaxContentSize = 36 }
    sylveon = @{ CanvasSize = 96; MaxContentSize = 36 }
    greninja = @{ CanvasSize = 96; MaxContentSize = 40 }
    inteleon = @{ CanvasSize = 96; MaxContentSize = 40 }
    decidueye = @{ CanvasSize = 96; MaxContentSize = 42 }
    scizor = @{ CanvasSize = 96; MaxContentSize = 42 }
    blissey = @{ CanvasSize = 96; MaxContentSize = 43 }
    blastoise = @{ CanvasSize = 96; MaxContentSize = 42 }
    blaziken = @{ CanvasSize = 96; MaxContentSize = 44 }
    kleavor = @{ CanvasSize = 96; MaxContentSize = 44 }
    charizard = @{ CanvasSize = 96; MaxContentSize = 48 }
    emboar = @{ CanvasSize = 96; MaxContentSize = 46 }
    pangoro = @{ CanvasSize = 96; MaxContentSize = 46 }
    passimian = @{ CanvasSize = 96; MaxContentSize = 42 }
    oranguru = @{ CanvasSize = 96; MaxContentSize = 42 }
    dragalge = @{ CanvasSize = 96; MaxContentSize = 44 }
    heliolisk = @{ CanvasSize = 96; MaxContentSize = 42 }
    turtonator = @{ CanvasSize = 96; MaxContentSize = 46 }
    ribombee = @{ CanvasSize = 96; MaxContentSize = 36 }
    drampa = @{ CanvasSize = 96; MaxContentSize = 46 }
    kommoo = @{ CanvasSize = 96; MaxContentSize = 48 }
    thievul = @{ CanvasSize = 96; MaxContentSize = 40 }
    archaludon = @{ CanvasSize = 96; MaxContentSize = 48 }
    appletun = @{ CanvasSize = 96; MaxContentSize = 46 }
    goodra = @{ CanvasSize = 96; MaxContentSize = 46 }
    dedenne = @{ CanvasSize = 96; MaxContentSize = 32 }
    hawlucha = @{ CanvasSize = 96; MaxContentSize = 42 }
    bouffalant = @{ CanvasSize = 96; MaxContentSize = 46 }
    starmie = @{ CanvasSize = 96; MaxContentSize = 40 }
    drednaw = @{ CanvasSize = 96; MaxContentSize = 48 }
    orbeetle = @{ CanvasSize = 96; MaxContentSize = 40 }
    coalossal = @{ CanvasSize = 96; MaxContentSize = 48 }
    magmortar = @{ CanvasSize = 96; MaxContentSize = 44 }
    grapploct = @{ CanvasSize = 96; MaxContentSize = 44 }
    sirfetchd = @{ CanvasSize = 96; MaxContentSize = 42 }
    arboliva = @{ CanvasSize = 96; MaxContentSize = 44 }
    armarouge = @{ CanvasSize = 96; MaxContentSize = 44 }
    ceruledge = @{ CanvasSize = 96; MaxContentSize = 44 }
    gholdengo = @{ CanvasSize = 96; MaxContentSize = 42 }
    frosmoth = @{ CanvasSize = 96; MaxContentSize = 40 }
    shedinja = @{ CanvasSize = 96; MaxContentSize = 34 }
    ludicolo = @{ CanvasSize = 96; MaxContentSize = 42 }
    kingdra = @{ CanvasSize = 96; MaxContentSize = 42 }
    delibird = @{ CanvasSize = 96; MaxContentSize = 38 }
    cloyster = @{ CanvasSize = 96; MaxContentSize = 40 }
    electrode = @{ CanvasSize = 96; MaxContentSize = 34 }
    snorlax = @{ CanvasSize = 96; MaxContentSize = 48 }
    zeraora = @{ CanvasSize = 96; MaxContentSize = 42 }
    comfey = @{ CanvasSize = 96; MaxContentSize = 34 }
    skeledirge = @{ CanvasSize = 96; MaxContentSize = 48 }
    venusaur = @{ CanvasSize = 96; MaxContentSize = 46 }
    feraligatr = @{ CanvasSize = 96; MaxContentSize = 48 }
    ursaluna = @{ CanvasSize = 96; MaxContentSize = 50 }
    ursaluna_bloodmoon = @{ CanvasSize = 96; MaxContentSize = 50 }
    sawk = @{ CanvasSize = 96; MaxContentSize = 42 }
    throh = @{ CanvasSize = 96; MaxContentSize = 44 }
    sawk_throh = @{ CanvasSize = 96; MaxContentSize = 43 }
    hitmonchan = @{ CanvasSize = 96; MaxContentSize = 42 }
    hitmonlee = @{ CanvasSize = 96; MaxContentSize = 42 }
    hitmontop = @{ CanvasSize = 96; MaxContentSize = 40 }
    kilowattrel = @{ CanvasSize = 96; MaxContentSize = 42 }
    beheeyem = @{ CanvasSize = 96; MaxContentSize = 40 }
    beeheeyem = @{ CanvasSize = 96; MaxContentSize = 40 }
    gyarados = @{ CanvasSize = 96; MaxContentSize = 50 }
    noivern = @{ CanvasSize = 96; MaxContentSize = 44 }
    mantine = @{ CanvasSize = 96; MaxContentSize = 44 }
    cryogonal = @{ CanvasSize = 96; MaxContentSize = 40 }
    vanilluxe = @{ CanvasSize = 96; MaxContentSize = 42 }
    skarmory = @{ CanvasSize = 96; MaxContentSize = 46 }
    houndoom = @{ CanvasSize = 96; MaxContentSize = 42 }
    arbok = @{ CanvasSize = 96; MaxContentSize = 44 }
    clawitzer = @{ CanvasSize = 96; MaxContentSize = 42 }
    octillery = @{ CanvasSize = 96; MaxContentSize = 42 }
    pyukumuku = @{ CanvasSize = 96; MaxContentSize = 34 }
    banette = @{ CanvasSize = 96; MaxContentSize = 40 }
    kricketune = @{ CanvasSize = 96; MaxContentSize = 42 }
    ambipom = @{ CanvasSize = 96; MaxContentSize = 42 }
    gallade = @{ CanvasSize = 96; MaxContentSize = 44 }
    audino = @{ CanvasSize = 96; MaxContentSize = 42 }
    smeargle = @{ CanvasSize = 96; MaxContentSize = 40 }
    torterra = @{ CanvasSize = 96; MaxContentSize = 50 }
  }
  $key = $Name.ToLowerInvariant()
  if ($profiles.ContainsKey($key)) {
    $profile = $profiles[$key]
    if (-not $PSBoundParameters.ContainsKey("CanvasSize")) {
      $CanvasSize = $profile.CanvasSize
    }
    if (-not $PSBoundParameters.ContainsKey("MaxContentSize")) {
      $MaxContentSize = $profile.MaxContentSize
    }
  }

  if (-not (Test-Path -LiteralPath $Manifest)) {
    & $python .\tools\showdown_sprites.py index --manifest $Manifest
    if ($LASTEXITCODE -ne 0) { throw "Failed to build Showdown sprite manifest." }
  }

  & $python .\tools\showdown_sprites.py download --manifest $Manifest --output-dir $SourceDir --names $Name
  if ($LASTEXITCODE -ne 0) { throw "Failed to download Showdown sprite for $Name." }
  & $python .\tools\showdown_sprites.py batch-convert `
    --manifest $Manifest `
    --input-dir $SourceDir `
    --output-dir $OutputDir `
    --names $Name `
    --canvas-size $CanvasSize `
    --max-content-size $MaxContentSize `
    --bottom-padding $BottomPadding `
    --anchor $Anchor `
    --overwrite
  if ($LASTEXITCODE -ne 0) { throw "Failed to convert Showdown sprite for $Name." }

  if (-not $NoOverlay) {
    $base = Join-Path $OutputDir $Name
    & $python .\tools\generate-champion-action-overlays.py $base
    if ($LASTEXITCODE -ne 0) { throw "Failed to generate champion action overlays for $Name." }
  }
} finally {
  Pop-Location
}
