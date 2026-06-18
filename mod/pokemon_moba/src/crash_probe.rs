use std::any::Any;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::panic::{self, PanicHookInfo};
use std::path::Path;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

const PANIC_LOG_PATH: &str = "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\panic-probe.log";
const CALLBACK_PANIC_LOG_PATH: &str =
    "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\mod-panic-probe.log";
const STAT_PROBE_LOG_PATH: &str =
    "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\stat-probe.log";
const STAT_PROBE_FLAG_PATH: &str =
    "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\enable-stat-probe.txt";
const STAT_PROBE_DAMAGE_FLAG_PATH: &str =
    "C:\\Users\\james\\Documents\\TFT Pokemon Mod\\logs\\enable-stat-probe-damage.txt";
static PANIC_HOOK_INSTALL: Once = Once::new();

pub fn install_panic_hook() {
    PANIC_HOOK_INSTALL.call_once(|| {
        let previous_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            log_panic(info);
            previous_hook(info);
        }));
    });
}

pub fn catch_unwind_probe<T, F>(event: &str, detail: String, fallback: T, f: F) -> T
where
    F: FnOnce() -> T,
{
    match panic::catch_unwind(panic::AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(payload) => {
            write_line(
                CALLBACK_PANIC_LOG_PATH,
                &format!(
                    "event={} time_ms={} detail=\"{}\" payload=\"{}\"",
                    sanitize_field(event),
                    timestamp_millis(),
                    sanitize_field(&detail),
                    sanitize_field(&panic_payload_to_string(payload.as_ref()))
                ),
            );
            fallback
        }
    }
}

pub fn log_damage_probe(_line: &str) {}

#[allow(dead_code)]
pub fn log_credit_probe(_line: &str) {}

pub fn stat_probe_enabled() -> bool {
    Path::new(STAT_PROBE_FLAG_PATH).exists() || stat_probe_damage_enabled()
}

fn stat_probe_damage_enabled() -> bool {
    Path::new(STAT_PROBE_DAMAGE_FLAG_PATH).exists()
}

pub fn log_stat_probe(line: &str) {
    if stat_probe_damage_enabled() {
        write_line(STAT_PROBE_LOG_PATH, line);
    }
}

pub fn log_stat_probe_event(line: &str) {
    if stat_probe_enabled() {
        write_line(STAT_PROBE_LOG_PATH, line);
    }
}

pub fn log_kda_probe(line: &str) {
    log_stat_probe_event(line);
}

pub fn sanitize_log_field(value: &str) -> String {
    sanitize_field(value)
}

fn log_panic(info: &PanicHookInfo<'_>) {
    let location = info
        .location()
        .map(|location| {
            format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )
        })
        .unwrap_or_else(|| "unknown".to_string());
    let payload = info
        .payload()
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
        .unwrap_or("<non-string panic payload>");

    write_line(
        PANIC_LOG_PATH,
        &format!(
            "event=panic time_ms={} location=\"{}\" payload=\"{}\"",
            timestamp_millis(),
            sanitize_field(&location),
            sanitize_field(payload)
        ),
    );
}

fn panic_payload_to_string(payload: &(dyn Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("<non-string panic payload>")
        .to_string()
}

fn write_line(path: &str, line: &str) {
    if let Some(parent) = Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };

    let _ = writeln!(file, "{line}");
}

fn sanitize_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}

fn timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
