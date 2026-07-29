//! Tampers one seal byte per proof kind and checks that
//! `ProtocolAdapter.simulateExecute` with proof verification enabled reverts
//! with `VerificationFailed` instead of `Simulated`. Local-only: the seals
//! are the mock verifier's, including the aggregation seal minted here.

use alloy::primitives::{Bytes, keccak256};
use alloy::sol_types::SolError;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter as PaContract;
use anoma_pa_evm_integration_test::deploy::mock_risc0_bindings::MOCK_VERIFIER_SELECTOR;
use anoma_pa_evm_integration_test::deploy::pa::protocol_adapter;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_evm_integration_test::state::pa::pa_address;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::prove_actions;
use anoma_pa_testkit::transaction::Transaction;
use anyhow::Context;
use risc0_zkvm::sha::Digestible;
use risc0_zkvm::{Journal, MaybePruned, ReceiptClaim};
use rstest::*;

/// Tampered seal byte: past the 4-byte selector, inside the claim digest.
const TAMPER_INDEX: usize = 5;

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn simulate_execute_reverts_on_invalid_logic_proof_if_proof_verification_is_not_skipped<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let env = env.context("env setup failed")?;
    let tx = pa_transaction(&env).await?;
    assert_simulate_execute_reverts_with(
        &env,
        tx.clone(),
        PaContract::Simulated::SELECTOR,
        "the untampered transaction",
    )
    .await?;

    let mut tampered = tx;
    tampered.actions[0].logicVerifierInputs[0].proof =
        tamper(&tampered.actions[0].logicVerifierInputs[0].proof);

    assert_simulate_execute_reverts_with(
        &env,
        tampered,
        verification_failed_selector(),
        "the transaction with the tampered logic proof",
    )
    .await
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn simulate_execute_reverts_on_invalid_compliance_proof_if_proof_verification_is_not_skipped<
    Env,
>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let env = env.context("env setup failed")?;
    let tx = pa_transaction(&env).await?;
    assert_simulate_execute_reverts_with(
        &env,
        tx.clone(),
        PaContract::Simulated::SELECTOR,
        "the untampered transaction",
    )
    .await?;

    let mut tampered = tx;
    tampered.actions[0].complianceVerifierInputs[0].proof =
        tamper(&tampered.actions[0].complianceVerifierInputs[0].proof);

    assert_simulate_execute_reverts_with(
        &env,
        tampered,
        verification_failed_selector(),
        "the transaction with the tampered compliance proof",
    )
    .await
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn simulate_execute_reverts_on_invalid_aggregation_proof_if_proof_verification_is_not_skipped<
    Env,
>(
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

    assert_simulate_execute_reverts_with(
        &env,
        tx.clone(),
        PaContract::Simulated::SELECTOR,
        "the untampered aggregated transaction",
    )
    .await?;

    let mut tampered = tx;
    tampered.aggregationProof = tamper(&tampered.aggregationProof);

    assert_simulate_execute_reverts_with(
        &env,
        tampered,
        verification_failed_selector(),
        "the transaction with the tampered aggregation proof",
    )
    .await
}

/// Proves a transaction with a single trivial action, without executing it.
async fn prove_trivial_tx<Env>(env: &Env) -> anyhow::Result<Transaction>
where
    Env: Environment<Transaction = Transaction>,
{
    let action = trivial::build(1, trivial::Overrides::default())
        .context("failed to build trivial action")?
        .witnesses;

    prove_actions(env, &[action]).await
}

/// Proves a trivial transaction and converts it into protocol adapter calldata.
async fn pa_transaction<Env>(env: &Env) -> anyhow::Result<PaContract::Transaction>
where
    Env: Environment<Transaction = Transaction>,
{
    Ok(prove_trivial_tx(env).await?.into_arm().into())
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

/// Flips all bits of one seal byte to invalidate the proof it belongs to.
fn tamper(proof: &Bytes) -> Bytes {
    let mut bytes = proof.to_vec();
    bytes[TAMPER_INDEX] ^= 0xff;

    Bytes::from(bytes)
}

/// The selector of the verifier's `VerificationFailed()` error.
fn verification_failed_selector() -> [u8; 4] {
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&keccak256(b"VerificationFailed()")[..4]);

    selector
}

/// Asserts that `simulateExecute` with proof verification enabled reverts with
/// the expected error selector.
async fn assert_simulate_execute_reverts_with<Env>(
    env: &Env,
    tx: PaContract::Transaction,
    expected_selector: [u8; 4],
    what: &str,
) -> anyhow::Result<()>
where
    Env: Environment,
{
    let provider = default_signer(env)?;
    let pa = protocol_adapter(pa_address(env)?, provider);

    let err = match pa.simulateExecute(tx, false).call().await {
        Ok(_) => anyhow::bail!("simulateExecute returned instead of reverting for {what}"),
        Err(err) => err,
    };

    let data = err
        .as_revert_data()
        .with_context(|| format!("simulateExecute failed without revert data for {what}: {err}"))?;

    anyhow::ensure!(
        data.len() >= 4 && data[..4] == expected_selector,
        "unexpected revert selector for {what}: got 0x{}, want 0x{}",
        hex::encode(&data[..data.len().min(4)]),
        hex::encode(expected_selector),
    );

    Ok(())
}
