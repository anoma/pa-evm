// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ProductionScript} from "../../../script/production/ProductionScript.s.sol";
import {ProposeLogicRefDenial} from "../../../script/production/ProposeLogicRefDenial.s.sol";
import {ILogicRefDenylist} from "../../../src/interfaces/ILogicRefDenylist.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";

/// @notice Checks the production-only logic reference denial proposal script against a Safe-owned production proxy.
/// Outside broadcast mode, the script simulates the Safe executing the denial, so the proxy must end up with each logic
/// reference on its denylist.
contract ProposeLogicRefDenialTest is RiscZeroRouterFixture, SafeFixture {
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

    function test_run_denies_the_logic_refs() public {
        ILogicRefDenylist.DeniedLogicRef[] memory logicRefs = _logicRefs();
        for (uint256 i = 0; i < logicRefs.length; ++i) {
            vm.expectEmit(_productionProxy);
            emit ILogicRefDenylist.LogicRefDenied({logicRef: logicRefs[i].logicRef, consumed: logicRefs[i].consumed});
        }

        new ProposeLogicRefDenial().run({proxy: _productionProxy, proposer: _owner, logicRefs: logicRefs});

        ILogicRefDenylist denylist = ILogicRefDenylist(_productionProxy);
        assertTrue(
            denylist.isLogicRefDenied(_DENIED_LOGIC_REF, true),
            "the denied logic ref should be denied for consumed resources"
        );
        assertTrue(
            denylist.isLogicRefDenied(_DENIED_LOGIC_REF, false),
            "the denied logic ref should be denied for created resources"
        );
        assertTrue(
            denylist.isLogicRefDenied(_DEPRECATED_LOGIC_REF, false),
            "the deprecated logic ref should be denied for created resources"
        );
        assertFalse(
            denylist.isLogicRefDenied(_DEPRECATED_LOGIC_REF, true),
            "the deprecated logic ref should not be denied for consumed resources"
        );
    }

    function test_run_reverts_if_the_proxy_is_not_a_production_deployment() public {
        ProposeLogicRefDenial script = new ProposeLogicRefDenial();

        vm.expectRevert(abi.encodeWithSelector(ProductionScript.NotAProductionDeployment.selector, _stagingProxy));
        script.run({proxy: _stagingProxy, proposer: _owner, logicRefs: _logicRefs()});
    }

    function test_run_reverts_if_the_simulated_denial_fails() public {
        ProposeLogicRefDenial script = new ProposeLogicRefDenial();

        // The protocol adapter rejects the zero logic ref, so the simulated Safe execution fails.
        ILogicRefDenylist.DeniedLogicRef[] memory logicRefs = new ILogicRefDenylist.DeniedLogicRef[](1);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: bytes32(0), consumed: true});

        vm.expectRevert(ProductionScript.TransactionSimulationFailed.selector);
        script.run({proxy: _productionProxy, proposer: _owner, logicRefs: logicRefs});
    }

    function _logicRefs() internal pure returns (ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) {
        logicRefs = new ILogicRefDenylist.DeniedLogicRef[](3);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DENIED_LOGIC_REF, consumed: true});
        logicRefs[1] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DENIED_LOGIC_REF, consumed: false});
        logicRefs[2] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DEPRECATED_LOGIC_REF, consumed: false});
    }
}
