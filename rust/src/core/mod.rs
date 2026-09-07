pub mod models;

use models::{EdgePosition, MonitorInfo, ProviderUsage};
use std::sync::Mutex;

pub struct ApplicationState {
    pub current_edge: Mutex<EdgePosition>,
    pub monitor_index: Mutex<usize>,
    pub cached_usages: Mutex<Vec<ProviderUsage>>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            current_edge: Mutex::new(EdgePosition::Top),
            monitor_index: Mutex::new(0),
            cached_usages: Mutex::new(Vec::new()),
        }
    }
}
