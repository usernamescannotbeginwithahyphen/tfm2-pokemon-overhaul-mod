from __future__ import annotations

import base64
import html
import json
import re
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATA_PATH = ROOT / "scratch" / "pokemon_guide_data.json"
OUT_DIR = ROOT / "guide"
HTML_PATH = OUT_DIR / "pokemon_moba_guide.html"
MOD_GUIDE_DIR = ROOT / "mod" / "pokemon_moba" / "guide"
MOD_HTML_PATH = MOD_GUIDE_DIR / "pokemon_moba_guide.html"
ICON_DIR = ROOT / "mod" / "pokemon_moba" / "icons" / "champions"

TYPE_COLORS = {
    "Normal": "#b7b7a8",
    "Fire": "#ef6b3a",
    "Water": "#3f87d9",
    "Electric": "#f0c93d",
    "Grass": "#59a94f",
    "Ice": "#69bfd4",
    "Fighting": "#bc4b3f",
    "Poison": "#a45ab7",
    "Ground": "#c6924c",
    "Flying": "#7f9fe5",
    "Psychic": "#e85c96",
    "Bug": "#9eb83f",
    "Rock": "#a89462",
    "Ghost": "#6d5a9b",
    "Dragon": "#7657d9",
    "Dark": "#5c5268",
    "Steel": "#9ca7b7",
    "Fairy": "#e78fbd",
}

TYPE_ABBR = {
    "Normal": "Nor",
    "Fire": "Fire",
    "Water": "Water",
    "Electric": "Elec",
    "Grass": "Grass",
    "Ice": "Ice",
    "Fighting": "Fight",
    "Poison": "Pois",
    "Ground": "Ground",
    "Flying": "Fly",
    "Psychic": "Psych",
    "Bug": "Bug",
    "Rock": "Rock",
    "Ghost": "Ghost",
    "Dragon": "Drag",
    "Dark": "Dark",
    "Steel": "Steel",
    "Fairy": "Fairy",
}

SPECIAL_NOTES = {
    "pokemon_moba_clawitzer": [
        "Draft warning: Clawitzer is incompatible with Comfey. Both champions attach to allies and should not be drafted together.",
    ],
    "pokemon_moba_comfey": [
        "Draft warning: Comfey is incompatible with Clawitzer. Both champions attach to allies and should not be drafted together.",
    ],
    "pokemon_moba_passimian": [
        "Receiver exclusions: Passimian cannot copy Receiver, Clingy, Flower Veil, or Sketch, so kills on Passimian, Clawitzer, Comfey, and Smeargle do not replace Passimian's current copied passive.",
        "Receiver implementation note: Noivern's Infiltrator and Octillery's Suction Cups are copyable, but their copied effects are pending lower-level SDK hooks.",
    ],
}

STAT_LABELS = [
    ("attack", "Attack"),
    ("magic_power", "AP"),
    ("hp", "HP"),
    ("defence", "Defense"),
    ("magic_resistance", "Magic Resist"),
    ("move_speed", "Move Speed"),
    ("hp_regen", "HP Regen"),
    ("crit_chance", "Crit"),
]

