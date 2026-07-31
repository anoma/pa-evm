//! Pins the protocol adapter's Solidity aggregation-journal encoding to the
//! risc0 serde encoding of arm-risc0's `AggregationInstance`: the mock seal
//! the local prover mints over the Rust-built instance must pass
//! `simulateExecute`. A selector other than `Simulated` means the encodings
//! diverge — including the injected compliance key and kind table commitment.

mod common;

use alloy::sol_types::SolError;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter as PaContract;
use anoma_pa_evm_integration_test::deploy::pa::protocol_adapter;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_evm_integration_test::state::pa::pa_address;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::transaction::Transaction;
use anyhow::Context;
use rstest::*;

use common::prove_trivial_tx;

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn simulate_execute_accepts_an_aggregation_seal_over_the_rust_encoded_instance<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let env = env.context("env setup failed")?;
    let proven = prove_trivial_tx(&env).await?;

    let tx: PaContract::Transaction = proven.into_arm().into();

    let provider = default_signer(&env)?;
    let pa = protocol_adapter(pa_address(&env)?, provider);

    let err = match pa.simulateExecute(tx, false).call().await {
        Ok(_) => anyhow::bail!("simulateExecute returned instead of reverting"),
        Err(err) => err,
    };
    let data = err
        .as_revert_data()
        .with_context(|| format!("simulateExecute failed without revert data: {err}"))?;

    anyhow::ensure!(
        data.len() >= 4 && data[..4] == PaContract::Simulated::SELECTOR,
        "the aggregation instance encodings diverge: got selector 0x{}, want `Simulated` 0x{}",
        hex::encode(&data[..data.len().min(4)]),
        hex::encode(PaContract::Simulated::SELECTOR),
    );

    Ok(())
}
