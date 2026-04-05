//! Otter CLI - command-line interface for the daemon.

use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // TODO: Parse CLI args (start, stop, status, chat)
    // TODO: Connect to daemon or start embedded daemon

    otter_daemon::run()
}
