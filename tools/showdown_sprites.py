#!/usr/bin/env python
"""
Download Pokemon Showdown animated sprites and convert GIFs into Teamfight
Manager 2 manual animation assets.

The generated files use the TFM2 naming convention:
  <pokemon>#sheet.png
  <pokemon>#anim.fanim
"""

from __future__ import annotations

import argparse
import html.parser
import json
import math
import re
import sys
import urllib.request
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable

from PIL import Image, ImageSequence


DEFAULT_INDEX_URL = "https://play.pokemonshowdown.com/sprites/ani/?view=dir"
DEFAULT_SPRITE_BASE_URL = "https://play.pokemonshowdown.com/sprites/ani/"
DEFAULT_ACTION_TAGS = ("idle", "run", "attack", "skill", "skill2", "ult", "dead")
USER_AGENT = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) TFM2-Pokemon-Mod-AssetTool/0.1"


@dataclass(frozen=True)
class SpriteEntry:
    name: str
    file: str
    url: str


class DirectoryParser(html.parser.HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.links: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag != "a":
            return
        attr_map = dict(attrs)
        href = attr_map.get("href")
        if href and href.lower().endswith(".gif"):
            self.links.append(href)


def slug_from_file(file_name: str) -> str:
    stem = Path(file_name).stem.lower()
    return re.sub(r"[^a-z0-9_]+", "_", stem).strip("_")


def fetch_text(url: str) -> str:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read().decode("utf-8", errors="replace")


def fetch_bytes(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=120) as response:
        return response.read()


def read_index(index_url: str, sprite_base_url: str) -> list[SpriteEntry]:
    parser = DirectoryParser()
    parser.feed(fetch_text(index_url))
    entries: list[SpriteEntry] = []
    seen: set[str] = set()
    for href in parser.links:
        file_name = href.rsplit("/", 1)[-1]
        if file_name in seen:
            continue
        seen.add(file_name)
        entries.append(
            SpriteEntry(
                name=slug_from_file(file_name),
                file=file_name,
                url=sprite_base_url.rstrip("/") + "/" + file_name,
            )
        )
    return sorted(entries, key=lambda entry: entry.file)


def load_manifest(path: Path) -> list[SpriteEntry]:
    with path.open("r", encoding="utf-8") as f:
        raw = json.load(f)
    return [SpriteEntry(**entry) for entry in raw["sprites"]]


def write_manifest(entries: list[SpriteEntry], path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        json.dump(
            {
                "source": DEFAULT_INDEX_URL,
                "count": len(entries),
                "sprites": [asdict(entry) for entry in entries],
            },
            f,
            indent=2,
        )
        f.write("\n")


def select_entries(entries: list[SpriteEntry], names: Iterable[str] | None) -> list[SpriteEntry]:
    if not names:
        return entries
    wanted = {slug_from_file(name) for name in names}
    selected = [entry for entry in entries if entry.name in wanted or slug_from_file(entry.file) in wanted]
    missing = sorted(wanted - {entry.name for entry in selected})
    if missing:
        raise SystemExit(f"Missing sprite(s) in manifest: {', '.join(missing)}")
    return selected


def download_entries(entries: list[SpriteEntry], output_dir: Path, overwrite: bool = False) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    for entry in entries:
        target = output_dir / entry.file
        if target.exists() and not overwrite:
            print(f"SKIP {target}")
            continue
        print(f"GET  {entry.url}")
        target.write_bytes(fetch_bytes(entry.url))
        print(f"OK   {target}")


def frame_duration_seconds(frame: Image.Image) -> float:
    duration_ms = frame.info.get("duration", 100)
    if not isinstance(duration_ms, int | float) or duration_ms <= 0:
        duration_ms = 100
    return round(float(duration_ms) / 1000.0, 4)


def alpha_bbox(image: Image.Image) -> tuple[int, int, int, int] | None:
    return image.getchannel("A").getbbox()


def union_bbox(boxes: Iterable[tuple[int, int, int, int]]) -> tuple[int, int, int, int] | None:
    boxes = list(boxes)
    if not boxes:
        return None
    return (
        min(box[0] for box in boxes),
        min(box[1] for box in boxes),
        max(box[2] for box in boxes),
        max(box[3] for box in boxes),
    )


def extract_frames(gif_path: Path) -> tuple[list[Image.Image], list[float]]:
    with Image.open(gif_path) as image:
        frames: list[Image.Image] = []
        durations: list[float] = []
        for frame in ImageSequence.Iterator(image):
            frames.append(frame.convert("RGBA"))
            durations.append(frame_duration_seconds(frame))
    if not frames:
        raise ValueError(f"No frames found in {gif_path}")
    return frames, durations


def normalize_frames(
    frames: list[Image.Image],
    canvas_size: int,
    max_content_size: int,
    bottom_padding: int,
    anchor: str,
) -> list[Image.Image]:
    content_box = union_bbox(box for frame in frames if (box := alpha_bbox(frame)) is not None)
    if content_box is None:
        return [Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0)) for _ in frames]

    content_w = max(1, content_box[2] - content_box[0])
    content_h = max(1, content_box[3] - content_box[1])
    scale = min(max_content_size / content_w, max_content_size / content_h)
    target_w = max(1, int(round(content_w * scale)))
    target_h = max(1, int(round(content_h * scale)))

    normalized: list[Image.Image] = []
    for frame in frames:
        cropped = frame.crop(content_box)
        if cropped.size != (target_w, target_h):
            cropped = cropped.resize((target_w, target_h), Image.Resampling.NEAREST)
        canvas = Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0))
        x = (canvas_size - target_w) // 2
        if anchor == "center":
            y = (canvas_size - target_h) // 2
        elif anchor == "bottom":
            y = canvas_size - bottom_padding - target_h
            if y < 0:
                y = 0
        else:
            raise ValueError(f"Unknown anchor: {anchor}")
        canvas.alpha_composite(cropped, (x, y))
        normalized.append(canvas)
    return normalized


