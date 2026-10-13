// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {IKindTableRoot} from "../interfaces/IKindTableRoot.sol";
import {SHA256} from "../libs/SHA256.sol";

/// @title KindTableRoot
/// @author Anoma Foundation, 2026
/// @notice The kind table root being inherited by the protocol adapter. A transaction is proven against the kind
/// table it commits to or against the empty kind table.
/// @custom:security-contact security@anoma.foundation
abstract contract KindTableRoot is IKindTableRoot, Initializable {
    /// @custom:storage-location erc7201:anoma.storage.KindTableRoot
    struct KindTableRootStorage {
        bytes32 _kindTableRoot;
    }

    /// @inheritdoc IKindTableRoot
    /// @dev The Merkle root of a tree without entries is its padding leaf, `SHA256.EMPTY_HASH`.
    bytes32 public constant override EMPTY_KIND_TABLE_ROOT = SHA256.EMPTY_HASH;

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.KindTableRoot")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _KIND_TABLE_ROOT_STORAGE_SLOT =
        0x23b07f485020dfa88e29e80272974fc9e994c393dc78b59617b32884da393100;

    error ZeroKindTableRootNotAllowed();
    error UnacceptedKindTableRoot(bytes32 kindTableRoot);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc IKindTableRoot
    function setKindTableRoot(bytes32 newKindTableRoot) external override {
        _authorizeKindTableRootChange();
        _setKindTableRoot(newKindTableRoot);
    }

    /// @inheritdoc IKindTableRoot
    function getKindTableRoot() external view override returns (bytes32 kindTableRoot) {
        kindTableRoot = _getKindTableRoot();
    }

    /// @notice Initializes the kind table root to the empty kind table, under which every resource kind is
    /// derived via hash-to-curve.
    // forge-lint: disable-next-line(mixed-case-function)
    function __KindTableRoot_init() internal onlyInitializing {
        _getKindTableRootStorage()._kindTableRoot = EMPTY_KIND_TABLE_ROOT;

        emit KindTableRootUpdated({kindTableRoot: EMPTY_KIND_TABLE_ROOT});
    }

    /// @notice Reverts unless the caller is allowed to set the kind table root.
    function _authorizeKindTableRootChange() internal virtual;

    /// @notice Sets the kind table root and emits the `KindTableRootUpdated` event.
    /// @param newKindTableRoot The Merkle root of the new kind table.
    function _setKindTableRoot(bytes32 newKindTableRoot) internal {
        require(newKindTableRoot != bytes32(0), ZeroKindTableRootNotAllowed());

        _getKindTableRootStorage()._kindTableRoot = newKindTableRoot;

        emit KindTableRootUpdated({kindTableRoot: newKindTableRoot});
    }

    /// @notice Returns the stored kind table root.
    /// @return kindTableRoot The Merkle root of the current kind table.
    function _getKindTableRoot() internal view returns (bytes32 kindTableRoot) {
        kindTableRoot = _getKindTableRootStorage()._kindTableRoot;
    }

    /// @notice Checks whether a transaction can be proven against a kind table: the stored one or the empty one.
    /// @param kindTableRoot The kind table root to check.
    /// @return isAccepted Whether the kind table root is accepted or not.
    /// @dev The empty kind table merges no kinds, so a transaction that balances with it also balances with the stored
    /// kind table.
    function _isKindTableRootAccepted(bytes32 kindTableRoot) internal view returns (bool isAccepted) {
        isAccepted = kindTableRoot == _getKindTableRoot() || kindTableRoot == EMPTY_KIND_TABLE_ROOT;
    }

    /// @notice Returns the storage from the kind table root storage location.
    /// @return kindTableRootStorage The data associated with the kind table root storage.
    function _getKindTableRootStorage() internal pure returns (KindTableRootStorage storage kindTableRootStorage) {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            kindTableRootStorage.slot := _KIND_TABLE_ROOT_STORAGE_SLOT
        }
    }
}
