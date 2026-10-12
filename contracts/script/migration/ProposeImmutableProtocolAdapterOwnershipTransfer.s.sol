// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Script} from "forge-std-1.17.0/src/Script.sol";
import {IOwnerManager} from "safe-smart-account-1.5.0/contracts/interfaces/IOwnerManager.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {Parameters} from "../Parameters.sol";

/// @title ProposeImmutableProtocolAdapterOwnershipTransfer
/// @author Anoma Foundation, 2026
/// @notice A script to propose the transfer of one chain's legacy immutable protocol adapter to the deployment wallet,
/// to the Safe multisig that owns it. The Safe owners confirm and execute the proposed transfer in the Safe app. The
/// deployment wallet then stops the immutable protocol adapter itself, so the stop and the migration run follow each
/// other without a Safe transaction between them.
/// @dev Besides the `Ownable` functions, the owner of the immutable protocol adapter can only call `emergencyStop`, so
/// the transfer gives the deployment wallet nothing but the stop. `Ownable` transfers in one step, so the receiving
/// address is read from `Parameters` and never taken from a parameter. The stop cannot be undone: the immutable
/// protocol adapter has no function that lifts it.
/// @custom:security-contact security@anoma.foundation
contract ProposeImmutableProtocolAdapterOwnershipTransfer is Script {
    using Safe for *;

    Safe.Client internal _safe;

    /// @notice Thrown if the protocol adapter is not a legacy immutable protocol adapter deployment, i.e. not owned by
    /// the production Safe.
    error NotAnImmutableProtocolAdapterDeployment(address immutableProtocolAdapter);

    /// @notice Thrown if the simulated Safe execution of the transfer fails during a dry run.
    error TransactionSimulationFailed();

    /// @notice Thrown if the simulated transfer left the protocol adapter with another owner.
    error OwnerMismatch(address expected, address actual);

    /// @notice Proposes the transfer to the Safe owning the legacy immutable protocol adapter.
    /// @dev Without `--broadcast`, the Safe execution of the transfer is simulated instead of proposed.
    /// @param immutableProtocolAdapter The immutable protocol adapter to transfer.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    function run(address immutableProtocolAdapter, address proposer) public {
        address safe = Ownable(immutableProtocolAdapter).owner();
        require(safe == Parameters.PA_MULTISIG, NotAnImmutableProtocolAdapterDeployment(immutableProtocolAdapter));

        bytes memory callData = abi.encodeCall(Ownable.transferOwnership, (Parameters.DEPLOYMENT_WALLET));

        _safe.initialize(safe);

        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(immutableProtocolAdapter, callData, proposer);
        } else {
            require(
                _safe.simulateTransactionMultiSigNoSign(
                    immutableProtocolAdapter, callData, IOwnerManager(safe).getOwners()
                ),
                TransactionSimulationFailed()
            );

            address owner = Ownable(immutableProtocolAdapter).owner();
            require(
                owner == Parameters.DEPLOYMENT_WALLET,
                OwnerMismatch({expected: Parameters.DEPLOYMENT_WALLET, actual: owner})
            );
        }
    }
}
