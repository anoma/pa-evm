// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IKindTableRoot} from "../../src/interfaces/IKindTableRoot.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteKindTableUpdate
/// @author Anoma Foundation, 2026
/// @notice A script to execute updating the kind table root of the staging environment protocol adapter proxy.
/// Staging only: the production proxy is owned by a Safe multisig, whose owners execute the update proposed by
/// `production/ProposeKindTableUpdate` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteKindTableUpdate is StagingScript {
    /// @notice Updates the kind table root as the proxy owner. Without `--broadcast`, the update is simulated
    /// locally.
    /// @param proxy The staging environment protocol adapter proxy to update.
    /// @param newKindTableRoot The Merkle root of the new kind table.
    function run(address proxy, bytes32 newKindTableRoot) public {
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        IKindTableRoot(proxy).setKindTableRoot(newKindTableRoot);
        vm.stopBroadcast();
    }
}