STATUS_GLOSSARY = [
    (
        "Burn",
        "Damage over time, usually applied by Fire moves. Burn ticks once per second for its listed duration, and several Fire kits gain bonus effects against Burned targets.",
    ),
    (
        "Poison",
        "Stacking Poison-type damage over time. Poison entries in move text list the stack count, damage per second, and duration.",
    ),
    (
        "Miasma",
        "A Poison-family stack mechanic used by Arbok. Miasma builds stacks, can slow or detonate, and converts into Poison after enough stacks.",
    ),
    (
        "Bleed",
        "Physical damage over time. Several Fighting or slashing kits use Bleed to punish extended trades.",
    ),
    (
        "Infestation",
        "A rapid ticking damage state used by parasite or swarm-style effects. Read the move text for the damage and duration.",
    ),
    (
        "Paralysis",
        "Long-duration Electric control. While Paralyzed, a target has periodic chances to be stunned, and some Electric champions deal extra damage to Paralyzed enemies.",
    ),
    (
        "Frozen",
        "Hard crowd control from Ice moves. Frozen targets are temporarily unable to act for the duration listed in the move text.",
    ),
    (
        "Confusion",
        "Stacking disruption used by Psychic and trickster kits. Confusion can convert into forced or erratic behavior depending on the applying move.",
    ),
    (
        "Sleep",
        "A control state that can disable or set up healing windows. Frosmoth and Snorlax are the main Sleep users in this roster.",
    ),
    (
        "Stun",
        "Short hard CC. Stunned targets cannot act until the stun expires.",
    ),
    (
        "Root / Bind",
        "Movement control. Rooted or bound targets can be forced to stand still, sometimes while continuing to attack.",
    ),
    (
        "Taunt",
        "Forces a target to attack the taunter and blocks skill use for the listed duration.",
    ),
    (
        "Silence",
        "Prevents skill use while active. Some duel and challenge effects combine Silence with Root or Taunt.",
    ),
    (
        "Blind / Disarm",
        "Basic attack suppression. These effects are strongest into BasicAttacker and CritCarry champions.",
    ),
    (
        "Anti-Heal",
        "Healing reduction. This is a draft answer to Healer, HealReliant, and Sustain champions.",
    ),
    (
        "Soak",
        "Water setup effect. Soaked enemies are treated as Water-altered targets for the relevant follow-up effects.",
    ),
    (
        "Illuminate",
        "Starmie mark mechanic. Illuminate marks targets for Starmie's follow-up damage pattern.",
    ),
    (
        "Grassy Terrain",
        "Area sustain and positional value. Grass teams often want fights to happen inside their terrain or aura zones.",
    ),
    (
        "Sticky Web",
        "Area slow and anti-mobility tool. It is most valuable against Dive, HighMobility, and melee engage champions.",
    ),
    (
        "Protect / Guard",
        "Temporary damage prevention or shielding. Treat these as timing tools that can waste burst windows.",
    ),
    (
        "Untargetable",
        "A defensive state that temporarily removes the champion from normal targeting or follow-up windows.",
    ),
]

TRAIT_NOTES = [
    ("AdDamage", "Primary physical damage source."),
    ("ApDamage", "Primary magic or special damage source."),
    ("BacklineCarry", "Needs space and protection; high payoff if left alone."),
    ("BasicAttacker", "Scales heavily with attack speed, crit, and uninterrupted uptime."),
    ("CritCarry", "Physical carry that benefits from long fights and protection."),
    ("Dive", "Can reach backline or bypass ordinary front-to-back fights."),
    ("Dot", "Wins through damage over time, repeat hits, and status pressure."),
    ("Frontline", "Can stand in first contact and absorb pressure."),
    ("HardCc", "Reliable crowd control for picks, peel, or engage."),
    ("HealReliant", "Value drops sharply into Anti-Heal or burst."),
    ("Healer", "Team sustain engine; protect it or draft anti-heal into it."),
    ("HighDefense", "Naturally punishes low-damage or basic-attack-heavy teams."),
    ("HighHealth", "Can soak damage; vulnerable to TankBuster and percent HP damage."),
    ("HighMobility", "Can reposition or evade; weak to HardCc and Sticky Web style tools."),
    ("Poke", "Damages before full engage and pressures low mobility targets."),
    ("SingleTargetAssassin", "Kills isolated or squishy targets quickly."),
    ("Squishy", "High priority target if reachable."),
    ("Sustain", "Wins longer fights through healing, shields, or repeated recovery."),
    ("TankBuster", "Draft answer into HighHealth or HighDefense frontline."),
    ("TeamBuff", "Raises ally output or durability; stronger with coordinated comps."),
    ("Zoner", "Controls space and punishes enemies for walking through areas."),
]


def main() -> None:
    data = json.loads(DATA_PATH.read_text(encoding="utf-8"))
    champions = sorted(data["champions"], key=lambda item: item["name"])
    html_text = render_html(champions, data["type_chart"])
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    HTML_PATH.write_text(html_text, encoding="utf-8")
    MOD_GUIDE_DIR.mkdir(parents=True, exist_ok=True)
    MOD_HTML_PATH.write_text(html_text, encoding="utf-8")
    print(f"wrote {HTML_PATH}")
    print(f"wrote {MOD_HTML_PATH}")
    print(f"champions={len(champions)} type_chart_cells={len(data['type_chart'])}")


def render_html(champions: list[dict], type_chart: list[dict]) -> str:
    return "\n".join(
        [
            "<!doctype html>",
            '<html lang="en">',
            "<head>",
            '<meta charset="utf-8">',
            '<meta name="viewport" content="width=device-width, initial-scale=1">',
            "<title>Pokemon MOBA Mod Guide</title>",
            f"<style>{css()}</style>",
            "</head>",
            "<body>",
            render_cover(champions),
            render_toc(),
            render_quick_reference(champions),
            render_roster_index(champions),
            render_champion_sections(champions),
            render_status_section(),
            render_type_chart(type_chart),
            render_trait_section(),
            "</body>",
            "</html>",
        ]
    )


