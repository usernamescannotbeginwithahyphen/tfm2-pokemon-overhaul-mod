use crate::pokemon_positions::{
    info_for_champion_id, position_icon_source, PokemonPositionInfo, POKEMON_POSITIONS,
};
use engine_ui::runner::{ButtonRunner, ImageRunner, LabelRunner, SvgRunner};
use mod_api::{GameUI, Node};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

const UI_DUMP_DIR: &str = r"C:\Users\james\Documents\TFT Pokemon Mod\logs";
const UI_DUMP_LATEST: &str = r"C:\Users\james\Documents\TFT Pokemon Mod\logs\ui-tree-latest.txt";
const UI_DUMP_FLAG: &str = r"C:\Users\james\Documents\TFT Pokemon Mod\logs\enable-ui-dump.txt";
const MAX_UI_DUMPS: usize = 120;

static LAST_UI_HASH: AtomicU64 = AtomicU64::new(0);
static UI_DUMP_COUNT: AtomicUsize = AtomicUsize::new(0);

const POKEMON_NAMES: [&str; 101] = [
    "Pikachu",
    "Charizard",
    "Blastoise",
    "Feraligatr",
    "Emboar",
    "Blaziken",
    "Greninja",
    "Decidueye",
    "Inteleon",
    "Skeledirge",
    "Porygon-Z",
    "Blissey",
    "Kleavor",
    "Scizor",
    "Ursaluna",
    "Sawk & Throh",
    "Hitmonchan",
    "Hitmonlee",
    "Hitmontop",
    "Kilowattrel",
    "Beeheeyem",
    "Gyarados",
    "Noivern",
    "Mantine",
    "Cryogonal",
    "Vanilluxe",
    "Skarmory",
    "Houndoom",
    "Arbok",
    "Clawitzer",
    "Octillery",
    "Pyukumuku",
    "Banette",
    "Kricketune",
    "Ambipom",
    "Gallade",
    "Audino",
    "Pangoro",
    "Passimian",
    "Oranguru",
    "Dragalge",
    "Heliolisk",
    "Turtonator",
    "Ribombee",
    "Drampa",
    "Kommo-o",
    "Thievul",
    "Archaludon",
    "Appletun",
    "Goodra",
    "Dedenne",
    "Hawlucha",
    "Bouffalant",
    "Starmie",
    "Drednaw",
    "Orbeetle",
    "Coalossal",
    "Magmortar",
    "Grapploct",
    "Sirfetch'd",
    "Arboliva",
    "Armarouge",
    "Ceruledge",
    "Gholdengo",
    "Frosmoth",
    "Shedinja",
    "Ludicolo",
    "Kingdra",
    "Delibird",
    "Cloyster",
    "Electrode",
    "Snorlax",
    "Zeraora",
    "Rillaboom",
    "Dragapult",
    "Shiftry",
    "Sigilyph",
    "Weavile",
    "Swanna",
    "Marowak",
    "Garganacl",
    "Ampharos",
    "Xatu",
    "Quaquaval",
    "Arcanine",
    "MissingNo.",
    "Yanmega",
    "Wishiwashi",
    "Comfey",
    "Smeargle",
    "Torterra",
    "Venusaur",
    "Eevee",
    "Jolteon",
    "Flareon",
    "Vaporeon",
    "Leafeon",
    "Glaceon",
    "Umbreon",
    "Espeon",
    "Sylveon",
];

pub fn patch_position_ui(ui: &mut GameUI) {
    patch_position_nodes(&mut ui.root);
    patch_position_tooltips(&mut ui.root);

    if fs::metadata(UI_DUMP_FLAG).is_ok() {
        inspect_position_ui(ui);
    }
}

fn patch_position_nodes(node: &mut Node) {
    if let Some(info) = info_for_champion_id(&node.id) {
        patch_pokemon_card(node, info);
    }

    for child in &mut node.child {
        patch_position_nodes(child);
    }
}

fn patch_pokemon_card(card: &mut Node, info: &PokemonPositionInfo) {
    patch_position_slot(card, &["pos1", "position1"], info.positions.get(0).copied());
    patch_position_slot(card, &["pos2", "position2"], info.positions.get(1).copied());
    patch_position_slot(card, &["pos3", "position3"], info.positions.get(2).copied());
    patch_position_label(card, info.label);
}

