import json
import re
from collections import Counter
from hashlib import sha256
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTENT = ROOT / "mod" / "pokemon_moba_stable" / "src" / "legacy" / "pokemon_content.rs"
VFX = ROOT / "assets" / "custom_spritework" / "stable_vfx_bindings.generated.json"
OUT = ROOT / "docs" / "stable-champion-migration-audit.generated.md"
GUIDE = ROOT / "scratch" / "pokemon_guide_data.json"


REVIEWS = {
    "banette": "M1 fixed 0.2.4: native Phantom Force teleport; live landing test.",
    "skarmory": "M1 fixed 0.2.4: native Fly teleport; Sturdy covers Pokemon resolver hits only (C3).",
    "audino": "M1 fixed 0.2.4: native Substitute position swap; Protect redirection preserved (C3).",
    "kricketune": "Shared detection wired; 0.2.7 named web refresh and immediate field leases. Live overlap/lifecycle validation remains.",
    "octillery": "M2 fixed 0.2.4: ignore_wall and -15% speed; route selection needs testing.",
    "noivern": "Explicitly deferred: Infiltrator unchanged; no tower-aggro interception API found.",
    "oranguru": "I1: equipment-derived Symbiosis implemented; no item-proc sharing; copied Instruct targeting needs A1.",
    "ambipom": "I1: equipment Thief implemented; Technician still Pokemon buff ledger.",
    "ceruledge": "I1: equipment-derived Poltergeist implemented; validate equipment changes/expiry.",
    "gholdengo": "G1: +10 credited-CS callback implemented; duplicate/death/replay tests remain.",
    "wishiwashi": "C3/F1: native shields supported; Schooling state and overlays are not body swaps.",
    "smeargle": "0.2.7 preserves copied move element/art with actual caster STAB; persistent copies still need live tests.",
    "shiftry": "Map-grid occupancy, named speed refresh and Receiver camouflage updater wired. Live map validation pending.",
    "weavile": "S1 improved 0.2.4: native Hunt invisibility plus custom protection; reveal/break live tests.",
    "kleavor": "S1: Stealth Rock uses custom alternating untargetability; do not blindly replace with banish.",
    "comfey": "A1/S1: attachment target/recall gates; retain casting while attached; Encore is currently haste.",
    "clawitzer": "A1/S1: attachment target/recall gates; soft untargetability not engine-wide immunity.",
    "ampharos": "0.2.6 continuous shared team detection and Weavile reveal wired; live host attack acceptance pending.",
    "dedenne": "0.2.7 grid refresh, configured node damage and cooldown gate fixed. Bespoke Play Rough timing remains separate; live grid acceptance pending.",
    "mantine": "0.2.7 Swift Swim refreshes one named speed buff; live water entry/exit validation remains.",
    "scizor": "0.2.7 Light Metal uses native shield without custom double absorption; live engine-origin hits need validation.",
    "heliolisk": "0.2.7 separate Shed Tail unit/taunt and swept piercing dash; live summon/body/lifetime validation remains.",
    "electrode": "0.2.7 post-respawn speed outcome fixed and one-shot tested; no public respawn-countdown setter.",
    "xatu": "S1: custom reveal must coordinate with any native invisibility migration.",
    "starmie": "S1/F1: Illuminate reveal, mark size/anchor/lifetime verification.",
    "sawk_throh": "A1/F1: state-dependent cast gates available; full-model swapping not exposed.",
    "ursaluna": "C3/F1: Blood Moon combat rules preserved; no public full-model swapping API found.",
    "shedinja": "0.2.7 dash-cone resolves on arrival; missing damage classification fixed. Wonder Guard remains custom pre-damage logic.",
    "blissey": "C1/C2 fixed 0.2.4: one anti-heal owner and native/custom cleanse; live engine verification.",
    "thievul": "C1 fixed 0.2.4: strongest-source named anti-heal with expiry tests; live engine verification.",
    "appletun": "C1: Ripen amplification, HP cap and anti-heal order need engine tests.",
    "yanmega": "C3: drain immunity is Pokemon-layer-only; engine item lifesteal requires verification.",
    "passimian": "Copied CS payout, Inteleon/Emboar named refresh and Shiftry updater wired; full copied-passive live acceptance remains.",
    "weezing": "C3: Nullifying Gas suppression must cover copied passives and all new callbacks.",
    "magmortar": "P1: per-shot impacts implemented; launch-vs-impact stat sampling changed.",
    "marowak": "P1: outbound/return collisions, catch timing and Rhythm refresh require live tests.",
    "dragapult": "P1: Dreepy callback flight, spawn origin and source/copy ownership tests.",
}


def inventory(text: str) -> dict:
    # Narrow extraction of this file's constant declarations, not a general Rust parser.
    constants = dict(re.findall(r"^const (\w+): Pokemon(?:Champion|Move) = .*?\{\n(.*?)^};", text, re.M | re.S))
    result = {}
    for symbol, body in constants.items():
        match = re.search(r'id: "(pokemon_moba_[^"]+)"', body)
        if not match:
            continue
        actions = {}
        for slot in ("attack", "skill", "skill2", "ult"):
            value = re.search(rf"^    {slot}: (.*?)(?=^    (?:attack|skill|skill2|ult):|\Z)", body, re.M | re.S)
            if not value:
                raise ValueError(f"Missing {symbol}.{slot}")
            action = value.group(1)
            if "PokemonMove {" not in action:
                reference = re.fullmatch(r"(?:Some\()?([A-Z_][A-Z_0-9]*)\)?,\s*", action)
                if not reference:
                    raise ValueError(f"Unsupported action syntax: {symbol}.{slot}")
                action = constants[reference.group(1)]
            effects = re.findall(r"effect: PokemonMoveEffect::(\w+)", action)
            if len(effects) != 1:
                raise ValueError(f"Expected one effect: {symbol}.{slot}: {effects}")
            actions[slot] = effects[0]
        if match.group(1) in result:
            raise ValueError(f"Duplicate champion: {match.group(1)}")
        result[match.group(1)] = actions
    return result