def render_cover(champions: list[dict]) -> str:
    type_counts = Counter(type_name for champ in champions for type_name in champ["types"])
    top_types = ", ".join(f"{name} {count}" for name, count in type_counts.most_common(6))
    role_counts = Counter(
        trait for champ in champions for trait in champ.get("strategy", {}).get("traits", [])
    )
    top_roles = ", ".join(human_trait(name) for name, _ in role_counts.most_common(6))
    portraits = "".join(
        f'<img src="{portrait_data_uri(champ)}" alt="{esc(champ["name"])}">'
        for champ in champions[:16]
    )
    return f"""
<header class="cover" id="top">
  <div class="cover-copy">
    <p class="kicker">Teamfight Manager 2 Pokemon MOBA Mod</p>
    <h1>Champion Field Guide</h1>
    <p class="lede">A draft-ready reference for every Pokemon in the roster: types, roles, stats, weaknesses, passives, basic attacks, skills, ultimates, status effects, and the type chart.</p>
    <div class="cover-stats">
      <span><b>{len(champions)}</b> champions</span>
      <span><b>18</b> types</span>
      <span><b>5</b> moves per entry</span>
    </div>
    <p class="cover-note">Top type coverage: {esc(top_types)}. Common draft roles: {esc(top_roles)}.</p>
  </div>
  <div class="cover-grid" aria-label="Champion portraits">{portraits}</div>
</header>
"""


def render_toc() -> str:
    return """
<nav class="toc" aria-label="Guide contents">
  <h2>Contents</h2>
  <a href="#quick-reference">Quick Draft Reference</a>
  <a href="#roster-index">Roster Index</a>
  <a href="#pokemon-roster">Pokemon Roster</a>
  <a href="#status-effects">DOTs and Status Effects</a>
  <a href="#type-chart">Type Effectiveness Chart</a>
  <a href="#strategy-traits">Strategy Trait Glossary</a>
</nav>
"""


def render_quick_reference(champions: list[dict]) -> str:
    by_position: dict[str, list[str]] = defaultdict(list)
    by_type: dict[str, list[str]] = defaultdict(list)
    for champ in champions:
        for position in split_positions(champ["positions"]):
            by_position[position].append(champ["name"])
        for type_name in champ["types"]:
            by_type[type_name].append(champ["name"])

    positions_html = "".join(
        f"<tr><th>{esc(position)}</th><td>{', '.join(link_champion(name) for name in names)}</td></tr>"
        for position, names in sorted(by_position.items())
    )
    types_html = "".join(
        f"<tr><th>{type_badge(type_name)}</th><td>{', '.join(link_champion(name) for name in sorted(names))}</td></tr>"
        for type_name, names in sorted(by_type.items())
    )

    return f"""
<section class="guide-section" id="quick-reference">
  <div class="section-heading">
    <p class="kicker">Draft setup</p>
    <h2>Quick Draft Reference</h2>
    <p>Use this section when you are scanning for a position, type slot, or counter profile before a pick or ban.</p>
  </div>
  <div class="draft-warning">
    <strong>Hard incompatibility:</strong> do not draft Clawitzer and Comfey together. Both champions attach to allies, and the draft scorer applies a heavy penalty to prevent the AI from pairing them.
  </div>
  <div class="two-column">
    <div class="panel">
      <h3>By Position</h3>
      <table class="reference-table">{positions_html}</table>
    </div>
    <div class="panel">
      <h3>By Type</h3>
      <table class="reference-table">{types_html}</table>
    </div>
  </div>
</section>
"""


def render_roster_index(champions: list[dict]) -> str:
    rows = []
    for champ in champions:
        traits = champ.get("strategy", {}).get("traits", [])
        key_traits = ", ".join(human_trait(name) for name in traits[:5])
        rows.append(
            "<tr>"
            f"<td>{link_champion(champ['name'])}</td>"
            f"<td>{''.join(type_badge(name) for name in champ['types'])}</td>"
            f"<td>{esc(champ['positions'])}</td>"
            f"<td>{esc(key_traits)}</td>"
            f"<td>{esc(', '.join(champ['matchups']['weak_to']) or 'None')}</td>"
            "</tr>"
        )
    return f"""
<section class="guide-section" id="roster-index">
  <div class="section-heading">
    <p class="kicker">Ctrl+F friendly</p>
    <h2>Roster Index</h2>
    <p>Every champion appears here and in the full Pokedex entries below.</p>
  </div>
  <table class="roster-table">
    <thead><tr><th>Pokemon</th><th>Types</th><th>Position</th><th>Primary traits</th><th>Weak to</th></tr></thead>
    <tbody>{''.join(rows)}</tbody>
  </table>
</section>
"""


