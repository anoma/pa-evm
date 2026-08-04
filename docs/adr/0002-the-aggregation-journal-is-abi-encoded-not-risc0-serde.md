# The aggregation journal is ABI-encoded, not risc0-serde-encoded

## Context

The protocol adapter rebuilds the aggregation journal from calldata (ADR-0001)
and hashes it to obtain the digest the verifier router checks. Through
migration step 2 the journal was the risc0-serde encoding of arm's
`AggregationInstance` — the zkVM-native format `env::commit` produces:
little-endian `u32` words, length-prefixed vectors, raw digests. Mirroring
that on-chain required a hand-rolled field walk (`RiscZeroUtils`) with
quadratic `abi.encodePacked` accumulation, a byte-order helper imported from
risc0-ethereum, and a test-side `JournalEncoder` boundary library, because the
calldata-only encoders could not accept memory-built structs.

Two candidate formats:

- **risc0 serde** (guest-native): free in the guest, hand-maintained and
  error-prone on-chain — the Solidity encoder must track arm's serialization
  in lockstep.
- **ABI encoding** (EVM-native): the guest pays cycles to encode, the chain
  reconstructs with one built-in `abi.encode` over calldata.

## Decision

The journal is the ABI **arguments** encoding of the instance fields:
`abi.encode(complianceKey, kindTableCommitment, actions)` on-chain
(`Aggregation.toJournal`), equivalently `abi_encode_params` over
`(compliance_key, kind_table_commitment, actions)` in Rust — no outer tuple
offset. The proving side moves to match: the arm-risc0 batch-aggregation
guest commits these bytes (migration step 3, tracked in `TODO.md`).

The journal's only consumer that pays per byte of encoding logic is the EVM.
Off-chain the cost lands in the already-expensive proving step; on-chain the
reconstruction collapses from a foreign-format mirror to a single native
builtin that solc guarantees canonical.

## Consequences

- pa-evm adopted the encoding ahead of arm-risc0: until the guest ships it,
  `crates-test` stays red, and the `aggregation_instance.rs` pinning test is
  the canary that turns green when both sides agree.
- The arguments/no-outer-offset detail is the cross-language contract: Rust
  must use `abi_encode_params`, not single-value `abi_encode`, which prepends
  a 32-byte offset and diverges only in the digest.
- `abi.encode` is location-agnostic, retiring the `JournalEncoder`
  memory→calldata boundary and the `AppDataJournal` slot-distinguishability
  test — payload-slot separation is guaranteed by canonical ABI encoding.
