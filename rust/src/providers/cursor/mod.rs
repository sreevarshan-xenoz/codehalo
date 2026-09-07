use crate::core::errors::ProviderError;
use crate::core::models::{ProviderId, ProviderSnapshot, ProviderStatus, UsageSource};
use crate::providers::traits::{BoxFuture, UsageProvider};
use chrono::Utc;
use std::path::PathBuf;

/// Provider adapter for Cursor AI editor.
#[derive(Debug, Default, Clone)]
pub struct CursorProvider {
    config_path: Option<PathBuf>,
}

impl CursorProvider {
    pub fn new() -> Self {
        Self {
            config_path: Self::detect_config_path(),
        }
    }

    fn detect_config_path() -> Option<PathBuf> {
        #[cfg(windows)]
        {
            std::env::var_os("APPDATA")
                .map(PathBuf::from)
                .map(|appdata| appdata.join("Cursor"))
        }
        #[cfg(not(windows))]
        {
            dirs_home().map(|h| h.join(".config").join("Cursor"))
        }
    }
}

#[allow(dead_code)]
fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

impl UsageProvider for CursorProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Cursor
    }

    fn name(&self) -> &'static str {
        "Cursor"
    }

    fn is_available(&self) -> BoxFuture<'_, bool> {
        Box::pin(async move {
            if let Some(ref path) = self.config_path {
                if path.exists() {
                    return true;
                }
            }
            false
        })
    }

    fn fetch_usage(&self) -> BoxFuture<'_, Result<ProviderSnapshot, ProviderError>> {
        Box::pin(async move {
            if self.is_available().await {
                Ok(ProviderSnapshot {
                    provider: ProviderId::Cursor,
                    status: ProviderStatus::Available,
                    usage_percent: None,
                    remaining_percent: None,
                    reset_at: None,
                    session_active: false,
                    source: UsageSource::Local,
                    updated_at: Utc::now(),
                })
            } else {
                Err(ProviderError::NotInstalled(ProviderId::Cursor))
            }
        })
    }
}
