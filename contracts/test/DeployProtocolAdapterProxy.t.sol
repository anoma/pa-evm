// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployRiscZeroContracts} from "anoma-risc0-deployments-1.2.1/script/DeployRiscZeroContracts.s.sol";
import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {DeployProtocolAdapterProxy} from "../script/DeployProtocolAdapterProxy.s.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

/// @notice Checks the deploy script and checks the deployments recorded in `deployments.json` — the
/// single source of truth for deterministic deployments — against the chain state.
contract DeployProtocolAdapterProxyTest is SupportedNetworks, Test {
    /// @notice A protocol adapter deployment recorded in `deployments.json`.
    /// @dev Fields are ordered alphabetically so the struct decodes from `vm.parseJson`.
    struct Deployment {
        uint256 chainId;
        address contractAddress;
    }

    string internal constant _DEPLOYMENTS_PATH = "../crates/bindings/deployments.json";

    function test_run_succeeds_for_a_test_deployment() public {
        // Deploy the RISC Zero stack locally instead of forking a network.
        (RiscZeroVerifierRouter router,,) = new DeployRiscZeroContracts().run({admin: msg.sender, guardian: msg.sender});

        // The script resolves the router from the chain ID, so clone the local router — code and
        // storage, which holds the registered verifier — onto the router address of a supported network.
        vm.chainId(84532); // base-sepolia
        SupportedNetworks.Data memory data = getRouterData();
        vm.etch(address(data.router), address(router).code);
        vm.copyStorage(address(router), address(data.router));

        new DeployProtocolAdapterProxy().run({isTestDeployment: true, initialOwner: msg.sender});
    }

    function test_recorded_deployments_exist_and_revert_on_prod_redeployment() public {
        Deployment[] memory deployments = _recordedDeployments();

        for (uint256 i = 0; i < deployments.length; ++i) {
            string memory networkName = _supportedNetworks[deployments[i].chainId];
            assertGt(bytes(networkName).length, 0, "recorded deployment on an unsupported network");

            vm.selectFork(vm.createFork(networkName));

            address recordedProxy = deployments[i].contractAddress;
            SupportedNetworks.Data memory data = getRouterData();

            // The recorded contract must exist and run the version of this source tree.
            assertGt(recordedProxy.code.length, 0, "recorded deployment missing on-chain");
            bytes32 sourceVersion =
                new ProtocolAdapter(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR).getVersion();
            assertEq(
                ProtocolAdapter(recordedProxy).getVersion(), sourceVersion, "recorded deployment runs another version"
            );

            // Redeploying must hit the CREATE2 collision of the recorded deployment. Until builds are
            // reproducible (`via_ir` output depends on the compilation unit), the initcode compiled into
            // this test differs from the broadcast one and would land beside the recorded contracts, so
            // the collision is staged at the address the redeployment actually targets.
            DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
            address predictedImplementation = vm.computeCreate2Address(
                script.IMPLEMENTATION_SALT(),
                keccak256(
                    abi.encodePacked(
                        type(ProtocolAdapter).creationCode,
                        abi.encode(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR)
                    )
                )
            );
            if (predictedImplementation.code.length == 0) {
                vm.etch(predictedImplementation, hex"01");
            }

            vm.expectRevert();
            script.run({isTestDeployment: false, initialOwner: msg.sender});
        }
    }

    /// @notice Reads the deployments recorded in `deployments.json`.
    /// @return deployments The recorded deployments.
    function _recordedDeployments() internal view returns (Deployment[] memory deployments) {
        deployments = abi.decode(vm.parseJson(vm.readFile(_DEPLOYMENTS_PATH)), (Deployment[]));
    }
}