fn patch_position_tooltips(node: &mut Node) {
    if node.visible && is_tooltip_or_popover_node(node, node_label_text(node).as_deref()) {
        if let Some(info) = pokemon_info_in_subtree(node) {
            patch_position_tooltip(node, info);
        }
    }

    for child in &mut node.child {
        patch_position_tooltips(child);
    }
}

fn patch_position_tooltip(tooltip: &mut Node, info: &PokemonPositionInfo) {
    patch_position_label(tooltip, info.label);
    patch_position_slot(
        tooltip,
        &["position_icon", "main_position_icon", "pos_icon"],
        info.positions.first().copied(),
    );
    patch_position_slot(
        tooltip,
        &["pos1", "position1"],
        info.positions.get(0).copied(),
    );
    patch_position_slot(
        tooltip,
        &["pos2", "position2"],
        info.positions.get(1).copied(),
    );
    patch_position_slot(
        tooltip,
        &["pos3", "position3"],
        info.positions.get(2).copied(),
    );
}

fn patch_position_slot(card: &mut Node, ids: &[&str], position: Option<usize>) {
    let Some(slot) = find_descendant_by_id_mut(card, ids) else {
        return;
    };

    if let Some(position) = position {
        slot.visible = true;
        if let Some(source) = position_icon_source(position) {
            patch_image_source(slot, source);
        }
    } else {
        slot.visible = false;
    }
}

fn patch_position_label(card: &mut Node, label: &str) {
    if let Some(node) = find_descendant_by_id_mut(
        card,
        &[
            "position_name",
            "position",
            "main_position",
            "main_position_label",
        ],
    ) {
        patch_label_text(node, label);
    }
}

fn find_descendant_by_id_mut<'a>(node: &'a mut Node, ids: &[&str]) -> Option<&'a mut Node> {
    for child in &mut node.child {
        if ids.iter().any(|id| child.id == *id) {
            return Some(child);
        }
        if let Some(found) = find_descendant_by_id_mut(child, ids) {
            return Some(found);
        }
    }
    None
}

fn patch_image_source(node: &mut Node, source: &str) -> bool {
    let mut patched = false;

    if let Some(runner) = node.runner_as_mut::<ImageRunner>() {
        set_image_runner_source(runner, source);
        patched = true;
    }
    if let Some(runner) = node.runner_as_mut::<ButtonRunner>() {
        set_button_runner_source(runner, source);
        patched = true;
    }
    if let Some(runner) = node.runner_as_mut::<SvgRunner>() {
        set_svg_runner_source(runner, source);
        patched = true;
    }

    if !patched {
        let mut runner = ImageRunner::default();
        runner.ignore_event = true;
        set_image_runner_source(&mut runner, source);
        node.runner = Box::new(runner);
        patched = true;
    }

    for child in &mut node.child {
        patched |= patch_image_source(child, source);
    }

    patched
}

fn set_image_runner_source(runner: &mut ImageRunner, source: &str) {
    runner.style.normal.source = source.to_string();
    runner.style.hover.source = source.to_string();
    runner.style.active.source = source.to_string();
    runner.style.disabled.source = source.to_string();
}

fn set_button_runner_source(runner: &mut ButtonRunner, source: &str) {
    runner.style.normal.source = source.to_string();
    runner.style.hover.source = source.to_string();
    runner.style.active.source = source.to_string();
    runner.style.disabled.source = source.to_string();
}

fn set_svg_runner_source(runner: &mut SvgRunner, source: &str) {
    runner.style.normal.source = source.to_string();
    runner.style.hover.source = source.to_string();
    runner.style.active.source = source.to_string();
    runner.style.disabled.source = source.to_string();
}

fn patch_label_text(node: &mut Node, text: &str) -> bool {
    let mut patched = false;
    if let Some(runner) = node.runner_as_mut::<LabelRunner>() {
        runner.clear_bind();
        runner.text = text.to_string();
        patched = true;
    }

    for child in &mut node.child {
        patched |= patch_label_text(child, text);
    }

    patched
}

fn pokemon_info_in_subtree(node: &Node) -> Option<&'static PokemonPositionInfo> {
    if let Some(info) = info_for_node_id(&node.id) {
        return Some(info);
    }

    if let Some(text) = node_label_text(node) {
        if let Some(info) = info_for_display_text(&text) {
            return Some(info);
        }
    }

    if let Some(source) = node_image_source(node) {
        if let Some(info) = info_for_image_source(&source) {
            return Some(info);
        }
    }

    for child in &node.child {
        if let Some(info) = pokemon_info_in_subtree(child) {
            return Some(info);
        }
    }

    None
}

