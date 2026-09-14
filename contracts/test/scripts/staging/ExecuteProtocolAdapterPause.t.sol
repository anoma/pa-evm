// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteProtocolAdapterPause} from "../../../script/staging/ExecuteProtocolAdapterPause.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the guards of the staging-only pause script. The pause itself is not exercised here: the script
/// broadcasts as the proxy owner, and forge rejects broadcasts under the prank that makes the sender the owner in the
/// first place. `ProtocolAdapter.unit.t.sol` covers that the owner can pause.
contract ExecuteProtocolAdapterPauseTest is RiscZeroRouterFixture {
    address internal _stagingOwner;
    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();
        _stagingOwner = deployScript.PROXY_OWNER_STAGING();

        (_stagingProxy,,,) = deployScript.run({isProduction: false, isTransitional: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isTransitional: false});
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteProtocolAdapterPause script = new ExecuteProtocolAdapterPause();

        vm.prank(_stagingOwner);
        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy});
    }

    function test_run_reverts_if_the_sender_is_not_the_proxy_owner() public {
        ExecuteProtocolAdapterPause script = new ExecuteProtocolAdapterPause();
        address outsider = makeAddr("outsider");

        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(StagingScript.UnauthorizedSender.selector, outsider));
        script.run({proxy: _stagingProxy});
    }
}
