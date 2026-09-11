// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.1/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.16.2/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {ICommitmentTree} from "../src/interfaces/ICommitmentTree.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {IProtocolAdapterTransition} from "../src/interfaces/IProtocolAdapterTransition.sol";
import {SHA256} from "../src/libs/SHA256.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {ProtocolAdapterTransition} from "../src/ProtocolAdapterTransition.sol";
import {TxGen} from "./libs/TxGen.sol";
import {ProtocolAdapterV1Mock} from "./mocks/ProtocolAdapterV1.m.sol";

/// @dev The v1 protocol adapter is a stand-in holding its state in v1's storage layout. The transition adapter reads
/// every value from it, so the test fills it, stops it, and then checks what the transition adapter copies in.
contract ProtocolAdapterTransitionTest is Test {
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

    ProtocolAdapterV1Mock internal _v1;
    ProtocolAdapterTransition internal _pa;

    function setUp() public {
        (_router, _emergencyStop, _verifier) = new DeployRiscZeroContractsMock().run();

        _v1 = _deployProtocolAdapterV1({commitments: _COMMITMENT_COUNT, nullifiers: _NULLIFIER_COUNT});
        _v1.emergencyStop();

        _pa = _deployTransitionProxy(address(_v1));
    }

    function test_initialize_starts_paused_and_records_the_v1_protocol_adapter() public view {
        assertTrue(_pa.paused(), "the transition adapter should start paused");
        assertEq(_pa.getProtocolAdapterV1(), address(_v1), "the recorded v1 protocol adapter differs");
        assertEq(_pa.owner(), _OWNER, "owner differs");
        assertEq(_pa.commitmentCount(), 0, "the tree should start empty");
    }

    function test_constructor_reverts_on_a_zero_v1_protocol_adapter() public {
        vm.expectRevert(ProtocolAdapterTransition.ZeroProtocolAdapterV1NotAllowed.selector);
        new ProtocolAdapterTransition(address(_router), _verifier.SELECTOR(), address(0));
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

    function test_seedCommitmentTree_reproduces_the_v1_tree() public {
        vm.expectEmit(address(_pa));
        emit IProtocolAdapterTransition.CommitmentTreeSeeded({
            root: _v1.latestCommitmentTreeRoot(), leafCount: _COMMITMENT_COUNT
        });
        _seedCommitmentTree();

        assertEq(_pa.latestCommitmentTreeRoot(), _v1.latestCommitmentTreeRoot(), "latest root differs from v1");
        assertEq(_pa.commitmentCount(), _v1.commitmentCount(), "commitment count differs from v1");
        assertEq(_pa.commitmentTreeDepth(), _v1.commitmentTreeDepth(), "tree depth differs from v1");
        assertEq(_pa.commitmentTreeCapacity(), _v1.commitmentTreeCapacity(), "capacity differs from v1");
    }

    function test_seedCommitmentTree_keeps_the_empty_tree_root_and_v1_s_latest_root() public {
        _seedCommitmentTree();

        assertEq(_pa.commitmentTreeRootCount(), 2, "the historical root count differs");
        assertEq(_pa.commitmentTreeRootAtIndex(0), SHA256.EMPTY_HASH, "the empty-tree root should be the first root");
        assertEq(_pa.commitmentTreeRootAtIndex(1), _v1.latestCommitmentTreeRoot(), "the second root differs from v1");
    }

    function test_unpause_reverts_if_the_empty_tree_root_is_not_the_first_root() public {
        _copyInEverything();

        bytes32 wrong = bytes32(uint256(SHA256.EMPTY_HASH) ^ 1);
        _overwriteHistoricalRoot({index: 0, root: wrong});

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapterTransition.HistoricalRootMismatch.selector, 0, SHA256.EMPTY_HASH, wrong
            )
        );
        _pa.unpause();
    }

    function test_unpause_reverts_if_the_second_root_is_not_the_root_of_v1() public {
        _copyInEverything();

        bytes32 expected = _v1.latestCommitmentTreeRoot();
        bytes32 wrong = bytes32(uint256(expected) ^ 1);
        _overwriteHistoricalRoot({index: 1, root: wrong});

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolAdapterTransition.HistoricalRootMismatch.selector, 1, expected, wrong)
        );
        _pa.unpause();
    }

    /// @dev The indexer rebuilds the tree from `CommitmentTreeRootAdded` events, and the seeded leaves emit nothing,
    /// so this event is the only way it learns that the tree went from empty to v1's latest root.
    function test_seedCommitmentTree_emits_the_seeded_root_as_a_commitment_tree_root() public {
        vm.expectEmit(address(_pa));
        emit ICommitmentTree.CommitmentTreeRootAdded({root: _v1.latestCommitmentTreeRoot()});
        _seedCommitmentTree();
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
        assertEq(txn.actions[0].consumed[0].commitmentTreeRoot, SHA256.EMPTY_HASH, "the fixture uses another root");

        _pa.execute(txn);

        assertEq(_pa.nullifierCount(), _NULLIFIER_COUNT + 1, "the transaction did not settle");
    }

    function test_seedCommitmentTree_reverts_on_sides_that_do_not_reproduce_the_v1_root() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();
        sides[0] = bytes32(uint256(sides[0]) ^ 1);

        vm.prank(_OWNER);
        vm.expectRevert();
        _pa.seedCommitmentTree(sides);
    }

    function test_seedCommitmentTree_reverts_if_the_tree_is_not_empty() public {
        _seedCommitmentTree();

        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.prank(_OWNER);
        vm.expectRevert(ProtocolAdapterTransition.CommitmentTreeNotEmpty.selector);
        _pa.seedCommitmentTree(sides);
    }

    function test_seedCommitmentTree_reverts_on_an_empty_v1_tree() public {
        ProtocolAdapterV1Mock empty = _deployProtocolAdapterV1({commitments: 0, nullifiers: 0});
        empty.emergencyStop();
        ProtocolAdapterTransition pa = _deployTransitionProxy(address(empty));

        bytes32[] memory sides = empty.commitmentTreeSides();

        vm.prank(_OWNER);
        vm.expectRevert(ProtocolAdapterTransition.EmptyCommitmentTreeNotAllowed.selector);
        pa.seedCommitmentTree(sides);
    }

    function test_seedCommitmentTree_reverts_if_the_leaf_count_exceeds_the_capacity() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();
        bytes32[] memory tooFew = new bytes32[](sides.length - 1);
        for (uint256 i = 0; i < tooFew.length; ++i) {
            tooFew[i] = sides[i];
        }
        uint256 capacity = uint256(1) << tooFew.length;

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapterTransition.LeafCountExceedsCapacity.selector, _COMMITMENT_COUNT, capacity
            )
        );
        _pa.seedCommitmentTree(tooFew);
    }

    function test_seedCommitmentTree_reverts_while_the_v1_protocol_adapter_runs() public {
        ProtocolAdapterV1Mock running = _deployProtocolAdapterV1({commitments: 1, nullifiers: 0});
        ProtocolAdapterTransition pa = _deployTransitionProxy(address(running));

        bytes32[] memory sides = running.commitmentTreeSides();

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolAdapterTransition.ProtocolAdapterV1NotStopped.selector, address(running))
        );
        pa.seedCommitmentTree(sides);
    }

    function test_seedCommitmentTree_reverts_for_an_unauthorized_caller() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.prank(_UNAUTHORIZED_CALLER);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, _UNAUTHORIZED_CALLER));
        _pa.seedCommitmentTree(sides);
    }

    function test_seedNullifierSet_copies_the_batches_at_the_v1_indices() public {
        vm.startPrank(_OWNER);
        vm.expectEmit(address(_pa));
        emit IProtocolAdapterTransition.NullifierSetSeeded({start: 0, count: 3});
        _pa.seedNullifierSet(3);

        vm.expectEmit(address(_pa));
        emit IProtocolAdapterTransition.NullifierSetSeeded({start: 3, count: _NULLIFIER_COUNT - 3});
        _pa.seedNullifierSet(_NULLIFIER_COUNT - 3);
        vm.stopPrank();

        assertEq(_pa.nullifierCount(), _v1.nullifierCount(), "nullifier count differs from v1");
        for (uint256 i = 0; i < _NULLIFIER_COUNT; ++i) {
            assertEq(_pa.nullifierAtIndex(i), _v1.nullifierAtIndex(i), "a nullifier sits at a different index");
        }
    }

    function test_seedNullifierSet_reverts_on_a_batch_beyond_what_v1_holds() public {
        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapterTransition.NullifierBatchOutOfRange.selector, _NULLIFIER_COUNT, _NULLIFIER_COUNT + 1
            )
        );
        _pa.seedNullifierSet(_NULLIFIER_COUNT + 1);
    }

    function test_seedNullifierSet_reverts_for_an_unauthorized_caller() public {
        vm.prank(_UNAUTHORIZED_CALLER);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, _UNAUTHORIZED_CALLER));
        _pa.seedNullifierSet(1);
    }

    function test_unpause_reverts_before_the_commitment_tree_is_copied_in() public {
        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapterTransition.CommitmentCountMismatch.selector, _COMMITMENT_COUNT, uint256(0)
            )
        );
        _pa.unpause();
    }

    function test_unpause_reverts_while_nullifiers_are_missing() public {
        _seedCommitmentTree();
        vm.startPrank(_OWNER);
        _pa.seedNullifierSet(_NULLIFIER_COUNT - 1);

        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapterTransition.NullifierCountMismatch.selector, _NULLIFIER_COUNT, _NULLIFIER_COUNT - 1
            )
        );
        _pa.unpause();
        vm.stopPrank();
    }

    function test_unpause_succeeds_once_the_state_matches_v1() public {
        _seedState();

        assertFalse(_pa.paused(), "the protocol adapter should be unpaused");
    }

    function test_seeding_reverts_once_unpaused() public {
        _seedState();

        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.startPrank(_OWNER);
        vm.expectRevert(Pausable.ExpectedPause.selector);
        _pa.seedCommitmentTree(sides);

        vm.expectRevert(Pausable.ExpectedPause.selector);
        _pa.seedNullifierSet(1);
        vm.stopPrank();
    }

    function test_the_copied_tree_grows_as_the_v1_tree_would_have() public {
        _seedState();

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 2, consumedCount: 1, createdCount: 2})
        });
        _pa.execute(txn);

        // The v1 protocol adapter receives the same commitments in the same order.
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            for (uint256 j = 0; j < txn.actions[i].created.length; ++j) {
                _v1.addCommitment(txn.actions[i].created[j].commitment);
            }
        }

        assertEq(_pa.latestCommitmentTreeRoot(), _v1.latestCommitmentTreeRoot(), "the copied tree grows differently");
        assertEq(_pa.commitmentCount(), _v1.commitmentCount(), "commitment count differs after growth");
    }

    function test_upgrade_to_the_base_adapter_keeps_the_state_and_drops_the_copy_in() public {
        _seedState();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR());
        opts.referenceContract = "ProtocolAdapterTransition.sol";
        Upgrades.upgradeProxy({
            proxy: address(_pa), contractName: "ProtocolAdapter.sol", data: "", opts: opts, tryCaller: _OWNER
        });

        ProtocolAdapter base = ProtocolAdapter(address(_pa));
        assertEq(base.latestCommitmentTreeRoot(), _v1.latestCommitmentTreeRoot(), "the root should survive");
        assertEq(base.commitmentCount(), _COMMITMENT_COUNT, "the commitments should survive");
        assertEq(base.nullifierCount(), _NULLIFIER_COUNT, "the nullifiers should survive");
        assertFalse(base.paused(), "the adapter should stay unpaused");

        vm.prank(_OWNER);
        vm.expectRevert();
        _pa.seedNullifierSet(1);
    }

    /// @notice Deploys a stand-in for the v1 protocol adapter and fills it.
    function _deployProtocolAdapterV1(uint256 commitments, uint256 nullifiers)
        internal
        returns (ProtocolAdapterV1Mock protocolAdapterV1)
    {
        protocolAdapterV1 = new ProtocolAdapterV1Mock(address(this));
        for (uint256 i = 0; i < commitments; ++i) {
            protocolAdapterV1.addCommitment(keccak256(abi.encode("commitment", i)));
        }
        for (uint256 i = 0; i < nullifiers; ++i) {
            protocolAdapterV1.addNullifier(keccak256(abi.encode("nullifier", i)));
        }
    }

    /// @notice Deploys a paused transition proxy bound to the given v1 protocol adapter.
    function _deployTransitionProxy(address protocolAdapterV1)
        internal
        returns (ProtocolAdapterTransition transitionProxy)
    {
        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR(), protocolAdapterV1);
        transitionProxy = ProtocolAdapterTransition(
            Upgrades.deployUUPSProxy(
                "ProtocolAdapterTransition.sol", abi.encodeCall(ProtocolAdapterTransition.initialize, (_OWNER)), opts
            )
        );
    }

    function _seedCommitmentTree() internal {
        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.prank(_OWNER);
        _pa.seedCommitmentTree(sides);
    }

    function _copyInEverything() internal {
        _seedCommitmentTree();
        vm.prank(_OWNER);
        _pa.seedNullifierSet(_NULLIFIER_COUNT);
    }

    function _seedState() internal {
        _copyInEverything();
        vm.prank(_OWNER);
        _pa.unpause();
    }

    /// @dev Overwrites one entry of the historical root set, which the protocol adapter itself cannot do. The set
    /// sits behind the commitment tree in the namespace, and the tree takes the first three slots.
    function _overwriteHistoricalRoot(uint256 index, bytes32 root) internal {
        uint256 valuesSlot = uint256(_COMMITMENT_TREE_STORAGE_SLOT) + 3;
        assertEq(
            uint256(vm.load({target: address(_pa), slot: bytes32(valuesSlot)})),
            _pa.commitmentTreeRootCount(),
            "the historical root set is not at the slot this test writes to"
        );

        vm.store({target: address(_pa), slot: bytes32(uint256(keccak256(abi.encode(valuesSlot))) + index), value: root});
    }
}