def review_families(actions: dict, passive: str) -> str:
    content = " ".join(actions.values()) + " " + passive
    families = []
    for code, pattern in [
        ("C1/C2", r"Heal|Cleanse|healing|cleanse"),
        ("C3", r"Shield|Endure|Reflect|Immune|immune|reflect|critical|Crit|Guard"),
        ("M1/A2", r"Dash|Force|Swap|Fly|Retreat|Throw|Rescue|Knock|Pull"),
        ("A1", r"Charge|Channel|Stance|Attach|Sketch|Instruct|Sleep|cooldown|stack"),
        ("F1", r"Area|Aura|Terrain|Field|Web|Trail|Grid|Zone|Whirlpool|Tornado|Patch"),
        ("S1", r"invisib|untarget|stealth|revea|bush"),
    ]:
        if re.search(pattern, content, re.I):
            families.append(code)
    return ", ".join(families) or "Shared baseline"


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


def main() -> None:
    source = CONTENT.read_text(encoding="utf-8")
    actions = inventory(source)
    guide = json.loads(GUIDE.read_text(encoding="utf-8"))["champions"]
    vfx_data = json.loads(VFX.read_text(encoding="utf-8"))
    champion_ids = {champion_id for champion_id, _ in extract_champions()}
    if champion_ids != set(actions) or champion_ids != {champion["id"] for champion in guide}:
        raise ValueError("Roster/source/guide coverage mismatch; regenerate the guide export first")
    if len(actions) != 103 or len(guide) != 103 or sum(map(len, actions.values())) != 412:
        raise ValueError("Roster changed; review the full-audit coverage expectation")
    for champion in guide:
        if len(champion["moves"]) != 5 or sum(move["id"] == "passive" for move in champion["moves"]) != 1:
            raise ValueError(f"Expected four actions and one passive: {champion['id']}")
    counts = Counter(binding["champion_id"] for binding in vfx_data["bindings"])
    projectiles = Counter(binding["champion_id"] for binding in vfx_data["bindings"] if binding["projectile"])
    lines = [
        "# Stable Champion Migration Audit",
        "",
        "SDK 0.6.4 / build 0.2.8 source inventory. 103 champions, 412 actions, 103 passive descriptions.",
        "Run `cargo test --offline` in the stable package before regenerating to refresh the guide export.",
        f"Content SHA-256 (normalized text): `{sha256(source.encode()).hexdigest()}`.",
        "",
        "This is an exhaustive inventory and review checklist, NOT 103 live-tested champions.",
        "All rows inherit the shared runtime, damage/statistics, AI, draft and deployment checks.",
        "Family tags are mechanically assigned review prompts, not confirmed per-champion defects.",
        "Finding codes and evidence: [full SDK audit](sdk-0.6.4-findings.md#whole-mod-audit-follow-up).",
        "Shared fixes and remaining limits: [0.2.4 implementation](sdk-0.6.4-findings.md#build-024-audit-fixes).",
        "Current fixes and coverage limits: [0.2.7 combat semantics](sdk-0.6.4-findings.md#build-027-combat-semantics-fixes).",
        "VFX counts are staged asset bindings, not proof of correct runtime playback. P = projectile assets.",
        "Actions are listed in attack / skill / skill2 / ultimate order; repeated variants are intentional.",
        "",
        "| Champion | Four action implementations | Passive | VFX / P | Review families | Specific follow-up |",
        "| --- | --- | --- | ---: | --- | --- |",
    ]
    for champion in guide:
        champion_id, name = champion["id"], champion["name"]
        passive = next(move for move in champion["moves"] if move["id"] == "passive")
        passive_name = re.sub(r"<[^>]*>", "", passive["name"]).removeprefix("Passive - ")
        effects = " / ".join(f"`{effect}`" for effect in actions[champion_id].values())
        revisit = REVIEWS.get(champion_id.removeprefix("pokemon_moba_"), "Shared checks + family-specific regression tests.")
        families = review_families(actions[champion_id], passive["description"])
        lines.append(f"| {name} (`{champion_id}`) | {effects} | {passive_name} | {counts[champion_id]} / {projectiles[champion_id]} | {families} | {revisit} |")

    lines.extend(["", "## Shared Effect Coverage", "", "Every action variant below inherits the shared resolver/adapter audit; occurrence counts are not test counts.", "", "| Effect | Roster actions |", "| --- | ---: |"])
    for effect, count in sorted(Counter(effect for moves in actions.values() for effect in moves.values()).items()):
        lines.append(f"| `{effect}` | {count} |")
    missing = [champion["name"] for champion in guide if not counts[champion["id"]]]
    lines.extend(["", "## No Staged VFX Bindings", "", ", ".join(missing), "", "No staged binding does not imply a broken skill: some moves have no separate VFX artwork. Smeargle reuses the copied move's assets."])

    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Wrote {OUT}")


if __name__ == "__main__":
    main()
