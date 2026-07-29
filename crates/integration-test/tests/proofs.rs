//! Verifies proofs one by one against the on-chain `RiscZeroVerifierRouter`
//! to pinpoint the failing proof kind, instead of relying on the entire
//! transaction settling via `ProtocolAdapter.execute`. The logic proof is
//! checked for both roles because `is_consumed` is part of the journal.
//!
//! The queue prover backing `e2e` aggregates and erases the individual
//! proofs, so compliance and logic proofs are checked in `local` only, and
//! `e2e` checks the aggregation proof (a real Groth16 seal).

use alloy::primitives::{B256, Bytes};
use alloy::providers::DynProvider;
use anoma_pa_evm_integration_test::deploy::pa::protocol_adapter;
#[cfg(feature = "e2e")]
use anoma_pa_evm_integration_test::envs::e2e::Environment as EvmE2eEnv;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_evm_integration_test::state::pa::pa_address;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::prove_actions;
use anoma_pa_testkit::transaction::Transaction;
use anoma_risc0_verifier_bindings::generated::risc_zero_verifier_router::RiscZeroVerifierRouter;
use anoma_rm_risc0::action::Action;
use anoma_rm_risc0::action_tree::MerkleTree;
use anoma_rm_risc0::proving_system::encode_seal;
use anyhow::Context;
use risc0_zkvm::sha::Digestible;
use rstest::*;

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn verify_verifies_a_compliance_proof<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let env = env.context("env setup failed")?;
    let tx = prove_trivial_tx(&env).await?;
    let router = router(&env).await?;

    let unit = single_action(&tx)?
        .compliance_units
        .first()
        .context("action has no compliance unit")?;
    let proof = unit
        .proof
        .as_ref()
        .context("compliance unit carries no proof")?;
    let seal = encode_seal(proof).context("failed to encode the compliance seal")?;

    router
        .verify(
            Bytes::from(seal),
            B256::from_slice(anoma_rm_risc0::constants::COMPLIANCE_VK.as_bytes()),
            journal_digest(&unit.instance),
        )
        .call()
        .await
        .context("on-chain verification failed for the compliance proof")?;

    Ok(())
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn verify_verifies_a_logic_proof_of_a_consumed_resource<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    verify_logic_proof(env, true).await
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn verify_verifies_a_logic_proof_of_a_created_resource<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    verify_logic_proof(env, false).await
}

#[cfg(feature = "e2e")]
#[rstest]
#[case::e2e_test(EvmE2eEnv::setup_bare())]
#[tokio::test]
async fn verify_verifies_the_aggregation_proof<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let env = env.context("env setup failed")?;
    let tx = prove_trivial_tx(&env).await?;
    let router = router(&env).await?;

    let arm = tx.as_arm();
    let proof = arm
        .aggregation_proof
        .as_ref()
        .context("transaction carries no aggregation proof")?;
    let seal = encode_seal(proof).context("failed to encode the aggregation seal")?;
    let instance = arm
        .construct_aggregation_instance()
        .context("failed to construct the aggregation instance")?;

    router
        .verify(
            Bytes::from(seal),
            B256::from_slice(anoma_rm_risc0::constants::BATCH_AGGREGATION_VK.as_bytes()),
            journal_digest(&instance),
        )
        .call()
        .await
        .context("on-chain verification failed for the aggregation proof")?;

    Ok(())
}

/// Verifies the logic proof of a consumed or created resource against the
/// router, rebuilding the instance like `Action::get_logic_verifiers`.
async fn verify_logic_proof<Env>(env: anyhow::Result<Env>, is_consumed: bool) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let role = if is_consumed { "consumed" } else { "created" };

    let env = env.context("env setup failed")?;
    let tx = prove_trivial_tx(&env).await?;
    let router = router(&env).await?;

    let action = single_action(&tx)?;
    let instance = action
        .compliance_units
        .first()
        .context("action has no compliance unit")?
        .get_instance()
        .context("failed to decode the compliance instance")?;

    let root = MerkleTree::from(vec![
        instance.consumed_nullifier,
        instance.created_commitment,
    ])
    .root()
    .context("failed to compute the action tree root")?;
    let tag = if is_consumed {
        instance.consumed_nullifier
    } else {
        instance.created_commitment
    };

    let verifier = action
        .logic_verifier_inputs
        .iter()
        .find(|input| input.tag == tag)
        .with_context(|| format!("no logic verifier input for the {role} tag {tag}"))?
        .clone()
        .to_logic_verifier(is_consumed, root)
        .with_context(|| {
            format!("failed to rebuild the logic instance for the {role} tag {tag}")
        })?;
    let proof = verifier.proof.as_ref().with_context(|| {
        format!("logic verifier input for the {role} tag {tag} carries no proof")
    })?;
    let seal = encode_seal(proof)
        .with_context(|| format!("failed to encode the seal for the {role} tag {tag}"))?;

    router
        .verify(
            Bytes::from(seal),
            B256::from_slice(verifier.verifying_key.as_bytes()),
            journal_digest(&verifier.instance),
        )
        .call()
        .await
        .with_context(|| {
            format!("on-chain verification failed for the logic proof of the {role} resource")
        })?;

    Ok(())
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

/// Returns the trivial transaction's single action.
fn single_action(tx: &Transaction) -> anyhow::Result<&Action> {
    tx.as_arm()
        .actions
        .first()
        .context("transaction has no action")
}

/// Connects to the verifier router the protocol adapter is configured with.
async fn router<Env>(
    env: &Env,
) -> anyhow::Result<RiscZeroVerifierRouter::RiscZeroVerifierRouterInstance<DynProvider>>
where
    Env: Environment,
{
    let provider = default_signer(env)?;
    let pa = protocol_adapter(pa_address(env)?, provider.clone());

    let router_address = pa
        .getRiscZeroVerifierRouter()
        .call()
        .await
        .context("failed to query the verifier router address from the protocol adapter")?;

    Ok(RiscZeroVerifierRouter::new(router_address, provider))
}

/// SHA-256 of the journal bytes — the digest the on-chain verifier checks.
fn journal_digest(journal_bytes: &[u8]) -> B256 {
    let digest = risc0_zkvm::Journal::new(journal_bytes.to_vec()).digest();

    B256::from_slice(digest.as_bytes())
}
