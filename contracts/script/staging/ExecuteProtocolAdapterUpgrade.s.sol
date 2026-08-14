// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteProtocolAdapterUpgrade
/// @author Anoma Foundation, 2025
/// @notice A script to execute upgrading the staging environment protocol adapter proxy to the deployed
/// implementation of the current source version (see `DeployProtocolAdapterImplementation`). Staging only: the
/// production proxy is owned by a Safe multisig, whose owners execute the upgrade proposed by
/// `production/ProposeProtocolAdapterUpgrade` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteProtocolAdapterUpgrade is StagingScript {
    /// @notice Executes the upgrade as the proxy owner, which the sender must be. Without `--broadcast`, the upgrade
    /// is simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to upgrade.
    /// @return implementation The deployed implementation contract the proxy was upgraded to.
    function run(address proxy) public returns (address implementation) {
        DeployProtocolAdapterImplementation implementationScript = new DeployProtocolAdapterImplementation();
        implementation = implementationScript.deployed();

        _checkSenderAuthorization(proxy);

        vm.startBroadcast();
        UUPSUpgradeable(proxy).upgradeToAndCall(implementation, implementationScript.INITIALIZATION_DATA());
        vm.stopBroadcast();
    }
}
