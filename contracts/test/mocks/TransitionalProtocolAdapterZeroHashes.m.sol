// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";

/// @notice A mock transitional protocol adapter that exposes `_zeroHashes` to the tests.
contract TransitionalProtocolAdapterZeroHashesMock is TransitionalProtocolAdapter {
    /// @custom:oz-upgrades-unsafe-allow constructor state-variable-immutable
    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector, address protocolAdapterV1)
        TransitionalProtocolAdapter(riscZeroVerifierRouter, riscZeroVerifierSelector, protocolAdapterV1)
    {}

    /// @notice Returns the empty-subtree roots the transitional protocol adapter writes for a tree of the given depth.
    function zeroHashes(uint8 treeDepth) external pure returns (bytes32[] memory hashes) {
        hashes = _zeroHashes(treeDepth);
    }
}
