// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteProtocolAdapterPause} from "../../../script/staging/ExecuteProtocolAdapterPause.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only pause script.
contract ExecuteProtocolAdapterPauseTest is RiscZeroRouterFixture {
    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_pauses_the_proxy() public {
        new ExecuteProtocolAdapterPause().run({proxy: _stagingProxy});

        assertTrue(ProtocolAdapter(_stagingProxy).paused(), "the proxy is not paused");
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteProtocolAdapterPause script = new ExecuteProtocolAdapterPause();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy});
    }
}
