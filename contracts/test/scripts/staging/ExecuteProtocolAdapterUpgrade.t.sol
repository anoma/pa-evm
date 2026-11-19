// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.1.0/contracts/src/RiscZeroVerifierRouter.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {RecordedDeploymentsScript} from "../../../script/RecordedDeploymentsScript.s.sol";
import {ExecuteProtocolAdapterUpgrade} from "../../../script/staging/ExecuteProtocolAdapterUpgrade.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {DeployProtocolAdapterProxyMock} from "../../mocks/DeployProtocolAdapterProxy.m.sol";
import {ExecuteProtocolAdapterUpgradeMock} from "../../mocks/ExecuteProtocolAdapterUpgrade.m.sol";

/// @notice Checks the staging-only upgrade execution script. `ProtocolAdapter.upgrade.t.sol` covers the upgrade
/// mechanism itself.
contract ExecuteProtocolAdapterUpgradeTest is RiscZeroRouterFixture {
    address internal _stagingProxy;
    address internal _productionProxy;
    address internal _implementation;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxyMock deployScript = new DeployProtocolAdapterProxyMock();

        (_stagingProxy, _implementation,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_upgrades_the_proxy() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgradeMock(_stagingProxy);

        vm.expectEmit(_stagingProxy);
        emit IERC1967.Upgraded(_implementation);
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgradeMock(_productionProxy);

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_implementation_is_not_deployed() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgradeMock(_stagingProxy);

        vm.etch(_implementation, "");

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployProtocolAdapterImplementation.ImplementationNotDeployed.selector, _implementation
            )
        );
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_proxy_is_not_recorded() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgradeMock(address(0));

        vm.expectRevert(
            abi.encodeWithSelector(RecordedDeploymentsScript.DeploymentNotRecorded.selector, "staging", block.chainid)
        );
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_risc_zero_verifier_is_stopped() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgradeMock(_stagingProxy);

        _stopRiscZeroVerifier(ProtocolAdapter(_implementation).RISC_ZERO_VERIFIER_SELECTOR());

        vm.expectRevert(
            abi.encodeWithSelector(DeployProtocolAdapterImplementation.RiscZeroVerifierPaused.selector, _implementation)
        );
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_router_has_no_verifier_for_the_selector() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgradeMock(_stagingProxy);

        bytes4 selector = ProtocolAdapter(_implementation).RISC_ZERO_VERIFIER_SELECTOR();
        _removeRiscZeroVerifier(selector);

        vm.expectRevert(abi.encodeWithSelector(RiscZeroVerifierRouter.SelectorRemoved.selector, selector));
        script.run({isProduction: false});
    }
}
