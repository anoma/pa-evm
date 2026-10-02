# Check coverage

Every check a transaction passes on its way to settlement — in-circuit (the
batch aggregation EVM circuit, the compliance circuit, the resource logic
contract) and out-of-circuit (the protocol adapter) — with the test that covers
it. Check descriptions use Simplified Technical English.

## How the checks are tested

- **Solidity** — `contracts/test/` runs the protocol adapter with the RISC Zero
  mock verifier; every out-of-circuit check is hit on-chain.
- **Rust, native** — `compliance_circuit.rs` and `trivial_logic.rs` call the
  exact `constrain()` functions the guests wrap; no executor is needed.
- **Rust, executor** — `aggregation_circuit.rs` runs the real
  `batch-aggregation-evm` guest ELF in the RISC Zero executor (no proving).
  Fabricated assumptions stand in for the compliance and logic receipts:
  `sys_verify` resolves them by claim digest, so a claim the guest derives from
  context that no assumption matches fails execution. The queue prover
  (`bento`) composes the same witness — these tests are the contract its
  construction must satisfy.
- **Rust, chain** — `integration.rs` settles proven transactions on a local
  anvil chain through the testkit. When e2e is enabled (after the backend and
  queue adapt to arm v2), the happy-path cases also run with real proving: the
  three guests execute for real and the real Groth16 verifier checks the seal
  on-chain. Negative cases stay local — the executor covers the same ELF.

Rust test files live in `crates/integration-test/tests/`.

## Compliance circuit (in-circuit)

The guest wraps `ComplianceWitness::constrain()`.

| ID | Check | What the check does | Covered by |
|---|---|---|---|
| C1 | rcv validity | The circuit reads the rcv value. The value must be 32 bytes and a valid scalar. The circuit rejects other values. | `compliance_circuit::constrain_errors_on_a_malformed_rcv` |
| C2 | Commitment recomputation | The circuit computes each commitment from all fields of the resource. The prover cannot supply a commitment. | Structural — pinned by `aggregation_circuit::aggregation_guest_commits_the_journal_the_local_prover_mirrors` and every settlement test |
| C3 | Nullifier key | The nullifier key must match the key commitment in the consumed resource. The circuit then computes the nullifier. | `compliance_circuit::constrain_errors_on_a_foreign_nullifier_key` |
| C4 | Commitment tree root | For a persistent resource, the circuit computes the tree root from the Merkle path. For an ephemeral resource, the circuit uses the witnessed ephemeral root. The protocol adapter checks the root against its history (P3). | `compliance_circuit::constrain_binds_the_commitment_tree_root_to_the_ephemerality` |
| C5 | Kind lookup | The circuit reads the kind point from the kind table entry. The point must decode to a curve point. Without an entry, the circuit computes the kind with hash-to-curve. | `compliance_circuit::constrain_errors_on_a_corrupt_kind_table_point` |
| C6 | Nonce derivation | The circuit derives each created nonce from its index and the hash of all consumed nullifiers, and rejects other nonces. At least one consumed resource is necessary. This makes commitments unique when nullifiers are unique. | `compliance_circuit::constrain_errors_on_a_tampered_created_nonce`, `compliance_circuit::constrain_errors_on_an_empty_consumed_list` |
| C7 | Delta accumulation | The circuit adds each quantity to the sum of its kind: plus for consumed, minus for created. It adds rcv times the generator and encodes the point. The identity point has no encoding and is rejected. | `compliance_circuit::constrain_errors_on_the_identity_delta`, `DeltaProof::testFuzz_verify_passes_if_balanced_kind_and_quantity_pairs_accumulate_to_a_zero_delta` |
| C8 | Kind table commitment | The circuit hashes the witnessed kind table with SHA-256 and puts the hash into the instance. The protocol adapter checks the hash (P8). | `compliance_circuit::constrain_commits_to_the_witnessed_kind_table` |

## Batch aggregation EVM circuit (in-circuit)

| ID | Check | What the check does | Covered by |
|---|---|---|---|
| A1 | Non-empty transaction | The transaction must contain at least one action. | `aggregation_circuit::aggregation_guest_rejects_an_empty_action_list` |
| A2 | Compliance proof | The circuit verifies one compliance proof for each action. The verifying key comes from the witness (see the non-checks below). | `aggregation_circuit::aggregation_guest_rejects_an_unproven_compliance_instance` |
| A3 | Shared kind table | All actions must commit to the same kind table. | `aggregation_circuit::aggregation_guest_rejects_a_kind_table_commitment_mismatch` |
| A4 | App data counts | The number of app data entries must equal the number of resources, on the consumed side and on the created side. | `aggregation_circuit::aggregation_guest_rejects_a_consumed_app_data_count_mismatch`, `…_a_created_app_data_count_mismatch` |
| A5 | Action tree root | The circuit computes each action tree root from the compliance tags: first the nullifiers, then the commitments. It does not accept a root from the prover. | `aggregation_circuit::aggregation_guest_rejects_a_logic_proof_over_a_foreign_root` |
| A6 | Journal encoding | The circuit ABI-encodes the instance and commits it as the journal. The protocol adapter rebuilds the same bytes from calldata. | `aggregation_circuit::aggregation_guest_commits_the_journal_the_local_prover_mirrors`, `aggregation_instance::simulate_execute_accepts_an_aggregation_seal_over_the_rust_encoded_instance`, `aggregation_proof::verify_verifies_the_aggregation_proof` |

