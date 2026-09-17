// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteKindTableUpdate} from "../../../script/staging/ExecuteKindTableUpdate.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only kind table update script.
contract ExecuteKindTableUpdateTest is RiscZeroRouterFixture {
    bytes32 internal constant _NEW_KIND_TABLE_COMMITMENT = keccak256("new kind table commitment");

    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_sets_the_kind_table_commitment() public {
        new ExecuteKindTableUpdate().run({proxy: _stagingProxy, newKindTableCommitment: _NEW_KIND_TABLE_COMMITMENT});

        assertEq(
            ProtocolAdapter(_stagingProxy).getKindTableCommitment(),
            _NEW_KIND_TABLE_COMMITMENT,
            "the proxy holds another kind table commitment"
        );
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteKindTableUpdate script = new ExecuteKindTableUpdate();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, newKindTableCommitment: _NEW_KIND_TABLE_COMMITMENT});
    }
}
