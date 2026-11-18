# Journal constants are injected, not passed in calldata

## Context

The protocol adapter verifies exactly one RISC Zero proof for each transaction, the aggregation proof. So it rebuilds the aggregation journal from calldata and hashes it to get the journal digest that the verifier router checks.

The journal is the `AggregationInstance` of arm-risc0:

```rust
pub struct AggregationInstance {
    pub compliance_vk: Digest,          // the verifying key of the compliance circuit
    pub kind_tree_root: Digest,         // the Merkle root of the kind tree
    pub actions: Vec<ActionAggregated>,
}
```

The aggregation guest verifies the compliance proof of each action against `compliance_vk`. This value is not data of one transaction: it is the same for all transactions.

Two ways to get such a value when the protocol adapter encodes the journal:

- **Fields.** Mirror the Rust structs 1:1: carry the value in the `Transaction` calldata and check it against the value of the contract, with a specific error, before the verifier call.
- **Injection.** Leave the value out of `Transaction` and let the protocol adapter write its own value into the encoding.

## Decision

**Injection.** The compliance and aggregation verifying keys are not part of the `Transaction` calldata. They are compile-time constants (`VerifyingKeys._COMPLIANCE`, `VerifyingKeys._BATCH_AGGREGATION_EVM`). They change only on a circuit vulnerability or a new protocol version, and both need a new implementation anyway.

So the protocol adapter accepts exactly one pair of circuit verifying keys. No check rejects a transaction proven with other keys: its journal digest does not match. The keys can be read from the verified source.

The kind tree root is not injected: the transaction carries it (ADR-0004).

## Consequences

- A proof made with other verifying keys reverts with `VerificationFailed()` from the RISC Zero router, not with a specific error.
- The Solidity structs do not mirror the Rust instances 1:1. `crates/bindings/src/conversion.rs` drops the compliance verifying key on the way to Solidity, and the protocol adapter adds it again when it encodes the journal.
- To change a circuit verifying key is an implementation upgrade.
- Each transaction saves the 32 bytes of the compliance verifying key.
