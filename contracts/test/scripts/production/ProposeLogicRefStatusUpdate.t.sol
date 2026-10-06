// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ProductionScript} from "../../../script/production/ProductionScript.s.sol";
import {ProposeLogicRefStatusUpdate} from "../../../script/production/ProposeLogicRefStatusUpdate.s.sol";
import {ILogicRefRegistry} from "../../../src/interfaces/ILogicRefRegistry.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";

/// @notice Checks the production-only status update proposal script against a Safe-owned production proxy.
/// Outside broadcast mode, the script simulates the Safe executing the updates, so the proxy must store the requested
/// statuses.
contract ProposeLogicRefStatusUpdateTest is RiscZeroRouterFixture, SafeFixture {
    bytes32 internal constant _DENIED_LOGIC_REF = keccak256("denied logic ref");
    bytes32 internal constant _DEPRECATED_LOGIC_REF = keccak256("deprecated logic ref");

    address internal _owner;
    address internal _safe;
    address internal _productionProxy;
    address internal _stagingProxy;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, deployScript.PROXY_OWNER_PRODUCTION());

        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
    }

    function test_run_updates_the_logic_ref_statuses() public {
        ILogicRefRegistry.StatusUpdate[] memory updates = _updates();
        vm.expectEmit(_productionProxy);
        emit ILogicRefRegistry.LogicRefDenied({logicRef: _DENIED_LOGIC_REF});
        vm.expectEmit(_productionProxy);
        emit ILogicRefRegistry.LogicRefDeprecated({logicRef: _DEPRECATED_LOGIC_REF});

        new ProposeLogicRefStatusUpdate().run({proxy: _productionProxy, proposer: _owner, updates: updates});

        ILogicRefRegistry registry = ILogicRefRegistry(_productionProxy);
        assertEq(uint8(registry.logicRefStatus(_DENIED_LOGIC_REF)), uint8(ILogicRefRegistry.Status.Denied));
        assertEq(uint8(registry.logicRefStatus(_DEPRECATED_LOGIC_REF)), uint8(ILogicRefRegistry.Status.Deprecated));
        assertEq(registry.nonActiveLogicRefCount(), 2);
    }

    function test_run_reverts_if_the_proxy_is_not_a_production_deployment() public {
        ProposeLogicRefStatusUpdate script = new ProposeLogicRefStatusUpdate();

        vm.expectRevert(abi.encodeWithSelector(ProductionScript.NotAProductionDeployment.selector, _stagingProxy));
        script.run({proxy: _stagingProxy, proposer: _owner, updates: _updates()});
    }

    function test_run_reverts_if_the_simulated_update_fails() public {
        ProposeLogicRefStatusUpdate script = new ProposeLogicRefStatusUpdate();

        // The protocol adapter rejects the zero logic ref, so the simulated Safe execution fails.
        ILogicRefRegistry.StatusUpdate[] memory updates = new ILogicRefRegistry.StatusUpdate[](1);
        updates[0] = ILogicRefRegistry.StatusUpdate({logicRef: bytes32(0), status: ILogicRefRegistry.Status.Denied});

        vm.expectRevert(ProductionScript.TransactionSimulationFailed.selector);
        script.run({proxy: _productionProxy, proposer: _owner, updates: updates});
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
