// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";

import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeploymentsFixture} from "../fixtures/DeploymentsFixture.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the proxy deploy script against a fresh chain. The deployments it records are checked in
/// `Deployments.t.sol` and its promotion gates instead.
contract DeployProtocolAdapterProxyTest is RiscZeroRouterFixture, DeploymentsFixture {
    function test_run_succeeds_for_a_staging_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: false});
    }

    function test_run_succeeds_for_a_production_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: true});
    }

    function test_run_deploys_distinct_proxies_sharing_the_implementation() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address stagingProxy, address stagingImplementation,,) = script.run({isProduction: false});
        (address productionProxy, address productionImplementation,,) = script.run({isProduction: true});

        assertNotEq(stagingProxy, productionProxy, "staging and production proxy addresses are equal");
        assertEq(stagingImplementation, productionImplementation, "staging and production implementations differ");
    }

    function test_run_reverts_if_the_chain_has_a_recorded_deployment() public {
        Deployment[] memory deployments = _recordedDeployments({isProduction: false});

        for (uint256 i = 0; i < deployments.length; ++i) {
            vm.chainId(deployments[i].chainId);

            DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

            vm.expectRevert(
                abi.encodeWithSelector(
                    DeployProtocolAdapterProxy.DeploymentAlreadyRecorded.selector,
                    _environmentName({isProduction: false}),
                    deployments[i].chainId
                )
            );
            script.run({isProduction: false});
        }
    }

    function test_run_reverts_if_the_proxy_is_already_deployed() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address proxy,,,) = script.run({isProduction: false});

        vm.expectRevert(abi.encodeWithSelector(DeployProtocolAdapterProxy.ProxyAlreadyDeployed.selector, proxy));
        script.run({isProduction: false});
    }

    function test_predict_matches_the_addresses_run_deploys() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

        (address predictedProxy, address predictedImplementation) = script.predict({isProduction: false});
        (address proxy, address implementation,,) = script.run({isProduction: false});

        assertEq(proxy, predictedProxy, "staging: prediction differs from the deployed proxy");
        assertEq(
            implementation, predictedImplementation, "staging: prediction differs from the deployed implementation"
        );

        (address predictedProductionProxy,) = script.predict({isProduction: true});
        (address productionProxy,,,) = script.run({isProduction: true});

        assertEq(productionProxy, predictedProductionProxy, "production: prediction differs from the deployed proxy");
    }

    function test_environmentName_names_the_environments() public {
        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();

        assertEq(script.environmentName({isProduction: false}), "staging", "staging environment name differs");
        assertEq(script.environmentName({isProduction: true}), "production", "production environment name differs");
    }

    /// @notice Runs the deploy script for the environment and checks that the proxy lands at the predicted
    /// deterministic address, delegates to a deployed implementation, and is initialized with the environment owner.
    /// @param isProduction Whether to deploy the production or the staging environment proxy.
    function _expectDeployment(bool isProduction) private {
        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address proxy, address implementation,,) = script.run({isProduction: isProduction});

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

        string memory environment = _environmentName(isProduction);

        assertEq(proxy, predicted, string.concat(environment, ": proxy address differs from the prediction"));
        assertGt(implementation.code.length, 0, string.concat(environment, ": implementation is not deployed"));
        assertEq(
            ProtocolAdapter(proxy).getImplementation(),
            implementation,
            string.concat(environment, ": proxy does not delegate to the implementation")
        );
        assertEq(ProtocolAdapter(proxy).owner(), owner, string.concat(environment, ": proxy owner differs"));
    }
}
