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

**Forwarder call**:
A call from the PA to a forwarder that a resource carries in its external payload, with the output that the call must return and its ordering constraints. The PA executes the forwarder calls of a transaction after the nullifiers and commitments of all actions are added, after the proofs, and after the new commitment tree root is stored.

**Call-carrying resource**:
A resource with forwarder calls in its external payload.
_Avoid_: carrier

**Call list**:
The forwarder calls of a call-carrying resource, in the order of its external payload. The PA executes the calls of a call list together and in their order, and the call lists of an action together.

**Ordering constraint**:
An `afterTags` or `beforeTags` entry of a forwarder call. It names the tag of another call-carrying resource of the transaction, which is the nullifier of a consumed resource or the commitment of a created resource. The PA executes the calls of the named resource before (`afterTags`) or after (`beforeTags`) the calls of the resource that carries the constraint. Without ordering constraints, the PA executes the calls in the order of the actions, with the consumed resources of an action before its created resources.

**Call order**:
The order of the call lists that a transaction can carry in `Transaction.callOrder`, as the numbers of the call lists in the order of the transaction. The delta proof signs a non-empty call order. The PA checks it instead of computing an order: it must name each call list once, keep the actions in the order of the transaction, and meet every ordering constraint (ADR-0005).

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

**Sides**:
The stored left-sibling hashes of the commitment tree of the legacy protocol adapter, one per level. The legacy protocol adapter exposes no getter for them, so the script reads them from its storage, and the migrational implementation proves that they reproduce its root.
