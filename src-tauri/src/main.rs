#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{NaiveDate, Utc, Duration};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf, sync::atomic::{AtomicBool, Ordering}};
static MICA: AtomicBool = AtomicBool::new(false);
use winreg::{enums::*, RegKey};

const PATH: &str = r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings";
const KEYS: [&str; 6] = ["PauseUpdatesExpiryTime","PauseFeatureUpdatesStartTime","PauseFeatureUpdatesEndTime","PauseQualityUpdatesStartTime","PauseQualityUpdatesEndTime","PauseUpdatesRequestedUntilTime"];
const MAX_DAYS: i64 = 3650;
type Snap = BTreeMap<String, Option<String>>;

#[derive(Serialize)]
struct Status { paused: bool, expiry: Option<String>, days_left: i64, total_days: i64, by_app: bool, os: String }

fn hklm() -> RegKey { RegKey::predef(HKEY_LOCAL_MACHINE) }
fn dir() -> PathBuf {
    let p = PathBuf::from(std::env::var("APPDATA").unwrap_or_default()).join("Stasis").join("backups");
    let _ = fs::create_dir_all(&p); p
}
fn snapshot() -> Snap {
    let k = hklm().open_subkey(PATH).ok();
    KEYS.iter().map(|n| (n.to_string(), k.as_ref().and_then(|k| k.get_value::<String, _>(n).ok()))).collect()
}
fn write_snap(s: &Snap) -> Result<(), String> {
    let (k, _) = hklm().create_subkey(PATH).map_err(|e| e.to_string())?;
    for (n, v) in s {
        match v {
            Some(v) => k.set_value(n, v).map_err(|e| e.to_string())?,
            None => { let _ = k.delete_value(n); }
        }
    }
    Ok(())
}
fn state_file() -> PathBuf { dir().parent().unwrap().join("state.json") }
fn mark(expiry: Option<&str>) {
    match expiry { Some(e) => { let _ = fs::write(state_file(), e); } None => { let _ = fs::remove_file(state_file()); } }
}
fn backup() -> Result<(), String> {
    let f = dir().join(format!("stasis_{}.json", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    fs::write(f, serde_json::to_string_pretty(&snapshot()).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
fn status() -> Status {
    let snap = snapshot();
    let expiry = snap.get("PauseUpdatesExpiryTime").cloned().flatten();
    let start = snap.get("PauseFeatureUpdatesStartTime").cloned().flatten();
    let total_days = match (start.as_deref().map(chrono::DateTime::parse_from_rfc3339), expiry.as_deref().map(chrono::DateTime::parse_from_rfc3339)) { (Some(Ok(s)), Some(Ok(e))) => (e - s).num_days(), _ => 0 };
    let secs_left = expiry.as_ref()
        .and_then(|e| chrono::DateTime::parse_from_rfc3339(e).ok())
        .map(|d| (d.with_timezone(&Utc) - Utc::now()).num_seconds()).unwrap_or(0);
    let days_left = if secs_left > 0 { (secs_left + 86_399) / 86_400 } else { 0 };
    // Windows Settings pauses for 5 weeks (35 days) at most, so anything longer must be a Stasis freeze.
    let marked = expiry.is_some() && fs::read_to_string(state_file()).map(|s| Some(s.trim()) == expiry.as_deref()).unwrap_or(false);
    let by_app = marked || days_left > 36 || total_days > 36;
    let cv = hklm().open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion").ok();
    let g = |n: &str| cv.as_ref().and_then(|k| k.get_value::<String, _>(n).ok()).unwrap_or_default();
    let build: u32 = g("CurrentBuild").parse().unwrap_or(0);
    let name = g("ProductName").replace("Windows 10", if build >= 22000 { "Windows 11" } else { "Windows 10" });
    let ver = g("DisplayVersion");
    Status { paused: days_left > 0, expiry, days_left, total_days, by_app: by_app && days_left > 0, os: format!("{} {} (build {})", name, ver, build).replace("  ", " ") }
}

#[tauri::command]
fn apply(days: Option<i64>, date: Option<String>, extend: Option<bool>) -> Result<String, String> {
    let now = Utc::now();
    let ext = extend.unwrap_or(false);
    let cur = snapshot().get("PauseUpdatesExpiryTime").cloned().flatten()
        .and_then(|e| chrono::DateTime::parse_from_rfc3339(&e).ok())
        .map(|x| x.with_timezone(&Utc)).filter(|c| *c > now);
    let end = match (days, date) {
        (Some(d), _) => match (ext, cur) { (true, Some(c)) => c + Duration::days(d), _ => now + Duration::days(d) },
        (_, Some(s)) => NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| "Pick a date first.")?
            .and_hms_opt(0, 0, 0).unwrap().and_utc(),
        _ => return Err("Pick a duration first.".into()),
    };
    if end <= now { return Err("Pick a date in the future.".into()); }
    if let (true, Some(c)) = (ext, cur) { if end <= c { return Err("Pick a date after your current end date.".into()); } }
    if end > now + Duration::days(MAX_DAYS) { return Err("The limit is 10 years from today. Pick a shorter time.".into()); }
    backup()?;
    let (s, e) = (now.format("%Y-%m-%dT%H:%M:%SZ").to_string(), end.format("%Y-%m-%dT%H:%M:%SZ").to_string());
    let mut m = Snap::new();
    for k in KEYS { m.insert(k.into(), Some(if k.contains("Start") { s.clone() } else { e.clone() })); }
    write_snap(&m)?;
    mark(Some(&e));
    Ok(format!("Updates frozen until {}", end.format("%d %b %Y")))
}

#[tauri::command]
fn resume() -> Result<String, String> {
    backup()?;
    write_snap(&KEYS.iter().map(|k| (k.to_string(), None)).collect())?;
    mark(None);
    Ok("Updates resumed".into())
}

#[tauri::command]
fn backups() -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(dir()).map(|r| r.flatten().map(|e| e.file_name().to_string_lossy().into()).collect()).unwrap_or_default();
    v.sort(); v.reverse(); v
}

#[tauri::command]
fn restore(name: String) -> Result<String, String> {
    if name.contains(['/', '\\']) { return Err("Invalid backup name.".into()); }
    let s: Snap = serde_json::from_str(&fs::read_to_string(dir().join(&name)).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    backup()?; write_snap(&s)?; mark(None);
    Ok("Backup restored".into())
}

#[tauri::command]
fn mica() -> bool { MICA.load(Ordering::Relaxed) }

#[tauri::command]
fn version() -> String { env!("CARGO_PKG_VERSION").to_string() }

#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    const OK: [&str; 3] = ["https://github.com/Shariqtechie/Stasis", "https://x.com/intent/post", "https://www.reddit.com/submit"];
    if !OK.iter().any(|p| url.starts_with(p)) { return Err("Blocked link".into()); }
    std::process::Command::new("explorer").arg(&url).spawn().map(|_| ()).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;
            if let Some(w) = app.get_webview_window("main") {
                let fx = tauri::window::EffectsBuilder::new().effect(tauri::window::Effect::Mica).build();
                MICA.store(w.set_effects(fx).is_ok(), Ordering::Relaxed);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![status, apply, resume, backups, restore, mica, version, open_url])
        .run(tauri::generate_context!()).expect("error while running Stasis");
}
