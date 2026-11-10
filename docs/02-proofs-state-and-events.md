# Proofs, State and Events

This document specifies the proofs that the protocol adapter verifies, the storage of the commitment tree and the nullifier set, and the events. The glossary [`CONTEXT.md`](../CONTEXT.md#language) defines the terms.

## 1. Proofs

The protocol adapter verifies two proofs for each transaction, after it processed all actions. Both verifications are out-of-circuit checks:

- **The delta proof.** An ECDSA signature over the Keccak-256 hash of the action tree roots, concatenated in the order of the actions. So the signature also fixes the order of the actions. The protocol adapter adds the deltas of all actions and checks that the signer is the key that this sum gives (`DeltaMismatch`). This hash is also the transaction ID.
- **The aggregation proof.** One RISC Zero proof of the batch aggregation circuit for the EVM (`VerifyingKeys._BATCH_AGGREGATION_EVM`). It aggregates the compliance proof of each action and the resource logic proof of each resource, so it shows their in-circuit checks.

The protocol adapter does not receive the journal of the aggregation proof. It calculates the journal digest itself: the SHA-256 hash of the ABI encoding of the verifying key of the compliance circuit (`VerifyingKeys._COMPLIANCE`), the kind tree root of the transaction and the actions. So the transaction cannot select another compliance circuit ([ADR-0001](./adr/0001-journal-constants-are-injected-not-passed-in-calldata.md)), and the journal has the ABI encoding ([ADR-0002](./adr/0002-the-aggregation-journal-is-abi-encoded-not-risc0-serde.md)). Only an upgrade of the protocol adapter can change the verifying keys.

`execute` reverts for a transaction without actions (`EmptyTransactionNotAllowed`). So the protocol adapter verifies both proofs for each transaction.

## 2. Commitment tree

The commitment tree stores the commitment count, one left sibling for each level below the root and one empty subtree root for each level up to the root, instead of its leaves. On each level where the commitment count has a one bit, the left sibling is the sibling on that level in the Merkle path of the next commitment. The other left siblings are old values, which the tree does not read.

- At setup, the tree stores the empty subtree root of the leaf level. Each commitment that fills the tree adds a level: one left sibling and one empty subtree root. So the depth of the tree, the number of levels below the root, is the number of binary digits of the commitment count. For example, a tree with four commitments has depth 3.
- To add a commitment that does not fill the tree, the tree writes one left sibling and the commitment count, and it calculates no root.
- The tree calculates the latest root when the root is read, from the left siblings. It calculates the empty subtree roots again, so the stored ones enter no root.
- After the last action of a transaction that adds commitments, the protocol adapter stores the latest root as a historical root. A mapping only shows whether a root is historical (`isCommitmentTreeRootHistorical`).

`commitmentTreeLeftSiblings()` and `commitmentTreeEmptySubtreeRoots()` return the stored values, so that a client can calculate Merkle paths and check the tree.

## 3. Nullifier set

The nullifier set is a mapping that only shows whether it contains a nullifier (`isNullifierContained`).

## 4. Events

| Event                                                                          | Emitted                                                                                                                                                                               |
| ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `TransactionExecuted`                                                          | After the last action of a transaction, with the transaction ID                                                                                                                       |
| `ActionExecuted`                                                               | After the last resource of an action, with the action tree root, and the nullifiers and the commitments with their logic refs, in two arrays. Each array keeps the order of the list. |
| `ForwarderCallExecuted`                                                        | After each external call, with the forwarder, the input and the output ([`04-resource-order-and-external-calls.md`](./04-resource-order-and-external-calls.md))                       |
| `ResourcePayload`, `DiscoveryPayload`, `ExternalPayload`, `ApplicationPayload` | For each app data blob whose deletion criterion is `Never`                                                                                                                            |
| `CommitmentTreeRootAdded`                                                      | When the protocol adapter stores a historical root                                                                                                                                    |
| `KindTreeRootUpdated`, `LogicRefDenied`                                        | When the owner changes a control ([`03-kind-tree-root-and-denylists.md`](./03-kind-tree-root-and-denylists.md))                                                                       |
| `Initialized`, `Upgraded`, `OwnershipTransferred`, `Paused`, `Unpaused`        | From OpenZeppelin                                                                                                                                                                     |

The migrational implementation emits three more events ([`06-state-migration.md`](./06-state-migration.md)).

## 5. Trust assumptions

- **The RISC Zero verifier accepts only valid proofs.** The protocol adapter uses the fields of each resource (consumed or created, tag, logic ref, commitment tree root, app data), the delta and the action tree root of each action, and the kind tree root as the transaction gives them. It trusts them because the journal digest, which it calculates from these fields, must match the aggregation proof. So the in-circuit checks apply to the fields that the protocol adapter processes.
