// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title Parameters
/// @author Anoma Foundation, 2026
/// @notice The deterministic deployment parameters — the CREATE2 salts and the environment proxy owners. They fix
/// where a deployment lands and who may upgrade it, so they are held once here and read by the deploy scripts, their
/// tests, and the bindings crate through `DeploymentParameters`.
/// @custom:security-contact security@anoma.foundation
library Parameters {
    /// @notice The CREATE2 salt for the staging environment proxy deployment.
    bytes32 internal constant PROXY_SALT_STAGING = "ProtocolAdapterProxyStaging";

    /// @notice The CREATE2 salt for the production environment proxy deployment.
    bytes32 internal constant PROXY_SALT_PRODUCTION = "ProtocolAdapterProxyProduction";

    /// @notice The CREATE2 salt for the implementation deployment, shared by the staging and production environments.
    bytes32 internal constant IMPLEMENTATION_SALT = "ProtocolAdapterImpl";

    /// @notice The staging environment proxy owner — the deployment wallet, upgrading instantly.
    address internal constant PROXY_OWNER_STAGING = 0x61462bE56782568376f9cB069382EFa72764a407;

    /// @notice The production environment proxy owner — the Safe multisig queueing upgrades.
    address internal constant PROXY_OWNER_PRODUCTION = 0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10;
}

/// @title DeploymentParameters
/// @author Anoma Foundation, 2026
/// @notice Exposes the deployment parameters through getters, so consumers outside Solidity — the bindings crate and
/// its tests — read the values this source holds instead of restating them.
/// @custom:security-contact security@anoma.foundation
contract DeploymentParameters {
    /// @notice Returns the CREATE2 salt for the staging environment proxy deployment.
    /// @return salt The staging proxy salt.
    function PROXY_SALT_STAGING() external pure returns (bytes32 salt) {
        salt = Parameters.PROXY_SALT_STAGING;
    }

    /// @notice Returns the CREATE2 salt for the production environment proxy deployment.
    /// @return salt The production proxy salt.
    function PROXY_SALT_PRODUCTION() external pure returns (bytes32 salt) {
        salt = Parameters.PROXY_SALT_PRODUCTION;
    }

    /// @notice Returns the CREATE2 salt for the implementation deployment.
    /// @return salt The implementation salt, shared by both environments.
    function IMPLEMENTATION_SALT() external pure returns (bytes32 salt) {
        salt = Parameters.IMPLEMENTATION_SALT;
    }

    /// @notice Returns the staging environment proxy owner.
    /// @return owner The deployment wallet upgrading the staging proxies.
    function PROXY_OWNER_STAGING() external pure returns (address owner) {
        owner = Parameters.PROXY_OWNER_STAGING;
    }

    /// @notice Returns the production environment proxy owner.
    /// @return owner The Safe multisig queueing production upgrades.
    function PROXY_OWNER_PRODUCTION() external pure returns (address owner) {
        owner = Parameters.PROXY_OWNER_PRODUCTION;
    }
}
