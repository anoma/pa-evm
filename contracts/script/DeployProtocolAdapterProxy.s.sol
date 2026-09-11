// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";

import {RecordedDeployments} from "../generated/RecordedDeployments.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "./DeployProtocolAdapterImplementation.s.sol";
import {
    DeployTransitionalProtocolAdapterImplementation
} from "./migration/DeployTransitionalProtocolAdapterImplementation.s.sol";
import {Parameters} from "./Parameters.sol";

/// @title DeployProtocolAdapterProxy
/// @author Anoma Foundation, 2026
/// @notice A script to deploy the protocol adapter implementation and an ERC-1967 proxy pointing to it on supported
/// networks. A chain that ran v1 starts its proxy on the transitional implementation instead, which copies the v1
/// state in.
/// @custom:security-contact security@anoma.foundation
contract DeployProtocolAdapterProxy is Script {
    /// @notice The CREATE2 salt for the staging environment proxy deployment.
    bytes32 public constant PROXY_SALT_STAGING = Parameters.PROXY_SALT_STAGING;

    /// @notice The CREATE2 salt for the production environment proxy deployment.
    bytes32 public constant PROXY_SALT_PRODUCTION = Parameters.PROXY_SALT_PRODUCTION;

    /// @notice The staging environment proxy owner — the deployment wallet, upgrading instantly.
    address public constant PROXY_OWNER_STAGING = Parameters.PROXY_OWNER_STAGING;

    /// @notice The production environment proxy owner — the Safe multisig queueing upgrades.
    address public constant PROXY_OWNER_PRODUCTION = Parameters.PROXY_OWNER_PRODUCTION;

    /// @notice Thrown if the environment already has a deployment recorded for this chain.
    error DeploymentAlreadyRecorded(string environment, uint256 chainId);

    /// @notice Thrown if the proxy of this source version is already deployed.
    error ProxyAlreadyDeployed(address proxy);

    /// @notice Deploys an ERC-1967 proxy pointing to the protocol adapter implementation deterministically. The
    /// implementation is validated for upgrade safety and deployed first, unless the other environment deployed it
    /// already.
    /// @param isProduction Whether to deploy the production or the staging environment proxy, selecting the CREATE2
    /// salt and the owner receiving the authority to stop the protocol adapter in an emergency and to authorize
    /// upgrades.
    /// @param isTransitional Whether the proxy starts on the transitional implementation, which begins paused and
    /// copies in the state of the chain's v1 protocol adapter, instead of the plain implementation. The proxy address
    /// commits to the implementation, so the two land at different addresses.
    /// @return proxy The protocol adapter proxy contract to interact with.
    /// @return implementation The protocol adapter implementation contract the proxy delegates to.
    /// @return initializerData The proxy constructor's initializer data, to record in `deployments.json`.
    /// @return creationCode The ERC-1967 proxy creation code, to record in `deployments.json`.
    function run(bool isProduction, bool isTransitional)
        public
        returns (address proxy, address implementation, bytes memory initializerData, bytes memory creationCode)
    {
        bytes32 salt = isProduction ? PROXY_SALT_PRODUCTION : PROXY_SALT_STAGING;

        // Checks
        {
            require(
                !RecordedDeployments.isRecorded({isProduction: isProduction, chainId: block.chainid}),
                DeploymentAlreadyRecorded(environmentName(isProduction), block.chainid)
            );

            implementation = _predictImplementation(isTransitional);

            (proxy, initializerData, creationCode) =
                _predict({salt: salt, implementation: implementation, isProduction: isProduction});
            require(proxy.code.length == 0, ProxyAlreadyDeployed({proxy: proxy}));
        }

        // Deployment
        if (implementation.code.length == 0) {
            _deployImplementation(isTransitional);
        }

        vm.startBroadcast();
        proxy = address(new ERC1967Proxy{salt: salt}({implementation: implementation, _data: initializerData}));
        vm.stopBroadcast();
    }

    /// @notice Predicts the deterministic address the proxy of this source version deploys to.
    /// @param isProduction Whether to predict the production or the staging environment proxy.
    /// @param isTransitional Whether the proxy starts on the transitional or the plain implementation.
    /// @return proxy The predicted protocol adapter proxy contract address.
    /// @return implementation The predicted implementation contract address the proxy commits to.
    function predict(bool isProduction, bool isTransitional) public returns (address proxy, address implementation) {
        bytes32 salt = isProduction ? PROXY_SALT_PRODUCTION : PROXY_SALT_STAGING;

        implementation = _predictImplementation(isTransitional);

        (proxy,,) = _predict({salt: salt, implementation: implementation, isProduction: isProduction});
    }

    /// @notice Returns the name of an environment, which keys its deployments in `deployments.json`.
    /// @param isProduction Whether to name the production or the staging environment.
    /// @return name The environment name.
    function environmentName(bool isProduction) public pure returns (string memory name) {
        name = isProduction ? "production" : "staging";
    }

    /// @notice Predicts the implementation the proxy starts on.
    /// @param isTransitional Whether to predict the transitional or the plain implementation.
    /// @return implementation The predicted implementation contract address.
    function _predictImplementation(bool isTransitional) internal returns (address implementation) {
        if (isTransitional) {
            // forge-lint: disable-next-line(unused-return)
            (implementation,) = new DeployTransitionalProtocolAdapterImplementation().predict();
        } else {
            // forge-lint: disable-next-line(unused-return)
            (implementation,) = new DeployProtocolAdapterImplementation().predict();
        }
    }

    /// @notice Deploys the implementation the proxy starts on.
    /// @param isTransitional Whether to deploy the transitional or the plain implementation.
    function _deployImplementation(bool isTransitional) internal {
        if (isTransitional) {
            // forge-lint: disable-next-line(unused-return)
            new DeployTransitionalProtocolAdapterImplementation().run();
        } else {
            // forge-lint: disable-next-line(unused-return)
            new DeployProtocolAdapterImplementation().run();
        }
    }

    /// @notice Derives the deterministic proxy address and the constructor arguments it commits to.
    /// @param salt The CREATE2 salt of the environment.
    /// @param implementation The implementation contract the proxy delegates to.
    /// @param isProduction Whether to derive the production or the staging environment proxy, selecting the owner.
    /// @return proxy The deterministic protocol adapter proxy contract address.
    /// @return initializerData The proxy constructor's initializer data.
    /// @return creationCode The ERC-1967 proxy creation code.
    function _predict(bytes32 salt, address implementation, bool isProduction)
        internal
        pure
        returns (address proxy, bytes memory initializerData, bytes memory creationCode)
    {
        // The transitional implementation overrides `initialize` with the same signature, so the data serves both.
        initializerData =
            abi.encodeCall(ProtocolAdapter.initialize, (isProduction ? PROXY_OWNER_PRODUCTION : PROXY_OWNER_STAGING));
        creationCode = type(ERC1967Proxy).creationCode;

        bytes memory constructorArgs = abi.encode(implementation, initializerData);

        // The creation code of one fixed type precedes its ABI-encoded arguments, so the split is unambiguous.
        // forge-lint: disable-next-line(encode-packed-collision)
        bytes memory initCode = abi.encodePacked(creationCode, constructorArgs);

        proxy = vm.computeCreate2Address({salt: salt, initCodeHash: keccak256(initCode)});
    }
}
