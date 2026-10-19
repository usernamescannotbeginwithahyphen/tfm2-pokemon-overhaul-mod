# Native SDK

Active official Steam SDK: **0.6.4**, stable ABI level **10** (verified 2026-10-18).

The only active crate is:

```text
C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mod-sdk-stable\mod-api-stable
```

Its adjacent `base_version.txt` reports 0.6.4. The crate's Cargo package version
is still 0.1.0; that is not the game/SDK version. Cargo.toml points directly to
this installed SDK. No downloaded older SDK is retained as another build input.
GitHub releases/latest still selects 0.5.8; Steam's installed SDK is newer.

Official documentation checkout `.mod-docs` was refreshed to `064c2fa`.
See [migration policy](stable-sdk-migration.md) and
[new API findings](sdk-0.6.4-findings.md).

```powershell
cargo test --release --manifest-path mod/pokemon_moba_stable/Cargo.toml
.\tools\build-native.ps1
.\tools\deploy.ps1
```

When updating, remove obsolete downloaded SDK archives/trees, refresh version
references, and ensure Cargo and the build script select the same current crate.
The stable SDK does not need the retired classic nightly toolchain pin.