def render_champion_sections(champions: list[dict]) -> str:
    entries = "\n".join(render_champion(champ) for champ in champions)
    return f"""
<main class="guide-section roster" id="pokemon-roster">
  <div class="section-heading">
    <p class="kicker">Pokedex</p>
    <h2>Pokemon Roster</h2>
    <p>Each entry uses the mod's exported data: basic attack, Skill 1, Skill 2, ultimate, passive, stats, strategy tags, and type matchups.</p>
  </div>
  {entries}
</main>
"""


def render_champion(champ: dict) -> str:
    strategy = champ.get("strategy", {})
    traits = strategy.get("traits", [])
    threatens = strategy.get("threatens", [])
    vulnerable = strategy.get("vulnerable_to", [])
    weak = champ["matchups"]["weak_to"]
    resists = champ["matchups"]["resists"]
    moves = sorted(champ["moves"], key=move_sort_key)
    notes = SPECIAL_NOTES.get(champ["id"], [])
    return f"""
<article class="dex-entry" id="{anchor(champ['name'])}">
  <div class="dex-header">
    <img class="portrait" src="{portrait_data_uri(champ)}" alt="{esc(champ['name'])}">
    <div>
      <h3>{esc(champ['name'])}</h3>
      <div class="badges">{''.join(type_badge(name) for name in champ['types'])}<span class="badge neutral">{esc(champ['positions'])}</span></div>
    </div>
  </div>
  <div class="dex-grid">
    <section class="info-block">
      <h4>Draft Identity</h4>
      <dl class="identity">
        <dt>Category</dt><dd>{esc(human_trait(champ['category']))}</dd>
        <dt>Tags</dt><dd>{chip_list([human_trait(tag) for tag in champ['tags']])}</dd>
        <dt>Traits</dt><dd>{chip_list([human_trait(tag) for tag in traits])}</dd>
        <dt>Threatens</dt><dd>{chip_list([human_trait(tag) for tag in threatens])}</dd>
        <dt>Vulnerable To</dt><dd>{chip_list([human_trait(tag) for tag in vulnerable])}</dd>
      </dl>
    </section>
    <section class="info-block">
      <h4>Type Matchups</h4>
      <div class="matchup">
        <strong>Weak to</strong>
        <div>{chip_list(weak, "danger")}</div>
      </div>
      <div class="matchup">
        <strong>Resists</strong>
        <div>{chip_list(resists, "safe")}</div>
      </div>
      {render_special_notes(notes)}
    </section>
    <section class="info-block stats-block">
      <h4>Stats</h4>
      {render_stat_table(champ['stats'])}
    </section>
  </div>
  <section class="moves">
    <h4>Moveset</h4>
    <div class="move-grid">{''.join(render_move(move) for move in moves)}</div>
  </section>
</article>
"""


def render_move(move: dict) -> str:
    cooldown = cooldown_label(move)
    return f"""
<div class="move-card">
  <div class="move-top">
    <span class="move-role">{esc(move['role'])}</span>
    <span class="cooldown">{esc(cooldown)}</span>
  </div>
  <h5>{esc(clean_text(move['name']))}</h5>
  <p>{esc(clean_text(move['description']))}</p>
</div>
"""


def cooldown_label(move: dict) -> str:
    ticks = move["cooldown"]
    if ticks is None:
        return "No cooldown"
    seconds = ticks / 60
    value = f"{seconds:.1f}s" if ticks % 60 else f"{int(seconds)}s"
    if move["role"] == "Basic":
        return f"{value} attack timer"
    return f"{value} cooldown"


def render_stat_table(stats: dict) -> str:
    rows = []
    base = stats["base"]
    growth = stats["growth"]
    level12 = stats["level12"]
    for key, label in STAT_LABELS:
        rows.append(
            "<tr>"
            f"<th>{esc(label)}</th>"
            f"<td>{base.get(key, 0)}</td>"
            f"<td>{growth.get(key, 0)}</td>"
            f"<td>{level12.get(key, 0)}</td>"
            "</tr>"
        )
    return (
        '<table class="stat-table">'
        "<thead><tr><th>Stat</th><th>Base</th><th>Growth</th><th>Lv.12</th></tr></thead>"
        f"<tbody>{''.join(rows)}</tbody></table>"
    )


