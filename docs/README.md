# Protocol Adapter

This folder specifies the protocol adapter, the contract that executes the transactions of the Anoma Resource Machine (ARM) on EVM chains. The glossary of this repository, [`CONTEXT.md`](../CONTEXT.md#language), defines the terms. [`RELEASE_CHECKLIST.md`](../RELEASE_CHECKLIST.md) and [`MIGRATION_CHECKLIST.md`](../MIGRATION_CHECKLIST.md) have the commands.

## 1. Summary

On each chain, the protocol adapter executes the transactions of the ARM. It verifies their proofs, keeps the commitment tree and the nullifier set, and makes the external calls of their resources. Its owner can upgrade it, pause it, set its kind tree root, and deprecate or deny logic refs.

## 2. Documents

| Document                                                                               | Content                                                                                                                 |
| -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| [`01-upgradeability-and-ownership.md`](./01-upgradeability-and-ownership.md)           | The proxy, its owners, the actions of the owner, and the controls for a flaw                                            |
| [`02-proofs-state-and-events.md`](./02-proofs-state-and-events.md)                     | The proofs that the protocol adapter verifies, the storage of the commitment tree and the nullifier set, and the events |
| [`03-kind-tree-root-and-denylists.md`](./03-kind-tree-root-and-denylists.md)           | The kind tree root, which decides which kinds are aliases, and the denylists of logic refs                              |
| [`04-resource-order-and-external-calls.md`](./04-resource-order-and-external-calls.md) | How the protocol adapter processes the resources of an action, and how their order sets the order of the external calls |
| [`05-legacy-protocol-adapter.md`](./05-legacy-protocol-adapter.md)                     | How the protocol adapter differs from the legacy protocol adapter                                                       |
| [`06-state-migration.md`](./06-state-migration.md)                                     | The copy of the commitment tree and the nullifier set of a legacy protocol adapter into the protocol adapter            |

Documents 05 and 06 cover the legacy stack and the migration from it.
