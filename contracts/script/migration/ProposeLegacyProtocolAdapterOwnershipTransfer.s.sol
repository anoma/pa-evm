// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Script} from "forge-std-1.17.0/src/Script.sol";
import {IOwnerManager} from "safe-smart-account-1.5.0/contracts/interfaces/IOwnerManager.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {Parameters} from "../Parameters.sol";

/// @title ProposeLegacyProtocolAdapterOwnershipTransfer
/// @author Anoma Foundation, 2026
/// @notice A script to propose the transfer of one chain's legacy protocol adapter to the deployment wallet,
/// to the Safe multisig that owns it. The Safe owners confirm and execute the proposed transfer in the Safe app. The
/// deployment wallet then stops the legacy protocol adapter itself, so the stop and the migration run follow each
/// other without a Safe transaction between them.
/// @dev Besides the `Ownable` functions, the owner of the legacy protocol adapter can only call `emergencyStop`, so
/// the transfer gives the deployment wallet nothing but the stop. `Ownable` transfers in one step, so the receiving
/// address is read from `Parameters` and never taken from a parameter. The stop cannot be undone: the legacy
/// protocol adapter has no function that lifts it.
/// @custom:security-contact security@anoma.foundation
contract ProposeLegacyProtocolAdapterOwnershipTransfer is Script {
    using Safe for *;

    Safe.Client internal _safe;

    /// @notice Thrown if the protocol adapter is not a legacy protocol adapter deployment, i.e. not owned by
    /// the production Safe.
    error NotALegacyProtocolAdapterDeployment(address legacyProtocolAdapter);

    /// @notice Thrown if the simulated Safe execution of the transfer fails during a dry run.
    error TransactionSimulationFailed();

    /// @notice Thrown if the simulated transfer left the protocol adapter with another owner.
    error OwnerMismatch(address expected, address actual);

    /// @notice Proposes the transfer to the Safe owning the legacy protocol adapter.
    /// @dev Without `--broadcast`, the Safe execution of the transfer is simulated instead of proposed.
    /// @param legacyProtocolAdapter The legacy protocol adapter to transfer.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    function run(address legacyProtocolAdapter, address proposer) public {
        address safe = Ownable(legacyProtocolAdapter).owner();
        require(safe == Parameters.PA_MULTISIG, NotALegacyProtocolAdapterDeployment(legacyProtocolAdapter));

        bytes memory callData = abi.encodeCall(Ownable.transferOwnership, (Parameters.DEPLOYMENT_WALLET));

        _safe.initialize(safe);

        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(legacyProtocolAdapter, callData, proposer);
        } else {
            require(
                _safe.simulateTransactionMultiSigNoSign(
                    legacyProtocolAdapter, callData, IOwnerManager(safe).getOwners()
                ),
                TransactionSimulationFailed()
            );

            address owner = Ownable(legacyProtocolAdapter).owner();
            require(
                owner == Parameters.DEPLOYMENT_WALLET,
                OwnerMismatch({expected: Parameters.DEPLOYMENT_WALLET, actual: owner})
            );
        }
    }
}
