// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteLogicRefStatusUpdate} from "../../../script/staging/ExecuteLogicRefStatusUpdate.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ILogicRefRegistry} from "../../../src/interfaces/ILogicRefRegistry.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only logic reference status update script.
contract ExecuteLogicRefStatusUpdateTest is RiscZeroRouterFixture {
    bytes32 internal constant _DENIED_LOGIC_REF = keccak256("denied logic ref");
    bytes32 internal constant _DEPRECATED_LOGIC_REF = keccak256("deprecated logic ref");

    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_updates_the_logic_ref_statuses() public {
        new ExecuteLogicRefStatusUpdate().run({proxy: _stagingProxy, updates: _updates()});

        ILogicRefRegistry registry = ILogicRefRegistry(_stagingProxy);
        assertEq(uint8(registry.logicRefStatus(_DENIED_LOGIC_REF)), uint8(ILogicRefRegistry.Status.Denied));
        assertEq(uint8(registry.logicRefStatus(_DEPRECATED_LOGIC_REF)), uint8(ILogicRefRegistry.Status.Deprecated));
        assertEq(registry.nonActiveLogicRefCount(), 2);
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteLogicRefStatusUpdate script = new ExecuteLogicRefStatusUpdate();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, updates: _updates()});
    }

    function _updates() internal pure returns (ILogicRefRegistry.StatusUpdate[] memory updates) {
        updates = new ILogicRefRegistry.StatusUpdate[](2);
        updates[0] =
            ILogicRefRegistry.StatusUpdate({logicRef: _DENIED_LOGIC_REF, status: ILogicRefRegistry.Status.Denied});
        updates[1] = ILogicRefRegistry.StatusUpdate({
            logicRef: _DEPRECATED_LOGIC_REF, status: ILogicRefRegistry.Status.Deprecated
        });
    }
}
