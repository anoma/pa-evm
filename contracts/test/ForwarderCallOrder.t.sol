// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ForwarderCallOrder} from "../src/libs/ForwarderCallOrder.sol";

contract ForwarderCallOrderTest is Test {
    bytes32 internal constant _A = keccak256("a");
    bytes32 internal constant _B = keccak256("b");
    bytes32 internal constant _C = keccak256("c");
    bytes32 internal constant _NO_CALLS = keccak256("resource without calls");
    bytes32 internal constant _ABSENT = keccak256("absent");

    function test_order_reverts_on_an_after_tag_that_is_not_in_the_transaction() public {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](1);
        calls[0] = _callAfter(_A, _ABSENT);

        vm.expectRevert(abi.encodeWithSelector(ForwarderCallOrder.AfterTagNotInTransaction.selector, _ABSENT));
        ForwarderCallOrder.order(_tags(), calls);
    }

    function test_order_reverts_on_constraints_that_form_a_cycle() public {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](3);
        calls[0] = _callAfter(_A, _C);
        calls[1] = _callAfter(_B, _A);
        calls[2] = _callAfter(_C, _B);

        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        ForwarderCallOrder.order(_tags(), calls);
    }

    function test_order_reverts_on_a_call_that_must_run_after_itself() public {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](1);
        calls[0] = _callAfter(_A, _A);

        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        ForwarderCallOrder.order(_tags(), calls);
    }

    function test_order_keeps_the_order_of_unconstrained_calls() public pure {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](3);
        calls[0] = _call(_A);
        calls[1] = _call(_B);
        calls[2] = _call(_C);

        _assertSequence(ForwarderCallOrder.order(_tags(), calls), _indices(0, 1, 2));
    }

    function test_order_runs_a_call_after_the_calls_of_its_after_tags() public pure {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](3);
        calls[0] = _callAfter(_A, _C);
        calls[1] = _call(_B);
        calls[2] = _call(_C);

        _assertSequence(ForwarderCallOrder.order(_tags(), calls), _indices(1, 2, 0));
    }

    /// @dev Call B waits for nothing and comes before call C in the list, so it runs first.
    function test_order_runs_a_call_before_the_calls_of_its_before_tags() public pure {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](3);
        calls[0] = _call(_A);
        calls[1] = _call(_B);
        calls[2] = _callBefore(_C, _A);

        _assertSequence(ForwarderCallOrder.order(_tags(), calls), _indices(1, 2, 0));
    }

    /// @dev The second call of resource A has no constraints, but it still waits for the first one.
    function test_order_keeps_the_order_of_the_calls_of_one_resource() public pure {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](3);
        calls[0] = _callAfter(_A, _B);
        calls[1] = _call(_A);
        calls[2] = _call(_B);

        _assertSequence(ForwarderCallOrder.order(_tags(), calls), _indices(2, 0, 1));
    }

    /// @dev The protocol adapter changes the state of all resources before any call runs.
    function test_order_counts_a_resource_without_calls_as_done() public pure {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](2);
        calls[0] = _callAfter(_A, _NO_CALLS);
        calls[1] = _call(_B);

        _assertSequence(ForwarderCallOrder.order(_tags(), calls), _indices(0, 1));
    }

    function test_order_ignores_a_before_tag_that_names_no_call() public pure {
        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](2);
        calls[0] = _call(_A);
        calls[1] = _callBefore(_B, _ABSENT);

        _assertSequence(ForwarderCallOrder.order(_tags(), calls), _indices(0, 1));
    }

    function test_order_returns_no_calls_for_no_calls() public pure {
        assertEq(ForwarderCallOrder.order(_tags(), new ForwarderCallOrder.Constraints[](0)).length, 0);
    }

    /// @dev Draws a rank for each call and adds constraints that agree with the ranks, so that an order exists.
    function testFuzz_order_meets_every_constraint(uint8 callCount, uint256 seed) public pure {
        callCount = uint8(bound(callCount, 1, 12));

        uint256[] memory rank = _permutation({length: callCount, seed: seed});
        bytes32[] memory tags = new bytes32[](callCount);
        for (uint256 i = 0; i < callCount; ++i) {
            tags[i] = keccak256(abi.encode("tag", i));
        }

        ForwarderCallOrder.Constraints[] memory calls = new ForwarderCallOrder.Constraints[](callCount);
        for (uint256 i = 0; i < callCount; ++i) {
            uint256 other = uint256(keccak256(abi.encode(seed, "other", i))) % callCount;
            calls[i].tag = tags[i];
            calls[i].afterTags = new bytes32[](0);
            calls[i].beforeTags = new bytes32[](0);
            if (rank[other] < rank[i]) {
                calls[i].afterTags = _single(tags[other]);
            } else if (rank[other] > rank[i]) {
                calls[i].beforeTags = _single(tags[other]);
            }
        }

        uint256[] memory sequence = ForwarderCallOrder.order(tags, calls);

        uint256[] memory position = new uint256[](callCount);
        bool[] memory seen = new bool[](callCount);
        assertEq(sequence.length, callCount, "every call runs");
        for (uint256 k = 0; k < callCount; ++k) {
            assertFalse(seen[sequence[k]], "no call runs twice");
            seen[sequence[k]] = true;
            position[sequence[k]] = k;
        }
        for (uint256 i = 0; i < callCount; ++i) {
            uint256 other = uint256(keccak256(abi.encode(seed, "other", i))) % callCount;
            if (calls[i].afterTags.length != 0) {
                assertLt(position[other], position[i], "a call runs after its after tags");
            }
            if (calls[i].beforeTags.length != 0) {
                assertLt(position[i], position[other], "a call runs before its before tags");
            }
        }
    }

    function _assertSequence(uint256[] memory actual, uint256[] memory expected) private pure {
        assertEq(actual.length, expected.length, "number of calls");
        for (uint256 i = 0; i < expected.length; ++i) {
            assertEq(actual[i], expected[i], "call at this position");
        }
    }

    /// @notice The tags of the transaction: the three resources with calls and one resource without calls.
    function _tags() private pure returns (bytes32[] memory tags) {
        tags = new bytes32[](4);
        tags[0] = _A;
        tags[1] = _B;
        tags[2] = _C;
        tags[3] = _NO_CALLS;
    }

    function _call(bytes32 tag) private pure returns (ForwarderCallOrder.Constraints memory call) {
        call = ForwarderCallOrder.Constraints({tag: tag, afterTags: new bytes32[](0), beforeTags: new bytes32[](0)});
    }

    function _callAfter(bytes32 tag, bytes32 afterTag)
        private
        pure
        returns (ForwarderCallOrder.Constraints memory call)
    {
        call = ForwarderCallOrder.Constraints({tag: tag, afterTags: _single(afterTag), beforeTags: new bytes32[](0)});
    }

    function _callBefore(bytes32 tag, bytes32 beforeTag)
        private
        pure
        returns (ForwarderCallOrder.Constraints memory call)
    {
        call = ForwarderCallOrder.Constraints({tag: tag, afterTags: new bytes32[](0), beforeTags: _single(beforeTag)});
    }

    function _single(bytes32 tag) private pure returns (bytes32[] memory tags) {
        tags = new bytes32[](1);
        tags[0] = tag;
    }

    function _indices(uint256 a, uint256 b) private pure returns (uint256[] memory indices) {
        indices = new uint256[](2);
        indices[0] = a;
        indices[1] = b;
    }

    function _indices(uint256 a, uint256 b, uint256 c) private pure returns (uint256[] memory indices) {
        indices = new uint256[](3);
        indices[0] = a;
        indices[1] = b;
        indices[2] = c;
    }

    /// @notice Returns a permutation of `0..length` drawn from `seed`.
    function _permutation(uint256 length, uint256 seed) private pure returns (uint256[] memory permutation) {
        permutation = new uint256[](length);
        for (uint256 i = 0; i < length; ++i) {
            permutation[i] = i;
        }
        for (uint256 i = length - 1; i > 0; --i) {
            uint256 j = uint256(keccak256(abi.encode(seed, "swap", i))) % (i + 1);
            (permutation[i], permutation[j]) = (permutation[j], permutation[i]);
        }
    }
}
