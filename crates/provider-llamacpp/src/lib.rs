//! llama.cpp provider for Otter.
//!
//! This crate adapts llama.cpp's HTTP server to Otter's provider interface.
//!
//! TODO: Implement full provider functionality.

mod provider;

pub use provider::LlamaCppProvider;
