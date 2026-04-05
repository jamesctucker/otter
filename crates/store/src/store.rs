//! SQLite-based persistence store.

use otter_core::SessionId;

/// SQLite-backed store for Otter.
pub struct Store {
    // TODO: SQLite connection pool, database path
}

impl Store {
    pub async fn new(_path: &str) -> anyhow::Result<Self> {
        // TODO: Open/create database, run migrations, init pool
        Ok(Self {})
    }

    pub async fn save_session(&self, _session_id: &SessionId) -> anyhow::Result<()> {
        // TODO: Persist session
        Ok(())
    }

    pub async fn load_session(&self, _session_id: &SessionId) -> anyhow::Result<()> {
        // TODO: Load session
        Ok(())
    }

    pub async fn list_sessions(&self) -> anyhow::Result<Vec<SessionId>> {
        // TODO: List sessions
        Ok(Vec::new())
    }
}
