// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";
import {IOwnerManager} from "safe-smart-account-1.5.0/contracts/interfaces/IOwnerManager.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {Parameters} from "../Parameters.sol";

/// @title ProposeProtocolAdapterV1OwnershipTransfer
/// @author Anoma Foundation, 2026
/// @notice A script to propose the transfer of one chain's v1 protocol adapter to the deployment wallet, to the Safe
/// multisig that owns it. The Safe owners confirm and execute the proposed transfer in the Safe app. The deployment
/// wallet then stops v1 itself, so the stop and the migration run follow each other without a Safe transaction
/// between them.
/// @dev Besides the `Ownable` functions, the owner of v1 can only call `emergencyStop`, so the transfer gives the
/// deployment wallet nothing but the stop. `Ownable` transfers in one step, so the receiving address is read from
/// `Parameters` and never taken from a parameter. The stop cannot be undone: v1 has no function that lifts it.
/// @custom:security-contact security@anoma.foundation
contract ProposeProtocolAdapterV1OwnershipTransfer is Script {
    using Safe for *;

    Safe.Client internal _safe;

    /// @notice Thrown if the protocol adapter is not a v1 deployment, i.e. not owned by the production Safe.
    error NotAV1Deployment(address protocolAdapterV1);

    /// @notice Thrown if the simulated Safe execution of the transfer fails during a dry run.
    error TransactionSimulationFailed();

    /// @notice Thrown if the simulated transfer left the protocol adapter with another owner.
    error OwnerMismatch(address expected, address actual);

    /// @notice Proposes the transfer to the Safe owning the v1 protocol adapter.
    /// @dev Without `--broadcast`, the Safe execution of the transfer is simulated instead of proposed.
    /// @param protocolAdapterV1 The v1 protocol adapter to transfer.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    function run(address protocolAdapterV1, address proposer) public {
        address safe = Ownable(protocolAdapterV1).owner();
        require(safe == Parameters.PA_MULTISIG, NotAV1Deployment(protocolAdapterV1));

        bytes memory callData = abi.encodeCall(Ownable.transferOwnership, (Parameters.DEPLOYMENT_WALLET));

        _safe.initialize(safe);

        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(protocolAdapterV1, callData, proposer);
        } else {
            require(
                _safe.simulateTransactionMultiSigNoSign(protocolAdapterV1, callData, IOwnerManager(safe).getOwners()),
                TransactionSimulationFailed()
            );

            address owner = Ownable(protocolAdapterV1).owner();
            require(
                owner == Parameters.DEPLOYMENT_WALLET,
                OwnerMismatch({expected: Parameters.DEPLOYMENT_WALLET, actual: owner})
            );
        }
    }
}
