use std::env;

use alloy_chains::NamedChain;
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
    /// Base URL of the remote proving queue.
    pub queue_base_url: String,
    /// Auth token for the remote proving queue.
    pub queue_auth_token: String,
}

impl E2eConfig {
    /// Read the config from the environment.
    ///
    /// `E2E_CHAIN_ID` selects the chain (defaults to Base Sepolia);
    /// `QUEUE_BASE_URL` and `QUEUE_AUTH_TOKEN` configure the proving queue.
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

        Ok(Self {
            chain,
            queue_base_url: env::var("QUEUE_BASE_URL").context("failed to read QUEUE_BASE_URL")?,
            queue_auth_token: env::var("QUEUE_AUTH_TOKEN")
                .context("failed to read QUEUE_AUTH_TOKEN")?,
        })
    }
}
