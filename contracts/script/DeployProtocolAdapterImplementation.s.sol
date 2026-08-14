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
    /// @notice The CREATE2 salt for the implementation deployment, shared by the staging and production environments.
    bytes32 public constant IMPLEMENTATION_SALT = "ProtocolAdapterImpl";

    /// @notice The initialization data to pass to `upgradeToAndCall` when upgrading a proxy to this implementation —
    /// empty because the current version requires no reinitialization.
    bytes public constant INITIALIZATION_DATA = "";

    /// @notice Thrown if the implementation of the current source version is not deployed yet.
    error ImplementationNotDeployed(address implementation);

    /// @notice Thrown if the implementation of the current source version is already deployed.
    error ImplementationAlreadyDeployed(address implementation);

    /// @notice Initializes the supported networks and associated RISC Zero verifier router addresses
    /// (see https://dev.risczero.com/api/3.0/blockchain-integration/contracts/verifier).
    constructor() SupportedNetworks() {}

    /// @notice Validates the protocol adapter implementation for upgrade safety and deploys it on supported networks.
    /// @return implementation The protocol adapter implementation contract.
    function run() public returns (address implementation) {
        (address router, bytes4 selector, bytes memory constructorData) = _constructorArgs();

        implementation = _predict(constructorData);
        require(implementation.code.length == 0, ImplementationAlreadyDeployed(implementation));

        _validate(constructorData);

        vm.startBroadcast();
        implementation = address(
            new ProtocolAdapter{salt: IMPLEMENTATION_SALT}({
                riscZeroVerifierRouter: router, riscZeroVerifierSelector: selector
            })
        );
        vm.stopBroadcast();
    }

    /// @notice Returns the deployed implementation of the current source version, validated for upgrade safety.
    /// @return implementation The deployed protocol adapter implementation contract.
    function deployed() public returns (address implementation) {
        (,, bytes memory constructorData) = _constructorArgs();

        implementation = _predict(constructorData);
        require(implementation.code.length != 0, ImplementationNotDeployed(implementation));

        _validate(constructorData);
    }

    /// @notice Predicts the deterministic address the implementation of this source version deploys to.
    /// @return implementation The predicted protocol adapter implementation contract address.
    function predict() public view returns (address implementation) {
        (,, bytes memory constructorData) = _constructorArgs();

        implementation = _predict(constructorData);
    }

    /// @notice Validates the protocol adapter implementation for upgrade safety.
    function _validate(bytes memory constructorData) internal {
        Options memory opts;
        opts.constructorData = constructorData;

        Upgrades.validateImplementation("ProtocolAdapter.sol", opts);
    }

    /// @notice Returns the implementation constructor arguments for the current chain.
    function _constructorArgs() internal view returns (address router, bytes4 selector, bytes memory constructorData) {
        router = address(getRouterData().router);
        selector = RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR;
        constructorData = abi.encode(router, selector);
    }

    /// @notice Predicts the deterministic address for the provided constructor arguments.
    function _predict(bytes memory constructorData) internal view returns (address implementation) {
        implementation = vm.computeCreate2Address(
            IMPLEMENTATION_SALT, keccak256(abi.encodePacked(type(ProtocolAdapter).creationCode, constructorData))
        );
    }
}
