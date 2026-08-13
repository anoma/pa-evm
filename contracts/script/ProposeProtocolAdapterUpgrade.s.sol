// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";
import {Safe} from "safe-utils-0.0.22/src/Safe.sol";

import {DeployProtocolAdapterImplementation} from "./DeployProtocolAdapterImplementation.s.sol";

/// @title ProposeUpgradeProtocolAdapterProxy
/// @author Anoma Foundation, 2025
/// @notice A script to deploy the current protocol adapter implementation and propose upgrading the proxy to it to
/// the Safe owning the proxy.
/// @custom:security-contact security@anoma.foundation
contract ProposeProtocolAdapterUpgrade is Script {
    using Safe for *;

    Safe.Client internal _safe;

    /// @notice Thrown if the simulated Safe execution of the upgrade fails during a dry run.
    error UpgradeSimulationFailed();

    /// @notice Deploys the protocol adapter implementation and proposes upgrading the proxy to it to the Safe
    /// Transaction Service. Without `--broadcast`, the Safe execution of the upgrade is simulated instead.
    /// @param isTestDeployment Whether the implementation deployment is a test deployment or not. If set to `false`,
    /// the implementation is deployed deterministically.
    /// @param proxy The protocol adapter proxy to upgrade.
    /// @param safe The Safe owning the proxy.
    /// @param proposer The Safe owner or delegate proposing the transaction. Must be the unlocked script account
    /// (`--account`), which signs the proposal; hardware wallets require the `sign` and
    /// `proposeTransactionWithSignature` flow of `safe-utils`.
    /// @return implementation The protocol adapter implementation contract the upgrade was proposed for.
    function run(bool isTestDeployment, address proxy, address safe, address proposer)
        public
        returns (address implementation)
    {
        implementation = new DeployProtocolAdapterImplementation().run(isTestDeployment);

        bytes memory upgradeCall = abi.encodeCall(UUPSUpgradeable.upgradeToAndCall, (implementation, bytes("")));

        _safe.initialize(safe);

        if (Safe.isBroadcastMode()) {
            _safe.proposeTransaction(proxy, upgradeCall, proposer);
        } else {
            require(_safe.simulateTransactionNoSign(proxy, upgradeCall, proposer), UpgradeSimulationFailed());
        }
    }
}
