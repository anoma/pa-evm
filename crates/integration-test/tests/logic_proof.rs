//! Verifies logic proofs directly against the on-chain
//! `RiscZeroVerifierRouter`, pinpointing this proof kind instead of relying
//! on the entire transaction settling. Both roles are checked because
//! `is_consumed` is part of the journal. Local-only: the queue prover backing
//! `e2e` aggregates and erases the individual proofs.

mod common;

use alloy::primitives::{B256, Bytes};
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::transaction::Transaction;
use anoma_rm_risc0::action_tree::MerkleTree;
use anoma_rm_risc0::proving_system::encode_seal;
use anyhow::Context;
use rstest::*;

use common::{journal_digest, prove_trivial_tx, router, single_action};

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
