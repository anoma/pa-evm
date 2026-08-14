// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ProductionScript} from "../../../script/production/ProductionScript.s.sol";
import {ProposeProtocolAdapterUpgrade} from "../../../script/production/ProposeProtocolAdapterUpgrade.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";

/// @notice Checks the production-only upgrade proposal script against a Safe-owned production proxy. Outside
/// broadcast mode, the script simulates the Safe executing the upgrade, so the proxy must end up on the new
/// implementation.
contract ProposeProtocolAdapterUpgradeTest is RiscZeroRouterFixture, SafeFixture {
    address internal _owner;
    address internal _safe;
    address internal _proxy;
    address internal _stagingProxy;
    address internal _implementation;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, deployScript.PROXY_OWNER_PRODUCTION());

        (_proxy, _implementation) = deployScript.run({isProduction: true});
        (_stagingProxy,) = deployScript.run({isProduction: false});
    }

    function test_run_upgrades_the_proxy() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgrade();

        vm.expectEmit({checkTopic1: true, checkTopic2: false, checkTopic3: false, checkData: true, emitter: _proxy});
        emit IERC1967.Upgraded(_implementation);

        address implementation = script.run({proxy: _proxy, proposer: _owner});

        assertEq(implementation, _implementation, "proposed implementation differs");
        assertEq(ProtocolAdapter(_proxy).implementation(), _implementation, "proxy runs a different implementation");
    }

    function test_run_reverts_if_the_proxy_is_not_a_production_deployment() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgrade();

        vm.expectRevert(abi.encodeWithSelector(ProductionScript.NotAProductionDeployment.selector, _stagingProxy));
        script.run({proxy: _stagingProxy, proposer: _owner});
    }

    function test_run_reverts_if_the_implementation_is_not_deployed() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgrade();

        vm.etch(_implementation, "");

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployProtocolAdapterImplementation.ImplementationNotDeployed.selector, _implementation
            )
        );
        script.run({proxy: _proxy, proposer: _owner});
    }
}
