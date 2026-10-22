// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.5/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.5/src/SupportedNetworks.sol";

import {RecordedDeployments} from "../../../generated/RecordedDeployments.sol";
import {
    DeployMigrationalProtocolAdapterImplementation
} from "../../../script/migration/DeployMigrationalProtocolAdapterImplementation.s.sol";
import {MigrationalProtocolAdapter} from "../../../src/MigrationalProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the migrational implementation deploy script.
contract DeployMigrationalProtocolAdapterImplementationTest is RiscZeroRouterFixture {
    /// @dev The local development chain, which has no RISC Zero deployment.
    uint256 internal constant _UNSUPPORTED_CHAIN_ID = 31337;

    /// @dev Arbitrum Sepolia, a supported network that never ran a legacy protocol adapter.
    uint256 internal constant _CHAIN_ID_WITHOUT_LEGACY_PROTOCOL_ADAPTER = 421614;

    function test_run_deploys_deterministically() public {
        _deployRiscZeroRouter();

        DeployMigrationalProtocolAdapterImplementation script = new DeployMigrationalProtocolAdapterImplementation();
        address implementation = script.run();

        SupportedNetworks.Data memory data = getRouterData();
        address legacyProtocolAdapter = script.getLegacyProtocolAdapter(block.chainid);
        address predicted = vm.computeCreate2Address(
            script.IMPLEMENTATION_SALT(),
            keccak256(
                abi.encodePacked(
                    type(MigrationalProtocolAdapter).creationCode,
                    abi.encode(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR, legacyProtocolAdapter)
                )
            )
        );

        assertEq(implementation, predicted, "implementation address differs from the prediction");
        assertGt(implementation.code.length, 0, "implementation is not deployed");
        assertEq(
            MigrationalProtocolAdapter(implementation).getLegacyProtocolAdapter(),
            legacyProtocolAdapter,
            "implementation records a different legacy protocol adapter"
        );
    }

    function test_run_reverts_if_the_implementation_is_already_deployed() public {
        _deployRiscZeroRouter();

        DeployMigrationalProtocolAdapterImplementation script = new DeployMigrationalProtocolAdapterImplementation();
        address implementation = script.run();

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployMigrationalProtocolAdapterImplementation.ImplementationAlreadyDeployed.selector, implementation
            )
        );
        script.run();
    }

    function test_predict_matches_the_address_run_deploys() public {
        _deployRiscZeroRouter();

        DeployMigrationalProtocolAdapterImplementation script = new DeployMigrationalProtocolAdapterImplementation();
        (address predicted,) = script.predict(block.chainid);

        assertEq(script.run(), predicted, "prediction differs from the deployed implementation");
    }

    function test_predict_reverts_on_an_unsupported_network() public {
        DeployMigrationalProtocolAdapterImplementation script = new DeployMigrationalProtocolAdapterImplementation();

        vm.expectRevert(abi.encodeWithSelector(SupportedNetworks.UnsupportedNetwork.selector, _UNSUPPORTED_CHAIN_ID));
        script.predict(_UNSUPPORTED_CHAIN_ID);
    }

    function test_predict_reverts_on_a_chain_without_a_legacy_protocol_adapter() public {
        DeployMigrationalProtocolAdapterImplementation script = new DeployMigrationalProtocolAdapterImplementation();

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployMigrationalProtocolAdapterImplementation.NoLegacyProtocolAdapter.selector,
                _CHAIN_ID_WITHOUT_LEGACY_PROTOCOL_ADAPTER
            )
        );
        script.predict(_CHAIN_ID_WITHOUT_LEGACY_PROTOCOL_ADAPTER);
    }

    function test_getLegacyProtocolAdapter_returns_the_recorded_legacy_protocol_adapter_of_every_chain() public {
        DeployMigrationalProtocolAdapterImplementation script = new DeployMigrationalProtocolAdapterImplementation();
        RecordedDeployments.LegacyDeployment[] memory deployments = RecordedDeployments.legacy();
        assertGt(deployments.length, 0, "no legacy protocol adapter is recorded");

        for (uint256 i = 0; i < deployments.length; ++i) {
            assertEq(
                script.getLegacyProtocolAdapter(deployments[i].chainId),
                deployments[i].protocolAdapter,
                "the legacy protocol adapter differs from the record"
            );
        }
    }
}
