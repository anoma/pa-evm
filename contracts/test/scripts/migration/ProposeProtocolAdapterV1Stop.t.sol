// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";

import {ProposeProtocolAdapterV1Stop} from "../../../script/migration/ProposeProtocolAdapterV1Stop.s.sol";
import {Parameters} from "../../../script/Parameters.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";
import {ProtocolAdapterV1Mock} from "../../mocks/ProtocolAdapterV1.m.sol";

/// @notice Checks the v1 stop proposal script against a stand-in for the v1 protocol adapter, owned by a Safe at the
/// address of the production owner. Outside broadcast mode, the script simulates the Safe executing the stop, so v1 must end
/// up stopped.
contract ProposeProtocolAdapterV1StopTest is SafeFixture {
    address internal _owner;
    address internal _safe;
    ProtocolAdapterV1Mock internal _v1;
    ProposeProtocolAdapterV1Stop internal _script;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _script = new ProposeProtocolAdapterV1Stop();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, Parameters.PROXY_OWNER_PRODUCTION);

        _v1 = new ProtocolAdapterV1Mock(_safe);
    }

    function test_run_stops_the_v1_protocol_adapter() public {
        vm.expectEmit(address(_v1));
        emit Pausable.Paused(_safe);

        _script.run({protocolAdapterV1: address(_v1), proposer: _owner});

        assertTrue(_v1.paused(), "v1 should be stopped");
    }

    function test_run_reverts_if_the_protocol_adapter_is_not_a_v1_deployment() public {
        ProtocolAdapterV1Mock other = new ProtocolAdapterV1Mock(makeAddr("other owner"));

        vm.expectRevert(abi.encodeWithSelector(ProposeProtocolAdapterV1Stop.NotAV1Deployment.selector, address(other)));
        _script.run({protocolAdapterV1: address(other), proposer: _owner});
    }

    function test_run_reverts_if_the_simulated_stop_fails() public {
        // v1 rejects a second stop, so the simulated Safe execution fails.
        vm.prank(_safe);
        _v1.emergencyStop();

        vm.expectRevert(ProposeProtocolAdapterV1Stop.TransactionSimulationFailed.selector);
        _script.run({protocolAdapterV1: address(_v1), proposer: _owner});
    }
}
