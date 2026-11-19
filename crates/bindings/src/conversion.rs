use alloy::primitives::{B256, Bytes, U256};
use anoma_rm_risc0::Digest;
use anoma_rm_risc0::aggregation_instance::{ActionAggregated, AggregationResourceInstance};
use anoma_rm_risc0::logic_instance::{AppData, ExpirableBlob};
use anoma_rm_risc0::proving_system::encode_seal;
use anoma_rm_risc0::transaction::{Delta as ArmDelta, Transaction};
use anoma_rm_risc0::utils::words_to_bytes;

use crate::generated::protocol_adapter::IProtocolAdapter;

impl From<ExpirableBlob> for IProtocolAdapter::ExpirableBlob {
    fn from(expirable_blob: ExpirableBlob) -> Self {
        Self {
            blob: words_to_bytes(&expirable_blob.blob).to_vec().into(),
            deletionCriterion: expirable_blob.deletion_criterion as u8,
        }
    }
}

impl From<AppData> for IProtocolAdapter::AppData {
    fn from(app_data: AppData) -> Self {
        Self {
            discoveryPayload: app_data
                .discovery_payload
                .into_iter()
                .map(IProtocolAdapter::ExpirableBlob::from)
                .collect(),
            resourcePayload: app_data
                .resource_payload
                .into_iter()
                .map(IProtocolAdapter::ExpirableBlob::from)
                .collect(),
            externalPayload: app_data
                .external_payload
                .into_iter()
                .map(IProtocolAdapter::ExpirableBlob::from)
                .collect(),
            applicationPayload: app_data
                .application_payload
                .into_iter()
                .map(IProtocolAdapter::ExpirableBlob::from)
                .collect(),
        }
    }
}

impl From<AggregationResourceInstance> for IProtocolAdapter::ResourceInstance {
    fn from(resource: AggregationResourceInstance) -> Self {
        let consumed = resource.is_consumed();
        let tag = B256::from_slice(resource.tag().as_bytes());
        let logic_ref = B256::from_slice(resource.logic_ref().as_bytes());
        let commitment_tree_root = B256::from_slice(
            resource
                .commitment_tree_root()
                .unwrap_or(Digest::ZERO)
                .as_bytes(),
        );
        let app_data = match resource {
            AggregationResourceInstance::Consumed { app_data, .. }
            | AggregationResourceInstance::Created { app_data, .. } => app_data,
        };
        Self {
            consumed,
            tag,
            logicRef: logic_ref,
            commitmentTreeRoot: commitment_tree_root,
            appData: app_data.into(),
        }
    }
}

impl From<ActionAggregated> for IProtocolAdapter::Action {
    fn from(action: ActionAggregated) -> Self {
        Self {
            unitDelta: IProtocolAdapter::Delta {
                x: U256::from_be_slice(words_to_bytes(&action.delta_x)),
                y: U256::from_be_slice(words_to_bytes(&action.delta_y)),
            },
            actionTreeRoot: B256::from_slice(action.action_tree_root.as_bytes()),
            resources: action
                .resource_instances
                .into_iter()
                .map(IProtocolAdapter::ResourceInstance::from)
                .collect(),
        }
    }
}

impl From<Transaction> for IProtocolAdapter::Transaction {
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
                .map(IProtocolAdapter::Action::from)
                .collect(),
            deltaProof: Bytes::from(delta_proof),
            aggregationProof: Bytes::from(encode_seal(&aggregation.proof).unwrap()),
            kindTreeRoot: B256::from_slice(aggregation.instance.kind_tree_root.as_bytes()),
        }
    }
}
