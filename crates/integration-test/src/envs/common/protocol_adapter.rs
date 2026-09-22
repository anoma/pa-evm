use alloy::primitives::B256;
use alloy::providers::DynProvider;
use anoma_pa_evm_bindings::generated::protocol_adapter::{
    IProtocolAdapter, ProtocolAdapter as PaContract,
};
use anoma_pa_testkit::environment::ProtocolAdapter as CoreProtocolAdapter;
use anoma_pa_testkit::environment::Transaction as CoreTransaction;
use anoma_pa_testkit::transaction::Transaction;
use anoma_rm_risc0::transaction::Transaction as ArmTxn;
use anyhow::Context;
use risc0_zkvm::Digest;

use super::commitment_tree::CommitmentTree;

pub struct ProtocolAdapter {
    pub pa: PaContract::ProtocolAdapterInstance<DynProvider>,
    pub commitment_tree: CommitmentTree,
}

impl ProtocolAdapter {
    /// Wraps the adapter, with the commitment tree it holds.
    pub(in crate::envs) async fn new(
        pa: PaContract::ProtocolAdapterInstance<DynProvider>,
    ) -> anyhow::Result<Self> {
        let commitment_tree = CommitmentTree::read(&pa)
            .await
            .context("failed to read the commitment tree")?;
        Ok(Self {
            pa,
            commitment_tree,
        })
    }

    async fn assert_root_consistency(&self, tx: &ArmTxn) -> anyhow::Result<()> {
        let pa_root = self.commitment_tree.ensure_latest_root(&self.pa).await?;

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
                     pa_latest={pa_root:?}"
                );
            }
        }

        Ok(())
    }
}

impl CoreProtocolAdapter for ProtocolAdapter {
    type Transaction = Transaction;
    type CommitmentTree = CommitmentTree;

    async fn execute(&mut self, transaction: Self::Transaction) -> anyhow::Result<()> {
        let created_commitments: Vec<Digest> = transaction.created_commitments()?.collect();
        let tx = transaction.into_arm();

        self.assert_root_consistency(&tx).await?;

        let pa_tx: IProtocolAdapter::Transaction = tx.into();

        super::execute::execute_on_pa(&self.pa, pa_tx).await?;

        self.commitment_tree.add_leaves(created_commitments);

        Ok(())
    }

    fn commitment_tree(&self) -> &Self::CommitmentTree {
        &self.commitment_tree
    }
}
