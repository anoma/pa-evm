// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.17.0/src/Script.sol";

import {RecordedDeployments} from "../../generated/RecordedDeployments.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title MigrationScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts that move one chain's state from the stopped legacy immutable protocol adapter into
/// the proxy. Both come from the recorded deployments, so `--rpc-url` alone names the chain.
/// @custom:security-contact security@anoma.foundation
abstract contract MigrationScript is Script {
    /// @notice Thrown if the chain records no legacy immutable protocol adapter, i.e. it has no state to migrate.
    error ImmutableProtocolAdapterNotRecorded(uint256 chainId);

    /// @notice Thrown if the environment records no proxy for the chain, i.e. the state has no destination.
    error DeploymentNotRecorded(string environment, uint256 chainId);

    /// @notice Thrown if the migrational proxy copies its state from another legacy immutable protocol adapter than the
    /// recorded one.
    error ImmutableProtocolAdapterMismatch(address expected, address actual);

    /// @notice Returns the chain's recorded legacy immutable protocol adapter and the environment's recorded proxy, and
    /// reverts unless both are recorded.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return immutableProtocolAdapter The immutable protocol adapter, which holds the state.
    /// @return proxy The proxy of the protocol adapter, which receives it.
    function _configuration(bool isProduction) internal returns (address immutableProtocolAdapter, address proxy) {
        (immutableProtocolAdapter, proxy) = _recordedDeployments(isProduction);
        require(immutableProtocolAdapter != address(0), ImmutableProtocolAdapterNotRecorded(block.chainid));

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
    /// @return immutableProtocolAdapter The recorded legacy immutable protocol adapter, or the zero address if the
    /// chain records none.
    /// @return proxy The recorded proxy, or the zero address if the environment records none for the chain.
    function _recordedDeployments(bool isProduction)
        internal
        view
        virtual
        returns (address immutableProtocolAdapter, address proxy)
    {
        immutableProtocolAdapter = RecordedDeployments.immutableProtocolAdapter(block.chainid);
        proxy = RecordedDeployments.protocolAdapterProxy({isProduction: isProduction, chainId: block.chainid});
    }

    /// @notice Reverts unless the proxy copies its state from the recorded legacy immutable protocol adapter. Only the
    /// migrational implementation has the getter, so the call also rejects any other proxy.
    /// @param immutableProtocolAdapter The immutable protocol adapter the records name.
    /// @param proxy The proxy of the protocol adapter.
    function _requireImmutableProtocolAdapter(address immutableProtocolAdapter, address proxy) internal view {
        address copiedFrom = MigrationalProtocolAdapter(proxy).getImmutableProtocolAdapter();
        require(
            copiedFrom == immutableProtocolAdapter,
            ImmutableProtocolAdapterMismatch({expected: immutableProtocolAdapter, actual: copiedFrom})
        );
    }
}
