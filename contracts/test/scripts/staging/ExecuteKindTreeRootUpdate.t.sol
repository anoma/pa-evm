// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteKindTreeRootUpdate} from "../../../script/staging/ExecuteKindTreeRootUpdate.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only kind tree root update script.
contract ExecuteKindTreeRootUpdateTest is RiscZeroRouterFixture {
    bytes32 internal constant _NEW_KIND_TREE_ROOT = keccak256("new kind tree root");

    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_sets_the_kind_tree_root() public {
        new ExecuteKindTreeRootUpdate().run({proxy: _stagingProxy, newKindTreeRoot: _NEW_KIND_TREE_ROOT});

        assertEq(
            ProtocolAdapter(_stagingProxy).getKindTreeRoot(),
            _NEW_KIND_TREE_ROOT,
            "the proxy holds another kind tree root"
        );
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteKindTreeRootUpdate script = new ExecuteKindTreeRootUpdate();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, newKindTreeRoot: _NEW_KIND_TREE_ROOT});
    }
}
