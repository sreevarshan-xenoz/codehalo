use crate::core::errors::ProviderError;
use crate::core::models::{ProviderId, ProviderSnapshot, ProviderStatus, UsageSource};
use crate::providers::traits::{BoxFuture, UsageProvider};
use chrono::{DateTime, Duration, Utc};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Metrics parsed from a local Claude Code CLI session.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClaudeSessionMetrics {
    pub session_id: Option<String>,
    pub project_name: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_creation_tokens: u64,
    pub last_interaction: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Provider adapter for Anthropic Claude Code CLI.
#[derive(Debug, Clone)]
pub struct ClaudeProvider {
    config_path: Option<PathBuf>,
    active_threshold: Duration,
}

impl Default for ClaudeProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ClaudeProvider {
    pub fn new() -> Self {
        Self {
            config_path: Self::detect_config_path(),
            active_threshold: Duration::minutes(15),
        }
    }

    pub fn with_config_path(path: PathBuf) -> Self {
        Self {
            config_path: Some(path),
            active_threshold: Duration::minutes(15),
        }
    }

    fn detect_config_path() -> Option<PathBuf> {
        dirs_home().map(|h| h.join(".claude"))
    }

    /// Scans ~/.claude/projects for the most recently updated session transcript.
    pub fn find_latest_session_file(&self) -> Option<PathBuf> {
        let base = self.config_path.as_ref()?;
        let projects_dir = base.join("projects");
        if !projects_dir.is_dir() {
            return None;
        }

        let mut latest_file: Option<(PathBuf, SystemTime)> = None;

        if let Ok(entries) = std::fs::read_dir(projects_dir) {
            for entry in entries.flatten() {
                let project_path = entry.path();
                if project_path.is_dir() {
                    if let Ok(files) = std::fs::read_dir(&project_path) {
                        for file in files.flatten() {
                            let p = file.path();
                            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                                if let Ok(meta) = p.metadata() {
                                    if let Ok(modified) = meta.modified() {
                                        match &latest_file {
                                            Some((_, best_time)) if modified <= *best_time => {}
                                            _ => {
                                                latest_file = Some((p, modified));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        latest_file.map(|(path, _)| path)
    }

    /// Parses a session `.jsonl` file without touching any credentials.
    pub fn parse_session_file(&self, path: &Path) -> Result<ClaudeSessionMetrics, ProviderError> {
        let file = File::open(path)
            .map_err(|e| ProviderError::IoError(ProviderId::Claude, e.to_string()))?;
        let reader = BufReader::new(file);

        let mut metrics = ClaudeSessionMetrics::default();

        if let Some(parent) = path.parent() {
            if let Some(name) = parent.file_name() {
                metrics.project_name = Some(name.to_string_lossy().to_string());
            }
        }

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            let v: Value = match serde_json::from_str(&line) {
                Ok(val) => val,
                Err(_) => continue,
            };

            if metrics.session_id.is_none() {
                if let Some(id) = v.get("sessionId").and_then(|s| s.as_str()) {
                    metrics.session_id = Some(id.to_string());
                }
            }

            if let Some(ts_str) = v.get("timestamp").and_then(|s| s.as_str()) {
                if let Ok(dt) = DateTime::parse_from_rfc3339(ts_str) {
                    metrics.last_interaction = Some(dt.with_timezone(&Utc));
                }
            }

            // Extract usage tokens from assistant messages
            if let Some(msg) = v.get("message") {
                if let Some(usage) = msg.get("usage") {
                    if let Some(n) = usage.get("input_tokens").and_then(|v| v.as_u64()) {
                        metrics.input_tokens = metrics.input_tokens.saturating_add(n);
                    }
                    if let Some(n) = usage.get("output_tokens").and_then(|v| v.as_u64()) {
                        metrics.output_tokens = metrics.output_tokens.saturating_add(n);
                    }
                    if let Some(n) = usage.get("cache_read_input_tokens").and_then(|v| v.as_u64()) {
                        metrics.cache_read_tokens = metrics.cache_read_tokens.saturating_add(n);
                    }
                    if let Some(n) = usage.get("cache_creation_input_tokens").and_then(|v| v.as_u64()) {
                        metrics.cache_creation_tokens = metrics.cache_creation_tokens.saturating_add(n);
                    }
                }
            }
        }

        // Determine if session is recently active
        let now = Utc::now();
        if let Some(last) = metrics.last_interaction {
            metrics.is_active = (now - last) <= self.active_threshold;
        }

        Ok(metrics)
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

impl UsageProvider for ClaudeProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Claude
    }

    fn name(&self) -> &'static str {
        "Claude Code"
    }

    fn is_available(&self) -> BoxFuture<'_, bool> {
        Box::pin(async move {
            if let Some(ref path) = self.config_path {
                if path.exists() {
                    return true;
                }
            }
            which_exists("claude")
        })
    }

    fn fetch_usage(&self) -> BoxFuture<'_, Result<ProviderSnapshot, ProviderError>> {
        Box::pin(async move {
            if !self.is_available().await {
                return Err(ProviderError::NotInstalled(ProviderId::Claude));
            }

            let mut session_active = false;

            if let Some(latest_path) = self.find_latest_session_file() {
                if let Ok(metrics) = self.parse_session_file(&latest_path) {
                    session_active = metrics.is_active;
                }
            }

            Ok(ProviderSnapshot {
                provider: ProviderId::Claude,
                status: ProviderStatus::Available,
                usage_percent: None,
                remaining_percent: None,
                reset_at: None,
                session_active,
                source: UsageSource::Local,
                updated_at: Utc::now(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_claude_provider_unavailable_on_empty() {
        let provider = ClaudeProvider::with_config_path(PathBuf::from("Z:/nonexistent/claude/path"));
        // if claude is not in PATH, will be false; either way shouldn't panic
        let _ = futures::executor::block_on(provider.is_available());
    }

    #[test]
    fn test_parse_session_file() {
        let temp_dir = std::env::temp_dir().join(format!("claude_test_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)));
        let project_dir = temp_dir.join("projects").join("test-project");
        std::fs::create_dir_all(&project_dir).unwrap();

        let session_file = project_dir.join("session.jsonl");
        {
            let mut f = File::create(&session_file).unwrap();
            writeln!(
                f,
                r#"{{"sessionId":"test-123","timestamp":"2026-08-11T14:48:56.792Z","type":"attachment"}}"#
            )
            .unwrap();
            writeln!(
                f,
                r#"{{"type":"assistant","message":{{"usage":{{"input_tokens":150,"output_tokens":42,"cache_read_input_tokens":10,"cache_creation_input_tokens":5}}}}}}"#
            )
            .unwrap();
        }

        let provider = ClaudeProvider::with_config_path(temp_dir.clone());
        let found = provider.find_latest_session_file();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), session_file);

        let metrics = provider.parse_session_file(&session_file).unwrap();
        assert_eq!(metrics.session_id, Some("test-123".to_string()));
        assert_eq!(metrics.project_name, Some("test-project".to_string()));
        assert_eq!(metrics.input_tokens, 150);
        assert_eq!(metrics.output_tokens, 42);
        assert_eq!(metrics.cache_read_tokens, 10);
        assert_eq!(metrics.cache_creation_tokens, 5);

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
