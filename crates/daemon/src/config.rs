//! Typed configuration for Phase 1 local model control.
//!
//! Config lives in the daemon crate because it is a runtime wiring concern,
//! not a domain type. The daemon owns process lifecycle, provider routing,
//! and server binding – all of which are driven by these values.

use std::net::SocketAddr;

/// Runtime configuration for the Otter daemon.
#[derive(Debug, Clone)]
pub struct OtterConfig {
    /// Address the daemon HTTP server binds to.
    pub daemon_bind_addr: SocketAddr,

    /// Base URL of the llama.cpp HTTP server.
    pub llamacpp_base_url: String,

    /// Path to the llama.cpp server binary.
    ///
    /// When `Some`, the daemon may manage the llama.cpp process lifecycle.
    /// When `None`, the daemon expects an externally managed llama.cpp server
    /// already running at `llamacpp_base_url`.
    pub llamacpp_binary_path: Option<String>,

    /// Path to the directory containing model files.
    ///
    /// Used when the daemon manages the llama.cpp process to pass `-m` flags.
    pub model_path: Option<String>,

    /// Default model ID to use when none is specified in a request.
    pub default_model_id: Option<String>,
}

impl Default for OtterConfig {
    fn default() -> Self {
        Self {
            daemon_bind_addr: "127.0.0.1:8910".parse().unwrap(),
            llamacpp_base_url: "http://127.0.0.1:8080".to_string(),
            llamacpp_binary_path: None,
            model_path: None,
            default_model_id: None,
        }
    }
}

impl OtterConfig {
    /// Create a new config with all fields set explicitly.
    pub fn new(
        daemon_bind_addr: SocketAddr,
        llamacpp_base_url: String,
        llamacpp_binary_path: Option<String>,
        model_path: Option<String>,
        default_model_id: Option<String>,
    ) -> Self {
        Self {
            daemon_bind_addr,
            llamacpp_base_url,
            llamacpp_binary_path,
            model_path,
            default_model_id,
        }
    }

    /// Return true if the daemon should manage the llama.cpp process.
    pub fn manages_llamacpp_process(&self) -> bool {
        self.llamacpp_binary_path.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_sensible_values() {
        let config = OtterConfig::default();

        assert_eq!(
            config.daemon_bind_addr,
            "127.0.0.1:8910".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(config.llamacpp_base_url, "http://127.0.0.1:8080");
        assert!(config.llamacpp_binary_path.is_none());
        assert!(config.model_path.is_none());
        assert!(config.default_model_id.is_none());
        assert!(!config.manages_llamacpp_process());
    }

    #[test]
    fn manages_process_when_binary_path_set() {
        let config = OtterConfig {
            llamacpp_binary_path: Some("/usr/local/bin/llama-server".to_string()),
            ..Default::default()
        };

        assert!(config.manages_llamacpp_process());
    }
}
