//! Checks that the protocol adapter executes transactions of trivial resources, and that proving or
//! execution fails on an invalid resource or seal.

use alloy::sol_types::SolError;
#[cfg(feature = "e2e")]
use anoma_pa_evm_integration_test::envs::e2e::Environment as EvmE2eEnv;
use anoma_pa_evm_integration_test::envs::local::Environment as EvmLocalEnv;
use anoma_pa_testkit::assert::{Needle, expect_integration_panic};
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::fixtures::{action, trivial};
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

    // A nonce outside the `[seed; 32]` default, which the arm-risc0 transaction
    // generator produces too: one such transaction settled on Sepolia, so a fork
    // starts with its nullifier spent.
    let witnesses = action::build([
        trivial::consumed(
            1,
            trivial::Overrides {
                nonce: Some(*b"anoma pa-evm e2e trivial nonce 1"),
                ..trivial::Overrides::default()
            },
        ),
        trivial::created(1),
    ])
    .context("failed to build trivial action")?;
    let tx = prove_actions(&env, [&witnesses]).await?;

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

    let witnesses = action::build([
        trivial::consumed(21, trivial::Overrides::default()),
        trivial::consumed(22, trivial::Overrides::default()),
        trivial::created(21),
        trivial::created(22),
        trivial::created(23),
    ])
    .context("failed to build an n:m trivial action")?;
    let tx = prove_actions(&env, [&witnesses]).await?;

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

    let actions = (31..=33)
        .map(|seed| {
            action::build([
                trivial::consumed(seed, trivial::Overrides::default()),
                trivial::created(seed),
            ])
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("failed to build trivial actions")?;
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

    let consume_only = |seeds: [u8; 2]| {
        action::build(seeds.map(|seed| trivial::consumed(seed, trivial::Overrides::default())))
            .context("failed to build a consume-only trivial action")
    };

    let tx = prove_actions(&env, [&consume_only([41, 42])?]).await?;
    execute_tx(&mut env, tx).await?;

    anyhow::ensure!(
        commitment_root(&env)? == before,
        "a consume-only transaction must leave the commitment tree root unchanged"
    );

    // A second consume-only transaction settles as well — no root is stored,
    // so no pre-existing-root collision can occur.
    let tx = prove_actions(&env, [&consume_only([43, 44])?]).await?;
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

    let bad = action::build([
        trivial::consumed(7, trivial::Overrides::invalid_nonzero_quantity()),
        trivial::created(7),
    ])
    .expect("failed to build invalid trivial action");

    assert_err(prove_actions(&env, [&bad]).await)
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

    let bad = action::build([
        trivial::consumed(8, trivial::Overrides::invalid_non_ephemeral()),
        trivial::created(8),
    ])
    .expect("failed to build invalid trivial action");

    assert_err(prove_actions(&env, [&bad]).await)
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

    let witnesses = action::build([
        trivial::consumed(11, trivial::Overrides::default()),
        trivial::created(11),
    ])
    .context("failed to build trivial action")?;
    let mut tx = prove_actions(&env, [&witnesses])
        .await
        .context("valid witnesses should prove before tampering")?;

    tamper(&mut tx).context("failed to tamper transaction proof")?;

    assert_err(execute_tx(&mut env, tx).await)
}
