//! Executes the real batch-aggregation EVM guest — the prebuilt ELF this repo
//! pins — in the RISC Zero executor to cover its in-circuit checks without
//! proving. Unresolved assumptions stand in for the compliance and logic
//! receipts: `sys_verify` resolves them by claim digest, so any claim the
//! guest derives from context (tag, role, recomputed action tree root,
//! declared verifying key, supplied app data) that no fabricated assumption
//! matches fails execution. The queue prover composes the same witness, so
//! these tests double as the contract its construction must satisfy.

use std::collections::HashMap;

use anoma_pa_testkit::environment::Prover;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::prover::LocalProver;
use anoma_pa_testkit::witness::ActionWitnesses;
use anoma_rm_risc0::action_tree::ActionTree;
use anoma_rm_risc0::aggregation_instance::{abi_decode_instance, abi_encode_instance};
use anoma_rm_risc0::aggregation_witness::{ActionWitness, AggregationWitness};
use anoma_rm_risc0::compliance::KindTableEntry;
use anoma_rm_risc0::constants::{BATCH_AGGREGATION_EVM_PK, COMPLIANCE_VK};
use anoma_rm_risc0::logic_instance::{AppData, ExpirableBlob, LogicInstance};
use anoma_rm_risc0::utils::words_to_bytes;
use anyhow::Context;
use risc0_zkvm::sha::Digestible;
use risc0_zkvm::{Assumption, Digest, ExecutorEnv, ReceiptClaim};

/// Fabricates the assumption `env::verify(image_id, instance)` resolves —
/// the claim a real receipt for this image and journal would carry.
fn assumption_over<T: serde::Serialize>(
    image_id: Digest,
    instance: &T,
) -> anyhow::Result<Assumption> {
    let words = risc0_zkvm::serde::to_vec(instance).context("failed to serialize the instance")?;

    Ok(Assumption {
        claim: ReceiptClaim::ok(image_id, words_to_bytes(&words).to_vec()).digest(),
        control_root: Digest::default(),
    })
}

/// Builds the guest witness for one action plus the assumptions the guest's
/// `env::verify` calls resolve, ordered `[compliance, consumed.., created..]`.
fn prepare_action(witnesses: &ActionWitnesses) -> anyhow::Result<(ActionWitness, Vec<Assumption>)> {
    let compliance_instance = witnesses
        .compliance_witness
        .constrain()
        .context("failed to constrain the compliance witness")?;

    let mut assumptions = vec![assumption_over(*COMPLIANCE_VK, &compliance_instance)?];

    let action_tree_root = ActionTree::new(compliance_instance.tags().collect())
        .root()
        .context("failed to compute the action tree root")?;

    let mut by_tag = HashMap::new();
    for logic in &witnesses.logic_witnesses {
        let instance = logic
            .constrain()
            .context("failed to constrain a logic witness")?;
        by_tag.insert(instance.tag, (logic.verifying_key(), instance.app_data));
    }

    let mut consumed_app_data = Vec::new();
    for consumed in &compliance_instance.consumed_publics {
        let (verifying_key, app_data) = by_tag
            .remove(&consumed.resource_nullifier)
            .context("missing the logic witness of a consumed resource")?;
        assumptions.push(assumption_over(
            verifying_key,
            &LogicInstance {
                tag: consumed.resource_nullifier,
                is_consumed: true,
                root: action_tree_root,
                app_data: app_data.clone(),
            },
        )?);
        consumed_app_data.push(app_data);
    }

    let mut created_app_data = Vec::new();
    for created in &compliance_instance.created_publics {
        let (verifying_key, app_data) = by_tag
            .remove(&created.resource_commitment)
            .context("missing the logic witness of a created resource")?;
        assumptions.push(assumption_over(
            verifying_key,
            &LogicInstance {
                tag: created.resource_commitment,
                is_consumed: false,
                root: action_tree_root,
                app_data: app_data.clone(),
            },
        )?);
        created_app_data.push(app_data);
    }

    Ok((
        ActionWitness {
            compliance_instance,
            consumed_app_data,
            created_app_data,
        },
        assumptions,
    ))
}

/// Builds the full guest witness and assumption set for the given actions.
fn prepare(actions: &[ActionWitnesses]) -> anyhow::Result<(AggregationWitness, Vec<Assumption>)> {
    let mut action_witnesses = Vec::with_capacity(actions.len());
    let mut assumptions = Vec::new();

    for action in actions {
        let (witness, action_assumptions) = prepare_action(action)?;
        action_witnesses.push(witness);
        assumptions.extend(action_assumptions);
    }

    Ok((
        AggregationWitness {
            compliance_key: *COMPLIANCE_VK,
            actions: action_witnesses,
        },
        assumptions,
    ))
}

/// Runs the batch-aggregation EVM guest in the executor, returning its journal.
fn execute_guest(
    witness: &AggregationWitness,
    assumptions: Vec<Assumption>,
) -> anyhow::Result<Vec<u8>> {
    let mut builder = ExecutorEnv::builder();
    builder
        .write(witness)
        .context("failed to write the aggregation witness")?;
    for assumption in assumptions {
        builder.add_assumption(assumption);
    }
    let env = builder
        .build()
        .context("failed to build the executor environment")?;

    let session = risc0_zkvm::default_executor()
        .execute(env, BATCH_AGGREGATION_EVM_PK)
        .context("guest execution failed")?;

    Ok(session.journal.bytes)
}

