//! The commitment tree of a protocol adapter. The adapter stores only its commitment count and, for each level, the
//! last left node (its sides), so the tree starts from these and adds the leaves the tests create.

use alloy::primitives::B256;
use alloy::providers::DynProvider;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter as PaContract;
use anoma_pa_testkit::environment::CommitmentTree as CoreCommitmentTree;
use anoma_rm_risc0::merkle_path::{MerklePath, PADDING_LEAF};
use anoma_rm_risc0::utils::hash_two;
use anyhow::Context;
use risc0_zkvm::Digest;

pub struct CommitmentTree {
    /// The adapter's commitment count when the tree was read.
    commitment_count: usize,
    /// The adapter's sides when the tree was read, as `commitmentTreeSides` returns them.
    sides: Vec<Digest>,
    /// The leaves added since, in order.
    leaves: Vec<Digest>,
}

impl CommitmentTree {
    /// Reads the tree the adapter holds.
    pub(in crate::envs) async fn read(
        pa: &PaContract::ProtocolAdapterInstance<DynProvider>,
    ) -> anyhow::Result<Self> {
        let count = pa
            .commitmentCount()
            .call()
            .await
            .context("failed to query the commitment count")?;
        let commitment_count =
            usize::try_from(count).context("the commitment count exceeds usize")?;

        let sides = pa
            .commitmentTreeSides()
            .call()
            .await
            .context("failed to query the commitment tree sides")?;
        anyhow::ensure!(
            sides.len() == depth_at(commitment_count),
            "the adapter returns {} sides at {commitment_count} leaves",
            sides.len()
        );

        let tree = Self {
            commitment_count,
            sides: sides
                .into_iter()
                .map(|side| Digest::from_bytes(side.0))
                .collect(),
            leaves: Vec::new(),
        };
        tree.ensure_latest_root(pa).await?;
        Ok(tree)
    }

    /// Checks that the tree gives the root the adapter reports as its latest, and returns that root.
    pub(in crate::envs) async fn ensure_latest_root(
        &self,
        pa: &PaContract::ProtocolAdapterInstance<DynProvider>,
    ) -> anyhow::Result<B256> {
        let root = B256::from_slice(self.root()?.as_bytes());
        let latest = pa
            .latestCommitmentTreeRoot()
            .call()
            .await
            .context("failed to query the latest commitment tree root")?;
        anyhow::ensure!(
            root == latest,
            "the tree gives the root {root}, the adapter reports {latest}"
        );
        Ok(latest)
    }

    /// Adds the commitments a transaction created as the next leaves, in order.
    pub(in crate::envs) fn add(&mut self, commitments: impl IntoIterator<Item = Digest>) {
        self.leaves.extend(commitments);
    }

    fn count(&self) -> usize {
        self.commitment_count + self.leaves.len()
    }

    /// The node at `level` and `index`, from the added leaves, the sides, or the empty subtree.
    fn node(&self, level: usize, index: usize) -> Digest {
        if index << level >= self.count() {
            return empty(level);
        }
        if (index + 1) << level <= self.commitment_count {
            // A path reaches a node over read commitments only as its level's last left node, which the sides hold.
            return self.sides[level];
        }
        if level == 0 {
            return self.leaves[index - self.commitment_count];
        }
        hash_two(
            &self.node(level - 1, 2 * index),
            &self.node(level - 1, 2 * index + 1),
        )
    }
}

impl CoreCommitmentTree for CommitmentTree {
    fn root(&self) -> anyhow::Result<Digest> {
        Ok(self.node(depth_at(self.count()), 0))
    }

    fn path_to(&self, leaf: Digest) -> anyhow::Result<MerklePath> {
        let position = self
            .leaves
            .iter()
            .position(|added| *added == leaf)
            .context("the tests did not add this leaf")?;
        let index = self.commitment_count + position;
        let path: Vec<_> = (0..depth_at(self.count()))
            .map(|level| {
                let node = index >> level;
                (self.node(level, node ^ 1), node % 2 == 1)
            })
            .collect();
        Ok(MerklePath::from_path(&path))
    }
}

/// The depth of the adapter's tree at `count` leaves. It grows by one level each time it fills up.
fn depth_at(count: usize) -> usize {
    (usize::BITS - count.leading_zeros()) as usize
}

/// The root of an empty subtree of height `level`.
fn empty(level: usize) -> Digest {
    (0..level).fold(*PADDING_LEAF, |node, _| hash_two(&node, &node))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anoma_rm_risc0::action_tree::ActionTree;

    fn leaves(count: usize) -> Vec<Digest> {
        (1..=count)
            .map(|seed| Digest::from([seed as u32; 8]))
            .collect()
    }

    /// The tree over all leaves, one level deeper when full, as the adapter grows it.
    fn reference(leaves: &[Digest]) -> ActionTree {
        let mut padded = leaves.to_vec();
        if padded.is_empty() || padded.len().is_power_of_two() {
            padded.push(*PADDING_LEAF);
        }
        ActionTree::new(padded)
    }

    /// The tree read after the first `commitment_count` leaves, with the rest added since. A side no path reads
    /// stays zero.
    fn read_after(leaves: &[Digest], commitment_count: usize) -> CommitmentTree {
        let sides = (0..depth_at(commitment_count))
            .map(|level| {
                let node = commitment_count >> level;
                if node.is_multiple_of(2) {
                    return Digest::default();
                }
                let start = (node - 1) << level;
                ActionTree::new(leaves[start..start + (1 << level)].to_vec())
                    .root()
                    .unwrap()
            })
            .collect();
        CommitmentTree {
            commitment_count,
            sides,
            leaves: leaves[commitment_count..].to_vec(),
        }
    }

    #[test]
    fn root_matches_the_tree_over_all_leaves() {
        for count in 0..=33 {
            let all = leaves(count);
            let expected = reference(&all).root().unwrap();
            for commitment_count in 0..=count {
                assert_eq!(
                    read_after(&all, commitment_count).root().unwrap(),
                    expected,
                    "{count} leaves, {commitment_count} read"
                );
            }
        }
    }

    #[test]
    fn path_to_matches_the_tree_over_all_leaves() {
        for count in 1..=33 {
            let all = leaves(count);
            let expected = reference(&all);
            for commitment_count in 0..count {
                let tree = read_after(&all, commitment_count);
                for leaf in &all[commitment_count..] {
                    assert_eq!(
                        tree.path_to(*leaf).unwrap(),
                        expected.generate_path(leaf).unwrap(),
                        "{count} leaves, {commitment_count} read"
                    );
                }
            }
        }
    }
}
