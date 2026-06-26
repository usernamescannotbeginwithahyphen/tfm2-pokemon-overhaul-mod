#!/usr/bin/env python3
"""Scale or shift visible pixels inside a TFM2 #sheet.png in place."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image


def rect_key(rect: dict) -> tuple[int, int, int, int]:
    return tuple(int(float(rect[key])) for key in ("x", "y", "w", "h"))


def transform_frame(frame: Image.Image, scale: float, shift_y: int) -> Image.Image:
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    bbox = frame.getbbox()
    if not bbox:
        return out

    visible = frame.crop(bbox)
    if scale != 1.0:
        new_size = (
            max(1, round(visible.width * scale)),
            max(1, round(visible.height * scale)),
        )
        visible = visible.resize(new_size, Image.Resampling.NEAREST)

    old_center_x = (bbox[0] + bbox[2]) / 2
    old_bottom = bbox[3]
    x = round(old_center_x - visible.width / 2)
    y = round(old_bottom - visible.height + shift_y)

    out.alpha_composite(visible, (x, y))
    return out


def transform_sheet(base: Path, scale: float, shift_y: int) -> None:
    sheet_path = base.with_name(base.name + "#sheet.png")
    anim_path = base.with_name(base.name + "#anim.fanim")
    if not sheet_path.exists():
        raise FileNotFoundError(sheet_path)
    if not anim_path.exists():
        raise FileNotFoundError(anim_path)

    data = json.loads(anim_path.read_text(encoding="utf-8"))
    rects: list[tuple[int, int, int, int]] = []
    seen: set[tuple[int, int, int, int]] = set()
    for anim in data.get("anims", {}).values():
        for frame in anim.get("frames", []):
            key = rect_key(frame["data"])
            if key not in seen:
                seen.add(key)
                rects.append(key)

    sheet = Image.open(sheet_path).convert("RGBA")
    out = Image.new("RGBA", sheet.size, (0, 0, 0, 0))
    for x, y, w, h in rects:
        crop = sheet.crop((x, y, x + w, y + h))
        out.alpha_composite(transform_frame(crop, scale, shift_y), (x, y))

    out.save(sheet_path)
    print(f"OK {sheet_path} scale={scale:g} shift_y={shift_y}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", type=Path, help="Base path without #sheet.png or #anim.fanim.")
    parser.add_argument("--scale", type=float, default=1.0, help="Visible-pixel scale factor.")
    parser.add_argument("--shift-y", type=int, default=0, help="Vertical pixel shift; negative moves content up.")
    args = parser.parse_args()

    if args.scale <= 0:
        raise SystemExit("--scale must be positive.")
    transform_sheet(args.base, args.scale, args.shift_y)


if __name__ == "__main__":
    main()
