// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";

import {RecordedDeployments} from "../../generated/RecordedDeployments.sol";
import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {
    DeployTransitionalProtocolAdapterImplementation
} from "../../script/migration/DeployTransitionalProtocolAdapterImplementation.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the proxy deploy script against a fresh chain. The deployments it records are checked against the
/// chains in the bindings crate, which owns the record.
contract DeployProtocolAdapterProxyTest is RiscZeroRouterFixture {
    /// @dev Arbitrum Sepolia, a supported network that never ran a v1 protocol adapter.
    uint256 internal constant _CHAIN_ID_WITHOUT_V1 = 421614;

    function test_run_succeeds_for_a_staging_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: false, isTransitional: false});
    }

    function test_run_succeeds_for_a_production_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: true, isTransitional: false});
    }

    function test_run_succeeds_for_a_transitional_staging_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: false, isTransitional: true});
    }

    function test_run_succeeds_for_a_transitional_production_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: true, isTransitional: true});
    }

    function test_run_deploys_distinct_proxies_sharing_the_implementation() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address stagingProxy, address stagingImplementation,,) =
            script.run({isProduction: false, isTransitional: false});
        (address productionProxy, address productionImplementation,,) =
            script.run({isProduction: true, isTransitional: false});

        assertNotEq(stagingProxy, productionProxy, "staging and production proxy addresses are equal");
        assertEq(stagingImplementation, productionImplementation, "staging and production implementations differ");
    }

    function test_run_deploys_the_transitional_proxy_apart_from_the_plain_one() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address plainProxy, address plainImplementation,,) = script.run({isProduction: false, isTransitional: false});
        (address transitionalProxy, address transitionalImplementation,,) =
            script.run({isProduction: false, isTransitional: true});

        assertNotEq(plainProxy, transitionalProxy, "plain and transitional proxy addresses are equal");
        assertNotEq(plainImplementation, transitionalImplementation, "plain and transitional implementations are equal");
    }

    function test_run_reverts_if_the_chain_has_a_recorded_deployment() public {
        RecordedDeployments.Deployment[] memory deployments = RecordedDeployments.staging();

        for (uint256 i = 0; i < deployments.length; ++i) {
            uint256 chainId = deployments[i].chainId;
            vm.chainId(chainId);

            DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

            vm.expectRevert(
                abi.encodeWithSelector(
                    DeployProtocolAdapterProxy.DeploymentAlreadyRecorded.selector,
                    script.environmentName({isProduction: false}),
                    chainId
                )
            );
            script.run({isProduction: false, isTransitional: false});
        }
    }

    function test_run_reverts_if_the_proxy_is_already_deployed() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address proxy,,,) = script.run({isProduction: false, isTransitional: false});

        vm.expectRevert(abi.encodeWithSelector(DeployProtocolAdapterProxy.ProxyAlreadyDeployed.selector, proxy));
        script.run({isProduction: false, isTransitional: false});
    }

    function test_run_reverts_for_a_transitional_deployment_on_a_chain_without_a_v1_protocol_adapter() public {
        vm.chainId(_CHAIN_ID_WITHOUT_V1);

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployTransitionalProtocolAdapterImplementation.NoProtocolAdapterV1.selector, _CHAIN_ID_WITHOUT_V1
            )
        );
        script.run({isProduction: false, isTransitional: true});
    }

    function test_predict_matches_the_addresses_run_deploys() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

        (address predictedProxy, address predictedImplementation) =
            script.predict({isProduction: false, isTransitional: false});
        (address proxy, address implementation,,) = script.run({isProduction: false, isTransitional: false});

        assertEq(proxy, predictedProxy, "staging: prediction differs from the deployed proxy");
        assertEq(
            implementation, predictedImplementation, "staging: prediction differs from the deployed implementation"
        );

        (address predictedProductionProxy,) = script.predict({isProduction: true, isTransitional: false});
        (address productionProxy,,,) = script.run({isProduction: true, isTransitional: false});

        assertEq(productionProxy, predictedProductionProxy, "production: prediction differs from the deployed proxy");

        (address predictedTransitionalProxy, address predictedTransitionalImplementation) =
            script.predict({isProduction: false, isTransitional: true});
        (address transitionalProxy, address transitionalImplementation,,) =
            script.run({isProduction: false, isTransitional: true});

        assertEq(
            transitionalProxy, predictedTransitionalProxy, "transitional: prediction differs from the deployed proxy"
        );
        assertEq(
            transitionalImplementation,
            predictedTransitionalImplementation,
            "transitional: prediction differs from the deployed implementation"
        );
    }

    function test_environmentName_names_the_environments() public {
        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

        assertEq(script.environmentName({isProduction: false}), "staging", "staging environment name differs");
        assertEq(script.environmentName({isProduction: true}), "production", "production environment name differs");
    }

    /// @notice Runs the deploy script and checks that the proxy lands at the predicted deterministic address,
    /// delegates to a deployed implementation, is initialized with the environment owner, and starts paused on the
    /// chain's v1 protocol adapter exactly when it is transitional.
    /// @param isProduction Whether to deploy the production or the staging environment proxy.
    /// @param isTransitional Whether to deploy the proxy on the transitional implementation.
    function _expectDeployment(bool isProduction, bool isTransitional) private {
        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address proxy, address implementation,,) =
            script.run({isProduction: isProduction, isTransitional: isTransitional});

        address owner = isProduction ? script.PROXY_OWNER_PRODUCTION() : script.PROXY_OWNER_STAGING();
        address predicted = vm.computeCreate2Address(
            isProduction ? script.PROXY_SALT_PRODUCTION() : script.PROXY_SALT_STAGING(),
            keccak256(
                abi.encodePacked(
                    type(ERC1967Proxy).creationCode,
                    abi.encode(implementation, abi.encodeCall(ProtocolAdapter.initialize, (owner)))
                )
            )
        );

        string memory environment = script.environmentName(isProduction);

        assertEq(proxy, predicted, string.concat(environment, ": proxy address differs from the prediction"));
        assertGt(implementation.code.length, 0, string.concat(environment, ": implementation is not deployed"));
        assertEq(
            ProtocolAdapter(proxy).getImplementation(),
            implementation,
            string.concat(environment, ": proxy does not delegate to the implementation")
        );
        assertEq(ProtocolAdapter(proxy).owner(), owner, string.concat(environment, ": proxy owner differs"));
        assertEq(
            ProtocolAdapter(proxy).paused(),
            isTransitional,
            string.concat(environment, ": proxy must start paused exactly when it is transitional")
        );

        if (isTransitional) {
            assertEq(
                TransitionalProtocolAdapter(proxy).getProtocolAdapterV1(),
                new DeployTransitionalProtocolAdapterImplementation().getProtocolAdapterV1(block.chainid),
                string.concat(environment, ": proxy records a different v1 protocol adapter")
            );
        }
    }
}
