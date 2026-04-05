//! Domain types shared across crates.

use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::ProviderError;

/// Unique identifier for a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// Unique identifier for a message within a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub String);

/// Unique identifier for a model instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub String);

/// Response mode for agent output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseMode {
    Ack,
    Plan,
    Act,
    Result,
}

/// Provider type for model routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    LlamaCpp,
    OpenAI,
}

/// Role of a message in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub role: MessageRole,
    pub content: String,
    // TODO: Add optional tool_call_id, tool_calls fields when tools crate lands.
}

/// Declared capabilities of a model.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub streaming: bool,
    pub tool_use: bool,
    pub json_mode: bool,
}

/// Model metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: ModelId,
    pub name: String,
    pub provider: Provider,
    pub context_length: usize,
    pub capabilities: ModelCapabilities,
}

/// A request for model inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub model_id: ModelId,
    pub messages: Vec<Message>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub response_mode: Option<ResponseMode>,
    pub stream: bool,
}

/// A single event in a streaming inference response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InferenceEvent {
    /// A chunk of generated text.
    Token { delta: String },

    /// Inference finished.
    Done { usage: InferenceUsage },

    /// Provider-level error mid-stream.
    Error { message: String },
}

/// Token usage for an inference call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InferenceUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
}

/// Persistable session data.
///
/// This is the domain representation of a session suitable for storage
/// and cross-crate exchange. Runtime-only state (active model handle,
/// tool budget, etc.) lives in `otter-daemon::Session`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: SessionId,
    pub messages: Vec<Message>,
    pub response_mode: ResponseMode,
    // TODO: Add created_at, updated_at, summary, context_chunks when store is wired.
}

/// A pinned, boxed, Send stream of inference events.
///
/// This is the return type for streaming completions from any provider.
pub type InferenceStream =
    Pin<Box<dyn futures_core::Stream<Item = Result<InferenceEvent, ProviderError>> + Send>>;

// --- Constructors and defaults ---

impl SessionId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl MessageId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl InferenceUsage {
    pub fn total(&self) -> usize {
        self.prompt_tokens + self.completion_tokens
    }
}
