// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteProtocolAdapterPause
/// @author Anoma Foundation, 2026
/// @notice A script to pause the staging environment protocol adapter proxy, so that it executes no transaction until
/// the owner calls `unpause`. Staging only: the production proxy is owned by a Safe multisig, whose owners execute the
/// pause proposed by `production/ProposeProtocolAdapterPause` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteProtocolAdapterPause is StagingScript {
    /// @notice Pauses the protocol adapter as the proxy owner, which the sender must be. Without `--broadcast`, the
    /// pause is simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to pause.
    function run(address proxy) public {
        _checkSenderAuthorization({proxy: proxy});

        vm.startBroadcast();
        IProtocolAdapter(proxy).pause();
        vm.stopBroadcast();
    }
}
