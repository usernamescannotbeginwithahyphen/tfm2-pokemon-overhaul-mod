"""Append one tag from a source TFM2 sheet/fanim into a target sheet/fanim."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image


def sheet_path(base: Path) -> Path:
    return base.with_name(base.name + "#sheet.png")


def anim_path(base: Path) -> Path:
    return base.with_name(base.name + "#anim.fanim")


def load_anims(base: Path) -> dict:
    with anim_path(base).open("r", encoding="utf-8") as f:
        return json.load(f)


def replace_tag(args: argparse.Namespace) -> None:
    target_sheet = Image.open(sheet_path(args.target_base)).convert("RGBA")
    source_sheet = Image.open(sheet_path(args.source_base)).convert("RGBA")
    target_anim = load_anims(args.target_base)
    source_anim = load_anims(args.source_base)

    source_tag = args.source_tag or args.tag
    if source_tag not in source_anim["anims"]:
        raise ValueError(f"{anim_path(args.source_base)} has no tag {source_tag!r}.")

    x_offset = target_sheet.width
    merged_sheet = Image.new(
        "RGBA",
        (target_sheet.width + source_sheet.width, max(target_sheet.height, source_sheet.height)),
        (0, 0, 0, 0),
    )
    merged_sheet.alpha_composite(target_sheet, (0, 0))
    merged_sheet.alpha_composite(source_sheet, (x_offset, 0))

    frames = []
    for frame in source_anim["anims"][source_tag]["frames"]:
        copied = dict(frame)
        data = dict(copied["data"])
        data["x"] = int(float(data["x"])) + x_offset
        copied["data"] = data
        frames.append(copied)

    target_anim["anims"][args.tag] = {"frames": frames}
    merged_sheet.save(sheet_path(args.target_base))
    with anim_path(args.target_base).open("w", encoding="utf-8") as f:
        json.dump(target_anim, f, indent=2)
        f.write("\n")
    print(f"OK   replaced {args.tag} in {anim_path(args.target_base)}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target-base", type=Path, required=True)
    parser.add_argument("--source-base", type=Path, required=True)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--source-tag")
    replace_tag(parser.parse_args())


if __name__ == "__main__":
    main()
