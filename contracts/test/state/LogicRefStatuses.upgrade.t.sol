// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {ILogicRefStatuses} from "../../src/interfaces/ILogicRefStatuses.sol";
import {LogicRefStatuses} from "../../src/state/LogicRefStatuses.sol";
import {LegacyLogicRefDenylistMock} from "../mocks/LegacyLogicRefDenylist.m.sol";
import {LogicRefStatusesUpgradeMock} from "../mocks/LogicRefStatusesUpgrade.m.sol";

contract LogicRefStatusesUpgradeTest is Test {
    LegacyLogicRefDenylistMock internal _legacy;
    LogicRefStatusesUpgradeMock internal _statuses;

    function setUp() public {
        _legacy = LegacyLogicRefDenylistMock(
            address(
                new ERC1967Proxy(
                    address(new LegacyLogicRefDenylistMock()), abi.encodeCall(LegacyLogicRefDenylistMock.initialize, ())
                )
            )
        );
        _statuses = LogicRefStatusesUpgradeMock(address(_legacy));
    }

    function test_upgrade_has_a_compatible_storage_layout() public {
        Options memory opts;
        opts.referenceContract = "LegacyLogicRefDenylist.m.sol:LegacyLogicRefDenylistMock";
        Upgrades.validateUpgrade("LogicRefStatusesUpgrade.m.sol:LogicRefStatusesUpgradeMock", opts);
    }

    function test_upgrade_preserves_legacy_denials_and_enumeration() public {
        for (uint256 i = 1; i <= 3; ++i) {
            _legacy.denyLogicRef(bytes32(i));
            assertTrue(_legacy.isLogicRefDenied(bytes32(i)), "the legacy ref should be denied");
        }

        _legacy.upgradeToAndCall(address(new LogicRefStatusesUpgradeMock()), "");

        assertEq(_statuses.listedLogicRefCount(), 3, "the legacy list should survive the upgrade");
        for (uint256 i = 1; i <= 3; ++i) {
            bytes32 logicRef = bytes32(i);
            assertEq(
                uint8(_statuses.getLogicRefStatus(logicRef)),
                uint8(ILogicRefStatuses.Status.Denied),
                "the legacy denial should survive the upgrade"
            );
            assertEq(_statuses.listedLogicRefAtIndex(i - 1), logicRef, "the listing order should be preserved");

            vm.expectRevert(
                abi.encodeWithSelector(LogicRefStatuses.DeniedLogicRef.selector, logicRef), address(_statuses)
            );
            _statuses.checkConsumedLogicRef(logicRef);
            vm.expectRevert(
                abi.encodeWithSelector(LogicRefStatuses.DeniedLogicRef.selector, logicRef), address(_statuses)
            );
            _statuses.checkCreatedLogicRef(logicRef);
            vm.expectRevert(
                abi.encodeWithSelector(LogicRefStatuses.LogicRefAlreadyDenied.selector, logicRef), address(_statuses)
            );
            _statuses.deprecateLogicRef(logicRef);
            vm.expectRevert(
                abi.encodeWithSelector(LogicRefStatuses.LogicRefAlreadyDenied.selector, logicRef), address(_statuses)
            );
            _statuses.denyLogicRef(logicRef);
        }

        bytes32 fresh = bytes32(uint256(4));
        assertEq(
            uint8(_statuses.getLogicRefStatus(fresh)),
            uint8(ILogicRefStatuses.Status.Active),
            "an unlisted ref should stay active"
        );
        _statuses.deprecateLogicRef(fresh);
        _statuses.checkConsumedLogicRef(fresh);
        vm.expectRevert(abi.encodeWithSelector(LogicRefStatuses.DeprecatedLogicRef.selector, fresh), address(_statuses));
        _statuses.checkCreatedLogicRef(fresh);
        _statuses.denyLogicRef(fresh);
        assertEq(
            uint8(_statuses.getLogicRefStatus(fresh)),
            uint8(ILogicRefStatuses.Status.Denied),
            "the new ref should be denied"
        );
        assertEq(_statuses.listedLogicRefCount(), 4, "the new ref should be listed once");
        assertEq(_statuses.listedLogicRefAtIndex(3), fresh, "new refs should follow legacy entries");
    }

    function test_upgrade_from_an_empty_denylist_keeps_unlisted_refs_active() public {
        _legacy.upgradeToAndCall(address(new LogicRefStatusesUpgradeMock()), "");

        bytes32 logicRef = bytes32(uint256(1));
        assertEq(_statuses.listedLogicRefCount(), 0, "the list should stay empty");
        assertEq(
            uint8(_statuses.getLogicRefStatus(logicRef)),
            uint8(ILogicRefStatuses.Status.Active),
            "an unlisted ref should stay active"
        );
        _statuses.checkConsumedLogicRef(logicRef);
        _statuses.checkCreatedLogicRef(logicRef);
    }
}
