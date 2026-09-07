use crate::core::errors::ProviderError;
use crate::core::models::{ProviderId, ProviderSnapshot};
use std::future::Future;
use std::pin::Pin;

/// Convenient type alias for boxed, thread-safe asynchronous operations.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Core contract implemented by all AI coding assistant adapters.
pub trait UsageProvider: Send + Sync {
    /// Canonical provider identity.
    fn id(&self) -> ProviderId;

    /// Human-readable brand name.
    fn name(&self) -> &'static str;

    /// Probes the local machine to determine if this provider is installed or usable.
    fn is_available(&self) -> BoxFuture<'_, bool>;

    /// Fetches normalized point-in-time usage information.
    fn fetch_usage(&self) -> BoxFuture<'_, Result<ProviderSnapshot, ProviderError>>;
}
