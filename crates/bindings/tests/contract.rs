//! Deployment checks against the live environments. They run on the promotion gate: the
//! `VERIFY_*` flags arm them, because between a version bump on `next` and the environment's
//! upgrade the source and the deployments legitimately disagree.

#[cfg(test)]
extern crate dotenvy;

use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::{Environment, protocol_adapter_deployments_map};
use anoma_pa_evm_bindings::contract::protocol_adapter;
use anoma_pa_evm_bindings::generated::protocol_adapter;
use anoma_pa_evm_bindings::helpers::alchemy_url;

#[tokio::test]
async fn staging_deployments_run_the_source_version() {
    expect_source_versions(Environment::Staging, "VERIFY_STAGING_DEPLOYMENTS").await;
}

#[tokio::test]
async fn production_deployments_run_the_source_version() {
    expect_source_versions(Environment::Production, "VERIFY_PRODUCTION_DEPLOYMENTS").await;
}

/// Forks every chain recorded for the environment and checks that the deployed protocol adapter
/// answers the version this source compiles to. Skips unless the flag is set.
async fn expect_source_versions(environment: Environment, flag: &str) {
    if std::env::var(flag).as_deref() != Ok("true") {
        eprintln!("skipped: {flag} is not set");
        return;
    }

    for chain in protocol_adapter_deployments_map(environment).keys() {
        let existing_pa = pa_instance(chain, environment).await;

        // `VERSION` is a constant, so the freshly deployed implementation answers
        // it without being put behind a proxy and initialized.
        let current_pa_implementation = protocol_adapter::ProtocolAdapter::deploy(
            existing_pa.provider(),
            existing_pa
                .RISC_ZERO_VERIFIER_ROUTER()
                .call()
                .await
                .expect("Couldn't get risc zero verifier router address"),
            existing_pa
                .RISC_ZERO_VERIFIER_SELECTOR()
                .call()
                .await
                .expect("Couldn't get risc zero verifier selector"),
        )
        .await
        .expect("Couldn't deploy protocol adapter implementation");

        let expected_version = current_pa_implementation
            .VERSION()
            .call()
            .await
            .expect("Couldn't get version");

        let actual_version = existing_pa
            .VERSION()
            .call()
            .await
            .expect("Couldn't get protocol adapter version");

        assert_eq!(
            actual_version, expected_version,
            "Protocol adapter version mismatch on network '{chain}' of environment {environment:?}."
        );
    }
}

async fn pa_instance(
    chain: &NamedChain,
    environment: Environment,
) -> protocol_adapter::ProtocolAdapter::ProtocolAdapterInstance<DynProvider> {
    let rpc_url = alchemy_url(chain).expect("Couldn't get RPC URL for chain");

    let provider = ProviderBuilder::new()
        .connect_anvil_with_wallet_and_config(|a| a.fork(rpc_url))
        .expect("Couldn't create anvil provider");
    protocol_adapter(&provider.erased(), environment)
        .await
        .expect("Couldn't get protocol adapter instance")
}
