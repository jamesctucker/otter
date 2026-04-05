//! Otter daemon - orchestration entrypoint.
//!
//! This crate owns session state, context assembly, policy,
//! provider routing, tool supervision, and streaming.
//!
//! TODO: Implement full runtime behavior.

mod daemon;
mod session;
mod supervisor;

pub use daemon::Daemon;
pub use session::Session;
pub use supervisor::ModelSupervisor;

pub fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("otter daemon starting");

    // TODO: Implement actual daemon startup
    // - Load configuration
    // - Initialize model supervisor
    // - Start HTTP server
    // - Accept client connections

    Ok(())
}
