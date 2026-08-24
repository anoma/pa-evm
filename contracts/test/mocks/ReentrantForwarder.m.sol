// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IForwarder} from "anoma-forwarder-bases-2.0.0/src/interfaces/IForwarder.sol";

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";

/// @notice A malicious forwarder that reenters `ProtocolAdapter.execute` from within a forwarder call.
contract ReentrantForwarderMock is IForwarder {
    IProtocolAdapter internal immutable _PROTOCOL_ADAPTER;

    constructor(IProtocolAdapter protocolAdapter) {
        _PROTOCOL_ADAPTER = protocolAdapter;
    }

    /// @inheritdoc IForwarder
    /// @dev The reentrant `execute` call reverts before any argument is inspected, so the empty transaction suffices.
    function forwardCall(
        bytes32,
        /* logicRef */
        bytes memory /* input */
    )
        external
        returns (bytes memory output)
    {
        _PROTOCOL_ADAPTER.execute(
            IProtocolAdapter.Transaction({
                actions: new IProtocolAdapter.Action[](0), deltaProof: "", aggregationProof: ""
            })
        );
        output = "";
    }
}
