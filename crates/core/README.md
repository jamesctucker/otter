# otter-core

Shared domain types, traits, and events for the Otter runtime.

This crate does not own orchestration or runtime behavior. It provides:
- Domain types (SessionId, MessageId, ModelId, etc.)
- Core traits (ModelProvider)
- Event types for daemon-to-client communication
