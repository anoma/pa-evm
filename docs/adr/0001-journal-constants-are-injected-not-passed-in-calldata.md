# Journal constants are injected, not passed in calldata

## Context

Since migration step 1 the protocol adapter verifies exactly one RISC Zero
proof — the aggregation proof — which means it must rebuild the aggregation
journal from calldata and hash it to obtain the journal digest the verifier
router checks against.

The arm-risc0 v2 journal contains three values that are *not* per-transaction
data:

```rust
pub struct AggregationInstance {
    pub compliance_key: Digest,          // the compliance circuit VK
    pub kind_table_commitment: Digest,   // SHA-256 over the supported kinds
    pub actions: Vec<ActionAggregated>,
}
```

and the aggregation guest additionally verifies each action's compliance proof
against `compliance_key`, so `kind_table_commitment` also appears inside every
`ComplianceInstance`.

Two ways to obtain them when encoding the journal on chain:

- **Fields.** Mirror the Rust structs 1:1: carry both values in the
  `Transaction` calldata and `require`-check them against the contract's own
  values, with specific errors, before calling the verifier.
- **Injection.** Leave them out of `Transaction` entirely and have the protocol
  adapter write its own values into the encoding.

## Decision

**Injection.** Neither value is part of the `Transaction` calldata.

- `kindTableCommitment` is protocol adapter storage, rotated by the
  owner-protected `updateKindTable`. The commitment changes whenever a token is
  added to the set of supported resources, so it must be mutable without an
  implementation upgrade.
- The compliance and aggregation verifying keys stay compile-time constants
  (`Compliance._VERIFYING_KEY`, `Aggregation._VERIFYING_KEY`). They change only
  on a circuit vulnerability or a new protocol version, and both of those
  require a new implementation anyway.

The protocol adapter therefore accepts exactly one kind table — the current one
— and exactly one pair of circuit keys. A transaction proven against any other
is not rejected by a check; it is simply unencodable, and its journal digest
will not match.

Both values are exposed through public view getters so that clients can read
what the deployed adapter expects and compare it off chain.

## Consequences

- Wrong values surface as an opaque `VerificationFailed()` from the RISC Zero
  router rather than as a specific error. The view getters are the intended
  remedy: a client diffs its expected commitment against the deployed one
  before spending gas.
- The Solidity structs deliberately no longer mirror the Rust instances 1:1,
  which makes `crates/bindings/src/conversion.rs` asymmetric — it drops these
  fields on the way to Solidity, and the protocol adapter re-adds them when
  encoding the journal. Anyone comparing the two type sets will notice the gap;
  this ADR is the reason.
- Rotating the kind table is an owner transaction. Rotating a circuit verifying
  key is an implementation upgrade. This split is intentional and is the
  concrete instance of the wider immutability question tracked in `TODO.md`
  ("Revisit the immutability design").
- Every transaction saves the 64 bytes the two digests would have cost.
