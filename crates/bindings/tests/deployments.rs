//! Checks of the deployment records themselves. They need no chain access and hold everywhere, so they run on
//! every push: a record that does not parse, or a proxy that does not sit at the address its genesis deployment
//! determines, is wrong regardless of what any chain currently runs.

mod common;

use alloy::primitives::{Address, keccak256};
use alloy_chains::NamedChain;
use anoma_pa_evm_bindings::addresses::{
    protocol_adapter_address, protocol_adapter_deployments_map,
};
use common::{
    CREATE2_DEPLOYER, ENVIRONMENTS, context, is_release, is_release_candidate, proxy_init_code,
    proxy_salt, raw_entries,
};
use std::collections::HashSet;

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
            entry.proxy.address.parse::<Address>().unwrap_or_else(|_| {
                panic!(
                    "invalid proxy address '{}' for chain ID '{}' of environment {environment:?}",
                    entry.proxy.address, entry.chain_id
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

/// Every recorded proxy sits at the address its genesis deployment determines under the environment salt — the
/// check that the first deployment of an environment used the right salt.
#[test]
fn recorded_deployments_use_the_environment_salt() {
    for environment in ENVIRONMENTS {
        let salt = proxy_salt(environment);

        for entry in raw_entries(environment) {
            let context = context(environment, entry.chain_id);
            let init_code = proxy_init_code(&entry.proxy, &context);
            let expected = CREATE2_DEPLOYER.create2(salt, keccak256(&init_code));

            assert_eq!(
                expected.to_string().to_lowercase(),
                entry.proxy.address.to_lowercase(),
                "{context}: recorded proxy address differs"
            );
        }
    }
}

#[test]
fn is_release_accepts_a_version_without_a_suffix() {
    assert!(is_release("2.0.0"));
}

#[test]
fn is_release_rejects_a_prerelease() {
    assert!(!is_release("2.0.0-rc.1"));
    assert!(!is_release("2.0.0-alpha.6"));
}

#[test]
fn is_release_candidate_accepts_a_numbered_candidate() {
    assert!(is_release_candidate("2.0.0-rc.0"));
    assert!(is_release_candidate("2.0.0-rc.12"));
}

#[test]
fn is_release_candidate_rejects_another_prerelease() {
    assert!(!is_release_candidate("2.0.0-alpha.6"));
    assert!(!is_release_candidate("2.0.0-beta.1"));
}

#[test]
fn is_release_candidate_rejects_a_candidate_without_a_number() {
    assert!(!is_release_candidate("2.0.0-rc"));
    assert!(!is_release_candidate("2.0.0-rc."));
    assert!(!is_release_candidate("2.0.0-rc.x"));
}

#[test]
fn is_release_candidate_rejects_a_version_without_a_suffix() {
    assert!(!is_release_candidate("2.0.0"));
}
