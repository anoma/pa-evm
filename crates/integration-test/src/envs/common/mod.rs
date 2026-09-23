//! Building blocks shared by the `local` and `e2e` environments: the protocol
//! adapter with its commitment tree, and the on-chain execution and
//! revert-diagnostics glue.

pub(in crate::envs) mod commitment_tree;
pub(in crate::envs) mod execute;
pub(in crate::envs) mod protocol_adapter;
