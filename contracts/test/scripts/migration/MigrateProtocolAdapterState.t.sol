// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Vm} from "forge-std-1.16.2/src/Vm.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {MigrateProtocolAdapterState} from "../../../script/migration/MigrateProtocolAdapterState.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {TransitionalProtocolAdapter} from "../../../src/TransitionalProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {ProtocolAdapterV1Mock} from "../../mocks/ProtocolAdapterV1.m.sol";

/// @notice Checks the state migration end to end against a stand-in for the v1 protocol adapter. The proxy owner is
/// forge's default sender, because the script's transactions are broadcast and a broadcast cannot be pranked.
contract MigrateProtocolAdapterStateTest is RiscZeroRouterFixture {
    uint256 internal constant _COMMITMENT_COUNT = 9;
    /// @dev More than one batch, so the loop is exercised.
    uint256 internal constant _NULLIFIER_COUNT = 450;

    ProtocolAdapterV1Mock internal _v1;
    address internal _proxy;
    MigrateProtocolAdapterState internal _script;

    function setUp() public {
        _deployRiscZeroRouter();

        // The implementation the script upgrades to, at its deterministic address.
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

        _proxy = _deployTransitionalProxy(address(_v1));

        _script = new MigrateProtocolAdapterState();
    }

    function test_run_copies_the_state_and_leaves_the_proxy_on_the_plain_implementation() public {
        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        ProtocolAdapter pa = ProtocolAdapter(_proxy);
        assertEq(pa.latestCommitmentTreeRoot(), _v1.latestCommitmentTreeRoot(), "the root differs from v1");
        assertEq(pa.commitmentCount(), _COMMITMENT_COUNT, "the commitment count differs from v1");
        assertEq(pa.commitmentTreeDepth(), _v1.commitmentTreeDepth(), "the tree depth differs from v1");
        assertEq(pa.nullifierCount(), _NULLIFIER_COUNT, "the nullifier count differs from v1");
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            assertTrue(pa.isNullifierContained(_v1.nullifierAtIndex(i)), "a nullifier of v1 is missing");
        }

        assertFalse(pa.paused(), "the proxy should be unpaused");
        _script.verify({protocolAdapterV1: address(_v1), proxy: _proxy});
        (address implementation,) = new DeployProtocolAdapterImplementation().predict();
        assertEq(pa.getImplementation(), implementation, "the proxy should run the plain implementation");

        // The copy-in is gone with the implementation that carried it.
        vm.prank(DEFAULT_SENDER);
        vm.expectRevert();
        TransitionalProtocolAdapter(_proxy).seedNullifierSet(1);
    }

    function test_run_reverts_if_the_v1_protocol_adapter_is_still_running() public {
        ProtocolAdapterV1Mock running = new ProtocolAdapterV1Mock(DEFAULT_SENDER);
        running.addCommitment(keccak256("commitment"));
        address proxy = _deployTransitionalProxy(address(running));

        vm.expectRevert(
            abi.encodeWithSelector(TransitionalProtocolAdapter.ProtocolAdapterV1NotStopped.selector, address(running))
        );
        _script.run({protocolAdapterV1: address(running), proxy: proxy});
    }

    function test_run_sends_one_transaction_per_nullifier_batch() public {
        uint256 perBatch = _script.NULLIFIERS_PER_BATCH();
        uint256 expectedBatches = (_NULLIFIER_COUNT + perBatch - 1) / perBatch;
        assertGt(expectedBatches, 1, "the fixture should need more than one batch");

        vm.recordLogs();
        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        uint256 batches;
        bytes32 topic = keccak256("NullifierSetSeeded(uint256,uint256)");
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; ++i) {
            if (logs[i].emitter == _proxy && logs[i].topics[0] == topic) ++batches;
        }

        assertEq(batches, expectedBatches, "one seeding transaction per batch");
    }

    function test_run_reverts_if_the_proxy_is_not_paused() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.startPrank(DEFAULT_SENDER);
        TransitionalProtocolAdapter(_proxy).seedCommitmentTree(sides);
        TransitionalProtocolAdapter(_proxy).seedNullifierSet(_NULLIFIER_COUNT);
        TransitionalProtocolAdapter(_proxy).unpause();
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(MigrateProtocolAdapterState.ProxyNotPaused.selector, _proxy));
        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy});
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
