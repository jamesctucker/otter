//! Core traits for provider and tool abstractions.

use crate::{ModelId, ModelInfo, Provider};
use async_trait::async_trait;

/// Trait for model providers.
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// List available models.
    async fn list_models(&self) -> Result<Vec<ModelInfo>, ProviderError>;

    /// Check if a model is ready.
    async fn is_ready(&self, model_id: &ModelId) -> bool;

    /// Get the provider type.
    fn provider_type(&self) -> Provider;
}

/// Errors from model providers.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Model not ready: {0}")]
    ModelNotReady(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Streaming error: {0}")]
    StreamingError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
