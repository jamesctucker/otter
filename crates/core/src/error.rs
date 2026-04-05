//! Error types shared across provider and daemon crates.

/// Errors from model providers.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("model not found: {0}")]
    ModelNotFound(String),

    #[error("model not ready: {0}")]
    ModelNotReady(String),

    #[error("request failed: {0}")]
    RequestFailed(String),

    #[error("streaming error: {0}")]
    StreamingError(String),

    #[error("configuration error: {0}")]
    ConfigError(String),
}
