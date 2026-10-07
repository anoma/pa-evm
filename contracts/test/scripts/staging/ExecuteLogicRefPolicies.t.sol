// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteLogicRefPolicies} from "../../../script/staging/ExecuteLogicRefPolicies.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ILogicRefPolicyRegistry} from "../../../src/interfaces/ILogicRefPolicyRegistry.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only logic reference denial script.
contract ExecuteLogicRefPoliciesTest is RiscZeroRouterFixture {
    bytes32 internal constant _DENIED_LOGIC_REF = keccak256("denied logic ref");
    bytes32 internal constant _CREATION_DENIED_LOGIC_REF = keccak256("creation-denied logic ref");

    address internal _stagingProxy;
    address internal _productionProxy;

    function setUp() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
        (_productionProxy,,,) = deployScript.run({isProduction: true, isMigrational: false});
    }

    function test_run_denies_the_logic_refs() public {
        new ExecuteLogicRefPolicies().run({proxy: _stagingProxy, logicRefs: _logicRefs()});

        ILogicRefPolicyRegistry registry = ILogicRefPolicyRegistry(_stagingProxy);
        assertEq(
            uint8(registry.logicRefPolicy(_DENIED_LOGIC_REF)), uint8(ILogicRefPolicyRegistry.LogicRefPolicy.FullyDenied)
        );
        assertEq(
            uint8(registry.logicRefPolicy(_CREATION_DENIED_LOGIC_REF)),
            uint8(ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied)
        );
    }

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteLogicRefPolicies script = new ExecuteLogicRefPolicies();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, logicRefs: _logicRefs()});
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
