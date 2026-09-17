// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";

import {RecordedDeployments} from "../../generated/RecordedDeployments.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title MigrationScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts that move one chain's state from the stopped v1 protocol adapter into the v2 proxy.
/// Both come from the recorded deployments, so `--rpc-url` alone names the chain.
/// @custom:security-contact security@anoma.foundation
abstract contract MigrationScript is Script {
    /// @notice Thrown if the chain records no v1 protocol adapter, i.e. it has no state to migrate.
    error ProtocolAdapterV1NotRecorded(uint256 chainId);

    /// @notice Thrown if the environment records no proxy for the chain, i.e. the state has no destination.
    error DeploymentNotRecorded(string environment, uint256 chainId);

    /// @notice Thrown if the migrational proxy copies its state from another v1 protocol adapter than the recorded one.
    error ProtocolAdapterV1Mismatch(address expected, address actual);

    /// @notice Returns the chain's recorded v1 protocol adapter and the environment's recorded proxy, and reverts
    /// unless both are recorded.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return protocolAdapterV1 The v1 protocol adapter, which holds the state.
    /// @return proxy The v2 protocol adapter proxy, which receives it.
    function _configuration(bool isProduction) internal returns (address protocolAdapterV1, address proxy) {
        (protocolAdapterV1, proxy) = _recordedDeployments(isProduction);
        require(protocolAdapterV1 != address(0), ProtocolAdapterV1NotRecorded(block.chainid));

        if (proxy == address(0)) {
            // A `require` would create the script on every call: it evaluates the error arguments first.
            revert DeploymentNotRecorded(new DeployProtocolAdapterProxy().environmentName(isProduction), block.chainid);
        }
    }

    /// @notice Returns the plain implementation the proxy ends on, and reverts unless it is deployed.
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

    /// @notice Returns what the records hold for the chain `--rpc-url` names.
    /// @dev Tests override it: the records are compiled into the script, so they cannot name a test deployment.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return protocolAdapterV1 The recorded v1 protocol adapter, or the zero address if the chain records none.
    /// @return proxy The recorded proxy, or the zero address if the environment records none for the chain.
    function _recordedDeployments(bool isProduction)
        internal
        view
        virtual
        returns (address protocolAdapterV1, address proxy)
    {
        protocolAdapterV1 = RecordedDeployments.protocolAdapterV1(block.chainid);
        proxy = RecordedDeployments.protocolAdapterProxy({isProduction: isProduction, chainId: block.chainid});
    }

    /// @notice Reverts unless the proxy copies its state from the recorded v1 protocol adapter. Only the migrational
    /// implementation has the getter, so the call also rejects any other proxy.
    /// @param protocolAdapterV1 The v1 protocol adapter the records name.
    /// @param proxy The v2 protocol adapter proxy.
    function _requireProtocolAdapterV1(address protocolAdapterV1, address proxy) internal view {
        address copiedFrom = MigrationalProtocolAdapter(proxy).getProtocolAdapterV1();
        require(
            copiedFrom == protocolAdapterV1,
            ProtocolAdapterV1Mismatch({expected: protocolAdapterV1, actual: copiedFrom})
        );
    }
}
