// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {IKindTableCommitment} from "../interfaces/IKindTableCommitment.sol";

/// @title KindTableCommitment
/// @author Anoma Foundation, 2026
/// @notice The kind table commitment being inherited by the protocol adapter. A transaction is proven against the kind
/// table it commits to or against the empty kind table.
/// @custom:security-contact security@anoma.foundation
abstract contract KindTableCommitment is IKindTableCommitment, Initializable {
    /// @custom:storage-location erc7201:anoma.storage.KindTableCommitment
    struct KindTableCommitmentStorage {
        bytes32 _kindTableCommitment;
    }

    /// @inheritdoc IKindTableCommitment
    /// @dev Note that this is not `SHA256.EMPTY_HASH`.
    bytes32 public constant override EMPTY_KIND_TABLE_COMMITMENT =
        0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855;

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.KindTableCommitment")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _KIND_TABLE_COMMITMENT_STORAGE_SLOT =
        0x54765cac2cb330b12e843854496f3e05fcf80a7a08eec8213621776191851900;

    error ZeroKindTableCommitmentNotAllowed();
    error UnacceptedKindTableCommitment(bytes32 kindTableCommitment);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc IKindTableCommitment
    function getKindTableCommitment() external view override returns (bytes32 kindTableCommitment) {
        kindTableCommitment = _getKindTableCommitment();
    }

    /// @notice Initializes the kind table commitment to the empty kind table, under which every resource kind is
    /// derived via hash-to-curve.
    // forge-lint: disable-next-item(mixed-case-function)
    // solhint-disable-next-line func-name-mixedcase
    function __KindTableCommitment_init() internal onlyInitializing {
        _getKindTableCommitmentStorage()._kindTableCommitment = EMPTY_KIND_TABLE_COMMITMENT;

        emit KindTableCommitmentUpdated({kindTableCommitment: EMPTY_KIND_TABLE_COMMITMENT});
    }

    /// @notice Sets the kind table commitment and emits the `KindTableCommitmentUpdated` event.
    /// @param newKindTableCommitment The commitment (SHA-256 hash) of the new kind table.
    function _setKindTableCommitment(bytes32 newKindTableCommitment) internal {
        require(newKindTableCommitment != bytes32(0), ZeroKindTableCommitmentNotAllowed());

        _getKindTableCommitmentStorage()._kindTableCommitment = newKindTableCommitment;

        emit KindTableCommitmentUpdated({kindTableCommitment: newKindTableCommitment});
    }

    /// @notice Returns the stored kind table commitment.
    /// @return kindTableCommitment The commitment (SHA-256 hash) of the current kind table.
    function _getKindTableCommitment() internal view returns (bytes32 kindTableCommitment) {
        kindTableCommitment = _getKindTableCommitmentStorage()._kindTableCommitment;
    }

    /// @notice Checks whether a transaction can be proven against a kind table: the stored one or the empty one.
    /// @param kindTableCommitment The kind table commitment to check.
    /// @return isAccepted Whether the kind table commitment is accepted or not.
    /// @dev The empty kind table merges no kinds, so a transaction that balances with it also balances with the stored
    /// kind table.
    function _isKindTableCommitmentAccepted(bytes32 kindTableCommitment) internal view returns (bool isAccepted) {
        isAccepted =
            kindTableCommitment == _getKindTableCommitment() || kindTableCommitment == EMPTY_KIND_TABLE_COMMITMENT;
    }

    /// @notice Returns the storage from the kind table commitment storage location.
    /// @return kindTableCommitmentStorage The data associated with the kind table commitment storage.
    function _getKindTableCommitmentStorage()
        internal
        pure
        returns (KindTableCommitmentStorage storage kindTableCommitmentStorage)
    {
        /* solhint-disable no-inline-assembly */

        // forge-lint: disable-next-item(inline-assembly)
        // slither-disable-next-line assembly
        assembly {
            kindTableCommitmentStorage.slot := _KIND_TABLE_COMMITMENT_STORAGE_SLOT
        }

        /* solhint-enable no-inline-assembly */
    }
}
