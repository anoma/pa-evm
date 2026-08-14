// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteKindTableUpdate
/// @author Anoma Foundation, 2025
/// @notice A script to execute updating the kind table commitment of the staging environment protocol adapter proxy.
/// Staging only: the production proxy is owned by a Safe multisig, whose owners execute the update proposed by
/// `production/ProposeKindTableUpdate` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteKindTableUpdate is StagingScript {
    /// @notice Updates the kind table commitment as the proxy owner, which the sender must be. Without
    /// `--broadcast`, the update is simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to update.
    /// @param newKindTableCommitment The commitment (SHA-256 hash) of the new kind table.
    function run(address proxy, bytes32 newKindTableCommitment) public {
        _authorizeSender(proxy);

        vm.startBroadcast();
        IProtocolAdapter(proxy).setKindTableCommitment(newKindTableCommitment);
        vm.stopBroadcast();
    }
}
