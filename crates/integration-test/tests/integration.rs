use alloy::sol_types::SolError;
use anoma_pa_evm_bindings::generated::protocol_adapter::{
    IProtocolAdapter, ProtocolAdapter as PaContract,
};
use anoma_pa_evm_integration_test::deploy::pa::protocol_adapter;
#[cfg(feature = "e2e")]
use anoma_pa_evm_integration_test::envs::e2e::Environment as EvmE2eEnv;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_evm_integration_test::state::pa::pa_address;
use anoma_pa_testkit::assert::{Needle, expect_integration_panic};
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::transaction::Transaction;
use anoma_pa_testkit::{commitment_root, execute_tx, prove_actions};
use anoma_risc0_verifier_bindings::generated::risc_zero_mock_verifier::RiscZeroMockVerifier;
use anoma_rm_risc0::Digest;
use anyhow::Context;
use rstest::*;

/// Expects a revert carrying the given custom error selector.
fn expect_revert_selector<T>(
    selector: [u8; 4],
) -> impl FnOnce(anyhow::Result<T>) -> anyhow::Result<()> {
    expect_integration_panic(Needle::Regexp(
        regex::Regex::new(&regex::escape(&format!(
            "custom error 0x{}",
            hex::encode(selector)
        )))
        .unwrap(),
    ))
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[cfg_attr(feature = "e2e", case::e2e_test(EvmE2eEnv::setup_bare()))]
#[tokio::test]
async fn execute_tx_settles_a_trivial_transaction<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()> {
    let mut env = env.context("env setup failed")?;

    let before = commitment_root(&env)?;

    let action = trivial::build(1, trivial::Overrides::default())
        .context("failed to build trivial action")?
        .witnesses;
    let tx = prove_actions(&env, &[action]).await?;

    execute_tx(&mut env, tx).await?;

    let after = commitment_root(&env)?;

    anyhow::ensure!(before != after, "commitment tree root must change");
    Ok(())
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[cfg_attr(feature = "e2e", case::e2e_test(EvmE2eEnv::setup_bare()))]
#[tokio::test]
async fn execute_tx_settles_an_n_to_m_transaction<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()> {
    let mut env = env.context("env setup failed")?;

    let before = commitment_root(&env)?;

    let action = trivial::build(
        21,
        trivial::Overrides {
            consumed_count: Some(2),
            created_count: Some(3),
            ..trivial::Overrides::default()
        },
    )
    .context("failed to build an n:m trivial action")?
    .witnesses;
    let tx = prove_actions(&env, &[action]).await?;

    execute_tx(&mut env, tx).await?;

    let after = commitment_root(&env)?;

    anyhow::ensure!(before != after, "commitment tree root must change");
    Ok(())
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[cfg_attr(feature = "e2e", case::e2e_test(EvmE2eEnv::setup_bare()))]
#[tokio::test]
async fn execute_tx_settles_a_multi_action_transaction<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()> {
    let mut env = env.context("env setup failed")?;

    let before = commitment_root(&env)?;

    let actions = trivial::build_many(3, 31).context("failed to build trivial actions")?;
    let tx = prove_actions(&env, &actions).await?;

    execute_tx(&mut env, tx).await?;

    let after = commitment_root(&env)?;

    anyhow::ensure!(before != after, "commitment tree root must change");
    Ok(())
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn execute_tx_settles_consume_only_transactions_without_a_root_change<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()> {
    let mut env = env.context("env setup failed")?;

    let before = commitment_root(&env)?;

    let consume_only = |seed| {
        trivial::build(
            seed,
            trivial::Overrides {
                consumed_count: Some(2),
                created_count: Some(0),
                ..trivial::Overrides::default()
            },
        )
        .context("failed to build a consume-only trivial action")
    };

    let tx = prove_actions(&env, &[consume_only(41)?.witnesses]).await?;
    execute_tx(&mut env, tx).await?;

    anyhow::ensure!(
        commitment_root(&env)? == before,
        "a consume-only transaction must leave the commitment tree root unchanged"
    );

    // A second consume-only transaction settles as well — no root is stored,
    // so no pre-existing-root collision can occur.
    let tx = prove_actions(&env, &[consume_only(42)?.witnesses]).await?;
    execute_tx(&mut env, tx).await?;

    anyhow::ensure!(
        commitment_root(&env)? == before,
        "the commitment tree root must still be unchanged"
    );
    Ok(())
}

#[rstest]
#[case::local(
    EvmLocalEnv::setup_bare(),
    expect_integration_panic(Needle::Static("Invalid padding resource"))
)]
#[tokio::test]
async fn prove_actions_errors_on_nonzero_quantity<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
    #[case] assert_err: impl FnOnce(anyhow::Result<Env::Transaction>) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let env = env.expect("env setup failed");

    let bad = trivial::build(7, trivial::Overrides::invalid_nonzero_quantity())
        .expect("failed to build invalid trivial action");

    assert_err(prove_actions(&env, &[bad.witnesses]).await)
}

#[rstest]
#[case::local(
    EvmLocalEnv::setup_bare(),
    expect_integration_panic(Needle::Static("Invalid padding resource"))
)]
#[tokio::test]
async fn prove_actions_errors_on_non_ephemeral_consumed<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
    #[case] assert_err: impl FnOnce(anyhow::Result<Env::Transaction>) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let env = env.expect("env setup failed");

    let bad = trivial::build(8, trivial::Overrides::invalid_consumed_non_ephemeral())
        .expect("failed to build invalid trivial action");

    assert_err(prove_actions(&env, &[bad.witnesses]).await)
}

