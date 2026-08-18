// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeProtocolAdapterUpgrade
/// @author Anoma Foundation, 2026
/// @notice A script to propose upgrading the production environment protocol adapter proxy to the deployed
/// implementation of the current source version (see `DeployProtocolAdapterImplementation`) to the Safe owning the
/// proxy. The Safe owners confirm and execute the proposed upgrade in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeProtocolAdapterUpgrade is ProductionScript {
    /// @notice Proposes the upgrade to the Safe owning the proxy.
    /// @param proxy The production environment protocol adapter proxy to upgrade.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @param newImplementation The implementation contract to upgrade to, which must be the one this source version
    /// deploys to. Take it from the deployment that produced it, not from this source, so that the check below
    /// compares two independent derivations.
    function run(address proxy, address proposer, address newImplementation) public {
        DeployProtocolAdapterImplementation implementationScript = new DeployProtocolAdapterImplementation();
        (address predictedImplementation,) = implementationScript.predict();

        require(
            newImplementation == predictedImplementation,
            DeployProtocolAdapterImplementation.UnexpectedImplementation(predictedImplementation, newImplementation)
        );
        require(
            newImplementation.code.length != 0,
            DeployProtocolAdapterImplementation.ImplementationNotDeployed(newImplementation)
        );

        _propose({
            proxy: proxy,
            callData: abi.encodeCall(
                UUPSUpgradeable.upgradeToAndCall, (newImplementation, implementationScript.INITIALIZATION_DATA())
            ),
            proposer: proposer
        });
    }
}
