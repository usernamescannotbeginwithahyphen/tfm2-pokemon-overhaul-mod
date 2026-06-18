#!/usr/bin/env python3
"""Generate a Teamfight Manager 2 minion animation sheet from Falinks."""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

from showdown_sprites import extract_frames, normalize_frames


MINION_TAGS = (
    "melee_blue_idle",
    "melee_blue_run",
    "melee_blue_attack",
    "melee_blue_dead",
    "melee_blue_morgard_idle",
    "melee_blue_morgard_run",
    "melee_blue_morgard_attack",
    "melee_blue_morgard_dead",
    "range_blue_idle",
    "range_blue_run",
    "range_blue_attack",
    "range_blue_dead",
    "range_blue_morgard_idle",
    "range_blue_morgard_run",
    "range_blue_morgard_attack",
    "range_blue_morgard_dead",
    "melee_red_idle",
    "melee_red_run",
    "melee_red_attack",
    "melee_red_dead",
    "melee_red_morgard_idle",
    "melee_red_morgard_run",
    "melee_red_morgard_attack",
    "melee_red_morgard_dead",
    "range_red_idle",
    "range_red_run",
    "range_red_attack",
    "range_red_dead",
    "range_red_morgard_idle",
    "range_red_morgard_run",
    "range_red_morgard_attack",
    "range_red_morgard_dead",
)


@dataclass
class AnimFrame:
    tag: str
    duration: float
    image: Image.Image


def colorize_team(frame: Image.Image, color: tuple[int, int, int, int], buffed: bool) -> Image.Image:
    result = Image.new("RGBA", frame.size, (0, 0, 0, 0))

    alpha = frame.getchannel("A")
    outline_alpha = alpha.filter(ImageFilter.MaxFilter(5))
    outline = Image.new("RGBA", frame.size, color)
    outline.putalpha(outline_alpha.point(lambda value: min(value, color[3])))
    result.alpha_composite(outline)

    draw = ImageDraw.Draw(result, "RGBA")
    w, h = frame.size
    y = int(h * 0.79)
    draw.ellipse(
        (int(w * 0.2), y, int(w * 0.8), min(h - 1, y + max(3, int(h * 0.09)))),
        fill=(color[0], color[1], color[2], 70 if buffed else 45),
    )
    if buffed:
        draw.ellipse(
            (int(w * 0.14), int(h * 0.16), int(w * 0.86), int(h * 0.88)),
            outline=(color[0], color[1], color[2], 95),
            width=2,
        )

    result.alpha_composite(frame)
    return result


def transform_action(frame: Image.Image, tag: str, index: int) -> Image.Image:
    if "_attack" not in tag:
        return frame

    offset = 2 if index % 2 else 0
    shifted = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    shifted.alpha_composite(frame, (offset, 0))
    return shifted


def make_projectile_frames(color: tuple[int, int, int, int], tag: str) -> list[AnimFrame]:
    frames: list[AnimFrame] = []
    for i, radius in enumerate((3, 4, 3)):
        image = Image.new("RGBA", (18, 18), (0, 0, 0, 0))
        draw = ImageDraw.Draw(image, "RGBA")
        center = 9
        draw.ellipse(
            (center - radius, center - radius, center + radius, center + radius),
            fill=(color[0], color[1], color[2], 210),
            outline=(255, 255, 255, 170),
        )
        frames.append(AnimFrame(tag, 0.1, image))
    return frames


def make_hit_frames(color: tuple[int, int, int, int], tag: str) -> list[AnimFrame]:
    frames: list[AnimFrame] = []
    for radius in (5, 7, 9):
        image = Image.new("RGBA", (24, 24), (0, 0, 0, 0))
        draw = ImageDraw.Draw(image, "RGBA")
        center = 12
        draw.ellipse(
            (center - radius, center - radius, center + radius, center + radius),
            outline=(color[0], color[1], color[2], 180),
            width=2,
        )
        frames.append(AnimFrame(tag, 0.1, image))
    return frames


