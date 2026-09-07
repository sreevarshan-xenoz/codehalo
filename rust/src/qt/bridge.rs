#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qproperty(bool, expanded)]
        #[qproperty(QString, current_edge)]
        #[qproperty(QString, primary_monitor_name)]
        #[qproperty(f64, scale_factor)]
        #[qproperty(QString, summary_text)]
        #[qproperty(QString, last_refresh)]
        #[qproperty(i32, active_count)]
        #[qproperty(i32, total_count)]

        // Claude Code properties
        #[qproperty(bool, claude_active)]
        #[qproperty(f64, claude_usage)]
        #[qproperty(f64, claude_remaining)]
        #[qproperty(QString, claude_status)]

        // Codex properties
        #[qproperty(bool, codex_active)]
        #[qproperty(f64, codex_usage)]
        #[qproperty(f64, codex_remaining)]
        #[qproperty(QString, codex_status)]

        // Cursor properties
        #[qproperty(bool, cursor_active)]
        #[qproperty(f64, cursor_usage)]
        #[qproperty(f64, cursor_remaining)]
        #[qproperty(QString, cursor_status)]

        type CodeHaloBridge = super::CodeHaloBridgeRust;

        #[qinvokable]
        fn toggle_expanded(self: Pin<&mut CodeHaloBridge>);

        #[qinvokable]
        fn set_edge(self: Pin<&mut CodeHaloBridge>, edge: &QString);

        #[qinvokable]
        fn refresh_all(self: Pin<&mut CodeHaloBridge>);

        #[qinvokable]
        fn refresh_provider(self: Pin<&mut CodeHaloBridge>, provider: &QString);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use crate::core::models::{ProviderId, ProviderStatus};
use crate::core::state::AppState;
use crate::providers::registry::ProviderRegistry;
use crate::services::usage_service::UsageService;
use chrono::Utc;
use std::sync::Arc;

pub struct CodeHaloBridgeRust {
    pub expanded: bool,
    pub current_edge: QString,
    pub primary_monitor_name: QString,
    pub scale_factor: f64,
    pub summary_text: QString,
    pub last_refresh: QString,
    pub active_count: i32,
    pub total_count: i32,

    pub claude_active: bool,
    pub claude_usage: f64,
    pub claude_remaining: f64,
    pub claude_status: QString,

    pub codex_active: bool,
    pub codex_usage: f64,
    pub codex_remaining: f64,
    pub codex_status: QString,

    pub cursor_active: bool,
    pub cursor_usage: f64,
    pub cursor_remaining: f64,
    pub cursor_status: QString,

    usage_service: Arc<UsageService>,
}

impl Default for CodeHaloBridgeRust {
    fn default() -> Self {
        let registry = Arc::new(ProviderRegistry::with_defaults());
        let state = Arc::new(AppState::new());
        let usage_service = Arc::new(UsageService::new(registry, state));

        Self {
            expanded: false,
            current_edge: QString::from("Top"),
            primary_monitor_name: QString::from("Primary"),
            scale_factor: 1.0,
            summary_text: QString::from("Monitoring active"),
            last_refresh: QString::from("Never"),
            active_count: 0,
            total_count: 3,

            claude_active: false,
            claude_usage: 0.0,
            claude_remaining: 1.0,
            claude_status: QString::from("Ready"),

            codex_active: false,
            codex_usage: 0.0,
            codex_remaining: 1.0,
            codex_status: QString::from("Ready"),

            cursor_active: false,
            cursor_usage: 0.0,
            cursor_remaining: 1.0,
            cursor_status: QString::from("Ready"),

            usage_service,
        }
    }
}

impl qobject::CodeHaloBridge {
    pub fn toggle_expanded(mut self: Pin<&mut Self>) {
        let next = !self.expanded();
        self.as_mut().set_expanded(next);
    }

    pub fn set_edge(mut self: Pin<&mut Self>, edge: &QString) {
        self.as_mut().set_current_edge(edge.clone());
    }

