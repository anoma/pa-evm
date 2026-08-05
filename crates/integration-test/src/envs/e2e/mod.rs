use alloy::node_bindings::AnvilInstance;
use alloy::primitives::B256;
use alloy::providers::DynProvider;
use anoma_pa_evm_bindings::generated::protocol_adapter::{ProtocolAdapter as PaContract, Types};
use anoma_pa_testkit::environment::CommitmentTree as CoreCommitmentTree;
use anoma_pa_testkit::environment::Environment as CoreEnvironment;
use anoma_pa_testkit::environment::ProtocolAdapter as CoreProtocolAdapter;
use anoma_pa_testkit::environment::State;
use anoma_pa_testkit::environment::Transaction as CoreTransaction;
use anoma_rm_risc0::action_tree::ActionTree as ArmTree;
use anoma_rm_risc0::merkle_path::MerklePath;
use anoma_rm_risc0::transaction::Transaction as ArmTxn;
use anyhow::Context;
use risc0_zkvm::Digest;

pub mod config;
mod setup;

pub use anoma_pa_testkit::transaction::Transaction;
pub use config::E2eConfig;

pub struct Environment {
    pub anvil: AnvilInstance,
    pub state: State,
    pub prover: anoma_pa_testkit::prover::QueueProver,
    pub protocol_adapter: ProtocolAdapter,
}

pub struct ProtocolAdapter {
    pub pa: PaContract::ProtocolAdapterInstance<DynProvider>,
    pub commitment_tree: CommitmentTree,
}

#[derive(Default)]
pub struct CommitmentTree {
    leaves: Vec<Digest>,
}

impl CoreEnvironment for Environment {
    type Transaction = Transaction;
    type ProtocolAdapter = ProtocolAdapter;
    type Prover = anoma_pa_testkit::prover::QueueProver;

    fn prover(&self) -> &Self::Prover {
        &self.prover
    }

    fn state(&self) -> &State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    fn protocol_adapter(&self) -> &Self::ProtocolAdapter {
        &self.protocol_adapter
    }

    fn protocol_adapter_mut(&mut self) -> &mut Self::ProtocolAdapter {
        &mut self.protocol_adapter
    }
}

impl CoreProtocolAdapter for ProtocolAdapter {
    type Transaction = Transaction;
    type CommitmentTree = CommitmentTree;

    async fn execute(&mut self, transaction: Self::Transaction) -> anyhow::Result<()> {
        let created_commitments: Vec<Digest> = transaction.created_commitments()?.collect();
        let tx = transaction.into_arm();

        self.assert_root_consistency(&tx).await?;

        let pa_tx: Types::Transaction = tx.into();

        crate::envs::common::execute::execute_on_pa(&self.pa, pa_tx).await?;

        self.commitment_tree.leaves.extend(created_commitments);

        Ok(())
    }

    fn commitment_tree(&self) -> &Self::CommitmentTree {
        &self.commitment_tree
    }
}

impl ProtocolAdapter {
    async fn assert_root_consistency(&self, tx: &ArmTxn) -> anyhow::Result<()> {
        let local_root = self.commitment_tree.root()?;
        let pa_root = self
            .pa
            .latestCommitmentTreeRoot()
            .call()
            .await
            .context("failed to query latest commitment tree root from protocol adapter")?;

        let local_root_b256 = B256::from_slice(local_root.as_bytes());
        anyhow::ensure!(
            local_root_b256 == pa_root,
            "commitment tree root mismatch before execution: local={local_root_b256:?}, pa={pa_root:?}"
        );

        let aggregation = tx
            .aggregation
            .as_ref()
            .context("the transaction must be aggregated")?;

        for (action_idx, action) in aggregation.instance.actions.iter().enumerate() {
            for (resource_idx, consumed) in action.consumed_publics.iter().enumerate() {
                let consumed_root = B256::from_slice(consumed.commitment_tree_root.as_bytes());
                let contained = self
                    .pa
                    .isCommitmentTreeRootContained(consumed_root)
                    .call()
                    .await
                    .with_context(|| {
                        format!(
                            "failed to query root containment for action {action_idx} consumed \
                             resource {resource_idx}"
                        )
                    })?;

                anyhow::ensure!(
                    contained,
                    "consumed commitment tree root not found in PA for action {action_idx} \
                     consumed resource {resource_idx}: root={consumed_root:?}, \
                     pa_latest={pa_root:?}, local_latest={local_root_b256:?}"
                );
            }
        }

        Ok(())
    }
}

impl CoreCommitmentTree for CommitmentTree {
    fn root(&self) -> anyhow::Result<Digest> {
        if self.leaves.is_empty() {
            return Ok(*anoma_rm_risc0::compliance::INITIAL_ROOT);
        }

        Ok(self.build_tree().root()?)
    }

    fn path_to(&self, leaf: Digest) -> anyhow::Result<MerklePath> {
        Ok(self.build_tree().generate_path(&leaf)?)
    }
}

impl CommitmentTree {
    fn build_tree(&self) -> ArmTree {
        let mut leaves = self.leaves.clone();
        if leaves.is_empty() || leaves.len().is_power_of_two() {
            leaves.push(*anoma_rm_risc0::merkle_path::PADDING_LEAF);
        }
        ArmTree::new(leaves)
    }
}
