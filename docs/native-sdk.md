# Native SDK

Installed SDK version: `0.5.0_hotfix` release, reporting base SDK `0.5.0`

Source release:

```text
https://github.com/teamsamoyed/TeamfightManager2Mod/releases/tag/0.5.0_hotfix
```

Verified release asset:

```text
0.5.0_hotfix.zip
sha256:69b2ec48f93cbe4a4dfa2659d36561ab8e4b2a67647cda877e8229ede858f41e
```

Local install path expected by the uploader:

```text
C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mod-sdk
```

The SDK reports:

```text
base_version.txt: 0.5.0
toolchain_version.txt: rustc 1.98.0-nightly (23a3312d9 2026-05-23)
```

Project toolchain pin:

```text
mod/pokemon_moba/rust-toolchain.toml: nightly-2026-05-24-x86_64-pc-windows-msvc
```

Retained SDK cache:

```text
downloads\tfm2-sdk\0.5.0_hotfix
```

## Update Policy

When updating SDK versions, keep exactly one retained SDK cache: the currently installed release under `downloads\tfm2-sdk\<version>`. Remove older SDK folders/zips, aborted temp extracts, and stale docs/tool references in the same change. `tools\build-native.ps1`, `mod\pokemon_moba\rust-toolchain.toml`, `mod\pokemon_moba\mod.mod_info`, and this document must all point at the active SDK version.
