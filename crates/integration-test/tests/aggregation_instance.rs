//! Pins the protocol adapter's Solidity aggregation-instance encoding to
//! arm-risc0's `construct_aggregation_instance`: a mock seal minted over the
//! Rust-built instance must pass `simulateExecute`. Local-only unique
//! coverage — the local prover does not aggregate, and tampered-proof
//! rejection lives in the Solidity mock suite.

mod common;

use alloy::primitives::Bytes;
use alloy::sol_types::SolError;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter as PaContract;
use anoma_pa_evm_integration_test::deploy::mock_risc0_bindings::MOCK_VERIFIER_SELECTOR;
use anoma_pa_evm_integration_test::deploy::pa::protocol_adapter;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_evm_integration_test::state::pa::pa_address;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::transaction::Transaction;
use anyhow::Context;
use risc0_zkvm::sha::Digestible;
use risc0_zkvm::{Journal, MaybePruned, ReceiptClaim};
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

    let aggregation_proof = mock_aggregation_seal(&proven)?;
    let mut tx: PaContract::Transaction = proven.into_arm().into();
    tx.aggregationProof = aggregation_proof;

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

/// Mints the mock seal for the aggregation instance like the local prover
/// mints the individual ones: selector ++ "ok" receipt claim digest.
fn mock_aggregation_seal(tx: &Transaction) -> anyhow::Result<Bytes> {
    let instance = tx
        .as_arm()
        .construct_aggregation_instance()
        .context("failed to construct the aggregation instance")?;
    let journal_digest = Journal::new(instance).digest();

    let claim_digest = ReceiptClaim::ok(
        *anoma_rm_risc0::constants::BATCH_AGGREGATION_VK,
        MaybePruned::<Vec<u8>>::Pruned(journal_digest),
    )
    .digest();

    let mut seal = MOCK_VERIFIER_SELECTOR.to_vec();
    seal.extend_from_slice(claim_digest.as_bytes());

    Ok(Bytes::from(seal))
}
