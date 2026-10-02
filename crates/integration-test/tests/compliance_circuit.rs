//! Chain-free negative tests of the compliance circuit's in-circuit checks.
//! `ComplianceWitness::constrain()` is the exact function the compliance guest
//! wraps, so calling it natively exercises the real circuit code; the local
//! prover runs the same function before minting a seal, so these witnesses can
//! never reach the chain.

use anoma_pa_testkit::fixtures::trivial;
use anoma_rm_risc0::compliance::{ComplianceWitness, INITIAL_ROOT, KindTableEntry};
use anoma_rm_risc0::error::ArmError;
use anoma_rm_risc0::nullifier_key::NullifierKey;
use anoma_rm_risc0::resource::Resource;
use risc0_zkvm::sha::{Impl, Sha256};

/// The compliance witness of a freshly-built trivial action.
fn witness(seed: u8) -> ComplianceWitness {
    *trivial::build(seed, trivial::Overrides::default())
        .expect("must build a trivial action")
        .witnesses
        .compliance_witness
}

#[test]
fn constrain_accepts_a_trivial_witness() {
    let instance = witness(1).constrain().expect("the baseline must constrain");

    assert_eq!(instance.consumed_publics.len(), 1);
    assert_eq!(instance.created_publics.len(), 1);
}

#[test]
fn constrain_errors_on_a_tampered_created_nonce() {
    let mut tampered = witness(2);
    tampered.constrain().expect("the baseline must constrain");

    for byte in tampered.created_resources[0].nonce.iter_mut() {
        *byte ^= 0xff;
    }

    assert_eq!(
        tampered.constrain().unwrap_err(),
        ArmError::InvalidResourceNonce
    );
}

#[test]
fn constrain_errors_on_a_foreign_nullifier_key() {
    let mut tampered = witness(3);
    tampered.constrain().expect("the baseline must constrain");

    tampered.consumed_data[0].nf_key = NullifierKey::from_bytes([0xee; 32]);

    assert_eq!(
        tampered.constrain().unwrap_err(),
        ArmError::InvalidNullifierKey
    );
}

#[test]
fn constrain_errors_on_a_malformed_rcv() {
    let mut tampered = witness(4);
    tampered.constrain().expect("the baseline must constrain");

    // Wrong length.
    tampered.rcv = vec![0x11; 31];
    assert_eq!(tampered.constrain().unwrap_err(), ArmError::InvalidRcv);

    // Right length, but not a canonical scalar.
    tampered.rcv = vec![0xff; 32];
    assert_eq!(tampered.constrain().unwrap_err(), ArmError::InvalidRcv);
}

#[test]
fn constrain_errors_on_a_corrupt_kind_table_point() {
    let mut tampered = witness(5);
    tampered.constrain().expect("the baseline must constrain");

    let resource = tampered.consumed_data[0].resource;
    tampered.kind_table.push(KindTableEntry {
        logic_ref: resource.logic_ref,
        label_ref: resource.label_ref,
        kind_point: vec![0x11; 65],
    });

    assert_eq!(
        tampered.constrain().unwrap_err(),
        ArmError::InvalidResourceKind
    );
}

#[test]
fn constrain_errors_on_an_empty_consumed_list() {
    // Created nonces derive from the consumed nullifiers, so a unit must
    // consume at least one resource.
    let created_only = ComplianceWitness::from_resources(&[], &[Resource::default()], Vec::new());

    assert_eq!(
        created_only.constrain().unwrap_err(),
        ArmError::EmptyNullifiers
    );
}

#[test]
fn constrain_errors_on_the_identity_delta() {
    // The trivial resources have zero quantity, so a zero rcv folds the delta
    // to the identity point, which has no affine encoding.
    let mut tampered = witness(6);
    tampered.constrain().expect("the baseline must constrain");

    tampered.rcv = vec![0x00; 32];

    assert_eq!(tampered.constrain().unwrap_err(), ArmError::InvalidDelta);
}

#[test]
fn constrain_binds_the_commitment_tree_root_to_the_ephemerality() {
    // An ephemeral consumed resource carries the witnessed ephemeral root.
    let ephemeral = witness(7);
    let instance = ephemeral
        .constrain()
        .expect("the ephemeral witness must constrain");
    assert_eq!(
        instance.consumed_publics[0].commitment_tree_root,
        *INITIAL_ROOT
    );

    // A persistent consumed resource gets its root recomputed from the Merkle
    // path — the protocol adapter checks that root against its history.
    let mut persistent = witness(8);
    persistent.consumed_data[0].resource.is_ephemeral = false;

    // The flip changes the nullifier, so re-derive the created nonce to keep
    // the nonce-derivation check satisfied.
    let nullifier = persistent.consumed_data[0]
        .resource
        .nullifier(&persistent.consumed_data[0].nf_key)
        .expect("the nullifier must derive");
    persistent.created_resources[0].nonce = Resource::derive_nonce_from_nullifiers(0, &[nullifier])
        .expect("the created nonce must derive");

    let commitment = persistent.consumed_data[0].resource.commitment();
    let expected = persistent.consumed_data[0].cm_merkle_path.root(&commitment);

    let instance = persistent
        .constrain()
        .expect("the persistent witness must constrain");
    assert_eq!(instance.consumed_publics[0].commitment_tree_root, expected);
    assert_ne!(
        instance.consumed_publics[0].commitment_tree_root,
        *INITIAL_ROOT
    );
}

#[test]
fn constrain_commits_to_the_witnessed_kind_table() {
    // The empty table hashes to the protocol adapter's initialization default.
    let empty = witness(9);
    let empty_commitment = empty
        .constrain()
        .expect("the empty-table witness must constrain")
        .kind_table_commitment;
    assert_eq!(empty_commitment, *Impl::hash_bytes(&[]));

    // A non-empty table commits to its concatenated entries.
    let mut tabled = witness(10);
    let resource = tabled.consumed_data[0].resource;
    let entry = KindTableEntry::new(
        resource.logic_ref,
        resource.label_ref,
        &resource.kind().expect("the kind must derive"),
    );
    tabled.kind_table.push(entry.clone());

    let instance = tabled
        .constrain()
        .expect("the tabled witness must constrain");

    let mut bytes = Vec::new();
    bytes.extend_from_slice(entry.logic_ref.as_bytes());
    bytes.extend_from_slice(entry.label_ref.as_bytes());
    bytes.extend_from_slice(&entry.kind_point);
    assert_eq!(instance.kind_table_commitment, *Impl::hash_bytes(&bytes));
    assert_ne!(instance.kind_table_commitment, empty_commitment);
}
