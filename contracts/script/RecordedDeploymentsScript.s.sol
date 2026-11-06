// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.17.0/src/Script.sol";

import {RecordedDeployments} from "../generated/RecordedDeployments.sol";
import {DeployProtocolAdapterImplementation} from "./DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "./DeployProtocolAdapterProxy.s.sol";

/// @title RecordedDeploymentsScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts that act on a recorded protocol adapter proxy. The records name the proxy and this
/// source predicts the implementation, so `--rpc-url` alone names the chain.
/// @custom:security-contact security@anoma.foundation
abstract contract RecordedDeploymentsScript is Script {
    /// @notice Thrown if the environment records no proxy for the chain.
    error DeploymentNotRecorded(string environment, uint256 chainId);

    /// @notice Returns the proxy the environment records for the chain `--rpc-url` names, and reverts unless it is
    /// recorded.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return proxy The recorded proxy.
    function _requireRecordedProxy(bool isProduction) internal returns (address proxy) {
        proxy = _recordedProxy(isProduction);
        if (proxy == address(0)) {
            // A `require` would create the script on every call: it evaluates the error arguments first.
            revert DeploymentNotRecorded(new DeployProtocolAdapterProxy().environmentName(isProduction), block.chainid);
        }
    }

    /// @notice Returns the plain implementation of this source version, and reverts unless it is deployed.
    /// @param implementationDeployScript The script that predicts the implementation's deterministic address.
    /// @return implementation The deployed plain implementation.
    function _requireDeployedImplementation(DeployProtocolAdapterImplementation implementationDeployScript)
        internal
        view
        returns (address implementation)
    {
        // forge-lint: disable-next-line(unused-return)
        (implementation,) = implementationDeployScript.predict();
        require(
            implementation.code.length != 0,
            DeployProtocolAdapterImplementation.ImplementationNotDeployed(implementation)
        );
    }

    /// @notice Returns the proxy the environment records for the chain `--rpc-url` names.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return proxy The recorded proxy, or the zero address if the environment records none for the chain.
    function _recordedProxy(bool isProduction) internal view virtual returns (address proxy) {
        proxy = RecordedDeployments.protocolAdapterProxy({isProduction: isProduction, chainId: block.chainid});
    }
}
