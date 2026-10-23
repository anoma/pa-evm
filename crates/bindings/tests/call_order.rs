use alloy::hex;
use alloy::primitives::{B256, U256};
use anoma_pa_evm_bindings::call_order::call_order_delta_message;
use anoma_pa_evm_bindings::generated::protocol_adapter::IProtocolAdapter;

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
