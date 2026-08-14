// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

/// @title DeployProtocolAdapterImplementation
/// @author Anoma Foundation, 2025
/// @notice A script to deploy the protocol adapter implementation on supported networks.
/// @custom:security-contact security@anoma.foundation
contract DeployProtocolAdapterImplementation is SupportedNetworks, Script {
    /// @notice The CREATE2 salt for the implementation deployment, shared by the test and prod environments.
    bytes32 public constant IMPLEMENTATION_SALT = "ProtocolAdapterImpl";

    /// @notice Initializes the supported networks and associated RISC Zero verifier router addresses
    /// (see https://dev.risczero.com/api/3.0/blockchain-integration/contracts/verifier).
    constructor() SupportedNetworks() {}

    /// @notice Validates the protocol adapter implementation for upgrade safety and deploys it on supported networks.
    /// Idempotent: if the current version is already deployed, the existing deployment is returned so that the test
    /// and prod environments can share it.
    /// @return implementation The protocol adapter implementation contract — the contract to verify on block
    /// explorers, which carries the source.
    function run() public returns (address implementation) {
        // Lookup the RISC Zero router address from the supported networks.
        SupportedNetworks.Data memory data = getRouterData();

        Options memory opts;
        opts.constructorData = abi.encode(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR);

        implementation = vm.computeCreate2Address(
            IMPLEMENTATION_SALT, keccak256(abi.encodePacked(type(ProtocolAdapter).creationCode, opts.constructorData))
        );
        if (implementation.code.length != 0) {
            return implementation;
        }

        // Validate the implementation for upgrade safety.
        Upgrades.validateImplementation("ProtocolAdapter.sol", opts);

        vm.startBroadcast();
        implementation = address(
            new ProtocolAdapter{salt: IMPLEMENTATION_SALT}({
                riscZeroVerifierRouter: data.router,
                riscZeroVerifierSelector: RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR
            })
        );
        vm.stopBroadcast();
    }
}
