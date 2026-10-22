// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IKindTreeRoot} from "../../src/interfaces/IKindTreeRoot.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteKindTableUpdate
/// @author Anoma Foundation, 2026
/// @notice A script to execute updating the kind tree root of a recorded protocol adapter proxy that the staging proxy
/// owner owns. Once a Safe multisig owns the proxy, its owners execute the update proposed by
/// `production/ProposeKindTableUpdate` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteKindTableUpdate is StagingScript {
    /// @notice Updates the kind tree root of the recorded proxy as the proxy owner. Without `--broadcast`, the update is
    /// simulated locally.
    /// @param isProduction Whether to update the production or the staging proxy.
    /// @param newKindTreeRoot The Merkle root of the new kind table.
    function run(bool isProduction, bytes32 newKindTreeRoot) public {
        address proxy = _requireRecordedProxy(isProduction);
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        IKindTreeRoot(proxy).setKindTreeRoot(newKindTreeRoot);
        vm.stopBroadcast();
    }
}
