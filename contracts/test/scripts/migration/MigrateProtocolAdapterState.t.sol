// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Vm} from "forge-std-1.17.0/src/Vm.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {MigrateProtocolAdapterState} from "../../../script/migration/MigrateProtocolAdapterState.s.sol";
import {MigrationScript} from "../../../script/migration/MigrationScript.s.sol";
import {ICommitmentTree} from "../../../src/interfaces/ICommitmentTree.sol";
import {IProtocolAdapter} from "../../../src/interfaces/IProtocolAdapter.sol";
import {SHA256} from "../../../src/libs/SHA256.sol";
import {MigrationalProtocolAdapter} from "../../../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {MigrationFixture} from "../../fixtures/MigrationFixture.sol";
import {TxGen} from "../../libs/TxGen.sol";
import {LegacyProtocolAdapterMock} from "../../mocks/LegacyProtocolAdapter.m.sol";
import {MigrateProtocolAdapterStateMock} from "../../mocks/MigrateProtocolAdapterState.m.sol";

/// @notice Checks the migration run end to end against a stand-in for the legacy protocol adapter, and the
/// check of a migrated proxy.
contract MigrateProtocolAdapterStateTest is MigrationFixture {
    using TxGen for Vm;

    function test_run_copies_the_state_and_leaves_the_proxy_paused() public {
        ProtocolAdapter pa = ProtocolAdapter(_proxy);
        bytes32 kindTreeRoot = pa.getKindTreeRoot();
        address migrationalImplementation = pa.getImplementation();

        _migrationScript.run({isProduction: false});

        assertEq(
            pa.latestCommitmentTreeRoot(),
            _legacyProtocolAdapter.latestCommitmentTreeRoot(),
            "the root differs from the legacy protocol adapter"
        );
        assertEq(
            pa.commitmentCount(), _COMMITMENT_COUNT, "the commitment count differs from the legacy protocol adapter"
        );
        assertEq(
            pa.commitmentTreeDepth(),
            _legacyProtocolAdapter.commitmentTreeDepth(),
            "the tree depth differs from the legacy protocol adapter"
        );
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            assertTrue(
                pa.isNullifierContained(_legacyProtocolAdapter.nullifierAtIndex(i)),
                "a nullifier of the legacy protocol adapter is missing"
            );
        }

        assertTrue(pa.paused(), "the proxy should stay paused");
        assertEq(
            pa.getImplementation(), migrationalImplementation, "the proxy should keep the migrational implementation"
        );
        assertEq(pa.getKindTreeRoot(), kindTreeRoot, "the run should not change the kind tree root");
        assertEq(pa.owner(), DEFAULT_SENDER, "the run should not change the owner");
    }

    function test_run_reverts_if_the_legacy_protocol_adapter_is_still_running() public {
        LegacyProtocolAdapterMock running = new LegacyProtocolAdapterMock(DEFAULT_SENDER);
        running.addCommitment(keccak256("commitment"));
        address proxy = _deployMigrationalProxy(address(running));
        MigrateProtocolAdapterState script =
            new MigrateProtocolAdapterStateMock({legacyProtocolAdapter: address(running), proxy: proxy});

        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationalProtocolAdapter.LegacyProtocolAdapterNotStopped.selector, address(running)
            )
        );
        script.run({isProduction: false});
    }

    function test_run_reverts_for_a_proxy_that_copies_from_another_legacy_protocol_adapter() public {
        address other = makeAddr("other legacy protocol adapter");
        MigrateProtocolAdapterState script =
            new MigrateProtocolAdapterStateMock({legacyProtocolAdapter: other, proxy: _proxy});

        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationScript.LegacyProtocolAdapterMismatch.selector, other, address(_legacyProtocolAdapter)
            )
        );
        script.run({isProduction: false});
    }

    function test_run_reverts_on_a_chain_that_records_no_legacy_protocol_adapter() public {
        MigrateProtocolAdapterState script = new MigrateProtocolAdapterState();
        uint256 chainId = 31337;
        vm.chainId(chainId);

        vm.expectRevert(abi.encodeWithSelector(MigrationScript.LegacyProtocolAdapterNotRecorded.selector, chainId));
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_environment_records_no_proxy() public {
        MigrateProtocolAdapterState script = new MigrateProtocolAdapterStateMock({
            legacyProtocolAdapter: address(_legacyProtocolAdapter), proxy: address(0)
        });

        vm.expectRevert(
            abi.encodeWithSelector(MigrationScript.DeploymentNotRecorded.selector, "production", block.chainid)
        );
        script.run({isProduction: true});
    }

    function test_run_sends_one_transaction_per_nullifier_batch() public {
        uint256 perBatch = _migrationScript.NULLIFIERS_PER_BATCH();
        uint256 expectedBatches = (_NULLIFIER_COUNT + perBatch - 1) / perBatch;
        assertGt(expectedBatches, 1, "the fixture should need more than one batch");

        vm.recordLogs();
        _migrationScript.run({isProduction: false});

        uint256 batches;
        bytes32 topic = keccak256("NullifierBatchMigrated(uint256,uint256)");
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; ++i) {
            if (logs[i].emitter == _proxy && logs[i].topics[0] == topic) ++batches;
        }

        assertEq(batches, expectedBatches, "one seeding transaction per batch");
    }

    function test_run_resumes_a_run_that_stopped_after_the_commitment_tree() public {
        bytes32[] memory sides = _legacyProtocolAdapter.commitmentTreeSides();
        vm.prank(DEFAULT_SENDER);
        MigrationalProtocolAdapter(_proxy).migrateCommitmentTree(sides);

        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});

        _migrationScript.verify({isProduction: false});
    }

    /// @dev The proxy holds one nullifier more than a whole batch, so neither a restart at zero nor one at a batch
    /// boundary continues the copy.
    function test_run_resumes_a_run_that_stopped_inside_the_nullifier_set() public {
        uint256 copied = _migrationScript.NULLIFIERS_PER_BATCH() + 1;
        assertLt(copied, _NULLIFIER_COUNT, "the earlier run should leave nullifiers to copy");
        bytes32[] memory sides = _legacyProtocolAdapter.commitmentTreeSides();
        vm.startPrank(DEFAULT_SENDER);
        MigrationalProtocolAdapter(_proxy).migrateCommitmentTree(sides);
        MigrationalProtocolAdapter(_proxy).migrateNullifierSet({start: 0, count: copied});
        vm.stopPrank();

        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});

        _migrationScript.verify({isProduction: false});
    }

    function test_verify_reverts_if_a_production_proxy_was_not_transferred() public {
        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});
        address productionOwner = new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION();

        vm.expectRevert(
            abi.encodeWithSelector(MigrateProtocolAdapterState.OwnerMismatch.selector, productionOwner, DEFAULT_SENDER)
        );
        _migrationScript.verify({isProduction: true});
    }

    function test_verify_accepts_a_proxy_that_settled_after_the_finalization() public {
        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});
        ProtocolAdapter pa = ProtocolAdapter(_proxy);

        // A mock aggregation proof stands in for the real one; the delta proof is checked as usual.
        RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(pa.RISC_ZERO_VERIFIER_SELECTOR());
        RiscZeroVerifierEmergencyStop verifier = RiscZeroVerifierEmergencyStop(
            address(
                RiscZeroVerifierRouter(pa.RISC_ZERO_VERIFIER_ROUTER()).getVerifier(pa.RISC_ZERO_VERIFIER_SELECTOR())
            )
        );
        vm.etch(address(verifier.verifier()), address(mockVerifier).code);
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        pa.execute(txn);

        assertEq(pa.commitmentCount(), _COMMITMENT_COUNT + 1, "the transaction should add a commitment");
        assertNotEq(
            pa.latestCommitmentTreeRoot(),
            _legacyProtocolAdapter.latestCommitmentTreeRoot(),
            "the transaction should move the latest root on"
        );
        _migrationScript.verify({isProduction: false});
    }

    function test_verify_reverts_if_the_proxy_holds_fewer_commitments() public {
        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});
        vm.mockCall(_proxy, abi.encodeCall(ICommitmentTree.commitmentCount, ()), abi.encode(_COMMITMENT_COUNT - 1));

        vm.expectRevert(
            abi.encodeWithSelector(
                MigrateProtocolAdapterState.CommitmentCountMismatch.selector, _COMMITMENT_COUNT, _COMMITMENT_COUNT - 1
            )
        );
        _migrationScript.verify({isProduction: false});
    }

    function test_verify_reverts_on_a_different_latest_root_without_new_commitments() public {
        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});
        bytes32 expected = _legacyProtocolAdapter.latestCommitmentTreeRoot();
        bytes32 actual = keccak256("different root");
        vm.mockCall(_proxy, abi.encodeCall(ICommitmentTree.latestCommitmentTreeRoot, ()), abi.encode(actual));

        vm.expectRevert(
            abi.encodeWithSelector(MigrateProtocolAdapterState.CommitmentTreeRootMismatch.selector, expected, actual)
        );
        _migrationScript.verify({isProduction: false});
    }

    function test_verify_reverts_if_a_root_the_copy_left_is_missing() public {
        _migrationScript.run({isProduction: false});
        _finalizationScript.run({isProduction: false});
        bytes32[2] memory roots = [_legacyProtocolAdapter.latestCommitmentTreeRoot(), SHA256.EMPTY_HASH];

        for (uint256 i = 0; i < roots.length; ++i) {
            vm.mockCall(
                _proxy, abi.encodeCall(ICommitmentTree.isCommitmentTreeRootHistorical, (roots[i])), abi.encode(false)
            );
            vm.expectRevert(
                abi.encodeWithSelector(MigrateProtocolAdapterState.MissingHistoricalRoot.selector, roots[i])
            );
            _migrationScript.verify({isProduction: false});
            vm.clearMockedCalls();
        }
    }
}
