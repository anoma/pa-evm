// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";

import {DeployProtocolAdapterImplementation} from "../../script/DeployProtocolAdapterImplementation.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the implementation deploy script.
contract DeployProtocolAdapterImplementationTest is RiscZeroRouterFixture {
    /// @dev The local development chain, which has no RISC Zero deployment.
    uint256 internal constant _UNSUPPORTED_CHAIN_ID = 31337;

    function test_run_deploys_deterministically() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterImplementation script = new DeployProtocolAdapterImplementation();
        address implementation = script.run();

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

        assertEq(implementation, predicted, "implementation address differs from the prediction");
        assertGt(implementation.code.length, 0, "implementation is not deployed");
    }

    function test_run_reverts_if_the_implementation_is_already_deployed() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterImplementation script = new DeployProtocolAdapterImplementation();
        address implementation = script.run();

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployProtocolAdapterImplementation.ImplementationAlreadyDeployed.selector, implementation
            )
        );
        script.run();
    }

    function test_predict_matches_the_address_run_deploys() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterImplementation script = new DeployProtocolAdapterImplementation();
        (address predicted,) = script.predict(block.chainid);

        assertEq(script.run(), predicted, "prediction differs from the deployed implementation");
    }

    function test_predict_reverts_on_an_unsupported_network() public {
        DeployProtocolAdapterImplementation script = new DeployProtocolAdapterImplementation();

        vm.expectRevert(abi.encodeWithSelector(SupportedNetworks.UnsupportedNetwork.selector, _UNSUPPORTED_CHAIN_ID));
        script.predict(_UNSUPPORTED_CHAIN_ID);
    }
}
