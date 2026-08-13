// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";

/// @title ProposeKindTableUpdate
/// @author Anoma Foundation, 2025
/// @notice A script to propose updating the kind table commitment of the protocol adapter to the Safe owning the
/// proxy.
/// @custom:security-contact security@anoma.foundation
contract ProposeKindTableUpdate is Script {
    using Safe for *;

    Safe.Client internal _safe;

    /// @notice Thrown if the simulated Safe execution of the update fails during a dry run.
    error KindTableUpdateSimulationFailed();

    /// @notice Proposes updating the kind table commitment of the protocol adapter to the Safe Transaction Service.
    /// Without `--broadcast`, the Safe execution of the update is simulated instead.
    /// @param proxy The protocol adapter proxy to update.
    /// @param safe The Safe owning the proxy.
    /// @param proposer The Safe owner or delegate proposing the transaction. Must be the unlocked script account
    /// (`--account`), which signs the proposal; hardware wallets require the `sign` and
    /// `proposeTransactionWithSignature` flow of `safe-utils`.
    /// @param newKindTableCommitment The new kind table commitment.
    function run(address proxy, address safe, address proposer, bytes32 newKindTableCommitment) public {
        bytes memory updateCall = abi.encodeCall(IProtocolAdapter.setKindTableCommitment, (newKindTableCommitment));

        _safe.initialize(safe);

        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(proxy, updateCall, proposer);
        } else {
            require(_safe.simulateTransactionNoSign(proxy, updateCall, proposer), KindTableUpdateSimulationFailed());
        }
    }
}
