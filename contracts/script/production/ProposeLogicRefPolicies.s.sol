// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefPolicyRegistry} from "../../src/interfaces/ILogicRefPolicyRegistry.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeLogicRefPolicies
/// @author Anoma Foundation, 2026
/// @notice A script to propose restricting logic reference policies of the production environment protocol adapter
/// proxy to the Safe owning it. The Safe owners confirm and execute the proposed denial in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeLogicRefPolicies is ProductionScript {
    /// @notice Proposes the logic reference denial to the Safe owning the proxy.
    /// @param proxy The production environment protocol adapter proxy to update.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @param logicRefs The logic references and their target policies.
    function run(address proxy, address proposer, ILogicRefPolicyRegistry.PolicyUpdate[] memory logicRefs) public {
        _propose({
            proxy: proxy,
            callData: abi.encodeCall(ILogicRefPolicyRegistry.setLogicRefPolicies, (logicRefs)),
            proposer: proposer
        });
    }
}
