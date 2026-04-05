//! Main daemon orchestration.

use otter_core::SessionId;
use std::sync::Arc;
use tokio::sync::RwLock;

/// The main daemon struct.
pub struct Daemon {
    // TODO: Add fields for model supervisor, session store, policy engine, HTTP server
    _sessions: Arc<RwLock<Vec<SessionId>>>,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            _sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        // TODO: Initialize model supervisor, start HTTP server, accept connections
        Ok(())
    }

    pub async fn shutdown(&self) -> anyhow::Result<()> {
        // TODO: Stop accepting connections, drain sessions, stop supervisor
        Ok(())
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self::new()
    }
}
