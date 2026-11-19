// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.1.0/contracts/src/RiscZeroVerifierRouter.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {ProductionScript} from "../../../script/production/ProductionScript.s.sol";
import {ProposeProtocolAdapterUpgrade} from "../../../script/production/ProposeProtocolAdapterUpgrade.s.sol";
import {RecordedDeploymentsScript} from "../../../script/RecordedDeploymentsScript.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";
import {DeployProtocolAdapterProxyMock} from "../../mocks/DeployProtocolAdapterProxy.m.sol";
import {ProposeProtocolAdapterUpgradeMock} from "../../mocks/ProposeProtocolAdapterUpgrade.m.sol";

/// @notice Checks the production-only upgrade proposal script against a Safe-owned production proxy. Outside
/// broadcast mode, the script simulates the Safe executing the upgrade, so the proxy must end up on the new
/// implementation.
contract ProposeProtocolAdapterUpgradeTest is RiscZeroRouterFixture, SafeFixture {
    address internal _owner;
    address internal _safe;
    address internal _productionProxy;
    address internal _stagingProxy;
    address internal _implementation;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _deployRiscZeroRouter();

        DeployProtocolAdapterProxyMock deployScript = new DeployProtocolAdapterProxyMock();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, deployScript.PROXY_OWNER_PRODUCTION());

        (_productionProxy, _implementation,,) = deployScript.run({isProduction: true, isMigrational: false});
        (_stagingProxy,,,) = deployScript.run({isProduction: false, isMigrational: false});
    }

    function test_run_upgrades_the_proxy() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgradeMock(_productionProxy);

        vm.expectEmit({
            checkTopic1: true, checkTopic2: false, checkTopic3: false, checkData: true, emitter: _productionProxy
        });
        emit IERC1967.Upgraded(_implementation);

        script.run({proposer: _owner});

        assertEq(
            ProtocolAdapter(_productionProxy).getImplementation(),
            _implementation,
            "proxy runs a different implementation"
        );
    }

    function test_run_reverts_if_the_proxy_is_not_a_production_deployment() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgradeMock(_stagingProxy);

        vm.expectRevert(abi.encodeWithSelector(ProductionScript.NotAProductionDeployment.selector, _stagingProxy));
        script.run({proposer: _owner});
    }

    function test_run_reverts_if_the_implementation_is_not_deployed() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgradeMock(_productionProxy);

        vm.etch(_implementation, "");

        vm.expectRevert(
            abi.encodeWithSelector(
                DeployProtocolAdapterImplementation.ImplementationNotDeployed.selector, _implementation
            )
        );
        script.run({proposer: _owner});
    }

    function test_run_reverts_if_the_proxy_is_not_recorded() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgrade();

        vm.expectRevert(
            abi.encodeWithSelector(
                RecordedDeploymentsScript.DeploymentNotRecorded.selector, "production", block.chainid
            )
        );
        script.run({proposer: _owner});
    }

    function test_run_reverts_if_the_risc_zero_verifier_is_stopped() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgradeMock(_productionProxy);

        _stopRiscZeroVerifier(ProtocolAdapter(_implementation).RISC_ZERO_VERIFIER_SELECTOR());

        vm.expectRevert(
            abi.encodeWithSelector(DeployProtocolAdapterImplementation.RiscZeroVerifierPaused.selector, _implementation)
        );
        script.run({proposer: _owner});
    }

    function test_run_reverts_if_the_router_has_no_verifier_for_the_selector() public {
        ProposeProtocolAdapterUpgrade script = new ProposeProtocolAdapterUpgradeMock(_productionProxy);

        bytes4 selector = ProtocolAdapter(_implementation).RISC_ZERO_VERIFIER_SELECTOR();
        _removeRiscZeroVerifier(selector);

        vm.expectRevert(abi.encodeWithSelector(RiscZeroVerifierRouter.SelectorRemoved.selector, selector));
        script.run({proposer: _owner});
    }
}
