// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployRiscZeroContracts} from "anoma-risc0-deployments-1.2.1/script/DeployRiscZeroContracts.s.sol";
import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {DeployProtocolAdapterImplementation} from "../script/DeployProtocolAdapterImplementation.s.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

/// @notice Checks the implementation deploy script.
contract DeployProtocolAdapterImplementationTest is SupportedNetworks, Test {
    function test_run_succeeds_for_a_test_deployment() public {
        _deployRiscZeroRouter();

        address implementation = new DeployProtocolAdapterImplementation().run({isTestDeployment: true});

        assertGt(implementation.code.length, 0, "implementation should be deployed");
    }

    function test_run_succeeds_for_a_deterministic_deployment() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterImplementation script = new DeployProtocolAdapterImplementation();
        address implementation = script.run({isTestDeployment: false});

        SupportedNetworks.Data memory data = getRouterData();
        address predicted = vm.computeCreate2Address(
            script.IMPLEMENTATION_SALT(),
            keccak256(
                abi.encodePacked(
                    type(ProtocolAdapter).creationCode,
                    abi.encode(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR)
                )
            )
        );

        assertEq(implementation, predicted, "implementation should land at the predicted CREATE2 address");
        assertGt(implementation.code.length, 0, "implementation should be deployed");
    }

    /// @notice Deploys the RISC Zero stack locally instead of forking a network.
    /// @dev The script resolves the router from the chain ID, so the local router — code and storage, which holds
    /// the registered verifier — is cloned onto the router address of a supported network.
    function _deployRiscZeroRouter() private {
        (RiscZeroVerifierRouter router,,) = new DeployRiscZeroContracts().run({admin: msg.sender, guardian: msg.sender});

        vm.chainId(1); // mainnet
        SupportedNetworks.Data memory data = getRouterData();
        vm.etch(address(data.router), address(router).code);
        vm.copyStorage(address(router), address(data.router));
    }
}