## Resource logic contract (in-circuit, holds for every logic)

The circuit builds each logic instance itself and verifies the logic proof
against it. Thus a logic proof must attest to exactly this instance — for any
application logic, not only the trivial one.

| ID | Check | What the check does | Covered by |
|---|---|---|---|
| L1 | One proof per resource | Each resource in an action must have a logic proof. | `aggregation_circuit::aggregation_guest_rejects_a_missing_logic_proof`, `…_a_missing_created_logic_proof` |
| L2 | Tag binding | The instance tag is the nullifier of the consumed resource or the commitment of the created resource. The prover cannot choose the tag. | Structural — the claim in L1's tests contains the tag |
| L3 | Role binding | The `is_consumed` flag comes from the side the resource is on. | `aggregation_circuit::aggregation_guest_rejects_a_logic_proof_with_the_wrong_role` |
| L4 | Root binding | The instance root is the action tree root that the circuit computed (A5). Each logic sees its true action context. | `aggregation_circuit::aggregation_guest_rejects_a_logic_proof_over_a_foreign_root` |
| L5 | Verifying key binding | The circuit verifies each logic proof with the logic reference from the compliance instance. | `aggregation_circuit::aggregation_guest_rejects_a_logic_proof_under_a_foreign_verifying_key`, `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_tampered_logic_reference` |
| L6 | App data binding | The instance contains the app data from the witness, in tag order. The journal exposes the same app data. Thus the protocol adapter acts only on app data that a logic proved. | `aggregation_circuit::aggregation_guest_rejects_swapped_app_data`, `ProtocolAdapterMock.unit::test_execute_reverts_on_a_tampered_app_data_blob` |

## Trivial (padding) logic circuit (in-circuit)

The guest wraps `TrivialLogicWitness::constrain()` — the concrete logic the
trivial fixtures prove with.

| ID | Check | What the check does | Covered by |
|---|---|---|---|
| T1 | Tag computation | The circuit computes the tag from the resource. For a consumed resource, the nullifier key must match the key commitment. | `trivial_logic::constrain_errors_on_a_foreign_nullifier_key` |
| T2 | Padding shape | The resource must have zero quantity and must be ephemeral. | `trivial_logic::constrain_errors_on_a_non_padding_resource`, `integration::prove_actions_errors_on_nonzero_quantity`, `integration::prove_actions_errors_on_non_ephemeral_consumed` |

## Protocol adapter (out-of-circuit)

