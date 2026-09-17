// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {Vm} from "forge-std-1.16.2/src/Vm.sol";

import {DeployProtocolAdapterImplementation} from "../../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../../script/DeployProtocolAdapterProxy.s.sol";
import {
    FinalizeProtocolAdapterStateMigration
} from "../../../script/migration/FinalizeProtocolAdapterStateMigration.s.sol";
import {MigrationScript} from "../../../script/migration/MigrationScript.s.sol";
import {MigrationalProtocolAdapter} from "../../../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {MigrationFixture} from "../../fixtures/MigrationFixture.sol";
import {FinalizeProtocolAdapterStateMigrationMock} from "../../mocks/FinalizeProtocolAdapterStateMigration.m.sol";

/// @notice Checks the completion run against a stand-in for the v1 protocol adapter, after the migration run copied the
/// state.
contract FinalizeProtocolAdapterStateMigrationTest is MigrationFixture {
    function test_run_unpauses_and_leaves_the_proxy_on_the_plain_implementation() public {
        ProtocolAdapter pa = ProtocolAdapter(_proxy);
        _migrationScript.run({isProduction: false});

        _finalizationScript.run({isProduction: false});

        assertFalse(pa.paused(), "the proxy should be unpaused");
        assertEq(pa.owner(), DEFAULT_SENDER, "a staging proxy should keep its owner");
        (address implementation,) = new DeployProtocolAdapterImplementation().predict();
        assertEq(pa.getImplementation(), implementation, "the proxy should run the plain implementation");
        _migrationScript.verify({isProduction: false});

        // The copy-in is gone with the implementation that carried it.
        vm.prank(DEFAULT_SENDER);
        vm.expectRevert();
        MigrationalProtocolAdapter(_proxy).migrateNullifierSet(1);
    }

    function test_run_transfers_a_production_proxy_to_the_production_proxy_owner() public {
        _migrationScript.run({isProduction: false});

        _finalizationScript.run({isProduction: true});

        assertEq(
            ProtocolAdapter(_proxy).owner(),
            new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION(),
            "the production proxy owner should own the proxy"
        );
        _migrationScript.verify({isProduction: true});
    }

    function test_run_unpauses_before_it_upgrades_and_transfers() public {
        _migrationScript.run({isProduction: false});

        vm.recordLogs();
        _finalizationScript.run({isProduction: true});

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
                MigrationalProtocolAdapter.CommitmentCountMismatch.selector, _COMMITMENT_COUNT, uint256(0)
            )
        );
        _finalizationScript.run({isProduction: false});
    }

    function test_run_reverts_for_a_proxy_that_copies_from_another_v1_protocol_adapter() public {
        address other = makeAddr("other v1 protocol adapter");
        FinalizeProtocolAdapterStateMigration script =
            new FinalizeProtocolAdapterStateMigrationMock({protocolAdapterV1: other, proxy: _proxy});

        vm.expectRevert(abi.encodeWithSelector(MigrationScript.ProtocolAdapterV1Mismatch.selector, other, address(_v1)));
        script.run({isProduction: false});
    }

    function test_run_reverts_on_a_chain_that_records_no_v1_protocol_adapter() public {
        FinalizeProtocolAdapterStateMigration script = new FinalizeProtocolAdapterStateMigration();
        uint256 chainId = 31337;
        vm.chainId(chainId);

        vm.expectRevert(abi.encodeWithSelector(MigrationScript.ProtocolAdapterV1NotRecorded.selector, chainId));
        script.run({isProduction: false});
    }

    function test_run_reverts_if_the_environment_records_no_proxy() public {
        FinalizeProtocolAdapterStateMigration script =
            new FinalizeProtocolAdapterStateMigrationMock({protocolAdapterV1: address(_v1), proxy: address(0)});

        vm.expectRevert(
            abi.encodeWithSelector(MigrationScript.DeploymentNotRecorded.selector, "production", block.chainid)
        );
        script.run({isProduction: true});
    }

    function test_run_resumes_a_run_that_stopped_after_the_unpause() public {
        bytes32[] memory sides = _v1.commitmentTreeSides();

        vm.startPrank(DEFAULT_SENDER);
        MigrationalProtocolAdapter(_proxy).migrateCommitmentTree(sides);
        MigrationalProtocolAdapter(_proxy).migrateNullifierSet(_NULLIFIER_COUNT);
        MigrationalProtocolAdapter(_proxy).unpause();
        vm.stopPrank();

        _finalizationScript.run({isProduction: true});

        _migrationScript.verify({isProduction: true});
    }
}
