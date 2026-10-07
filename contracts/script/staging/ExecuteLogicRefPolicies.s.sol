// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefPolicyRegistry} from "../../src/interfaces/ILogicRefPolicyRegistry.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteLogicRefPolicies
/// @author Anoma Foundation, 2026
/// @notice A script to execute restricting logic reference policies of the staging environment protocol adapter
/// proxy. Staging only: the production proxy is owned by a Safe multisig, whose owners execute the denial proposed by
/// `production/ProposeLogicRefPolicies` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteLogicRefPolicies is StagingScript {
    /// @notice Sets the logic reference policies as the proxy owner. Without `--broadcast`, the denial is
    /// simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to update.
    /// @param logicRefs The logic references and their target policies.
    function run(address proxy, ILogicRefPolicyRegistry.PolicyUpdate[] memory logicRefs) public {
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        ILogicRefPolicyRegistry(proxy).setLogicRefPolicies(logicRefs);
        vm.stopBroadcast();
    }
}
