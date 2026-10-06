// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ILogicRefRegistry} from "../../src/interfaces/ILogicRefRegistry.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteLogicRefStatusUpdate
/// @author Anoma Foundation, 2026
/// @notice A script to update logic reference statuses on the staging environment protocol adapter proxy.
/// Staging only: the production proxy is owned by a Safe multisig, whose owners execute the update proposed by
/// `production/ProposeLogicRefStatusUpdate` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteLogicRefStatusUpdate is StagingScript {
    /// @notice Updates the logic reference statuses as the proxy owner. Without `--broadcast`, the update is
    /// simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to update.
    /// @param updates The logic references and their target statuses, `Deprecated` or `Denied`.
    function run(address proxy, ILogicRefRegistry.StatusUpdate[] memory updates) public {
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        ILogicRefRegistry(proxy).setLogicRefStatuses(updates);
        vm.stopBroadcast();
    }
}
