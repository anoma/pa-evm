// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";
import {Parameters} from "../Parameters.sol";

/// @title DeployTransitionalProtocolAdapterImplementation
/// @author Anoma Foundation, 2026
/// @notice A script to deploy the transitional protocol adapter implementation on a supported network that runs a v1
/// protocol adapter. A proxy on it starts paused, copies the v1 state in, and then upgrades to `ProtocolAdapter`.
/// @custom:security-contact security@anoma.foundation
contract DeployTransitionalProtocolAdapterImplementation is SupportedNetworks, Script {
    /// @notice The CREATE2 salt for the implementation deployment, shared by the staging and production environments.
    bytes32 public constant IMPLEMENTATION_SALT = Parameters.IMPLEMENTATION_SALT;

    /// @notice The v1 protocol adapter of each chain that runs one, as the v1 bindings (`bindings/v2.1.2`) record them.
    mapping(uint256 chainId => address protocolAdapterV1) internal _protocolAdaptersV1;

    /// @notice Thrown if the implementation of the current source version is already deployed.
    error ImplementationAlreadyDeployed(address implementation);

    /// @notice Thrown if the chain runs no v1 protocol adapter to copy the state from.
    error NoProtocolAdapterV1(uint256 chainId);

    /// @notice Records the v1 protocol adapter of each chain that runs one.
    constructor() {
        _protocolAdaptersV1[1] = 0x0eA3B55b68A3f307c8FE3fe66E443247c95F0CfF; // mainnet
        _protocolAdaptersV1[10] = 0x094FCC095323080e71a037b2B1e3519c07dd84F8; // optimism
        _protocolAdaptersV1[56] = 0xFC44b66a39fe6923Ad8d3c93bFeC369728862B68; // bsc
        _protocolAdaptersV1[8453] = 0x094FCC095323080e71a037b2B1e3519c07dd84F8; // base
        _protocolAdaptersV1[42161] = 0x094FCC095323080e71a037b2B1e3519c07dd84F8; // arbitrum
        _protocolAdaptersV1[84532] = 0x094FCC095323080e71a037b2B1e3519c07dd84F8; // base-sepolia
        _protocolAdaptersV1[11155111] = 0xf152BBA809d6cba122579cee997A54B8F3FBa417; // sepolia
    }

    /// @notice Validates the transitional implementation for upgrade safety and deploys it on a supported network
    /// that runs a v1 protocol adapter.
    /// @return implementation The transitional protocol adapter implementation contract.
    function run() public returns (address implementation) {
        bytes memory constructorData;
        (implementation, constructorData) = predict();
        require(implementation.code.length == 0, ImplementationAlreadyDeployed({implementation: implementation}));

        Options memory opts;
        opts.constructorData = constructorData;

        Upgrades.validateImplementation("TransitionalProtocolAdapter.sol", opts);

        vm.startBroadcast();
        implementation = address(
            new TransitionalProtocolAdapter{salt: IMPLEMENTATION_SALT}({
                riscZeroVerifierRouter: address(getRouterData().router),
                riscZeroVerifierSelector: RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR,
                protocolAdapterV1: getProtocolAdapterV1(block.chainid)
            })
        );
        vm.stopBroadcast();
    }

    /// @notice Predicts the deterministic address the transitional implementation of this source version deploys to.
    /// @return implementation The predicted implementation contract address.
    /// @return constructorData The constructor arguments the predicted address commits to.
    function predict() public view returns (address implementation, bytes memory constructorData) {
        (implementation, constructorData) = predict(block.chainid);
    }

    /// @notice Predicts the deterministic address the transitional implementation of this source version deploys to
    /// on a supported network that runs a v1 protocol adapter.
    /// @param chainId The chain ID of the network.
    /// @return implementation The predicted implementation contract address.
    /// @return constructorData The constructor arguments the predicted address commits to.
    function predict(uint256 chainId) public view returns (address implementation, bytes memory constructorData) {
        Data memory data = _riscZeroVerifierRouters[_supportedNetworks[chainId]];
        require(address(data.router) != address(0), UnsupportedNetwork({chainId: chainId}));

        constructorData = abi.encode(
            address(data.router), RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR, getProtocolAdapterV1(chainId)
        );
        // The creation code of one fixed type precedes its ABI-encoded arguments, so the split is unambiguous.
        // forge-lint: disable-next-line(encode-packed-collision)
        bytes memory initCode = abi.encodePacked(type(TransitionalProtocolAdapter).creationCode, constructorData);

        implementation = vm.computeCreate2Address({salt: IMPLEMENTATION_SALT, initCodeHash: keccak256(initCode)});
    }

    /// @notice Returns the v1 protocol adapter of a chain.
    /// @param chainId The chain ID of the network.
    /// @return protocolAdapterV1 The v1 protocol adapter the transitional implementation copies the state from.
    function getProtocolAdapterV1(uint256 chainId) public view returns (address protocolAdapterV1) {
        protocolAdapterV1 = _protocolAdaptersV1[chainId];
        require(protocolAdapterV1 != address(0), NoProtocolAdapterV1({chainId: chainId}));
    }
}
