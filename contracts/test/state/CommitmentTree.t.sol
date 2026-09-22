// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";

import {ICommitmentTree} from "../../src/interfaces/ICommitmentTree.sol";
import {SHA256} from "../../src/libs/SHA256.sol";
import {CommitmentTree} from "../../src/state/CommitmentTree.sol";
import {MerkleTreeExample} from "../examples/MerkleTree.e.sol";
import {CommitmentTreeMock} from "../mocks/CommitmentTree.m.sol";

contract CommitmentTreeTest is Test, MerkleTreeExample {
    CommitmentTreeMock internal _cmAcc;

    constructor() {
        _setupMockTree();
        _cmAcc = _deployCommitmentTreeMock();
    }

    function test_addCommitment_returns_correct_roots() public {
        bytes32 initialRoot = _cmAcc.latestCommitmentTreeRoot();

        assertEq(initialRoot, _roots[0], "initial root should match expected root");
        assertEq(initialRoot, _cmAcc.initialRoot(), "initial root should match initialRoot()");

        for (uint256 i = 0; i < _N_LEAVES; ++i) {
            assertEq(
                _cmAcc.addCommitment(_leaves[i + 1][i]), _roots[i + 1], "root should match after adding commitment"
            );
        }
    }

    function test_addCommitment_should_add_commitments() public {
        uint256 prevCount = 0;
        uint256 newCount = 0;

        for (uint256 i = 0; i < _N_LEAVES; ++i) {
            _cmAcc.addCommitment(_leaves[i + 1][i]);
            newCount = _cmAcc.commitmentCount();

            assertEq(newCount, ++prevCount, "commitment count should increment by 1");
            prevCount = newCount;
        }
    }

    function test_addCommitmentTreeRoot_reverts_on_pre_existing_root() public {
        bytes32 preExistingRoot = bytes32(type(uint256).max);
        _cmAcc.addCommitmentTreeRoot(preExistingRoot);

        vm.expectRevert(
            abi.encodeWithSelector(CommitmentTree.PreExistingRoot.selector, preExistingRoot), address(_cmAcc)
        );
        _cmAcc.addCommitmentTreeRoot(preExistingRoot);
    }

    function test_addCommitmentTreeRoot_stores_the_root() public {
        bytes32 rootToStore = bytes32(type(uint256).max);

        assertEq(
            _cmAcc.latestCommitmentTreeRoot(), _cmAcc.initialRoot(), "latest root should be initial root before store"
        );
        assertEq(_cmAcc.isCommitmentTreeRootContained(rootToStore), false, "root should not be contained before store");

        _cmAcc.addCommitmentTreeRoot(rootToStore);

        assertEq(_cmAcc.latestCommitmentTreeRoot(), rootToStore, "latest root should be the stored root");
        assertEq(_cmAcc.isCommitmentTreeRootContained(rootToStore), true, "root should be contained after store");
    }

    function test_addCommitmentTreeRoot_emits_the_CommitmentTreeRootAdded_event_on_store_() public {
        bytes32 rootToStore = bytes32(type(uint256).max);

        vm.expectEmit(address(_cmAcc));
        emit ICommitmentTree.CommitmentTreeRootAdded({root: rootToStore});

        _cmAcc.addCommitmentTreeRoot(rootToStore);
    }

    function test_commitmentTreeRootAtIndex_returns_the_right_index() public {
        for (uint256 i = 0; i < _N_LEAVES; ++i) {
            _cmAcc.addCommitmentTreeRoot(_cmAcc.addCommitment(_leaves[i + 1][i]));
        }

        for (uint256 i = 0; i < _N_LEAVES; ++i) {
            assertEq(_cmAcc.commitmentTreeRootAtIndex(i), _roots[i], "The returned root should have the expected index");
        }
    }

    function test_addCommitment_allows_adding_the_same_commitment_multiple_times() public {
        // Note: The compliance circuit will prevent the same commitment being added a second time.
        bytes32 cm = sha256("SOMETHING");

        _cmAcc.addCommitment(cm);
        _cmAcc.addCommitment(cm);
    }

    function test_commitmentTreeZeros_returns_the_root_of_an_empty_subtree_per_level() public {
        for (uint256 i = 0; i < _N_LEAVES; ++i) {
            _cmAcc.addCommitment(_leaves[i + 1][i]);
        }

        bytes32[] memory zeros = _cmAcc.commitmentTreeZeros();

        assertEq(zeros.length, _cmAcc.commitmentTreeDepth() + 1, "the zeros should cover every level up to the root");
        assertEq(zeros[0], SHA256.EMPTY_HASH, "the zero of the leaf level should be the empty hash");
        for (uint256 level = 1; level < zeros.length; ++level) {
            assertEq(zeros[level], SHA256.hash(zeros[level - 1], zeros[level - 1]), "a zero should pair the one below");
        }
    }

    function test_commitmentTreeSides_and_commitmentTreeZeros_reproduce_the_root_after_every_push() public {
        _assertTheSidesAndZerosReproduce(_cmAcc.latestCommitmentTreeRoot());

        for (uint256 i = 0; i < 33; ++i) {
            _assertTheSidesAndZerosReproduce(_cmAcc.addCommitment(keccak256(abi.encode(i))));
        }
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy, initialized through the proxy constructor because the
    /// implementation contract disables the initializers.
    function _deployCommitmentTreeMock() internal returns (CommitmentTreeMock mock) {
        mock = CommitmentTreeMock(
            address(
                new ERC1967Proxy(address(new CommitmentTreeMock()), abi.encodeCall(CommitmentTreeMock.initialize, ()))
            )
        );
    }

    /// @dev Rebuilds the root the way `MerkleTree.currentRoot` does, from the commitment count, the sides and the zeros.
    function _assertTheSidesAndZerosReproduce(bytes32 root) internal view {
        uint256 count = _cmAcc.commitmentCount();
        bytes32[] memory sides = _cmAcc.commitmentTreeSides();
        bytes32[] memory zeros = _cmAcc.commitmentTreeZeros();

        assertEq(sides.length, _cmAcc.commitmentTreeDepth(), "the sides should cover every level below the root");

        bytes32 rebuilt = zeros[0];
        for (uint256 level = 0; level < sides.length; ++level) {
            rebuilt =
                ((count >> level) & 1) == 0 ? SHA256.hash(rebuilt, zeros[level]) : SHA256.hash(sides[level], rebuilt);
        }
        assertEq(rebuilt, root, "the sides and zeros should reproduce the root");
    }
}
