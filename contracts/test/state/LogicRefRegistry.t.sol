// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefRegistry} from "../../src/interfaces/ILogicRefRegistry.sol";
import {LogicRefRegistry} from "../../src/state/LogicRefRegistry.sol";
import {LogicRefRegistryMock} from "../mocks/LogicRefRegistry.m.sol";

contract LogicRefRegistryTest is Test {
    bytes32 internal constant _EXAMPLE_LOGIC_REF = bytes32(uint256(1));

    LogicRefRegistryMock internal _logicRefRegistry;

    function setUp() public {
        _logicRefRegistry = new LogicRefRegistryMock();
    }

    function testFuzz_deprecateLogicRef_deprecates_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _logicRefRegistry.deprecateLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefRegistry.Status.Deprecated, "the logic ref should be deprecated");
    }

    function test_deprecateLogicRef_emits_the_LogicRefDeprecated_event() public {
        vm.expectEmit(address(_logicRefRegistry));
        emit ILogicRefRegistry.LogicRefDeprecated({logicRef: _EXAMPLE_LOGIC_REF});
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_deprecated_logic_ref() public {
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDeprecated.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefRegistry)
        );
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_denied_logic_ref() public {
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefRegistry)
        );
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefRegistry.ZeroLogicRefNotAllowed.selector, address(_logicRefRegistry));
        _logicRefRegistry.deprecateLogicRef(bytes32(0));
    }

    function testFuzz_denyLogicRef_denies_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _logicRefRegistry.denyLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefRegistry.Status.Denied, "the logic ref should be denied");
    }

    function test_denyLogicRef_denies_a_deprecated_logic_ref() public {
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Denied, "the logic ref should be denied");
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 1, "the logic ref should be listed once");
    }

    function test_denyLogicRef_emits_the_LogicRefDenied_event() public {
        vm.expectEmit(address(_logicRefRegistry));
        emit ILogicRefRegistry.LogicRefDenied({logicRef: _EXAMPLE_LOGIC_REF});
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_a_denied_logic_ref() public {
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefRegistry)
        );
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefRegistry.ZeroLogicRefNotAllowed.selector, address(_logicRefRegistry));
        _logicRefRegistry.denyLogicRef(bytes32(0));
    }

    function test_nonActiveLogicRefCount_counts_deprecated_and_denied_logic_refs() public {
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 0, "no logic ref should be non-active");

        uint256 n = 10;
        for (uint256 i = 1; i < n; ++i) {
            if (i % 2 == 0) {
                _logicRefRegistry.deprecateLogicRef(bytes32(i));
            } else {
                _logicRefRegistry.denyLogicRef(bytes32(i));
            }
            assertEq(_logicRefRegistry.nonActiveLogicRefCount(), i, "the count should match non-active logic refs");
        }
    }

    function test_nonActiveLogicRefAtIndex_preserves_insertion_order() public {
        uint256 n = 10;
        for (uint256 i = 0; i < n; ++i) {
            if (i % 2 == 0) {
                _logicRefRegistry.deprecateLogicRef(bytes32(n - i));
            } else {
                _logicRefRegistry.denyLogicRef(bytes32(n - i));
            }
        }

        for (uint256 i = 0; i < n; ++i) {
            assertEq(
                _logicRefRegistry.nonActiveLogicRefAtIndex(i), bytes32(n - i), "the non-active logic ref should match"
            );
        }
    }

    function test_setLogicRefStatuses_deprecates_then_denies_a_reference_without_listing_it_twice() public {
        ILogicRefRegistry.StatusUpdate[] memory updates = new ILogicRefRegistry.StatusUpdate[](2);
        updates[0] = ILogicRefRegistry.StatusUpdate(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Deprecated);
        updates[1] = ILogicRefRegistry.StatusUpdate(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Denied);

        _logicRefRegistry.setLogicRefStatuses(updates);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Denied, "both transitions should apply");
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 1);
        assertEq(_logicRefRegistry.nonActiveLogicRefAtIndex(0), _EXAMPLE_LOGIC_REF);
    }

    function testFuzz_setLogicRefStatuses_rolls_back_invalid_transitions(
        ILogicRefRegistry.Status target,
        bool alreadyDenied
    ) public {
        vm.assume(alreadyDenied || target != ILogicRefRegistry.Status.Denied);
        if (alreadyDenied) {
            _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);
        } else {
            _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
        }
        ILogicRefRegistry.Status original = _logicRefRegistry.logicRefStatus(_EXAMPLE_LOGIC_REF);
        bytes32 otherRef = bytes32(uint256(2));
        ILogicRefRegistry.StatusUpdate[] memory updates = new ILogicRefRegistry.StatusUpdate[](2);
        updates[0] = ILogicRefRegistry.StatusUpdate(otherRef, ILogicRefRegistry.Status.Denied);
        updates[1] = ILogicRefRegistry.StatusUpdate(_EXAMPLE_LOGIC_REF, target);

        bytes4 expectedError = target == ILogicRefRegistry.Status.Active
            ? LogicRefRegistry.InvalidLogicRefStatus.selector
            : alreadyDenied
                ? LogicRefRegistry.LogicRefAlreadyDenied.selector
                : LogicRefRegistry.LogicRefAlreadyDeprecated.selector;
        vm.expectRevert(
            target == ILogicRefRegistry.Status.Active
                ? abi.encodeWithSelector(expectedError, target)
                : abi.encodeWithSelector(expectedError, _EXAMPLE_LOGIC_REF)
        );
        _logicRefRegistry.setLogicRefStatuses(updates);

        _assertStatus(otherRef, ILogicRefRegistry.Status.Active, "the earlier update must roll back");
        _assertStatus(_EXAMPLE_LOGIC_REF, original, "the existing status must be preserved");
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 1);
        assertEq(_logicRefRegistry.nonActiveLogicRefAtIndex(0), _EXAMPLE_LOGIC_REF);
    }

    function test_setLogicRefStatuses_rolls_back_repeated_updates() public {
        ILogicRefRegistry.StatusUpdate[] memory updates = new ILogicRefRegistry.StatusUpdate[](2);
        updates[0] = ILogicRefRegistry.StatusUpdate(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Deprecated);
        updates[1] = updates[0];

        vm.expectRevert(abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDeprecated.selector, _EXAMPLE_LOGIC_REF));
        _logicRefRegistry.setLogicRefStatuses(updates);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Active, "the first update must roll back");
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 0);
    }

    function test_setLogicRefStatuses_rolls_back_a_batch_containing_the_zero_reference() public {
        ILogicRefRegistry.StatusUpdate[] memory updates = new ILogicRefRegistry.StatusUpdate[](2);
        updates[0] = ILogicRefRegistry.StatusUpdate(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Deprecated);
        updates[1] = ILogicRefRegistry.StatusUpdate(bytes32(0), ILogicRefRegistry.Status.Denied);

        vm.expectRevert(LogicRefRegistry.ZeroLogicRefNotAllowed.selector);
        _logicRefRegistry.setLogicRefStatuses(updates);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Active, "the earlier update must roll back");
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 0);
    }

    function testFuzz_logicRefStatus_defaults_to_active(bytes32 logicRef) public view {
        _assertStatus(logicRef, ILogicRefRegistry.Status.Active, "an unlisted logic ref should be active");
    }

    function _assertStatus(bytes32 logicRef, ILogicRefRegistry.Status expected, string memory message) internal view {
        assertEq(uint8(_logicRefRegistry.logicRefStatus(logicRef)), uint8(expected), message);
    }
}
