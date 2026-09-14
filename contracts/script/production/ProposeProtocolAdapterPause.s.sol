// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeProtocolAdapterPause
/// @author Anoma Foundation, 2026
/// @notice A script to propose pausing the production environment protocol adapter proxy to the Safe owning it. The
/// Safe owners confirm and execute the proposed pause in the Safe app. The protocol adapter then executes no
/// transaction until the Safe calls `unpause`.
/// @custom:security-contact security@anoma.foundation
contract ProposeProtocolAdapterPause is ProductionScript {
    /// @notice Proposes the pause to the Safe owning the proxy.
    /// @param proxy The production environment protocol adapter proxy to pause.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    function run(address proxy, address proposer) public {
        _propose({proxy: proxy, callData: abi.encodeCall(IProtocolAdapter.pause, ()), proposer: proposer});
    }
}
