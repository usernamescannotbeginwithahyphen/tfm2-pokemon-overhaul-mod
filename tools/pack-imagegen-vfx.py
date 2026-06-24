"""Pack a flat-chroma ImageGen VFX strip into TFM2-style sheet/fanim assets."""

from __future__ import annotations

import argparse
from collections import deque
import json
from pathlib import Path

from PIL import Image


def parse_color(value: str) -> tuple[int, int, int]:
    parts = [int(part.strip()) for part in value.split(",")]
    if len(parts) != 3:
        raise argparse.ArgumentTypeError("Color must be R,G,B.")
    return (parts[0], parts[1], parts[2])


def chroma_to_alpha(image: Image.Image, key: tuple[int, int, int], tolerance: int) -> Image.Image:
    rgba = image.convert("RGBA")
    pixels = rgba.load()
    for y in range(rgba.height):
        for x in range(rgba.width):
            r, g, b, a = pixels[x, y]
            if abs(r - key[0]) <= tolerance and abs(g - key[1]) <= tolerance and abs(b - key[2]) <= tolerance:
                pixels[x, y] = (r, g, b, 0)
            else:
                pixels[x, y] = (r, g, b, a)
    return rgba


def key_like(pixel: tuple[int, int, int, int], key: tuple[int, int, int], tolerance: int) -> bool:
    r, g, b, _ = pixel
    if abs(r - key[0]) > tolerance or abs(g - key[1]) > tolerance or abs(b - key[2]) > tolerance:
        return False
    if key == (0, 255, 0):
        return g >= r + 18 and g >= b + 18
    if key == (255, 0, 255):
        return r >= g + 18 and b >= g + 18
    return True


def flood_chroma_to_alpha(image: Image.Image, key: tuple[int, int, int], tolerance: int) -> Image.Image:
    rgba = image.convert("RGBA")
    pixels = rgba.load()
    width, height = rgba.size
    seen: set[tuple[int, int]] = set()
    queue: deque[tuple[int, int]] = deque()

    for x in range(width):
        queue.append((x, 0))
        queue.append((x, height - 1))
    for y in range(height):
        queue.append((0, y))
        queue.append((width - 1, y))

    while queue:
        x, y = queue.popleft()
        if (x, y) in seen or x < 0 or y < 0 or x >= width or y >= height:
            continue
        seen.add((x, y))
        if not key_like(pixels[x, y], key, tolerance):
            continue
        r, g, b, _ = pixels[x, y]
        pixels[x, y] = (r, g, b, 0)
        queue.append((x + 1, y))
        queue.append((x - 1, y))
        queue.append((x, y + 1))
        queue.append((x, y - 1))

    return rgba


def trim_alpha(image: Image.Image) -> Image.Image:
    bbox = image.getchannel("A").getbbox()
    if not bbox:
        return Image.new("RGBA", (1, 1), (0, 0, 0, 0))
    return image.crop(bbox)


def find_frame_columns(image: Image.Image, frame_count: int, min_gap: int) -> list[tuple[int, int]]:
    alpha = image.getchannel("A")
    active = []
    for x in range(image.width):
        column = alpha.crop((x, 0, x + 1, image.height))
        if column.getbbox():
            active.append(x)
    if not active:
        raise ValueError("No non-keyed pixels found in VFX source.")

    raw_groups: list[tuple[int, int]] = []
    start = prev = active[0]
    for x in active[1:]:
        if x - prev > min_gap:
            raw_groups.append((start, prev + 1))
            start = x
        prev = x
    raw_groups.append((start, prev + 1))

    if len(raw_groups) == frame_count:
        return raw_groups
    if len(raw_groups) > frame_count:
        raw_groups.sort(key=lambda group: group[1] - group[0], reverse=True)
        return sorted(raw_groups[:frame_count])

    # Fallback for sources whose glow bridges neighboring frames.
    bbox = alpha.getbbox()
    if not bbox:
        raise ValueError("No non-keyed pixels found in VFX source.")
    left, _, right, _ = bbox
    step = (right - left) / frame_count
    return [(round(left + index * step), round(left + (index + 1) * step)) for index in range(frame_count)]


def nearest_fit(image: Image.Image, max_width: int, max_height: int) -> Image.Image:
    if image.width <= max_width and image.height <= max_height:
        return image
    scale = min(max_width / image.width, max_height / image.height)
    return image.resize((max(1, round(image.width * scale)), max(1, round(image.height * scale))), Image.Resampling.NEAREST)


def pack_vfx(args: argparse.Namespace) -> None:
    original = Image.open(args.input)
    if args.key_mode == "flood":
        source = flood_chroma_to_alpha(original, args.key, args.tolerance)
    else:
        source = chroma_to_alpha(original, args.key, args.tolerance)
    output_base = args.output_base
    output_base.parent.mkdir(parents=True, exist_ok=True)

    frames: list[Image.Image] = []
    fanim_frames: list[dict[str, object]] = []
    columns = find_frame_columns(source, args.frame_count, args.min_gap)
    for index, (left, right) in enumerate(columns):
        frame = trim_alpha(source.crop((left, 0, right, source.height)))
        if args.flip_x:
            frame = frame.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        frame = nearest_fit(frame, args.max_content_width, args.max_content_height)
        canvas = Image.new("RGBA", (args.canvas_width, args.canvas_height), (0, 0, 0, 0))
        x = (args.canvas_width - frame.width) // 2
        y = (args.canvas_height - frame.height) // 2
        canvas.alpha_composite(frame, (x, y))
        frames.append(canvas)
        fanim_frames.append(
            {
                "duration": args.duration,
                "data": {
                    "x": index * args.canvas_width,
                    "y": 0,
                    "w": args.canvas_width,
                    "h": args.canvas_height,
                },
            }
        )

    sheet = Image.new("RGBA", (args.canvas_width * len(frames), args.canvas_height), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        sheet.alpha_composite(frame, (index * args.canvas_width, 0))

    sheet_path = output_base.with_name(output_base.name + "#sheet.png")
    anim_path = output_base.with_name(output_base.name + "#anim.fanim")
    cleaned_path = output_base.with_name(output_base.name + "_cleaned.png")
    source.save(cleaned_path)
    sheet.save(sheet_path)
    with anim_path.open("w", encoding="utf-8") as f:
        json.dump({"anims": {args.tag: {"frames": fanim_frames}}}, f, indent=2)
        f.write("\n")
    print(f"OK   wrote {sheet_path}")
    print(f"OK   wrote {anim_path}")
    print(f"OK   wrote {cleaned_path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output-base", type=Path, required=True, help="Output base without #sheet/#anim suffix.")
    parser.add_argument("--tag", default="projectile")
    parser.add_argument("--frame-count", type=int, default=6)
    parser.add_argument("--canvas-width", type=int, default=96)
    parser.add_argument("--canvas-height", type=int, default=96)
    parser.add_argument("--max-content-width", type=int, default=58)
    parser.add_argument("--max-content-height", type=int, default=34)
    parser.add_argument("--duration", type=float, default=0.0667)
    parser.add_argument("--key", type=parse_color, default=(0, 255, 0))
    parser.add_argument("--key-mode", choices=("simple", "flood"), default="simple")
    parser.add_argument("--tolerance", type=int, default=18)
    parser.add_argument("--min-gap", type=int, default=28, help="Minimum transparent horizontal gap between generated frames.")
    parser.add_argument("--flip-x", action="store_true", help="Mirror each extracted VFX frame horizontally.")
    args = parser.parse_args()
    pack_vfx(args)


if __name__ == "__main__":
    main()
