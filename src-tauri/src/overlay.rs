use tauri::{AppHandle, Manager, PhysicalPosition};
use crate::models::{EdgePosition, HudWindowConfig, MonitorInfo};

/// Enumerate available displays with DPI scaling factors and bounds
pub fn get_available_monitors(app: &AppHandle) -> Result<Vec<MonitorInfo>, String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not initialized".to_string())?;

    let monitors = window.available_monitors().map_err(|e| e.to_string())?;
    let primary = window.primary_monitor().ok().flatten();

    let mut result = Vec::new();
    for (idx, mon) in monitors.iter().enumerate() {
        let is_primary = primary
            .as_ref()
            .map(|p| p.position() == mon.position())
            .unwrap_or(idx == 0);

        result.push(MonitorInfo {
            index: idx,
            name: mon.name().cloned(),
            width: mon.size().width,
            height: mon.size().height,
            scale_factor: mon.scale_factor(),
            is_primary,
        });
    }

    Ok(result)
}

/// Reposition the overlay along the configured edge on the designated monitor
pub fn position_hud_overlay(app: &AppHandle, config: &HudWindowConfig) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not initialized".to_string())?;

    let monitors = window.available_monitors().map_err(|e| e.to_string())?;
    let target_monitor = monitors
        .get(config.monitor_index)
        .or_else(|| monitors.first())
        .ok_or_else(|| "No monitors found".to_string())?;

    let monitor_pos = target_monitor.position();
    let monitor_size = target_monitor.size();
    let scale_factor = target_monitor.scale_factor();

    let win_size = window.outer_size().map_err(|e| e.to_string())?;

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

    // Maintain always-on-top overlay
    let _ = window.set_always_on_top(true);

    Ok(())
}
