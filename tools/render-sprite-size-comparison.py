"""Render a first-idle-frame size comparison sheet for custom Pokemon sprites."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image, ImageDraw


DEFAULT_REFERENCES = ("shedinja", "electrode", "gallade", "venusaur", "snorlax")
DEFAULT_CHAMPION_DIR = Path("assets/custom_spritework/champions")
DEFAULT_OUTPUT_DIR = Path("assets/custom_spritework/previews")


def parse_color(value: str) -> tuple[int, int, int, int]:
    parts = [int(part.strip()) for part in value.split(",")]
    if len(parts) == 3:
        return (parts[0], parts[1], parts[2], 255)
    if len(parts) == 4:
        return (parts[0], parts[1], parts[2], parts[3])
    raise argparse.ArgumentTypeError("Color must be R,G,B or R,G,B,A.")


def resolve_base(value: str, champion_dir: Path) -> Path:
    path = Path(value)
    if path.exists() or path.with_name(path.name + "#sheet.png").exists():
        return path
    return champion_dir / value


def first_idle_frame(base: Path) -> tuple[Image.Image, tuple[int, int, int, int] | None]:
    sheet_path = base.with_name(base.name + "#sheet.png")
    anim_path = base.with_name(base.name + "#anim.fanim")
    if not sheet_path.exists():
        raise FileNotFoundError(sheet_path)
    if not anim_path.exists():
        raise FileNotFoundError(anim_path)

    with anim_path.open("r", encoding="utf-8") as f:
        anims = json.load(f)["anims"]
    idle = anims.get("idle")
    if not idle or not idle.get("frames"):
        raise ValueError(f"{anim_path} has no idle frames.")

    rect = idle["frames"][0]["data"]
    x = int(float(rect["x"]))
    y = int(float(rect["y"]))
    w = int(float(rect["w"]))
    h = int(float(rect["h"]))
    with Image.open(sheet_path) as image:
        frame = image.convert("RGBA").crop((x, y, x + w, y + h))
    return frame, frame.getchannel("A").getbbox()


def render_comparison(
    target: Path,
    references: list[Path],
    output_path: Path,
    scale: int,
    background: tuple[int, int, int, int],
    ground_y: int,
) -> None:
    bases = [target, *references]
    labels = [base.name for base in bases]
    frames: list[Image.Image] = []
    metrics: list[str] = []

    for base in bases:
        frame, bbox = first_idle_frame(base)
        frames.append(frame)
        if bbox:
            visible_h = bbox[3] - bbox[1]
            metrics.append(f"h {visible_h}px, bottom {bbox[3]}")
        else:
            metrics.append("empty")

    frame_w = max(frame.width for frame in frames)
    frame_h = max(frame.height for frame in frames)
    panel_w = frame_w * scale
    panel_h = frame_h * scale + 34
    output = Image.new("RGBA", (panel_w * len(frames), panel_h), background)
    draw = ImageDraw.Draw(output)

    for index, (label, metric, frame) in enumerate(zip(labels, metrics, frames, strict=True)):
        scaled = frame.resize((frame.width * scale, frame.height * scale), Image.Resampling.NEAREST)
        x = index * panel_w + (panel_w - scaled.width) // 2
        output.alpha_composite(scaled, (x, 0))
        gx0 = index * panel_w
        gy = ground_y * scale
        draw.line((gx0, gy, gx0 + panel_w, gy), fill=(255, 255, 255, 90), width=1)
        draw.text((gx0 + 8, frame_h * scale + 4), label, fill=(235, 235, 235, 255))
        draw.text((gx0 + 8, frame_h * scale + 18), metric, fill=(190, 200, 205, 255))

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output.save(output_path)
    print(f"OK   size comparison -> {output_path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("target", help="Target base path or champion short id, without #sheet/#anim suffix.")
    parser.add_argument(
        "--compare",
        nargs="+",
        default=list(DEFAULT_REFERENCES),
        help="Reference champion short ids or base paths.",
    )
    parser.add_argument("--champion-dir", type=Path, default=DEFAULT_CHAMPION_DIR)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT_DIR)
    parser.add_argument("--output", type=Path, help="Output PNG path. Defaults to <target>_size_compare.png.")
    parser.add_argument("--scale", type=int, default=4)
    parser.add_argument("--ground-y", type=int, default=78, help="Reference ground line inside the 96x96 frame.")
    parser.add_argument("--background", type=parse_color, default=(38, 42, 48, 255))
    args = parser.parse_args()

    if args.scale < 1:
        raise SystemExit("--scale must be >= 1")

    target = resolve_base(args.target, args.champion_dir)
    references = [resolve_base(value, args.champion_dir) for value in args.compare]
    output_path = args.output or args.output_dir / f"{target.name}_size_compare.png"
    render_comparison(target, references, output_path, args.scale, args.background, args.ground_y)


if __name__ == "__main__":
    main()
