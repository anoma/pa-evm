// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteKindTableUpdate} from "../../../script/staging/ExecuteKindTableUpdate.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the guards of the staging-only kind table update script. The update itself is not exercised here:
/// the script broadcasts as the proxy owner, and forge rejects broadcasts under the prank that makes the sender the
/// owner in the first place. `KindTableCommitment.t.sol` covers that the owner can set the commitment.
contract ExecuteKindTableUpdateTest is RiscZeroRouterFixture {
    bytes32 internal constant _NEW_KIND_TABLE_COMMITMENT = keccak256("new kind table commitment");

    address internal _stagingOwner;
    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();
        new DeployProtocolAdapterImplementation().run();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();
        _stagingOwner = deployScript.PROXY_OWNER_STAGING();

        (_stagingProxy,,,) = deployScript.run({isProduction: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true});
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteKindTableUpdate script = new ExecuteKindTableUpdate();

        vm.prank(_stagingOwner);
        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, newKindTableCommitment: _NEW_KIND_TABLE_COMMITMENT});
    }

    function test_run_reverts_if_the_sender_is_not_the_proxy_owner() public {
        ExecuteKindTableUpdate script = new ExecuteKindTableUpdate();
        address outsider = makeAddr("outsider");

        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(StagingScript.UnauthorizedSender.selector, outsider));
        script.run({proxy: _stagingProxy, newKindTableCommitment: _NEW_KIND_TABLE_COMMITMENT});
    }
}
