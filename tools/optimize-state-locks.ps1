param([switch]$Apply)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path "$PSScriptRoot/..").Path
$init = '(\|\| Mutex::new\(Vec::new\(\)\)|Default::default|\|\| Mutex::new\(usize::MAX\)|\|\| Mutex::new\(0\))'
$direct = '\.get_or_init\(' + $init + '\)\s*\.lock\(\)'
$pair = 'let (?<var>\w+) = (?<store>\w+)\.get_or_init\((?<init>' + $init + ')\);\s*let (?<mut>mut )?\k<var> = \k<var>\s*\.lock\(\)'
foreach ($name in @('pokemon_status.rs', 'pokemon_content.rs')) {
    $path = Join-Path $root "mod/pokemon_moba_stable/src/legacy/$name"
    $source = [IO.File]::ReadAllText($path)
    $paired = [regex]::Matches($source, $pair).Count
    $source = [regex]::Replace($source, $pair, {
        param($m)
        'let ' + $m.Groups['mut'].Value + $m.Groups['var'].Value + ' = ' +
            $m.Groups['store'].Value + '.lock_or_init(' + $m.Groups['init'].Value + ')'
    })
    $directCount = [regex]::Matches($source, $direct).Count
    $source = [regex]::Replace($source, $direct, '.lock_or_init($1)')
    if ($Apply -and ($paired + $directCount -gt 0)) {
        [IO.File]::WriteAllText($path, $source, [Text.UTF8Encoding]::new($false))
    }
    [PSCustomObject]@{ File = $name; Paired = $paired; Direct = $directCount; Applied = $Apply.IsPresent }
}
