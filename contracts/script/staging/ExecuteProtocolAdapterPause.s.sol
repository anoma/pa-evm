// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteProtocolAdapterPause
/// @author Anoma Foundation, 2026
/// @notice A script to pause a recorded protocol adapter proxy that the staging proxy owner owns, so that it executes
/// no transaction until the owner calls `unpause`. Once a Safe multisig owns the proxy, its owners execute the pause
/// proposed by `production/ProposeProtocolAdapterPause` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteProtocolAdapterPause is StagingScript {
    /// @notice Pauses the recorded proxy as the proxy owner. Without `--broadcast`, the pause is simulated locally.
    /// @param isProduction Whether to pause the production or the staging proxy.
    function run(bool isProduction) public {
        address proxy = _requireRecordedProxy(isProduction);
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        IProtocolAdapter(proxy).pause();
        vm.stopBroadcast();
    }
}
