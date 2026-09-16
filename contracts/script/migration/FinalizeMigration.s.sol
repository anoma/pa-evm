// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title FinalizeMigration
/// @author Anoma Foundation, 2026
/// @notice A script for the completion run of one chain's migration, once the ERC20 forwarder balances moved. It
/// unpauses the v2 proxy `MigrateProtocolAdapterState` copied the v1 state into, upgrades it from
/// `TransitionalProtocolAdapter` to the plain `ProtocolAdapter` implementation and, in production, transfers it to the
/// production proxy owner. The unpause compares the copied state against v1 and reverts on a difference, so no run
/// reaches the upgrade with the wrong state. Every step skips once it is done, so a run that stops early is repeated
/// until it reaches the end.
/// @dev The proxy owner sends every transaction, so this serves a chain whose owner is an account. A chain owned by a
/// Safe multisig needs the same calls proposed there instead.
/// @custom:security-contact security@anoma.foundation
contract FinalizeMigration is Script {
    /// @notice Thrown if the transitional proxy copies its state from another v1 protocol adapter than the given one.
    error ProtocolAdapterV1Mismatch(address expected, address actual);

    /// @notice Unpauses, upgrades to the plain implementation and, in production, transfers the proxy to the
    /// production proxy owner. The unpause is what checks the copied state against v1. Without `--broadcast` the run
    /// is simulated locally.
    /// @dev Run it with `--slow`, so that each transaction is confirmed before the next is sent. Without it every
    /// transaction goes out at once, and a transaction that reverts on chain does not stop the ones behind it. Every
    /// step skips once it is done, so a run that stops early can be repeated. Run `MigrateProtocolAdapterState.verify`
    /// afterwards to assert the result against the chain.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter the state was copied from.
    /// @param proxy The v2 protocol adapter proxy, running `TransitionalProtocolAdapter` or, after the upgrade,
    /// `ProtocolAdapter`.
    /// @param isProduction Whether the proxy belongs to the production environment, whose proxy owner receives it.
    function run(address protocolAdapterV1, address proxy, bool isProduction) public {
        DeployProtocolAdapterImplementation implementationDeployScript = new DeployProtocolAdapterImplementation();
        address implementation = _requireDeployedImplementation(implementationDeployScript);
        ProtocolAdapter protocolAdapter = ProtocolAdapter(proxy);

        if (protocolAdapter.getImplementation() != implementation) {
            _requireProtocolAdapterV1({protocolAdapterV1: protocolAdapterV1, proxy: proxy});

            if (protocolAdapter.paused()) {
                vm.broadcast();
                protocolAdapter.unpause();
            }

            // Read before the broadcast: `vm.broadcast` arms only the next call, and a view call would take it.
            bytes memory initializationData = implementationDeployScript.INITIALIZATION_DATA();
            vm.broadcast();
            protocolAdapter.upgradeToAndCall(implementation, initializationData);
        }

        if (isProduction) {
            address productionOwner = new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION();
            if (protocolAdapter.owner() != productionOwner) {
                vm.broadcast();
                protocolAdapter.transferOwnership(productionOwner);
            }
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

    /// @notice Reverts unless the proxy copies its state from the given v1 protocol adapter. Only the transitional
    /// implementation has the getter, so the call also rejects any other proxy.
    /// @param protocolAdapterV1 The v1 protocol adapter the caller names.
    /// @param proxy The v2 protocol adapter proxy.
    function _requireProtocolAdapterV1(address protocolAdapterV1, address proxy) internal view {
        address copiedFrom = TransitionalProtocolAdapter(proxy).getProtocolAdapterV1();
        require(
            copiedFrom == protocolAdapterV1,
            ProtocolAdapterV1Mismatch({expected: protocolAdapterV1, actual: copiedFrom})
        );
    }
}
