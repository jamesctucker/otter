//! Persistence layer for Otter.
//!
//! This crate handles SQLite storage for sessions, messages,
//! memories, and other persistent state.
//!
//! TODO: Implement full persistence layer.

mod schema;
mod store;

pub use store::Store;
