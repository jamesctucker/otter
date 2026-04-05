//! Core domain types, traits, and events for Otter.
//!
//! This crate contains shared abstractions used across the daemon,
//! providers, and tools. It does not own orchestration or runtime behavior.

mod error;
mod events;
mod traits;
mod types;

pub use error::*;
pub use events::*;
pub use traits::*;
pub use types::*;

#[cfg(test)]
mod tests;
