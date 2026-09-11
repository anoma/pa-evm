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
A set of actions plus a delta proof, submitted to the PA's `execute` function.

**Compliance / Logic / Delta**:
The three proof kinds the PA verifies — compliance for resource bookkeeping, logic
for each resource's application rules, delta for value balance. Aggregation folds
them into a single proof.

**Aggregation journal**:
The public statement the aggregation proof attests to — the compliance verifying
key, the kind table commitment, and the transaction's actions. The PA reconstructs
it from calldata, injecting the first two, and verifies the aggregation proof
against its digest.

**Unit Delta**:
The elliptic-curve point carrying the delta value of an action's compliance
unit — one per action, summed across the transaction and checked against the
delta proof.

**Commitment Tree / Nullifier Set**:
The PA's on-chain state — a Merkle tree of resource commitments and the set of
spent nullifiers.

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

**v1**:
The protocol adapter deployed before the v2 release: one immutable contract per chain, with no kind table. A chain that ran it moves its state into v2 through the transitional implementation.

**Transitional implementation**:
`TransitionalProtocolAdapter`, the implementation the proxy of a chain that ran v1 starts on. It is a protocol adapter that begins paused and accepts the v1 state.
_Avoid_: transition implementation, migration contract

**Plain implementation**:
`ProtocolAdapter`, the implementation every chain ends on. Its address is deterministic per chain.

**Copy-in**:
Writing the v1 state into the proxy: `seedCommitmentTree` once, then `seedNullifierSet` per batch. Allowed only while the proxy is paused, and removed by the upgrade to the plain implementation.
_Avoid_: seeding (the function names say it; the act has its own word), import

**Migration run**:
One execution of `MigrateProtocolAdapterState` for one chain: the copy-in, the unpause, which checks the result against v1, and the upgrade. A run that stops early has to be repeated until it reaches the upgrade.

**Sides**:
The stored left-sibling hashes of the v1 commitment tree, one per level. v1 exposes no getter for them, so the script reads them from v1's storage and the transitional implementation proves they reproduce v1's root.
