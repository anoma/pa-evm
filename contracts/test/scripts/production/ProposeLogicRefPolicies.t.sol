// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ProductionScript} from "../../../script/production/ProductionScript.s.sol";
import {ProposeLogicRefPolicies} from "../../../script/production/ProposeLogicRefPolicies.s.sol";
import {ILogicRefPolicyRegistry} from "../../../src/interfaces/ILogicRefPolicyRegistry.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";

/// @notice Checks the production-only logic reference denial proposal script against a Safe-owned production proxy.
/// Outside broadcast mode, the script simulates the Safe executing the denial, so the proxy must end up with each logic
/// reference under its requested policy.
contract ProposeLogicRefPoliciesTest is RiscZeroRouterFixture, SafeFixture {
    bytes32 internal constant _DENIED_LOGIC_REF = keccak256("denied logic ref");
    bytes32 internal constant _CREATION_DENIED_LOGIC_REF = keccak256("creation-denied logic ref");

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
        ILogicRefPolicyRegistry.PolicyUpdate[] memory logicRefs = _logicRefs();
        for (uint256 i = 0; i < logicRefs.length; ++i) {
            vm.expectEmit(_productionProxy);
            emit ILogicRefPolicyRegistry.LogicRefPolicyChanged(
                logicRefs[i].logicRef, ILogicRefPolicyRegistry.LogicRefPolicy.Unrestricted, logicRefs[i].policy
            );
        }

        new ProposeLogicRefPolicies().run({proxy: _productionProxy, proposer: _owner, logicRefs: logicRefs});

        ILogicRefPolicyRegistry registry = ILogicRefPolicyRegistry(_productionProxy);
        assertEq(
            uint8(registry.logicRefPolicy(_DENIED_LOGIC_REF)), uint8(ILogicRefPolicyRegistry.LogicRefPolicy.FullyDenied)
        );
        assertEq(
            uint8(registry.logicRefPolicy(_CREATION_DENIED_LOGIC_REF)),
            uint8(ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied)
        );
    }

    function test_run_reverts_if_the_proxy_is_not_a_production_deployment() public {
        ProposeLogicRefPolicies script = new ProposeLogicRefPolicies();

        vm.expectRevert(abi.encodeWithSelector(ProductionScript.NotAProductionDeployment.selector, _stagingProxy));
        script.run({proxy: _stagingProxy, proposer: _owner, logicRefs: _logicRefs()});
    }

    function test_run_reverts_if_the_simulated_denial_fails() public {
        ProposeLogicRefPolicies script = new ProposeLogicRefPolicies();

        // The protocol adapter rejects the zero logic ref, so the simulated Safe execution fails.
        ILogicRefPolicyRegistry.PolicyUpdate[] memory logicRefs = new ILogicRefPolicyRegistry.PolicyUpdate[](1);
        logicRefs[0] = ILogicRefPolicyRegistry.PolicyUpdate({
            logicRef: bytes32(0), policy: ILogicRefPolicyRegistry.LogicRefPolicy.FullyDenied
        });

        vm.expectRevert(ProductionScript.TransactionSimulationFailed.selector);
        script.run({proxy: _productionProxy, proposer: _owner, logicRefs: logicRefs});
    }

    function _logicRefs() internal pure returns (ILogicRefPolicyRegistry.PolicyUpdate[] memory logicRefs) {
        logicRefs = new ILogicRefPolicyRegistry.PolicyUpdate[](2);
        logicRefs[0] =
            ILogicRefPolicyRegistry.PolicyUpdate(_DENIED_LOGIC_REF, ILogicRefPolicyRegistry.LogicRefPolicy.FullyDenied);
        logicRefs[1] = ILogicRefPolicyRegistry.PolicyUpdate(
            _CREATION_DENIED_LOGIC_REF, ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied
        );
    }
}
