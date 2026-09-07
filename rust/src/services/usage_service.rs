use crate::core::errors::ProviderError;
use crate::core::events::AppEvent;
use crate::core::models::{ProviderId, ProviderSnapshot};
use crate::core::state::AppState;
use crate::providers::registry::ProviderRegistry;
use chrono::Utc;
use std::sync::{Arc, RwLock};

pub type EventListener = Box<dyn Fn(&AppEvent) + Send + Sync + 'static>;

/// Orchestrates polling provider metrics, normalizing data, and updating application state.
pub struct UsageService {
    registry: Arc<ProviderRegistry>,
    state: Arc<AppState>,
    listeners: Arc<RwLock<Vec<EventListener>>>,
}

impl UsageService {
    pub fn new(registry: Arc<ProviderRegistry>, state: Arc<AppState>) -> Self {
        Self {
            registry,
            state,
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Attaches an event observer to receive state update events.
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&AppEvent) + Send + Sync + 'static,
    {
        if let Ok(mut list) = self.listeners.write() {
            list.push(Box::new(listener));
        }
    }

    fn dispatch_event(&self, event: AppEvent) {
        if let Ok(listeners) = self.listeners.read() {
            for listener in listeners.iter() {
                listener(&event);
            }
        }
    }

    /// Refreshes a single provider, isolating failures from affecting global application state.
    pub async fn refresh_provider(&self, id: &ProviderId) -> Result<ProviderSnapshot, ProviderError> {
        let provider = match self.registry.get(id) {
            Some(p) => p,
            None => {
                let err = ProviderError::NotInstalled(id.clone());
                self.state.record_error(id, err.to_string());
                self.dispatch_event(AppEvent::ProviderFailed {
                    provider_id: id.clone(),
                    error: err.clone(),
                    timestamp: Utc::now(),
                });
                return Err(err);
            }
        };

        match provider.fetch_usage().await {
            Ok(snapshot) => {
                self.state.update_snapshot(snapshot.clone());
                self.dispatch_event(AppEvent::ProviderUpdated {
                    provider_id: id.clone(),
                    snapshot: snapshot.clone(),
                    timestamp: Utc::now(),
                });
                Ok(snapshot)
            }
            Err(err) => {
                self.state.record_error(id, err.to_string());
                self.dispatch_event(AppEvent::ProviderFailed {
                    provider_id: id.clone(),
                    error: err.clone(),
                    timestamp: Utc::now(),
                });
                Err(err)
            }
        }
    }

    /// Polls all registered providers, collecting individual results without failing the batch.
    pub async fn refresh_all(&self) -> Vec<(ProviderId, Result<ProviderSnapshot, ProviderError>)> {
        let providers = self.registry.all();
        let mut results = Vec::with_capacity(providers.len());

        for provider in providers {
            let id = provider.id();
            let result = self.refresh_provider(&id).await;
            results.push((id, result));
        }

        results
    }

    pub fn state(&self) -> Arc<AppState> {
        Arc::clone(&self.state)
    }

    pub fn registry(&self) -> Arc<ProviderRegistry> {
        Arc::clone(&self.registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_usage_service_refresh() {
        let registry = Arc::new(ProviderRegistry::with_defaults());
        let state = Arc::new(AppState::new());
        let service = UsageService::new(registry, state.clone());

        let event_count = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&event_count);
        service.add_listener(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Block on refresh_provider using futures executor
        let _ = futures::executor::block_on(service.refresh_provider(&ProviderId::Claude));
        assert_eq!(event_count.load(Ordering::SeqCst), 1);
    }
}
