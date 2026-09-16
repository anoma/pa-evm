// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Vm} from "forge-std-1.16.2/src/Vm.sol";

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {MigrateProtocolAdapterState} from "../../../script/migration/MigrateProtocolAdapterState.s.sol";
import {MigrationalProtocolAdapter} from "../../../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {MigrationFixture} from "../../fixtures/MigrationFixture.sol";
import {ProtocolAdapterV1Mock} from "../../mocks/ProtocolAdapterV1.m.sol";

/// @notice Checks the migration run end to end against a stand-in for the v1 protocol adapter, and the check of a
/// migrated proxy.
contract MigrateProtocolAdapterStateTest is MigrationFixture {
    function test_run_copies_the_state_and_leaves_the_proxy_paused() public {
        ProtocolAdapter pa = ProtocolAdapter(_proxy);
        bytes32 kindTableCommitment = pa.getKindTableCommitment();
        address migrationalImplementation = pa.getImplementation();

        _migrationScript.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        assertEq(pa.latestCommitmentTreeRoot(), _v1.latestCommitmentTreeRoot(), "the root differs from v1");
        assertEq(pa.commitmentCount(), _COMMITMENT_COUNT, "the commitment count differs from v1");
        assertEq(pa.commitmentTreeDepth(), _v1.commitmentTreeDepth(), "the tree depth differs from v1");
        assertEq(pa.nullifierCount(), _NULLIFIER_COUNT, "the nullifier count differs from v1");
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            assertTrue(pa.isNullifierContained(_v1.nullifierAtIndex(i)), "a nullifier of v1 is missing");
        }

        assertTrue(pa.paused(), "the proxy should stay paused");
        assertEq(
            pa.getImplementation(), migrationalImplementation, "the proxy should keep the migrational implementation"
        );
        assertEq(
            pa.getKindTableCommitment(), kindTableCommitment, "the run should not change the kind table commitment"
        );
        assertEq(pa.owner(), DEFAULT_SENDER, "the run should not change the owner");
    }

    function test_run_reverts_if_the_v1_protocol_adapter_is_still_running() public {
        ProtocolAdapterV1Mock running = new ProtocolAdapterV1Mock(DEFAULT_SENDER);
        running.addCommitment(keccak256("commitment"));
        address proxy = _deployMigrationalProxy(address(running));

        vm.expectRevert(
            abi.encodeWithSelector(MigrationalProtocolAdapter.ProtocolAdapterV1NotStopped.selector, address(running))
        );
        _migrationScript.run({protocolAdapterV1: address(running), proxy: proxy});
    }

    function test_run_reverts_for_a_proxy_that_copies_from_another_v1_protocol_adapter() public {
        address other = makeAddr("other v1 protocol adapter");

        vm.expectRevert(
            abi.encodeWithSelector(MigrateProtocolAdapterState.ProtocolAdapterV1Mismatch.selector, other, address(_v1))
        );
        _migrationScript.run({protocolAdapterV1: other, proxy: _proxy});
    }

    function test_run_sends_one_transaction_per_nullifier_batch() public {
        uint256 perBatch = _migrationScript.NULLIFIERS_PER_BATCH();
        uint256 expectedBatches = (_NULLIFIER_COUNT + perBatch - 1) / perBatch;
        assertGt(expectedBatches, 1, "the fixture should need more than one batch");

        vm.recordLogs();
        _migrationScript.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        uint256 batches;
        bytes32 topic = keccak256("NullifierBatchMigrated(uint256,uint256)");
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; ++i) {
            if (logs[i].emitter == _proxy && logs[i].topics[0] == topic) ++batches;
        }

        assertEq(batches, expectedBatches, "one seeding transaction per batch");
    }

    function test_run_resumes_a_run_that_stopped_after_the_commitment_tree() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();
        vm.prank(DEFAULT_SENDER);
        MigrationalProtocolAdapter(_proxy).migrateCommitmentTree(sides);

        _migrationScript.run({protocolAdapterV1: address(_v1), proxy: _proxy});
        _finalizationScript.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: false});

        _migrationScript.verify({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: false});
    }

    function test_verify_reverts_if_a_production_proxy_was_not_transferred() public {
        _migrationScript.run({protocolAdapterV1: address(_v1), proxy: _proxy});
        _finalizationScript.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: false});
        address productionOwner = new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION();

        vm.expectRevert(
            abi.encodeWithSelector(MigrateProtocolAdapterState.OwnerMismatch.selector, productionOwner, DEFAULT_SENDER)
        );
        _migrationScript.verify({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: true});
    }
}
