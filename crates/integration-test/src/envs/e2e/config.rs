use std::env;

use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::Environment;
use anyhow::Context;

/// Configuration for the e2e environment, read from the process environment.
///
/// Selects the chain to fork (and read deployed addresses for) and the proving
/// queue to submit to. The fork RPC and the per-chain protocol-adapter address
/// are resolved from `anoma-pa-evm-bindings`; only the queue connection lives
/// here.
pub struct E2eConfig {
    /// Chain to fork and whose deployed contracts the test targets.
    pub chain: NamedChain,
    /// Deployment environment whose proxy the test targets.
    pub environment: Environment,
    /// Base URL of the remote proving queue.
    pub queue_base_url: String,
    /// Auth token for the remote proving queue.
    pub queue_auth_token: String,
}

impl E2eConfig {
    /// Read the config from the environment.
    ///
    /// `E2E_CHAIN_ID` selects the chain (defaults to Base Sepolia);
    /// `E2E_ENVIRONMENT` selects the deployment environment (defaults to
    /// staging), which a promotion into `main` sets to production so the tests
    /// exercise the deployment being promoted; `QUEUE_BASE_URL` and
    /// `QUEUE_AUTH_TOKEN` configure the proving queue.
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let chain = match env::var("E2E_CHAIN_ID") {
            Ok(raw) => {
                let chain_id: u64 = raw
                    .parse()
                    .context("E2E_CHAIN_ID must be a numeric chain id")?;
                NamedChain::try_from(chain_id)
                    .map_err(|_| anyhow::anyhow!("unsupported E2E_CHAIN_ID {chain_id}"))?
            }
            Err(_) => NamedChain::BaseSepolia,
        };

        let environment = match env::var("E2E_ENVIRONMENT") {
            Ok(raw) => match raw.as_str() {
                "staging" => Environment::Staging,
                "production" => Environment::Production,
                other => {
                    anyhow::bail!("E2E_ENVIRONMENT must be staging or production, got {other}")
                }
            },
            Err(_) => Environment::Staging,
        };

        Ok(Self {
            chain,
            environment,
            queue_base_url: env::var("QUEUE_BASE_URL").context("failed to read QUEUE_BASE_URL")?,
            queue_auth_token: env::var("QUEUE_AUTH_TOKEN")
                .context("failed to read QUEUE_AUTH_TOKEN")?,
        })
    }
}
