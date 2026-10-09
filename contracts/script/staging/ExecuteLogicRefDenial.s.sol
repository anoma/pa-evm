// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefDenylist} from "../../src/interfaces/ILogicRefDenylist.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteLogicRefDenial
/// @author Anoma Foundation, 2026
/// @notice A script to execute adding logic references to the denylists of the staging environment protocol adapter
/// proxy. Staging only: the production proxy is owned by a Safe multisig, whose owners execute the denial proposed by
/// `production/ProposeLogicRefDenial` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteLogicRefDenial is StagingScript {
    /// @notice Adds the logic references to the denylists as the proxy owner. Without `--broadcast`, the denial is
    /// simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to update.
    /// @param logicRefs The logic references to deny, each with the denylist to add it to.
    function run(address proxy, ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) public {
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        ILogicRefDenylist(proxy).denyLogicRefs(logicRefs);
        vm.stopBroadcast();
    }
}