def make_sheet(frames: list[Image.Image], max_columns: int) -> tuple[Image.Image, list[dict[str, int]]]:
    frame_w, frame_h = frames[0].size
    columns = max(1, min(max_columns, len(frames)))
    rows = math.ceil(len(frames) / columns)
    sheet = Image.new("RGBA", (columns * frame_w, rows * frame_h), (0, 0, 0, 0))
    rects: list[dict[str, int]] = []
    for index, frame in enumerate(frames):
        col = index % columns
        row = index // columns
        x = col * frame_w
        y = row * frame_h
        sheet.alpha_composite(frame, (x, y))
        rects.append({"x": x, "y": y, "w": frame_w, "h": frame_h})
    return sheet, rects


def convert_gif(
    gif_path: Path,
    output_base: Path,
    action_tags: tuple[str, ...],
    canvas_size: int,
    max_content_size: int,
    bottom_padding: int,
    max_columns: int,
    anchor: str,
    overwrite: bool = False,
) -> None:
    sheet_path = output_base.with_name(output_base.name + "#sheet.png")
    anim_path = output_base.with_name(output_base.name + "#anim.fanim")
    if (sheet_path.exists() or anim_path.exists()) and not overwrite:
        print(f"SKIP {output_base}")
        return

    frames, durations = extract_frames(gif_path)
    frames = normalize_frames(frames, canvas_size, max_content_size, bottom_padding, anchor)
    sheet, rects = make_sheet(frames, max_columns)

    anim_frames = [
        {"duration": duration, "data": rect}
        for duration, rect in zip(durations, rects, strict=True)
    ]
    anim = {"anims": {tag: {"frames": anim_frames} for tag in action_tags}}

    output_base.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(sheet_path)
    with anim_path.open("w", encoding="utf-8") as f:
        json.dump(anim, f, indent=2)
        f.write("\n")

    print(f"OK   {sheet_path}")
    print(f"OK   {anim_path}")


def command_index(args: argparse.Namespace) -> None:
    entries = read_index(args.index_url, args.sprite_base_url)
    write_manifest(entries, args.manifest)
    print(f"Wrote {len(entries)} entries to {args.manifest}")


def command_download(args: argparse.Namespace) -> None:
    entries = select_entries(load_manifest(args.manifest), args.names)
    if args.limit is not None:
        entries = entries[: args.limit]
    download_entries(entries, args.output_dir, args.overwrite)


