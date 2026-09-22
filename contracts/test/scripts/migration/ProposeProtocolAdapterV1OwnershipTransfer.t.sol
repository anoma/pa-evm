// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";

import {
    ProposeProtocolAdapterV1OwnershipTransfer
} from "../../../script/migration/ProposeProtocolAdapterV1OwnershipTransfer.s.sol";
import {Parameters} from "../../../script/Parameters.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";
import {ProtocolAdapterV1Mock} from "../../mocks/ProtocolAdapterV1.m.sol";

/// @notice Checks the v1 ownership transfer proposal script against a stand-in for the v1 protocol adapter, owned by a
/// Safe at the address of the production owner. Outside broadcast mode, the script simulates the Safe executing the
/// transfer, so v1 must end up owned by the deployment wallet.
contract ProposeProtocolAdapterV1OwnershipTransferTest is SafeFixture {
    address internal _owner;
    address internal _safe;
    ProtocolAdapterV1Mock internal _v1;
    ProposeProtocolAdapterV1OwnershipTransfer internal _script;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _script = new ProposeProtocolAdapterV1OwnershipTransfer();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, Parameters.PA_MULTISIG);

        _v1 = new ProtocolAdapterV1Mock(_safe);
    }

    function test_run_transfers_the_v1_protocol_adapter_to_the_deployment_wallet() public {
        vm.expectEmit(address(_v1));
        emit Ownable.OwnershipTransferred(_safe, Parameters.DEPLOYMENT_WALLET);

        _script.run({protocolAdapterV1: address(_v1), proposer: _owner});

        assertEq(_v1.owner(), Parameters.DEPLOYMENT_WALLET, "v1 should be owned by the deployment wallet");
    }

    function test_run_leaves_the_v1_protocol_adapter_running() public {
        _script.run({protocolAdapterV1: address(_v1), proposer: _owner});

        assertFalse(_v1.paused(), "v1 should not be stopped by the transfer");
    }

    function test_run_lets_the_deployment_wallet_stop_v1_after_the_transfer() public {
        _script.run({protocolAdapterV1: address(_v1), proposer: _owner});

        vm.prank(Parameters.DEPLOYMENT_WALLET);
        _v1.emergencyStop();

        assertTrue(_v1.paused(), "the deployment wallet should be able to stop v1");
    }

    function test_run_reverts_if_the_protocol_adapter_is_not_a_v1_deployment() public {
        ProtocolAdapterV1Mock other = new ProtocolAdapterV1Mock(makeAddr("other owner"));

        vm.expectRevert(
            abi.encodeWithSelector(ProposeProtocolAdapterV1OwnershipTransfer.NotAV1Deployment.selector, address(other))
        );
        _script.run({protocolAdapterV1: address(other), proposer: _owner});
    }
}
