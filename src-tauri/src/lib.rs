pub mod models;
pub mod overlay;
pub mod provider;

use models::{HudWindowConfig, ProviderUsageSnapshot};
use overlay::position_hud_overlay;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub config: Mutex<HudWindowConfig>,
}

#[tauri::command]
fn sync_overlay_position(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    position_hud_overlay(&app, &config)
}

#[tauri::command]
fn get_provider_usages() -> Vec<ProviderUsageSnapshot> {
    vec![
        ProviderUsageSnapshot {
            id: "claude".into(),
            name: "Claude Code".into(),
            percentage: 82,
            used_display: "820k / 1M tokens".into(),
            limit_display: "Daily quota".into(),
            status: "ok".into(),
            is_estimate: false,
            last_updated: "just now".into(),
        },
        ProviderUsageSnapshot {
            id: "codex".into(),
            name: "OpenAI Codex".into(),
            percentage: 61,
            used_display: "61 / 100 requests".into(),
            limit_display: "5h limit".into(),
            status: "ok".into(),
            is_estimate: false,
            last_updated: "just now".into(),
        },
        ProviderUsageSnapshot {
            id: "cursor".into(),
            name: "Cursor Pro".into(),
            percentage: 48,
            used_display: "240 / 500 fast calls".into(),
            limit_display: "Monthly cycle".into(),
            status: "ok".into(),
            is_estimate: false,
            last_updated: "just now".into(),
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config: Mutex::new(HudWindowConfig::default()),
        })
        .invoke_handler(tauri::generate_handler![
            sync_overlay_position,
            get_provider_usages
        ])
        .setup(|app| {
            let state = app.state::<AppState>();
            if let Ok(config) = state.config.lock() {
                let _ = position_hud_overlay(app.handle(), &config);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running CodeHalo application");
}
