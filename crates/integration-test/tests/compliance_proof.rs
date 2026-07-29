//! Verifies a compliance proof directly against the on-chain
//! `RiscZeroVerifierRouter`, pinpointing this proof kind instead of relying
//! on the entire transaction settling. Local-only: the queue prover backing
//! `e2e` aggregates and erases the individual proofs.

mod common;

use alloy::primitives::{B256, Bytes};
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::transaction::Transaction;
use anoma_rm_risc0::proving_system::encode_seal;
use anyhow::Context;
use rstest::*;

use common::{journal_digest, prove_trivial_tx, router, single_action};

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
