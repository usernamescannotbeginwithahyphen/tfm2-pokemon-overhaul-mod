import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTENT = ROOT / "mod" / "pokemon_moba" / "src" / "pokemon_content.rs"
REFERENCE = ROOT / "docs" / "project-reference.md"
VFX = ROOT / "mod" / "pokemon_moba_stable" / "stable_vfx_bindings.generated.json"
OUT = ROOT / "docs" / "stable-champion-migration-audit.generated.md"


UNLOCKS = {
    "pokemon_moba_banette": "Review Phantom Force as true teleport-to-target behavior.",
    "pokemon_moba_skarmory": "Review Fly/global travel separately from normal dashes.",
    "pokemon_moba_audino": "Review Substitute/ally rescue as true displacement or swap.",
    "pokemon_moba_kricketune": "Implement Sticky Web as persistent deployable/control zone with staged VFX.",
    "pokemon_moba_octillery": "Revisit Suction Cups with stable ignore-wall/pathing options.",
    "pokemon_moba_noivern": "Revisit Infiltrator tower targeting/aggro with live sim and AI hooks.",
    "pokemon_moba_oranguru": "Revisit Symbiosis using stable stat/item-derived read paths.",
    "pokemon_moba_ambipom": "Revisit Technician with stable buff reads and Pokemon buff ledger.",
    "pokemon_moba_ceruledge": "Revisit Poltergeist item/stat-surplus behavior and staged VFX.",
    "pokemon_moba_gholdengo": "Use stable player gold mutation for Good as Gold/Make It Rain.",
    "pokemon_moba_wishiwashi": "Implement Cowardice allied shield as a real shield buff.",
}


def extract_champions() -> list[tuple[str, str]]:
    text = CONTENT.read_text(encoding="utf-8")
    rows = re.findall(
        r'id:\s*"(?P<id>pokemon_moba_[^"]+)".{0,160}?display_name:\s*"(?P<name>[^"]+)"',
        text,
        flags=re.DOTALL,
    )
    seen = set()
    result = []
    for champion_id, name in rows:
        if champion_id not in seen:
            seen.add(champion_id)
            result.append((champion_id, name))
    return result


def reference_line_for(champion_id: str, name: str) -> str:
    ref = REFERENCE.read_text(encoding="utf-8")
    for line in ref.splitlines():
        if f"- {name}:" in line or champion_id in line:
            return line.strip()
    return ""


def main() -> None:
    vfx_data = json.loads(VFX.read_text(encoding="utf-8")) if VFX.exists() else {"champions": {}}
    lines = [
        "# Stable Champion Migration Audit",
        "",
        "Generated from the classic roster, project reference notes, and staged stable VFX bindings.",
        "",
        "| Champion | VFX | Stable Revisit | Reference |",
        "| --- | ---: | --- | --- |",
    ]
    for champion_id, name in extract_champions():
        vfx_count = len(vfx_data.get("champions", {}).get(champion_id, []))
        revisit = UNLOCKS.get(champion_id, "")
        reference = reference_line_for(champion_id, name)
        reference = reference.replace("|", "\\|")
        lines.append(f"| {name} (`{champion_id}`) | {vfx_count} | {revisit} | {reference} |")

    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Wrote {OUT}")


if __name__ == "__main__":
    main()
