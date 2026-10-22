// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILegacyNullifierSet
/// @author Anoma Foundation, 2026
/// @notice The functions of the nullifier set of the legacy protocol adapter that the state migration reads.
/// @custom:security-contact security@anoma.foundation
interface ILegacyNullifierSet {
    /// @notice Returns the number of nullifiers in the nullifier set.
    /// @return count The number of nullifiers in the set.
    function nullifierCount() external view returns (uint256 count);

    /// @notice Returns the nullifier with the given index.
    /// @param index The index to return the nullifier for.
    /// @return nullifier The nullifier at the given index.
    function nullifierAtIndex(uint256 index) external view returns (bytes32 nullifier);
}
