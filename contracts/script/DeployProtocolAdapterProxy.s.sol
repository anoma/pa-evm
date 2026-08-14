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

    /// @notice Deploys the protocol adapter implementation and an ERC-1967 proxy pointing to it on supported networks
    /// and allows for test deployments. The implementation is validated for upgrade safety in both cases.
    /// @param isTestDeployment Whether the deployment is a test deployment or not. If set to `false`, the
    /// implementation and proxy are deployed deterministically.
    /// @param initialOwner The account receiving ownership of the deployed proxy, and with it the authority to stop the
    /// protocol adapter in an emergency and to authorize upgrades.
    /// @return proxy The protocol adapter proxy contract to interact with.
    /// @return implementation The protocol adapter implementation contract the proxy delegates to — the contract to
    /// verify on block explorers, which carries the source, whereas the proxy carries the ERC-1967 bytecode.
    function run(bool isTestDeployment, address initialOwner) public returns (address proxy, address implementation) {
        implementation = new DeployProtocolAdapterImplementation().run();

        bytes memory initializerData = abi.encodeCall(ProtocolAdapter.initialize, (initialOwner));

        vm.startBroadcast();

        proxy = address(
            new ERC1967Proxy{salt: isTestDeployment ? TEST_PROXY_SALT : PROD_PROXY_SALT}({
                implementation: implementation, _data: initializerData
            })
        );

        vm.stopBroadcast();
    }
}
