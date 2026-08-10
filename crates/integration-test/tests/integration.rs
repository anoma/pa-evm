use alloy::sol_types::SolError;
#[cfg(feature = "e2e")]
use anoma_pa_evm_integration_test::envs::e2e::Environment as EvmE2eEnv;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_testkit::assert::{Needle, expect_integration_panic};
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::transaction::Transaction;
use anoma_pa_testkit::{commitment_root, execute_tx, prove_actions};
use anoma_risc0_verifier_bindings::generated::risc_zero_mock_verifier::RiscZeroMockVerifier;
use anyhow::Context;
use rstest::*;

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
