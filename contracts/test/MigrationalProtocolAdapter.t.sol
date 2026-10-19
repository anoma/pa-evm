// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.5/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {ICommitmentTree} from "../src/interfaces/ICommitmentTree.sol";
import {IMigrational} from "../src/interfaces/IMigrational.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {MerkleTree} from "../src/libs/MerkleTree.sol";
import {SHA256} from "../src/libs/SHA256.sol";
import {MigrationalProtocolAdapter} from "../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {NullifierSet} from "../src/state/NullifierSet.sol";
import {TxGen} from "./libs/TxGen.sol";
import {LegacyProtocolAdapterMock} from "./mocks/LegacyProtocolAdapter.m.sol";
import {
    MigrationalProtocolAdapterEmptySubtreeRootsMock
} from "./mocks/MigrationalProtocolAdapterEmptySubtreeRoots.m.sol";

/// @dev The legacy protocol adapter is a stand-in with the storage layout of the deployed one. The
/// migrational adapter reads every value from it, so the test fills it, stops it, and then checks what the migrational
/// adapter copies in.
contract MigrationalProtocolAdapterTest is Test {
    using MerkleTree for MerkleTree.Tree;
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    address internal constant _UNAUTHORIZED_CALLER = address(uint160(2));
    /// @dev `CommitmentTree._COMMITMENT_TREE_STORAGE_SLOT`, which the base declares as internal.
    bytes32 internal constant _COMMITMENT_TREE_STORAGE_SLOT =
        0x762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900;

    uint256 internal constant _COMMITMENT_COUNT = 5;
    uint256 internal constant _NULLIFIER_COUNT = 7;

    RiscZeroVerifierRouter internal _router;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    RiscZeroMockVerifier internal _verifier;

    LegacyProtocolAdapterMock internal _legacyProtocolAdapter;
    MigrationalProtocolAdapter internal _pa;

    MerkleTree.Tree internal _grownTree;

    function setUp() public {
        (_router, _emergencyStop, _verifier) = new DeployRiscZeroContractsMock().run();

        _legacyProtocolAdapter =
            _deployLegacyProtocolAdapter({commitments: _COMMITMENT_COUNT, nullifiers: _NULLIFIER_COUNT});
        _legacyProtocolAdapter.emergencyStop();

        _pa = _deployMigrationalProxy(address(_legacyProtocolAdapter));
    }

    function test_computeEmptySubtreeRoots_matches_the_empty_subtree_roots_a_grown_tree_stores() public {
        MigrationalProtocolAdapterEmptySubtreeRootsMock mock = new MigrationalProtocolAdapterEmptySubtreeRootsMock(
            address(_router), _verifier.SELECTOR(), address(_legacyProtocolAdapter)
        );

        _grownTree.setup();
        for (uint256 i = 0; i < 20; ++i) {
            _grownTree.push(keccak256(abi.encode("leaf", i)));
        }

        bytes32[] memory roots = mock.computeEmptySubtreeRoots(_grownTree.depth());

        assertEq(roots.length, _grownTree._emptySubtreeRoots.length, "the number of empty subtree roots differs");
        for (uint256 i = 0; i < roots.length; ++i) {
            assertEq(roots[i], _grownTree._emptySubtreeRoots[i], "an empty subtree root differs");
        }
    }

    function test_constructor_reverts_on_a_zero_legacy_protocol_adapter() public {
        vm.expectRevert(MigrationalProtocolAdapter.ZeroLegacyProtocolAdapterNotAllowed.selector);
        new MigrationalProtocolAdapter(address(_router), _verifier.SELECTOR(), address(0));
    }

    function test_execute_reverts_while_paused() public {
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });

        vm.expectRevert(Pausable.EnforcedPause.selector, address(_pa));
        _pa.execute(txn);
    }

    function test_migrateCommitmentTree_reproduces_the_tree_of_the_legacy_protocol_adapter() public {
        vm.expectEmit(address(_pa));
        emit IMigrational.CommitmentTreeMigrated({
            root: _legacyProtocolAdapter.latestCommitmentTreeRoot(), leafCount: _COMMITMENT_COUNT
        });
        _migrateCommitmentTree();

        assertEq(
            _pa.latestCommitmentTreeRoot(),
            _legacyProtocolAdapter.latestCommitmentTreeRoot(),
            "latest root differs from the legacy protocol adapter"
        );
        assertEq(
            _pa.commitmentCount(),
            _legacyProtocolAdapter.commitmentCount(),
            "commitment count differs from the legacy protocol adapter"
        );
        assertEq(
            _pa.commitmentTreeDepth(),
            _legacyProtocolAdapter.commitmentTreeDepth(),
            "tree depth differs from the legacy protocol adapter"
        );
        assertEq(
            _pa.commitmentTreeCapacity(),
            _legacyProtocolAdapter.commitmentTreeCapacity(),
            "capacity differs from the legacy protocol adapter"
        );
    }

    function test_migrateCommitmentTree_keeps_the_empty_tree_root_and_the_latest_root_of_the_legacy_protocol_adapter()
        public
    {
        _migrateCommitmentTree();

        assertTrue(_pa.isCommitmentTreeRootHistorical(SHA256.EMPTY_HASH), "the empty-tree root should stay");
        assertTrue(
            _pa.isCommitmentTreeRootHistorical(_legacyProtocolAdapter.latestCommitmentTreeRoot()),
            "the latest root of the legacy protocol adapter is missing"
        );
    }

    function test_unpause_reverts_if_the_empty_tree_root_is_missing() public {
        _copyInEverything();
        _removeHistoricalRoot(SHA256.EMPTY_HASH);

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(MigrationalProtocolAdapter.MissingHistoricalRoot.selector, SHA256.EMPTY_HASH)
        );
        _pa.unpause();
    }

    function test_unpause_reverts_if_the_latest_root_of_the_legacy_protocol_adapter_is_missing() public {
        _copyInEverything();
        bytes32 root = _legacyProtocolAdapter.latestCommitmentTreeRoot();
        _removeHistoricalRoot(root);

        vm.prank(_OWNER);
        vm.expectRevert(abi.encodeWithSelector(MigrationalProtocolAdapter.MissingHistoricalRoot.selector, root));
        _pa.unpause();
    }

    function test_migrateCommitmentTree_emits_the_migrated_root_as_a_commitment_tree_root() public {
        vm.expectEmit(address(_pa));
        emit ICommitmentTree.CommitmentTreeRootAdded({root: _legacyProtocolAdapter.latestCommitmentTreeRoot()});
        _migrateCommitmentTree();
    }

    /// @dev A resource that is created and consumed in the same transaction carries the root of the empty tree, so
    /// dropping that root would stop every wrap and every unwrap on the migrated chain.
    function test_execute_settles_a_transaction_that_proves_against_the_empty_tree_root() public {
        _seedState();

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        assertEq(txn.actions[0].resources[0].commitmentTreeRoot, SHA256.EMPTY_HASH, "the fixture uses another root");

        _pa.execute(txn);

        assertTrue(_pa.isNullifierContained(TxGen.collectNullifiers(txn)[0]), "the transaction did not settle");
    }

    function test_migrateCommitmentTree_reverts_on_left_siblings_that_do_not_reproduce_the_root_of_the_legacy_protocol_adapter()
        public
    {
        bytes32[] memory leftSiblings = _legacyProtocolAdapter.commitmentTreeSides();
        leftSiblings[0] = bytes32(uint256(leftSiblings[0]) ^ 1);

        vm.prank(_OWNER);
        vm.expectRevert();
        _pa.migrateCommitmentTree(leftSiblings);
    }

    function test_migrateCommitmentTree_reverts_if_the_tree_is_not_empty() public {
        _migrateCommitmentTree();

        bytes32[] memory leftSiblings = _legacyProtocolAdapter.commitmentTreeSides();

        vm.prank(_OWNER);
        vm.expectRevert(MigrationalProtocolAdapter.CommitmentTreeNotEmpty.selector);
        _pa.migrateCommitmentTree(leftSiblings);
    }

    function test_migrateCommitmentTree_reverts_on_an_empty_tree_of_the_legacy_protocol_adapter() public {
        LegacyProtocolAdapterMock empty = _deployLegacyProtocolAdapter({commitments: 0, nullifiers: 0});
        empty.emergencyStop();
        MigrationalProtocolAdapter pa = _deployMigrationalProxy(address(empty));

        bytes32[] memory leftSiblings = empty.commitmentTreeSides();

        vm.prank(_OWNER);
        vm.expectRevert(MigrationalProtocolAdapter.EmptyCommitmentTreeNotAllowed.selector);
        pa.migrateCommitmentTree(leftSiblings);
    }

    function test_migrateCommitmentTree_reverts_if_the_leaf_count_exceeds_the_capacity() public {
        bytes32[] memory leftSiblings = _legacyProtocolAdapter.commitmentTreeSides();
        bytes32[] memory tooFew = new bytes32[](leftSiblings.length - 1);
        for (uint256 i = 0; i < tooFew.length; ++i) {
            tooFew[i] = leftSiblings[i];
        }
        uint256 capacity = uint256(1) << tooFew.length;

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationalProtocolAdapter.LeafCountExceedsCapacity.selector, _COMMITMENT_COUNT, capacity
            )
        );
        _pa.migrateCommitmentTree(tooFew);
    }

    function test_migrateCommitmentTree_reverts_while_the_legacy_protocol_adapter_runs() public {
        LegacyProtocolAdapterMock running = _deployLegacyProtocolAdapter({commitments: 1, nullifiers: 0});
        MigrationalProtocolAdapter pa = _deployMigrationalProxy(address(running));

        bytes32[] memory leftSiblings = running.commitmentTreeSides();

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationalProtocolAdapter.LegacyProtocolAdapterNotStopped.selector, address(running)
            )
        );
        pa.migrateCommitmentTree(leftSiblings);
    }

    function test_migrateCommitmentTree_reverts_for_an_unauthorized_caller() public {
        bytes32[] memory leftSiblings = _legacyProtocolAdapter.commitmentTreeSides();

        vm.prank(_UNAUTHORIZED_CALLER);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, _UNAUTHORIZED_CALLER));
        _pa.migrateCommitmentTree(leftSiblings);
    }

    function test_migrateNullifierSet_copies_the_batches_at_the_indices_of_the_legacy_protocol_adapter() public {
        vm.startPrank(_OWNER);
        vm.expectEmit(address(_pa));
        emit IMigrational.NullifierBatchMigrated({start: 0, count: 3});
        _pa.migrateNullifierSet({start: 0, count: 3});

        vm.expectEmit(address(_pa));
        emit IMigrational.NullifierBatchMigrated({start: 3, count: _NULLIFIER_COUNT - 3});
        _pa.migrateNullifierSet({start: 3, count: _NULLIFIER_COUNT - 3});
        vm.stopPrank();

        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            assertTrue(
                _pa.isNullifierContained(_legacyProtocolAdapter.nullifierAtIndex(i)),
                "a nullifier of the legacy protocol adapter is missing"
            );
        }
    }

    function test_migrateNullifierSet_reverts_on_a_batch_beyond_what_the_legacy_protocol_adapter_holds() public {
        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationalProtocolAdapter.NullifierBatchOutOfRange.selector, _NULLIFIER_COUNT + 1, _NULLIFIER_COUNT
            )
        );
        _pa.migrateNullifierSet({start: 0, count: _NULLIFIER_COUNT + 1});
    }

    function test_migrateNullifierSet_reverts_on_a_batch_that_leaves_a_gap() public {
        uint256 copied = 3;
        vm.startPrank(_OWNER);
        _pa.migrateNullifierSet({start: 0, count: copied});

        vm.expectRevert(abi.encodeWithSelector(MigrationalProtocolAdapter.NullifierBatchLeavesGap.selector, copied + 1));
        _pa.migrateNullifierSet({start: copied + 1, count: 1});
        vm.stopPrank();
    }

    function test_migrateNullifierSet_reverts_on_a_batch_that_repeats_a_nullifier() public {
        uint256 copied = 3;
        vm.startPrank(_OWNER);
        _pa.migrateNullifierSet({start: 0, count: copied});

        vm.expectRevert(
            abi.encodeWithSelector(
                NullifierSet.PreExistingNullifier.selector, _legacyProtocolAdapter.nullifierAtIndex(copied - 1)
            )
        );
        _pa.migrateNullifierSet({start: copied - 1, count: 2});
        vm.stopPrank();
    }

    function test_migrateNullifierSet_reverts_for_an_unauthorized_caller() public {
        vm.prank(_UNAUTHORIZED_CALLER);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, _UNAUTHORIZED_CALLER));
        _pa.migrateNullifierSet({start: 0, count: 1});
    }

    function test_unpause_reverts_before_the_commitment_tree_is_copied_in() public {
        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationalProtocolAdapter.CommitmentCountMismatch.selector, _COMMITMENT_COUNT, uint256(0)
            )
        );
        _pa.unpause();
    }

    function test_unpause_reverts_while_nullifiers_are_missing() public {
        _migrateCommitmentTree();
        vm.startPrank(_OWNER);
        _pa.migrateNullifierSet({start: 0, count: _NULLIFIER_COUNT - 1});

        vm.expectRevert(
            abi.encodeWithSelector(
                MigrationalProtocolAdapter.MissingNullifier.selector,
                _legacyProtocolAdapter.nullifierAtIndex(_NULLIFIER_COUNT - 1)
            )
        );
        _pa.unpause();
        vm.stopPrank();
    }

    function test_unpause_succeeds_for_a_legacy_protocol_adapter_without_nullifiers() public {
        LegacyProtocolAdapterMock legacyProtocolAdapter = _deployLegacyProtocolAdapter({commitments: 1, nullifiers: 0});
        legacyProtocolAdapter.emergencyStop();
        MigrationalProtocolAdapter pa = _deployMigrationalProxy(address(legacyProtocolAdapter));
        bytes32[] memory leftSiblings = legacyProtocolAdapter.commitmentTreeSides();

        vm.startPrank(_OWNER);
        pa.migrateCommitmentTree(leftSiblings);
        pa.unpause();
        vm.stopPrank();

        assertFalse(pa.paused(), "the protocol adapter should be unpaused");
    }

    function test_unpause_succeeds_once_the_state_matches_the_legacy_protocol_adapter() public {
        _seedState();

        assertFalse(_pa.paused(), "the protocol adapter should be unpaused");
    }

    function test_seeding_reverts_once_unpaused() public {
        _seedState();

        bytes32[] memory leftSiblings = _legacyProtocolAdapter.commitmentTreeSides();

        vm.startPrank(_OWNER);
        vm.expectRevert(Pausable.ExpectedPause.selector);
        _pa.migrateCommitmentTree(leftSiblings);

        vm.expectRevert(Pausable.ExpectedPause.selector);
        _pa.migrateNullifierSet({start: 0, count: 1});
        vm.stopPrank();
    }

    function test_the_copied_tree_grows_as_the_tree_of_the_legacy_protocol_adapter_would_have() public {
        _seedState();

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 2, consumedCount: 1, createdCount: 2})
        });
        _pa.execute(txn);

        // The legacy protocol adapter receives the same commitments in the same order.
        bytes32[] memory commitments = TxGen.collectCommitments(txn);
        for (uint256 i = 0; i < commitments.length; ++i) {
            _legacyProtocolAdapter.addCommitment(commitments[i]);
        }

        assertEq(
            _pa.latestCommitmentTreeRoot(),
            _legacyProtocolAdapter.latestCommitmentTreeRoot(),
            "the copied tree grows differently"
        );
        assertEq(
            _pa.commitmentCount(), _legacyProtocolAdapter.commitmentCount(), "commitment count differs after growth"
        );
    }

    function test_upgrade_to_the_base_adapter_keeps_the_state_and_drops_the_copy_in() public {
        _seedState();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR());
        opts.referenceContract = "MigrationalProtocolAdapter.sol";
        Upgrades.upgradeProxy({
            proxy: address(_pa), contractName: "ProtocolAdapter.sol", data: "", opts: opts, tryCaller: _OWNER
        });

        ProtocolAdapter base = ProtocolAdapter(address(_pa));
        assertEq(
            base.latestCommitmentTreeRoot(),
            _legacyProtocolAdapter.latestCommitmentTreeRoot(),
            "the root should survive"
        );
        assertEq(base.commitmentCount(), _COMMITMENT_COUNT, "the commitments should survive");
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            assertTrue(
                base.isNullifierContained(_legacyProtocolAdapter.nullifierAtIndex(i)), "the nullifiers should survive"
            );
        }
        assertFalse(base.paused(), "the adapter should stay unpaused");

        vm.prank(_OWNER);
        vm.expectRevert();
        _pa.migrateNullifierSet({start: 0, count: 1});
    }

    function test_initialize_starts_paused_and_records_the_legacy_protocol_adapter() public view {
        assertTrue(_pa.paused(), "the migrational adapter should start paused");
        assertEq(
            _pa.getLegacyProtocolAdapter(),
            address(_legacyProtocolAdapter),
            "the recorded legacy protocol adapter differs"
        );
        assertEq(_pa.owner(), _OWNER, "owner differs");
        assertEq(_pa.commitmentCount(), 0, "the tree should start empty");
    }

    /// @notice Deploys a stand-in for the legacy protocol adapter and fills it.
    function _deployLegacyProtocolAdapter(uint256 commitments, uint256 nullifiers)
        internal
        returns (LegacyProtocolAdapterMock legacyProtocolAdapter)
    {
        legacyProtocolAdapter = new LegacyProtocolAdapterMock(address(this));
        for (uint256 i = 0; i < commitments; ++i) {
            legacyProtocolAdapter.addCommitment(keccak256(abi.encode("commitment", i)));
        }
        for (uint256 i = 0; i < nullifiers; ++i) {
            legacyProtocolAdapter.addNullifier(keccak256(abi.encode("nullifier", i)));
        }
    }

    /// @notice Deploys a paused migrational proxy bound to the given legacy protocol adapter.
    function _deployMigrationalProxy(address legacyProtocolAdapter)
        internal
        returns (MigrationalProtocolAdapter migrationalProxy)
    {
        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR(), legacyProtocolAdapter);
        migrationalProxy = MigrationalProtocolAdapter(
            Upgrades.deployUUPSProxy(
                "MigrationalProtocolAdapter.sol", abi.encodeCall(MigrationalProtocolAdapter.initialize, (_OWNER)), opts
            )
        );
    }

    function _migrateCommitmentTree() internal {
        bytes32[] memory leftSiblings = _legacyProtocolAdapter.commitmentTreeSides();

        vm.prank(_OWNER);
        _pa.migrateCommitmentTree(leftSiblings);
    }

    function _copyInEverything() internal {
        _migrateCommitmentTree();
        vm.prank(_OWNER);
        _pa.migrateNullifierSet({start: 0, count: _NULLIFIER_COUNT});
    }

    function _seedState() internal {
        _copyInEverything();
        vm.prank(_OWNER);
        _pa.unpause();
    }

    /// @dev Removes a root from the set of historical roots, after checking that the set is where this test writes.
    function _removeHistoricalRoot(bytes32 root) internal {
        bytes32 slot = keccak256(abi.encode(root, uint256(_COMMITMENT_TREE_STORAGE_SLOT) + 3));
        assertEq(
            uint256(vm.load({target: address(_pa), slot: slot})), 1, "the root is not stored where this test writes"
        );

        vm.store({target: address(_pa), slot: slot, value: bytes32(0)});
    }
}
