//! Core traits for provider abstractions.

use crate::{InferenceRequest, InferenceStream, ModelId, ModelInfo, Provider, ProviderError};
use async_trait::async_trait;

/// Trait for model providers.
///
/// Each provider crate implements this trait to adapt a specific inference
/// backend (llama.cpp, OpenAI, etc.) to Otter's shared contract.
#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// List available models.
    async fn list_models(&self) -> Result<Vec<ModelInfo>, ProviderError>;

    /// Check if a model is loaded and ready for inference.
    async fn is_ready(&self, model_id: &ModelId) -> Result<bool, ProviderError>;

    /// Start a streaming completion.
    ///
    /// Returns a stream of `InferenceEvent`s. The stream ends with
    /// `InferenceEvent::Done` on success or `InferenceEvent::Error` on failure.
    /// Callers should also handle `Err` at the stream-item level for
    /// transport errors.
    async fn complete(&self, request: InferenceRequest) -> Result<InferenceStream, ProviderError>;

    /// Get the provider type.
    fn provider_type(&self) -> Provider;
}
