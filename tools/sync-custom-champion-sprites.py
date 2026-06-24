#!/usr/bin/env python3
"""Sync staged custom Pokemon champion sprites into the runtime mod package."""

from __future__ import annotations

import json
import re
import shutil
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SRC_RS = ROOT / "mod" / "pokemon_moba" / "src" / "pokemon_content.rs"
STAGED_DIR = ROOT / "assets" / "custom_spritework" / "champions"
RUNTIME_DIR = ROOT / "mod" / "pokemon_moba" / "champions_custom"
OVERRIDE_INFO = ROOT / "mod" / "pokemon_moba" / "mod.override_info"


def roster_ids() -> list[str]:
    text = SRC_RS.read_text(encoding="utf-8")
    seen: set[str] = set()
    ids: list[str] = []
    for match in re.finditer(r'id:\s*"pokemon_moba_([a-z0-9_\-]+)"', text):
        short_id = match.group(1)
        if short_id not in seen:
            seen.add(short_id)
            ids.append(short_id)
    return ids


def sync_files(ids: list[str]) -> None:
    missing: list[str] = []
    RUNTIME_DIR.mkdir(parents=True, exist_ok=True)

    for short_id in ids:
        for suffix in ("sheet.png", "anim.fanim"):
            src = STAGED_DIR / f"{short_id}#{suffix}"
            if not src.exists():
                missing.append(str(src.relative_to(ROOT)))
                continue
            dst = RUNTIME_DIR / f"{short_id}#{suffix}"
            shutil.copy2(src, dst)

    if missing:
        details = "\n".join(f"- {path}" for path in missing)
        raise SystemExit(f"Missing staged custom sprite assets:\n{details}")


def normalize_overrides(ids: list[str]) -> None:
    data = json.loads(OVERRIDE_INFO.read_text(encoding="utf-8"))
    remap = data.setdefault("remap", {})

    for short_id in ids:
        base = f"asset/base/aseprite_resources/champions/pokemon_moba_{short_id}"
        custom = f"asset/pokemon_moba/champions_custom/{short_id}"
        remap[f"{base}#sheet"] = f"{custom}#sheet"
        remap[f"{base}#anim"] = f"{custom}#anim"

    stale = [
        key
        for key, value in remap.items()
        if isinstance(value, str) and value.startswith("asset/pokemon_moba/champions/")
    ]
    if stale:
        details = "\n".join(f"- {key} -> {remap[key]}" for key in stale)
        raise SystemExit(f"Refusing to leave stale Smogon champion remaps:\n{details}")

    OVERRIDE_INFO.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    ids = roster_ids()
    if not ids:
        raise SystemExit("No pokemon_moba champion IDs found.")
    sync_files(ids)
    normalize_overrides(ids)
    print(f"Synced {len(ids)} custom champion sprite pairs into {RUNTIME_DIR.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
