// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IProtocolAdapterV1
/// @author Anoma Foundation, 2026
/// @notice The part of the v1 protocol adapter interface that the migration calls and that no v2 interface carries.
/// @custom:security-contact security@anoma.foundation
interface IProtocolAdapterV1 {
    /// @notice Stops the protocol adapter permanently: it executes no transaction afterwards, and no function lifts
    /// the stop. Only the owner can call it, and only once.
    function emergencyStop() external;
}
