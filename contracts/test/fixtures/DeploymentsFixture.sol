// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.2/src/Test.sol";

/// @notice A test fixture providing the protocol adapter deployments recorded per environment in `deployments.json` —
/// the single source of truth for the deterministic deployments. The recorded deployments are checked against the
/// chains and this source in the bindings crate, which owns the file; this fixture only serves the tests of the deploy
/// script that reads the same record.
abstract contract DeploymentsFixture is Test {
    /// @notice A protocol adapter proxy recorded in `deployments.json`.
    /// @dev The genesis fields pin the first deployment: the ERC-1967 proxy creation code and constructor arguments
    /// determine the address together with the environment salt, and none of them can be recovered from the chain
    /// once the proxy is upgraded.
    struct ProxyData {
        address addr;
        bytes creationCode;
        address initialImplementation;
        bytes initializerData;
    }

    /// @notice A protocol adapter deployment recorded in `deployments.json`.
    /// @dev Fields are ordered alphabetically by their JSON key so the struct decodes from `vm.parseJson`, which
    /// encodes object values in that order — the Solidity names themselves are irrelevant.
    struct Deployment {
        uint256 chainId;
        ProxyData proxy;
    }

    string internal constant _DEPLOYMENTS_PATH = "../crates/bindings/deployments.json";

    /// @notice Reads the deployments of an environment recorded in `deployments.json`.
    /// @param isProduction Whether to read the production or the staging environment.
    /// @return deployments The recorded deployments.
    function _recordedDeployments(bool isProduction) internal view returns (Deployment[] memory deployments) {
        string memory environment = string.concat(".", _environmentName(isProduction));

        deployments = abi.decode(vm.parseJson(vm.readFile(_DEPLOYMENTS_PATH), environment), (Deployment[]));
    }

    /// @notice Returns the name of an environment, which keys its deployments in `deployments.json`.
    /// @param isProduction Whether to name the production or the staging environment.
    /// @return name The environment name.
    function _environmentName(bool isProduction) internal pure returns (string memory name) {
        name = isProduction ? "production" : "staging";
    }
}
