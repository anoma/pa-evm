# State Migration — Specification & Architecture

This document specifies the state migration: the one-time move of a chain from its legacy protocol adapter, an immutable contract, to the protocol adapter, an upgradeable proxy. The state migration is one part of the Galileo upgrade, which [`00-galileo-upgrade.md`](./00-galileo-upgrade.md) summarizes. This document gives the order of the steps in the staging and the production environment and the reasons for it, and it states how anoma-envio indexes the events of both contracts on one chain. The domain vocabulary is in [`CONTEXT.md`](../CONTEXT.md), and the commands are in [`MIGRATION_CHECKLIST.md`](../MIGRATION_CHECKLIST.md). Every later change of the protocol adapter is an upgrade, which [`02-upgrades.md`](./02-upgrades.md) specifies. How resource owners move their resources from one kind to another is in [`03-resource-kind-migration.md`](./03-resource-kind-migration.md).

## 1. Purpose

A chain that ran a legacy protocol adapter holds its resources in the commitment tree of that contract, and the nullifiers of its consumed resources in its nullifier set. The legacy protocol adapter is immutable, so no upgrade can change it. The chain therefore gets a protocol adapter at a new address, which copies both in. Without the commitment tree, no owner could prove that their resource exists. Without the nullifier set, a consumed resource could be consumed again.

The state migration is the only change of a chain's protocol adapter that is not an upgrade, and it happens once. A chain cannot go back, because nothing lifts the stop of its legacy protocol adapter. After the state migration, the chain runs the same plain implementation as every other chain, and each later change is an upgrade of its proxy. A chain without a legacy protocol adapter, or whose legacy protocol adapter holds no commitment, starts on the plain implementation and skips the state migration.

The legacy ERC20 forwarder accepts calls only from the legacy protocol adapter, so the chain also gets a new ERC20 forwarder, and the tokens move to it. That is the asset migration, which the ERC20 forwarder repository specifies. The state migration and the asset migration together leave each resource and its tokens where its owner can use them.

## 2. Three kinds of migration

Three different processes are called migration, and they move different things. This document covers the state migration. The documents in the last column cover the rest.