/// The executor error for an `env::verify` call no fabricated assumption
/// matches — the rejection surface of every claim-binding tamper. (The guest's
/// own panic messages only appear for its explicit asserts; a real prover
/// fails the same way at proving time.)
const UNRESOLVED_ASSUMPTION: &str = "no receipt found to resolve assumption";

/// Asserts that the guest rejects the witness with the given failure message.
fn assert_rejects(witness: &AggregationWitness, assumptions: Vec<Assumption>, needle: &str) {
    let error = execute_guest(witness, assumptions).expect_err("the guest must reject the witness");
    let rendered = format!("{error:?}");
    assert!(
        rendered.contains(needle),
        "expected {needle:?} in the guest failure, got: {rendered}"
    );
}

/// An app data value distinguishable per resource, for binding tests.
fn app_data_blob(byte: u32) -> AppData {
    AppData {
        resource_payload: vec![ExpirableBlob {
            blob: vec![byte],
            deletion_criterion: 0,
        }],
        ..AppData::default()
    }
}

#[tokio::test]
async fn aggregation_guest_commits_the_journal_the_local_prover_mirrors() -> anyhow::Result<()> {
    let actions = trivial::build_many(2, 1).context("failed to build trivial actions")?;
    let (witness, assumptions) = prepare(&actions)?;

    let journal = execute_guest(&witness, assumptions)?;

    let instance = LocalProver
        .prove(&actions)
        .await
        .context("the local prover must aggregate the same actions")?
        .into_arm()
        .aggregation
        .context("the local prover must produce an aggregation")?
        .instance;

    anyhow::ensure!(
        journal == abi_encode_instance(instance),
        "the guest journal must equal the local prover's ABI-encoded instance"
    );
    Ok(())
}

#[test]
fn aggregation_guest_rejects_an_empty_action_list() {
    let witness = AggregationWitness {
        compliance_key: *COMPLIANCE_VK,
        actions: Vec::new(),
    };

    assert_rejects(&witness, Vec::new(), "no actions provided");
}

#[test]
fn aggregation_guest_rejects_an_unproven_compliance_instance() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 11).context("failed to build a trivial action")?;
    let (witness, assumptions) = prepare(&actions)?;

    // Drop the compliance assumption, keeping the logic ones.
    let without_compliance = assumptions.into_iter().skip(1).collect();

    assert_rejects(&witness, without_compliance, UNRESOLVED_ASSUMPTION);
    Ok(())
}

/// The guest verifies compliance proofs under the witnessed key and exposes
/// that key in the journal — key honesty is deferred to the protocol adapter,
/// which reconstructs the journal with its own compliance key constant.
#[test]
fn aggregation_guest_defers_the_compliance_key_check_to_the_journal() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 21).context("failed to build a trivial action")?;
    let (mut witness, mut assumptions) = prepare(&actions)?;

    let foreign_key = Digest::from([0x77u8; 32]);
    witness.compliance_key = foreign_key;
    assumptions[0] = assumption_over(foreign_key, &witness.actions[0].compliance_instance)?;

    let journal = execute_guest(&witness, assumptions)?;

    let instance = abi_decode_instance(&journal).context("the journal must decode")?;
    anyhow::ensure!(
        instance.compliance_key == foreign_key,
        "the journal must expose the witnessed compliance key"
    );
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_kind_table_commitment_mismatch() -> anyhow::Result<()> {
    let mut actions = trivial::build_many(2, 31).context("failed to build trivial actions")?;

    // Give the second action an honestly-committed, non-empty kind table, so
    // both compliance instances are individually valid but disagree.
    let resource = actions[1].compliance_witness.consumed_data[0].resource;
    actions[1]
        .compliance_witness
        .kind_table
        .push(KindTableEntry::new(
            resource.logic_ref,
            resource.label_ref,
            &resource.kind().context("the kind must derive")?,
        ));

    let (witness, assumptions) = prepare(&actions)?;

    assert_rejects(
        &witness,
        assumptions,
        "kind_table_commitment mismatch across actions",
    );
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_consumed_app_data_count_mismatch() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 41).context("failed to build a trivial action")?;
    let (mut witness, assumptions) = prepare(&actions)?;

    witness.actions[0]
        .consumed_app_data
        .push(AppData::default());

    assert_rejects(&witness, assumptions, "consumed app_data count mismatch");
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_created_app_data_count_mismatch() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 42).context("failed to build a trivial action")?;
    let (mut witness, assumptions) = prepare(&actions)?;

    witness.actions[0].created_app_data.push(AppData::default());

    assert_rejects(&witness, assumptions, "created app_data count mismatch");
    Ok(())
}

