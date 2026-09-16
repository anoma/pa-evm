// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {DeployProtocolAdapterImplementation} from "../../script/DeployProtocolAdapterImplementation.s.sol";
import {
    FinalizeProtocolAdapterStateMigration
} from "../../script/migration/FinalizeProtocolAdapterStateMigration.s.sol";
import {MigrateProtocolAdapterState} from "../../script/migration/MigrateProtocolAdapterState.s.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapterV1Mock} from "../mocks/ProtocolAdapterV1.m.sol";
import {RiscZeroRouterFixture} from "./RiscZeroRouterFixture.sol";

/// @notice A test fixture providing the starting point of a migration: a stopped stand-in for the v1 protocol adapter,
/// a paused migrational proxy bound to it, and the migration and finalization scripts. Forge's default sender owns the
/// stand-in and the proxy, because the scripts broadcast their transactions and a broadcast cannot be pranked.
abstract contract MigrationFixture is RiscZeroRouterFixture {
    uint256 internal constant _COMMITMENT_COUNT = 9;
    /// @dev More than one batch, so the nullifier loop of the migration run is exercised.
    uint256 internal constant _NULLIFIER_COUNT = 450;

    ProtocolAdapterV1Mock internal _v1;
    address internal _proxy;
    MigrateProtocolAdapterState internal _migrationScript;
    FinalizeProtocolAdapterStateMigration internal _finalizationScript;

    function setUp() public {
        _deployRiscZeroRouter();

        // The implementation the completion run upgrades to, at its deterministic address.
        new DeployProtocolAdapterImplementation().run();

        _v1 = new ProtocolAdapterV1Mock(DEFAULT_SENDER);
        for (uint256 i = 0; i < _COMMITMENT_COUNT; ++i) {
            _v1.addCommitment(keccak256(abi.encode("commitment", i)));
        }
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            _v1.addNullifier(keccak256(abi.encode("nullifier", i)));
        }
        vm.prank(DEFAULT_SENDER);
        _v1.emergencyStop();

        _proxy = _deployMigrationalProxy(address(_v1));

        _migrationScript = new MigrateProtocolAdapterState();
        _finalizationScript = new FinalizeProtocolAdapterStateMigration();
    }

    /// @notice Deploys a paused migrational proxy bound to the given v1 protocol adapter.
    /// @param protocolAdapterV1 The v1 protocol adapter the proxy copies its state from.
    /// @return proxy The proxy address.
    function _deployMigrationalProxy(address protocolAdapterV1) internal returns (address proxy) {
        // forge-lint: disable-next-line(unused-return)
        (, bytes memory implementationData) = new DeployProtocolAdapterImplementation().predict();
        (address router, bytes4 selector) = abi.decode(implementationData, (address, bytes4));

        Options memory opts;
        opts.constructorData = abi.encode(router, selector, protocolAdapterV1);
        proxy = Upgrades.deployUUPSProxy(
            "MigrationalProtocolAdapter.sol",
            abi.encodeCall(MigrationalProtocolAdapter.initialize, (DEFAULT_SENDER)),
            opts
        );
    }
}