fn info_for_node_id(node_id: &str) -> Option<&'static PokemonPositionInfo> {
    info_for_champion_id(node_id).or_else(|| {
        POKEMON_POSITIONS
            .iter()
            .find(|info| !node_id.is_empty() && node_id.contains(info.id))
    })
}

fn info_for_display_text(text: &str) -> Option<&'static PokemonPositionInfo> {
    POKEMON_NAMES
        .iter()
        .position(|name| text.contains(name))
        .and_then(|index| POKEMON_POSITIONS.get(index))
}

fn info_for_image_source(source: &str) -> Option<&'static PokemonPositionInfo> {
    let source = source.to_ascii_lowercase();
    POKEMON_POSITIONS.iter().find(|info| {
        let id = info.id.to_ascii_lowercase();
        let short_id = id.strip_prefix("pokemon_moba_").unwrap_or(&id);
        source.contains(&id) || source.contains(short_id)
    })
}

fn node_label_text(node: &Node) -> Option<String> {
    node.runner_as::<LabelRunner>()
        .map(|runner| runner.text.clone())
}

pub fn inspect_position_ui(ui: &GameUI) {
    let mut lines = Vec::new();
    let mut index = Vec::new();
    let mut tooltip_index = Vec::new();
    let mut relevant = false;
    collect_node(
        &ui.root,
        0,
        "root".to_string(),
        &mut lines,
        &mut index,
        &mut tooltip_index,
        &mut relevant,
    );
    if !relevant {
        return;
    }

    let text = format!(
        "Relevant node index:\n{}\n\nFloating tooltip/popover candidates:\n{}\n\nFull UI tree:\n{}",
        if index.is_empty() {
            "(none)".to_string()
        } else {
            index.join("\n")
        },
        if tooltip_index.is_empty() {
            "(none)".to_string()
        } else {
            tooltip_index.join("\n")
        },
        lines.join("\n")
    );
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let hash = hasher.finish();
    if LAST_UI_HASH.swap(hash, Ordering::Relaxed) == hash {
        return;
    }

    let dump_index = UI_DUMP_COUNT.fetch_add(1, Ordering::Relaxed);
    if dump_index >= MAX_UI_DUMPS {
        return;
    }

    let _ = fs::create_dir_all(UI_DUMP_DIR);
    let header = format!(
        "Pokemon MOBA UI tree dump #{dump_index}\nRelevant because Pokemon/position/tooltip UI nodes are visible.\nHover position icons during pick-ban and position-swap screens to capture transient floating tooltip nodes.\n\n"
    );
    let output = format!("{header}{text}\n");
    let _ = fs::write(UI_DUMP_LATEST, &output);
    let numbered_path = format!(r"{UI_DUMP_DIR}\ui-tree-{dump_index:02}.txt");
    let _ = fs::write(numbered_path, output);
}

fn collect_node(
    node: &Node,
    depth: usize,
    path: String,
    lines: &mut Vec<String>,
    relevant_index: &mut Vec<String>,
    tooltip_index: &mut Vec<String>,
    relevant: &mut bool,
) {
    let label = node
        .runner_as::<LabelRunner>()
        .map(|runner| format!("{:?}", runner.text));
    let image = node_image_source(node);
    let runner_kind = node_runner_kind(node);

    let is_relevant = is_relevant_node(node, label.as_deref(), image.as_deref());
    let is_tooltip = is_tooltip_or_popover_node(node, label.as_deref());
    if is_relevant || is_tooltip {
        *relevant = true;
        relevant_index.push(format!(
            "{path} id={:?} rect={:?} visible={} focus={:?}{}{}{}",
            node.id,
            node.rect,
            node.visible,
            node.focus,
            runner_kind
                .as_ref()
                .map(|value| format!(" runner={value}"))
                .unwrap_or_default(),
            label
                .as_ref()
                .map(|value| format!(" label={value}"))
                .unwrap_or_default(),
            image
                .as_ref()
                .map(|value| format!(" image={value:?}"))
                .unwrap_or_default()
        ));
    }
    if is_tooltip {
        tooltip_index.push(format!(
            "{path} id={:?} rect={:?} visible={} focus={:?} labels=[{}] images=[{}]",
            node.id,
            node.rect,
            node.visible,
            node.focus,
            subtree_labels(node).join(" | "),
            subtree_images(node).join(" | ")
        ));
    }

    let indent = "  ".repeat(depth);
    lines.push(format!(
        "{indent}{path} id={:?} visible={} disabled={} focus={:?} rect={:?} contents={:?} children={}{}{}{}",
        node.id,
        node.visible,
        node.disabled,
        node.focus,
        node.rect,
        node.contents_rect,
        node.child.len(),
        runner_kind
            .as_ref()
            .map(|value| format!(" runner={value}"))
            .unwrap_or_default(),
        label
            .as_ref()
            .map(|value| format!(" label={value}"))
            .unwrap_or_default(),
        image
            .as_ref()
            .map(|value| format!(" image={value:?}"))
            .unwrap_or_default()
    ));

    for (child_index, child) in node.child.iter().enumerate() {
        let child_path = if child.id.is_empty() {
            format!("{path}/{child_index}")
        } else {
            format!("{path}/{}:{child_index}", child.id)
        };
        collect_node(
            child,
            depth + 1,
            child_path,
            lines,
            relevant_index,
            tooltip_index,
            relevant,
        );
    }
}

