# Anoma EVM Protocol Adapter

The protocol adapter contract and integration-test layer that settles **Anoma Resource Machine** transactions on EVM-compatible chains.

## Language

**Protocol Adapter (PA)**:
The EVM contract that verifies and settles ARM transactions on-chain — checking the compliance, logic, and delta proofs and updating the commitment tree and nullifier set. Use "protocol adapter" (not "verifier" or "settler") for the contract. Without a qualifier, it is the upgradeable protocol adapter of an environment on a chain, behind its proxy; the legacy protocol adapter always carries its qualifier.
_Avoid_: v2, PA v2, new adapter, current adapter, protocol adapter proxy

**Anoma Resource Machine (ARM)**:
The state model the PA settles: state is a set of immutable **resources** that transactions consume and create. The off-chain ARM types live in `anoma-rm-risc0`.

**Resource**:
The unit of ARM state. Created and consumed by actions; committed to the commitment tree and spent via a nullifier.

**Action**:
A bundle of consumed and created resources proven together — one compliance unit plus the per-resource logic proofs.

**Transaction**:
A set of actions plus a delta proof, an aggregation proof and the root of the kind tree that the aggregation proof is proven against, submitted to the PA's `execute` function.

**Compliance / Logic / Delta**:
The three proof kinds the PA verifies — compliance for resource bookkeeping, logic for each resource's application rules, delta for value balance. Aggregation folds them into a single proof.

**Aggregation journal**:
The public statement the aggregation proof attests to — the compliance verifying key, the kind tree root, and the transaction's actions. The PA reconstructs it from calldata, injecting the compliance verifying key, and verifies the aggregation proof against its digest.

**Kind tree root**:
The root of the Merkle tree over a kind table. The PA stores one, which its owner sets, and accepts a transaction proven against the stored kind table or against the empty kind table, whose tree root is the padding leaf `sha256("EMPTY")`.

**Unit Delta**:
The elliptic-curve point carrying the delta value of an action's compliance unit — one per action, summed across the transaction and checked against the delta proof.

**Commitment Tree / Nullifier Set**:
The PA's on-chain state — a Merkle tree of resource commitments and the set of spent nullifiers.

**Logic ref denylist**:
Two lists of logic refs, one for consumed and one for created resources: a transaction reverts if it consumes a resource whose logic ref is on the first list or creates a resource whose logic ref is on the second. The owner *deprecates* a logic ref once a newer circuit version replaces it: it goes on the list for created resources, and transactions still consume its resources. The owner *denies* a logic ref when its circuit turns out to be broken: it goes on both lists. No entry is ever removed. Say "denied" (not "refused" or "blocked").

**Forwarder**:
An application contract (e.g. the ERC20 or generic-call forwarder) that the PA drives to enact EVM side effects on behalf of an action. Each lives in its own repo.

**Environment**:
One of the two protocol adapter deployments the repo maintains, each recorded per chain in the deployment record and tracking a branch. Say "environment" (not "network" or "deployment target") — a chain is where an environment lives, not which one it is.

**Staging / Production**:
The two environments. Staging is owned by the deployment wallet and upgraded directly; production is owned by a Safe multisig whose signers confirm and execute upgrades. The branches tracking them keep their own names, `staging` and `main`.

**Promotion**:
Moving a commit unchanged from `next` to `staging`, or from `staging` to `main`. The pull request opening one carries the gate proving the environment it targets runs that commit's source. Changes only ever flow this way.

**Deployment record**:
`crates/bindings/deployments.json` — the proxy address of each environment on each chain, plus the genesis fields pinning how that address was derived, and, under `legacy`, the legacy protocol adapter of each chain that ran one. Written once per chain at its first deploy and never edited, except that a redeployed release candidate replaces its record; what an environment currently runs is read from the chain, not from here. A replaced release-candidate proxy is short-lived and never becomes legacy.

**Legacy protocol adapter**:
The protocol adapter that a chain ran before its upgradeable protocol adapter: one immutable contract, with no kind table, on each chain that ran one. No new one is deployed. It is stopped, and the migrational implementation copies its state into the protocol adapter.
_Avoid_: v1, PA v1, old adapter, immutable protocol adapter

**Migrational implementation**:
`MigrationalProtocolAdapter`, the implementation that the proxy of a chain that ran a legacy protocol adapter starts on. It is a protocol adapter that begins paused and accepts the state of the legacy protocol adapter. Its proxy is owned by the deployment wallet in both environments; a production one moves to the Safe after the upgrade to the plain implementation.
_Avoid_: transition implementation, migration contract

**Plain implementation**:
`ProtocolAdapter`, the implementation every chain ends on. Its address is deterministic per chain.

**Copy-in**:
Writing the state of the legacy protocol adapter into the proxy: `migrateCommitmentTree` once, then `migrateNullifierSet` per batch. Allowed only while the proxy is paused, and removed by the upgrade to the plain implementation.
_Avoid_: seeding (the function names say it; the act has its own word), import

**Migration run**:
One execution of `MigrateProtocolAdapterState.run` for one chain: the copy-in. The proxy stays paused, so the ERC20 forwarder balances move before anyone can transact. Every step skips once it is done, so a run that stops early is repeated until it reaches the end.

**Completion run**:
One execution of `FinalizeProtocolAdapterStateMigration.run` for one chain, after the ERC20 forwarder balances moved: the unpause, which checks the copied state against the legacy protocol adapter, the upgrade and, in production, the transfer to the production proxy owner. Every step skips once it is done, so a run that stops early is repeated until it reaches the end.

**Left siblings**:
The stored roots of the last complete left subtree on each level of a commitment tree, of which only the levels where the leaf count has a one bit are meaningful. The legacy protocol adapter stores them as `_sides` and exposes no getter for them, so the script reads them from its storage, and the migrational implementation proves that they reproduce its root.

**State migration**:
The one-time copy of the commitment tree and the nullifier set of a chain's legacy protocol adapter into the protocol adapter, through the migrational implementation. It changes no resource. In the same window, the forwarder repository moves the tokens of the legacy ERC20 forwarder in the asset migration.
_Avoid_: migration (on its own)

**Conversion**:
A transaction that consumes a resource of one kind and creates a resource of another kind that the stored kind table assigns the same kind point. Only a conversion needs the stored kind table.

**Soft migration**:
The process in which resource owners convert their resources of a deprecated logic ref into resources of the active one. The protocol adapter still consumes resources of the deprecated logic ref, so their circuit must still be trusted.
_Avoid_: migration (on its own)

**Hard migration**:
The process in which resource owners create resources of a new logic ref from a proof that they hold unspent resources of a denied one. The protocol adapter consumes no resource of the denied logic ref, and the application records their nullifiers, so no proof of the denied circuit runs again. The ERC20 forwarder and its circuit call it emergency migration.
_Avoid_: migration (on its own), rescue
