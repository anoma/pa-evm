# State Migration

This document specifies the state migration: the copy of the commitment tree and the nullifier set of a legacy protocol adapter into the protocol adapter. The glossary [`CONTEXT.md`](../CONTEXT.md#language) defines the terms.

## 1. Purpose

The legacy protocol adapter of a chain holds its commitment tree and its nullifier set. Without the commitment tree, no user can prove that a resource exists. Without the nullifier set, the protocol adapter cannot reject a second consumption of a resource. The legacy protocol adapter is immutable, so the protocol adapter copies its state once. The copy changes no resource.

In production, each chain whose legacy protocol adapter holds resources migrates. On a chain without a legacy protocol adapter, or with an empty legacy commitment tree, the proxy starts on the plain implementation, because `migrateCommitmentTree` rejects an empty tree. In staging, a proxy can also start on the plain implementation beside a legacy protocol adapter that still works, for example on Base Sepolia. Then the two contracts keep separate states, and no tokens move.

## 2. The migrational implementation

The proxy of a migrated chain starts on `MigrationalProtocolAdapter`, a `ProtocolAdapter` with these differences:

- It starts paused, so it executes no transaction before the copy is complete.
- Two functions copy the state. Only the owner can call them, and only while the protocol adapter is paused and the legacy protocol adapter is stopped (`LegacyProtocolAdapterNotStopped`). So the state that they read does not change.
- `unpause` lifts the pause only after a complete copy.

**`migrateCommitmentTree(leftSiblings)`** writes the commitment tree in one call. The left siblings are the hashes from which the tree calculates its root: on each level where the commitment count has a one bit, the sibling on that level in the Merkle path of the next commitment. The legacy protocol adapter has no function that returns them, so the caller reads them from its storage. The function reads the commitment count and the latest root from the legacy protocol adapter. It reverts if the tree of the protocol adapter is not empty (`CommitmentTreeNotEmpty`), if the legacy tree is empty (`EmptyCommitmentTreeNotAllowed`), or if the count does not fit the depth that the left siblings give (`LeafCountExceedsCapacity`). It also reverts if the calculated root is not the latest root of the legacy protocol adapter (`CommitmentTreeRootMismatch`). This check fixes the depth and the left siblings on the levels where the count has a one bit. The tree overwrites the other levels before it reads them. It stores the root as a historical root and emits `CommitmentTreeMigrated`.

**`migrateNullifierSet(start, count)`** copies `count` nullifiers from the index `start`. It reads each nullifier by its index from the legacy protocol adapter, in the order in which that contract added them. It reverts if the batch ends after the last legacy nullifier (`NullifierBatchOutOfRange`), if the set does not contain the nullifier at the index `start − 1` (`NullifierBatchLeavesGap`), or if the set already contains a nullifier of the batch (`PreExistingNullifier`). So no batch skips a nullifier or copies one twice. It emits `NullifierBatchMigrated`.

**`unpause`** reverts unless the protocol adapter has the same commitment count (`CommitmentCountMismatch`) and the same latest root (`CommitmentTreeRootMismatch`) as the legacy protocol adapter, both the root of the empty tree and that latest root are historical roots (`MissingHistoricalRoot`), and the set contains the last legacy nullifier (`MissingNullifier`). The batches copy the nullifiers in order, so the last one shows a complete copy.

**The upgrade to the plain implementation** removes the copy functions, so each state migration ends with it. The migrational implementation emits `MigrationFinalized` when it authorizes the upgrade. It has no storage of its own.

**Historical roots.** The legacy protocol adapter keeps every root that it stored. The protocol adapter keeps two: the root of the empty tree, against which a transaction proves each consumed ephemeral resource, and the latest root of the legacy protocol adapter. A prover with a Merkle path for an older root calculates a new Merkle path for the latest root.

## 3. Events and indexers

The copy emits no event for each commitment or nullifier, only `CommitmentTreeRootAdded` and `CommitmentTreeMigrated` for the tree and `NullifierBatchMigrated` for each batch. So an indexer of a migrated chain reads both contracts: the legacy protocol adapter until the stop, and the protocol adapter after the copy. It builds one commitment tree for the chain in this order. The protocol adapter does not reject a repeated commitment, so each commitment in an event is one leaf.

## 4. Trust assumptions

- **The pause lasts until the tokens of the legacy ERC20 forwarder move.** `unpause` checks the copy, but not the move of the tokens. The kind tree lets legacy-forwarder resources convert and unwrap at the ERC20 forwarder, which holds the tokens that back them only after the move. If the pause ends before the move, legacy-forwarder resources can unwrap tokens that back other resources.
- **The logic ref of the legacy ERC20 forwarder is deprecated before the pause ends.** The kind tree makes the kinds of the legacy-forwarder resources and the kinds of the ERC20 forwarder aliases in both directions. Without the deprecation, a transaction could convert resources of the ERC20 forwarder into new legacy-forwarder resources.
- **The pause ends before the upgrade to the plain implementation.** The `_authorizeUpgrade` of the migrational implementation checks neither the copy nor the pause. Only `unpause` checks the copy.
- **The deployment wallet uses its ownership only for the state migration.** It owns the protocol adapter in both environments, and it receives the legacy protocol adapter from the PA Safe and stops it. In production, the PA Safe receives the protocol adapter after the upgrade to the plain implementation. Each transfer of ownership completes in one step.
- **The copy completes.** A protocol adapter with an incomplete copy stays paused, and no other deployment can take its CREATE2 address.

## 5. Parameters

| Name                             | Value                   | Use                                                          |
| -------------------------------- | ----------------------- | ------------------------------------------------------------ |
| `Parameters.IMPLEMENTATION_SALT` | `"ProtocolAdapterImpl"` | CREATE2 salt of the migrational and the plain implementation |

The deployment record, [`crates/bindings/deployments.json`](../crates/bindings/deployments.json), lists the legacy protocol adapter of each chain under `legacy`.
