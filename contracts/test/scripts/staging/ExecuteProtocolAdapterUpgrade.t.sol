// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteProtocolAdapterUpgrade} from "../../../script/staging/ExecuteProtocolAdapterUpgrade.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only upgrade execution script. `ProtocolAdapter.upgrade.t.sol` covers the upgrade
/// mechanism itself.
contract ExecuteProtocolAdapterUpgradeTest is RiscZeroRouterFixture {
    address internal _stagingProxy;
    address internal _productionProxy;
    address internal _implementation;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        (_stagingProxy, _implementation,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_upgrades_the_proxy() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgrade();

        vm.expectEmit(_stagingProxy);
        emit IERC1967.Upgraded(_implementation);
        script.run({proxy: _stagingProxy, newImplementation: _implementation});
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgrade();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, newImplementation: _implementation});
    }

    function test_run_reverts_if_the_implementation_is_not_deployed() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgrade();

        vm.etch(_implementation, "");

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployProtocolAdapterImplementation.ImplementationNotDeployed.selector, _implementation
            )
        );
        script.run({proxy: _stagingProxy, newImplementation: _implementation});
    }

    function test_run_reverts_if_the_implementation_is_unexpected() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgrade();
        address unexpected = makeAddr("unexpected implementation");

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployProtocolAdapterImplementation.UnexpectedImplementation.selector, _implementation, unexpected
            )
        );
        script.run({proxy: _stagingProxy, newImplementation: unexpected});
    }
}
