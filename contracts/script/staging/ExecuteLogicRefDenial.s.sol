// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefDenylist} from "../../src/interfaces/ILogicRefDenylist.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteLogicRefDenial
/// @author Anoma Foundation, 2026
/// @notice A script to execute adding logic references to the denylists of a recorded protocol adapter proxy that the
/// staging proxy owner owns. Once a Safe multisig owns the proxy, its owners execute the denial proposed by
/// `production/ProposeLogicRefDenial` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteLogicRefDenial is StagingScript {
    /// @notice Adds the logic references to the denylists of the recorded proxy as the proxy owner. Without
    /// `--broadcast`, the denial is simulated locally.
    /// @param isProduction Whether to update the production or the staging proxy.
    /// @param logicRefs The logic references to deny, each with the denylist to add it to.
    function run(bool isProduction, ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) public {
        address proxy = _requireRecordedProxy(isProduction);
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        ILogicRefDenylist(proxy).denyLogicRefs(logicRefs);
        vm.stopBroadcast();
    }
}
