"""Render small QC GIFs from a staged TFM2 #sheet.png + #anim.fanim pair."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image


DEFAULT_TAGS = ("idle", "run", "attack", "skill", "skill2", "ult", "dead")
PREVIEW_NAMES = {
    "attack": "basicattack",
    "skill": "skill1",
}


def parse_color(value: str) -> tuple[int, int, int, int]:
    parts = [int(part.strip()) for part in value.split(",")]
    if len(parts) == 3:
        return (parts[0], parts[1], parts[2], 255)
    if len(parts) == 4:
        return (parts[0], parts[1], parts[2], parts[3])
    raise argparse.ArgumentTypeError("Color must be R,G,B or R,G,B,A.")


def load_anim(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        data = json.load(f)
    if not isinstance(data, dict) or not isinstance(data.get("anims"), dict):
        raise ValueError(f"{path} does not look like a TFM2 .fanim file.")
    return data["anims"]


def to_gif_frame(image: Image.Image, transparent: bool) -> Image.Image:
    if not transparent:
        return image.convert("P", palette=Image.Palette.ADAPTIVE)

    rgba = image.convert("RGBA")
    alpha = rgba.getchannel("A")
    # Reserve palette index 255 for fully transparent pixels. GIF cannot
    # represent partial alpha, but this keeps empty frame pixels from being
    # matted into QC previews.
    paletted = rgba.convert("P", palette=Image.Palette.ADAPTIVE, colors=255)
    palette = paletted.getpalette() or []
    if len(palette) < 768:
        palette.extend([0] * (768 - len(palette)))
    palette[255 * 3 : 255 * 3 + 3] = [0, 255, 0]
    paletted.putpalette(palette)

    pixels = list(paletted.getdata())
    alpha_pixels = list(alpha.getdata())
    paletted.putdata([255 if a == 0 else p for p, a in zip(pixels, alpha_pixels, strict=True)])
    paletted.info["transparency"] = 255
    return paletted


def rect_to_box(rect: dict, sheet_size: tuple[int, int]) -> tuple[int, int, int, int]:
    x = int(float(rect["x"]))
    y = int(float(rect["y"]))
    w = int(float(rect["w"]))
    h = int(float(rect["h"]))
    if w <= 0 or h <= 0:
        raise ValueError(f"Invalid empty frame rectangle: {rect}")
    if x < 0 or y < 0 or x + w > sheet_size[0] or y + h > sheet_size[1]:
        raise ValueError(f"Frame rectangle outside sheet bounds: {rect}")
    return (x, y, x + w, y + h)


def render_tag(
    sheet: Image.Image,
    tag: str,
    frames: list[dict],
    output_path: Path,
    scale: int,
    background: tuple[int, int, int, int],
) -> None:
    rects = [frame["data"] for frame in frames]
    boxes = [rect_to_box(rect, sheet.size) for rect in rects]
    canvas_w = max(box[2] - box[0] for box in boxes)
    canvas_h = max(box[3] - box[1] for box in boxes)

    rendered: list[Image.Image] = []
    durations_ms: list[int] = []
    transparent = background[3] == 0
    for frame, box in zip(frames, boxes, strict=True):
        cropped = sheet.crop(box)
        canvas = Image.new("RGBA", (canvas_w, canvas_h), background)
        x = (canvas_w - cropped.width) // 2
        y = (canvas_h - cropped.height) // 2
        canvas.alpha_composite(cropped, (x, y))
        if scale != 1:
            canvas = canvas.resize((canvas.width * scale, canvas.height * scale), Image.Resampling.NEAREST)
        rendered.append(to_gif_frame(canvas, transparent))
        durations_ms.append(max(20, round(float(frame.get("duration", 0.1)) * 1000)))

    output_path.parent.mkdir(parents=True, exist_ok=True)
    rendered[0].save(
        output_path,
        save_all=True,
        append_images=rendered[1:],
        duration=durations_ms,
        loop=0,
        optimize=False,
        disposal=2,
        transparency=255 if transparent else None,
    )
    print(f"OK   {tag} -> {output_path}")


def render_previews(base: Path, output_dir: Path, tags: tuple[str, ...], scale: int, background: tuple[int, int, int, int]) -> None:
    sheet_path = base.with_name(base.name + "#sheet.png")
    anim_path = base.with_name(base.name + "#anim.fanim")
    if not sheet_path.exists():
        raise FileNotFoundError(sheet_path)
    if not anim_path.exists():
        raise FileNotFoundError(anim_path)

    anims = load_anim(anim_path)
    with Image.open(sheet_path) as image:
        sheet = image.convert("RGBA")

    for tag in tags:
        anim = anims.get(tag)
        if not anim:
            print(f"SKIP {tag}: missing tag")
            continue
        frames = anim.get("frames")
        if not frames:
            print(f"SKIP {tag}: no frames")
            continue
        preview_name = PREVIEW_NAMES.get(tag, tag)
        output_path = output_dir / f"{base.name}_{preview_name}_preview.gif"
        render_tag(sheet, tag, frames, output_path, scale, background)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", type=Path, help="Base path without #sheet.png or #anim.fanim suffix.")
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("assets/custom_spritework/previews"),
        help="Directory for generated preview GIFs.",
    )
    parser.add_argument("--tags", nargs="+", default=list(DEFAULT_TAGS), help="Animation tags to render.")
    parser.add_argument("--scale", type=int, default=3, help="Nearest-neighbor preview scale.")
    parser.add_argument(
        "--background",
        type=parse_color,
        default=(112, 100, 88, 255),
        help="Preview matte as R,G,B or R,G,B,A.",
    )
    args = parser.parse_args()

    if args.scale < 1:
        raise SystemExit("--scale must be >= 1")

    render_previews(args.base, args.output_dir, tuple(args.tags), args.scale, args.background)


if __name__ == "__main__":
    main()
