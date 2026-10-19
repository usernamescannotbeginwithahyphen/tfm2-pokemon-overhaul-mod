//! Read map geometry without changing it; freeze an unambiguous grid per simulation.
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, OnceLock};

use mod_api_stable::{EntityKindV1, GameModeKindV1, StableJsonDoc, StableMapCustomizer, StableSim};
use serde_json::Value;

const CELLS: usize = 30;
const CELL_SIZE: u64 = 32_000;
type Grid = Vec<Vec<i64>>;
type Anchor = (u32, usize, u64, u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BushMap {
    // Canonical row-major storage, independently of the engine JSON's orientation.
    cells: Vec<bool>,
}

impl BushMap {
    pub(crate) fn contains(&self, x: u64, y: u64) -> bool {
        let (x, y) = (x / CELL_SIZE, y / CELL_SIZE);
        x < CELLS as u64 && y < CELLS as u64 && self.cells[y as usize * CELLS + x as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CapturedMap {
    anchors: Vec<Anchor>,
    bushes: Result<Arc<BushMap>, String>,
}

#[derive(Default)]
struct Catalog {
    maps: Vec<CapturedMap>,
    unknown_layout: bool,
}

impl Catalog {
    fn capture(&mut self, document: &Value) -> Result<usize, String> {
        let anchors = match anchors(document) {
            Ok(anchors) => anchors,
            Err(error) => {
                self.unknown_layout = true;
                return Err(error);
            }
        };
        let bushes = parse_bushes(document).map(Arc::new);
        let result = bushes.as_ref().map(|map| map.cells.iter().filter(|value| **value).count())
            .map_err(Clone::clone);
        let map = CapturedMap { anchors, bushes };
        if !self.maps.contains(&map) {
            if map.bushes.is_err() {
                diagnostic(&format!("unrecognized geometry={}", serde_json::json!({
                    "bushes": document["bushes"], "regions": document["regions"],
                    "region_centers": document["region_centers"], "nexus_pos": document["nexus_pos"]
                })));
            }
            self.maps.push(map);
        }
        result
    }

    fn select(&self, live: &[Anchor]) -> Result<Arc<BushMap>, String> {
        if self.unknown_layout { return Err("a captured map had an unknown layout schema".into()); }
        let mut selected: Option<Arc<BushMap>> = None;
        for candidate in self.maps.iter().filter(|map| map.anchors == live) {
            let map = candidate.bushes.as_ref().map_err(Clone::clone)?;
            if selected.as_ref().is_some_and(|other| other.as_ref() != map.as_ref()) {
                return Err("multiple bush grids share this tower/nexus layout".into());
            }
            selected = Some(map.clone());
        }
        selected.ok_or_else(|| "no captured map matches the initial tower/nexus layout".into())
    }
}

static CATALOG: OnceLock<Mutex<Catalog>> = OnceLock::new();
static MATCH_MAP: crate::runtime_state::SimOnceLock<Mutex<Option<Arc<BushMap>>>> =
    crate::runtime_state::SimOnceLock::new();

pub struct TerrainReader;

impl StableMapCustomizer for TerrainReader {
    fn customize(&self, mode: Option<GameModeKindV1>, doc: &mut StableJsonDoc<'_>) {
        let result = doc.get_json("").ok_or_else(|| "map document unavailable".to_string())
            .and_then(|json| serde_json::from_str::<Value>(&json).map_err(|error| error.to_string()));
        let result = match CATALOG.get_or_init(Default::default).lock() {
            Ok(mut catalog) => match result {
                Ok(document) => catalog.capture(&document),
                Err(error) => { catalog.unknown_layout = true; Err(error) }
            },
            Err(_) => Err("map catalog lock poisoned".into()),
        };
        diagnostic(&format!("capture mode={mode:?} result={result:?}"));
    }
}

pub fn on_match_start(sim: &StableSim<'_>) {
    let _scope = crate::runtime_state::Scope::enter(sim);
    let mut live = Vec::new();
    for index in 0..sim.entity_count() {
        let Some(entity) = sim.entity_at(index) else { continue; };
        if let Some(kind @ (EntityKindV1::Tower | EntityKindV1::Nexus)) = entity.kind() {
            let (x, y) = entity.pos();
            live.push((kind.code(), entity.team(), x, y));
        }
    }
    live.sort_unstable();
    let result = CATALOG.get_or_init(Default::default).lock()
        .map_err(|_| "map catalog lock poisoned".to_string())
        .and_then(|catalog| catalog.select(&live));
    let summary = result.as_ref().map(|map| map.cells.iter().filter(|value| **value).count());
    diagnostic(&format!("bind origin={:?} seed={} anchors={} result={summary:?}", sim.sim_origin(), sim.seed(), live.len()));
    if let Ok(mut slot) = MATCH_MAP.get_or_init(|| Mutex::new(None)).lock() { *slot = result.ok(); }
}

pub fn in_bush(x: u64, y: u64) -> Option<bool> {
    let state = MATCH_MAP.get()?.lock().ok()?;
    state.as_ref().map(|map| map.contains(x, y))
}

pub(crate) fn clamp_to_known_map(x: u64, y: u64) -> Option<(u64,u64)> {
    MATCH_MAP.get_or_init(Default::default).lock().ok()?.as_ref()?;
    let max=CELLS as u64 * CELL_SIZE-1;
    Some((x.min(max),y.min(max)))
}

fn point(value: &Value) -> Result<(u64, u64), String> {
    let pair = value.as_array().filter(|array| array.len() == 2).ok_or("expected [x,y]")?;
    let (x, y) = (pair[0].as_u64().ok_or("invalid x")?, pair[1].as_u64().ok_or("invalid y")?);
    if x >= CELL_SIZE * CELLS as u64 || y >= CELL_SIZE * CELLS as u64 { return Err("map coordinate outside documented grid".into()); }
    Ok((x, y))
}

fn anchors(document: &Value) -> Result<Vec<Anchor>, String> {
    let mut result = Vec::new();
    let nexus = document["nexus_pos"].as_array().filter(|array| array.len() == 2).ok_or("expected two nexus positions")?;
    for (team, value) in nexus.iter().enumerate() {
        let (x, y) = point(value)?;
        result.push((EntityKindV1::Nexus.code(), team, x, y));
    }
    for tower in document["towers"].as_array().ok_or("missing tower definitions")? {
        let positions = tower["pos"].as_array().filter(|array| array.len() == 2).ok_or("expected two tower positions")?;
        for (team, value) in positions.iter().enumerate() {
            let (x, y) = point(value)?;
            result.push((EntityKindV1::Tower.code(), team, x, y));
        }
    }
    result.sort_unstable();
    Ok(result)
}

fn grid(document: &Value, name: &str) -> Result<Grid, String> {
    let rows = document[name].as_array().filter(|array| array.len() == CELLS).ok_or_else(|| format!("{name}: expected 30 rows"))?;
    rows.iter().map(|row| {
        row.as_array().filter(|array| array.len() == CELLS).ok_or_else(|| format!("{name}: expected 30 columns"))?
            .iter().map(|value| value.as_i64()
                .or_else(|| (value.as_u64() == Some(u64::MAX)).then_some(-1))
                .ok_or_else(|| format!("{name}: expected integer cells"))).collect()
    }).collect()
}

fn cell(grid: &Grid, x: u64, y: u64, x_first: bool) -> i64 {
    let (x, y) = ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize);
    if x_first { grid[x][y] } else { grid[y][x] }
}

fn oriented_bushes(document: &Value, bushes: &Grid, x_first: bool) -> Result<BushMap, String> {
    let nexus = document["nexus_pos"].as_array().filter(|array| array.len() == 2).ok_or("expected nexus positions")?;
    let empty: BTreeSet<i64> = nexus.iter().map(|value| point(value)
        .map(|(x, y)| cell(bushes, x, y, x_first))).collect::<Result<_, _>>()?;
    if empty.len() != 1 { return Err("nexus cells disagree on empty-bush encoding".into()); }
    let empty = *empty.first().ok_or("missing empty-bush encoding")?;
    if !matches!(empty, -1 | 0) || bushes.iter().flatten().any(|value| *value < empty) {
        return Err("unrecognized empty-bush encoding".into());
    }
    let mut cells = Vec::with_capacity(CELLS * CELLS);
    for y in 0..CELLS { for x in 0..CELLS {
        cells.push(cell(bushes, x as u64 * CELL_SIZE, y as u64 * CELL_SIZE, x_first) != empty);
    } }
    Ok(BushMap { cells })
}

fn parse_bushes(document: &Value) -> Result<BushMap, String> {
    let bushes = grid(document, "bushes")?;
    let regions = grid(document, "regions")?;
    let centers = document["region_centers"].as_array().filter(|array| !array.is_empty()).ok_or("missing region centers")?;
    let centers: Vec<_> = centers.iter().map(point).collect::<Result<_, _>>()?;
    // Region IDs index region_centers. Use their world coordinates to verify
    // axis order instead of guessing that Rust's nested arrays are row-major.
    let mut candidates = Vec::new();
    for x_first in [false, true] {
        if centers.iter().enumerate().all(|(id, &(x, y))| cell(&regions, x, y, x_first) == id as i64) {
            if let Ok(map) = oriented_bushes(document, &bushes, x_first) { candidates.push(map); }
        }
    }
    let Some(first) = candidates.first() else { return Err("cannot validate grid axes/empty value against regions and nexus cells".into()); };
    if candidates.iter().any(|map| map != first) { return Err("grid axis order is ambiguous".into()); }
    Ok(first.clone())
}

fn diagnostic(message: &str) {
    use std::io::Write;
    // Diagnostics never participate in gameplay decisions.
    let Some(appdata) = std::env::var_os("APPDATA") else { return; };
    let path = std::path::PathBuf::from(appdata).join("TeamSamoyed/TeamfightManager2/data/pokemon_moba_stable-map.log");
    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).create(true).open(path) {
        let _ = writeln!(file, "{message}");
    }
}

#[cfg(test)]
pub(crate) fn install_test_bush(x: u64, y: u64) {
    let mut cells = vec![false; CELLS * CELLS];
    cells[(y / CELL_SIZE) as usize * CELLS + (x / CELL_SIZE) as usize] = true;
    *MATCH_MAP.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(Arc::new(BushMap { cells }));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn document(x_first: bool, empty: i64) -> Value {
        let mut regions = vec![vec![0; CELLS]; CELLS];
        let mut bushes = vec![vec![empty; CELLS]; CELLS];
        if x_first { regions[4][9] = 1; bushes[3][7] = empty + 1; }
        else { regions[9][4] = 1; bushes[7][3] = empty + 1; }
        serde_json::json!({"regions": regions, "bushes": bushes,
            "region_centers": [[16000,16000], [144000,304000]],
            "nexus_pos": [[16000,16000], [944000,944000]],
            "towers": [{"pos": [[48000,48000],[912000,912000]]}]})
    }
    #[test]
    fn validates_axes_empty_encoding_edges_and_zero_label_bushes() {
        for x_first in [false, true] { for empty in [-1, 0] {
            let map = parse_bushes(&document(x_first, empty)).unwrap();
            assert!(map.contains(96000, 224000));
            assert!(map.contains(127999, 255999));
            assert!(!map.contains(128000, 224000));
            assert!(!map.contains(224000, 96000));
            assert!(!map.contains(960000, 0));
            assert!(!map.contains(u64::MAX, u64::MAX));
        } }
    }
    #[test]
    fn malformed_and_ambiguous_maps_are_not_guessed() {
        let mut doc = document(false, 0);
        doc["bushes"] = serde_json::json!([[0]]);
        assert!(parse_bushes(&doc).is_err());
        let mut doc = document(false, 0);
        doc["regions"] = serde_json::json!(vec![vec![0; CELLS]; CELLS]);
        doc["region_centers"] = serde_json::json!([[16000,16000]]);
        assert!(parse_bushes(&doc).is_err());
    }
    #[test]
    fn layout_matching_is_order_independent_and_never_uses_last_map() {
        let a = document(false, 0);
        let mut b = document(true, -1);
        b["nexus_pos"][0] = serde_json::json!([80000,80000]);
        let mut catalog = Catalog::default();
        catalog.capture(&a).unwrap();
        let frozen = catalog.select(&anchors(&a).unwrap()).unwrap();
        catalog.capture(&b).unwrap();
        assert_eq!(catalog.select(&anchors(&a).unwrap()).unwrap(), frozen);
        assert!(catalog.select(&[]).is_err());
        let mut changed = a.clone();
        changed["bushes"][8][3] = serde_json::json!(1);
        catalog.capture(&changed).unwrap();
        assert!(catalog.select(&anchors(&a).unwrap()).is_err());
        assert!(!frozen.contains(96000,256000));
    }
}
