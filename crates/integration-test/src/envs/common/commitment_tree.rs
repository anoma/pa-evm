use anoma_pa_testkit::environment::CommitmentTree as CoreCommitmentTree;
use anoma_rm_risc0::action_tree::ActionTree as ArmTree;
use anoma_rm_risc0::merkle_path::MerklePath;
use risc0_zkvm::Digest;

#[derive(Default)]
pub struct CommitmentTree {
    leaves: Vec<Digest>,
}

impl CommitmentTree {
    /// Adds the commitments a transaction created.
    pub(in crate::envs) fn extend(&mut self, commitments: impl IntoIterator<Item = Digest>) {
        self.leaves.extend(commitments);
    }

    fn build_tree(&self) -> ArmTree {
        let mut leaves = self.leaves.clone();
        if leaves.is_empty() || leaves.len().is_power_of_two() {
            leaves.push(*anoma_rm_risc0::merkle_path::PADDING_LEAF);
        }
        ArmTree::new(leaves)
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
