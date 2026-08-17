// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {ProductionScript} from "./ProductionScript.s.sol";

/// @title ProposeProtocolAdapterUpgrade
/// @author Anoma Foundation, 2025
/// @notice A script to propose upgrading the production environment protocol adapter proxy to the deployed
/// implementation of the current source version (see `DeployProtocolAdapterImplementation`) to the Safe owning the
/// proxy. The Safe owners confirm and execute the proposed upgrade in the Safe app.
/// @custom:security-contact security@anoma.foundation
contract ProposeProtocolAdapterUpgrade is ProductionScript {
    /// @notice Proposes the upgrade to the Safe owning the proxy. Without `--broadcast`, the Safe execution of the
    /// upgrade is simulated instead of proposed.
    /// @param proxy The production environment protocol adapter proxy to upgrade.
    /// @param proposer The Safe owner or delegate proposing the transaction.
    /// @return implementation The deployed implementation contract the upgrade was proposed for.
    function run(address proxy, address proposer) public returns (address implementation) {
        DeployProtocolAdapterImplementation implementationScript = new DeployProtocolAdapterImplementation();
        implementation = implementationScript.deployed();

        _propose({
            proxy: proxy,
            callData: abi.encodeCall(
                UUPSUpgradeable.upgradeToAndCall, (implementation, implementationScript.INITIALIZATION_DATA())
            ),
            proposer: proposer
        });
    }
}
