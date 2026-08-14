// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {ProposeKindTableUpdate} from "../../script/ProposeKindTableUpdate.s.sol";
import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../fixtures/SafeFixture.sol";

/// @notice Checks the kind table update proposal script against a Safe-owned proxy. Outside broadcast mode, the
/// script simulates the Safe executing the update, so the proxy must end up on the new kind table commitment.
contract ProposeKindTableUpdateTest is RiscZeroRouterFixture, SafeFixture {
    bytes32 internal constant _NEW_KIND_TABLE_COMMITMENT = keccak256("new kind table commitment");

    address internal _owner;
    address internal _safe;
    address internal _proxy;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy deployScript = new DeployProtocolAdapterProxy();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, deployScript.PROD_PROXY_OWNER());

        (_proxy,) = deployScript.run({isTestDeployment: false});
    }

    function test_run_updates_the_kind_table_commitment() public {
        vm.expectEmit({checkTopic1: true, checkTopic2: false, checkTopic3: false, checkData: true, emitter: _proxy});
        emit IProtocolAdapter.KindTableCommitmentUpdated(_NEW_KIND_TABLE_COMMITMENT);

        new ProposeKindTableUpdate()
            .run({proxy: _proxy, safe: _safe, proposer: _owner, newKindTableCommitment: _NEW_KIND_TABLE_COMMITMENT});

        assertEq(
            ProtocolAdapter(_proxy).getKindTableCommitment(),
            _NEW_KIND_TABLE_COMMITMENT,
            "proxy should hold the new kind table commitment"
        );
    }

    function test_run_reverts_if_the_simulated_update_fails() public {
        ProposeKindTableUpdate script = new ProposeKindTableUpdate();

        // The protocol adapter rejects the zero commitment, so the simulated Safe execution fails.
        vm.expectRevert(ProposeKindTableUpdate.KindTableUpdateSimulationFailed.selector);
        script.run({proxy: _proxy, safe: _safe, proposer: _owner, newKindTableCommitment: bytes32(0)});
    }
}