/// The queue prover must supply app data in the canonical tag order — the
/// logic claims commit to each (tag, app data) pair, so swapping the app data
/// of two resources fails their logic verification.
#[test]
fn aggregation_guest_rejects_swapped_app_data() -> anyhow::Result<()> {
    let action = trivial::build(
        51,
        trivial::Overrides {
            consumed_count: Some(2),
            created_count: Some(1),
            ..trivial::Overrides::default()
        },
    )
    .context("failed to build a trivial action")?
    .witnesses;

    let compliance_instance = action
        .compliance_witness
        .constrain()
        .context("failed to constrain the compliance witness")?;
    let root = ActionTree::new(compliance_instance.tags().collect())
        .root()
        .context("failed to compute the action tree root")?;

    // Fabricate distinct app data per consumed resource, with matching claims.
    let (first, second) = (app_data_blob(0xa1), app_data_blob(0xb2));
    let mut assumptions = vec![assumption_over(*COMPLIANCE_VK, &compliance_instance)?];
    for (consumed, app_data) in compliance_instance
        .consumed_publics
        .iter()
        .zip([&first, &second])
    {
        assumptions.push(assumption_over(
            consumed.resource_logic_ref,
            &LogicInstance {
                tag: consumed.resource_nullifier,
                is_consumed: true,
                root,
                app_data: app_data.clone(),
            },
        )?);
    }
    let created = &compliance_instance.created_publics[0];
    assumptions.push(assumption_over(
        created.resource_logic_ref,
        &LogicInstance {
            tag: created.resource_commitment,
            is_consumed: false,
            root,
            app_data: AppData::default(),
        },
    )?);

    let mut witness = AggregationWitness {
        compliance_key: *COMPLIANCE_VK,
        actions: vec![ActionWitness {
            compliance_instance,
            consumed_app_data: vec![first, second],
            created_app_data: vec![AppData::default()],
        }],
    };

    execute_guest(&witness, assumptions.clone())
        .context("the correctly-ordered app data must be accepted")?;

    witness.actions[0].consumed_app_data.swap(0, 1);

    assert_rejects(&witness, assumptions, UNRESOLVED_ASSUMPTION);
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_missing_logic_proof() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 61).context("failed to build a trivial action")?;
    let (witness, mut assumptions) = prepare(&actions)?;

    // Drop the consumed resource's logic assumption.
    assumptions.remove(1);

    assert_rejects(&witness, assumptions, UNRESOLVED_ASSUMPTION);
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_missing_created_logic_proof() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 62).context("failed to build a trivial action")?;
    let (witness, mut assumptions) = prepare(&actions)?;

    // Drop the created resource's logic assumption (last in the ordering).
    assumptions.pop();

    assert_rejects(&witness, assumptions, UNRESOLVED_ASSUMPTION);
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_logic_proof_over_a_foreign_root() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 63).context("failed to build a trivial action")?;
    let (witness, mut assumptions) = prepare(&actions)?;

    // Re-fabricate the consumed logic claim over a root the guest does not
    // recompute — the guest constructs the instance with its own root.
    let consumed = &witness.actions[0].compliance_instance.consumed_publics[0];
    assumptions[1] = assumption_over(
        consumed.resource_logic_ref,
        &LogicInstance {
            tag: consumed.resource_nullifier,
            is_consumed: true,
            root: Digest::from([0x99u8; 32]),
            app_data: AppData::default(),
        },
    )?;

    assert_rejects(&witness, assumptions, UNRESOLVED_ASSUMPTION);
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_logic_proof_with_the_wrong_role() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 64).context("failed to build a trivial action")?;
    let (witness, mut assumptions) = prepare(&actions)?;

    let instance = &witness.actions[0].compliance_instance;
    let root = ActionTree::new(instance.tags().collect())
        .root()
        .context("failed to compute the action tree root")?;

    // The claim is correct except for the role flag.
    let consumed = &instance.consumed_publics[0];
    assumptions[1] = assumption_over(
        consumed.resource_logic_ref,
        &LogicInstance {
            tag: consumed.resource_nullifier,
            is_consumed: false,
            root,
            app_data: AppData::default(),
        },
    )?;

    assert_rejects(&witness, assumptions, UNRESOLVED_ASSUMPTION);
    Ok(())
}

#[test]
fn aggregation_guest_rejects_a_logic_proof_under_a_foreign_verifying_key() -> anyhow::Result<()> {
    let actions = trivial::build_many(1, 65).context("failed to build a trivial action")?;
    let (witness, mut assumptions) = prepare(&actions)?;

    let instance = &witness.actions[0].compliance_instance;
    let root = ActionTree::new(instance.tags().collect())
        .root()
        .context("failed to compute the action tree root")?;

    // The claim covers the correct instance but under a different image ID
    // than the compliance-declared `resource_logic_ref`.
    let consumed = &instance.consumed_publics[0];
    assumptions[1] = assumption_over(
        *COMPLIANCE_VK,
        &LogicInstance {
            tag: consumed.resource_nullifier,
            is_consumed: true,
            root,
            app_data: AppData::default(),
        },
    )?;

    assert_rejects(&witness, assumptions, UNRESOLVED_ASSUMPTION);
    Ok(())
}