def command_convert(args: argparse.Namespace) -> None:
    action_tags = tuple(tag.strip() for tag in args.action_tags.split(",") if tag.strip())
    if not action_tags:
        raise SystemExit("At least one action tag is required.")

    for gif_path in args.gif:
        name = args.name or slug_from_file(gif_path.name)
        output_base = args.output_dir / name
        convert_gif(
            gif_path,
            output_base,
            action_tags,
            args.canvas_size,
            args.max_content_size,
            args.bottom_padding,
            args.max_columns,
            args.anchor,
            args.overwrite,
        )


def command_batch_convert(args: argparse.Namespace) -> None:
    action_tags = tuple(tag.strip() for tag in args.action_tags.split(",") if tag.strip())
    entries = select_entries(load_manifest(args.manifest), args.names)
    if args.limit is not None:
        entries = entries[: args.limit]

    for entry in entries:
        gif_path = args.input_dir / entry.file
        if not gif_path.exists():
            print(f"MISS {gif_path}", file=sys.stderr)
            continue
        convert_gif(
            gif_path,
            args.output_dir / entry.name,
            action_tags,
            args.canvas_size,
            args.max_content_size,
            args.bottom_padding,
            args.max_columns,
            args.anchor,
            args.overwrite,
        )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    index = sub.add_parser("index", help="Fetch the Showdown sprite directory into a JSON manifest.")
    index.add_argument("--index-url", default=DEFAULT_INDEX_URL)
    index.add_argument("--sprite-base-url", default=DEFAULT_SPRITE_BASE_URL)
    index.add_argument("--manifest", type=Path, default=Path("data/showdown_ani_manifest.json"))
    index.set_defaults(func=command_index)

    download = sub.add_parser("download", help="Download GIFs from the manifest.")
    download.add_argument("--manifest", type=Path, default=Path("data/showdown_ani_manifest.json"))
    download.add_argument("--output-dir", type=Path, default=Path("assets/source/showdown/ani"))
    download.add_argument("--names", nargs="*")
    download.add_argument("--limit", type=int)
    download.add_argument("--overwrite", action="store_true")
    download.set_defaults(func=command_download)

    convert = sub.add_parser("convert", help="Convert one or more GIFs to TFM2 animation assets.")
    convert.add_argument("gif", type=Path, nargs="+")
    convert.add_argument("--name", help="Output base name. Only valid when converting one GIF.")
    convert.add_argument("--output-dir", type=Path, default=Path("assets/generated/champions"))
    convert.add_argument("--action-tags", default=",".join(DEFAULT_ACTION_TAGS))
    convert.add_argument("--canvas-size", type=int, default=96)
    convert.add_argument("--max-content-size", type=int, default=58)
    convert.add_argument("--bottom-padding", type=int, default=8)
    convert.add_argument("--max-columns", type=int, default=16)
    convert.add_argument("--anchor", choices=["bottom", "center"], default="bottom")
    convert.add_argument("--overwrite", action="store_true")
    convert.set_defaults(func=command_convert)

    batch = sub.add_parser("batch-convert", help="Convert downloaded GIFs by manifest selection.")
    batch.add_argument("--manifest", type=Path, default=Path("data/showdown_ani_manifest.json"))
    batch.add_argument("--input-dir", type=Path, default=Path("assets/source/showdown/ani"))
    batch.add_argument("--output-dir", type=Path, default=Path("assets/generated/champions"))
    batch.add_argument("--names", nargs="*")
    batch.add_argument("--limit", type=int)
    batch.add_argument("--action-tags", default=",".join(DEFAULT_ACTION_TAGS))
    batch.add_argument("--canvas-size", type=int, default=96)
    batch.add_argument("--max-content-size", type=int, default=58)
    batch.add_argument("--bottom-padding", type=int, default=8)
    batch.add_argument("--max-columns", type=int, default=16)
    batch.add_argument("--anchor", choices=["bottom", "center"], default="bottom")
    batch.add_argument("--overwrite", action="store_true")
    batch.set_defaults(func=command_batch_convert)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    if getattr(args, "name", None) and len(getattr(args, "gif", [])) != 1:
        parser.error("--name can only be used with a single GIF")
    args.func(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
