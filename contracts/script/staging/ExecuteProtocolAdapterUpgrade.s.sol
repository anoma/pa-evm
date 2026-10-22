// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteProtocolAdapterUpgrade
/// @author Anoma Foundation, 2026
/// @notice A script to execute upgrading a recorded protocol adapter proxy that the staging proxy owner owns to the
/// deployed implementation of the current source version (see `DeployProtocolAdapterImplementation`). Once a Safe
/// multisig owns the proxy, its owners execute the upgrade proposed by `production/ProposeProtocolAdapterUpgrade` in
/// the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteProtocolAdapterUpgrade is StagingScript {
    /// @notice Executes the upgrade of the recorded proxy to the implementation this source version predicts, as the
    /// proxy owner. Without `--broadcast`, the upgrade is simulated locally.
    /// @param isProduction Whether to upgrade the production or the staging proxy.
    function run(bool isProduction) public {
        DeployProtocolAdapterImplementation implementationDeployScript = new DeployProtocolAdapterImplementation();
        address implementation = _requireDeployedImplementation(implementationDeployScript);
        address proxy = _requireRecordedProxy(isProduction);
        address owner = _stagingOwner({proxy: proxy});

        vm.startBroadcast(owner);
        UUPSUpgradeable(proxy).upgradeToAndCall(implementation, implementationDeployScript.INITIALIZATION_DATA());
        vm.stopBroadcast();
    }
}
