// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {ExecuteLogicRefDenial} from "../../../script/staging/ExecuteLogicRefDenial.s.sol";
import {StagingScript} from "../../../script/staging/StagingScript.s.sol";
import {ILogicRefDenylist} from "../../../src/interfaces/ILogicRefDenylist.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";

/// @notice Checks the staging-only logic reference denial script.
contract ExecuteLogicRefDenialTest is RiscZeroRouterFixture {
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

    function test_run_denies_the_logic_refs() public {
        new ExecuteLogicRefDenial().run({proxy: _stagingProxy, logicRefs: _logicRefs()});

        ILogicRefDenylist denylist = ILogicRefDenylist(_stagingProxy);
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

    function test_run_reverts_if_the_proxy_is_not_a_staging_deployment() public {
        ExecuteLogicRefDenial script = new ExecuteLogicRefDenial();

        vm.expectRevert(abi.encodeWithSelector(StagingScript.NotAStagingDeployment.selector, _productionProxy));
        script.run({proxy: _productionProxy, logicRefs: _logicRefs()});
    }

    function _logicRefs() internal pure returns (ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) {
        logicRefs = new ILogicRefDenylist.DeniedLogicRef[](3);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DENIED_LOGIC_REF, consumed: true});
        logicRefs[1] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DENIED_LOGIC_REF, consumed: false});
        logicRefs[2] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DEPRECATED_LOGIC_REF, consumed: false});
    }
}
