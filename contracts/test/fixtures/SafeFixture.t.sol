// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Safe as SafeSmartAccount} from "safe-smart-account-1.5.0/contracts/Safe.sol";

import {SafeFixture} from "./SafeFixture.sol";

/// @notice Checks the Safe deployment and identification helpers of the fixture.
contract SafeFixtureTest is SafeFixture {
    function test_deploySafeAt_installs_a_safe_at_the_target() public {
        address target = makeAddr("target");

        assertEq(_deploySafeAt(makeAddr("safe owner"), target), target);
        assertTrue(_isSafe(target));
    }

    function test_isSafe_identifies_a_safe() public {
        assertTrue(_isSafe(_deploySafe(makeAddr("safe owner"))));
    }

    function test_isSafe_rejects_an_eoa() public {
        assertFalse(_isSafe(makeAddr("eoa")));
    }

    function test_isSafe_rejects_a_safe_singleton_without_owners() public {
        // The singleton locks itself with a threshold of one but has no owners, so the linkage fails.
        assertFalse(_isSafe(address(new SafeSmartAccount())));
    }

    function test_isSafe_rejects_a_contract_without_the_safe_getters() public view {
        assertFalse(_isSafe(address(this)));
    }
}
