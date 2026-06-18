#!/usr/bin/env python3
"""Copy one animation tag's pixels from a source sheet into a target sheet."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image


def load_anim(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def sheet_path(base: Path) -> Path:
    return base.with_name(base.name + "#sheet.png")


def anim_path(base: Path) -> Path:
    return base.with_name(base.name + "#anim.fanim")


def frame_box(frame: dict) -> tuple[int, int, int, int]:
    data = frame["data"]
    x = int(data["x"])
    y = int(data["y"])
    return x, y, x + int(data["w"]), y + int(data["h"])


def copy_tag(source_base: Path, target_base: Path, tag: str) -> None:
    source_anim = load_anim(anim_path(source_base))
    target_anim = load_anim(anim_path(target_base))
    source_frames = source_anim["anims"][tag]["frames"]
    target_frames = target_anim["anims"][tag]["frames"]
    if len(source_frames) != len(target_frames):
        raise ValueError(
            f"Frame count mismatch for {tag}: {len(source_frames)} source, {len(target_frames)} target"
        )

    source_sheet = Image.open(sheet_path(source_base)).convert("RGBA")
    target_sheet = Image.open(sheet_path(target_base)).convert("RGBA")
    for source_frame, target_frame in zip(source_frames, target_frames):
        crop = source_sheet.crop(frame_box(source_frame))
        target_sheet.alpha_composite(crop, frame_box(target_frame)[:2])
    target_sheet.save(sheet_path(target_base))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, type=Path)
    parser.add_argument("--target", required=True, type=Path)
    parser.add_argument("--tag", required=True)
    args = parser.parse_args()
    copy_tag(args.source, args.target, args.tag)


if __name__ == "__main__":
    main()
