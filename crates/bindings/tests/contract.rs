#[cfg(test)]
extern crate dotenvy;

use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::{Environment, protocol_adapter_deployments_map};
use anoma_pa_evm_bindings::contract::protocol_adapter;
use anoma_pa_evm_bindings::generated::protocol_adapter;
use anoma_pa_evm_bindings::helpers::alchemy_url;

#[tokio::test]
async fn versions_of_deployed_protocol_adapters_match_the_expected_version() {
    // Iterate over all supported chains
    for chain in protocol_adapter_deployments_map(Environment::Test).keys() {
        let existing_pa = pa_instance(chain).await;

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

        //  Check that the deployed protocol adapter version matches the expected version.
        assert_eq!(
            actual_version, expected_version,
            "Protocol adapter version mismatch on network '{chain}'."
        );
    }
}

async fn pa_instance(
    chain: &NamedChain,
) -> protocol_adapter::ProtocolAdapter::ProtocolAdapterInstance<DynProvider> {
    let rpc_url = alchemy_url(chain).expect("Couldn't get RPC URL for chain");

    let provider = ProviderBuilder::new()
        .connect_anvil_with_wallet_and_config(|a| a.fork(rpc_url))
        .expect("Couldn't create anvil provider");
    protocol_adapter(&provider.erased(), Environment::Test)
        .await
        .expect("Couldn't get protocol adapter instance")
}
