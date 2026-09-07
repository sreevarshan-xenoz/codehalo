use super::models::{EdgePosition, ProviderId, ProviderSnapshot, ProviderStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

/// Full state record for a single provider, tracking historical stability and errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderState {
    pub snapshot: ProviderSnapshot,
    pub last_successful_fetch: Option<DateTime<Utc>>,
    pub consecutive_failures: u32,
    pub last_error: Option<String>,
}

impl ProviderState {
    pub fn new(provider: ProviderId) -> Self {
        Self {
            snapshot: ProviderSnapshot::unavailable(provider),
            last_successful_fetch: None,
            consecutive_failures: 0,
            last_error: None,
        }
    }

    pub fn apply_snapshot(&mut self, snapshot: ProviderSnapshot) {
        if snapshot.status.is_operational() {
            self.last_successful_fetch = Some(snapshot.updated_at);
            self.consecutive_failures = 0;
            self.last_error = None;
        }
        self.snapshot = snapshot;
    }

    pub fn apply_error(&mut self, error: String) {
        self.consecutive_failures += 1;
        self.last_error = Some(error);
        self.snapshot.status = ProviderStatus::Error;
        self.snapshot.updated_at = Utc::now();
    }
}

/// Global synchronized state container for CodeHalo.
#[derive(Debug, Clone)]
pub struct AppState {
    providers: Arc<RwLock<HashMap<ProviderId, ProviderState>>>,
    pub current_edge: Arc<Mutex<EdgePosition>>,
    pub monitor_index: Arc<Mutex<usize>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            current_edge: Arc::new(Mutex::new(EdgePosition::Top)),
            monitor_index: Arc::new(Mutex::new(0)),
        }
    }

    pub fn update_snapshot(&self, snapshot: ProviderSnapshot) {
        if let Ok(mut map) = self.providers.write() {
            let entry = map
                .entry(snapshot.provider.clone())
                .or_insert_with(|| ProviderState::new(snapshot.provider.clone()));
            entry.apply_snapshot(snapshot);
        }
    }

    pub fn record_error(&self, provider_id: &ProviderId, error_msg: String) {
        if let Ok(mut map) = self.providers.write() {
            let entry = map
                .entry(provider_id.clone())
                .or_insert_with(|| ProviderState::new(provider_id.clone()));
            entry.apply_error(error_msg);
        }
    }

    pub fn get_snapshot(&self, provider_id: &ProviderId) -> Option<ProviderSnapshot> {
        self.providers
            .read()
            .ok()
            .and_then(|map| map.get(provider_id).map(|s| s.snapshot.clone()))
    }

    pub fn all_snapshots(&self) -> Vec<ProviderSnapshot> {
        self.providers
            .read()
            .map(|map| map.values().map(|s| s.snapshot.clone()).collect())
            .unwrap_or_default()
    }

    pub fn provider_state(&self, provider_id: &ProviderId) -> Option<ProviderState> {
        self.providers
            .read()
            .ok()
            .and_then(|map| map.get(provider_id).cloned())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::UsageSource;

    #[test]
    fn test_app_state_lifecycle() {
        let state = AppState::new();
        assert!(state.all_snapshots().is_empty());

        let snapshot = ProviderSnapshot::available(ProviderId::Claude, Some(0.42), UsageSource::Local);
        state.update_snapshot(snapshot.clone());

        let retrieved = state.get_snapshot(&ProviderId::Claude).expect("snapshot should exist");
        assert_eq!(retrieved.usage_percent, Some(0.42));
        assert_eq!(retrieved.status, ProviderStatus::Available);

        state.record_error(&ProviderId::Claude, "Connection timed out".to_string());
        let err_snap = state.get_snapshot(&ProviderId::Claude).expect("snapshot should exist");
        assert_eq!(err_snap.status, ProviderStatus::Error);

        let prov_state = state.provider_state(&ProviderId::Claude).expect("state should exist");
        assert_eq!(prov_state.consecutive_failures, 1);
        assert_eq!(prov_state.last_error.as_deref(), Some("Connection timed out"));

        // Recovery on subsequent successful snapshot
        state.update_snapshot(snapshot);
        let recovered = state.provider_state(&ProviderId::Claude).expect("state should exist");
        assert_eq!(recovered.consecutive_failures, 0);
        assert!(recovered.last_error.is_none());
        assert_eq!(recovered.snapshot.status, ProviderStatus::Available);
    }
}
