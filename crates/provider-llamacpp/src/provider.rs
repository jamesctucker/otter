//! llama.cpp provider implementation.

use async_trait::async_trait;
use otter_core::{ModelId, ModelInfo, ModelProvider, Provider, ProviderError};

/// Provider adapter for llama.cpp HTTP server.
pub struct LlamaCppProvider {
    // TODO: HTTP client, base URL, model registry
}

impl LlamaCppProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn with_base_url(_base_url: String) -> Self {
        // TODO: Configure HTTP client with base URL
        Self {}
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
        // TODO: Query llama.cpp server, map to ModelInfo
        Ok(Vec::new())
    }

    async fn is_ready(&self, _model_id: &ModelId) -> bool {
        // TODO: Ping model endpoint, check health
        false
    }

    fn provider_type(&self) -> Provider {
        Provider::LlamaCpp
    }
}