def pack_frames(frames: list[AnimFrame], max_width: int) -> tuple[Image.Image, dict[str, dict[str, list[dict]]]]:
    x = 0
    y = 0
    row_h = 0
    padding = 1
    placements: list[tuple[AnimFrame, int, int]] = []

    for frame in frames:
        w, h = frame.image.size
        if x > 0 and x + w > max_width:
            x = 0
            y += row_h + padding
            row_h = 0
        placements.append((frame, x, y))
        x += w + padding
        row_h = max(row_h, h)

    sheet_w = max((px + frame.image.width for frame, px, _ in placements), default=1)
    sheet_h = max((py + frame.image.height for frame, _, py in placements), default=1)
    sheet = Image.new("RGBA", (sheet_w, sheet_h), (0, 0, 0, 0))
    anims: dict[str, dict[str, list[dict]]] = {}

    for frame, px, py in placements:
        sheet.alpha_composite(frame.image, (px, py))
        anims.setdefault(frame.tag, {"frames": []})["frames"].append(
            {
                "duration": frame.duration,
                "data": {
                    "x": px,
                    "y": py,
                    "w": frame.image.width,
                    "h": frame.image.height,
                },
            }
        )

    return sheet, anims


def generate(args: argparse.Namespace) -> None:
    raw_frames, durations = extract_frames(args.gif)
    if args.source_crop:
        x, y, w, h = parse_crop(args.source_crop)
        raw_frames = [frame.crop((x, y, x + w, y + h)) for frame in raw_frames]

    normalized = normalize_frames(
        raw_frames,
        args.canvas_size,
        args.max_content_size,
        args.bottom_padding,
        "bottom",
    )

    blue = (78, 125, 255, 115)
    red = (246, 75, 88, 115)

    anim_frames: list[AnimFrame] = []
    for tag in MINION_TAGS:
        team_color = blue if "_blue_" in tag else red
        buffed = "_morgard_" in tag
        if "_idle" in tag:
            source_indices = [0]
        elif "_dead" in tag:
            source_indices = list(range(min(3, len(normalized))))
        else:
            source_indices = list(range(len(normalized)))

        for out_index, source_index in enumerate(source_indices):
            frame = normalized[source_index]
            frame = colorize_team(frame, team_color, buffed)
            frame = transform_action(frame, tag, out_index)
            duration = durations[source_index]
            anim_frames.append(AnimFrame(tag, duration, frame))

    anim_frames.extend(make_projectile_frames(blue, "blue_projectile"))
    anim_frames.extend(make_projectile_frames(red, "red_projectile"))
    anim_frames.extend(make_hit_frames(blue, "blue_hit_effect"))
    anim_frames.extend(make_hit_frames(red, "red_hit_effect"))

    sheet, anims = pack_frames(anim_frames, args.max_sheet_width)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(args.output.with_name(args.output.name + "#sheet.png"))
    with args.output.with_name(args.output.name + "#anim.fanim").open("w", encoding="utf-8") as f:
        json.dump({"anims": anims}, f, indent=2)
        f.write("\n")


def parse_crop(value: str) -> tuple[int, int, int, int]:
    parts = [int(part.strip()) for part in value.split(",")]
    if len(parts) != 4:
        raise argparse.ArgumentTypeError("Crop must be x,y,w,h.")
    return parts[0], parts[1], parts[2], parts[3]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--gif", type=Path, default=Path("assets/source/showdown/ani/falinks.gif"))
    parser.add_argument("--output", type=Path, default=Path("mod/pokemon_moba/ingame/falinks_minion"))
    parser.add_argument("--canvas-size", type=int, default=44)
    parser.add_argument("--max-content-size", type=int, default=24)
    parser.add_argument("--bottom-padding", type=int, default=4)
    parser.add_argument("--max-sheet-width", type=int, default=2048)
    parser.add_argument(
        "--source-crop",
        default="0,0,36,49",
        help="Crop x,y,w,h from each source frame. Default extracts one Falinks body from the six-member train.",
    )
    generate(parser.parse_args())


if __name__ == "__main__":
    main()