#[rstest]
#[case::local(
    EvmLocalEnv::setup_bare(),
    Transaction::tamper_aggregation_seal,
    // The RiscZeroMockVerifier rejects the tampered seal with `VerificationFailed()`.
    expect_integration_panic(Needle::Regexp(
        regex::Regex::new(&regex::escape(&format!(
            "execution reverted: custom error 0x{}",
            hex::encode(RiscZeroMockVerifier::VerificationFailed::SELECTOR)
        )))
        .unwrap(),
    ))
)]
#[tokio::test]
async fn execute_tx_reverts_on_invalid_seal<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
    #[case] tamper: impl FnOnce(&mut Env::Transaction) -> anyhow::Result<()>,
    #[case] assert_err: impl FnOnce(anyhow::Result<()>) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let mut env = env.context("env setup failed")?;

    let actions = trivial::build_many(1, 11).context("failed to build trivial actions")?;
    let mut tx = prove_actions(&env, &actions)
        .await
        .context("valid witnesses should prove before tampering")?;

    tamper(&mut tx).context("failed to tamper transaction proof")?;

    assert_err(execute_tx(&mut env, tx).await)
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn execute_reverts_on_a_non_historical_root<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let env = env.context("env setup failed")?;

    let actions = trivial::build_many(1, 71).context("failed to build trivial actions")?;
    let mut tx = prove_actions(&env, &actions).await?;

    tx.as_arm_mut()
        .aggregation
        .as_mut()
        .context("the transaction must carry an aggregation")?
        .instance
        .actions[0]
        .consumed_publics[0]
        .commitment_tree_root = Digest::from([0x42u8; 32]);

    // `execute_tx` pre-checks the roots client-side, so call the contract
    // directly to reach the on-chain check. It runs before proof verification,
    // so tampering the proven instance still hits `NonExistingRoot` first.
    let pa_tx: IProtocolAdapter::Transaction = tx.into_arm().into();
    let pa = protocol_adapter(pa_address(&env)?, default_signer(&env)?);

    let err = match pa.execute(pa_tx).call().await {
        Ok(_) => anyhow::bail!("execute accepted a non-historical root"),
        Err(err) => err,
    };
    let data = err
        .as_revert_data()
        .with_context(|| format!("execute failed without revert data: {err}"))?;

    anyhow::ensure!(
        data.len() >= 4 && data[..4] == PaContract::NonExistingRoot::SELECTOR,
        "expected `NonExistingRoot`, got selector 0x{}",
        hex::encode(&data[..data.len().min(4)]),
    );
    Ok(())
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn execute_tx_reverts_on_a_replayed_transaction<Env: Environment>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()> {
    let mut env = env.context("env setup failed")?;

    // The trivial fixture is deterministic per seed, so a rebuild consumes the
    // same nullifiers.
    let tx = prove_actions(&env, &trivial::build_many(1, 72)?).await?;
    execute_tx(&mut env, tx)
        .await
        .context("the first execution must settle")?;

    let replay = prove_actions(&env, &trivial::build_many(1, 72)?).await?;

    expect_revert_selector(PaContract::PreExistingNullifier::SELECTOR)(
        execute_tx(&mut env, replay).await,
    )
}

#[rstest]
#[case::local(EvmLocalEnv::setup_bare())]
#[tokio::test]
async fn execute_tx_reverts_on_a_tampered_action_tree_root<Env>(
    #[future(awt)]
    #[case]
    env: anyhow::Result<Env>,
) -> anyhow::Result<()>
where
    Env: Environment<Transaction = Transaction>,
{
    let mut env = env.context("env setup failed")?;

    let actions = trivial::build_many(1, 73).context("failed to build trivial actions")?;
    let mut tx = prove_actions(&env, &actions).await?;

    // The delta proof signs the action tree roots, so the tampered root fails
    // the delta check before the aggregation proof is even considered.
    let root = &mut tx
        .as_arm_mut()
        .aggregation
        .as_mut()
        .context("the transaction must carry an aggregation")?
        .instance
        .actions[0]
        .action_tree_root;
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(root.as_bytes());
    bytes[0] ^= 0xff;
    *root = Digest::from(bytes);

    expect_revert_selector(PaContract::DeltaMismatch::SELECTOR)(execute_tx(&mut env, tx).await)
}
