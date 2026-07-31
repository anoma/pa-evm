//! Helpers shared between the integration test binaries.

// Each test binary compiles only the subset of helpers it uses.
#![allow(dead_code)]

use alloy::primitives::B256;
use alloy::providers::DynProvider;
use anoma_pa_evm_integration_test::deploy::pa::protocol_adapter;
use anoma_pa_evm_integration_test::state::actors::default_signer;
use anoma_pa_evm_integration_test::state::pa::pa_address;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::prove_actions;
use anoma_pa_testkit::transaction::Transaction;
use anoma_risc0_verifier_bindings::generated::risc_zero_verifier_router::RiscZeroVerifierRouter;
use anyhow::Context;
use risc0_zkvm::sha::Digestible;

/// Proves a transaction with a single trivial action, without executing it.
pub async fn prove_trivial_tx<Env>(env: &Env) -> anyhow::Result<Transaction>
where
    Env: Environment<Transaction = Transaction>,
{
    let action = trivial::build(1, trivial::Overrides::default())
        .context("failed to build trivial action")?
        .witnesses;

    prove_actions(env, &[action]).await
}

/// Connects to the verifier router the protocol adapter is configured with.
pub async fn router<Env>(
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
pub fn journal_digest(journal_bytes: &[u8]) -> B256 {
    let digest = risc0_zkvm::Journal::new(journal_bytes.to_vec()).digest();

    B256::from_slice(digest.as_bytes())
}
