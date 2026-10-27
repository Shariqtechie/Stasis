#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{NaiveDate, Utc, Duration};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf};
use winreg::{enums::*, RegKey};

const PATH: &str = r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings";
const KEYS: [&str; 6] = ["PauseUpdatesExpiryTime","PauseFeatureUpdatesStartTime","PauseFeatureUpdatesEndTime","PauseQualityUpdatesStartTime","PauseQualityUpdatesEndTime","PauseUpdatesRequestedUntilTime"];
type Snap = BTreeMap<String, Option<String>>;

#[derive(Serialize)]
struct Status { paused: bool, expiry: Option<String>, days_left: i64, os: String }

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
fn backup() -> Result<(), String> {
    let f = dir().join(format!("stasis_{}.json", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    fs::write(f, serde_json::to_string_pretty(&snapshot()).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
fn status() -> Status {
    let expiry = snapshot().get("PauseUpdatesExpiryTime").cloned().flatten();
    let days_left = expiry.as_ref()
        .and_then(|e| chrono::DateTime::parse_from_rfc3339(e).ok())
        .map(|d| (d.with_timezone(&Utc) - Utc::now()).num_days()).unwrap_or(0);
    let cv = hklm().open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion").ok();
    let g = |n: &str| cv.as_ref().and_then(|k| k.get_value::<String, _>(n).ok()).unwrap_or_default();
    let build: u32 = g("CurrentBuild").parse().unwrap_or(0);
    let name = g("ProductName").replace("Windows 10", if build >= 22000 { "Windows 11" } else { "Windows 10" });
    let ver = g("DisplayVersion");
    Status { paused: days_left > 0, expiry, days_left, os: format!("{} {} (build {})", name, ver, build).replace("  ", " ") }
}

#[tauri::command]
fn apply(days: Option<i64>, date: Option<String>) -> Result<String, String> {
    let now = Utc::now();
    let end = match (days, date) {
        (Some(d), _) => now + Duration::days(d),
        (_, Some(s)) => NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| "Use a date like 2029-12-31.")?
            .and_hms_opt(0, 0, 0).unwrap().and_utc(),
        _ => return Err("Pick a duration first.".into()),
    };
    if end <= now { return Err("Pick a date in the future.".into()); }
    backup()?;
    let (s, e) = (now.format("%Y-%m-%dT%H:%M:%SZ").to_string(), end.format("%Y-%m-%dT%H:%M:%SZ").to_string());
    let mut m = Snap::new();
    for k in KEYS { m.insert(k.into(), Some(if k.contains("Start") { s.clone() } else { e.clone() })); }
    write_snap(&m)?;
    Ok(format!("Updates frozen until {}", end.format("%d %b %Y")))
}

#[tauri::command]
fn resume() -> Result<String, String> {
    backup()?;
    write_snap(&KEYS.iter().map(|k| (k.to_string(), None)).collect())?;
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
    backup()?; write_snap(&s)?;
    Ok("Backup restored".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![status, apply, resume, backups, restore])
        .run(tauri::generate_context!()).expect("error while running Stasis");
}
