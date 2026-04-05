//! Model lifecycle supervisor.
//!
//! Responsible for:
//! - Discovering configured models
//! - Starting and stopping llama.cpp processes
//! - Warming and cooling models
//! - Tracking ports, PIDs, health, and lease state
//!
//! # Dependency direction
//!
//! `ModelSupervisor` is daemon-owned. It depends on provider crates
//! (e.g. `otter-provider-llamacpp`) to create provider instances after
//! spawning a backend process. Provider crates must never depend on the
//! supervisor or the daemon. The flow is:
//!
//! ```text
//! daemon::ModelSupervisor
//!   -> spawns llama.cpp process
//!   -> constructs LlamaCppProvider(base_url)
//!   -> hands provider to daemon routing layer
//! ```

use otter_core::{ModelId, ModelInfo};

/// Model supervisor for local model lifecycle.
pub struct ModelSupervisor {
    // TODO: Model registry, active processes, health state, lease tracking
}

impl ModelSupervisor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn discover_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        // TODO: Scan model directories, parse metadata, return available models
        Ok(Vec::new())
    }

    pub async fn load_model(&self, _model_id: &ModelId) -> anyhow::Result<()> {
        // TODO: Check if loaded, start llama.cpp, wait for health, register lease
        Ok(())
    }

    pub async fn unload_model(&self, _model_id: &ModelId) -> anyhow::Result<()> {
        // TODO: Check lease, stop process, cleanup
        Ok(())
    }

    pub async fn warm_model(&self, _model_id: &ModelId) -> anyhow::Result<()> {
        // TODO: Preload model, run warmup prompt
        Ok(())
    }
}

impl Default for ModelSupervisor {
    fn default() -> Self {
        Self::new()
    }
}
