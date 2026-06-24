"""Convert PMDCollab animation sheets into staged TFM2 champion assets.

This converter keeps the original PMD cell spacing intact so attack reach and
body anchors survive the trip into TFM2's fixed-frame #sheet/#anim format.
"""

from __future__ import annotations

import argparse
import json
import math
import urllib.request
import xml.etree.ElementTree as ET
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw


RAW_BASE_URL = "https://raw.githubusercontent.com/PMDCollab/SpriteCollab/master/sprite"
DEFAULT_MAPPINGS = (
    "idle=Idle",
    "run=Walk",
    "attack=MultiStrike",
    "skill=Idle",
    "skill2=Attack",
    "ult=RearUp",
    "dead=Hurt",
)


@dataclass(frozen=True)
class AnimMeta:
    name: str
    frame_width: int
    frame_height: int
    durations: tuple[int, ...]


def parse_mapping(values: list[str]) -> dict[str, tuple[str, ...]]:
    mapping: dict[str, tuple[str, ...]] = {}
    for value in values:
        if "=" not in value:
            raise ValueError(f"Invalid mapping {value!r}; expected tag=PmdAnimName.")
        tag, anim = value.split("=", 1)
        tag = tag.strip()
        anim = anim.strip()
        if not tag or not anim:
            raise ValueError(f"Invalid mapping {value!r}; empty tag or animation name.")
        mapping[tag] = tuple(part.strip() for part in anim.split("+") if part.strip())
    return mapping


def parse_effects(values: list[str]) -> dict[str, str]:
    effects: dict[str, str] = {}
    for value in values:
        if "=" not in value:
            raise ValueError(f"Invalid effect {value!r}; expected tag=effect_name.")
        tag, effect = value.split("=", 1)
        tag = tag.strip()
        effect = effect.strip()
        if not tag or not effect:
            raise ValueError(f"Invalid effect {value!r}; empty tag or effect name.")
        effects[tag] = effect
    return effects


def parse_tag_durations(values: list[str]) -> dict[str, float]:
    durations: dict[str, float] = {}
    for value in values:
        if "=" not in value:
            raise ValueError(f"Invalid tag duration {value!r}; expected tag=seconds.")
        tag, seconds = value.split("=", 1)
        tag = tag.strip()
        if not tag:
            raise ValueError(f"Invalid tag duration {value!r}; empty tag.")
        durations[tag] = float(seconds)
    return durations


def parse_frame_selects(values: list[str]) -> dict[str, tuple[int, ...]]:
    selections: dict[str, tuple[int, ...]] = {}
    for value in values:
        if "=" not in value:
            raise ValueError(f"Invalid frame selection {value!r}; expected tag=0,1,2 or tag=0-4.")
        tag, spec = value.split("=", 1)
        tag = tag.strip()
        if not tag:
            raise ValueError(f"Invalid frame selection {value!r}; empty tag.")
        indices: list[int] = []
        for part in spec.split(","):
            part = part.strip()
            if not part:
                continue
            if "-" in part:
                start_text, end_text = part.split("-", 1)
                start = int(start_text)
                end = int(end_text)
                step = 1 if end >= start else -1
                indices.extend(range(start, end + step, step))
            else:
                indices.append(int(part))
        if not indices:
            raise ValueError(f"Invalid frame selection {value!r}; no indices.")
        selections[tag] = tuple(indices)
    return selections


def parse_tag_floats(values: list[str], option_name: str) -> dict[str, float]:
    parsed: dict[str, float] = {}
    for value in values:
        if "=" not in value:
            raise ValueError(f"Invalid {option_name} {value!r}; expected tag=value.")
        tag, number = value.split("=", 1)
        tag = tag.strip()
        if not tag:
            raise ValueError(f"Invalid {option_name} {value!r}; empty tag.")
        parsed[tag] = float(number)
    return parsed


def parse_anim_data(path: Path) -> dict[str, AnimMeta]:
    root = ET.parse(path).getroot()
    metas: dict[str, AnimMeta] = {}
    copies: dict[str, str] = {}
    for node in root.find("Anims").findall("Anim"):
        name = node.findtext("Name")
        copy_of = node.findtext("CopyOf")
        if copy_of:
            copies[name] = copy_of
            continue
        durations = tuple(int(item.text) for item in node.find("Durations").findall("Duration"))
        metas[name] = AnimMeta(
            name=name,
            frame_width=int(node.findtext("FrameWidth")),
            frame_height=int(node.findtext("FrameHeight")),
            durations=durations,
        )
    for name, source in copies.items():
        metas[name] = metas[source]
    return metas


