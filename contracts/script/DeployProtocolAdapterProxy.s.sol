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

    /// @notice Deploys the protocol adapter implementation and an ERC-1967 proxy pointing to it deterministically.
    /// The implementation is validated for upgrade safety. Idempotent: if the proxy of this source version is
    /// already deployed, the existing deployment is returned. The proxy address commits to the implementation and
    /// the owner it is initialized with, so a later version deploys a new proxy instead.
    /// @param isProduction Whether to deploy the production or the staging environment proxy, selecting
    /// the CREATE2 salt and the owner receiving the authority to stop the protocol adapter in an emergency and to
    /// authorize upgrades.
    /// @return proxy The protocol adapter proxy contract to interact with.
    /// @return implementation The protocol adapter implementation contract the proxy delegates to — the contract to
    /// verify on block explorers, which carries the source, whereas the proxy carries the ERC-1967 bytecode.
    function run(bool isProduction) public returns (address proxy, address implementation) {
        implementation = new DeployProtocolAdapterImplementation().run();

        bytes memory initializerData =
            abi.encodeCall(ProtocolAdapter.initialize, (isProduction ? PROXY_OWNER_PRODUCTION : PROXY_OWNER_STAGING));
        bytes32 salt = isProduction ? PROXY_SALT_PRODUCTION : PROXY_SALT_STAGING;

        proxy = vm.computeCreate2Address(
            salt,
            keccak256(abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(implementation, initializerData)))
        );
        if (proxy.code.length != 0) {
            return (proxy, implementation);
        }

        vm.startBroadcast();
        proxy = address(new ERC1967Proxy{salt: salt}({implementation: implementation, _data: initializerData}));
        vm.stopBroadcast();
    }
}
