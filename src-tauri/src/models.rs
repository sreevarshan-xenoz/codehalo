use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub index: usize,
    pub name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EdgePosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudWindowConfig {
    pub edge: EdgePosition,
    pub monitor_index: usize,
    pub offset_px: i32,
}

impl Default for HudWindowConfig {
    fn default() -> Self {
        Self {
            edge: EdgePosition::Top,
            monitor_index: 0,
            offset_px: 12,
        }
    }
}

/// Normalized provider usage snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsageSnapshot {
    pub id: String,
    pub name: String,
    pub percentage: u8,
    #[serde(rename = "usedDisplay")]
    pub used_display: String,
    #[serde(rename = "limitDisplay")]
    pub limit_display: String,
    pub status: String,
    #[serde(rename = "isEstimate")]
    pub is_estimate: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}
