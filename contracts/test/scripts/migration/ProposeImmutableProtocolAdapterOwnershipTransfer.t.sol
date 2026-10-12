// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";

import {
    ProposeImmutableProtocolAdapterOwnershipTransfer
} from "../../../script/migration/ProposeImmutableProtocolAdapterOwnershipTransfer.s.sol";
import {Parameters} from "../../../script/Parameters.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";
import {ImmutableProtocolAdapterMock} from "../../mocks/ImmutableProtocolAdapter.m.sol";

/// @notice Checks the script that proposes to transfer the immutable protocol adapter, against a stand-in owned by a
/// Safe at the address of the production owner. Outside broadcast mode, the script simulates the Safe executing the
/// transfer, so the stand-in must end up owned by the deployment wallet.
contract ProposeImmutableProtocolAdapterOwnershipTransferTest is SafeFixture {
    address internal _owner;
    address internal _safe;
    ImmutableProtocolAdapterMock internal _immutableProtocolAdapter;
    ProposeImmutableProtocolAdapterOwnershipTransfer internal _script;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _script = new ProposeImmutableProtocolAdapterOwnershipTransfer();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, Parameters.PA_MULTISIG);

        _immutableProtocolAdapter = new ImmutableProtocolAdapterMock(_safe);
    }

    function test_run_transfers_the_immutable_protocol_adapter_to_the_deployment_wallet() public {
        vm.expectEmit(address(_immutableProtocolAdapter));
        emit Ownable.OwnershipTransferred(_safe, Parameters.DEPLOYMENT_WALLET);

        _script.run({immutableProtocolAdapter: address(_immutableProtocolAdapter), proposer: _owner});

        assertEq(
            _immutableProtocolAdapter.owner(),
            Parameters.DEPLOYMENT_WALLET,
            "the immutable protocol adapter should be owned by the deployment wallet"
        );
    }

    function test_run_leaves_the_immutable_protocol_adapter_running() public {
        _script.run({immutableProtocolAdapter: address(_immutableProtocolAdapter), proposer: _owner});

        assertFalse(
            _immutableProtocolAdapter.paused(), "the immutable protocol adapter should not be stopped by the transfer"
        );
    }

    function test_run_lets_the_deployment_wallet_stop_the_immutable_protocol_adapter_after_the_transfer() public {
        _script.run({immutableProtocolAdapter: address(_immutableProtocolAdapter), proposer: _owner});

        vm.prank(Parameters.DEPLOYMENT_WALLET);
        _immutableProtocolAdapter.emergencyStop();

        assertTrue(
            _immutableProtocolAdapter.paused(),
            "the deployment wallet should be able to stop the immutable protocol adapter"
        );
    }

    function test_run_reverts_if_the_protocol_adapter_is_not_an_immutable_protocol_adapter_deployment() public {
        ImmutableProtocolAdapterMock other = new ImmutableProtocolAdapterMock(makeAddr("other owner"));

        vm.expectRevert(
            abi.encodeWithSelector(
                ProposeImmutableProtocolAdapterOwnershipTransfer.NotAnImmutableProtocolAdapterDeployment.selector,
                address(other)
            )
        );
        _script.run({immutableProtocolAdapter: address(other), proposer: _owner});
    }
}
