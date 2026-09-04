//! Checks of the recorded deployments against the chains they run on. They run on the promotion gate: the
//! `VERIFY_*` flags arm them, because between a version bump on `next` and the environment's upgrade the source
//! and the deployments legitimately disagree.

mod common;

use alloy::primitives::{Address, keccak256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::sol;
use alloy::sol_types::SolConstructor;
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::{Environment, protocol_adapter_deployments_map};
use anoma_pa_evm_bindings::contract::protocol_adapter;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter;
use anoma_pa_evm_bindings::helpers::alchemy_url;
use common::{
    CREATE2_DEPLOYER, IMPLEMENTATION_SALT, bytes32, context, is_armed, is_release,
    is_release_candidate,
};

sol! {
    /// The owner set a Safe links through `1 <= threshold <= owners.length`, which identifies one.
    #[sol(rpc)]
    interface IOwnerManager {
        function getOwners() external view returns (address[] memory);
        function getThreshold() external view returns (uint256);
    }
}

#[tokio::test]
async fn staging_deployments_run_the_source_implementation() {
    expect_source_implementations(Environment::Staging).await;
}

#[tokio::test]
async fn production_deployments_run_the_source_implementation() {
    expect_source_implementations(Environment::Production).await;
}

#[tokio::test]
async fn staging_deployments_run_a_release_or_release_candidate_version() {
    if !is_armed(Environment::Staging) {
        return;
    }

    for (chain, adapter) in adapters(Environment::Staging).await {
        let version = adapter.VERSION().call().await.expect("VERSION");
        assert!(
            is_release(&version) || is_release_candidate(&version),
            "{chain}: version is neither a release nor a release candidate: {version}"
        );
    }
}

#[tokio::test]
async fn production_deployments_run_a_release_version_and_are_safe_owned() {
    if !is_armed(Environment::Production) {
        return;
    }

    for (chain, adapter) in adapters(Environment::Production).await {
        let version = adapter.VERSION().call().await.expect("VERSION");
        assert!(
            is_release(&version),
            "{chain}: version is a prerelease: {version}"
        );

        let owner = adapter.owner().call().await.expect("owner");
        assert!(
            is_safe(adapter.provider(), owner).await,
            "{chain}: proxy is not owned by a Safe"
        );
    }
}

/// Every recorded proxy delegates to the implementation this source predicts for its chain, which proves the
/// environment runs this source. The record is not a term in the comparison — the chain answers what it runs.
async fn expect_source_implementations(environment: Environment) {
    if !is_armed(environment) {
        return;
    }

    for (chain, adapter) in adapters(environment).await {
        let context = context(environment, chain as u64);

        let router = adapter
            .RISC_ZERO_VERIFIER_ROUTER()
            .call()
            .await
            .expect("RISC_ZERO_VERIFIER_ROUTER");
        let selector = adapter
            .RISC_ZERO_VERIFIER_SELECTOR()
            .call()
            .await
            .expect("RISC_ZERO_VERIFIER_SELECTOR");

        let constructor_args = ProtocolAdapter::constructorCall {
            riscZeroVerifierRouter: router,
            riscZeroVerifierSelector: selector,
        }
        .abi_encode();
        let init_code = [
            ProtocolAdapter::BYTECODE.as_ref(),
            constructor_args.as_ref(),
        ]
        .concat();
        let source_implementation =
            CREATE2_DEPLOYER.create2(bytes32(IMPLEMENTATION_SALT), keccak256(&init_code));

        let deployed_implementation = adapter
            .getImplementation()
            .call()
            .await
            .expect("getImplementation");

        assert_eq!(
            deployed_implementation, source_implementation,
            "{context}: does not run the source implementation"
        );
    }
}

/// The protocol adapter instance of every chain recorded for the environment, read over its own RPC.
async fn adapters(
    environment: Environment,
) -> Vec<(
    NamedChain,
    ProtocolAdapter::ProtocolAdapterInstance<DynProvider>,
)> {
    let mut adapters = Vec::new();

    for chain in protocol_adapter_deployments_map(environment).keys() {
        let url = alchemy_url(chain).expect("RPC URL");
        let provider = ProviderBuilder::new().connect_http(url).erased();
        let adapter = protocol_adapter(&provider, environment)
            .await
            .expect("protocol adapter instance");

        adapters.push((*chain, adapter));
    }

    adapters
}

/// Identifies a Safe by probing its owner set and threshold, which every set-up Safe links through
/// `1 <= threshold <= owners.length`.
async fn is_safe(provider: &DynProvider, account: Address) -> bool {
    if provider
        .get_code_at(account)
        .await
        .is_ok_and(|code| code.is_empty())
    {
        return false;
    }

    let safe = IOwnerManager::new(account, provider);
    let (Ok(owners), Ok(threshold)) = (
        safe.getOwners().call().await,
        safe.getThreshold().call().await,
    ) else {
        return false;
    };

    threshold >= alloy::primitives::U256::from(1)
        && threshold <= alloy::primitives::U256::from(owners.len())
}
