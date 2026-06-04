//! Chain-free test that a locally-proven ARM transaction converts into a
//! protocol-adapter transaction. (The trivial-action / local-prover smoke tests
//! themselves live in the testkit; this asserts the EVM conversion seam, which
//! needs the bindings.)

use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter as PaContract;
use anoma_pa_testkit::environment::Prover;
use anoma_pa_testkit::fixtures::trivial;
use anoma_pa_testkit::prover::LocalProver;
use anoma_rm_risc0::transaction::Transaction as ArmTxn;

#[tokio::test]
async fn proven_transaction_into_pa_transaction_yields_actions_and_delta_proof() {
    let actions = trivial::build_many(2, 1).expect("must build trivial action witnesses");
    let tx = LocalProver.prove(&actions).await.unwrap();
    let arm_tx: ArmTxn = tx.into_arm();

    let pa_tx: PaContract::Transaction = arm_tx.into();
    assert!(!pa_tx.actions.is_empty());
    assert!(!pa_tx.deltaProof.is_empty());
}
