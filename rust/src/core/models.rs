use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for each supported AI coding assistant / provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderId {
    Claude,
    Codex,
    Cursor,
    Gemini,
    Custom(String),
}

impl ProviderId {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Claude => "claude",
            Self::Codex => "codex",
            Self::Cursor => "cursor",
            Self::Gemini => "gemini",
            Self::Custom(s) => s.as_str(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Claude => "Claude Code",
            Self::Codex => "Codex",
            Self::Cursor => "Cursor",
            Self::Gemini => "Gemini / AGY",
            Self::Custom(s) => s.as_str(),
        }
    }

    pub fn short_label(&self) -> &str {
        match self {
            Self::Claude => "C",
            Self::Codex => "X",
            Self::Cursor => "K",
            Self::Gemini => "G",
            Self::Custom(s) => s.get(0..1).unwrap_or("?"),
        }
    }

    pub fn default_color(&self) -> &str {
        match self {
            Self::Claude => "#DA7756",
            Self::Codex => "#A78BFA",
            Self::Cursor => "#34D399",
            Self::Gemini => "#38BDF8",
            Self::Custom(_) => "#94A3B8",
        }
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Operational status of a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderStatus {
    Unknown,
    Available,
    Unavailable,
    Authenticating,
    Error,
}

impl ProviderStatus {
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Available)
    }
}

impl fmt::Display for ProviderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Available => write!(f, "available"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::Authenticating => write!(f, "authenticating"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// Provenance of the retrieved usage data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UsageSource {
    Official,
    Local,
    Derived,
    Estimated,
    Unavailable,
}

impl fmt::Display for UsageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Official => write!(f, "official"),
            Self::Local => write!(f, "local"),
            Self::Derived => write!(f, "derived"),
            Self::Estimated => write!(f, "estimated"),
            Self::Unavailable => write!(f, "unavailable"),
        }
    }
}

/// Normalized point-in-time usage snapshot produced by any provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderSnapshot {
    pub provider: ProviderId,
    pub status: ProviderStatus,
    /// 0.0 to 1.0 (or None if unmetered or unavailable)
    pub usage_percent: Option<f32>,
    pub remaining_percent: Option<f32>,
    pub reset_at: Option<DateTime<Utc>>,
    pub session_active: bool,
    pub source: UsageSource,
    pub updated_at: DateTime<Utc>,
}

impl ProviderSnapshot {
    pub fn new(provider: ProviderId) -> Self {
        Self {
            provider,
            status: ProviderStatus::Unknown,
            usage_percent: None,
            remaining_percent: None,
            reset_at: None,
            session_active: false,
            source: UsageSource::Unavailable,
            updated_at: Utc::now(),
        }
    }

    pub fn available(provider: ProviderId, usage: Option<f32>, source: UsageSource) -> Self {
        let remaining = usage.map(|u| (1.0 - u).clamp(0.0, 1.0));
        Self {
            provider,
            status: ProviderStatus::Available,
            usage_percent: usage,
            remaining_percent: remaining,
            reset_at: None,
            session_active: false,
            source,
            updated_at: Utc::now(),
        }
    }

    pub fn unavailable(provider: ProviderId) -> Self {
        Self {
            provider,
            status: ProviderStatus::Unavailable,
            usage_percent: None,
            remaining_percent: None,
            reset_at: None,
            session_active: false,
            source: UsageSource::Unavailable,
            updated_at: Utc::now(),
        }
    }

    pub fn error(provider: ProviderId) -> Self {
        Self {
            provider,
            status: ProviderStatus::Error,
            usage_percent: None,
            remaining_percent: None,
            reset_at: None,
            session_active: false,
            source: UsageSource::Unavailable,
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgePosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub index: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_id_attributes() {
        assert_eq!(ProviderId::Claude.as_str(), "claude");
        assert_eq!(ProviderId::Claude.display_name(), "Claude Code");
        assert_eq!(ProviderId::Claude.short_label(), "C");
        assert_eq!(ProviderId::Claude.default_color(), "#DA7756");

        assert_eq!(ProviderId::Codex.short_label(), "X");
        assert_eq!(ProviderId::Cursor.short_label(), "K");
        assert_eq!(ProviderId::Gemini.short_label(), "G");
    }

    #[test]
    fn test_provider_snapshot_creation() {
        let snap = ProviderSnapshot::available(ProviderId::Claude, Some(0.35), UsageSource::Local);
        assert_eq!(snap.status, ProviderStatus::Available);
        assert_eq!(snap.usage_percent, Some(0.35));
        assert!((snap.remaining_percent.unwrap() - 0.65).abs() < 0.001);
        assert_eq!(snap.source, UsageSource::Local);

        let unavail = ProviderSnapshot::unavailable(ProviderId::Cursor);
        assert_eq!(unavail.status, ProviderStatus::Unavailable);
        assert!(unavail.usage_percent.is_none());
        assert_eq!(unavail.source, UsageSource::Unavailable);
    }
}
