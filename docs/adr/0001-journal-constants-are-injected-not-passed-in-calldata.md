# Journal constants are injected, not passed in calldata

## Context

The protocol adapter verifies exactly one RISC Zero proof per transaction — the
aggregation proof — so it must rebuild the aggregation journal from calldata and
hash it to obtain the journal digest the verifier router checks against.

The journal is arm-risc0's `AggregationInstance`, and two of its three values
are not per-transaction data:

```rust
pub struct AggregationInstance {
    pub compliance_key: Digest,          // the compliance circuit VK
    pub kind_table_commitment: Digest,   // SHA-256 over the supported kinds
    pub actions: Vec<ActionAggregated>,
}
```

The aggregation guest verifies each action's compliance proof against
`compliance_key`, and `kind_table_commitment` appears inside every
`ComplianceInstance` as well.

Two ways to obtain them when encoding the journal on chain:

- **Fields.** Mirror the Rust structs 1:1: carry both values in the
  `Transaction` calldata and `require`-check them against the contract's own
  values, with specific errors, before calling the verifier.
- **Injection.** Leave them out of `Transaction` entirely and have the protocol
  adapter write its own values into the encoding.

## Decision

**Injection.** Neither value is part of the `Transaction` calldata.

- `kindTableCommitment` is protocol adapter storage, rotated by the
  owner-protected `setKindTableCommitment`. The commitment changes whenever a
  token is added to the set of supported resources, so it must be mutable
  without an implementation upgrade.
- The compliance and aggregation verifying keys are compile-time constants
  (`VerifyingKeys._COMPLIANCE`, `VerifyingKeys._BATCH_AGGREGATION_EVM`). They
  change only on a circuit vulnerability or a new protocol version, and both of
  those require a new implementation anyway.

The protocol adapter therefore accepts exactly one kind table — the current one
— and exactly one pair of circuit keys. A transaction proven against any other
is not rejected by a check; it is simply unencodable, and its journal digest
will not match.

All three values are exposed through getters — `getKindTableCommitment`,
`getComplianceVerifyingKey` and `getBatchAggregationVerifyingKey` — so a client
can read what the deployed adapter expects and compare it off chain.

## Consequences

- Wrong values surface as an opaque `VerificationFailed()` from the RISC Zero
  router rather than as a specific error. Reading the getters back is the
  intended remedy: a client diffs its own three values against the deployed ones
  before spending gas.
- The Solidity structs deliberately do not mirror the Rust instances 1:1, which
  makes `crates/bindings/src/conversion.rs` asymmetric — it drops these fields
  on the way to Solidity, and the protocol adapter re-adds them when encoding
  the journal. Anyone comparing the two type sets will notice the gap; this ADR
  is the reason.
- Rotating the kind table is an owner transaction; rotating a circuit verifying
  key is an implementation upgrade.
- Every transaction saves the 64 bytes the two digests would have cost.
