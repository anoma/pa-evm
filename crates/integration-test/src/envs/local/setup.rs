use alloy::node_bindings::Anvil;
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::ext::AnvilApi;
use alloy_chains::NamedChain;
use anoma_pa_testkit::environment::StateBuilder;
use anoma_pa_testkit::fixtures::identities;

use crate::keychain::EvmSigner;
use anyhow::Context;

use crate::deploy::pa::protocol_adapter;
use crate::state::actors::insert_default_signer;
use crate::state::chains::insert_chain;
use crate::state::pa::insert_pa_address;

use super::{CommitmentTree, Environment, ProtocolAdapter};
use crate::deploy::mock_risc0_bindings::{MOCK_VERIFIER_SELECTOR, deploy_mock_risc0_stack};

impl Environment {
    pub async fn setup_bare() -> anyhow::Result<Self> {
        Self::setup(async |_| anyhow::Ok(())).await
    }

    pub async fn setup<F>(insert_additional: F) -> anyhow::Result<Self>
    where
        F: AsyncFnOnce(&mut StateBuilder) -> anyhow::Result<()>,
    {
        let anvil = Anvil::new().spawn();

        let signer = identities::alice()?.signer();
        let deployer = signer.address();

        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect_http(anvil.endpoint_url())
            .erased();

        provider
            .anvil_set_balance(
                deployer,
                parse_ether("100").context("failed to parse deployer balance amount")?,
            )
            .await?;

        let pa_address = deploy_protocol_adapter(&provider, deployer).await?;
        let pa = protocol_adapter(pa_address, provider.clone());

        let chain_id = provider.get_chain_id().await?;
        let named_chain = NamedChain::try_from(chain_id)
            .with_context(|| format!("unsupported chain id {chain_id}"))?;

        let state = {
            let mut builder = StateBuilder::new();

            insert_default_signer(&mut builder, provider.clone());
            insert_chain(&mut builder, named_chain);
            insert_pa_address(&mut builder, pa_address);
            insert_additional(&mut builder)
                .await
                .context("failed to insert additional data into state")?;

            builder.finalize()
        };

        Ok(Self {
            anvil,
            state,
            prover: anoma_pa_testkit::prover::LocalProver,
            protocol_adapter: ProtocolAdapter {
                pa,
                commitment_tree: CommitmentTree::default(),
            },
        })
    }
}

/// Deploys a protocol adapter backed by a freshly deployed mock Risc0 verifier
/// stack, returning its address.
async fn deploy_protocol_adapter(
    default_signer: &alloy::providers::DynProvider,
    fee_recipient: alloy::primitives::Address,
) -> anyhow::Result<alloy::primitives::Address> {
    use alloy::primitives::FixedBytes;
    use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter;

    let mock_risc0 = deploy_mock_risc0_stack(default_signer, fee_recipient)
        .await
        .context("failed to deploy mock Risc0 verifier stack")?;
    let selector = FixedBytes::<4>::from(MOCK_VERIFIER_SELECTOR);

    let deployed = ProtocolAdapter::deploy(
        default_signer.clone(),
        *mock_risc0.router.address(),
        selector,
    )
    .await
    .context("failed to deploy protocol adapter")?;

    Ok(*deployed.address())
}
