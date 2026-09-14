// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title Parameters
/// @author Anoma Foundation, 2026
/// @notice The deterministic deployment parameters — the CREATE2 salts and the environment proxy owners. They fix
/// where a deployment lands and who may upgrade it, so they are held once here and read by the deploy scripts, their
/// tests, and the bindings tests through `DeploymentParameters`.
/// @custom:security-contact security@anoma.foundation
library Parameters {
    /// @notice The CREATE2 salt for the staging environment proxy deployment.
    bytes32 internal constant PROXY_SALT_STAGING = "ProtocolAdapterProxyStaging";

    /// @notice The CREATE2 salt for the production environment proxy deployment.
    bytes32 internal constant PROXY_SALT_PRODUCTION = "ProtocolAdapterProxyProduction";

    /// @notice The CREATE2 salt for the implementation deployment, shared by the staging and production environments.
    bytes32 internal constant IMPLEMENTATION_SALT = "ProtocolAdapterImpl";

    /// @notice The deployment wallet, which owns the staging proxies and upgrades them instantly.
    address internal constant DEPLOYMENT_WALLET = 0x61462bE56782568376f9cB069382EFa72764a407;

    /// @notice The protocol adapter multisig, a Safe that owns the production proxies and v1 on every chain.
    address internal constant PA_MULTISIG = 0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10;
}
