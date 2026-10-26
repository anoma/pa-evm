// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {CommitmentTreeMock} from "./CommitmentTree.m.sol";
import {LegacyMerkleTree} from "./LegacyMerkleTree.m.sol";

/// @notice The commitment tree as deployed before `push` wrote one side: the same ERC-7201 storage location and the
/// same layout, with the legacy `push`. It stands for the live trees that the new implementation must take over.
contract CommitmentTreeLegacyMock is Initializable, UUPSUpgradeable {
    using LegacyMerkleTree for LegacyMerkleTree.Tree;

    /// @custom:storage-location erc7201:anoma.storage.CommitmentTree
    struct CommitmentTreeStorage {
        LegacyMerkleTree.Tree _merkleTree;
        mapping(bytes32 root => bool isHistorical) _historicalRoots;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.CommitmentTree")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _COMMITMENT_TREE_STORAGE_SLOT =
        0x762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize() external initializer {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();
        $._historicalRoots[$._merkleTree.setup()] = true;
    }

    function addCommitment(bytes32 commitment) external returns (bytes32 newRoot) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();
        (, newRoot) = $._merkleTree.push(commitment);
    }

    function latestCommitmentTreeRoot() external view returns (bytes32 root) {
        root = _getCommitmentTreeStorage()._merkleTree.currentRoot();
    }

    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address newImplementation) internal override {}

    function _getCommitmentTreeStorage() internal pure returns (CommitmentTreeStorage storage commitmentTreeStorage) {
        // forge-lint: disable-next-item(inline-assembly)
        // solhint-disable-next-line no-inline-assembly
        assembly {
            commitmentTreeStorage.slot := _COMMITMENT_TREE_STORAGE_SLOT
        }
    }
}

/// @notice The new commitment tree implementation made upgradeable, to take over the state of the legacy mock.
contract CommitmentTreeUpgradeMock is CommitmentTreeMock, UUPSUpgradeable {
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address newImplementation) internal override {}
}