def download_if_needed(species: str, source_dir: Path, anim_names: set[str]) -> None:
    source_dir.mkdir(parents=True, exist_ok=True)
    names = {"AnimData.xml", "credits.txt"}
    for anim_name in anim_names:
        names.add(f"{anim_name}-Anim.png")
        names.add(f"{anim_name}-Offsets.png")
        names.add(f"{anim_name}-Shadow.png")
    for name in sorted(names):
        out = source_dir / name
        if out.exists():
            continue
        url = f"{RAW_BASE_URL}/{species}/{name}"
        print(f"GET  {url}")
        try:
            urllib.request.urlretrieve(url, out)
        except Exception as exc:  # noqa: BLE001 - include URL context for art pipeline failures.
            if out.exists():
                out.unlink()
            raise RuntimeError(f"Failed to download {url}") from exc


def alpha_bbox(image: Image.Image) -> tuple[int, int, int, int] | None:
    return image.getchannel("A").getbbox()


def body_scale(source_dir: Path, metas: dict[str, AnimMeta], direction_row: int, max_content_size: int) -> float:
    max_body = 1
    for anim_name in ("Idle", "Walk"):
        meta = metas[anim_name]
        sheet = Image.open(source_dir / f"{anim_name}-Anim.png").convert("RGBA")
        for index in range(len(meta.durations)):
            cell = sheet.crop(
                (
                    index * meta.frame_width,
                    direction_row * meta.frame_height,
                    (index + 1) * meta.frame_width,
                    (direction_row + 1) * meta.frame_height,
                )
            )
            bbox = alpha_bbox(cell)
            if bbox:
                max_body = max(max_body, bbox[2] - bbox[0], bbox[3] - bbox[1])
    return max_content_size / max_body


def nearest_resize(image: Image.Image, scale: float) -> Image.Image:
    width = max(1, round(image.width * scale))
    height = max(1, round(image.height * scale))
    return image.resize((width, height), Image.Resampling.NEAREST)