    pub fn refresh_all(mut self: Pin<&mut Self>) {
        let usage_service = Arc::clone(&self.rust().usage_service);
        let results = futures::executor::block_on(usage_service.refresh_all());

        let mut active_count = 0;
        let total_count = results.len() as i32;

        for (id, res) in results {
            let (is_active, usage, remaining, status_str) = match res {
                Ok(snap) => {
                    let active = snap.session_active;
                    let u = snap.usage_percent.map(|v| v as f64).unwrap_or(0.0);
                    let r = snap.remaining_percent.map(|v| v as f64).unwrap_or(1.0 - u);
                    let st = match snap.status {
                        ProviderStatus::Available if active => "Active".to_string(),
                        ProviderStatus::Available => "Available".to_string(),
                        ProviderStatus::Unavailable => "Unavailable".to_string(),
                        ProviderStatus::Authenticating => "Authenticating".to_string(),
                        ProviderStatus::Error => "Error".to_string(),
                        ProviderStatus::Unknown => "Unknown".to_string(),
                    };
                    (active, u, r, st)
                }
                Err(err) => (false, 0.0, 1.0, err.to_string()),
            };

            if is_active {
                active_count += 1;
            }

            let q_status = QString::from(status_str.as_str());

            match id {
                ProviderId::Claude => {
                    self.as_mut().set_claude_active(is_active);
                    self.as_mut().set_claude_usage(usage);
                    self.as_mut().set_claude_remaining(remaining);
                    self.as_mut().set_claude_status(q_status);
                }
                ProviderId::Codex => {
                    self.as_mut().set_codex_active(is_active);
                    self.as_mut().set_codex_usage(usage);
                    self.as_mut().set_codex_remaining(remaining);
                    self.as_mut().set_codex_status(q_status);
                }
                ProviderId::Cursor => {
                    self.as_mut().set_cursor_active(is_active);
                    self.as_mut().set_cursor_usage(usage);
                    self.as_mut().set_cursor_remaining(remaining);
                    self.as_mut().set_cursor_status(q_status);
                }
                _ => {}
            }
        }

        let now_str = Utc::now().format("%H:%M:%S").to_string();
        self.as_mut().set_active_count(active_count);
        self.as_mut().set_total_count(total_count);
        self.as_mut().set_last_refresh(QString::from(now_str.as_str()));

        let summary = if active_count > 0 {
            format!("{} active", active_count)
        } else {
            "All idle".to_string()
        };
        self.as_mut().set_summary_text(QString::from(summary.as_str()));
    }

    pub fn refresh_provider(mut self: Pin<&mut Self>, provider: &QString) {
        let name = provider.to_string();
        let provider_id = match name.to_lowercase().as_str() {
            "claude" | "claude code" => ProviderId::Claude,
            "codex" => ProviderId::Codex,
            "cursor" => ProviderId::Cursor,
            _ => return,
        };

        let usage_service = Arc::clone(&self.rust().usage_service);
        let res = futures::executor::block_on(usage_service.refresh_provider(&provider_id));

        let (is_active, usage, remaining, status_str) = match res {
            Ok(snap) => {
                let active = snap.session_active;
                let u = snap.usage_percent.map(|v| v as f64).unwrap_or(0.0);
                let r = snap.remaining_percent.map(|v| v as f64).unwrap_or(1.0 - u);
                let st = match snap.status {
                    ProviderStatus::Available if active => "Active".to_string(),
                    ProviderStatus::Available => "Available".to_string(),
                    ProviderStatus::Unavailable => "Unavailable".to_string(),
                    ProviderStatus::Authenticating => "Authenticating".to_string(),
                    ProviderStatus::Error => "Error".to_string(),
                    ProviderStatus::Unknown => "Unknown".to_string(),
                };
                (active, u, r, st)
            }
            Err(err) => (false, 0.0, 1.0, err.to_string()),
        };

        let q_status = QString::from(status_str.as_str());

        match provider_id {
            ProviderId::Claude => {
                self.as_mut().set_claude_active(is_active);
                self.as_mut().set_claude_usage(usage);
                self.as_mut().set_claude_remaining(remaining);
                self.as_mut().set_claude_status(q_status);
            }
            ProviderId::Codex => {
                self.as_mut().set_codex_active(is_active);
                self.as_mut().set_codex_usage(usage);
                self.as_mut().set_codex_remaining(remaining);
                self.as_mut().set_codex_status(q_status);
            }
            ProviderId::Cursor => {
                self.as_mut().set_cursor_active(is_active);
                self.as_mut().set_cursor_usage(usage);
                self.as_mut().set_cursor_remaining(remaining);
                self.as_mut().set_cursor_status(q_status);
            }
            _ => {}
        }
    }
}
