# The transaction carries the kind tree root

## Context

Before this decision, the protocol adapter injected its stored kind tree root into the aggregation journal, as it injects the verifying keys (ADR-0001). A proof made with another kind tree then failed with `VerificationFailed()`. This happened each time a prover and the protocol adapter used different kind trees. For example, after the owner stored a new root, every proof made with the previous kind tree failed, and a prover failed until it loaded the new kind tree.

The empty kind tree has no entries. With it, the compliance circuit computes every kind base by hash to curve, so no two kinds share a kind base. A transaction that balances with the empty kind tree therefore also balances with the stored kind tree. Only a conversion between aliases needs the stored kind tree, for example from resources of the legacy ERC20 forwarder to resources of the environment's ERC20 forwarder.

## Decision

The transaction carries the root of the kind tree that its aggregation proof is proven against, in `Transaction.kindTreeRoot`. The protocol adapter accepts two values: the stored root and the root of the empty kind tree, `EMPTY_KIND_TREE_ROOT` (`sha256("EMPTY")`). It checks the value before it processes the actions, so before any forwarder call and before the proof checks. On any other value, it reverts with `UnacceptedKindTreeRoot(kindTreeRoot)`. The aggregation journal embeds the carried root.

The compliance and aggregation verifying keys stay injected, as ADR-0001 decides.

## Consequences

- A proof made with the empty kind tree stays valid when the owner stores a new root. A prover that converts nothing can always use the empty kind tree.
- A proof made with a replaced kind tree reverts with `UnacceptedKindTreeRoot`, which names the root, instead of `VerificationFailed()`.
- A kind tree cannot stop a kind, because a transaction can use the empty kind tree instead. The logic ref denylist stops logic refs.
- The new field changes the ABI of `execute`. A deployed protocol adapter needs an implementation upgrade, and its clients need a release that sends the field, at the same time.
- Each transaction carries 32 more bytes of calldata.
- `crates/bindings/src/conversion.rs` copies the kind tree root from arm-risc0's `AggregationInstance`. It still drops the compliance key.
