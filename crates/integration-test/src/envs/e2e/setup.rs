use alloy::node_bindings::Anvil;
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::ext::AnvilApi;
use anoma_pa_evm_bindings::addresses::{
    Environment as DeploymentEnvironment, protocol_adapter_address,
};
use anoma_pa_evm_bindings::helpers::alchemy_url;
use anoma_pa_testkit::environment::StateBuilder;
use anoma_pa_testkit::fixtures::identities;

use crate::keychain::EvmSigner;
use anoma_pa_testkit::prover::QueueProver;
use anyhow::Context;

use crate::deploy::pa::protocol_adapter;
use crate::state::actors::insert_default_signer;
use crate::state::chains::insert_chain;
use crate::state::pa::insert_pa_address;

use super::CommitmentTree;
use super::Environment;
use super::ProtocolAdapter;
use super::config::E2eConfig;

impl Environment {
    pub async fn setup_bare() -> anyhow::Result<Self> {
        Self::setup(async |_| anyhow::Ok(())).await
    }

    pub async fn setup<F>(insert_additional: F) -> anyhow::Result<Self>
    where
        F: AsyncFnOnce(&mut StateBuilder) -> anyhow::Result<()>,
    {
        let config = E2eConfig::from_env().context("failed to parse e2e test config")?;
        let chain = config.chain;

        // Fork the real chain. The protocol adapter is already deployed and
        // initialized there, so we read its address from the bindings rather than
        // deploying a fresh one; forking keeps real on-chain state from being
        // mutated.
        let fork_url = alchemy_url(&chain)
            .with_context(|| format!("failed to resolve fork RPC url for chain {chain:?}"))?;

        // Forking a live chain can be slow; give anvil a generous boot window.
        let anvil = Anvil::new()
            .fork(fork_url.to_string())
            .timeout(30_000) // 30 seconds
            .spawn();

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

        let pa_address = protocol_adapter_address(DeploymentEnvironment::Staging, &chain)
            .with_context(|| format!("no protocol adapter deployment for chain {chain:?}"))?;
        let pa = protocol_adapter(pa_address, provider.clone());

        let prover = QueueProver::new(&config.queue_base_url, &config.queue_auth_token)
            .context("failed to build queue prover")?;

        let state = {
            let mut builder = StateBuilder::new();

            insert_default_signer(&mut builder, provider.clone());
            insert_chain(&mut builder, chain);
            insert_pa_address(&mut builder, pa_address);
            insert_additional(&mut builder)
                .await
                .context("failed to insert additional data into state")?;

            builder.finalize()
        };

        Ok(Self {
            anvil,
            state,
            prover,
            protocol_adapter: ProtocolAdapter {
                pa,
                commitment_tree: CommitmentTree::default(),
            },
        })
    }
}
