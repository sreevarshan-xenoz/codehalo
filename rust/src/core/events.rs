use super::models::{ProviderId, ProviderSnapshot};
use super::errors::ProviderError;
use chrono::{DateTime, Utc};

/// Event emissions dispatched on state alterations.
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    ProviderUpdated {
        provider_id: ProviderId,
        snapshot: ProviderSnapshot,
        timestamp: DateTime<Utc>,
    },
    ProviderFailed {
        provider_id: ProviderId,
        error: ProviderError,
        timestamp: DateTime<Utc>,
    },
    ProviderDiscovered {
        provider_id: ProviderId,
        timestamp: DateTime<Utc>,
    },
}
