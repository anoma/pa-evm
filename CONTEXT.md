# Anoma EVM Protocol Adapter

The protocol adapter contract and integration-test layer that settles **Anoma
Resource Machine** transactions on EVM-compatible chains.

## Language

**Protocol Adapter (PA)**:
The EVM contract that verifies and settles ARM transactions on-chain — checking
the compliance, logic, and delta proofs and updating the commitment tree and
nullifier set. Use "protocol adapter" (not "verifier" or "settler") for the
contract.

**Anoma Resource Machine (ARM)**:
The state model the PA settles: state is a set of immutable **resources** that
transactions consume and create. The off-chain ARM types live in `anoma-rm-risc0`.

**Resource**:
The unit of ARM state. Created and consumed by actions; committed to the
commitment tree and spent via a nullifier.

**Action**:
A bundle of consumed and created resources proven together — one compliance unit
plus the per-resource logic proofs.

**Transaction**:
A set of actions plus a delta proof, an aggregation proof and the commitment of the kind table that the aggregation proof is proven against, submitted to the PA's `execute` function.

**Compliance / Logic / Delta**:
The three proof kinds the PA verifies — compliance for resource bookkeeping, logic
for each resource's application rules, delta for value balance. Aggregation folds
them into a single proof.

**Aggregation journal**:
The public statement the aggregation proof attests to — the compliance verifying key, the kind table commitment, and the transaction's actions. The PA reconstructs it from calldata, injecting the compliance verifying key, and verifies the aggregation proof against its digest.

**Kind table commitment**:
The SHA-256 commitment of a kind table. The PA stores one, which its owner sets, and accepts a transaction proven against the stored kind table or against the empty kind table, whose commitment is `sha256("")`.

**Unit Delta**:
The elliptic-curve point carrying the delta value of an action's compliance
unit — one per action, summed across the transaction and checked against the
delta proof.

**Commitment Tree / Nullifier Set**:
The PA's on-chain state — a Merkle tree of resource commitments and the set of
spent nullifiers.

**Logic ref denylist**:
Two lists of logic refs, one for consumed and one for created resources: a transaction reverts if it consumes a resource whose logic ref is on the first list or creates a resource whose logic ref is on the second. The owner *deprecates* a logic ref once a newer circuit version replaces it: it goes on the list for created resources, and transactions still consume its resources. The owner *denies* a logic ref when its circuit turns out to be broken: it goes on both lists. No entry is ever removed. Say "denied" (not "refused" or "blocked").

**Forwarder**:
An application contract (e.g. the ERC20 or generic-call forwarder) that the PA
drives to enact EVM side effects on behalf of an action. Each lives in its own repo.

**Environment**:
One of the two protocol adapter deployments the repo maintains, each recorded per
chain in the deployment record and tracking a branch. Say "environment" (not
"network" or "deployment target") — a chain is where an environment lives, not
which one it is.

**Staging / Production**:
The two environments. Staging is owned by the deployment wallet and upgraded
directly; production is owned by a Safe multisig whose signers confirm and execute
upgrades. The branches tracking them keep their own names, `staging` and `main`.

**Promotion**:
Moving a commit unchanged from `next` to `staging`, or from `staging` to `main`.
The pull request opening one carries the gate proving the environment it targets
runs that commit's source. Changes only ever flow this way.

**Deployment record**:
`crates/bindings/deployments.json` — the proxy address of each environment on each
chain, plus the genesis fields pinning how that address was derived. Written once
per chain at its first deploy and never edited; what an environment currently runs
is read from the chain, not from here.

**Immutable protocol adapter**:
The protocol adapter of a chain before its protocol adapter proxy: one immutable contract per chain, with no kind table. It cannot be upgraded, so it is stopped, and the migrational implementation copies its state into the protocol adapter proxy.
_Avoid_: v1, PA v1, old adapter, legacy adapter

**Protocol adapter proxy**:
The upgradeable protocol adapter of an environment on a chain, at the address that the deployment record names. Each release is an implementation behind it.
_Avoid_: v2, PA v2, new adapter, current adapter

**Migrational implementation**:
`MigrationalProtocolAdapter`, the implementation that the protocol adapter proxy of a chain with an immutable protocol adapter starts on. It is a protocol adapter that begins paused and accepts the state of the immutable protocol adapter. Its proxy is owned by the deployment wallet in both environments; a production one moves to the Safe after the upgrade to the plain implementation.
_Avoid_: transition implementation, migration contract

**Plain implementation**:
`ProtocolAdapter`, the implementation every chain ends on. Its address is deterministic per chain.

**Copy-in**:
Writing the state of the immutable protocol adapter into the protocol adapter proxy: `migrateCommitmentTree` once, then `migrateNullifierSet` per batch. Allowed only while the proxy is paused, and removed by the upgrade to the plain implementation.
_Avoid_: seeding (the function names say it; the act has its own word), import

**Migration run**:
One execution of `MigrateProtocolAdapterState.run` for one chain: the copy-in. The proxy stays paused, so the ERC20 forwarder balances move before anyone can transact. Every step skips once it is done, so a run that stops early is repeated until it reaches the end.

**Completion run**:
One execution of `FinalizeProtocolAdapterStateMigration.run` for one chain, after the ERC20 forwarder balances moved: the unpause, which checks the copied state against the immutable protocol adapter, the upgrade and, in production, the transfer to the production proxy owner. Every step skips once it is done, so a run that stops early is repeated until it reaches the end.

**Sides**:
The stored left-sibling hashes of the commitment tree of the immutable protocol adapter, one per level. The immutable protocol adapter exposes no getter for them, so the script reads them from its storage, and the migrational implementation proves that they reproduce its root.
