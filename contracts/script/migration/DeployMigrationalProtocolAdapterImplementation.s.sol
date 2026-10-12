// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.4/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.4/src/SupportedNetworks.sol";
import {Script} from "forge-std-1.17.0/src/Script.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {RecordedDeployments} from "../../generated/RecordedDeployments.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {Parameters} from "../Parameters.sol";

/// @title DeployMigrationalProtocolAdapterImplementation
/// @author Anoma Foundation, 2026
/// @notice A script to deploy the migrational protocol adapter implementation on a supported network that runs a
/// legacy protocol adapter. A proxy on it starts paused, copies its state in, and then upgrades to
/// `ProtocolAdapter`.
/// @custom:security-contact security@anoma.foundation
contract DeployMigrationalProtocolAdapterImplementation is SupportedNetworks, Script {
    /// @notice The CREATE2 salt for the implementation deployment, shared by the staging and production environments.
    bytes32 public constant IMPLEMENTATION_SALT = Parameters.IMPLEMENTATION_SALT;

    /// @notice Thrown if the implementation of the current source version is already deployed.
    error ImplementationAlreadyDeployed(address implementation);

    /// @notice Thrown if the chain runs no legacy protocol adapter to copy the state from.
    error NoLegacyProtocolAdapter(uint256 chainId);

    /// @notice Validates the migrational implementation for upgrade safety and deploys it on a supported network
    /// that runs a legacy protocol adapter.
    /// @return implementation The migrational protocol adapter implementation contract.
    function run() public returns (address implementation) {
        bytes memory constructorData;
        (implementation, constructorData) = predict();
        require(implementation.code.length == 0, ImplementationAlreadyDeployed({implementation: implementation}));

        Options memory opts;
        opts.constructorData = constructorData;

        Upgrades.validateImplementation("MigrationalProtocolAdapter.sol", opts);

        vm.startBroadcast();
        implementation = address(
            new MigrationalProtocolAdapter{salt: IMPLEMENTATION_SALT}({
                riscZeroVerifierRouter: address(getRouterData().router),
                riscZeroVerifierSelector: RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR,
                legacyProtocolAdapter: getLegacyProtocolAdapter(block.chainid)
            })
        );
        vm.stopBroadcast();
    }

    /// @notice Predicts the deterministic address the migrational implementation of this source version deploys to.
    /// @return implementation The predicted implementation contract address.
    /// @return constructorData The constructor arguments the predicted address commits to.
    function predict() public view returns (address implementation, bytes memory constructorData) {
        (implementation, constructorData) = predict(block.chainid);
    }

    /// @notice Predicts the deterministic address the migrational implementation of this source version deploys to
    /// on a supported network that runs a legacy protocol adapter.
    /// @param chainId The chain ID of the network.
    /// @return implementation The predicted implementation contract address.
    /// @return constructorData The constructor arguments the predicted address commits to.
    function predict(uint256 chainId) public view returns (address implementation, bytes memory constructorData) {
        Data memory data = _riscZeroVerifierRouters[_supportedNetworks[chainId]];
        require(address(data.router) != address(0), UnsupportedNetwork({chainId: chainId}));

        constructorData = abi.encode(
            address(data.router),
            RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR,
            getLegacyProtocolAdapter(chainId)
        );
        // The creation code of one fixed type precedes its ABI-encoded arguments, so the split is unambiguous.
        // forge-lint: disable-next-line(encode-packed-collision)
        bytes memory initCode = abi.encodePacked(type(MigrationalProtocolAdapter).creationCode, constructorData);

        implementation = vm.computeCreate2Address({salt: IMPLEMENTATION_SALT, initCodeHash: keccak256(initCode)});
    }

    /// @notice Returns the legacy protocol adapter of a chain, as `RecordedDeployments` records it.
    /// @param chainId The chain ID of the network.
    /// @return legacyProtocolAdapter The legacy protocol adapter the migrational implementation copies the state
    /// from.
    function getLegacyProtocolAdapter(uint256 chainId) public pure returns (address legacyProtocolAdapter) {
        legacyProtocolAdapter = RecordedDeployments.legacyProtocolAdapter(chainId);
        require(legacyProtocolAdapter != address(0), NoLegacyProtocolAdapter({chainId: chainId}));
    }
}
