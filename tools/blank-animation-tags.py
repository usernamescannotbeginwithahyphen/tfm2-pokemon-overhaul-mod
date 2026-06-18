#!/usr/bin/env python3
"""Point selected .fanim tags at a transparent frame."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image


def blank_tags(base: Path, tags: list[str], duration: float) -> None:
    sheet_path = base.with_name(base.name + "#sheet.png")
    anim_path = base.with_name(base.name + "#anim.fanim")

    sheet = Image.open(sheet_path).convert("RGBA")
    transparent_x = sheet.width
    transparent_y = 0
    expanded = Image.new("RGBA", (sheet.width + 1, sheet.height), (0, 0, 0, 0))
    expanded.alpha_composite(sheet, (0, 0))
    expanded.save(sheet_path)

    with anim_path.open("r", encoding="utf-8") as f:
        anim = json.load(f)

    blank_frame = {
        "duration": duration,
        "data": {"x": transparent_x, "y": transparent_y, "w": 1, "h": 1},
    }
    anims = anim.setdefault("anims", {})
    for tag in tags:
        anims[tag] = {"frames": [blank_frame]}

    with anim_path.open("w", encoding="utf-8") as f:
        json.dump(anim, f, indent=2)
        f.write("\n")

    print(f"OK   {sheet_path}")
    print(f"OK   {anim_path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", type=Path)
    parser.add_argument("--tags", nargs="+", required=True)
    parser.add_argument("--duration", type=float, default=0.1)
    args = parser.parse_args()
    blank_tags(args.base, args.tags, args.duration)


if __name__ == "__main__":
    main()
