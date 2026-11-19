//! Deploys risc0's mock verifier stack for the local environment — the
//! `RiscZeroMockVerifier` wrapped in a `RiscZeroVerifierEmergencyStop` and
//! registered in a `RiscZeroVerifierRouter`, mirroring risc0-ethereum's
//! `DeployRiscZeroContractsMock` script. The contract bindings come from
//! `anoma-risc0-verifier-bindings`. The mock verifier accepts seals of the
//! form `SELECTOR ++ ReceiptClaim::ok(imageId, journalDigest).digest()`, which is
//! exactly what the testkit's local prover emits.

use alloy::primitives::Address;
use alloy::primitives::FixedBytes;
use alloy::providers::DynProvider;
use anoma_risc0_verifier_bindings::generated::risc_zero_mock_verifier::RiscZeroMockVerifier;
use anoma_risc0_verifier_bindings::generated::risc_zero_verifier_emergency_stop::RiscZeroVerifierEmergencyStop;
use anoma_risc0_verifier_bindings::generated::risc_zero_verifier_router::RiscZeroVerifierRouter;
use anyhow::Context;

/// Selector the mock verifier is deployed and registered under (matches
/// `MOCK_VERIFIER_SELECTOR` in risc0-ethereum's mock deploy script).
pub const MOCK_VERIFIER_SELECTOR: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

#[derive(Clone)]
pub struct MockRisc0Stack {
    pub router: RiscZeroVerifierRouter::RiscZeroVerifierRouterInstance<DynProvider>,
}

pub async fn deploy_mock_risc0_stack(
    provider: &DynProvider,
    guardian: Address,
) -> anyhow::Result<MockRisc0Stack> {
    let selector = FixedBytes::<4>::from(MOCK_VERIFIER_SELECTOR);

    let mock_verifier = RiscZeroMockVerifier::deploy(provider.clone(), selector)
        .await
        .context("failed to deploy RiscZeroMockVerifier")?;

    let emergency_stop =
        RiscZeroVerifierEmergencyStop::deploy(provider.clone(), *mock_verifier.address(), guardian)
            .await
            .context("failed to deploy RiscZeroVerifierEmergencyStop")?;

    let router = RiscZeroVerifierRouter::deploy(provider.clone(), guardian)
        .await
        .context("failed to deploy RiscZeroVerifierRouter")?;

    router
        .addVerifier(selector, *emergency_stop.address())
        .send()
        .await
        .context("failed to submit addVerifier call to verifier router")?
        .get_receipt()
        .await
        .context("failed to fetch addVerifier receipt from verifier router")?;

    Ok(MockRisc0Stack { router })
}
