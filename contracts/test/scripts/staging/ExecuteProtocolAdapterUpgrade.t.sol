// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteProtocolAdapterUpgrade} from "../../../script/staging/ExecuteProtocolAdapterUpgrade.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the guards of the staging-only upgrade execution script. The upgrade itself is not exercised
/// here: the script broadcasts as the proxy owner, and forge rejects broadcasts under the prank that makes the
/// sender the owner in the first place. `ProtocolAdapter.upgrade.t.sol` covers the upgrade mechanism itself.
contract ExecuteProtocolAdapterUpgradeTest is RiscZeroRouterFixture {
    address internal _stagingOwner;
    address internal _stagingProxy;
    address internal _productionProxy;
    address internal _implementation;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();
        _stagingOwner = deployScript.PROXY_OWNER_STAGING();

        (_stagingProxy, _implementation,,) = deployScript.run({isProduction: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true});
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgrade();

        vm.prank(_stagingOwner);
        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, newImplementation: _implementation});
    }

    function test_run_reverts_if_the_sender_is_not_the_proxy_owner() public {
        ExecuteProtocolAdapterUpgrade script = new ExecuteProtocolAdapterUpgrade();
        address outsider = makeAddr("outsider");

        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(StagingScript.UnauthorizedSender.selector, outsider));
        script.run({proxy: _stagingProxy, newImplementation: _implementation});
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
