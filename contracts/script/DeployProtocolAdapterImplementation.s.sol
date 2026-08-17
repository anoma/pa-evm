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
        bytes memory constructorData;
        (implementation, constructorData) = predict();
        require(implementation.code.length == 0, ImplementationAlreadyDeployed({implementation: implementation}));

        Options memory opts;
        opts.constructorData = constructorData;

        Upgrades.validateImplementation("ProtocolAdapter.sol", opts);

        vm.startBroadcast();
        implementation = address(
            new ProtocolAdapter{salt: IMPLEMENTATION_SALT}({
                riscZeroVerifierRouter: address(getRouterData().router),
                riscZeroVerifierSelector: RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR
            })
        );
        vm.stopBroadcast();
    }

    /// @notice Predicts the deterministic address the implementation of this source version deploys to.
    /// @return implementation The predicted implementation contract address.
    /// @return constructorData The constructor arguments the predicted address commits to.
    function predict() public view returns (address implementation, bytes memory constructorData) {
        (implementation, constructorData) = predict(block.chainid);
    }

    /// @notice Predicts the deterministic address the implementation of this source version deploys to on a supported
    /// network, which needs no fork of it — the router is the only per-chain constructor argument.
    /// @param chainId The chain ID of the supported network.
    /// @return implementation The predicted implementation contract address.
    /// @return constructorData The constructor arguments the predicted address commits to.
    function predict(uint256 chainId) public view returns (address implementation, bytes memory constructorData) {
        Data memory data = _riscZeroVerifierRouters[_supportedNetworks[chainId]];
        require(address(data.router) != address(0), UnsupportedNetwork({chainId: chainId}));

        bytes32 salt = IMPLEMENTATION_SALT;
        constructorData = abi.encode(address(data.router), RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR);
        bytes memory initCode = abi.encodePacked(type(ProtocolAdapter).creationCode, constructorData);

        implementation = vm.computeCreate2Address({salt: salt, initCodeHash: keccak256(initCode)});
    }
}
