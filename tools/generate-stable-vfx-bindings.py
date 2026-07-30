import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MOD_ID = "pokemon_moba_stable"
VFX_DIR = ROOT / "mod" / MOD_ID / "vfx"
OUT = ROOT / "mod" / MOD_ID / "stable_vfx_bindings.generated.json"


PROJECTILE_HINTS = ("projectile", "cannonball")
BUFF_HINTS = ("passive", "aura", "field", "terrain", "trail", "mark")


def champion_from_stem(stem: str) -> str:
    return stem.split("_", 1)[0]


def classify(stem: str) -> str:
    lowered = stem.lower()
    if any(hint in lowered for hint in PROJECTILE_HINTS):
        return "projectile"
    if any(hint in lowered for hint in BUFF_HINTS):
        return "buff_or_effect"
    return "effect"


def first_anim_tag(anim_path: Path) -> str:
    try:
        data = json.loads(anim_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return "default"
    anims = data.get("anims")
    if isinstance(anims, dict) and anims:
        return next(iter(anims.keys()))
    return "default"


def binding_for(anim_path: Path) -> dict:
    stem = anim_path.name.removesuffix("#anim.fanim")
    champion = champion_from_stem(stem)
    kind = classify(stem)
    asset = f"asset/{MOD_ID}/vfx/{stem}"
    tag = first_anim_tag(anim_path)
    binding = {
        "champion_id": f"pokemon_moba_{champion}",
        "source": str(anim_path.relative_to(ROOT)).replace("\\", "/"),
        "binding_name": f"{MOD_ID}_{stem}",
        "asset": asset,
        "tag": tag,
        "kind": kind,
    }
    if kind == "projectile":
        binding["view_projectile"] = {
            "type": "Animated",
            "name": binding["binding_name"],
            "anim": asset,
            "tag": tag,
            "z": 0,
            "repeat": True,
        }
    else:
        binding["view_effect"] = {
            "type": "Animation",
            "name": binding["binding_name"],
            "anim": asset,
            "tag": tag,
            "z": 0,
            "is_follow": "aura" in stem or "passive" in stem,
        }
    return binding


def main() -> None:
    bindings = [
        binding_for(path)
        for path in sorted(VFX_DIR.glob("*#anim.fanim"))
    ]
    by_champion: dict[str, list[dict]] = {}
    for binding in bindings:
        by_champion.setdefault(binding["champion_id"], []).append(binding)

    OUT.write_text(
        json.dumps(
            {
                "mod_id": MOD_ID,
                "note": (
                    "Generated VFX binding candidates. Do not load directly as "
                    ".data_champion files until the matching stable Rust champion "
                    "is registered."
                ),
                "count": len(bindings),
                "champions": by_champion,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"Wrote {len(bindings)} VFX binding candidates to {OUT}")


if __name__ == "__main__":
    main()
