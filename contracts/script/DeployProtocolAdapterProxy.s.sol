// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "./DeployProtocolAdapterImplementation.s.sol";

/// @title DeployProtocolAdapterProxy
/// @author Anoma Foundation, 2025
/// @notice A script to deploy the protocol adapter implementation and an ERC-1967 proxy pointing to it on supported
/// networks.
/// @custom:security-contact security@anoma.foundation
contract DeployProtocolAdapterProxy is Script {
    /// @notice The CREATE2 salt for the staging environment proxy deployment.
    bytes32 public constant PROXY_SALT_STAGING = "ProtocolAdapterProxyStaging";

    /// @notice The CREATE2 salt for the production environment proxy deployment.
    bytes32 public constant PROXY_SALT_PRODUCTION = "ProtocolAdapterProxyProduction";

    /// @notice The staging environment proxy owner — the deployment wallet, upgrading instantly.
    address public constant PROXY_OWNER_STAGING = 0x61462bE56782568376f9cB069382EFa72764a407;

    /// @notice The production environment proxy owner — the Safe multisig queueing upgrades.
    address public constant PROXY_OWNER_PRODUCTION = 0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10;

    /// @notice The deployments recorded per environment, relative to the Foundry root.
    string internal constant _DEPLOYMENTS_PATH = "../crates/bindings/deployments.json";

    /// @notice Thrown if the environment already has a deployment recorded for this chain.
    error DeploymentAlreadyRecorded(string environment, uint256 chainId);

    /// @notice Thrown if the proxy of this source version is already deployed.
    error ProxyAlreadyDeployed(address proxy);

    /// @notice Deploys an ERC-1967 proxy pointing to the protocol adapter implementation deterministically, deploying
    /// the implementation first unless the environments already share it. The implementation is validated for upgrade
    /// safety.
    /// @param isProduction Whether to deploy the production or the staging environment proxy, selecting
    /// the CREATE2 salt and the owner receiving the authority to stop the protocol adapter in an emergency and to
    /// authorize upgrades.
    /// @return proxy The protocol adapter proxy contract to interact with.
    /// @return implementation The protocol adapter implementation contract the proxy delegates to.
    /// @return initializerData The proxy constructor's initializer data, to record in `deployments.json`.
    /// @return creationCode The ERC-1967 proxy creation code, to record in `deployments.json`.
    function run(bool isProduction)
        public
        returns (address proxy, address implementation, bytes memory initializerData, bytes memory creationCode)
    {
        DeployProtocolAdapterImplementation implementationScript = new DeployProtocolAdapterImplementation();

        bytes32 salt = isProduction ? PROXY_SALT_PRODUCTION : PROXY_SALT_STAGING;

        // Checks
        {
            _requireUnrecorded(isProduction);

            (implementation,) = implementationScript.predict();

            (proxy, initializerData, creationCode) =
                _predict({salt: salt, implementation: implementation, isProduction: isProduction});
            require(proxy.code.length == 0, ProxyAlreadyDeployed({proxy: proxy}));
        }

        // Deployment
        if (implementation.code.length == 0) {
            implementationScript.run();
        }

        vm.startBroadcast();
        proxy = address(new ERC1967Proxy{salt: salt}({implementation: implementation, _data: initializerData}));
        vm.stopBroadcast();
    }

    /// @notice Predicts the deterministic address the proxy of this source version deploys to.
    /// @return proxy The predicted protocol adapter proxy contract address.
    /// @return implementation The predicted implementation contract address the proxy commits to.
    function predict(bool isProduction) public returns (address proxy, address implementation) {
        bytes32 salt = isProduction ? PROXY_SALT_PRODUCTION : PROXY_SALT_STAGING;

        (implementation,) = new DeployProtocolAdapterImplementation().predict();

        (proxy,,) = _predict({salt: salt, implementation: implementation, isProduction: isProduction});
    }

    /// @notice Returns the name of an environment, which keys its deployments in `deployments.json`.
    function environmentName(bool isProduction) public pure returns (string memory name) {
        name = isProduction ? "production" : "staging";
    }

    /// @notice Checks that the environment has no deployment recorded for this chain yet.
    function _requireUnrecorded(bool isProduction) internal view {
        string memory json = vm.readFile(_DEPLOYMENTS_PATH);
        string memory environment = environmentName(isProduction);

        for (uint256 i = 0;; ++i) {
            // solhint-disable-next-line func-named-parameters
            string memory entry = string.concat(".", environment, "[", vm.toString(i), "]");
            if (!vm.keyExistsJson(json, entry)) {
                return;
            }

            require(
                vm.parseJsonUint(json, string.concat(entry, ".chainId")) != block.chainid,
                DeploymentAlreadyRecorded(environment, block.chainid)
            );
        }
    }

    /// @notice Derives the deterministic proxy address and the constructor arguments it commits to.
    function _predict(bytes32 salt, address implementation, bool isProduction)
        internal
        pure
        returns (address proxy, bytes memory initializerData, bytes memory creationCode)
    {
        initializerData = abi.encodeCall(
            ProtocolAdapter.initialize, (isProduction ? PROXY_OWNER_PRODUCTION : PROXY_OWNER_STAGING)
        );
        creationCode = type(ERC1967Proxy).creationCode;

        bytes memory constructorArgs = abi.encode(implementation, initializerData);

        bytes memory initCode = abi.encodePacked(creationCode, constructorArgs);

        proxy = vm.computeCreate2Address({salt: salt, initCodeHash: keccak256(initCode)});
    }
}
