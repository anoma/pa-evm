use alloy::primitives::{B256, Bytes, U256};
use anoma_rm_risc0::aggregation_instance::{
    ActionAggregated, ConsumedResourceAggregated, CreatedResourceAggregated,
};
use anoma_rm_risc0::logic_instance::{AppData, ExpirableBlob};
use anoma_rm_risc0::proving_system::encode_seal;
use anoma_rm_risc0::transaction::{Delta as ArmDelta, Transaction};
use anoma_rm_risc0::utils::words_to_bytes;

use crate::generated::protocol_adapter::{Logic, Types};

impl From<ExpirableBlob> for Logic::ExpirableBlob {
    fn from(expirable_blob: ExpirableBlob) -> Self {
        Self {
            blob: words_to_bytes(&expirable_blob.blob).to_vec().into(),
            deletionCriterion: expirable_blob.deletion_criterion as u8,
        }
    }
}

impl From<AppData> for Logic::AppData {
    fn from(app_data: AppData) -> Self {
        Self {
            discoveryPayload: app_data
                .discovery_payload
                .into_iter()
                .map(Logic::ExpirableBlob::from)
                .collect(),
            resourcePayload: app_data
                .resource_payload
                .into_iter()
                .map(Logic::ExpirableBlob::from)
                .collect(),
            externalPayload: app_data
                .external_payload
                .into_iter()
                .map(Logic::ExpirableBlob::from)
                .collect(),
            applicationPayload: app_data
                .application_payload
                .into_iter()
                .map(Logic::ExpirableBlob::from)
                .collect(),
        }
    }
}

impl From<ConsumedResourceAggregated> for Types::Consumed {
    fn from(consumed: ConsumedResourceAggregated) -> Self {
        Self {
            nullifier: B256::from_slice(consumed.resource_nullifier.as_bytes()),
            logicRef: B256::from_slice(consumed.resource_logic_ref.as_bytes()),
            commitmentTreeRoot: B256::from_slice(consumed.commitment_tree_root.as_bytes()),
            appData: consumed.app_data.into(),
        }
    }
}

impl From<CreatedResourceAggregated> for Types::Created {
    fn from(created: CreatedResourceAggregated) -> Self {
        Self {
            commitment: B256::from_slice(created.resource_commitment.as_bytes()),
            logicRef: B256::from_slice(created.resource_logic_ref.as_bytes()),
            appData: created.app_data.into(),
        }
    }
}

impl From<ActionAggregated> for Types::Action {
    fn from(action: ActionAggregated) -> Self {
        Self {
            delta: Types::Delta {
                x: U256::from_be_slice(words_to_bytes(&action.delta_x)),
                y: U256::from_be_slice(words_to_bytes(&action.delta_y)),
            },
            actionTreeRoot: B256::from_slice(action.action_tree_root.as_bytes()),
            consumed: action
                .consumed_publics
                .into_iter()
                .map(Types::Consumed::from)
                .collect(),
            created: action
                .created_publics
                .into_iter()
                .map(Types::Created::from)
                .collect(),
        }
    }
}

impl From<Transaction> for Types::Transaction {
    fn from(tx: Transaction) -> Self {
        let delta_proof = match &tx.delta_proof {
            ArmDelta::Witness(_) => panic!("Unbalanced Transactions cannot be converted"),
            ArmDelta::Proof(proof) => proof.to_bytes().to_vec(),
        };

        let aggregation = tx
            .aggregation
            .expect("Transactions without an aggregation cannot be converted");

        Self {
            actions: aggregation
                .instance
                .actions
                .into_iter()
                .map(Types::Action::from)
                .collect(),
            deltaProof: Bytes::from(delta_proof),
            aggregationProof: Bytes::from(encode_seal(&aggregation.proof).unwrap()),
        }
    }
}
