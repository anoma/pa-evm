// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IKindTreeRoot} from "../../src/interfaces/IKindTreeRoot.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeKindTableUpdate
/// @author Anoma Foundation, 2026
/// @notice A script to propose updating the kind tree root of the production environment protocol adapter
/// proxy to the Safe owning it. The Safe owners confirm and execute the proposed update in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeKindTableUpdate is ProductionScript {
    /// @notice Proposes the kind table update to the Safe owning the proxy.
    /// @param proxy The production environment protocol adapter proxy to update.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @param newKindTreeRoot The Merkle root of the new kind table.
    function run(address proxy, address proposer, bytes32 newKindTreeRoot) public {
        _propose({
            proxy: proxy, callData: abi.encodeCall(IKindTreeRoot.setKindTreeRoot, (newKindTreeRoot)), proposer: proposer
        });
    }
}
