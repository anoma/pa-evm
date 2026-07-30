// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.6.1/proxy/utils/Initializable.sol";

import {EnumerableSet} from "@openzeppelin-contracts-5.6.1/utils/structs/EnumerableSet.sol";

import {INullifierSet} from "../interfaces/INullifierSet.sol";

/// @title NullifierSet
/// @author Anoma Foundation, 2025
/// @notice A nullifier set being inherited by the protocol adapter.
/// @dev The implementation is based on OpenZeppelin's `EnumerableSet` implementation.
/// @custom:security-contact security@anoma.foundation
abstract contract NullifierSet is INullifierSet, Initializable {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @custom:storage-location erc7201:anoma.storage.NullifierSet
    struct NullifierSetStorage {
        EnumerableSet.Bytes32Set _nullifierSet;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.NullifierSet")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _NULLIFIER_SET_STORAGE_SLOT =
        0x5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00;

    error PreExistingNullifier(bytes32 nullifier);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc INullifierSet
    function isNullifierContained(bytes32 nullifier) external view override returns (bool isContained) {
        NullifierSetStorage storage $ = _getNullifierSetStorage();

        isContained = $._nullifierSet.contains(nullifier);
    }

    /// @inheritdoc INullifierSet
    function nullifierCount() external view override returns (uint256 count) {
        NullifierSetStorage storage $ = _getNullifierSetStorage();

        count = $._nullifierSet.length();
    }

    /// @inheritdoc INullifierSet
    function nullifierAtIndex(uint256 index) external view override returns (bytes32 nullifier) {
        NullifierSetStorage storage $ = _getNullifierSetStorage();

        nullifier = $._nullifierSet.at(index);
    }

    /// @notice Initializes the NullifierSet contract.
    /// @dev The nullifier set requires no setup. The function exists for consistency with the OpenZeppelin
    /// initializer convention.
    // solhint-disable-next-line func-name-mixedcase, no-empty-blocks
    function __NullifierSet_init() internal onlyInitializing {}

    /// @notice Adds a nullifier to the set, if it does not exist already.
    /// @param nullifier The nullifier to add.
    function _addNullifier(bytes32 nullifier) internal {
        NullifierSetStorage storage $ = _getNullifierSetStorage();

        bool success = $._nullifierSet.add(nullifier);
        require(success, PreExistingNullifier(nullifier));
    }

    /// @notice Returns the storage from the nullifier set storage location.
    /// @return nullifierSetStorage The data associated with the nullifier set storage.
    function _getNullifierSetStorage() internal pure returns (NullifierSetStorage storage nullifierSetStorage) {
        /* solhint-disable no-inline-assembly */

        // slither-disable-next-line assembly
        assembly {
            nullifierSetStorage.slot := _NULLIFIER_SET_STORAGE_SLOT
        }

        /* solhint-enable no-inline-assembly */
    }
}
