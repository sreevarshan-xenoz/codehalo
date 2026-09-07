use crate::core::errors::ProviderError;
use crate::core::models::{ProviderId, ProviderSnapshot, ProviderStatus, UsageSource};
use crate::providers::traits::{BoxFuture, UsageProvider};
use chrono::Utc;
use std::path::PathBuf;

/// Provider adapter for OpenAI Codex / GitHub Copilot CLI.
#[derive(Debug, Default, Clone)]
pub struct CodexProvider {
    config_path: Option<PathBuf>,
}

impl CodexProvider {
    pub fn new() -> Self {
        Self {
            config_path: dirs_home().map(|h| h.join(".copilot")),
        }
    }
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

fn which_exists(cmd: &str) -> bool {
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            let exe = path.join(format!("{}.exe", cmd));
            let cmd_file = path.join(format!("{}.cmd", cmd));
            let plain = path.join(cmd);
            if exe.is_file() || cmd_file.is_file() || plain.is_file() {
                return true;
            }
        }
    }
    false
}

impl UsageProvider for CodexProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Codex
    }

    fn name(&self) -> &'static str {
        "Codex"
    }

    fn is_available(&self) -> BoxFuture<'_, bool> {
        Box::pin(async move {
            if let Some(ref path) = self.config_path {
                if path.exists() {
                    return true;
                }
            }
            which_exists("github-copilot-cli") || which_exists("copilot")
        })
    }

    fn fetch_usage(&self) -> BoxFuture<'_, Result<ProviderSnapshot, ProviderError>> {
        Box::pin(async move {
            if self.is_available().await {
                Ok(ProviderSnapshot {
                    provider: ProviderId::Codex,
                    status: ProviderStatus::Available,
                    usage_percent: None,
                    remaining_percent: None,
                    reset_at: None,
                    session_active: false,
                    source: UsageSource::Local,
                    updated_at: Utc::now(),
                })
            } else {
                Err(ProviderError::NotInstalled(ProviderId::Codex))
            }
        })
    }
}
