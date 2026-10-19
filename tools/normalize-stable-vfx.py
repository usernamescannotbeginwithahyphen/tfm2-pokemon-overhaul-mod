"""Normalize staged stable VFX to dimensions that match the in-game renderer."""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[1]
MOD_VFX = ROOT / "mod" / "pokemon_moba_stable" / "vfx"
SOURCE_VFX = ROOT / "assets" / "custom_spritework" / "vfx"
BINDINGS = ROOT / "assets" / "custom_spritework" / "stable_vfx_bindings.generated.json"


@dataclass(frozen=True)
class Profile:
    max_width: int
    max_height: int
    anchor_y: int | None = None


ATTACHMENT_PROFILES = {
    "ludicolo_passive_rain_dish_aura": Profile(58, 58, anchor_y=68),
    "quaquaval_passive_aqua_step_trail": Profile(54, 18, anchor_y=50),
    "quaquaval_passive_aqua_step_empowered_trail": Profile(54, 18, anchor_y=50),
    "sigilyph_passive_glypher_mark": Profile(18, 26, anchor_y=20),
    "shedinja_passive_wonder_guard_chits": Profile(36, 8, anchor_y=16),
}
PROJECTILE_PROFILE = Profile(56, 28)


def fit_content(image: Image.Image, profile: Profile) -> Image.Image:
    if image.width <= profile.max_width and image.height <= profile.max_height:
        return image
    scale = min(profile.max_width / image.width, profile.max_height / image.height)
    size = (max(1, int(image.width * scale)), max(1, int(image.height * scale)))
    return image.resize(size, Image.Resampling.NEAREST)


def normalize_sheet(
    sheet_path: Path, fanim_path: Path, profile: Profile, *, write: bool
) -> bool:
    sheet = Image.open(sheet_path).convert("RGBA")
    fanim = json.loads(fanim_path.read_text(encoding="utf-8"))
    normalized = Image.new("RGBA", sheet.size, (0, 0, 0, 0))
    changed = False
    processed_boxes: set[tuple[int, int, int, int]] = set()

    for animation in fanim["anims"].values():
        for frame in animation["frames"]:
            data = frame["data"]
            box = (
                int(data["x"]),
                int(data["y"]),
                int(data["x"]) + int(data["w"]),
                int(data["y"]) + int(data["h"]),
            )
            if box in processed_boxes:
                continue
            processed_boxes.add(box)
            original_frame = sheet.crop(box)
            bbox = original_frame.getchannel("A").getbbox()
            if not bbox:
                continue
            content = fit_content(original_frame.crop(bbox), profile)
            x = int(data["x"]) + (int(data["w"]) - content.width) // 2
            if profile.anchor_y is None:
                y = int(data["y"]) + (int(data["h"]) - content.height) // 2
            else:
                y = int(data["y"]) + profile.anchor_y - content.height // 2
            normalized.alpha_composite(content, (x, y))
            changed |= normalized.crop(box).tobytes() != original_frame.tobytes()

    if changed and write:
        normalized.save(sheet_path)
    return changed


def staged_profiles() -> dict[str, Profile]:
    profiles = dict(ATTACHMENT_PROFILES)
    data = json.loads(BINDINGS.read_text(encoding="utf-8"))
    for bindings in data["champions"].values():
        for binding in bindings:
            if binding["kind"] == "projectile":
                profiles[Path(binding["asset"]).name] = PROJECTILE_PROFILE
    return profiles


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Report sheets that need normalization without writing them.",
    )
    args = parser.parse_args()

    changed = 0
    missing = 0
    for stem, profile in sorted(staged_profiles().items()):
        mod_sheet = MOD_VFX / f"{stem}#sheet.png"
        mod_fanim = MOD_VFX / f"{stem}#anim.fanim"
        if not mod_sheet.exists() or not mod_fanim.exists():
            print(f"MISS {stem}")
            missing += 1
            continue

        would_change = normalize_sheet(mod_sheet, mod_fanim, profile, write=not args.check)
        if would_change:
            changed += 1
            print(f"{'NEED' if args.check else 'OK  '} {stem}")

        source_sheet = SOURCE_VFX / mod_sheet.name
        source_fanim = SOURCE_VFX / mod_fanim.name
        if not args.check and source_sheet.exists() and source_fanim.exists():
            source_sheet.write_bytes(mod_sheet.read_bytes())

    print(f"normalized={changed} missing={missing}")


if __name__ == "__main__":
    main()
