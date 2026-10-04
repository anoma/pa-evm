//! A resource's external call, settled through the adapter: pa-testkit's
//! pass-through action carries the call the adapter makes to a forwarder
//! (`abi.encode(forwarder, input, expectedOutput)`), here to the example
//! `BlockTimeForwarder`, which compares a time with the block's.

use alloy::primitives::{Address, Bytes};
use alloy::sol_types::{SolError, SolType, SolValue, sol_data};
use anoma_pa_evm_bindings::generated::block_time_forwarder::BlockTimeForwarder;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_testkit::assert::{Needle, expect_integration_panic};
use anoma_pa_testkit::fixtures::passthrough;
use anoma_pa_testkit::witness::{AppData, ExpirableBlob};
use anoma_pa_testkit::{execute_tx, prove_actions};
use anoma_rm_risc0::utils::bytes_to_words;
use anyhow::Context;

/// `BlockTimeForwarder.TimeComparison`: the given time is before (`LT`),
/// at, or after the block's.
const LT: u8 = 0;
const GT: u8 = 2;

/// App data whose external payload is one call to `forwarder` asking how
/// `time` compares with the block's, expecting `expected`.
fn block_time_call(forwarder: Address, time: u64, expected: u8) -> AppData {
    let input =
        <sol_data::Uint<48> as SolType>::abi_encode(&alloy::primitives::aliases::U48::from(time));
    let output = <sol_data::Uint<8> as SolType>::abi_encode(&expected);
    let call = (forwarder, Bytes::from(input), Bytes::from(output)).abi_encode_params();
    AppData {
        external_payload: vec![ExpirableBlob {
            blob: bytes_to_words(&call),
            deletion_criterion: 0,
        }],
        ..AppData::default()
    }
}

async fn deploy_forwarder(env: &EvmLocalEnv) -> anyhow::Result<Address> {
    let forwarder = BlockTimeForwarder::deploy(default_signer(env)?)
        .await
        .context("failed to deploy the BlockTimeForwarder")?;
    Ok(*forwarder.address())
}

#[tokio::test]
async fn execute_tx_settles_an_external_call_whose_output_matches() -> anyhow::Result<()> {
    let mut env = EvmLocalEnv::setup_bare().await?;
    let forwarder = deploy_forwarder(&env).await?;

    // Time 0 is before every block.
    let action = passthrough::build(
        1,
        block_time_call(forwarder, 0, LT),
        passthrough::Overrides::default(),
    )?
    .witnesses;
    let tx = prove_actions(&env, &[action]).await?;
    execute_tx(&mut env, tx).await
}

#[tokio::test]
async fn execute_tx_refuses_an_external_call_whose_output_differs() -> anyhow::Result<()> {
    let mut env = EvmLocalEnv::setup_bare().await?;
    let forwarder = deploy_forwarder(&env).await?;

    let action = passthrough::build(
        2,
        block_time_call(forwarder, 0, GT),
        passthrough::Overrides::default(),
    )?
    .witnesses;
    let tx = prove_actions(&env, &[action]).await?;
    expect_integration_panic(Needle::Regexp(regex::Regex::new(&regex::escape(
        &format!(
            "execution reverted: custom error 0x{}",
            hex::encode(ProtocolAdapter::ForwarderCallOutputMismatch::SELECTOR)
        ),
    ))?))(execute_tx(&mut env, tx).await)
}
