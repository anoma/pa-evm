# The aggregation journal is ABI-encoded, not risc0-serde-encoded

## Context

The protocol adapter rebuilds the aggregation journal from calldata (ADR-0001)
and hashes it to obtain the digest the verifier router checks. The journal
format is a contract between the arm-risc0 batch-aggregation guest and
Solidity: both sides must produce the same bytes from the same instance.

Two candidate formats:

- **risc0 serde** (guest-native): free in the guest, but Solidity must mirror
  arm's serialization by hand — little-endian `u32` words, length-prefixed
  vectors, raw digests — and track it in lockstep.
- **ABI encoding** (EVM-native): the guest pays cycles to encode, the chain
  reconstructs with one built-in `abi.encode` over calldata.

## Decision

The journal is the ABI **arguments** encoding of the `AggregationInstance`
fields: `abi.encode(complianceKey, kindTableCommitment, actions)` on-chain
(`Aggregation.toJournal`), equivalently `abi_encode_params` over
`(compliance_key, kind_table_commitment, actions)` in Rust — no outer tuple
offset.

The journal's only consumer that pays per byte of encoding logic is the EVM.
Off-chain the cost lands in the already-expensive proving step; on-chain the
reconstruction is a single native builtin that solc guarantees canonical.

## Consequences

- The missing outer offset is where the two sides most easily diverge: Rust must
  use `abi_encode_params`, not single-value `abi_encode`, which prepends a
  32-byte offset and differs only in the digest.
- arm-risc0 commits these bytes from a guest of its own behind the
  `abi_encoding` feature, carrying its own image id, which
  `VerifyingKeys._BATCH_AGGREGATION_EVM` pins. Encoding and key therefore rotate
  together, and a proof from the serde guest reverts with `VerificationFailed()`
  like any other digest mismatch — the adapter never decodes the journal, so
  there is no earlier point at which the format could be caught.
- `abi.encode` is location-agnostic, so the journal builds identically whether
  the actions come from memory or calldata.
- `aggregation_instance.rs` holds the two encodings to each other: a seal minted
  over the Rust-built instance must pass `simulateExecute`.
