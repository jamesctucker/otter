//! Otter daemon - orchestration entrypoint.
//!
//! This crate owns session state, context assembly, policy,
//! provider routing, tool supervision, and streaming.
//!
//! NOTE: Tracing subscriber initialization belongs in the CLI binary,
//! not here. Library/daemon code should only emit traces, not configure sinks.

pub mod config;
mod daemon;
mod session;
mod supervisor;

pub use config::OtterConfig;
pub use daemon::Daemon;
pub use session::Session;
pub use supervisor::ModelSupervisor;
