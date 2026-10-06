// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefRegistry} from "../../src/interfaces/ILogicRefRegistry.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeLogicRefStatusUpdate
/// @author Anoma Foundation, 2026
/// @notice A script to propose logic reference status updates to the Safe owning the production environment protocol
/// adapter proxy. The Safe owners confirm and execute the proposed update in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeLogicRefStatusUpdate is ProductionScript {
    /// @notice Proposes the logic reference status updates to the Safe owning the proxy.
    /// @param proxy The production environment protocol adapter proxy to update.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @param updates The logic references and their target statuses, `Deprecated` or `Denied`.
    function run(address proxy, address proposer, ILogicRefRegistry.StatusUpdate[] memory updates) public {
        _propose({
            proxy: proxy, callData: abi.encodeCall(ILogicRefRegistry.setLogicRefStatuses, (updates)), proposer: proposer
        });
    }
}
