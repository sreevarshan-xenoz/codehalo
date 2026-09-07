use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};
use crate::models::{EdgePosition, HudWindowConfig};

/// Position the HUD overlay window along the requested monitor edge
pub fn position_hud_overlay(app: &AppHandle, config: &HudWindowConfig) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        // Retrieve available monitors
        let monitors = window.available_monitors().map_err(|e| e.to_string())?;
        let target_monitor = monitors
            .get(config.monitor_index)
            .or_else(|| monitors.first())
            .ok_or_else(|| "No monitors found".to_string())?;

        let monitor_pos = target_monitor.position();
        let monitor_size = target_monitor.size();
        let scale_factor = target_monitor.scale_factor();

        let win_size = window.outer_size().map_err(|e| e.to_string())?;

        // Calculate position based on edge selection
        let (x, y) = match config.edge {
            EdgePosition::Top => {
                let centered_x = monitor_pos.x + ((monitor_size.width as i32 - win_size.width as i32) / 2);
                let top_y = monitor_pos.y + (config.offset_px as f64 * scale_factor) as i32;
                (centered_x, top_y)
            }
            EdgePosition::Bottom => {
                let centered_x = monitor_pos.x + ((monitor_size.width as i32 - win_size.width as i32) / 2);
                let bottom_y = monitor_pos.y + monitor_size.height as i32 - win_size.height as i32 - (config.offset_px as f64 * scale_factor) as i32;
                (centered_x, bottom_y)
            }
            EdgePosition::Left => {
                let left_x = monitor_pos.x + (config.offset_px as f64 * scale_factor) as i32;
                let centered_y = monitor_pos.y + ((monitor_size.height as i32 - win_size.height as i32) / 2);
                (left_x, centered_y)
            }
            EdgePosition::Right => {
                let right_x = monitor_pos.x + monitor_size.width as i32 - win_size.width as i32 - (config.offset_px as f64 * scale_factor) as i32;
                let centered_y = monitor_pos.y + ((monitor_size.height as i32 - win_size.height as i32) / 2);
                (right_x, centered_y)
            }
        };

        window
            .set_position(PhysicalPosition::new(x, y))
            .map_err(|e| e.to_string())?;

        // Ensure always on top
        let _ = window.set_always_on_top(true);
    }

    Ok(())
}
