#[cfg(test)]
extern crate dotenvy;

use alloy::primitives::B256;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::sol_types::SolError;
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::protocol_adapter_deployments_map;
use anoma_pa_evm_bindings::contract::protocol_adapter;
use anoma_pa_evm_bindings::generated::protocol_adapter;
use anoma_pa_evm_bindings::helpers::alchemy_url;

#[tokio::test]
async fn versions_of_deployed_protocol_adapters_match_the_expected_version() {
    // Iterate over all supported chains
    for chain in protocol_adapter_deployments_map().keys() {
        let existing_pa = pa_instance(chain).await;

        // `getVersion` is `pure`, so the freshly deployed implementation answers
        // it without being put behind a proxy and initialized.
        let current_pa_implementation = protocol_adapter::ProtocolAdapter::deploy(
            existing_pa.provider(),
            existing_pa
                .getRiscZeroVerifierRouter()
                .call()
                .await
                .expect("Couldn't get risc zero verifier router address"),
            existing_pa
                .getRiscZeroVerifierSelector()
                .call()
                .await
                .expect("Couldn't get risc zero verifier selector"),
        )
        .await
        .expect("Couldn't deploy protocol adapter implementation");

        let expected_version = current_pa_implementation
            .getVersion()
            .call()
            .await
            .expect("Couldn't get version");

        let actual_version: alloy::primitives::FixedBytes<32> = existing_pa
            .getVersion()
            .call()
            .await
            .expect("Couldn't get protocol adapter version");

        //  Check that the deployed protocol adapter version matches the expected version.
        assert_eq!(
            decode_bytes32_to_utf8(actual_version),
            decode_bytes32_to_utf8(expected_version),
            "Protocol adapter version mismatch on network '{chain}'."
        );
    }
}

#[tokio::test]
async fn call_reverts_on_the_empty_tx_on_all_supported_chains() {
    for chain in protocol_adapter_deployments_map().keys() {
        let empty_tx = protocol_adapter::ProtocolAdapter::Transaction {
            actions: vec![],
            deltaProof: Default::default(),
            aggregationProof: Default::default(),
        };

        let err = pa_instance(chain)
            .await
            .execute(empty_tx)
            .send()
            .await
            .expect_err("Empty transaction must be rejected");

        let data = err.as_revert_data().unwrap_or_else(|| {
            panic!("Empty transaction failed without revert data on network '{chain}'.")
        });

        assert_eq!(
            data[..4],
            protocol_adapter::ProtocolAdapter::EmptyTransactionNotAllowed::SELECTOR,
            "Empty transaction did not revert with `EmptyTransactionNotAllowed` on network '{chain}'."
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
    protocol_adapter(&provider.erased())
        .await
        .expect("Couldn't get protocol adapter instance")
}

fn decode_bytes32_to_utf8(encoded_string: B256) -> String {
    let bytes = alloy::hex::decode(encoded_string.to_string()).expect("Couldn't decode hex string");

    let trimmed = bytes
        .split(|b| *b == 0)
        .next()
        .expect("No null byte found in bytes");
    str::from_utf8(trimmed)
        .expect("Conversion to UTF-8 failed.")
        .to_string()
}
