pub mod models;
pub mod overlay;
pub mod provider;

use models::{EdgePosition, HudWindowConfig, MonitorInfo};
use overlay::{get_available_monitors, position_hud_overlay};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub config: Mutex<HudWindowConfig>,
}

#[tauri::command]
fn get_monitors(app: AppHandle) -> Result<Vec<MonitorInfo>, String> {
    get_available_monitors(&app)
}

#[tauri::command]
fn get_hud_config(state: State<'_, AppState>) -> Result<HudWindowConfig, String> {
    state.config.lock().map(|c| c.clone()).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_hud_position(
    edge: String,
    monitor_index: usize,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let edge_enum = match edge.to_lowercase().as_str() {
        "bottom" => EdgePosition::Bottom,
        "left" => EdgePosition::Left,
        "right" => EdgePosition::Right,
        _ => EdgePosition::Top,
    };

    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    config.edge = edge_enum;
    config.monitor_index = monitor_index;

    position_hud_overlay(&app, &config)
}

#[tauri::command]
fn sync_overlay_position(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    position_hud_overlay(&app, &config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config: Mutex::new(HudWindowConfig::default()),
        })
        .invoke_handler(tauri::generate_handler![
            get_monitors,
            get_hud_config,
            update_hud_position,
            sync_overlay_position
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
