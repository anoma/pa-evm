# The transaction carries the kind table commitment

## Context

ADR-0001 injects the stored kind table commitment into the aggregation journal. A proof made with another kind table then fails with `VerificationFailed()`. This happens each time a prover and the protocol adapter use different kind tables. For example, after the owner stores a new commitment, every proof made with the previous kind table fails, and a prover fails until it loads the new kind table.

The empty kind table has no entries. With it, the compliance circuit computes every kind point by hash to curve, so no two kinds share a kind point. A transaction that balances with the empty kind table therefore also balances with the stored kind table. Only a conversion between kinds that the stored kind table merges needs the stored kind table, for example from V1 forwarder resources to V2 resources.

## Decision

The transaction carries the commitment of the kind table that its aggregation proof is proven against, in `Transaction.kindTableCommitment`. The protocol adapter accepts two values: the stored commitment and the commitment of the empty kind table, `sha256("")`. It checks the value before it processes the actions, so before any forwarder call and before the proof checks. On any other value, it reverts with `UnacceptedKindTableCommitment(kindTableCommitment)`. The aggregation journal embeds the carried commitment.

The compliance and aggregation verifying keys stay injected, as ADR-0001 decides.

## Consequences

- A proof made with the empty kind table stays valid when the owner stores a new commitment. A prover that converts no merged kinds can always use the empty kind table.
- A proof made with a replaced kind table reverts with `UnacceptedKindTableCommitment`, which names the commitment, instead of `VerificationFailed()`.
- A kind table cannot stop a kind, because a transaction can use the empty kind table instead. The logic ref denylist stops logic refs.
- The new field changes the ABI of `execute` and `simulateExecute`. A deployed protocol adapter needs an implementation upgrade, and its clients need a release that sends the field, at the same time.
- Each transaction carries 32 more bytes of calldata.
- `crates/bindings/src/conversion.rs` copies the kind table commitment from arm-risc0's `AggregationInstance`. It still drops the compliance key.
