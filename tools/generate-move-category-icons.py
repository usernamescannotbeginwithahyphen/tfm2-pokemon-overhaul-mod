#!/usr/bin/env python
from __future__ import annotations

import argparse
import json
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter


USER_AGENT = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) TFM2-Pokemon-Mod-IconTool/0.1"

CATEGORY_SOURCES = {
    "physical": "Physical icon HOME.png",
    "special": "Special icon HOME.png",
    "status": "Status icon HOME.png",
}

TYPE_COLORS = {
    "normal": "#A8A77A",
    "fire": "#EE8130",
    "water": "#6390F0",
    "electric": "#F7D02C",
    "grass": "#7AC74C",
    "ice": "#96D9D6",
    "fighting": "#C22E28",
    "poison": "#A33EA1",
    "ground": "#E2BF65",
    "flying": "#A98FF3",
    "psychic": "#F95587",
    "bug": "#A6B91A",
    "rock": "#B6A136",
    "ghost": "#735797",
    "dragon": "#6F35FC",
    "dark": "#705746",
    "steel": "#B7B7CE",
    "fairy": "#D685AD",
}


@dataclass(frozen=True)
class CategorySource:
    category: str
    file_name: str

    @property
    def local_name(self) -> str:
        return self.file_name.replace(" ", "_")

    @property
    def url(self) -> str:
        return (
            "https://archives.bulbagarden.net/wiki/Special:Redirect/file/"
            + urllib.parse.quote(self.file_name)
        )


def download_file(url: str, output: Path) -> None:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    output.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(request, timeout=120) as response:
        output.write_bytes(response.read())


def ensure_sources(source_dir: Path, overwrite: bool = False) -> dict[str, Path]:
    sources: dict[str, Path] = {}
    for category, file_name in CATEGORY_SOURCES.items():
        source = CategorySource(category, file_name)
        output = source_dir / source.local_name
        if overwrite or not output.exists():
            print(f"GET  {source.url}")
            download_file(source.url, output)
        sources[category] = output
    return sources


def hex_to_rgb(value: str) -> tuple[int, int, int]:
    value = value.lstrip("#")
    return tuple(int(value[i : i + 2], 16) for i in (0, 2, 4))


def recolor_symbol(image: Image.Image, color: tuple[int, int, int]) -> Image.Image:
    rgba = image.convert("RGBA")
    alpha = rgba.getchannel("A")
    gray = rgba.convert("L")
    result = Image.new("RGBA", rgba.size, (0, 0, 0, 0))
    pixels = result.load()
    alpha_pixels = alpha.load()
    gray_pixels = gray.load()
    for y in range(rgba.height):
        for x in range(rgba.width):
            a = alpha_pixels[x, y]
            if a == 0:
                continue
            shade = 0.78 + (gray_pixels[x, y] / 255.0) * 0.28
            pixels[x, y] = (
                min(255, int(color[0] * shade)),
                min(255, int(color[1] * shade)),
                min(255, int(color[2] * shade)),
                a,
            )
    return result


def fit_center(image: Image.Image, size: int, padding: int) -> Image.Image:
    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    max_w = size - padding * 2
    max_h = size - padding * 2
    scale = min(max_w / image.width, max_h / image.height)
    target = image.resize(
        (max(1, round(image.width * scale)), max(1, round(image.height * scale))),
        Image.Resampling.LANCZOS,
    )
    canvas.alpha_composite(target, ((size - target.width) // 2, (size - target.height) // 2))
    return canvas


def make_icon(symbol: Image.Image, color: tuple[int, int, int], size: int, padding: int) -> Image.Image:
    icon = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(icon)
    draw.rounded_rectangle((2, 2, size - 3, size - 3), radius=10, fill=(18, 20, 24, 235))
    draw.rounded_rectangle((3, 3, size - 4, size - 4), radius=9, outline=(*color, 220), width=3)
    draw.rounded_rectangle((7, 7, size - 8, size - 8), radius=7, fill=(*color, 50))

    recolored = fit_center(recolor_symbol(symbol, color), size, padding)
    shadow = recolored.getchannel("A").filter(ImageFilter.GaussianBlur(2))
    shadow_image = Image.new("RGBA", (size, size), (0, 0, 0, 150))
    shadow_image.putalpha(shadow)
    icon.alpha_composite(shadow_image)
    icon.alpha_composite(recolored)
    return icon


def write_sprite_sheet(output_dir: Path, icon_size: int) -> None:
    tags: dict[str, dict[str, float]] = {}
    images = []
    for path in sorted(output_dir.glob("*.png")):
        if "#" in path.name:
            continue
        images.append((path.stem, Image.open(path).convert("RGBA")))

    if not images:
        return

    sheet = Image.new("RGBA", (icon_size * len(images), icon_size), (0, 0, 0, 0))
    for index, (tag, image) in enumerate(images):
        x = index * icon_size
        sheet.alpha_composite(image, (x, 0))
        tags[tag] = {
            "x": x / sheet.width,
            "y": 0.0,
            "w": icon_size / sheet.width,
            "h": 1.0,
        }

    sheet.save(output_dir / "move_categories#sheet.png")
    with (output_dir / "move_categories#data.sprite_sheet").open("w", encoding="utf-8") as f:
        json.dump({"images": tags}, f, indent=2)
        f.write("\n")


def generate(source_dir: Path, output_dir: Path, size: int, padding: int, overwrite_sources: bool) -> None:
    sources = ensure_sources(source_dir, overwrite_sources)
    output_dir.mkdir(parents=True, exist_ok=True)
    for category, source_path in sources.items():
        symbol = Image.open(source_path).convert("RGBA")
        for type_name, color_hex in TYPE_COLORS.items():
            output = output_dir / f"{type_name}_{category}.png"
            make_icon(symbol, hex_to_rgb(color_hex), size, padding).save(output)
            print(f"OK   {output}")
    write_sprite_sheet(output_dir, size)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-dir", type=Path, default=Path("assets/source/pokemon/move_categories"))
    parser.add_argument("--output-dir", type=Path, default=Path("mod/pokemon_moba/icons/move_categories"))
    parser.add_argument("--size", type=int, default=64)
    parser.add_argument("--padding", type=int, default=10)
    parser.add_argument("--overwrite-sources", action="store_true")
    args = parser.parse_args()
    generate(args.source_dir, args.output_dir, args.size, args.padding, args.overwrite_sources)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