def render_status_section() -> str:
    rows = "".join(
        f"<tr><th>{esc(name)}</th><td>{esc(description)}</td></tr>"
        for name, description in STATUS_GLOSSARY
    )
    return f"""
<section class="guide-section" id="status-effects">
  <div class="section-heading">
    <p class="kicker">Rules reference</p>
    <h2>DOTs and Status Effects</h2>
    <p>Move entries give exact numbers. This glossary explains what each recurring mechanic is for when reading drafts or comparing kits.</p>
  </div>
  <table class="glossary-table">{rows}</table>
</section>
"""


def render_type_chart(type_chart: list[dict]) -> str:
    types = list(TYPE_COLORS)
    lookup = {(cell["attack"], cell["defense"]): cell for cell in type_chart}
    header = "".join(
        f'<th><span class="chart-type-label" title="{esc(type_name)}">{esc(TYPE_ABBR[type_name])}</span></th>'
        for type_name in types
    )
    rows = []
    for attack in types:
        cells = []
        for defense in types:
            cell = lookup[(attack, defense)]
            mult = cell["num"] / cell["den"]
            label = multiplier_label(cell)
            if mult > 1:
                klass = "strong"
            elif mult < 1:
                klass = "weak"
            else:
                klass = "neutral-cell"
            cells.append(f'<td class="{klass}">{label}</td>')
        rows.append(f"<tr><th>{type_badge(attack)}</th>{''.join(cells)}</tr>")
    return f"""
<section class="guide-section chart-section" id="type-chart">
  <div class="section-heading">
    <p class="kicker">Damage rules</p>
    <h2>Type Effectiveness Chart</h2>
    <p>Rows are attack types. Columns are defender types. Dual-type champions combine these modifiers in their champion entry weakness and resistance chips.</p>
  </div>
  <div class="chart-wrap">
    <table class="type-chart">
      <thead><tr><th>Atk \\ Def</th>{header}</tr></thead>
      <tbody>{''.join(rows)}</tbody>
    </table>
  </div>
</section>
"""


def render_trait_section() -> str:
    rows = "".join(
        f"<tr><th>{esc(human_trait(name))}</th><td>{esc(note)}</td></tr>"
        for name, note in TRAIT_NOTES
    )
    return f"""
<section class="guide-section" id="strategy-traits">
  <div class="section-heading">
    <p class="kicker">Draft language</p>
    <h2>Strategy Trait Glossary</h2>
    <p>These labels come from the mod's strategy tables and are used for position logic, counter logic, and guide summaries.</p>
  </div>
  <table class="glossary-table">{rows}</table>
  <p class="footer-note"><a href="#top">Back to top</a></p>
</section>
"""


def split_positions(value: str) -> list[str]:
    return [part.strip() for part in value.split("/") if part.strip()]


def move_sort_key(move: dict) -> tuple[int, str]:
    order = {"Basic": 0, "Skill 1": 1, "Skill 2": 2, "Ult": 3, "Passive": 4}
    return (order.get(move["role"], 99), move["role"])


def clean_text(value: str) -> str:
    value = value.replace("<>", "")
    value = re.sub(r"<i#[^>]*ad_0>", "AD ", value)
    value = re.sub(r"<i#[^>]*ap_0>", "AP ", value)
    value = re.sub(r"<i#[^>]*hp_0>", "HP ", value)
    value = re.sub(r"<i#[^>]*attack_speed_0>", "Attack Speed ", value)
    value = re.sub(r"<i#[^>]*speed_0>", "Move Speed ", value)
    value = re.sub(r"<i#[^>]*armor_0>", "Armor ", value)
    value = re.sub(r"<i#[^>]*magic_resistance_0>", "Magic Resist ", value)
    value = re.sub(r"<#[0-9a-fA-F]{6,8}>", "", value)
    value = re.sub(r"\{(?:END|PHYSICAL|MAGIC|CONTROL|BUFF|HEAL|SHIELD)\}", "", value)
    value = re.sub(r"<[^>]+>", "", value)
    return re.sub(r"\s+", " ", value).strip()


def portrait_data_uri(champ: dict) -> str:
    candidates = [
        ICON_DIR / f"{champ['short_id']}.png",
        ICON_DIR / f"{champ['id'].replace('pokemon_moba_', '')}.png",
    ]
    for path in candidates:
        if path.exists():
            data = base64.b64encode(path.read_bytes()).decode("ascii")
            return f"data:image/png;base64,{data}"
    initials = "".join(part[:1] for part in champ["name"].split()[:2]).upper()
    svg = (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 96 96">'
        '<rect width="96" height="96" fill="#1f2230"/>'
        f'<text x="48" y="56" text-anchor="middle" font-size="28" fill="#e9edf5" font-family="Arial">{html.escape(initials)}</text>'
        "</svg>"
    )
    data = base64.b64encode(svg.encode("utf-8")).decode("ascii")
    return f"data:image/svg+xml;base64,{data}"


