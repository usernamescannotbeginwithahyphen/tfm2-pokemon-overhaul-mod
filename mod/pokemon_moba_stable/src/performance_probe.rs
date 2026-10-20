//! Sampled callback timings; diagnostic data never participates in simulation.
use std::cell::Cell;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const STRIDE: u32 = 128;
fn stride(stage: usize) -> u32 { if stage == Stage::State as usize { 4096 } else { STRIDE } }
const NAMES: [&str; 9] = ["ai_think", "ai_world", "activation_gate", "target_gate",
    "effect_apply", "effect_metadata", "passive_update", "match_tick", "state_access"];
#[derive(Clone, Copy)]
pub enum Stage { AiThink, AiWorld, Activation, Target, Effect, Metadata, Passive, Match, State }
struct Counter { samples: AtomicU64, nanos: AtomicU64, max_nanos: AtomicU64 }
impl Counter {
    const fn new() -> Self { Self { samples: AtomicU64::new(0), nanos: AtomicU64::new(0), max_nanos: AtomicU64::new(0) } }
}
static COUNTERS: [Counter; 9] = [const { Counter::new() }; 9];
static START: OnceLock<Instant> = OnceLock::new();
static NEXT_REPORT: AtomicU64 = AtomicU64::new(2);
thread_local! { static CALLS: Cell<[u32; 9]> = const { Cell::new([0; 9]) }; }
static WRITER: OnceLock<Mutex<crate::ai_trace::TraceWriter>> = OnceLock::new();

pub fn initialize() {
    START.get_or_init(Instant::now);
    let writer = WRITER.get_or_init(|| Mutex::new(crate::ai_trace::TraceWriter::open(
        "pokemon_moba_stable-performance.log", 512 * 1024)));
    if let Ok(mut writer) = writer.lock() {
        writer.write(&serde_json::json!({"build": env!("CARGO_PKG_VERSION"),
            "type": "start", "sample_stride": STRIDE, "state_sample_stride": 4096,
            "state_profiling_enabled": cfg!(feature = "profile-state-access"),
            "timings": "inclusive; do not sum nested stages"}));
    }
}

pub struct Probe { stage: usize, start: Option<Instant> }
pub fn measure(stage: Stage) -> Probe {
    let stage = stage as usize;
    if cfg!(test) { return Probe { stage, start: None }; }
    let due = CALLS.with(|counts| {
        let mut values = counts.get();
        values[stage] = values[stage].wrapping_add(1);
        let due = values[stage] % stride(stage) == 1;
        counts.set(values);
        due
    });
    Probe { stage, start: due.then(Instant::now) }
}

impl Drop for Probe {
    fn drop(&mut self) {
        let Some(start) = self.start else { return; };
        let nanos = start.elapsed().as_nanos().min(u64::MAX as u128) as u64;
        let counter = &COUNTERS[self.stage];
        counter.samples.fetch_add(1, Ordering::Relaxed);
        counter.nanos.fetch_add(nanos, Ordering::Relaxed);
        counter.max_nanos.fetch_max(nanos, Ordering::Relaxed);
        let elapsed = START.get_or_init(Instant::now).elapsed();
        let previous = NEXT_REPORT.load(Ordering::Relaxed);
        if elapsed.as_secs() < previous || NEXT_REPORT.compare_exchange(previous,
            elapsed.as_secs() + 10, Ordering::Relaxed, Ordering::Relaxed).is_err() { return; }
        let rows: Vec<_> = COUNTERS.iter().zip(NAMES).enumerate().map(|(stage, (c, name))| {
            let samples = c.samples.load(Ordering::Relaxed);
            let nanos = c.nanos.load(Ordering::Relaxed);
            serde_json::json!({"stage": name, "samples": samples,
                "enabled": stage != Stage::State as usize || cfg!(feature = "profile-state-access"),
                "sample_stride": stride(stage),
                "estimated_calls": samples.saturating_mul(stride(stage) as u64),
                "sampled_ms": nanos as f64 / 1_000_000.0,
                "max_sample_ms": c.max_nanos.load(Ordering::Relaxed) as f64 / 1_000_000.0})
        }).collect();
        let Some(writer) = WRITER.get() else { return; };
        if let Ok(mut writer) = writer.try_lock() {
            writer.write(&serde_json::json!({"type": "timing", "wall_ms": elapsed.as_millis(), "stages": rows}));
        }
    }
}
