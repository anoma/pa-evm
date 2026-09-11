// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ProductionScript} from "../../../script/production/ProductionScript.s.sol";
import {ProposeProtocolAdapterPause} from "../../../script/production/ProposeProtocolAdapterPause.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";

/// @notice Checks the production-only pause proposal script against a Safe-owned production proxy. Outside broadcast
/// mode, the script simulates the Safe executing the pause, so the proxy must end up paused.
contract ProposeProtocolAdapterPauseTest is RiscZeroRouterFixture, SafeFixture {
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

        (_productionProxy,,,) = deployScript.run({isProduction: true, isTransitional: false});
        (_stagingProxy,,,) = deployScript.run({isProduction: false, isTransitional: false});
    }

    function test_run_pauses_the_protocol_adapter() public {
        vm.expectEmit(_productionProxy);
        emit Pausable.Paused(_safe);

        new ProposeProtocolAdapterPause().run({proxy: _productionProxy, proposer: _owner});

        assertTrue(ProtocolAdapter(_productionProxy).paused(), "the proxy should be paused");
    }

    function test_run_reverts_if_the_proxy_is_not_a_production_deployment() public {
        ProposeProtocolAdapterPause script = new ProposeProtocolAdapterPause();

        vm.expectRevert(abi.encodeWithSelector(ProductionScript.NotAProductionDeployment.selector, _stagingProxy));
        script.run({proxy: _stagingProxy, proposer: _owner});
    }

    function test_run_reverts_if_the_simulated_pause_fails() public {
        ProposeProtocolAdapterPause script = new ProposeProtocolAdapterPause();

        // The protocol adapter rejects a second pause, so the simulated Safe execution fails.
        vm.prank(_safe);
        ProtocolAdapter(_productionProxy).pause();

        vm.expectRevert(ProductionScript.TransactionSimulationFailed.selector);
        script.run({proxy: _productionProxy, proposer: _owner});
    }
}
