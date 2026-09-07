use crate::core::errors::ProviderError;
use crate::core::models::{ProviderId, ProviderSnapshot};
use crate::providers::claude::ClaudeProvider;
use crate::providers::codex::CodexProvider;
use crate::providers::cursor::CursorProvider;
use crate::providers::traits::UsageProvider;
use std::collections::HashMap;
use std::sync::Arc;

/// Single authority managing all installed and registered AI usage providers.
#[derive(Default, Clone)]
pub struct ProviderRegistry {
    providers: HashMap<ProviderId, Arc<dyn UsageProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Creates a registry pre-populated with default supported providers (Claude, Codex, Cursor).
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register(Arc::new(ClaudeProvider::new()));
        registry.register(Arc::new(CodexProvider::new()));
        registry.register(Arc::new(CursorProvider::new()));
        registry
    }

    /// Registers a provider adapter into the registry.
    pub fn register(&mut self, provider: Arc<dyn UsageProvider>) {
        self.providers.insert(provider.id(), provider);
    }

    /// Retrieves a provider by identity.
    pub fn get(&self, id: &ProviderId) -> Option<Arc<dyn UsageProvider>> {
        self.providers.get(id).cloned()
    }

    /// Returns a list of all registered providers.
    pub fn all(&self) -> Vec<Arc<dyn UsageProvider>> {
        self.providers.values().cloned().collect()
    }

    /// Number of registered providers.
    pub fn count(&self) -> usize {
        self.providers.len()
    }

    /// Checks the availability of all registered providers.
    pub async fn check_all_availability(&self) -> Vec<(ProviderId, bool)> {
        let mut results = Vec::with_capacity(self.providers.len());
        for provider in self.providers.values() {
            let available = provider.is_available().await;
            results.push((provider.id(), available));
        }
        results
    }

    /// Polls usage from all registered providers.
    pub async fn fetch_all(&self) -> Vec<Result<ProviderSnapshot, ProviderError>> {
        let mut results = Vec::with_capacity(self.providers.len());
        for provider in self.providers.values() {
            results.push(provider.fetch_usage().await);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_with_defaults() {
        let registry = ProviderRegistry::with_defaults();
        assert_eq!(registry.count(), 3);
        assert!(registry.get(&ProviderId::Claude).is_some());
        assert!(registry.get(&ProviderId::Codex).is_some());
        assert!(registry.get(&ProviderId::Cursor).is_some());
        assert!(registry.get(&ProviderId::Gemini).is_none());
    }

    #[test]
    fn test_custom_provider_registration() {
        let mut registry = ProviderRegistry::new();
        assert_eq!(registry.count(), 0);

        registry.register(Arc::new(ClaudeProvider::new()));
        assert_eq!(registry.count(), 1);
        assert_eq!(registry.get(&ProviderId::Claude).unwrap().name(), "Claude Code");
    }
}