def type_badge(type_name_value: str) -> str:
    color = TYPE_COLORS.get(type_name_value, "#9ca7b7")
    text_color = "#171922" if type_name_value in {"Electric", "Fairy", "Ice", "Normal", "Steel"} else "#ffffff"
    return (
        f'<span class="type-badge" style="--type-color:{color};--type-text:{text_color}">'
        f"{esc(type_name_value)}</span>"
    )


def chip_list(values: list[str], mode: str = "") -> str:
    if not values:
        return '<span class="chip muted">None listed</span>'
    klass = f"chip {mode}".strip()
    return "".join(f'<span class="{klass}">{esc(value)}</span>' for value in values)


def render_special_notes(notes: list[str]) -> str:
    if not notes:
        return ""
    items = "".join(f"<li>{esc(note)}</li>" for note in notes)
    return f'<div class="special-notes"><strong>Special notes</strong><ul>{items}</ul></div>'


def link_champion(name: str) -> str:
    return f'<a href="#{anchor(name)}">{esc(name)}</a>'


def anchor(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


def human_trait(value: str) -> str:
    replacements = {
        "Ad": "AD",
        "Ap": "AP",
        "Cc": "CC",
        "Dot": "DOT",
        "Hp": "HP",
    }
    words = re.sub(r"(?<!^)(?=[A-Z])", " ", value).split()
    return " ".join(replacements.get(word, word) for word in words)


def multiplier_label(cell: dict) -> str:
    if cell["num"] == cell["den"]:
        return "1.0x"
    value = cell["num"] / cell["den"]
    return f"{value:.2g}x"


def esc(value: object) -> str:
    return html.escape(str(value), quote=True)


def css() -> str:
    return r"""
:root {
  color-scheme: light;
  --ink: #18202b;
  --muted: #5b6575;
  --page: #f3f5f1;
  --paper: #ffffff;
  --line: #cfd7df;
  --panel: #eef2f6;
  --accent: #1b7f74;
  --accent-2: #d3564a;
  --gold: #d4a82e;
}
* { box-sizing: border-box; }
html { scroll-behavior: smooth; }
body {
  margin: 0;
  background: var(--page);
  color: var(--ink);
  font-family: "Segoe UI", Arial, sans-serif;
  font-size: 15px;
  line-height: 1.48;
}
a { color: #0b6f79; text-decoration: none; font-weight: 700; }
a:hover { text-decoration: underline; }
.cover {
  min-height: 92vh;
  display: grid;
  grid-template-columns: minmax(320px, 1.05fr) minmax(280px, .95fr);
  gap: 36px;
  align-items: center;
  padding: 52px min(6vw, 76px) 40px;
  background:
    linear-gradient(135deg, rgba(20, 29, 43, .96), rgba(29, 52, 59, .95)),
    linear-gradient(90deg, #1b7f74, #d4a82e);
  color: #f9fbff;
}
.cover h1 {
  margin: 8px 0 18px;
  font-size: clamp(48px, 8vw, 104px);
  line-height: .92;
  letter-spacing: 0;
}
.kicker {
  margin: 0 0 8px;
  color: var(--gold);
  font-size: 12px;
  font-weight: 900;
  letter-spacing: .12em;
  text-transform: uppercase;
}
.lede {
  max-width: 760px;
  color: #e2e8ef;
  font-size: 20px;
}
.cover-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin: 28px 0 18px;
}
.cover-stats span {
  border: 1px solid rgba(255,255,255,.26);
  background: rgba(255,255,255,.08);
  border-radius: 8px;
  padding: 12px 16px;
}
.cover-stats b {
  display: block;
  font-size: 26px;
  line-height: 1;
}
.cover-note { color: #ccd7df; max-width: 760px; }
.cover-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(52px, 1fr));
  gap: 12px;
}
.cover-grid img {
  width: 100%;
  aspect-ratio: 1;
  object-fit: contain;
  background: rgba(255,255,255,.08);
  border: 1px solid rgba(255,255,255,.16);
  border-radius: 8px;
  padding: 8px;
}
.toc, .guide-section {
  max-width: 1320px;
  margin: 0 auto;
  padding: 44px min(4vw, 42px);
}
.toc {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
  background: var(--paper);
  border-bottom: 4px solid var(--accent);
}
.toc h2 {
  flex-basis: 100%;
  margin: 0 0 6px;
  font-size: 22px;
}
.toc a {
  display: inline-flex;
  border: 1px solid var(--line);
  border-radius: 8px;
  padding: 8px 12px;
  background: #f8fafc;
}
.section-heading {
  margin-bottom: 22px;
  border-left: 6px solid var(--accent);
  padding-left: 16px;
}
.section-heading h2 {
  margin: 0 0 6px;
  font-size: 34px;
  line-height: 1.05;
}
.section-heading p:last-child {
  max-width: 860px;
  margin: 0;
  color: var(--muted);
}
.draft-warning {
  margin: 0 0 18px;
  padding: 12px 14px;
  border: 1px solid #e5ad6d;
  border-left: 6px solid #d28b25;
  border-radius: 8px;
  background: #fff8eb;
  color: #4b3514;
}
.draft-warning strong {
  color: #87510a;
}
.two-column {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px;
}
.panel, .dex-entry {
  background: var(--paper);
  border: 1px solid var(--line);
  border-radius: 8px;
  box-shadow: 0 12px 26px rgba(20, 28, 38, .08);
}
.panel {
  padding: 18px;
}
.panel h3 {
  margin: 0 0 12px;
  font-size: 22px;
}
table {
  width: 100%;
  border-collapse: collapse;
}
th, td {
  border-bottom: 1px solid var(--line);
  padding: 9px 10px;
  vertical-align: top;
}
th {
  text-align: left;
  font-weight: 850;
}
.reference-table th { width: 145px; }
.roster-table, .glossary-table, .type-chart {
  background: var(--paper);
  border: 1px solid var(--line);
  border-radius: 8px;
  overflow: hidden;
}
.roster-table thead th, .type-chart thead th, .stat-table thead th {
  background: #202735;
  color: #f7f9fc;
}
.type-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 72px;
  margin: 2px 4px 2px 0;
  padding: 4px 8px;
  border-radius: 999px;
  background: var(--type-color);
  color: var(--type-text);
  font-size: 12px;
  font-weight: 900;
  text-transform: uppercase;
}
.badge, .chip {
  display: inline-flex;
  margin: 2px 5px 2px 0;
  padding: 4px 8px;
  border-radius: 999px;
  border: 1px solid #c8d3dd;
  background: #f5f7fa;
  color: #263343;
  font-size: 12px;
  font-weight: 750;
}
.badge.neutral {
  background: #273142;
  color: #f8fafc;
  border-color: #273142;
}
.chip.danger {
  background: #fff0ed;
  border-color: #f1b5ad;
  color: #9d3328;
}
.chip.safe {
  background: #eaf7f1;
  border-color: #a9d8c2;
  color: #17614c;
}
.chip.muted {
  background: #f0f2f5;
  color: #697281;
}
.roster {
  max-width: 1380px;
}
.dex-entry {
  margin: 0 0 24px;
  overflow: hidden;
  break-inside: avoid;
}
.dex-header {
  display: grid;
  grid-template-columns: 92px 1fr;
  gap: 16px;
  align-items: center;
  padding: 18px;
  background:
    linear-gradient(90deg, rgba(27,127,116,.18), rgba(212,168,46,.10)),
    #f9fbfc;
  border-bottom: 1px solid var(--line);
}
.portrait {
  width: 92px;
  height: 92px;
  object-fit: contain;
  border-radius: 8px;
  background: #202735;
  border: 1px solid #3a4658;
  padding: 8px;
}
.dex-header h3 {
  margin: 0 0 8px;
  font-size: 30px;
  line-height: 1.1;
}
.dex-grid {
  display: grid;
  grid-template-columns: 1.1fr .85fr 1.05fr;
  gap: 16px;
  padding: 16px;
}
.info-block h4, .moves h4 {
  margin: 0 0 10px;
  font-size: 18px;
  color: #202735;
}
.identity {
  display: grid;
  grid-template-columns: 110px 1fr;
  gap: 7px 10px;
  margin: 0;
}
.identity dt {
  color: var(--muted);
  font-weight: 850;
}
.identity dd { margin: 0; }
.matchup {
  margin-bottom: 12px;
}
.matchup strong {
  display: block;
  margin-bottom: 4px;
  color: var(--muted);
  font-size: 12px;
  text-transform: uppercase;
}
.special-notes {
  margin-top: 14px;
  padding: 10px 12px;
  border: 1px solid #f0c08f;
  border-radius: 8px;
  background: #fff8eb;
}
.special-notes strong {
  display: block;
  margin-bottom: 4px;
  color: #87510a;
  font-size: 12px;
  text-transform: uppercase;
}
.special-notes ul {
  margin: 0;
  padding-left: 18px;
}
.special-notes li {
  margin: 3px 0;
}
.stat-table th, .stat-table td {
  padding: 5px 8px;
  font-size: 13px;
}
.stat-table tbody th {
  width: 112px;
}
.moves {
  padding: 0 16px 18px;
  break-inside: avoid;
}
.moves h4 {
  break-after: avoid;
}
.move-grid {
  display: grid;
  grid-template-columns: repeat(5, minmax(0, 1fr));
  gap: 12px;
  break-before: avoid;
}
.move-card {
  min-height: 190px;
  border: 1px solid #cbd4df;
  border-radius: 8px;
  background: #fbfcfe;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 7px;
}
.move-top {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  align-items: center;
}
.move-role {
  color: #ffffff;
  background: #202735;
  border-radius: 999px;
  padding: 3px 8px;
  font-size: 11px;
  font-weight: 900;
  text-transform: uppercase;
}
.cooldown {
  color: var(--muted);
  font-size: 11px;
  font-weight: 800;
}
.move-card h5 {
  margin: 0;
  font-size: 16px;
  line-height: 1.16;
}
.move-card p {
  margin: 0;
  color: #303b4d;
  font-size: 13px;
}
.chart-wrap {
  max-width: 100%;
  overflow-x: auto;
  background: var(--paper);
  border: 1px solid var(--line);
  border-radius: 8px;
}
.type-chart {
  min-width: 1160px;
  border: 0;
}
.chart-type-label {
  display: inline-block;
  min-width: 34px;
}
.type-chart th, .type-chart td {
  text-align: center;
  padding: 7px 6px;
  font-size: 12px;
}
.type-chart tbody th {
  text-align: left;
  background: #f8fafc;
  position: sticky;
  left: 0;
}
.strong { background: #e3f5ec; color: #116344; font-weight: 900; }
.weak { background: #ffe9e5; color: #983425; font-weight: 900; }
.neutral-cell { background: #f6f7f9; color: #667080; }
.footer-note {
  margin: 24px 0 0;
}
@media (max-width: 1120px) {
  .cover, .two-column, .dex-grid { grid-template-columns: 1fr; }
  .move-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
}
@media (max-width: 680px) {
  .cover { padding: 32px 18px; }
  .cover h1 { font-size: 44px; }
  .cover-grid { grid-template-columns: repeat(4, 1fr); gap: 6px; }
  .toc, .guide-section { padding: 30px 14px; }
  .dex-header { grid-template-columns: 72px 1fr; }
  .portrait { width: 72px; height: 72px; }
  .move-grid { grid-template-columns: 1fr; }
  .identity { grid-template-columns: 1fr; }
}
@media print {
  @page {
    size: Letter portrait;
    margin: .45in .38in;
  }
  @page typechart {
    size: Letter landscape;
    margin: .32in;
  }
  body { background: #ffffff; font-size: 12px; }
  .cover { min-height: auto; page-break-after: always; }
  .toc, .guide-section { max-width: none; padding: 22px 0; }
  .panel, .dex-entry, .roster-table, .glossary-table, .chart-wrap {
    box-shadow: none;
  }
  .dex-entry {
    break-before: page;
    page-break-before: always;
    break-inside: auto;
    page-break-inside: auto;
  }
  .dex-header, .dex-grid, .moves, .move-grid {
    break-inside: avoid;
    page-break-inside: avoid;
  }
  .dex-header, .dex-grid {
    break-after: avoid;
    page-break-after: avoid;
  }
  .moves {
    break-before: page;
    page-break-before: always;
  }
  .moves h4 {
    break-after: avoid;
    page-break-after: avoid;
  }
  .move-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .move-card { min-height: 0; }
  .chart-section {
    page: typechart;
    break-before: page;
    break-after: page;
  }
  .chart-wrap {
    overflow: visible;
  }
  .type-chart {
    width: 100%;
    min-width: 0;
    table-layout: fixed;
  }
  .type-chart th, .type-chart td {
    padding: 3px 2px;
    font-size: 8px;
  }
  .type-chart .type-badge {
    min-width: 0;
    max-width: 54px;
    padding: 2px 3px;
    margin: 0;
    font-size: 7px;
  }
  .chart-type-label {
    min-width: 0;
    font-size: 8px;
  }
  a { color: inherit; text-decoration: none; }
}
"""


if __name__ == "__main__":
    main()
