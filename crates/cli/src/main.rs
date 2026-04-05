//! Otter CLI - command-line interface for the daemon.

use anyhow::Result;
use otter_daemon::OtterConfig;
use otter_provider_llamacpp::LlamaCppProvider;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = OtterConfig::default();

    tracing::info!(
        bind_addr = %config.daemon_bind_addr,
        llamacpp_url = %config.llamacpp_base_url,
        "otter starting"
    );

    let provider = LlamaCppProvider::with_base_url(config.llamacpp_base_url.clone());

    // Health check
    match provider.health_check().await {
        Ok(true) => println!("llama.cpp server: healthy"),
        Ok(false) => println!("llama.cpp server: not ready (server may be down)"),
        Err(e) => println!("llama.cpp health check error: {e}"),
    }

    // List models
    match provider.list_models_raw().await {
        Ok(models) if models.is_empty() => {
            println!("models: none reported (server may be down or no models loaded)")
        }
        Ok(models) => {
            println!("models:");
            for m in &models {
                println!("  - {} ({})", m.name, m.id.0);
            }
        }
        Err(e) => println!("models list error: {e}"),
    }

    Ok(())
}