| ID | Check | What the check does | Covered by |
|---|---|---|---|
| P1 | Emergency stop | `execute` is blocked when the protocol adapter is paused or when its RISC Zero verifier is stopped. | `ProtocolAdapter.unit::test_execute_reverts_if_the_pa_has_been_stopped`, `ProtocolAdapterMock.unit::test_execute_reverts_on_vulnerable_risc_zero_verifier`, `ProtocolAdapter.upgrade::test_upgrade_allows_lifting_the_pause_set_by_an_emergency_stop` |
| P2 | Non-empty transaction | The contract rejects the empty transaction, so the two global proofs are always checked. | `ProtocolAdapter.unit::test_execute_reverts_on_the_empty_transaction`, `…::test_simulateExecute_reverts_on_the_empty_transaction`, `ProtocolAdapterMock.unit::test_execute_reverts_on_the_empty_transaction` |
| P3 | Historical root | Each consumed resource names a commitment tree root. The root must be in the set of historical roots. | `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_non_existing_root`, `integration::execute_reverts_on_a_non_historical_root` |
| P4 | Nullifier freshness | The contract adds each nullifier to the nullifier set. A nullifier that is already in the set reverts — within an action, across actions, and across transactions. | `ProtocolAdapterMock.unit::test_execute_reverts_on_pre_existing_nullifier`, `…_a_duplicate_nullifier_within_an_action`, `…_a_duplicate_nullifier_across_actions`, `integration::execute_tx_reverts_on_a_replayed_transaction`, `NullifierSet::test_addNullifier_reverts_on_duplicate` |
| P5 | Forwarder call output | The output of each forwarder call must equal the expected output committed in the call blob. | `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_unexpected_forwarder_call_output`; application calls in the forwarder repos |
| P6 | Delta on curve | Each action delta must be a point on the curve before it is added to the sum. | `ProtocolAdapterMock.unit::test_execute_reverts_on_actions_without_resources`, `DeltaProof::testFuzz_add_reverts_when_adding_a_non_curve_from_the_right` |
| P7 | Delta proof | The transaction ID is the Keccak-256 hash of the action tree roots. The delta proof must be a signature over this ID by the account of the summed delta. | `DeltaProof` (verify suite), `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_tampered_action_tree_root`, `…::testFuzz_execute_reverts_on_ubalanced_delta`, `integration::execute_tx_reverts_on_a_tampered_action_tree_root` |
| P8 | Journal injection | The contract rebuilds the journal from calldata. It inserts its own compliance key and its stored kind table commitment. A transaction proven over other values cannot match. | `KindTableCommitment::test_execute_reverts_for_transactions_proven_against_a_different_kind_table`, `…_accepts_transactions_proven_against_the_rotated_kind_table`, `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_tampered_commitment` |
| P9 | Verifier selector | The first four bytes of the proof must name the verifier the protocol adapter trusts. This check is never skipped. | `ProtocolAdapter.unit::test_execute_reverts_if_the_aggregation_proof_has_been_generated_with_another_unstopped_verifier`, `ProtocolAdapterMock.unit::test_simulateExecute_checks_the_selector_even_when_skipping_verification` |
| P10 | Seal verification | The RISC Zero router verifies the seal against the journal digest and the aggregation image ID. | `ProtocolAdapterMock.unit::test_execute_calls_the_risc_zero_verifier_router_exactly_once`, `…::test_simulateExecute_reverts_on_invalid_aggregation_proof_if_proof_verification_is_not_skipped`, `integration::execute_tx_reverts_on_invalid_seal`, `aggregation_proof::verify_verifies_the_aggregation_proof` |
| P11 | Root storage | After execution, the contract stores the new commitment tree root as a historical root. A transaction that creates no resources stores no root. | `ProtocolAdapterMock.unit::testFuzz_execute_updates_commitment_root_exactly_with_desired_commitments`, `…::test_execute_executes_consume_only_transactions_without_storing_a_root`, `CommitmentTree::test_addCommitmentTreeRoot_reverts_on_pre_existing_root`, `integration::execute_tx_settles_consume_only_transactions_without_a_root_change` |
| P12 | Admin gates | Only the owner can stop the protocol adapter, set the kind table commitment, and authorize upgrades. The zero commitment is rejected. | `KindTableCommitment` (suite), `ProtocolAdapter.unit::test_emergencyStop_reverts_if_the_caller_is_not_the_owner`, `ProtocolAdapter.upgrade::test_upgrade_reverts_if_the_caller_is_not_the_owner` |
| P13 | Reentrancy | A forwarder call cannot reenter `execute`. | `ProtocolAdapterMock.unit::test_execute_reverts_on_a_reentering_forwarder_call` |
| P14 | Simulation | `simulateExecute` always reverts with the gas measurement. It can skip the seal verification only (P9 and all state checks still apply). | `ProtocolAdapterMock.unit::test_simulateExecute_reverts_with_Simulated_if_proof_verification_is_not_skipped`, `…::test_simulateExecute_skips_the_verification_of_an_invalid_aggregation_proof`, `aggregation_instance::simulate_execute_accepts_an_aggregation_seal_over_the_rust_encoded_instance` |
| P15 | Blob emission | The contract emits app data blobs with the deletion criterion `Never` as events, keyed by the resource tag. It does not emit other blobs. Call blobs are executed in both cases. | `ProtocolAdapterMock.unit::test_execute_emits_the_persistent_app_data_blobs_only`, `…::test_execute_emits_the_persistent_external_call_blob`, `…::test_execute_executes_expiring_call_blobs_without_emitting_them` |
| P16 | Deployment guards | The constructor rejects a zero verifier router and a zero selector. `initialize` runs once, only behind a proxy, and rejects a stopped verifier. | `ProtocolAdapter.initialization` (suite), `DeployProtocolAdapter` (suite), storage-slot tests |

## Deliberate non-checks

| What is not checked | Why | Pinned by |
|---|---|---|
| Commitment duplication (on-chain) | The nonce derivation (C6) makes commitments unique when nullifiers are unique, and P4 makes nullifiers unique. | `CommitmentTree::test_addCommitment_allows_adding_the_same_commitment_multiple_times` |
| Action tree root vs. tags (on-chain) | The contract does not recompute the roots. The journal binds roots and tags together (A5), and the delta proof signs the roots (P7). | `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_tampered_action_tree_root`, `…_on_tampered_commitment` |
| Compliance key honesty (in-circuit) | The circuit verifies under the witnessed key and exposes it in the journal. The protocol adapter injects its own key (P8). | `aggregation_circuit::aggregation_guest_defers_the_compliance_key_check_to_the_journal` |
| Kind table honesty (in-circuit) | The circuit only checks that table points decode (C5). The stored commitment is the source of truth (P8, P12). | `compliance_circuit::constrain_errors_on_a_corrupt_kind_table_point`, `KindTableCommitment` (suite) |
| Forwarder call determinism | The contract cannot check determinism. A call whose output changes fails the output equality (P5). | `ProtocolAdapterMock.unit::testFuzz_execute_reverts_on_unexpected_forwarder_call_output` |
