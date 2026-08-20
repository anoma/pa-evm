// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

/// @title PrintProtocolAdapterVersion
/// @author Anoma Foundation, 2025
/// @notice A script returning the version the protocol adapter source compiles to. The release flow reads it to label
/// the published package, so that the label always describes the source it ships.
/// @custom:security-contact security@anoma.foundation
contract PrintProtocolAdapterVersion is Script {
    /// @notice A router address satisfying the constructor, which rejects the zero address. The instance is never
    /// broadcast and only answers a compile-time constant, so the value is arbitrary.
    address internal constant _UNUSED_ROUTER = address(1);

    /// @notice A verifier selector satisfying the constructor, which rejects the zero selector. Arbitrary for the same
    /// reason as the router.
    bytes4 internal constant _UNUSED_SELECTOR = bytes4(uint32(1));

    /// @notice Returns the version of the protocol adapter this source compiles to.
    /// @return version The protocol adapter version.
    function run() public returns (string memory version) {
        version = new ProtocolAdapter({
            riscZeroVerifierRouter: _UNUSED_ROUTER, riscZeroVerifierSelector: _UNUSED_SELECTOR
        }).VERSION();
    }
}
