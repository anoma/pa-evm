// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.17.0/src/Script.sol";

import {DeploymentsJson} from "./libs/DeploymentsJson.sol";
import {RecordedDeployments} from "./libs/RecordedDeployments.sol";

/// @title RecordedDeploymentsScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts that read the protocol adapter proxy an environment records for the chain
/// `--rpc-url` names.
/// @custom:security-contact security@anoma.foundation
abstract contract RecordedDeploymentsScript is Script {
    /// @notice Thrown if the environment records no proxy for the chain.
    error DeploymentNotRecorded(string environment, uint256 chainId);

    /// @notice Returns the proxy the environment records for the chain `--rpc-url` names, and reverts unless it is
    /// recorded.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return proxy The recorded proxy.
    function _requireRecordedProxy(bool isProduction) internal view returns (address proxy) {
        proxy = _recordedProxy(isProduction);
        require(proxy != address(0), DeploymentNotRecorded(DeploymentsJson.environment(isProduction), block.chainid));
    }

    /// @notice Returns the proxy the environment records for the chain `--rpc-url` names.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return proxy The recorded proxy, or the zero address if the environment records none for the chain.
    function _recordedProxy(bool isProduction) internal view virtual returns (address proxy) {
        proxy = RecordedDeployments.protocolAdapterProxy({isProduction: isProduction, chainId: block.chainid});
    }
}
