// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";

import {
    ProposeLegacyProtocolAdapterOwnershipTransfer
} from "../../../script/migration/ProposeLegacyProtocolAdapterOwnershipTransfer.s.sol";
import {Parameters} from "../../../script/Parameters.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";
import {LegacyProtocolAdapterMock} from "../../mocks/LegacyProtocolAdapter.m.sol";

/// @notice Checks the script that proposes to transfer the legacy protocol adapter, against a stand-in owned
/// by a Safe at the address of the production owner. Outside broadcast mode, the script simulates the Safe executing
/// the transfer, so the stand-in must end up owned by the deployment wallet.
contract ProposeLegacyProtocolAdapterOwnershipTransferTest is SafeFixture {
    address internal _owner;
    address internal _safe;
    LegacyProtocolAdapterMock internal _legacyProtocolAdapter;
    ProposeLegacyProtocolAdapterOwnershipTransfer internal _script;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _script = new ProposeLegacyProtocolAdapterOwnershipTransfer();

        _owner = makeAddr("safe owner");
        _safe = _deploySafeAt(_owner, Parameters.PA_MULTISIG);

        _legacyProtocolAdapter = new LegacyProtocolAdapterMock(_safe);
    }

    function test_run_transfers_the_legacy_protocol_adapter_to_the_deployment_wallet() public {
        vm.expectEmit(address(_legacyProtocolAdapter));
        emit Ownable.OwnershipTransferred(_safe, Parameters.DEPLOYMENT_WALLET);

        _script.run({legacyProtocolAdapter: address(_legacyProtocolAdapter), proposer: _owner});

        assertEq(
            _legacyProtocolAdapter.owner(),
            Parameters.DEPLOYMENT_WALLET,
            "the legacy protocol adapter should be owned by the deployment wallet"
        );
    }

    function test_run_leaves_the_legacy_protocol_adapter_running() public {
        _script.run({legacyProtocolAdapter: address(_legacyProtocolAdapter), proposer: _owner});

        assertFalse(
            _legacyProtocolAdapter.paused(), "the legacy protocol adapter should not be stopped by the transfer"
        );
    }

    function test_run_lets_the_deployment_wallet_stop_the_legacy_protocol_adapter_after_the_transfer() public {
        _script.run({legacyProtocolAdapter: address(_legacyProtocolAdapter), proposer: _owner});

        vm.prank(Parameters.DEPLOYMENT_WALLET);
        _legacyProtocolAdapter.emergencyStop();

        assertTrue(
            _legacyProtocolAdapter.paused(), "the deployment wallet should be able to stop the legacy protocol adapter"
        );
    }

    function test_run_reverts_if_the_protocol_adapter_is_not_a_legacy_protocol_adapter_deployment() public {
        LegacyProtocolAdapterMock other = new LegacyProtocolAdapterMock(makeAddr("other owner"));

        vm.expectRevert(
            abi.encodeWithSelector(
                ProposeLegacyProtocolAdapterOwnershipTransfer.NotALegacyProtocolAdapterDeployment.selector,
                address(other)
            )
        );
        _script.run({legacyProtocolAdapter: address(other), proposer: _owner});
    }
}
