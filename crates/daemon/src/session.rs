//! Session management.

use otter_core::{MessageId, ResponseMode, SessionId};

/// A coding session.
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
