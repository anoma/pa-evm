// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";
import {IOwnerManager} from "safe-smart-account-1.5.0/contracts/interfaces/IOwnerManager.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {IProtocolAdapterV1} from "../../src/interfaces/IProtocolAdapterV1.sol";

/// @title ProposeProtocolAdapterV1Stop
/// @author Anoma Foundation, 2026
/// @notice A script to propose stopping one chain's v1 protocol adapter to the Safe multisig that owns it. The Safe
/// owners confirm and execute the proposed stop in the Safe app. The stop cannot be undone: v1 has no function that
/// lifts it.
/// @dev The same Safe owns v1 on every chain, so no account can stop v1 directly.
/// @custom:security-contact security@anoma.foundation
contract ProposeProtocolAdapterV1Stop is Script {
    using Safe for *;

    /// @notice The Safe multisig that owns the v1 protocol adapter on every chain.
    address public constant PROTOCOL_ADAPTER_V1_OWNER = 0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10;

    Safe.Client internal _safe;

    /// @notice Thrown if the protocol adapter is not a v1 deployment, i.e. not owned by `PROTOCOL_ADAPTER_V1_OWNER`.
    error NotAV1Deployment(address protocolAdapterV1);

    /// @notice Thrown if the simulated Safe execution of the stop fails during a dry run.
    error TransactionSimulationFailed();

    /// @notice Proposes the stop to the Safe owning the v1 protocol adapter.
    /// @dev Without `--broadcast`, the Safe execution of the stop is simulated instead of proposed.
    /// @param protocolAdapterV1 The v1 protocol adapter to stop.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    function run(address protocolAdapterV1, address proposer) public {
        address safe = Ownable(protocolAdapterV1).owner();
        require(safe == PROTOCOL_ADAPTER_V1_OWNER, NotAV1Deployment(protocolAdapterV1));

        _safe.initialize(safe);

        bytes memory callData = abi.encodeCall(IProtocolAdapterV1.emergencyStop, ());
        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(protocolAdapterV1, callData, proposer);
        } else {
            require(
                _safe.simulateTransactionMultiSigNoSign(protocolAdapterV1, callData, IOwnerManager(safe).getOwners()),
                TransactionSimulationFailed()
            );
        }
    }
}
