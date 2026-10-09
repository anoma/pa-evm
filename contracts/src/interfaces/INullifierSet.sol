// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title INullifierSet
/// @author Anoma Foundation, 2025
/// @notice The interface of the nullifier set contract.
/// @custom:security-contact security@anoma.foundation
interface INullifierSet {
    /// @notice Returns whether the set contains a given nullifier or not.
    /// @param nullifier The nullifier to check.
    /// @return isContained Whether the nullifier is contained or not.
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
}
