// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {StagingScript} from "./StagingScript.s.sol";

/// @title ExecuteProtocolAdapterUpgrade
/// @author Anoma Foundation, 2026
/// @notice A script to execute upgrading the staging environment protocol adapter proxy to the deployed
/// implementation of the current source version (see `DeployProtocolAdapterImplementation`). Staging only: the
/// production proxy is owned by a Safe multisig, whose owners execute the upgrade proposed by
/// `production/ProposeProtocolAdapterUpgrade` in the Safe app instead.
/// @custom:security-contact security@anoma.foundation
contract ExecuteProtocolAdapterUpgrade is StagingScript {
    /// @notice Executes the upgrade as the proxy owner, which the sender must be. Without `--broadcast`, the upgrade
    /// is simulated locally.
    /// @param proxy The staging environment protocol adapter proxy to upgrade.
    /// @param newImplementation The implementation contract to upgrade to, which must be the one this source version
    /// deploys to. Take it from the deployment that produced it, not from this source, so that the check below
    /// compares two independent derivations.
    function run(address proxy, address newImplementation) public {
        DeployProtocolAdapterImplementation implementationDeployScript = new DeployProtocolAdapterImplementation();
        // forge-lint: disable-next-line(unused-return)
        (address predictedImplementation,) = implementationDeployScript.predict();

        require(
            newImplementation == predictedImplementation,
            DeployProtocolAdapterImplementation.UnexpectedImplementation(predictedImplementation, newImplementation)
        );
        require(
            newImplementation.code.length != 0,
            DeployProtocolAdapterImplementation.ImplementationNotDeployed(newImplementation)
        );

        _checkSenderAuthorization({proxy: proxy});

        vm.startBroadcast();
        UUPSUpgradeable(proxy).upgradeToAndCall(newImplementation, implementationDeployScript.INITIALIZATION_DATA());
        vm.stopBroadcast();
    }
}
