use alloy::primitives::Address;
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::{
    Environment, protocol_adapter_address, protocol_adapter_deployments_map,
};
use std::collections::HashSet;

const ENVIRONMENTS: [Environment; 2] = [Environment::Test, Environment::Prod];

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawEntry {
    chain_id: u64,
    proxy: String,
}

#[derive(serde::Deserialize)]
struct RawDeployments {
    test: Vec<RawEntry>,
    prod: Vec<RawEntry>,
}

fn raw_entries(environment: Environment) -> Vec<RawEntry> {
    let deployments: RawDeployments = serde_json::from_str(include_str!("../deployments.json"))
        .expect("deployments.json: invalid JSON");

    match environment {
        Environment::Test => deployments.test,
        Environment::Prod => deployments.prod,
    }
}

#[test]
fn all_entries_have_valid_chain_ids() {
    for environment in ENVIRONMENTS {
        for entry in raw_entries(environment) {
            NamedChain::try_from(entry.chain_id).unwrap_or_else(|_| {
                panic!(
                    "chain ID {} of environment {environment:?} does not map to a known NamedChain variant",
                    entry.chain_id
                )
            });
        }
    }
}

#[test]
fn all_entries_have_valid_addresses() {
    for environment in ENVIRONMENTS {
        for entry in raw_entries(environment) {
            entry.proxy.parse::<Address>().unwrap_or_else(|_| {
                panic!(
                    "invalid proxy address '{}' for chain ID '{}' of environment {environment:?}",
                    entry.proxy, entry.chain_id
                )
            });
        }
    }
}

#[test]
fn no_duplicate_chain_ids_within_an_environment() {
    for environment in ENVIRONMENTS {
        let mut seen = HashSet::new();
        for entry in raw_entries(environment) {
            assert!(
                seen.insert(entry.chain_id),
                "duplicate chain ID {} in environment {environment:?}",
                entry.chain_id
            );
        }
    }
}

#[test]
fn deployments_map_has_expected_count() {
    for environment in ENVIRONMENTS {
        let map = protocol_adapter_deployments_map(environment);
        let entries = raw_entries(environment);
        assert_eq!(
            map.len(),
            entries.len(),
            "deployments map size ({}) does not match JSON entries ({}) of environment {environment:?}",
            map.len(),
            entries.len()
        );
    }
}

#[test]
fn each_chain_is_individually_addressable() {
    for environment in ENVIRONMENTS {
        let map = protocol_adapter_deployments_map(environment);
        for chain in map.keys() {
            assert!(
                protocol_adapter_address(environment, chain).is_some(),
                "protocol_adapter_address returned None for chain '{chain}' of environment {environment:?}"
            );
        }
    }
}
