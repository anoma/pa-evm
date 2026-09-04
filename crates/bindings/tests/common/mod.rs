//! Shared helpers for the deployment tests: the records themselves, the deterministic addresses they must sit at,
//! and the version classification the promotion gates rely on.
#![allow(dead_code)]

use alloy::primitives::{Address, B256, Bytes, address};
use anoma_pa_evm_bindings::addresses::Environment;

pub const ENVIRONMENTS: [Environment; 2] = [Environment::Staging, Environment::Production];

/// The deterministic deployer every `CREATE2` deployment goes through, which `forge` predicts against.
pub const CREATE2_DEPLOYER: Address = address!("4e59b44847b379578588920cA78FbF26c0B4956C");

// The deploy scripts hold these as Solidity constants, which `forge bind` does not carry into the bindings, so
// they are restated here; a mismatch surfaces as a failing genesis check rather than a wrong deployment.
pub const PROXY_SALT_STAGING: &str = "ProtocolAdapterProxyStaging";
pub const PROXY_SALT_PRODUCTION: &str = "ProtocolAdapterProxyProduction";
pub const IMPLEMENTATION_SALT: &str = "ProtocolAdapterImpl";

/// A protocol adapter proxy recorded in `deployments.json`. The genesis fields pin the first deployment: they
/// determine the address together with the environment salt and cannot be recovered once the proxy is upgraded.
/// They stay strings so the checks can report an unparseable record instead of failing to load one.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawProxy {
    pub address: String,
    pub initial_implementation: String,
    pub initializer_data: String,
    pub creation_code: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawEntry {
    pub chain_id: u64,
    pub proxy: RawProxy,
}

#[derive(serde::Deserialize)]
struct RawDeployments {
    staging: Vec<RawEntry>,
    production: Vec<RawEntry>,
}

/// The entries recorded for the environment, read from the same file the crate embeds.
pub fn raw_entries(environment: Environment) -> Vec<RawEntry> {
    let deployments: RawDeployments = serde_json::from_str(include_str!("../../deployments.json"))
        .expect("deployments.json: invalid JSON");

    match environment {
        Environment::Staging => deployments.staging,
        Environment::Production => deployments.production,
    }
}

/// The proxy salt of the environment.
pub fn proxy_salt(environment: Environment) -> B256 {
    bytes32(match environment {
        Environment::Staging => PROXY_SALT_STAGING,
        Environment::Production => PROXY_SALT_PRODUCTION,
    })
}

/// A Solidity `bytes32` string literal: the ASCII bytes, right-padded with zeros.
pub fn bytes32(literal: &str) -> B256 {
    assert!(literal.len() <= 32, "{literal}: too long for a bytes32");

    let mut padded = [0u8; 32];
    padded[..literal.len()].copy_from_slice(literal.as_bytes());
    B256::from(padded)
}

/// The `<environment>, <chain ID>` prefix identifying a recorded deployment in assert messages.
pub fn context(environment: Environment, chain_id: u64) -> String {
    format!("{environment:?}, {chain_id}")
}

/// The flag arming the on-chain checks of the environment. Between a version bump on `next` and the environment's
/// upgrade the source and the deployments legitimately disagree, so they run on the promotion gate only.
pub fn verify_flag(environment: Environment) -> &'static str {
    match environment {
        Environment::Staging => "VERIFY_STAGING_DEPLOYMENTS",
        Environment::Production => "VERIFY_PRODUCTION_DEPLOYMENTS",
    }
}

/// Whether the environment's on-chain checks are armed, reporting the skip when they are not.
pub fn is_armed(environment: Environment) -> bool {
    let flag = verify_flag(environment);
    if std::env::var(flag).as_deref() == Ok("true") {
        return true;
    }

    eprintln!("skipped: {flag} is not set");
    false
}

/// Whether a version is a release, i.e. carries no prerelease suffix.
pub fn is_release(version: &str) -> bool {
    !version.contains('-')
}

/// Whether a version is a release candidate, i.e. carries an `-rc.<number>` prerelease suffix. Any other
/// prerelease (`-alpha.1`, `-rc`, `-rc.x`) is not one.
pub fn is_release_candidate(version: &str) -> bool {
    let Some((_, suffix)) = version.split_once('-') else {
        return false;
    };
    let Some(number) = suffix.strip_prefix("rc.") else {
        return false;
    };

    !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
}

/// Parses a hex field of a record, naming the field and the deployment when it does not parse.
pub fn parse_field<T: std::str::FromStr>(value: &str, field: &str, context: &str) -> T {
    value
        .parse()
        .unwrap_or_else(|_| panic!("{context}: invalid {field} '{value}'"))
}

/// The init code of a recorded proxy: its genesis creation code and constructor arguments.
pub fn proxy_init_code(proxy: &RawProxy, context: &str) -> Vec<u8> {
    use alloy::sol_types::SolConstructor;
    use anoma_pa_evm_bindings::generated::erc1967_proxy::ERC1967Proxy;

    let creation_code: Bytes = parse_field(&proxy.creation_code, "creationCode", context);
    let constructor_args = ERC1967Proxy::constructorCall {
        implementation: parse_field(
            &proxy.initial_implementation,
            "initialImplementation",
            context,
        ),
        _data: parse_field(&proxy.initializer_data, "initializerData", context),
    }
    .abi_encode();

    [creation_code.as_ref(), constructor_args.as_ref()].concat()
}
