//! Checks of the recorded deployments against the chains they run on. They run on the promotion gate: the
//! `VERIFY_*` flags arm them, because between a version bump on `next` and the environment's upgrade the source
//! and the deployments legitimately disagree.

mod common;

use alloy::primitives::{Address, B256, FixedBytes, keccak256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::sol;
use alloy::sol_types::SolConstructor;
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::{Environment, protocol_adapter_deployments_map};
use anoma_pa_evm_bindings::contract::protocol_adapter;
use anoma_pa_evm_bindings::generated::i_migrational::IMigrational;
use anoma_pa_evm_bindings::generated::migrational_protocol_adapter::MigrationalProtocolAdapter;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter;
use anoma_pa_evm_bindings::helpers::alchemy_url;
use common::{CREATE2_DEPLOYER, context, is_armed, is_release, is_release_candidate, parameters};

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
/// environment runs this source. A proxy may also delegate to the migrational implementation this source
/// predicts, until its completion run upgrades it. The record is not a term in the comparison — the chain
/// answers what it runs.
async fn expect_source_implementations(environment: Environment) {
    if !is_armed(environment) {
        return;
    }

    let implementation_salt = parameters().await.implementation_salt;

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
        let source_implementation = create2_address(
            implementation_salt,
            &ProtocolAdapter::BYTECODE,
            &constructor_args,
        );

        let deployed_implementation = adapter
            .getImplementation()
            .call()
            .await
            .expect("getImplementation");

        if deployed_implementation != source_implementation
            && migrational_source_implementation(&adapter, implementation_salt, router, selector)
                .await
                .is_some_and(|implementation| implementation == deployed_implementation)
        {
            eprintln!("info: {context}: runs the migrational implementation this source predicts");
            continue;
        }

        assert_eq!(
            deployed_implementation, source_implementation,
            "{context}: does not run the source implementation"
        );
    }
}

/// The migrational implementation this source predicts for the proxy, or `None` if the proxy does not run
/// one. Only the migrational implementation reports the v1 protocol adapter, its third constructor argument,
/// so the call fails for any other implementation.
async fn migrational_source_implementation(
    adapter: &ProtocolAdapter::ProtocolAdapterInstance<DynProvider>,
    implementation_salt: B256,
    router: Address,
    selector: FixedBytes<4>,
) -> Option<Address> {
    let protocol_adapter_v1 = IMigrational::new(*adapter.address(), adapter.provider())
        .getProtocolAdapterV1()
        .call()
        .await
        .ok()?;

    let constructor_args = MigrationalProtocolAdapter::constructorCall {
        riscZeroVerifierRouter: router,
        riscZeroVerifierSelector: selector,
        protocolAdapterV1: protocol_adapter_v1,
    }
    .abi_encode();

    Some(create2_address(
        implementation_salt,
        &MigrationalProtocolAdapter::BYTECODE,
        &constructor_args,
    ))
}

/// The address at which the deterministic deployer creates a contract from its creation code and arguments.
fn create2_address(salt: B256, creation_code: &[u8], constructor_args: &[u8]) -> Address {
    CREATE2_DEPLOYER.create2(salt, keccak256([creation_code, constructor_args].concat()))
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
