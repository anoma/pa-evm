use alloy::hex;
use alloy::primitives::{B256, U256};
use anoma_pa_evm_bindings::call_order::{call_order_delta_message, sign_call_order};
use anoma_pa_evm_bindings::generated::protocol_adapter::IProtocolAdapter;
use anoma_rm_risc0::delta_proof::DeltaWitness;

/// The expected message is `cast abi-encode "f(bytes32,uint256[])" "$(cast keccak 0x11..11)" "[1,0]"`.
#[test]
fn call_order_delta_message_matches_the_abi_encoding_of_the_transaction_id_and_the_order() {
    let action = IProtocolAdapter::Action {
        actionTreeRoot: B256::repeat_byte(0x11),
        ..Default::default()
    };

    let message = call_order_delta_message(&[action], &[U256::from(1), U256::from(0)]);

    assert_eq!(
        message,
        hex!(
            "b569321de72d0af89c2fb48a484de3fc9343f31600ae1f3e13d633cb48cbf816"
            "0000000000000000000000000000000000000000000000000000000000000040"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "0000000000000000000000000000000000000000000000000000000000000001"
            "0000000000000000000000000000000000000000000000000000000000000000"
        )
    );
}

/// The delta proof that `sign_call_order` makes for the order `[1, 0]` of one action with the root `0x11..11`, signed
/// with the delta witness 1. `DeltaProofTest.test_verify_accepts_a_call_order_that_the_bindings_sign` checks that
/// the protocol adapter accepts this proof.
#[test]
fn sign_call_order_signs_the_call_order_delta_message() {
    let mut transaction = IProtocolAdapter::Transaction {
        actions: vec![IProtocolAdapter::Action {
            actionTreeRoot: B256::repeat_byte(0x11),
            ..Default::default()
        }],
        ..Default::default()
    };
    let witness = DeltaWitness::from_bytes(&U256::from(1).to_be_bytes::<32>()).unwrap();

    sign_call_order(&mut transaction, &[U256::from(1), U256::from(0)], &witness).unwrap();

    assert_eq!(
        transaction.deltaProof[..],
        hex!(
            "f2dc14e6d7df234de39fe8bc1a31083a215fc7613ad768a87e9148fdb8d30b11"
            "0e792bcd2f6fa6135ae468fd1dc8a583ebcfde45c568e926e768cc91e571d728"
            "1b"
        )
    );
}
