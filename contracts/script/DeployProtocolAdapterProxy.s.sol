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
    /// @notice The CREATE2 salt for the test environment proxy deployment.
    bytes32 public constant TEST_PROXY_SALT = "TEST_ProtocolAdapterProxy";

    /// @notice The CREATE2 salt for the prod environment proxy deployment.
    bytes32 public constant PROD_PROXY_SALT = "PROD_ProtocolAdapterProxy";

    /// @notice The test environment proxy owner — the deployment wallet, upgrading instantly.
    address public constant TEST_PROXY_OWNER = 0x61462bE56782568376f9cB069382EFa72764a407;

    /// @notice The prod environment proxy owner — the Safe multisig queueing upgrades.
    address public constant PROD_PROXY_OWNER = 0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10;

    /// @notice Deploys the protocol adapter implementation and an ERC-1967 proxy pointing to it deterministically.
    /// The implementation is validated for upgrade safety.
    /// @param isTestDeployment Whether to deploy the test or the prod environment proxy, selecting the CREATE2 salt
    /// and the owner receiving the authority to stop the protocol adapter in an emergency and to authorize upgrades.
    /// @return proxy The protocol adapter proxy contract to interact with.
    /// @return implementation The protocol adapter implementation contract the proxy delegates to — the contract to
    /// verify on block explorers, which carries the source, whereas the proxy carries the ERC-1967 bytecode.
    function run(bool isTestDeployment) public returns (address proxy, address implementation) {
        implementation = new DeployProtocolAdapterImplementation().run();

        bytes memory initializerData =
            abi.encodeCall(ProtocolAdapter.initialize, (isTestDeployment ? TEST_PROXY_OWNER : PROD_PROXY_OWNER));

        vm.startBroadcast();

        proxy = address(
            new ERC1967Proxy{salt: isTestDeployment ? TEST_PROXY_SALT : PROD_PROXY_SALT}({
                implementation: implementation, _data: initializerData
            })
        );

        vm.stopBroadcast();
    }
}
