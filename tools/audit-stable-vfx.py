"""Rebuild the per-move VFX checklist and validate every packaged animation frame.

First run cargo test --lib export_roster_visual_inventory. This is a source/asset
audit, not a replacement for an in-game render/collision acceptance test.
"""
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "mod/pokemon_moba_stable"
OUT = ROOT / "scratch/vfx-roster-audit"

# Findings require human review; absence of a note is not a visual sign-off.
NOTES = {
    "charizard": "Breaths use padded, 16-direction attached cast animations without per-tick restarting. Inferno has a detached targeted ring; baked ult ring removed. Mouth offset, pixel/world scale and facing need live acceptance.",
    "blastoise": "Twin basics launch separately, alternate cannon artwork and resolve one hit per missile. Detached reference cannon stays suppressed.",
    "feraligatr": "Muddy Water now travels at 10000 units/tick with swept contact damage, not immediate whole-line damage.",
    "decidueye": "Leafage launches three separate piercing leaves along the fan. Leaf Storm uses fast swept wave contact; self-retreat occurs once at launch.",
    "blissey": "Heal Pulse now has a standalone aura refreshed from its live 10-second healing state; duplicate body aura removed.",
    "beeheeyem": "Light Screen uses stored endpoints, expiry and a 16-direction asset variant. Exact rendered size/rotation still needs live acceptance.",
    "octillery": "Bouncy Bubble now has three 12-tick legs, three independent circle impacts 36000 units apart, and a final burst. Known-map boundaries clamp centers. Bullet Seed launches three independent projectiles.",
    "banette": "Phantom Force splash is emitted only on kill. Duplicate smoke/splash body overlays removed using the preserved source poses.",
    "oranguru": "Nasty Plot now stays on its target; Stored Power burst occurs only when stored damage is released, at the caster's current position.",
    "starmie": "Heart Swap remains conditional; Psybeam uses delayed directional art. Flip Turn now hits/stuns on movement contact and schedules retreat from contact, not from initial cast.",
    "gholdengo": "Make It Rain now uses the actual gold-expanded gameplay radius and center.",
    "shedinja": "Chit rendering now chooses the matching fixed count tag and removes other count tags; not a countdown animation.",
    "arboliva": "Seed Bomb now uses projectile transport and its splash resolver at impact. Persistent terrain remains position-driven; scale/ground anchoring needs engine acceptance.",
    "umbreon": "Dark Pulse uses an engine first-contact projectile and grants its buff at impact. Snarl remains caster-centered by user decision; its description now agrees.",
    "sylveon": "Disarming Voice remains caster-centered by user decision; its description now agrees. Charm's heal visual is currently a cast pulse rather than a five-second status marker.",
    "zeraora": "Zing Zap counter art now triggers on the actual counter, not entering parry. Wild Charge now emits at each selected victim. Counter movement still uses forced movement and damage can resolve before arrival.",
    "eevee": "Wish ring now fires on the heal, including early release. Baton Pass tether currently is one burst, not a live 30-second connection.",
    "ampharos": "Delayed strike and persistent zone now use separate art; zone refreshes without repeatedly playing the lightning strike.",
    "xatu": "Super Psy uses current endpoints and pre-rotated directional variants on each beam tick. Caster-channel rings remain distinct from target marks.",
    "yanmega": "Giga Drain uses current endpoints and directional variants on each drain tick. Live scale/facing acceptance remains.",
    "delibird": "Removed the reference-only Drill Peck overlay; the flourish is baked into the body. Healing Present has no separate staged heal effect.",
    "coalossal": "Each advancing pair of Magma Storm segments and its final circle now emit distinct phase assets at their authoritative endpoints.",
    "arcanine": "White Flames uses two offset fast-moving fronts with shared per-cast hit deduplication. Burn/heal resolve on swept contact. Removed the doubled-beam animation frame.",
    "kecleon": "Colorful Whip pulses are now separated by the configured interval instead of executing all hits immediately. Each pulse has directional dual-cone art.",
    "dedenne": "Grid sheets now contain one node/event per frame. Play Rough spends charges once at launch, then resolves damage/silence on actual swept movement contact.",
    "turtonator": "One trap idle per world trap; separate burst only on trigger. Sunny Day has directional variants, pending live footprint verification.",
    "clawitzer": "Water Sport now emits one isolated splash at each of its three gameplay centers. Attachment/copy paths need live acceptance.",
    "dragalge": "Sludge Bomb emits isolated splashes. Toxic Chain now places rotated chain links along the actual caster/victim segment, plus a separate victim impact, from successful poison/execute resolution. It does not rely on stretching one Animation. It is a brief cast link, not a 15-second poison tether. Link spacing/scale need live acceptance.",
    "ribombee": "U-Turn approaches the ally first; rescue movement/control only starts at actual ally contact. Its trail is attached for the outbound dash.",
    "drednaw": "Razor Shell retains the initial spin, then uses swept retreat-dash hits and a dash-lifetime trail. Copying retains the full original move.",
    "swanna": "Separate boundary, contact-slash and landing assets. Damage pulses use accumulated actual movement contacts, not planned untraveled waypoints; blocked casts cannot land remotely.",
    "cloyster": "Icicle Crash launches level-dependent, individually spaced first-contact icicles; one resolver hit per icicle.",
    "dragapult": "Dragon Darts launches two independent missiles with isolated one-dart artwork. Dreepy retains its own route.",
    "jolteon": "Pin Missile launches three independent missiles with isolated projectile artwork.",
    "passimian": "Copied passive visuals require testing under the receiver entity, not only on the original species.",
    "smeargle": "Source move identity selects detached VFX while Smeargle stays the caster. Body-baked artwork cannot be inherited without replacing/rebuilding his body animations.",
    "sawk_throh": "No public full-body sprite-set swap is wired. Current stance visuals are not a complete Throh transformation.",
    "ursaluna": "Blood Moon overlay is not a full Bloodmoon Ursaluna sprite-set replacement.",
    "comfey": "Petal Dance follows Comfey's attached entity position. Floral Healing and Encore have no standalone staged VFX.",
    "wishiwashi": "Wave Splash now uses fast swept contact for enemies and allies, preserving healing/cleanse and Schooling range. Massive Catch's body/grab/return phases still need live acceptance.",
}

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    roster = json.loads((ROOT / "scratch/vfx-runtime-inventory.json").read_text())
    guide = {c["id"]: c for c in json.loads((ROOT / "scratch/pokemon_guide_data.json").read_text())["champions"]}
    catalog = json.loads((ROOT / "assets/custom_spritework/stable_vfx_bindings.generated.json").read_text())["bindings"]
    assert len(roster) == 103
    assert sum(len(c["actions"]) for c in roster) == 412
    overrides = json.loads((PACKAGE / "mod.override_info").read_text())
    body_frames = 0
    bodies = []
    for champion in roster:
        key = f"asset/base/aseprite_resources/champions/{champion['id']}"
        base = f"asset/{PACKAGE.name}/"
        anim_path = PACKAGE / (overrides[key+"#anim"]["remapping"].removeprefix(base)+".fanim")
        sheet_path = PACKAGE / (overrides[key+"#sheet"]["remapping"].removeprefix(base)+".png")
        anim = json.loads(anim_path.read_text())["anims"]
        sheet = Image.open(sheet_path).convert("RGBA")
        strip = Image.new("RGBA", (300, 100))
        for column, tag in enumerate(("attack", "skill", "skill2", "ult")):
            assert tag in anim and anim[tag]["frames"], (champion["id"], tag)
            frames = anim[tag]["frames"]
            d = frames[len(frames)//2]["data"]
            tile = sheet.crop((d["x"], d["y"], d["x"]+d["w"], d["y"]+d["h"]))
            tile.thumbnail((74, 82), Image.Resampling.NEAREST)
            strip.paste(tile, (column*75+(75-tile.width)//2, 0), tile)
        for tag, animation in anim.items():
            for frame in animation["frames"]:
                d = frame["data"]
                assert frame["duration"] > 0
                assert 0 <= d["x"] < d["x"]+d["w"] <= sheet.width, (champion["id"], tag)
                assert 0 <= d["y"] < d["y"]+d["h"] <= sheet.height, (champion["id"], tag)
                body_frames += 1
        bodies.append((champion["name"],strip))
    for page in range((len(bodies)+35)//36):
        canvas=Image.new("RGB",(960,1320),(42,48,55))
        draw=ImageDraw.Draw(canvas)
        for i,(name,strip) in enumerate(bodies[page*36:(page+1)*36]):
            x,y=(i%3)*320,(i//3)*110
            canvas.paste(strip,(x,y),strip)
            draw.text((x+6,y+85),name+" | attack / skill / skill2 / ult",fill="white")
        canvas.save(OUT/f"bodies-{page+1}.png")
    thumbs, frame_count = [], 0
    for entry in catalog:
        stem = entry["stem"]
        anim = json.loads((PACKAGE / "vfx" / f"{stem}#anim.fanim").read_text())
        sheet = Image.open(PACKAGE / "vfx" / f"{stem}#sheet.png").convert("RGBA")
        for tag, data in anim["anims"].items():
            assert data["frames"], (stem, tag)
            for frame in data["frames"]:
                d = frame["data"]
                assert frame["duration"] > 0, (stem, tag, frame)
                assert d["x"] >= 0 and d["y"] >= 0 and d["w"] > 0 and d["h"] > 0
                assert d["x"] + d["w"] <= sheet.width and d["y"] + d["h"] <= sheet.height, (stem, tag, d)
                frame_count += 1
        frames = anim["anims"][entry["tag"]]["frames"]
        # Keep transparent canvas margins; they are part of the authored anchor.
        d = frames[len(frames)//2]["data"]
        tile = sheet.crop((d["x"], d["y"], d["x"]+d["w"], d["y"]+d["h"]))
        tile.thumbnail((150, 100), Image.Resampling.NEAREST)
        thumbs.append((stem, tile))
    for page in range((len(thumbs)+47)//48):
        canvas = Image.new("RGB", (1200, 1080), (42, 48, 55))
        draw = ImageDraw.Draw(canvas)
        for i, (stem, tile) in enumerate(thumbs[page*48:(page+1)*48]):
            x, y = (i % 6)*200, (i//6)*135
            canvas.paste(tile, (x+(200-tile.width)//2, y), tile)
            label = stem.replace("_", " ")
            chunks = [label[j:j+30] for j in range(0, len(label), 30)]
            draw.multiline_text((x+3,y+102), "\n".join(chunks), fill="white", spacing=0)
        canvas.save(OUT/f"assets-{page+1}.png")
    lines = ["# Roster VFX Audit", "", "Generated by `tools/audit-stable-vfx.py` from the compiled roster and packaged assets.", "",
             f"Scope: 103 Pokemon, 412 actions, 103 passives, {len(catalog)} staged VFX assets ({frame_count} validated frames), 103 mapped body sheets ({body_frames} validated frames and all four action tags).", "",
             "This is NOT an in-game visual sign-off. All rows still require live acceptance for scale, depth, orientation, cancellation and copied casts. 'Body only' means no separately staged VFX, not a missing attack.", "",
             "Native animation bindings expose no arbitrary rotation or mouth/bone anchor. Directional line/cone/rectangle art cannot currently be certified against arbitrary world-space hitboxes. Persistent ground visuals use short leases and can leave a tail of up to 30 ticks after removal.", ""]
    for champion in roster:
        short = champion["id"].removeprefix("pokemon_moba_")
        cg = guide[champion["id"]]
        lines.extend([f"## {champion['name']}", "", NOTES.get(short, "No extra character-specific note beyond the route/asset checks below. This is not a visual correctness sign-off."), ""])
        for action in champion["actions"]:
            move = cg["moves"][action["slot"]]
            assets = action["assets"]
            def policy(a):
                if a["stem"] in ("blastoise_ult_cannon", "delibird_ult_drill_peck_flourish"):
                    return "suppressed reference; baked into body"
                if action["effect"].startswith("TravelingWave"):
                    return "swept traveling-wave transport"
                return "projectile impact" if a["projectile"] else "state/event" if a["state_driven"] else "dash lifetime" if a["dash_streak"] else "cast animation"
            route = "; ".join(a["stem"] + " (" + policy(a) + ")" for a in assets) or "Body only / no standalone asset staged"
            lines.extend([f"### {action['slot']}: {move['name']}", "", move["description"], "",
                          f"Gameplay: `{action['effect']}`", "", f"Visual route: {route}.", "",
                          f"Action timing: start {action['start']}, duration {action['duration']} ticks; moving={action['moving']}; input={action['casting']}/{action['target']}.", ""])
        lines.extend(["### Passive", "", cg["moves"][4]["description"], "",
                      "Assets: " + (", ".join(champion["passive_assets"]) or "No separate passive asset staged") + ".", ""])
    (ROOT / "docs/roster-vfx-audit.generated.md").write_text("\n".join(lines), encoding="utf-8")
    print(f"Validated {len(catalog)} sheets, {frame_count} frames; wrote 515-entry checklist and {page+1} contact sheets.")

if __name__ == "__main__":
    main()
