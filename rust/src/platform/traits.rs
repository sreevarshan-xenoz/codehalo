use crate::core::models::{EdgePosition, MonitorInfo};

pub trait PlatformManager: Send + Sync {
    /// Retrieve all connected displays with geometry & DPI
    fn get_monitors(&self) -> Vec<MonitorInfo>;

    /// Position the frameless overlay window on the requested edge
    fn position_overlay(&self, edge: EdgePosition, monitor_idx: usize, width: u32, height: u32) -> Result<(), String>;

    /// Securely store provider credentials
    fn save_credential(&self, key: &str, secret: &str) -> Result<(), String>;

    /// Securely retrieve provider credentials
    fn get_credential(&self, key: &str) -> Result<Option<String>, String>;
}