def effect_anchor(frame: Image.Image, prefer_right: bool) -> tuple[int, int]:
    bbox = alpha_bbox(frame)
    if not bbox:
        return (frame.width // 2, frame.height // 2)
    x = bbox[2] - 6 if prefer_right else bbox[0] + 6
    y = bbox[1] + (bbox[3] - bbox[1]) // 2
    return (x, y)


def draw_tickle_cue(frame: Image.Image, frame_index: int, prefer_right: bool) -> None:
    draw = ImageDraw.Draw(frame, "RGBA")
    x, y = effect_anchor(frame, prefer_right)
    sign = 1 if prefer_right else -1
    reach = min(28, 10 + frame_index * 4)
    tail = (152, 86, 184, 230)
    hand = (255, 188, 63, 240)
    tickle = (255, 238, 126, 220)

    for idx, vertical in enumerate((-9, 8)):
        wiggle = -2 if (frame_index + idx) % 2 else 2
        start = (x - sign * 6, y + vertical)
        mid = (x + sign * (reach // 2), y + vertical + wiggle)
        end = (x + sign * reach, y + vertical - wiggle)
        draw.line((start, mid, end), fill=tail, width=3, joint="curve")
        draw.ellipse((end[0] - 3, end[1] - 3, end[0] + 3, end[1] + 3), fill=hand)
        if frame_index >= 2:
            for offset in (6, 11):
                px = end[0] + sign * offset
                points = [
                    (px, end[1] - 5),
                    (px + sign * 3, end[1] - 2 + wiggle),
                    (px, end[1] + 1),
                    (px + sign * 3, end[1] + 4 - wiggle),
                ]
                draw.line(points, fill=tickle, width=2)


def draw_thief_cue(frame: Image.Image, frame_index: int, prefer_right: bool) -> None:
    if frame_index < 1:
        return
    draw = ImageDraw.Draw(frame)
    x, y = effect_anchor(frame, prefer_right)
    sign = 1 if prefer_right else -1
    dark = (92, 54, 124, 220)
    gold = (255, 214, 90, 230)
    draw.arc((x - 10, y - 12, x + 10, y + 8), 295 if prefer_right else 115, 55 if prefer_right else 235, fill=dark, width=2)
    if frame_index in (2, 3, 4):
        draw.rectangle((x + sign * 6, y - 7, x + sign * 9, y - 4), fill=gold)
        draw.point((x + sign * 11, y - 6), fill=gold)


def draw_ult_aura(frame: Image.Image, frame_index: int) -> None:
    bbox = alpha_bbox(frame)
    if not bbox:
        return
    draw = ImageDraw.Draw(frame, "RGBA")
    cx = (bbox[0] + bbox[2]) // 2
    foot = bbox[3] - 2
    pulse = 1 + (frame_index % 4)
    draw.ellipse((cx - 19, foot - 5, cx + 19, foot + 5), outline=(255, 218, 82, 150), width=2)
    for angle in (250, 285, 320):
        length = 16 + pulse * 2
        rad = math.radians(angle)
        x2 = cx + round(math.cos(rad) * length)
        y2 = foot + round(math.sin(rad) * length)
        draw.line((cx, foot - 6, x2, y2), fill=(255, 240, 112, 120), width=2)


def draw_sweet_scent_aura(frame: Image.Image, frame_index: int) -> None:
    bbox = alpha_bbox(frame)
    if not bbox:
        return
    draw = ImageDraw.Draw(frame, "RGBA")
    cx = (bbox[0] + bbox[2]) // 2
    cy = bbox[1] + (bbox[3] - bbox[1]) // 2
    foot = bbox[3] - 4
    pulse = frame_index % 5
    colors = (
        (255, 171, 202, 150),
        (183, 238, 104, 145),
        (255, 231, 135, 120),
    )
    draw.ellipse((cx - 29, foot - 9 - pulse, cx + 29, foot + 7 + pulse), outline=colors[pulse % 3], width=2)
    draw.arc((cx - 24, cy - 23, cx + 24, cy + 25), 25 + pulse * 10, 150 + pulse * 10, fill=(255, 179, 211, 135), width=2)
    draw.arc((cx - 30, cy - 18, cx + 30, cy + 30), 205 - pulse * 8, 318 - pulse * 8, fill=(170, 235, 95, 125), width=2)
    mote_offsets = ((-23, -11), (-12, -24), (8, -21), (23, -8), (15, 12), (-18, 10))
    for index, (ox, oy) in enumerate(mote_offsets):
        if (index + frame_index) % 2 == 0:
            x = cx + ox
            y = cy + oy + ((frame_index + index) % 3) - 1
            draw.rectangle((x, y, x + 1, y + 1), fill=(255, 242, 159, 180))


def draw_venoshock_cast(frame: Image.Image, frame_index: int) -> None:
    bbox = alpha_bbox(frame)
    if not bbox:
        return
    draw = ImageDraw.Draw(frame, "RGBA")
    cx = (bbox[0] + bbox[2]) // 2
    cy = bbox[1] + (bbox[3] - bbox[1]) // 2
    pulse = frame_index % 4
    draw.arc((cx - 27, cy - 24, cx + 27, cy + 24), 20 + pulse * 14, 170 + pulse * 14, fill=(179, 66, 214, 155), width=2)
    draw.arc((cx - 31, cy - 18, cx + 31, cy + 29), 205 - pulse * 11, 340 - pulse * 11, fill=(93, 218, 87, 140), width=2)
    for ox, oy in ((-22, -12), (-7, -26), (12, -20), (24, -2), (10, 18), (-19, 11)):
        if (ox + oy + frame_index) % 2 == 0:
            x = cx + ox
            y = cy + oy
            draw.rectangle((x, y, x + 2, y + 2), fill=(134, 247, 94, 180))


def draw_electro_shot_aura(frame: Image.Image, frame_index: int) -> None:
    bbox = alpha_bbox(frame)
    if not bbox:
        return
    draw = ImageDraw.Draw(frame, "RGBA")
    cx = (bbox[0] + bbox[2]) // 2
    cy = bbox[1] + (bbox[3] - bbox[1]) // 2
    foot = bbox[3] - 3
    pulse = frame_index % 6
    ring = (72, 214, 255, 135 + (pulse % 3) * 18)
    bolt = (255, 244, 114, 205)
    draw.ellipse((cx - 28 - pulse, foot - 12, cx + 28 + pulse, foot + 8), outline=ring, width=2)
    for index, ox in enumerate((-24, -12, 3, 18, 29)):
        if (index + frame_index) % 2 == 0:
            x = cx + ox
            y = cy - 22 + ((frame_index + index) % 4) * 3
            draw.line((x, y, x + 4, y + 5, x - 1, y + 9), fill=bolt, width=2)
    if frame_index >= 2:
        draw.arc((cx - 35, cy - 28, cx + 35, cy + 30), 195 + pulse * 8, 345 + pulse * 8, fill=(96, 231, 255, 110), width=2)


def scale_tag_travel(frames: list[Image.Image], factor: float) -> list[Image.Image]:
    if factor >= 0.999:
        return frames
    centers: list[float] = []
    for frame in frames:
        bbox = alpha_bbox(frame)
        if bbox:
            centers.append((bbox[0] + bbox[2]) / 2)
    if not centers:
        return frames
    anchor = sorted(centers)[len(centers) // 2]
    scaled: list[Image.Image] = []
    for frame in frames:
        bbox = alpha_bbox(frame)
        if not bbox:
            scaled.append(frame)
            continue
        center = (bbox[0] + bbox[2]) / 2
        shift = round((center - anchor) * (factor - 1.0))
        if shift == 0:
            scaled.append(frame)
            continue
        moved = Image.new("RGBA", frame.size, (0, 0, 0, 0))
        moved.alpha_composite(frame, (shift, 0))
        scaled.append(moved)
    return scaled


def render_cell(
    cell: Image.Image,
    scale: float,
    canvas_size: int,
    bottom_padding: int,
    effect: str | None,
    frame_index: int,
    prefer_right: bool,
    recenter_x: bool,
) -> Image.Image:
    resized = nearest_resize(cell, scale)
    frame = Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0))
    bbox = alpha_bbox(resized)
    if bbox:
        if recenter_x:
            x = (canvas_size - (bbox[2] - bbox[0])) // 2 - bbox[0]
        else:
            x = (canvas_size - resized.width) // 2
        y = canvas_size - bottom_padding - bbox[3]
    else:
        x = (canvas_size - resized.width) // 2
        y = canvas_size - bottom_padding - resized.height
    frame.alpha_composite(resized, (x, y))
    if effect == "tickle":
        draw_tickle_cue(frame, frame_index, prefer_right)
    elif effect == "thief":
        draw_thief_cue(frame, frame_index, prefer_right)
    elif effect == "ult_aura":
        draw_ult_aura(frame, frame_index)
    elif effect == "sweet_scent_aura":
        draw_sweet_scent_aura(frame, frame_index)
    elif effect == "venoshock_cast":
        draw_venoshock_cast(frame, frame_index)
    elif effect == "electro_shot_aura":
        draw_electro_shot_aura(frame, frame_index)
    return frame


def build_assets(args: argparse.Namespace) -> None:
    source_dir = args.source_dir
    mapping = parse_mapping(args.map)
    effects = parse_effects(args.effect)
    tag_durations = parse_tag_durations(args.tag_duration)
    frame_selects = parse_frame_selects(args.frame_select)
    recenter_tags = set(args.recenter_tag)
    travel_scales = parse_tag_floats(args.travel_scale, "--travel-scale")
    anim_names = {anim_name for sequence in mapping.values() for anim_name in sequence}
    download_if_needed(args.species, source_dir, anim_names)
    metas = parse_anim_data(source_dir / "AnimData.xml")
    scale = body_scale(source_dir, metas, args.direction_row, args.max_content_size)

    output_base = args.output_base
    output_base.parent.mkdir(parents=True, exist_ok=True)
    frames: list[Image.Image] = []
    fanim: dict[str, dict[str, list[dict[str, object]]]] = {"anims": {}}
    x = 0

    for tag, anim_sequence in mapping.items():
        effect = effects.get(tag)
        tag_frames: list[dict[str, object]] = []
        rendered_index = 0
        pending_frames: list[Image.Image] = []
        pending_durations: list[int] = []
        for anim_name in anim_sequence:
            meta = metas[anim_name]
            sheet = Image.open(source_dir / f"{anim_name}-Anim.png").convert("RGBA")
            row = min(args.direction_row, max(0, sheet.height // meta.frame_height - 1))
            frame_indices = list(range(len(meta.durations)))
            durations = list(meta.durations)
            if effect == "tickle":
                frame_indices = [0] * 8
                durations = [3, 3, 3, 3, 3, 3, 3, 4]
            elif tag in frame_selects:
                selected = frame_selects[tag]
                frame_indices = [index for index in selected if 0 <= index < len(meta.durations)]
                durations = [meta.durations[index] for index in frame_indices]
            for source_index, duration in zip(frame_indices, durations, strict=True):
                cell = sheet.crop(
                    (
                        source_index * meta.frame_width,
                        row * meta.frame_height,
                        (source_index + 1) * meta.frame_width,
                        (row + 1) * meta.frame_height,
                    )
                )
                pending_frames.append(
                    render_cell(
                        cell,
                        scale,
                        args.canvas_size,
                        args.bottom_padding,
                        effect,
                        rendered_index,
                        args.prefer_right,
                        tag in recenter_tags,
                    )
                )
                pending_durations.append(duration)
                rendered_index += 1

        if tag in travel_scales:
            pending_frames = scale_tag_travel(pending_frames, travel_scales[tag])

        if tag in tag_durations and pending_durations:
            total_source = sum(max(1, duration) for duration in pending_durations)
            total_seconds = tag_durations[tag]
            frame_seconds = [
                max(args.min_duration, total_seconds * max(1, duration) / total_source)
                for duration in pending_durations
            ]
        else:
            frame_seconds = [max(args.min_duration, duration / args.fps) for duration in pending_durations]

        for rendered, seconds in zip(pending_frames, frame_seconds, strict=True):
            frames.append(rendered)
            tag_frames.append(
                {
                    "duration": round(seconds, 4),
                    "data": {"x": x, "y": 0, "w": args.canvas_size, "h": args.canvas_size},
                }
            )
            x += args.canvas_size
        fanim["anims"][tag] = {"frames": tag_frames}

    atlas = Image.new("RGBA", (len(frames) * args.canvas_size, args.canvas_size), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        atlas.alpha_composite(frame, (index * args.canvas_size, 0))

    atlas.save(output_base.with_name(output_base.name + "#sheet.png"))
    with output_base.with_name(output_base.name + "#anim.fanim").open("w", encoding="utf-8") as f:
        json.dump(fanim, f, indent=2)
        f.write("\n")
    print(f"OK   wrote {output_base.with_name(output_base.name + '#sheet.png')}")
    print(f"OK   wrote {output_base.with_name(output_base.name + '#anim.fanim')}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--species", required=True, help="PMDCollab species folder, e.g. 0424.")
    parser.add_argument("--source-dir", type=Path, required=True, help="Local PMDCollab cache folder.")
    parser.add_argument("--output-base", type=Path, required=True, help="Output base without #sheet/#anim suffix.")
    parser.add_argument("--map", action="append", default=list(DEFAULT_MAPPINGS), help="TFM2 tag to PMD anim mapping.")
    parser.add_argument("--effect", action="append", default=[], help="Optional per-tag overlay, e.g. skill2=sweet_scent_aura.")
    parser.add_argument("--tag-duration", action="append", default=[], help="Optional total duration override in seconds, e.g. skill=0.8667.")
    parser.add_argument("--frame-select", action="append", default=[], help="Optional per-tag source frame subset, e.g. skill=0-5 or skill=0,1,2.")
    parser.add_argument("--recenter-tag", action="append", default=[], help="Recenter a tag horizontally by visible pixels instead of preserving PMD cell offset.")
    parser.add_argument("--travel-scale", action="append", default=[], help="Scale a tag's horizontal body travel around its median center, e.g. skill2=0.45.")
    parser.add_argument("--direction-row", type=int, default=1, help="Zero-based PMD direction row. Index 1 is the second visible row and preferred right-facing three-quarter view; index 2 is pure right side-view.")
    parser.add_argument("--canvas-size", type=int, default=96)
    parser.add_argument("--max-content-size", type=int, default=38)
    parser.add_argument("--bottom-padding", type=int, default=14)
    parser.add_argument("--fps", type=float, default=60.0, help="PMD duration unit conversion.")
    parser.add_argument("--min-duration", type=float, default=0.0333)
    parser.add_argument("--prefer-left", dest="prefer_right", action="store_false", help="Place effect cues to the left.")
    parser.set_defaults(prefer_right=True)
    build_assets(parser.parse_args())


if __name__ == "__main__":
    main()
