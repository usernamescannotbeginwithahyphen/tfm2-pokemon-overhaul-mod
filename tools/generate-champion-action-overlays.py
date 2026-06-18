#!/usr/bin/env python3
"""Add action-specific visual overlays and UI-safe static frames to Pokemon sheets."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageOps


BASE_TAGS = ("idle", "run", "dead")
ACTION_TAGS = ("attack", "skill", "skill2", "ult")
UI_ICON_SIZE = 96
UI_ICON_VISIBLE_SIZE = 76
RUN_FRAME_LIMIT = 8
ACTION_FRAME_LIMIT = 8

CONFIG = {
    "pikachu": {
        "attack": ("electric_sparks", (255, 225, 55, 230)),
        "skill": ("electric_dash", (255, 195, 30, 230)),
        "skill2": ("electric_wave", (100, 210, 255, 210)),
        "ult": ("thunder_burst", (255, 240, 90, 245)),
    },
    "charizard": {
        "attack": ("flame_breath", (255, 106, 32, 230)),
        "skill": ("dragon_breath", (150, 105, 255, 220)),
        "skill2": ("fire_orb", (255, 64, 20, 240)),
        "ult": ("inferno", (255, 55, 20, 245)),
    },
    "blastoise": {
        "attack": ("water_cannon", (55, 160, 255, 230)),
        "skill": ("rapid_spin", (150, 230, 255, 210)),
        "skill2": ("water_tail", (45, 190, 255, 230)),
        "ult": ("hydro_beam", (85, 210, 255, 245)),
    },
    "venusaur": {
        "attack": ("impact", (210, 245, 150, 210)),
        "skill": ("vines", (75, 210, 80, 230)),
        "skill2": ("seed_roots", (95, 220, 105, 230)),
        "ult": ("poison_wave", (180, 90, 235, 230)),
    },
    "mewtwo": {
        "attack": ("psychic_pulse", (255, 90, 180, 220)),
        "skill": ("psychic_pulse", (190, 110, 255, 225)),
        "skill2": ("psychic_ring", (255, 120, 210, 225)),
        "ult": ("psychic_burst", (255, 80, 190, 245)),
    },
    "eevee": {
        "attack": ("impact", (220, 210, 185, 210)),
        "skill": ("psychic_ring", (255, 155, 210, 225)),
        "skill2": ("psychic_pulse", (255, 235, 170, 230)),
        "ult": ("psychic_burst", (240, 220, 170, 240)),
    },
    "jolteon": {
        "attack": ("electric_dash", (195, 245, 255, 230)),
        "skill": ("impact", (210, 185, 145, 220)),
        "skill2": ("electric_sparks", (255, 235, 65, 235)),
        "ult": ("thunder_burst", (255, 245, 80, 245)),
    },
    "flareon": {
        "attack": ("fire_orb", (255, 120, 45, 230)),
        "skill": ("inferno", (255, 80, 30, 235)),
        "skill2": ("flame_breath", (255, 100, 35, 235)),
        "ult": ("fire_orb", (255, 50, 25, 245)),
    },
    "vaporeon": {
        "attack": ("hydro_beam", (75, 205, 255, 230)),
        "skill": ("psychic_ring", (170, 210, 235, 215)),
        "skill2": ("water_cannon", (55, 175, 255, 235)),
        "ult": ("psychic_ring", (80, 220, 255, 240)),
    },
    "leafeon": {
        "attack": ("vines", (100, 225, 80, 230)),
        "skill": ("seed_roots", (130, 245, 95, 235)),
        "skill2": ("vines", (80, 215, 65, 240)),
        "ult": ("poison_wave", (120, 240, 80, 245)),
    },
    "glaceon": {
        "attack": ("water_cannon", (185, 235, 255, 230)),
        "skill": ("psychic_ring", (170, 230, 255, 230)),
        "skill2": ("electric_dash", (190, 240, 255, 230)),
        "ult": ("thunder_burst", (205, 245, 255, 245)),
    },
    "umbreon": {
        "attack": ("impact", (80, 80, 110, 230)),
        "skill": ("psychic_ring", (65, 55, 95, 230)),
        "skill2": ("psychic_pulse", (90, 75, 135, 235)),
        "ult": ("electric_dash", (45, 45, 75, 245)),
    },
    "espeon": {
        "attack": ("impact", (220, 190, 170, 215)),
        "skill": ("psychic_pulse", (255, 100, 210, 235)),
        "skill2": ("psychic_ring", (255, 90, 190, 235)),
        "ult": ("psychic_burst", (255, 80, 210, 245)),
    },
    "sylveon": {
        "attack": ("psychic_ring", (255, 150, 225, 225)),
        "skill": ("psychic_ring", (255, 170, 235, 235)),
        "skill2": ("psychic_pulse", (255, 210, 235, 235)),
        "ult": ("thunder_burst", (255, 180, 245, 245)),
    },
    "feraligatr": {
        "attack": ("impact", (210, 225, 240, 220)),
        "skill": ("water_tail", (70, 180, 255, 235)),
        "skill2": ("rapid_spin", (210, 225, 240, 230)),
        "ult": ("hydro_beam", (95, 165, 210, 245)),
    },
    "emboar": {
        "attack": ("impact", (255, 185, 90, 225)),
        "skill": ("impact", (255, 120, 45, 235)),
        "skill2": ("flame_breath", (255, 85, 30, 245)),
        "ult": ("inferno", (255, 120, 35, 245)),
    },
    "blaziken": {
        "attack": ("electric_dash", (255, 190, 70, 230)),
        "skill": ("fire_orb", (255, 90, 25, 240)),
        "skill2": ("psychic_ring", (255, 150, 70, 230)),
        "ult": ("flame_breath", (255, 55, 20, 245)),
    },
    "greninja": {
        "attack": ("electric_dash", (80, 80, 145, 230)),
        "skill": ("electric_dash", (150, 210, 255, 235)),
        "skill2": ("water_cannon", (70, 185, 255, 235)),
        "ult": ("hydro_beam", (70, 210, 255, 245)),
    },
    "decidueye": {
        "attack": ("vines", (100, 230, 105, 230)),
        "skill": ("psychic_pulse", (110, 90, 150, 235)),
        "skill2": ("poison_wave", (120, 225, 90, 240)),
        "ult": ("psychic_burst", (120, 80, 165, 245)),
    },
    "inteleon": {
        "attack": ("water_cannon", (80, 190, 255, 230)),
        "skill": ("psychic_ring", (95, 210, 255, 230)),
        "skill2": ("hydro_beam", (80, 220, 255, 235)),
        "ult": ("hydro_beam", (120, 230, 255, 250)),
    },
    "skeledirge": {
        "attack": ("psychic_pulse", (105, 75, 145, 230)),
        "skill": ("psychic_ring", (125, 70, 160, 235)),
        "skill2": ("fire_orb", (255, 90, 40, 240)),
        "ult": ("inferno", (255, 70, 25, 245)),
    },
    "porygonz": {
        "attack": ("psychic_pulse", (235, 110, 255, 230)),
        "skill": ("psychic_burst", (160, 120, 255, 240)),
        "skill2": ("psychic_ring", (150, 230, 255, 235)),
        "ult": ("thunder_burst", (255, 130, 235, 250)),
    },
    "blissey": {
        "attack": ("impact", (255, 210, 220, 210)),
        "skill": ("psychic_ring", (255, 190, 220, 235)),
        "skill2": ("psychic_pulse", (255, 210, 225, 235)),
        "ult": ("psychic_burst", (255, 220, 235, 250)),
    },
    "kleavor": {
        "attack": ("impact", (190, 185, 155, 230)),
        "skill": ("electric_dash", (185, 215, 110, 235)),
        "skill2": ("psychic_ring", (175, 165, 135, 235)),
        "ult": ("thunder_burst", (190, 180, 145, 245)),
    },
    "scizor": {
        "attack": ("electric_dash", (210, 215, 225, 235)),
        "skill": ("psychic_ring", (235, 235, 245, 235)),
        "skill2": ("electric_dash", (245, 245, 255, 245)),
        "ult": ("thunder_burst", (200, 230, 255, 245)),
    },
    "ursaluna": {
        "attack": ("impact", (210, 190, 155, 225)),
        "skill": ("psychic_ring", (120, 95, 80, 235)),
        "skill2": ("impact", (170, 115, 70, 245)),
        "ult": ("inferno", (205, 45, 35, 250)),
    },
    "ursaluna_bloodmoon": {
        "attack": ("impact", (210, 70, 65, 230)),
        "skill": ("psychic_ring", (160, 65, 60, 235)),
        "skill2": ("impact", (205, 70, 55, 245)),
        "ult": ("inferno", (220, 35, 35, 250)),
    },
    "sawk_throh": {
        "attack": ("impact", (220, 82, 58, 230)),
        "skill": ("psychic_ring", (230, 210, 185, 235)),
        "skill2": ("electric_dash", (225, 82, 58, 240)),
        "ult": ("thunder_burst", (235, 80, 55, 250)),
    },
    "hitmonchan": {
        "attack": ("impact", (225, 90, 65, 230)),
        "skill": ("water_cannon", (175, 230, 255, 235)),
        "skill2": ("fire_orb", (255, 95, 35, 240)),
        "ult": ("electric_sparks", (255, 230, 55, 245)),
    },
    "hitmonlee": {
        "attack": ("electric_dash", (230, 95, 70, 230)),
        "skill": ("impact", (235, 120, 75, 235)),
        "skill2": ("thunder_burst", (240, 105, 70, 245)),
        "ult": ("impact", (255, 75, 55, 250)),
    },
    "hitmontop": {
        "attack": ("electric_dash", (225, 95, 70, 230)),
        "skill": ("impact", (235, 120, 75, 235)),
        "skill2": ("psychic_ring", (220, 205, 175, 235)),
        "ult": ("rapid_spin", (245, 90, 55, 250)),
    },
    "kilowattrel": {
        "attack": ("electric_sparks", (255, 225, 55, 230)),
        "skill": ("electric_dash", (255, 220, 70, 235)),
        "skill2": ("hydro_beam", (190, 230, 255, 235)),
        "ult": ("thunder_burst", (255, 240, 75, 250)),
    },
    "gyarados": {
        "attack": ("hydro_beam", (75, 180, 255, 230)),
        "skill": ("fire_orb", (255, 120, 55, 240)),
        "skill2": ("hydro_beam", (165, 225, 255, 240)),
        "ult": ("thunder_burst", (255, 245, 210, 250)),
    },
    "noivern": {
        "attack": ("psychic_pulse", (185, 175, 255, 230)),
        "skill": ("hydro_beam", (190, 230, 255, 235)),
        "skill2": ("flame_breath", (150, 215, 255, 235)),
        "ult": ("psychic_burst", (130, 95, 255, 250)),
    },
    "mantine": {
        "attack": ("water_cannon", (90, 205, 255, 230)),
        "skill": ("psychic_ring", (80, 190, 255, 235)),
        "skill2": ("electric_dash", (70, 180, 255, 240)),
        "ult": ("hydro_beam", (85, 215, 255, 250)),
    },
    "cryogonal": {
        "attack": ("water_cannon", (190, 235, 255, 230)),
        "skill": ("hydro_beam", (190, 235, 255, 240)),
        "skill2": ("psychic_pulse", (205, 245, 255, 240)),
        "ult": ("hydro_beam", (220, 250, 255, 250)),
    },
    "vanilluxe": {
        "attack": ("water_cannon", (210, 245, 255, 230)),
        "skill": ("hydro_beam", (165, 225, 255, 240)),
        "skill2": ("flame_breath", (215, 250, 255, 240)),
        "ult": ("thunder_burst", (225, 250, 255, 250)),
    },
    "skarmory": {
        "attack": ("impact", (205, 220, 235, 230)),
        "skill": ("psychic_ring", (185, 205, 225, 235)),
        "skill2": ("electric_dash", (210, 225, 240, 240)),
        "ult": ("hydro_beam", (200, 230, 255, 250)),
    },
    "houndoom": {
        "attack": ("fire_orb", (255, 95, 40, 230)),
        "skill": ("psychic_pulse", (95, 70, 125, 235)),
        "skill2": ("psychic_ring", (255, 150, 70, 235)),
        "ult": ("inferno", (255, 65, 25, 250)),
    },
    "arbok": {
        "attack": ("impact", (180, 85, 210, 230)),
        "skill": ("electric_dash", (150, 65, 190, 240)),
        "skill2": ("psychic_ring", (130, 85, 150, 235)),
        "ult": ("thunder_burst", (190, 70, 220, 250)),
    },
    "clawitzer": {
        "attack": ("water_cannon", (80, 175, 255, 235)),
        "skill": ("hydro_beam", (105, 205, 255, 235)),
        "skill2": ("water_cannon", (150, 225, 255, 245)),
        "ult": ("hydro_beam", (170, 235, 255, 250)),
    },
    "octillery": {
        "attack": ("impact", (155, 195, 95, 230)),
        "skill": ("psychic_ring", (220, 225, 210, 235)),
        "skill2": ("water_cannon", (130, 215, 255, 240)),
        "ult": ("hydro_beam", (160, 230, 255, 250)),
    },
    "pyukumuku": {
        "attack": ("psychic_ring", (190, 210, 235, 235)),
        "skill": ("poison_wave", (170, 85, 205, 240)),
        "skill2": ("psychic_pulse", (150, 215, 255, 235)),
        "ult": ("psychic_ring", (230, 225, 210, 250)),
    },
    "banette": {
        "attack": ("impact", (110, 85, 165, 235)),
        "skill": ("electric_dash", (90, 70, 145, 240)),
        "skill2": ("psychic_pulse", (120, 85, 175, 240)),
        "ult": ("psychic_burst", (145, 90, 205, 250)),
    },
    "kricketune": {
        "attack": ("impact", (165, 205, 85, 235)),
        "skill": ("psychic_ring", (165, 205, 85, 235)),
        "skill2": ("psychic_pulse", (225, 225, 190, 235)),
        "ult": ("psychic_burst", (175, 210, 80, 245)),
    },
    "ambipom": {
        "attack": ("impact", (220, 205, 170, 235)),
        "skill": ("psychic_pulse", (235, 205, 175, 235)),
        "skill2": ("impact", (110, 90, 120, 235)),
        "ult": ("psychic_burst", (235, 220, 185, 245)),
    },
    "torterra": {
        "attack": ("vines", (105, 210, 80, 225)),
        "skill": ("psychic_ring", (130, 160, 105, 235)),
        "skill2": ("psychic_pulse", (135, 230, 100, 235)),
        "ult": ("impact", (190, 150, 90, 245)),
    },
}


def load_anim(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def base_frames(anim: dict) -> list[dict]:
    for tag in ("idle", "run", "attack", "skill"):
        frames = anim.get("anims", {}).get(tag, {}).get("frames")
        if frames:
            return frames
    raise ValueError("No base animation frames found.")


def crop_frame(sheet: Image.Image, frame: dict) -> Image.Image:
    data = frame["data"]
    return sheet.crop((data["x"], data["y"], data["x"] + data["w"], data["y"] + data["h"]))


def centered_visible_frame(frame: Image.Image, canvas_size: int = UI_ICON_SIZE, visible_size: int = UI_ICON_VISIBLE_SIZE) -> Image.Image:
    """Create a stable square portrait from the visible alpha content of one frame."""
    source = frame.convert("RGBA")
    bbox = source.getchannel("A").getbbox()
    out = Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0))
    if not bbox:
        return out

    cropped = source.crop(bbox)
    width, height = cropped.size
    scale = min(visible_size / max(1, width), visible_size / max(1, height), 1.0)
    resized = cropped.resize((max(1, round(width * scale)), max(1, round(height * scale))), Image.Resampling.LANCZOS)
    x = (canvas_size - resized.width) // 2
    y = (canvas_size - resized.height) // 2
    out.alpha_composite(resized, (x, y))
    return out


def icon_path_for(base: Path) -> Path:
    if base.parent.name == "champions":
        return base.parent.parent / "icons" / "champions" / f"{base.name}.png"
    return base.parent / "icons" / "champions" / f"{base.name}.png"


def glow_mask(frame: Image.Image, size: int = 7) -> Image.Image:
    alpha = frame.getchannel("A")
    return alpha.filter(ImageFilter.MaxFilter(size)).filter(ImageFilter.GaussianBlur(size / 2))


def overlay_frame(frame: Image.Image, pokemon: str, tag: str, index: int, total: int) -> Image.Image:
    if tag in BASE_TAGS:
        if tag == "dead":
            dead = ImageOps.grayscale(frame).convert("RGBA")
            dead.putalpha(frame.getchannel("A"))
            return dead
        return frame.copy()

    style, color = CONFIG.get(pokemon, {}).get(tag, ("impact", (255, 255, 255, 235)))
    result = frame.copy()
    effect = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    draw = ImageDraw.Draw(effect, "RGBA")
    w, h = frame.size
    t = index / max(1, total - 1)
    pulse = 0.55 + 0.45 * math.sin(t * math.pi)
    cx = w // 2
    cy = int(h * 0.56)
    if tag in ACTION_TAGS:
        draw_action_readout(draw, w, h, color, tag, pulse)

    if style in {"electric_sparks", "electric_dash", "thunder_burst"}:
        draw_glow(effect, frame, color, 9 if tag != "ult" else 13)
        bolts = 3 if tag != "ult" else 7
        for i in range(bolts):
            angle = (i / max(1, bolts)) * math.tau + t * math.pi
            radius = int((18 if tag != "ult" else 30) * pulse)
            x1 = cx + int(math.cos(angle) * 8)
            y1 = cy + int(math.sin(angle) * 8)
            x2 = cx + int(math.cos(angle) * radius)
            y2 = cy + int(math.sin(angle) * radius)
            draw.line((x1, y1, (x1 + x2) // 2, y2, x2, y2 - 5), fill=color, width=2)
        if style == "electric_dash":
            for yoff in (-10, 0, 10):
                draw.line((int(w * 0.15), cy + yoff, int(w * 0.85), cy + yoff - 6), fill=color, width=2)

    elif style in {"flame_breath", "dragon_breath", "fire_orb", "inferno"}:
        draw_glow(effect, frame, color, 11)
        if style in {"flame_breath", "dragon_breath"}:
            for i in range(4):
                x0 = int(w * (0.55 + 0.05 * i))
                y0 = int(h * (0.42 + 0.08 * i))
                x1 = min(w - 1, x0 + int((20 + 10 * i) * pulse))
                y1 = y0 + int((8 + 5 * i) * pulse)
                draw.ellipse((x0, y0 - 8, x1, y1 + 8), fill=fade(color, 95 + 25 * i))
        elif style == "fire_orb":
            r = int(10 + 10 * pulse)
            draw.ellipse((cx - r, cy - r, cx + r, cy + r), outline=color, width=3)
            draw.ellipse((cx - r // 2, cy - r // 2, cx + r // 2, cy + r // 2), fill=fade(color, 120))
        else:
            for r in (18, 28, 38):
                draw.ellipse((cx - r, cy - r, cx + r, cy + r), outline=fade(color, int(180 * pulse)), width=3)

    elif style in {"water_cannon", "rapid_spin", "water_tail", "hydro_beam"}:
        draw_glow(effect, frame, color, 9)
        if style in {"water_cannon", "hydro_beam"}:
            width = 3 if style == "water_cannon" else 6
            for yoff in (-5, 5) if style == "water_cannon" else (-8, 0, 8):
                draw.line((int(w * 0.45), cy + yoff, int(w * 0.92), cy + yoff), fill=color, width=width)
        elif style == "rapid_spin":
            for r in (18, 27, 36):
                draw.arc((cx - r, cy - r, cx + r, cy + r), int(360 * t), int(360 * t) + 245, fill=color, width=3)
        else:
            draw.arc((cx - 28, cy - 22, cx + 28, cy + 22), 195, 340, fill=color, width=5)

    elif style in {"vines", "seed_roots", "poison_wave"}:
        draw_glow(effect, frame, color, 9)
        if style in {"vines", "seed_roots"}:
            for i, yoff in enumerate((-12, 0, 12)):
                y = cy + yoff
                draw.line((int(w * 0.3), y, int(w * 0.8), y + int(math.sin(t * math.tau + i) * 8)), fill=color, width=3)
                draw.ellipse((int(w * 0.78), y - 4, int(w * 0.88), y + 6), fill=fade(color, 160))
        else:
            for r in (18, 28, 38):
                draw.arc((cx - r, cy - r, cx + r, cy + r), 20, 330, fill=color, width=3)

    else:
        draw_glow(effect, frame, color, 9)
        r = int(15 + 15 * pulse)
        draw.ellipse((cx - r, cy - r, cx + r, cy + r), outline=color, width=3)

    result.alpha_composite(effect)
    return result


def draw_glow(effect: Image.Image, frame: Image.Image, color: tuple[int, int, int, int], size: int) -> None:
    glow = Image.new("RGBA", frame.size, color)
    glow.putalpha(glow_mask(frame, size).point(lambda value: min(value, (color[3] * 2) // 3)))
    effect.alpha_composite(glow)


def draw_action_readout(
    draw: ImageDraw.ImageDraw,
    width: int,
    height: int,
    color: tuple[int, int, int, int],
    tag: str,
    pulse: float,
) -> None:
    alpha = int(min(170, color[3]) * pulse)
    readout = fade(color, alpha)
    y = max(2, int(height * 0.1))
    if tag == "attack":
        draw.line((int(width * 0.2), y, int(width * 0.8), y), fill=readout, width=2)
    elif tag == "skill":
        draw.arc((4, y, width - 4, height - 6), 200, 340, fill=readout, width=3)
    elif tag == "skill2":
        draw.line((int(width * 0.18), y + 3, int(width * 0.82), height - 8), fill=readout, width=3)
        draw.line((int(width * 0.18), height - 8, int(width * 0.82), y + 3), fill=readout, width=3)
    elif tag == "ult":
        for inset in (3, 9):
            draw.rectangle((inset, inset, width - inset - 1, height - inset - 1), outline=readout, width=3)


def fade(color: tuple[int, int, int, int], alpha: int) -> tuple[int, int, int, int]:
    return color[0], color[1], color[2], max(0, min(255, alpha))


def pack(frames: list[tuple[str, float, Image.Image]], max_columns: int) -> tuple[Image.Image, dict]:
    frame_w, frame_h = frames[0][2].size
    columns = max(1, min(max_columns, len(frames)))
    rows = math.ceil(len(frames) / columns)
    sheet = Image.new("RGBA", (columns * frame_w, rows * frame_h), (0, 0, 0, 0))
    anims: dict[str, dict[str, list[dict]]] = {}

    for idx, (tag, duration, image) in enumerate(frames):
        x = (idx % columns) * frame_w
        y = (idx // columns) * frame_h
        sheet.alpha_composite(image, (x, y))
        anims.setdefault(tag, {"frames": []})["frames"].append(
            {"duration": duration, "data": {"x": x, "y": y, "w": frame_w, "h": frame_h}}
        )

    return sheet, {"anims": anims}


def sampled_indices(total: int, limit: int) -> list[int]:
    if total <= limit:
        return list(range(total))
    if limit <= 1:
        return [0]
    return sorted({round(i * (total - 1) / (limit - 1)) for i in range(limit)})


def process(base: Path, max_columns: int) -> None:
    pokemon = base.name
    sheet_path = base.with_name(base.name + "#sheet.png")
    anim_path = base.with_name(base.name + "#anim.fanim")
    sheet = Image.open(sheet_path).convert("RGBA")
    anim = load_anim(anim_path)
    source_frames = base_frames(anim)
    frame_images = [crop_frame(sheet, frame) for frame in source_frames]
    durations = [float(frame.get("duration", 0.1)) for frame in source_frames]

    icon_path = icon_path_for(base)
    icon_path.parent.mkdir(parents=True, exist_ok=True)
    centered_visible_frame(frame_images[0]).save(icon_path)

    out_frames: list[tuple[str, float, Image.Image]] = []
    for tag in (*BASE_TAGS, *ACTION_TAGS):
        if tag in {"idle", "dead"}:
            indices = [0]
        elif tag == "run":
            indices = sampled_indices(len(frame_images), RUN_FRAME_LIMIT)
        else:
            indices = sampled_indices(len(frame_images), ACTION_FRAME_LIMIT)

        for index in indices:
            duration = durations[index]
            image = frame_images[index]
            out_frames.append((tag, duration, overlay_frame(image, pokemon, tag, index, len(frame_images))))

    out_sheet, out_anim = pack(out_frames, max_columns)
    out_sheet.save(sheet_path)
    with anim_path.open("w", encoding="utf-8") as f:
        json.dump(out_anim, f, indent=2)
        f.write("\n")
    print(f"OK   {sheet_path}")
    print(f"OK   {anim_path}")
    print(f"OK   {icon_path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", nargs="+", type=Path)
    parser.add_argument("--max-columns", type=int, default=16)
    args = parser.parse_args()
    for base in args.base:
        process(base, args.max_columns)


if __name__ == "__main__":
    main()
