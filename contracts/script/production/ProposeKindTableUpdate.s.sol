// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeKindTableUpdate
/// @author Anoma Foundation, 2025
/// @notice A script to propose updating the kind table commitment of the production environment protocol adapter
/// proxy to the Safe owning it. The Safe owners confirm and execute the proposed update in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeKindTableUpdate is ProductionScript {
    /// @notice Proposes the kind table update to the Safe owning the proxy. Without `--broadcast`, the Safe
    /// execution of the update is simulated instead of proposed.
    /// @param proxy The production environment protocol adapter proxy to update.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @param newKindTableCommitment The commitment (SHA-256 hash) of the new kind table.
    function run(address proxy, address proposer, bytes32 newKindTableCommitment) public {
        _propose({
            proxy: proxy,
            callData: abi.encodeCall(IProtocolAdapter.setKindTableCommitment, (newKindTableCommitment)),
            proposer: proposer
        });
    }
}
