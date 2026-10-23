use alloy::primitives::{Bytes, U256, keccak256};
use alloy::sol_types::SolValue;
use anoma_rm_risc0::delta_proof::{DeltaWitness, prove};
use anoma_rm_risc0::error::ArmError;

use crate::generated::protocol_adapter::IProtocolAdapter;

/// Returns the message that the delta proof signs for `executeWithCallOrder`: `abi.encode(transactionId, callOrder)`,
/// where `transactionId` is the Keccak-256 hash of the concatenated action tree roots. The delta proof hashes the
/// message with Keccak-256, as `DeltaProof.computeVerifyingKeyWithCallOrder` does.
pub fn call_order_delta_message(
    actions: &[IProtocolAdapter::Action],
    call_order: &[U256],
) -> Vec<u8> {
    let action_tree_roots: Vec<u8> = actions
        .iter()
        .flat_map(|action| action.actionTreeRoot.0)
        .collect();
    (keccak256(action_tree_roots), call_order.to_vec()).abi_encode_params()
}

/// Replaces the delta proof of the transaction with one that signs the call order, for `executeWithCallOrder`.
pub fn sign_call_order(
    transaction: &mut IProtocolAdapter::Transaction,
    call_order: &[U256],
    witness: &DeltaWitness,
) -> Result<(), ArmError> {
    let message = call_order_delta_message(&transaction.actions, call_order);
    transaction.deltaProof = Bytes::from(prove(&message, witness)?.to_bytes().to_vec());
    Ok(())
}
