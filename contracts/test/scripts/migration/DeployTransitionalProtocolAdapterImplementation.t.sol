// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";

import {
    DeployTransitionalProtocolAdapterImplementation
} from "../../../script/migration/DeployTransitionalProtocolAdapterImplementation.s.sol";
import {TransitionalProtocolAdapter} from "../../../src/TransitionalProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the transitional implementation deploy script.
contract DeployTransitionalProtocolAdapterImplementationTest is RiscZeroRouterFixture {
    /// @dev The local development chain, which has no RISC Zero deployment.
    uint256 internal constant _UNSUPPORTED_CHAIN_ID = 31337;

    /// @dev Arbitrum Sepolia, a supported network that never ran a v1 protocol adapter.
    uint256 internal constant _CHAIN_ID_WITHOUT_V1 = 421614;

    function test_run_deploys_deterministically() public {
        _deployRiscZeroRouter();

        DeployTransitionalProtocolAdapterImplementation script = new DeployTransitionalProtocolAdapterImplementation();
        address implementation = script.run();

        SupportedNetworks.Data memory data = getRouterData();
        address protocolAdapterV1 = script.getProtocolAdapterV1(block.chainid);
        address predicted = vm.computeCreate2Address(
            script.IMPLEMENTATION_SALT(),
            keccak256(
                abi.encodePacked(
                    type(TransitionalProtocolAdapter).creationCode,
                    abi.encode(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR, protocolAdapterV1)
                )
            )
        );

        assertEq(implementation, predicted, "implementation address differs from the prediction");
        assertGt(implementation.code.length, 0, "implementation is not deployed");
        assertEq(
            TransitionalProtocolAdapter(implementation).getProtocolAdapterV1(),
            protocolAdapterV1,
            "implementation records a different v1 protocol adapter"
        );
    }

    function test_run_reverts_if_the_implementation_is_already_deployed() public {
        _deployRiscZeroRouter();

        DeployTransitionalProtocolAdapterImplementation script = new DeployTransitionalProtocolAdapterImplementation();
        address implementation = script.run();

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployTransitionalProtocolAdapterImplementation.ImplementationAlreadyDeployed.selector, implementation
            )
        );
        script.run();
    }

    function test_predict_matches_the_address_run_deploys() public {
        _deployRiscZeroRouter();

        DeployTransitionalProtocolAdapterImplementation script = new DeployTransitionalProtocolAdapterImplementation();
        (address predicted,) = script.predict(block.chainid);

        assertEq(script.run(), predicted, "prediction differs from the deployed implementation");
    }

    function test_predict_reverts_on_an_unsupported_network() public {
        DeployTransitionalProtocolAdapterImplementation script = new DeployTransitionalProtocolAdapterImplementation();

        vm.expectRevert(abi.encodeWithSelector(SupportedNetworks.UnsupportedNetwork.selector, _UNSUPPORTED_CHAIN_ID));
        script.predict(_UNSUPPORTED_CHAIN_ID);
    }

    function test_predict_reverts_on_a_chain_without_a_v1_protocol_adapter() public {
        DeployTransitionalProtocolAdapterImplementation script = new DeployTransitionalProtocolAdapterImplementation();

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployTransitionalProtocolAdapterImplementation.NoProtocolAdapterV1.selector, _CHAIN_ID_WITHOUT_V1
            )
        );
        script.predict(_CHAIN_ID_WITHOUT_V1);
    }
}
