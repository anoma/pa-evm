use alloy::transports::http::reqwest::Url;
use alloy_chains::NamedChain;
use std::env;
use thiserror::Error;

pub type RpcUrlResult<T> = Result<T, RpcUrlError>;

#[derive(Error, Debug, Clone)]
pub enum RpcUrlError {
    #[error("No RPC provider subdomain was found for this chain.")]
    SubdomainNotFound,
    #[error("The RPC provider API key is not set in the environment.")]
    ApiKeyEnvVarNotSet,
    #[error("The url could not be parsed.")]
    UrlParsingError,
}

/// The RPC providers we can build URLs for.
///
/// Each variant knows how to turn a `(subdomain, api_key)` pair into a URL and
/// which environment variable holds its API key. Add a variant here when
/// onboarding a new provider, then wire its chains up in [`rpc_provider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcProvider {
    Alchemy,
    NowNodes,
}

impl RpcProvider {
    /// The environment variable holding this provider's API key.
    pub const fn api_key_env_var(self) -> &'static str {
        match self {
            RpcProvider::Alchemy => "ALCHEMY_API_KEY",
            RpcProvider::NowNodes => "NOWNODES_API_KEY",
        }
    }

    /// Builds the RPC URL for the given subdomain, reading the API key from the
    /// environment.
    fn build_url(self, subdomain: &str) -> RpcUrlResult<Url> {
        let api_key =
            env::var(self.api_key_env_var()).map_err(|_| RpcUrlError::ApiKeyEnvVarNotSet)?;

        let url = match self {
            RpcProvider::Alchemy => format!("https://{subdomain}.g.alchemy.com/v2/{api_key}"),
            RpcProvider::NowNodes => format!("https://{subdomain}.nownodes.io/{api_key}"),
        };

        url.parse().map_err(|_| RpcUrlError::UrlParsingError)
    }
}

/// Returns the RPC provider and subdomain configured for the given chain.
///
/// This is the single place that maps chains to providers; extend it when a new
/// chain is added or moved between providers. Chains served by Alchemy and those
/// served by NowNodes are matched side by side here.
pub fn rpc_provider(chain: &NamedChain) -> RpcUrlResult<(RpcProvider, &'static str)> {
    use NamedChain::*;
    use RpcProvider::*;

    Ok(match chain {
        // Alchemy
        Mainnet => (Alchemy, "eth-mainnet"),
        Sepolia => (Alchemy, "eth-sepolia"),
        //
        Arbitrum => (Alchemy, "arb-mainnet"),
        ArbitrumSepolia => (Alchemy, "arb-sepolia"),
        //
        Optimism => (Alchemy, "opt-mainnet"),
        OptimismSepolia => (Alchemy, "opt-sepolia"),
        //
        Base => (Alchemy, "base-mainnet"),
        BaseSepolia => (Alchemy, "base-sepolia"),
        //
        Polygon => (Alchemy, "polygon-mainnet"),
        PolygonAmoy => (Alchemy, "polygon-amoy"),
        //
        BinanceSmartChain => (Alchemy, "bnb-mainnet"),
        BinanceSmartChainTestnet => (Alchemy, "bnb-testnet"),
        //
        Monad => (Alchemy, "monad-mainnet"),
        MonadTestnet => (Alchemy, "monad-testnet"),
        //
        StableMainnet => (Alchemy, "stable-mainnet"),
        StableTestnet => (Alchemy, "stable-testnet"),
        //
        MegaEth => (Alchemy, "megaeth-mainnet"),
        MegaEthTestnet => (Alchemy, "megaeth-testnet"),

        // NowNodes
        Aurora => (NowNodes, "aurora"),

        _ => return Err(RpcUrlError::SubdomainNotFound),
    })
}

/// Returns the RPC URL for the given chain, using whichever provider is
/// configured for it in [`rpc_provider`].
pub fn rpc_url(chain: &NamedChain) -> RpcUrlResult<Url> {
    dotenvy::dotenv().ok();

    let (provider, subdomain) = rpc_provider(chain)?;
    provider.build_url(subdomain)
}

// ---------------------------------------------------------------------------
// Backwards-compatible API
//
// The helpers below predate [`rpc_url`]/[`rpc_provider`] and are kept so
// existing callers keep compiling. Prefer [`rpc_url`] for new code.
// ---------------------------------------------------------------------------

/// Backwards-compatible alias for [`RpcUrlResult`].
pub type AlchemyResult<T> = RpcUrlResult<T>;
/// Backwards-compatible alias for [`RpcUrlError`].
pub type AlchemyError = RpcUrlError;

/// Returns the Alchemy RPC URL for the given chain.
pub fn alchemy_url(chain: &NamedChain) -> RpcUrlResult<Url> {
    dotenvy::dotenv().ok();
    RpcProvider::Alchemy.build_url(alchemy_subdomain(chain)?)
}

/// Returns the Alchemy subdomain for the given chain.
pub fn alchemy_subdomain(chain: &NamedChain) -> RpcUrlResult<&'static str> {
    match rpc_provider(chain)? {
        (RpcProvider::Alchemy, subdomain) => Ok(subdomain),
        _ => Err(RpcUrlError::SubdomainNotFound),
    }
}

/// Returns the NowNodes RPC URL for the given chain.
pub fn nownodes_url(chain: &NamedChain) -> RpcUrlResult<Url> {
    dotenvy::dotenv().ok();
    RpcProvider::NowNodes.build_url(nownodes_subdomain(chain)?)
}

/// Returns the NowNodes subdomain for the given chain.
pub fn nownodes_subdomain(chain: &NamedChain) -> RpcUrlResult<&'static str> {
    match rpc_provider(chain)? {
        (RpcProvider::NowNodes, subdomain) => Ok(subdomain),
        _ => Err(RpcUrlError::SubdomainNotFound),
    }
}
