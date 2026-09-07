use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricStatus {
    Ok,
    Warning,
    Error,
    Offline,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
