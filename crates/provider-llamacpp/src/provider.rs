//! llama.cpp provider implementation.

use async_trait::async_trait;
use otter_core::{
    InferenceRequest, InferenceStream, ModelId, ModelInfo, ModelProvider, Provider, ProviderError,
};
use reqwest::Client;
use serde::Deserialize;

/// Provider adapter for llama.cpp HTTP server.
pub struct LlamaCppProvider {
    client: Client,
    base_url: String,
}

/// Response from llama.cpp `/health` endpoint.
#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

/// Response from llama.cpp `/v1/models` endpoint.
#[derive(Debug, Deserialize)]
struct ModelsResponse {
    data: Vec<ModelEntry>,
}

/// A single model entry from llama.cpp's model listing.
#[derive(Debug, Deserialize)]
struct ModelEntry {
    id: String,
}

impl LlamaCppProvider {
    /// Create a new provider with default settings.
    ///
    /// Uses `http://127.0.0.1:8080` as the base URL.
    pub fn new() -> Self {
        Self::with_base_url("http://127.0.0.1:8080".to_string())
    }

    /// Create a provider pointing at the given llama.cpp server URL.
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Check llama.cpp server health via the `/health` endpoint.
    ///
    /// Returns `true` if the server responds with status `"ok"`.
    pub async fn health_check(&self) -> Result<bool, ProviderError> {
        let url = format!("{}/health", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::RequestFailed(format!("health check failed: {e}")))?;

        let status = resp.status();
        if !status.is_success() {
            tracing::warn!(
                status_code = %status,
                url = %url,
                "llama.cpp health check returned non-2xx"
            );
            return Ok(false);
        }

        let body: HealthResponse = resp
            .json()
            .await
            .map_err(|e| ProviderError::RequestFailed(format!("health parse error: {e}")))?;

        Ok(body.status == "ok")
    }

    /// List models from llama.cpp via the `/v1/models` endpoint.
    ///
    /// Maps each model to Otter's `ModelInfo` with minimal capability detection.
    /// TODO: Enrich capabilities once we can query model metadata more deeply.
    pub async fn list_models_raw(&self) -> Result<Vec<ModelInfo>, ProviderError> {
        let url = format!("{}/v1/models", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::RequestFailed(format!("list models failed: {e}")))?;

        let status = resp.status();
        if !status.is_success() {
            tracing::warn!(
                status_code = %status,
                url = %url,
                "llama.cpp list models returned non-2xx"
            );
            return Ok(Vec::new());
        }

        let body: ModelsResponse = resp
            .json()
            .await
            .map_err(|e| ProviderError::RequestFailed(format!("models parse error: {e}")))?;

        let models = body
            .data
            .into_iter()
            .map(|entry| ModelInfo {
                id: ModelId(entry.id.clone()),
                name: entry.id,
                provider: Provider::LlamaCpp,
                // TODO: Query actual context length from llama.cpp server metadata.
                context_length: 0,
                // TODO: Detect actual capabilities once we have richer metadata.
                capabilities: Default::default(),
            })
            .collect();

        Ok(models)
    }
}

impl Default for LlamaCppProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModelProvider for LlamaCppProvider {
    async fn list_models(&self) -> Result<Vec<ModelInfo>, ProviderError> {
        self.list_models_raw().await
    }

    async fn is_ready(&self, _model_id: &ModelId) -> Result<bool, ProviderError> {
        self.health_check().await
    }

    async fn complete(&self, _request: InferenceRequest) -> Result<InferenceStream, ProviderError> {
        // TODO: POST to llama.cpp /v1/chat/completions, parse SSE stream, yield InferenceEvents
        Err(ProviderError::RequestFailed("not implemented".to_string()))
    }

    fn provider_type(&self) -> Provider {
        Provider::LlamaCpp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_constructs_with_default_url() {
        let provider = LlamaCppProvider::new();
        assert_eq!(provider.base_url, "http://127.0.0.1:8080");
    }

    #[test]
    fn provider_constructs_with_custom_url() {
        let provider = LlamaCppProvider::with_base_url("http://localhost:9999".to_string());
        assert_eq!(provider.base_url, "http://localhost:9999");
    }

    #[test]
    fn provider_type_is_llamacpp() {
        let provider = LlamaCppProvider::new();
        assert_eq!(provider.provider_type(), Provider::LlamaCpp);
    }

    #[tokio::test]
    async fn health_check_returns_false_when_server_unreachable() {
        let provider = LlamaCppProvider::with_base_url("http://127.0.0.1:1".to_string());
        // Should not panic; returns false or error when server is down.
        let result = provider.health_check().await;
        // Either an error (connection refused) or false is acceptable.
        assert!(result.is_err() || !result.unwrap());
    }

    #[tokio::test]
    async fn list_models_returns_empty_when_server_unreachable() {
        let provider = LlamaCppProvider::with_base_url("http://127.0.0.1:1".to_string());
        // Should not panic; returns empty or error when server is down.
        let result = provider.list_models_raw().await;
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
