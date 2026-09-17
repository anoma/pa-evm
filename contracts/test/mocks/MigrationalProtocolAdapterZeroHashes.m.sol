// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";

/// @notice A mock migrational protocol adapter that exposes `_zeroHashes` to the tests.
contract MigrationalProtocolAdapterZeroHashesMock is MigrationalProtocolAdapter {
    /// @custom:oz-upgrades-unsafe-allow constructor state-variable-immutable
    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector, address protocolAdapterV1)
        MigrationalProtocolAdapter(riscZeroVerifierRouter, riscZeroVerifierSelector, protocolAdapterV1)
    {}

    /// @notice Returns the empty-subtree roots the migrational protocol adapter writes for a tree of the given depth.
    function zeroHashes(uint8 treeDepth) external pure returns (bytes32[] memory hashes) {
        hashes = _zeroHashes(treeDepth);
    }
}
