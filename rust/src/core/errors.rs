use thiserror::Error;
use super::models::ProviderId;

/// Comprehensive error taxonomy for provider discovery, authentication, and usage fetching.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProviderError {
    #[error("Provider '{0}' is not installed or available on the system")]
    NotInstalled(ProviderId),

    #[error("Authentication required or credentials missing for '{0}'")]
    NotAuthenticated(ProviderId),

    #[error("Authentication session for '{0}' has expired: {1}")]
    SessionExpired(ProviderId, String),

    #[error("Rate limited by provider '{0}': {1}")]
    RateLimited(ProviderId, String),

    #[error("Local session or data file not found for '{0}': {1}")]
    FileNotFound(ProviderId, String),

    #[error("Failed to parse usage data for '{0}': {1}")]
    ParseError(ProviderId, String),

    #[error("I/O error while reading provider state for '{0}': {1}")]
    IoError(ProviderId, String),

    #[error("Network or API communication failure for '{0}': {1}")]
    NetworkError(ProviderId, String),

    #[error("Provider '{0}' is temporarily unavailable: {1}")]
    Unavailable(ProviderId, String),

    #[error("Internal error in provider '{0}': {1}")]
    Internal(ProviderId, String),
}
