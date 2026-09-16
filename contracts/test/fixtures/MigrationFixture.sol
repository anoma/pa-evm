// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {DeployProtocolAdapterImplementation} from "../../script/DeployProtocolAdapterImplementation.s.sol";
import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";
import {ProtocolAdapterV1Mock} from "../mocks/ProtocolAdapterV1.m.sol";
import {RiscZeroRouterFixture} from "./RiscZeroRouterFixture.sol";

/// @notice A test fixture providing the starting point of a migration: a stopped stand-in for the v1 protocol
/// adapter, and a paused transitional proxy bound to it. Forge's default sender owns both, because the migration
/// scripts broadcast their transactions and a broadcast cannot be pranked.
abstract contract MigrationFixture is RiscZeroRouterFixture {
    uint256 internal constant _COMMITMENT_COUNT = 9;
    /// @dev More than one batch, so the nullifier loop of the migration run is exercised.
    uint256 internal constant _NULLIFIER_COUNT = 450;

    /// @notice Deploys a stand-in for the v1 protocol adapter holding `_COMMITMENT_COUNT` commitments and
    /// `_NULLIFIER_COUNT` nullifiers, and stops it.
    /// @return v1 The stopped v1 protocol adapter.
    function _deployStoppedProtocolAdapterV1() internal returns (ProtocolAdapterV1Mock v1) {
        v1 = new ProtocolAdapterV1Mock(DEFAULT_SENDER);
        for (uint256 i = 0; i < _COMMITMENT_COUNT; ++i) {
            v1.addCommitment(keccak256(abi.encode("commitment", i)));
        }
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            v1.addNullifier(keccak256(abi.encode("nullifier", i)));
        }
        vm.prank(DEFAULT_SENDER);
        v1.emergencyStop();
    }

    /// @notice Deploys a paused transitional proxy bound to the given v1 protocol adapter.
    /// @param protocolAdapterV1 The v1 protocol adapter the proxy copies its state from.
    /// @return proxy The proxy address.
    function _deployTransitionalProxy(address protocolAdapterV1) internal returns (address proxy) {
        // forge-lint: disable-next-line(unused-return)
        (, bytes memory implementationData) = new DeployProtocolAdapterImplementation().predict();
        (address router, bytes4 selector) = abi.decode(implementationData, (address, bytes4));

        Options memory opts;
        opts.constructorData = abi.encode(router, selector, protocolAdapterV1);
        proxy = Upgrades.deployUUPSProxy(
            "TransitionalProtocolAdapter.sol",
            abi.encodeCall(TransitionalProtocolAdapter.initialize, (DEFAULT_SENDER)),
            opts
        );
    }
}
