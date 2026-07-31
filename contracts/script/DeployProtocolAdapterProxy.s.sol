// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.6.1/proxy/ERC1967/ERC1967Proxy.sol";
import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.0/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.0/src/SupportedNetworks.sol";
import {Script} from "forge-std-1.16.1/src/Script.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.1/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.1/src/Upgrades.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

/// @title DeployProtocolAdapterProxy
/// @author Anoma Foundation, 2025
/// @notice A script to deploy the protocol adapter implementation and an ERC-1967 proxy pointing to it on supported
/// networks.
/// @custom:security-contact security@anoma.foundation
contract DeployProtocolAdapterProxy is SupportedNetworks, Script {
    /// @notice The CREATE2 salt for the deterministic implementation deployment.
    bytes32 public constant IMPLEMENTATION_SALT = keccak256("ProtocolAdapter");

    /// @notice The CREATE2 salt for the deterministic proxy deployment.
    bytes32 public constant PROXY_SALT = keccak256("ProtocolAdapterProxy");

    /// @notice Initializes the supported networks and associated RISC Zero verifier router addresses
    /// (see https://dev.risczero.com/api/3.0/blockchain-integration/contracts/verifier).
    constructor() SupportedNetworks() {}

    /// @notice Deploys the protocol adapter implementation and proxy on supported networks and allows for test
    /// deployments. The implementation is validated for upgrade safety in both cases.
    /// @param isTestDeployment Whether the deployment is a test deployment or not. If set to `false`, the
    /// implementation and proxy are deployed deterministically.
    /// @param initialOwner The account receiving ownership of the deployed proxy, and with it the authority to stop the
    /// protocol adapter in an emergency and to authorize upgrades.
    /// @return protocolAdapterProxy The proxy contract to interact with.
    function run(bool isTestDeployment, address initialOwner) public returns (address protocolAdapterProxy) {
        // Lookup the RISC Zero router address from the supported networks.
        SupportedNetworks.Data memory data = getRouterData();

        Options memory opts;
        opts.constructorData = abi.encode(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR);

        bytes memory initializerData = abi.encodeCall(ProtocolAdapter.initialize, (initialOwner));

        if (isTestDeployment) {
            // Validate the implementation and deploy it and the proxy regularly.
            vm.startBroadcast();
            protocolAdapterProxy = Upgrades.deployUUPSProxy("ProtocolAdapter.sol", initializerData, opts);
            vm.stopBroadcast();
        } else {
            // Validate the implementation for upgrade safety.
            Upgrades.validateImplementation("ProtocolAdapter.sol", opts);

            // Deploy the implementation and proxy deterministically.
            vm.startBroadcast();
            address implementation = address(
                new ProtocolAdapter{salt: IMPLEMENTATION_SALT}({
                    riscZeroVerifierRouter: data.router,
                    riscZeroVerifierSelector: RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR
                })
            );

            protocolAdapterProxy =
                address(new ERC1967Proxy{salt: PROXY_SALT}({implementation: implementation, _data: initializerData}));
            vm.stopBroadcast();
        }
    }
}
