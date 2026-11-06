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
    /// @notice Proposes the upgrade of the recorded production proxy to the implementation this source version
    /// predicts to the Safe owning the proxy.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    function run(address proposer) public {
        DeployProtocolAdapterImplementation implementationDeployScript = new DeployProtocolAdapterImplementation();
        address implementation = _requireDeployedImplementation(implementationDeployScript);

        _propose({
            callData: abi.encodeCall(
                UUPSUpgradeable.upgradeToAndCall, (implementation, implementationDeployScript.INITIALIZATION_DATA())
            ),
            proposer: proposer
        });
    }
}