fn is_relevant_node(node: &Node, label: Option<&str>, image: Option<&str>) -> bool {
    let id = node.id.as_str();
    let id_lc = id.to_ascii_lowercase();
    if id_lc.contains("pos")
        || id_lc.contains("position")
        || id_lc.contains("banpick")
        || id_lc.contains("champion")
        || id_lc.contains("lineup")
        || id_lc.contains("swap")
        || id_lc.contains("pick")
        || id_lc.contains("ban")
    {
        return true;
    }

    if let Some(image) = image {
        if image.contains("pokemon_moba") || image.contains("champions/pokemon") {
            return true;
        }
    }

    if let Some(label) = label {
        if POKEMON_NAMES.iter().any(|name| label.contains(name)) {
            return true;
        }
        if is_position_text(label) {
            return true;
        }
    }

    false
}

fn is_tooltip_or_popover_node(node: &Node, label: Option<&str>) -> bool {
    let id = node.id.to_ascii_lowercase();
    if id.contains("tooltip")
        || id.contains("popover")
        || id.contains("hover")
        || id.contains("floating")
        || id.contains("description")
        || id == "popup"
        || (id.starts_with("popup_") && id != "popup_layer")
        || id.contains("position_tooltip")
        || id.contains("pos_tooltip")
    {
        return true;
    }

    if let Some(label) = label {
        let label = label.to_ascii_lowercase();
        if label.contains("main position")
            || label.contains("main pos")
            || label.contains("played position")
            || label.contains("selected position")
        {
            return true;
        }
    }

    false
}

fn is_position_text(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("top")
        || text.contains("jungle")
        || text.contains("mid")
        || text.contains("bottom")
        || text.contains("support")
        || text.contains("main position")
        || text.contains("main pos")
}

fn subtree_labels(node: &Node) -> Vec<String> {
    let mut labels = Vec::new();
    collect_subtree_labels(node, &mut labels);
    labels
}

fn collect_subtree_labels(node: &Node, labels: &mut Vec<String>) {
    if let Some(label) = node
        .runner_as::<LabelRunner>()
        .map(|runner| format!("{:?}", runner.text))
    {
        labels.push(label);
    }
    for child in &node.child {
        collect_subtree_labels(child, labels);
    }
}

fn subtree_images(node: &Node) -> Vec<String> {
    let mut images = Vec::new();
    collect_subtree_images(node, &mut images);
    images
}

fn collect_subtree_images(node: &Node, images: &mut Vec<String>) {
    if let Some(image) = node_image_source(node) {
        images.push(image);
    }
    for child in &node.child {
        collect_subtree_images(child, images);
    }
}

fn node_image_source(node: &Node) -> Option<String> {
    if let Some(runner) = node.runner_as::<ImageRunner>() {
        return Some(runner.style.normal.source.clone());
    }
    if let Some(runner) = node.runner_as::<ButtonRunner>() {
        return Some(runner.style.normal.source.clone());
    }
    if let Some(runner) = node.runner_as::<SvgRunner>() {
        return Some(runner.style.normal.source.clone());
    }
    None
}

fn node_runner_kind(node: &Node) -> Option<&'static str> {
    if node.runner_as::<LabelRunner>().is_some() {
        return Some("LabelRunner");
    }
    if node.runner_as::<ImageRunner>().is_some() {
        return Some("ImageRunner");
    }
    if node.runner_as::<ButtonRunner>().is_some() {
        return Some("ButtonRunner");
    }
    if node.runner_as::<SvgRunner>().is_some() {
        return Some("SvgRunner");
    }
    None
}
