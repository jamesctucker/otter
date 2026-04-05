//! Event types for daemon-to-client communication.

use crate::{MessageId, ModelId, ResponseMode, SessionId};
use serde::{Deserialize, Serialize};

/// Events streamed from daemon to clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonEvent {
    /// Session started.
    SessionStarted { session_id: SessionId },

    /// Token received from model.
    TokenReceived {
        message_id: MessageId,
        token: String,
    },

    /// Message completed.
    MessageCompleted { message_id: MessageId },

    /// Tool call requested.
    ToolCallRequested {
        tool_name: String,
        params: serde_json::Value,
    },

    /// Tool call completed.
    ToolCallCompleted {
        tool_name: String,
        result: serde_json::Value,
    },

    /// Model switched.
    ModelSwitched { model_id: ModelId },

    /// Response mode changed.
    ResponseModeChanged { mode: ResponseMode },

    /// Error occurred.
    Error { message: String },

    /// Session ended.
    SessionEnded { session_id: SessionId },
}
