use alloy::primitives::Address;
use alloy_chains::NamedChain;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

/// The deployment environment of a recorded protocol adapter proxy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Environment {
    /// The staging environment, owned by the deployment wallet and always running the source version.
    Staging,
    /// The production environment, owned by a Safe and possibly trailing the source version.
    Production,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeploymentEntry {
    chain_id: u64,
    proxy: String,
}

#[derive(Deserialize)]
struct Deployments {
    staging: Vec<DeploymentEntry>,
    production: Vec<DeploymentEntry>,
}

static DEPLOYMENTS: LazyLock<HashMap<Environment, HashMap<NamedChain, Address>>> =
    LazyLock::new(|| {
        let deployments: Deployments = serde_json::from_str(include_str!("../deployments.json"))
            .expect("deployments.json: invalid JSON");

        let to_map = |entries: Vec<DeploymentEntry>| {
            entries
                .into_iter()
                .filter_map(|e| {
                    let chain = NamedChain::try_from(e.chain_id).ok()?;
                    let proxy: Address = e.proxy.parse().ok()?;
                    Some((chain, proxy))
                })
                .collect()
        };

        HashMap::from([
            (Environment::Staging, to_map(deployments.staging)),
            (Environment::Production, to_map(deployments.production)),
        ])
    });

/// Returns a map of the protocol adapter proxies recorded for the environment.
pub fn protocol_adapter_deployments_map(environment: Environment) -> HashMap<NamedChain, Address> {
    DEPLOYMENTS[&environment].clone()
}

/// Returns the protocol adapter proxy recorded for the environment on the provided chain, if any.
pub fn protocol_adapter_address(environment: Environment, chain: &NamedChain) -> Option<Address> {
    DEPLOYMENTS[&environment].get(chain).cloned()
}
