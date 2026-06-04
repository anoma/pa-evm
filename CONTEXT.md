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

**Commitment Tree / Nullifier Set**:
The PA's on-chain state — a Merkle tree of resource commitments and the set of
spent nullifiers.

**Forwarder**:
An application contract (e.g. the ERC20 or generic-call forwarder) that the PA
drives to enact EVM side effects on behalf of an action. Each lives in its own repo.
