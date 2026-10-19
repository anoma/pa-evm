// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {IKindTreeRoot} from "../interfaces/IKindTreeRoot.sol";
import {SHA256} from "../libs/SHA256.sol";

/// @title KindTreeRoot
/// @author Anoma Foundation, 2026
/// @notice The kind tree root being inherited by the protocol adapter. A transaction is proven against the kind
/// table it commits to or against the empty kind table.
/// @custom:security-contact security@anoma.foundation
abstract contract KindTreeRoot is IKindTreeRoot, Initializable {
    /// @custom:storage-location erc7201:anoma.storage.KindTreeRoot
    struct KindTreeRootStorage {
        bytes32 _kindTreeRoot;
    }

    /// @inheritdoc IKindTreeRoot
    /// @dev The Merkle root of a tree without entries is its padding leaf, `SHA256.EMPTY_HASH`.
    bytes32 public constant override EMPTY_KIND_TREE_ROOT = SHA256.EMPTY_HASH;

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.KindTreeRoot")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _KIND_TREE_ROOT_STORAGE_SLOT =
        0x3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00;

    error ZeroKindTreeRootNotAllowed();
    error UnacceptedKindTreeRoot(bytes32 kindTreeRoot);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc IKindTreeRoot
    function setKindTreeRoot(bytes32 newKindTreeRoot) external override {
        _authorizeKindTreeRootChange();
        _setKindTreeRoot(newKindTreeRoot);
    }

    /// @inheritdoc IKindTreeRoot
    function getKindTreeRoot() external view override returns (bytes32 kindTreeRoot) {
        kindTreeRoot = _getKindTreeRoot();
    }

    /// @notice Initializes the kind tree root to the root of the empty kind tree, under which every resource kind is
    /// derived via hash-to-curve.
    // forge-lint: disable-next-line(mixed-case-function)
    function __KindTreeRoot_init() internal onlyInitializing {
        _getKindTreeRootStorage()._kindTreeRoot = EMPTY_KIND_TREE_ROOT;

        emit KindTreeRootUpdated({kindTreeRoot: EMPTY_KIND_TREE_ROOT});
    }

    /// @notice Reverts unless the caller is allowed to set the kind tree root.
    function _authorizeKindTreeRootChange() internal virtual;

    /// @notice Sets the kind tree root and emits the `KindTreeRootUpdated` event.
    /// @param newKindTreeRoot The Merkle root of the new kind table.
    function _setKindTreeRoot(bytes32 newKindTreeRoot) internal {
        require(newKindTreeRoot != bytes32(0), ZeroKindTreeRootNotAllowed());

        _getKindTreeRootStorage()._kindTreeRoot = newKindTreeRoot;

        emit KindTreeRootUpdated({kindTreeRoot: newKindTreeRoot});
    }

    /// @notice Returns the stored kind tree root.
    /// @return kindTreeRoot The Merkle root of the current kind table.
    function _getKindTreeRoot() internal view returns (bytes32 kindTreeRoot) {
        kindTreeRoot = _getKindTreeRootStorage()._kindTreeRoot;
    }

    /// @notice Checks whether a transaction can be proven against a kind table: the stored one or the empty one.
    /// @param kindTreeRoot The kind tree root to check.
    /// @return isAccepted Whether the kind tree root is accepted or not.
    /// @dev The empty kind table merges no kinds, so a transaction that balances with it also balances with the stored
    /// kind table.
    function _isKindTreeRootAccepted(bytes32 kindTreeRoot) internal view returns (bool isAccepted) {
        isAccepted = kindTreeRoot == _getKindTreeRoot() || kindTreeRoot == EMPTY_KIND_TREE_ROOT;
    }

    /// @notice Returns the storage from the kind tree root storage location.
    /// @return kindTreeRootStorage The data associated with the kind tree root storage.
    function _getKindTreeRootStorage() internal pure returns (KindTreeRootStorage storage kindTreeRootStorage) {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            kindTreeRootStorage.slot := _KIND_TREE_ROOT_STORAGE_SLOT
        }
    }
}