| Process | What moves | Who acts | How often | Specified in |
| --- | --- | --- | --- | --- |
| **State migration** and **asset migration** | The commitment tree and the nullifier set of the legacy protocol adapter are copied into the protocol adapter. The tokens of the legacy ERC20 forwarder move to the ERC20 forwarder. | The deployment wallet and the two Safes | Once per chain that ran a legacy protocol adapter | This document, and the ERC20 forwarder's [`01-asset-migration.md`](https://github.com/anoma/anomapay-erc20-forwarder/blob/next/docs/01-asset-migration.md) |
| **Soft migration** | Resource owners convert their resources of a deprecated logic ref into resources of the active logic ref. | Each resource owner | After each circuit release, and once for the legacy-forwarder resources of a migrated chain | [`03-resource-kind-migration.md`](./03-resource-kind-migration.md#4-soft-migration) |
| **Hard migration** | Resource owners create resources of the active logic ref from a proof that they hold unspent resources of a denied logic ref. | Each resource owner | After a flaw in a circuit | [`03-resource-kind-migration.md`](./03-resource-kind-migration.md#5-hard-migration) |

The state migration and the asset migration change no resource. A resource that the legacy protocol adapter created stays the same resource, of the same kind, in the copied commitment tree. Only a soft or a hard migration replaces a resource with a resource of another kind, and the owner of the resource authorizes it.

## 3. Contracts and owners

```mermaid
flowchart LR
    pasafe([PA Safe])
    wallet([Deployment wallet])
    legacy[(Legacy protocol adapter)]
    proxy[Protocol adapter proxy]
    migrational[MigrationalProtocolAdapter,<br/>migrational implementation]
    plain[ProtocolAdapter,<br/>plain implementation]

    pasafe -->|transferOwnership| legacy
    wallet -->|emergencyStop| legacy
    wallet -->|migrateCommitmentTree, migrateNullifierSet,<br/>unpause, upgradeToAndCall| proxy
    proxy -->|delegates to, until<br/>the completion run| migrational
    proxy -.->|delegates to, after<br/>the completion run| plain
    migrational -->|reads the commitment tree<br/>and the nullifier set| legacy
```

- **Legacy protocol adapter.** The immutable protocol adapter that a chain ran before. It has no kind table and no denylists. Its owner can only call `emergencyStop` and the `Ownable` functions, and no function lifts the stop.
- **Protocol adapter.** An ERC-1967 proxy that upgrades with UUPS, as [`02-upgrades.md`](./02-upgrades.md#1-contracts-and-owners) specifies. On a migrating chain, it starts on the migrational implementation and ends on the plain implementation.
- **Migrational implementation.** `MigrationalProtocolAdapter`, a protocol adapter that starts paused and copies the state of the legacy protocol adapter in. Section [The migrational implementation](#4-the-migrational-implementation) specifies it.
- **Plain implementation.** `ProtocolAdapter`, the implementation that every chain ends on.

| Contract | Before the state migration | During the state migration | After the state migration |
| --- | --- | --- | --- |
| Legacy protocol adapter | The PA Safe owns it. | The deployment wallet owns it from step 8 and stops it in step 9. | It stays stopped. |
| Protocol adapter proxy | — | The deployment wallet owns it, in both environments. | The deployment wallet owns it in staging, the PA Safe in production. |
| Legacy ERC20 forwarder | Its emergency committee is the FWD Safe. | The FWD Safe assigns `LegacyERC20ForwarderMigration` as its emergency caller. | The emergency caller stays assigned. |

The PA Safe and the FWD Safe are Safe multisigs, and each needs 3 of its 5 owners to confirm a transaction. Section [Parameters](#11-parameters) has their addresses.

## 4. The migrational implementation

`MigrationalProtocolAdapter` is a `ProtocolAdapter` with three changes:

- `initialize` pauses the proxy, so nothing executes before the state is in.
- Two owner-only functions copy the state in. Both run only while the proxy is paused and the legacy protocol adapter is stopped, so the state that they read cannot change.
- `unpause` proceeds only when the copy is complete.

**`migrateCommitmentTree(sides)`** writes the tree in one call, and only into an empty tree. The caller supplies the sides: the stored left-sibling hashes, one per level, which the legacy protocol adapter exposes through no getter. The function reads the leaf count and the latest root from the legacy protocol adapter, takes the depth from the number of sides, and computes the roots of the empty subtrees. It reverts unless the result reproduces the latest root of the legacy protocol adapter, so wrong sides cannot reach storage. It also reverts on an empty legacy tree.

**`migrateNullifierSet(start, count)`** copies `count` nullifiers from index `start` on, and reads each of them from the legacy protocol adapter. It reverts unless the set already contains the nullifier before `start`, so a batch cannot leave a gap. A nullifier that the set already contains makes the call revert, so a batch cannot copy a nullifier twice.

**`unpause`** checks that the proxy has the commitment count and the latest root of the legacy protocol adapter, that the empty-tree root and that latest root are historical roots, and that the set contains the last nullifier of the legacy protocol adapter. The batches copy the nullifiers in order, so the last one stands for all. A proxy that is short of a batch stays paused and does not run on part of the state.

**The upgrade to the plain implementation** removes both copy-in functions, and the migrational implementation emits `MigrationFinalized` when it authorizes it. The migrational implementation has no storage of its own, so the upgrade leaves no storage behind. Nothing else closes the copy-in, so every state migration must reach this upgrade.

**Historical roots.** The legacy protocol adapter keeps every root that it ever had. The protocol adapter keeps two of them: the empty-tree root, which a resource created and consumed in one transaction proves membership against, and the latest root of the legacy protocol adapter. A wallet that holds a Merkle path for an older root must compute a new one for the latest root.

## 5. Steps

[`MIGRATION_CHECKLIST.md`](../MIGRATION_CHECKLIST.md) has the commands. The steps are:

| # | Step | Signer |
| --- | --- | --- |
| 1 | Deploy and verify the plain implementation. Both runs refuse to start without it. | Deployment wallet |
| 2 | Deploy the proxy on the migrational implementation (`IS_MIGRATIONAL=true`). It starts paused, and the deployment wallet owns it in both environments, because it sends the calls of both runs. | Deployment wallet |
| 3 | Record the proxy in `deployments.json` and release the bindings. The runs and the kind tables read the proxy from the record. | — |
| 4 | Deploy the ERC20 forwarder and the generic call forwarder against the proxy. Record them, together with the legacy ERC20 forwarder of the chain. | Deployment wallet |
| 5 | Set the kind table root. The table has members under the label of the legacy ERC20 forwarder. | Deployment wallet, as proxy owner |
| 6 | Deprecate the logic ref that the legacy ERC20 forwarder accepts: `denyLogicRefs([(logicRef, false)])`. | Deployment wallet, as proxy owner |
| 7 | Read the latest root, the commitment count and the nullifier count of the legacy protocol adapter. The runs are checked against them. | — |
| 8 | Transfer the legacy protocol adapter to the deployment wallet. It keeps running, so this can happen days before step 9. | PA Safe |
| 9 | Stop the legacy protocol adapter with `emergencyStop()`. From here on, users cannot transact. | Deployment wallet, as legacy owner |
| 10 | The migration run: copy the commitment tree and the nullifier set. The proxy stays paused. | Deployment wallet |
| 11 | The asset migration: move the tokens of the legacy ERC20 forwarder to the ERC20 forwarder. | Deployment wallet, then the FWD Safe, then the deployment wallet |
| 12 | The completion run: lift the pause, upgrade to the plain implementation and, in production, transfer the proxy to the PA Safe. From here on, users can transact again. | Deployment wallet |
| 13 | Check the result against the legacy protocol adapter (`contracts-check-migration`). | — |

```mermaid
sequenceDiagram
    actor PASafe as PA Safe
    actor FWDSafe as FWD Safe
    participant Wallet as Deployment wallet
    participant Legacy as Legacy protocol adapter
    participant Proxy as Protocol adapter proxy
    participant Migration as LegacyERC20ForwarderMigration
    participant LegacyFwd as Legacy ERC20 forwarder
    participant Fwd as ERC20 forwarder

    rect rgba(128,128,128,0.1)
    Note over PASafe,Fwd: Before the stop
    Wallet->>Proxy: Deploy on MigrationalProtocolAdapter (paused)
    Wallet->>Fwd: Deploy against the proxy
    Wallet->>Proxy: setKindTableRoot (members under the legacy forwarder's label)
    Wallet->>Proxy: denyLogicRefs (deprecate the legacy logic ref)
    PASafe->>Legacy: transferOwnership(deployment wallet)
    end

    Wallet->>Legacy: emergencyStop()
    Note over Legacy,Proxy: Users cannot transact
    Wallet->>Proxy: migrateCommitmentTree(sides)
    loop Each batch of 200 nullifiers
        Wallet->>Proxy: migrateNullifierSet(start, count)
    end
    Wallet->>Migration: Deploy, propose as emergency caller
    FWDSafe->>LegacyFwd: setEmergencyCaller(migration)
    Wallet->>Migration: migrate(tokens)
    Migration->>LegacyFwd: forwardEmergencyCall(unwrap the full balance)
    LegacyFwd->>Fwd: Transfer each token
    Wallet->>Proxy: unpause() (checks the copied state)
    Wallet->>Proxy: upgradeToAndCall(ProtocolAdapter)
    Wallet->>Proxy: transferOwnership(PA Safe) (production only)
    Note over Legacy,Proxy: Users can transact again
```

## 6. Why this order

- **The kind table root and the deprecation go in before the stop.** A paused proxy accepts both calls, so they add no time to the period in which users cannot transact.
- **The proxy stays paused until the tokens moved.** The members under the label of the legacy ERC20 forwarder let a legacy-forwarder resource unwrap from the ERC20 forwarder. The ERC20 forwarder pays out only on calls from its protocol adapter, and a paused protocol adapter executes nothing. So no legacy-forwarder resource can unwrap before the ERC20 forwarder holds the tokens.
- **The deprecation goes in before the pause is lifted.** The members make legacy-forwarder resources fungible with resources of the ERC20 forwarder in both directions. Without the deprecation, a transaction could convert resources of the ERC20 forwarder into new legacy-forwarder resources. Legacy-forwarder resources stay consumable, so they still convert and unwrap.
- **The deployment wallet stops the legacy protocol adapter, not the PA Safe.** The stop and the migration run then follow each other without a Safe transaction between them. The transfer gives the deployment wallet nothing but the stop.
- **The completion run lifts the pause before it upgrades.** The `unpause` of the migrational implementation is what checks the copied state. The plain implementation does not check it.

## 7. Staging and production

| | Staging | Production |
| --- | --- | --- |
| Owner of the proxy during the state migration | the deployment wallet | the deployment wallet |
| Owner after the completion run | the deployment wallet | the PA Safe: the completion run transfers the proxy as its last call |
| Promotion gate during the state migration | accepts a proxy that runs the migrational implementation of the source | also accepts it, but fails for the chain until the PA Safe owns the proxy |
| Chains | Sepolia, as the rehearsal of every step | every chain whose legacy protocol adapter holds user funds, in one maintenance window |

The legacy ERC20 forwarder accepts one emergency caller and keeps it, so its tokens can move to the ERC20 forwarder of one environment only. The environment is chosen before the FWD Safe assigns the caller.

A chain can also start fresh in an environment while its legacy protocol adapter keeps running, as the staging deployment on Base Sepolia does. The protocol adapter then starts on the plain implementation with an empty tree, and the legacy-forwarder resources of that chain stay with the legacy contracts.

## 8. Duration

Users cannot transact from the stop of the legacy protocol adapter until the completion run lifts the pause, so every transaction is prepared before the stop. The migration run sends one transaction for the tree and one for each batch of `NULLIFIERS_PER_BATCH` (200) nullifiers. For example, the legacy protocol adapter on Sepolia holds 4,239 nullifiers, so its migration run sends 23 transactions. The completion run sends one transaction to lift the pause, one to upgrade and, in production, one to transfer the proxy. The recipes send each transaction only after the previous one is confirmed (`--slow`), so a revert stops the run. Each step of both runs is skipped once it is done, so a run that stops early is repeated until it reaches the end.

## 9. Events and the indexer

On a migrated chain, two contracts emitted the events of one commitment tree. The legacy protocol adapter emitted them for the commitments up to its stop, and the protocol adapter emits them for the commitments after the copy. The copy-in emits no event for a single copied commitment or nullifier. `migrateCommitmentTree` emits `CommitmentTreeRootAdded` with the latest root of the legacy protocol adapter, and `CommitmentTreeMigrated(root, leafCount)`. Each batch emits `NullifierBatchMigrated(start, count)`. So an indexer that reads only the protocol adapter misses every resource that the legacy protocol adapter created.

```mermaid
flowchart LR
    legacy[(Legacy protocol adapter)] -->|events up to the stop| envio[anoma-envio]
    proxy[Protocol adapter proxy] -->|events after the copy| envio
    envio -->|transactions, nullifiers and commitments<br/>of both contracts, per chain| galileo[galileo-indexer]
    galileo -->|one commitment tree per chain:<br/>roots and Merkle paths| backend[Backend]
```

[anoma-envio](https://github.com/anoma/anoma-envio) indexes both contracts of a migrated chain, on its branch `pa-v2`:

- **Two contract names and two ABIs.** Its config names the legacy protocol adapter `ProtocolAdapterV1` and the protocol adapter `ProtocolAdapterV2`. Nine events have the same signature in both contracts, and one handler serves both. `ActionExecuted` and `TransactionExecuted` differ, so each contract has its own handlers for them. The `ActionExecuted` of the legacy protocol adapter carries only the action tree root and the number of tags, so its handler reads the nullifiers and the commitments from the calldata. The `ActionExecuted` of the protocol adapter carries them, with their logic refs. Only the protocol adapter emits events for its kind table root, its denylists and its upgrades.
- **The contracts of each chain.** On a migrated chain, the config lists the legacy protocol adapter from its deployment block and the protocol adapter from its own deployment block. On a chain whose protocol adapter started fresh, the config lists only the protocol adapter, because the legacy protocol adapter there holds another tree and can still run. A comment in the config states this rule, and nothing checks it.
- **One set of tables.** The rows of both contracts go into the same tables, and each id starts with the chain ID. The id of a nullifier or a commitment also has the address of the contract that emitted it, because a nullifier is unique only in the contract that checks it.
- **No migration events.** anoma-envio does not index `CommitmentTreeMigrated`, `NullifierBatchMigrated` or `MigrationFinalized`. The copied commitments and nullifiers are in the index as the rows of the events that the legacy protocol adapter emitted before its stop. The upgrade at the end of the state migration is in the index through its `Upgraded` event.

anoma-envio builds no commitment tree. The galileo-indexer builds one tree per chain from the rows of both contracts. It reads the transactions of a chain in the order of their block number and log index, and the commitments of a transaction in the order of their actions and of their positions in an action. This order gives the tree of the protocol adapter, because:

- the index holds every commitment of the legacy protocol adapter, which it reads from the deployment block,
- `migrateCommitmentTree` continues the tree at the commitment count of the legacy protocol adapter, so the first new commitment of the protocol adapter follows the last commitment of the legacy protocol adapter,
- the protocol adapter lifts its pause only after the copy, and the copy runs only after the stop, so no transaction of the legacy protocol adapter follows a transaction of the protocol adapter.

Each commitment in an event is one leaf of the tree. A commitment that occurs in two events is two leaves, so an indexer that keeps one row for it computes a different tree from the second leaf on.

The first two `CommitmentTreeRootAdded` events of the protocol adapter carry the empty-tree root, from `initialize`, and the latest root of the legacy protocol adapter, from `migrateCommitmentTree`. The index also holds the older roots of the legacy protocol adapter, which the protocol adapter does not accept (see [The migrational implementation](#4-the-migrational-implementation)).

## 10. Trust assumptions

- **The operators keep the order.** `unpause` checks the copied state, but not the asset migration. Only the order of the steps keeps the pause in place until the tokens moved. A pause lifted too early lets legacy-forwarder resources unwrap tokens that back other resources.
- **One key holds each legacy protocol adapter from step 8 on.** The deployment wallet can then stop the legacy protocol adapter at any time. `Ownable` transfers in one step, so a wrong address would lose the stop, and nothing could get it back. The script therefore reads the address from `Parameters` and never from an argument.
- **One key owns a production proxy until the completion run.** The deployment wallet sends the calls of both runs, and the completion run transfers the proxy to the PA Safe as its last call.
- **A proxy that stops half-way stays paused.** `unpause` refuses an incomplete copy. The CREATE2 address of the proxy is taken, so the proxy cannot be replaced at that address, and the runs are repeated until they reach the end.
- **Older roots are lost.** Only the empty-tree root and the latest root of the legacy protocol adapter are historical roots after the copy.

## 11. Parameters

| Name | Value | Used for |
| --- | --- | --- |
| `Parameters.DEPLOYMENT_WALLET` | `0x61462bE56782568376f9cB069382EFa72764a407` | Owner of each proxy during its state migration. Stops each legacy protocol adapter. |
| `Parameters.PA_MULTISIG`, the PA Safe | `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`, 3 of 5 | Owner of each legacy protocol adapter until step 8, and of each production proxy after the completion run |
| The FWD Safe | `0xc703402252Ce1251aa07e0815D50060d27fdd6C4`, 3 of 5 | Emergency committee of every legacy ERC20 forwarder, `Parameters.FWD_MULTISIG` in the ERC20 forwarder repository |
| `IS_MIGRATIONAL` | `true` | Starts a proxy on the migrational implementation at its deployment |
| `Parameters.IMPLEMENTATION_SALT` | `"ProtocolAdapterImpl"` | CREATE2 salt of the migrational and the plain implementation |
| `NULLIFIERS_PER_BATCH` | `200` | Nullifiers per transaction in the migration run |

[`crates/bindings/deployments.json`](../crates/bindings/deployments.json) records, under `legacy`, the legacy protocol adapter of each chain.
