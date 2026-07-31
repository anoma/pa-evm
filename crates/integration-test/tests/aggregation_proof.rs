//! Verifies the aggregation proof — the only RISC Zero seal a transaction
//! carries — directly against the verifier router, pinpointing this proof
//! kind instead of relying on the entire transaction settling. The local
//! prover mints a mock seal the mock verifier accepts; the e2e queue prover
//! returns a real Groth16 seal checked by the real router.

mod common;

use alloy::primitives::{B256, Bytes};
#[cfg(feature = "e2e")]
use anoma_pa_evm_integration_test::envs::e2e::Environment as EvmE2eEnv;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::transaction::Transaction;
use anoma_rm_risc0::proving_system::encode_seal;
use anyhow::Context;
use rstest::*;

use common::{journal_digest, prove_trivial_tx, router};

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[cfg_attr(feature = "e2e", case::e2e_test(EvmE2eEnv::setup_bare()))]
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
