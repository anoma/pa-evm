// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefDenylist} from "../../src/interfaces/ILogicRefDenylist.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeLogicRefDenial
/// @author Anoma Foundation, 2026
/// @notice A script to propose adding logic references to the denylists of the production environment protocol adapter
/// proxy to the Safe owning it. The Safe owners confirm and execute the proposed denial in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeLogicRefDenial is ProductionScript {
    /// @notice Proposes the logic reference denial of the recorded production proxy to the Safe owning the proxy.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @param logicRefs The logic references to deny, each with the denylist to add it to.
    function run(address proposer, ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) public {
        _propose({callData: abi.encodeCall(ILogicRefDenylist.denyLogicRefs, (logicRefs)), proposer: proposer});
    }
}
