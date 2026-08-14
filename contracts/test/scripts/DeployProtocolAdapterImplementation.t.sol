// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";

import {DeployProtocolAdapterImplementation} from "../../script/DeployProtocolAdapterImplementation.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the implementation deploy script.
contract DeployProtocolAdapterImplementationTest is RiscZeroRouterFixture {
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
}
