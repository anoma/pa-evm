// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";
import {IOwnerManager} from "safe-smart-account-1.5.0/contracts/interfaces/IOwnerManager.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title ProductionScript
/// @author Anoma Foundation, 2025
/// @notice The base of the scripts acting on the production environment protocol adapter proxy, which is owned by a
/// Safe multisig whose owners confirm and execute the proposed transactions in the Safe app.
/// @custom:security-contact security@anoma.foundation
abstract contract ProductionScript is Script {
    using Safe for *;

    Safe.Client internal _safe;

    /// @notice Thrown if the proxy is not a production deployment, i.e. not owned by the production proxy owner.
    error NotAProductionDeployment(address proxy);

    /// @notice Thrown if the simulated Safe execution of the transaction fails during a dry run.
    error TransactionSimulationFailed();

    /// @notice Proposes a transaction on the proxy to the Safe owning it via the Safe Transaction Service. Without
    /// `--broadcast`, the Safe execution of the transaction is simulated instead of proposed.
    /// @param proxy The production environment protocol adapter proxy to act on.
    /// @param callData The call to propose.
    /// @param proposer The Safe owner or delegate proposing the transaction. Must be the unlocked script account
    /// (`--account`), which signs the proposal; hardware wallets require the `sign` and
    /// `proposeTransactionWithSignature` flow of `safe-utils`.
    function _propose(address proxy, bytes memory callData, address proposer) internal {
        address safe = ProtocolAdapter(proxy).owner();
        require(safe == new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION(), NotAProductionDeployment(proxy));

        _safe.initialize(safe);

        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(proxy, callData, proposer);
        } else {
            require(
                _safe.simulateTransactionMultiSigNoSign(proxy, callData, IOwnerManager(safe).getOwners()),
                TransactionSimulationFailed()
            );
        }
    }
}
