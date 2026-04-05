//! Session management.
//!
//! This struct holds runtime-only session state (active model handle,
//! tool budget, streaming state, etc.). Persistable session data should
//! live in `otter_core::SessionRecord`.
//!
//! TODO: When store is wired, load/save should round-trip through
//! `SessionRecord` and the daemon should own the conversion between
//! `SessionRecord` (persistence) and `Session` (runtime).

use otter_core::{MessageId, ResponseMode, SessionId};

/// A live coding session. Runtime state only.
pub struct Session {
    pub id: SessionId,
    pub response_mode: ResponseMode,
    // TODO: Message history, context state, active model, tool budget
}

impl Session {
    pub fn new() -> Self {
        Self {
            id: SessionId::new(),
            response_mode: ResponseMode::Act,
        }
    }

    pub async fn add_message(&mut self, _content: String) -> MessageId {
        // TODO: Implement message handling
        MessageId::new()
    }

    pub fn set_response_mode(&mut self, mode: ResponseMode) {
        self.response_mode = mode;
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
