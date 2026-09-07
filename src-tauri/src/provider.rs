use async_trait::async_trait;
use crate::models::ProviderUsageSnapshot;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Authentication failed or token missing")]
    AuthError,
    #[error("Rate limit reached: {0}")]
    RateLimited(String),
    #[error("Network failure: {0}")]
    Network(String),
    #[error("Provider CLI/Local parse error: {0}")]
    ParseError(String),
}

#[async_trait]
pub trait UsageProvider: Send + Sync {
    /// Identifier e.g. "claude", "cursor", "codex"
    fn id(&self) -> &'static str;
    
    /// Display name
    fn name(&self) -> &'static str;

    /// Icon identifier
    fn icon(&self) -> &'static str;

    /// Fetch latest usage stats
    async fn get_usage(&self) -> Result<ProviderUsageSnapshot, ProviderError>;
}
