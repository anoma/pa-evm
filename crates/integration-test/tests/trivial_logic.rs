//! Chain-free negative tests of the trivial (padding) logic circuit.
//! `TrivialLogicWitness::constrain()` is the exact function the trivial-logic
//! guest wraps — the concrete logic the trivial fixtures prove with. The
//! pipeline-level rejections live in `integration.rs`
//! (`prove_actions_errors_on_*`).

use anoma_rm_risc0::error::ArmError;
use anoma_rm_risc0::nullifier_key::NullifierKey;
use anoma_rm_risc0::resource_logic::{LogicCircuit, TrivialLogicWitness};

#[test]
fn constrain_accepts_a_default_padding_resource() {
    let consumed = TrivialLogicWitness {
        is_consumed: true,
        ..TrivialLogicWitness::default()
    };
    let instance = consumed
        .constrain()
        .expect("the consumed baseline must constrain");
    assert!(instance.is_consumed);

    let created = TrivialLogicWitness::default();
    let instance = created
        .constrain()
        .expect("the created baseline must constrain");
    assert!(!instance.is_consumed);
}

#[test]
fn constrain_errors_on_a_non_padding_resource() {
    let mut nonzero = TrivialLogicWitness::default();
    nonzero.resource.quantity = 1;
    assert_eq!(
        nonzero.constrain().unwrap_err(),
        ArmError::InvalidPaddingResource
    );

    let mut persistent = TrivialLogicWitness::default();
    persistent.resource.is_ephemeral = false;
    assert_eq!(
        persistent.constrain().unwrap_err(),
        ArmError::InvalidPaddingResource
    );
}

#[test]
fn constrain_errors_on_a_foreign_nullifier_key() {
    // The tag of a consumed resource is its nullifier, so the key must match
    // the resource's key commitment.
    let foreign_key = TrivialLogicWitness {
        is_consumed: true,
        nf_key: NullifierKey::from_bytes([0xee; 32]),
        ..TrivialLogicWitness::default()
    };

    assert_eq!(
        foreign_key.constrain().unwrap_err(),
        ArmError::InvalidNullifierKey
    );
}
