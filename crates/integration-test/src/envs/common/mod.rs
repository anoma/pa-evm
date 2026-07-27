//! Building blocks shared by the `local` and `e2e` environments: the on-chain
//! execution and revert-diagnostics glue, identical across both.

pub(in crate::envs) mod execute;
