// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";
import {MigrationScript} from "./MigrationScript.s.sol";

/// @title FinalizeProtocolAdapterStateMigration
/// @author Anoma Foundation, 2026
/// @notice A script for the completion run of one chain's migration, once the ERC20 forwarder balances moved. It
/// unpauses the v2 proxy `MigrateProtocolAdapterState` copied the v1 state into, upgrades it from
/// `MigrationalProtocolAdapter` to the plain `ProtocolAdapter` implementation and, in production, transfers it to the
/// production proxy owner. The unpause compares the copied state against v1 and reverts on a difference, so no run
/// reaches the upgrade with the wrong state. Every step skips once it is done, so a run that stops early is repeated
/// until it reaches the end.
/// @dev The proxy owner sends every transaction, so this serves a chain whose owner is an account. A chain owned by a
/// Safe multisig needs the same calls proposed there instead.
/// @custom:security-contact security@anoma.foundation
contract FinalizeProtocolAdapterStateMigration is MigrationScript {
    /// @notice Unpauses, upgrades to the plain implementation and, in production, transfers the proxy to the
    /// production proxy owner. The unpause is what checks the copied state against v1. Without `--broadcast` the run
    /// is simulated locally.
    /// @dev Run it with `--slow`, so that each transaction is confirmed before the next is sent. Without it every
    /// transaction goes out at once, and a transaction that reverts on chain does not stop the ones behind it. Every
    /// step skips once it is done, so a run that stops early can be repeated. Run `MigrateProtocolAdapterState.verify`
    /// afterwards to assert the result against the chain.
    /// @param isProduction Whether to finalize the production or the staging proxy. The production proxy owner
    /// receives a production proxy.
    function run(bool isProduction) public {
        (address protocolAdapterV1, address proxy) = _configuration(isProduction);

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
}
