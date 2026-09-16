// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {Vm} from "forge-std-1.16.2/src/Vm.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {FinalizeMigration} from "../../../script/migration/FinalizeMigration.s.sol";
import {MigrateProtocolAdapterState} from "../../../script/migration/MigrateProtocolAdapterState.s.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {TransitionalProtocolAdapter} from "../../../src/TransitionalProtocolAdapter.sol";
import {MigrationFixture} from "../../fixtures/MigrationFixture.sol";
import {ProtocolAdapterV1Mock} from "../../mocks/ProtocolAdapterV1.m.sol";

/// @notice Checks the completion run against a stand-in for the v1 protocol adapter, after the migration run copied
/// the state. The proxy owner is forge's default sender, because the script's transactions are broadcast and a
/// broadcast cannot be pranked.
contract FinalizeMigrationTest is MigrationFixture {
    ProtocolAdapterV1Mock internal _v1;
    address internal _proxy;
    /// @dev The migration run, which copies the state the completion run checks.
    MigrateProtocolAdapterState internal _migration;
    FinalizeMigration internal _script;

    function setUp() public {
        _deployRiscZeroRouter();

        // The implementation the script upgrades to, at its deterministic address.
        new DeployProtocolAdapterImplementation().run();

        _v1 = _deployStoppedProtocolAdapterV1();
        _proxy = _deployTransitionalProxy(address(_v1));

        _migration = new MigrateProtocolAdapterState();
        _script = new FinalizeMigration();
    }

    function test_run_unpauses_and_leaves_the_proxy_on_the_plain_implementation() public {
        ProtocolAdapter pa = ProtocolAdapter(_proxy);
        _migration.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: false});

        assertFalse(pa.paused(), "the proxy should be unpaused");
        assertEq(pa.owner(), DEFAULT_SENDER, "a staging proxy should keep its owner");
        (address implementation,) = new DeployProtocolAdapterImplementation().predict();
        assertEq(pa.getImplementation(), implementation, "the proxy should run the plain implementation");
        _migration.verify({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: false});

        // The copy-in is gone with the implementation that carried it.
        vm.prank(DEFAULT_SENDER);
        vm.expectRevert();
        TransitionalProtocolAdapter(_proxy).seedNullifierSet(1);
    }

    function test_run_transfers_a_production_proxy_to_the_production_proxy_owner() public {
        _migration.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: true});

        assertEq(
            ProtocolAdapter(_proxy).owner(),
            new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION(),
            "the production proxy owner should own the proxy"
        );
        _migration.verify({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: true});
    }

    function test_run_unpauses_before_it_upgrades_and_transfers() public {
        _migration.run({protocolAdapterV1: address(_v1), proxy: _proxy});

        vm.recordLogs();
        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: true});

        bytes32[3] memory order =
            [Pausable.Unpaused.selector, IERC1967.Upgraded.selector, Ownable.OwnershipTransferred.selector];

        uint256 found;
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length && found < order.length; ++i) {
            if (logs[i].emitter == _proxy && logs[i].topics[0] == order[found]) ++found;
        }

        assertEq(found, order.length, "the run should unpause, upgrade and transfer in that order");
    }

    function test_run_reverts_before_the_state_is_copied() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                TransitionalProtocolAdapter.CommitmentCountMismatch.selector, _COMMITMENT_COUNT, uint256(0)
            )
        );
        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: false});
    }

    function test_run_reverts_for_a_proxy_that_copies_from_another_v1_protocol_adapter() public {
        address other = makeAddr("other v1 protocol adapter");

        vm.expectRevert(
            abi.encodeWithSelector(FinalizeMigration.ProtocolAdapterV1Mismatch.selector, other, address(_v1))
        );
        _script.run({protocolAdapterV1: other, proxy: _proxy, isProduction: false});
    }

    function test_run_resumes_a_run_that_stopped_after_the_unpause() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.startPrank(DEFAULT_SENDER);
        TransitionalProtocolAdapter(_proxy).seedCommitmentTree(sides);
        TransitionalProtocolAdapter(_proxy).seedNullifierSet(_NULLIFIER_COUNT);
        TransitionalProtocolAdapter(_proxy).unpause();
        vm.stopPrank();

        _script.run({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: true});

        _migration.verify({protocolAdapterV1: address(_v1), proxy: _proxy, isProduction: true});
    }
}
