// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {DirectedGraph} from "../src/libs/DirectedGraph.sol";
import {ForwarderCallOrdering} from "../src/libs/ForwarderCallOrdering.sol";

contract ForwarderCallOrderingTest is Test {
    /// @dev Passes the payload in calldata, where the protocol adapter reads it.
    function hasOrderingConstraintsInCalldata(IProtocolAdapter.ExpirableBlob[] calldata externalPayload)
        external
        pure
        returns (bool constrained)
    {
        constrained = ForwarderCallOrdering.hasOrderingConstraints(externalPayload);
    }

    /// @dev The offsets of the tag lists are the fourth and fifth words of the encoded call.
    function test_hasOrderingConstraints_reverts_on_a_blob_too_short_for_the_tag_list_offsets() public {
        IProtocolAdapter.ExpirableBlob[] memory externalPayload = _payload(new bytes(3 * 32));

        vm.expectRevert();
        this.hasOrderingConstraintsInCalldata(externalPayload);
    }

    function test_order_reverts_on_more_call_lists_than_a_graph_has_nodes() public {
        uint256 callListCount = DirectedGraph.MAX_NODE_COUNT + 1;

        vm.expectRevert(
            abi.encodeWithSelector(ForwarderCallOrdering.TooManyCallCarryingResources.selector, callListCount)
        );
        ForwarderCallOrdering.order(new ForwarderCallOrdering.CallList[](callListCount));
    }

    function test_checkCallOrder_reverts_on_an_order_of_another_length() public {
        vm.expectRevert(ForwarderCallOrdering.CallOrderNotAPermutation.selector);
        ForwarderCallOrdering.checkCallOrder(_threeCallLists(), _indices(0, 1));
    }

    function test_checkCallOrder_reverts_on_a_call_list_named_twice() public {
        vm.expectRevert(ForwarderCallOrdering.CallOrderNotAPermutation.selector);
        ForwarderCallOrdering.checkCallOrder(_threeCallLists(), _indices(0, 0, 2));
    }

    function test_checkCallOrder_reverts_on_an_index_outside_the_call_lists() public {
        vm.expectRevert(ForwarderCallOrdering.CallOrderNotAPermutation.selector);
        ForwarderCallOrdering.checkCallOrder(_threeCallLists(), _indices(0, 1, 3));
    }

    function test_checkCallOrder_accepts_an_order_that_runs_a_later_action_first() public pure {
        ForwarderCallOrdering.checkCallOrder(_threeCallLists(), _indices(2, 0, 1));
    }

    function test_checkCallOrder_reverts_on_an_order_that_splits_an_action() public {
        vm.expectRevert(abi.encodeWithSelector(ForwarderCallOrdering.CallOrderSplitsAction.selector, 0));
        ForwarderCallOrdering.checkCallOrder(_threeCallLists(), _indices(0, 2, 1));
    }

    /// @dev The order splits action 0 and also names call list 0 twice. The permutation step comes first.
    function test_checkCallOrder_reports_a_call_list_named_twice_before_a_split_action() public {
        vm.expectRevert(ForwarderCallOrdering.CallOrderNotAPermutation.selector);
        ForwarderCallOrdering.checkCallOrder(_threeCallLists(), _indices(0, 2, 0));
    }

    function test_checkCallOrder_reverts_on_an_order_that_runs_a_call_list_before_its_after_tag() public {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        bytes32 afterTag = callLists[1].resourceTag;
        _constrain({callList: callLists[0], afterTag: afterTag, beforeTag: bytes32(0)});

        vm.expectRevert(abi.encodeWithSelector(ForwarderCallOrdering.CallOrderViolatesConstraint.selector, afterTag));
        ForwarderCallOrdering.checkCallOrder(callLists, _indices(0, 1, 2));
    }

    function test_checkCallOrder_reverts_on_an_order_that_runs_a_call_list_after_its_before_tag() public {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        bytes32 beforeTag = callLists[0].resourceTag;
        _constrain({callList: callLists[1], afterTag: bytes32(0), beforeTag: beforeTag});

        vm.expectRevert(abi.encodeWithSelector(ForwarderCallOrdering.CallOrderViolatesConstraint.selector, beforeTag));
        ForwarderCallOrdering.checkCallOrder(callLists, _indices(0, 1, 2));
    }

    /// @dev The sort would run action 1 first. A carried order keeps the actions in the order of the transaction, so
    /// no carried order meets this constraint.
    function test_checkCallOrder_reverts_on_a_constraint_that_runs_a_later_action_first() public {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        bytes32 afterTag = callLists[2].resourceTag;
        _constrain({callList: callLists[0], afterTag: afterTag, beforeTag: bytes32(0)});

        vm.expectRevert(abi.encodeWithSelector(ForwarderCallOrdering.CallOrderViolatesConstraint.selector, afterTag));
        ForwarderCallOrdering.checkCallOrder(callLists, _indices(0, 1, 2));
    }

    function test_checkCallOrder_reverts_on_a_call_list_that_must_run_after_itself() public {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        bytes32 ownTag = callLists[0].resourceTag;
        _constrain({callList: callLists[0], afterTag: ownTag, beforeTag: bytes32(0)});

        vm.expectRevert(abi.encodeWithSelector(ForwarderCallOrdering.CallOrderViolatesConstraint.selector, ownTag));
        ForwarderCallOrdering.checkCallOrder(callLists, _indices(0, 1, 2));
    }

    function test_checkCallOrder_reverts_on_an_after_tag_that_names_no_call_list() public {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        bytes32 unknownTag = keccak256("not a call-carrying resource");
        _constrain({callList: callLists[0], afterTag: unknownTag, beforeTag: bytes32(0)});

        vm.expectRevert(
            abi.encodeWithSelector(ForwarderCallOrdering.AfterTagNamesNoCallCarryingResource.selector, unknownTag)
        );
        ForwarderCallOrdering.checkCallOrder(callLists, _indices(0, 1, 2));
    }

    function testFuzz_hasOrderingConstraints_agrees_with_the_lengths_of_the_tag_lists(
        uint8 afterTagCount,
        uint8 beforeTagCount
    ) public view {
        afterTagCount = uint8(bound(afterTagCount, 0, 3));
        beforeTagCount = uint8(bound(beforeTagCount, 0, 3));
        IProtocolAdapter.ExpirableBlob[] memory externalPayload = _payload(
            abi.encode(address(1), hex"0102", hex"03", new bytes32[](afterTagCount), new bytes32[](beforeTagCount))
        );

        assertEq(
            this.hasOrderingConstraintsInCalldata(externalPayload),
            afterTagCount + beforeTagCount != 0,
            "constrained if a tag list is not empty"
        );
    }

    /// @dev The call lists carry no calls, so their order is the order of their indices.
    function test_order_accepts_as_many_call_lists_as_a_graph_has_nodes() public pure {
        uint256 maxNodeCount = DirectedGraph.MAX_NODE_COUNT;

        uint256[] memory orderedIndices =
            ForwarderCallOrdering.order(new ForwarderCallOrdering.CallList[](maxNodeCount));

        assertEq(orderedIndices.length, maxNodeCount, "number of ordered call lists");
        for (uint256 position = 0; position < maxNodeCount; ++position) {
            assertEq(orderedIndices[position], position, "ordered call list");
        }
    }

    /// @dev The call list of the first action waits for the call list of the last action.
    function test_order_accepts_more_actions_than_a_graph_has_nodes() public pure {
        uint256 actionCount = DirectedGraph.MAX_NODE_COUNT + 1;
        ForwarderCallOrdering.CallList[] memory callLists = new ForwarderCallOrdering.CallList[](2);
        callLists[0].resourceTag = keccak256("a resource of the first action");
        callLists[1].resourceTag = keccak256("a resource of the last action");
        callLists[1].actionIndex = actionCount - 1;
        callLists[0].calls = new IProtocolAdapter.ForwarderCall[](1);
        callLists[0].calls[0].afterTags = new bytes32[](1);
        callLists[0].calls[0].afterTags[0] = callLists[1].resourceTag;

        uint256[] memory orderedIndices = ForwarderCallOrdering.order(callLists);

        assertEq(orderedIndices.length, 2, "number of ordered call lists");
        assertEq(orderedIndices[0], 1, "the call list of the last action runs first");
        assertEq(orderedIndices[1], 0, "the call list of the first action runs second");
    }

    /// @dev Call lists 0 and 1 belong to action 0 and call list 2 to action 1. Call list 0 runs after call list 1.
    function test_checkCallOrder_accepts_an_order_that_meets_the_constraints() public pure {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        _constrain({callList: callLists[0], afterTag: callLists[1].resourceTag, beforeTag: bytes32(0)});

        ForwarderCallOrdering.checkCallOrder(callLists, _indices(1, 0, 2));
    }

    /// @dev A constraint across actions holds when the actions run in the order of the transaction.
    function test_checkCallOrder_accepts_a_constraint_on_an_earlier_action() public pure {
        ForwarderCallOrdering.CallList[] memory callLists = _threeCallLists();
        _constrain({callList: callLists[2], afterTag: callLists[0].resourceTag, beforeTag: bytes32(0)});

        ForwarderCallOrdering.checkCallOrder(callLists, _indices(1, 0, 2));
    }

    /// @dev `order` reverts on this many call lists once a call carries a constraint.
    function test_checkCallOrder_accepts_more_call_lists_than_a_graph_has_nodes() public pure {
        uint256 callListCount = DirectedGraph.MAX_NODE_COUNT + 1;
        ForwarderCallOrdering.CallList[] memory callLists = new ForwarderCallOrdering.CallList[](callListCount);
        uint256[] memory callOrder = new uint256[](callListCount);
        for (uint256 i = 0; i < callListCount; ++i) {
            callLists[i].resourceTag = bytes32(i + 1);
            callOrder[i] = callListCount - 1 - i;
        }
        _constrain({callList: callLists[0], afterTag: callLists[1].resourceTag, beforeTag: bytes32(0)});

        ForwarderCallOrdering.checkCallOrder(callLists, callOrder);
    }

    /// @notice Builds three call lists without calls: 0 and 1 of action 0, and 2 of action 1.
    function _threeCallLists() private pure returns (ForwarderCallOrdering.CallList[] memory callLists) {
        callLists = new ForwarderCallOrdering.CallList[](3);
        for (uint256 i = 0; i < 3; ++i) {
            callLists[i].resourceTag = keccak256(abi.encode("resource", i));
        }
        callLists[2].actionIndex = 1;
    }

    /// @notice Gives the call list one call with an `afterTags` and a `beforeTags` entry, each left out if zero.
    function _constrain(ForwarderCallOrdering.CallList memory callList, bytes32 afterTag, bytes32 beforeTag)
        private
        pure
    {
        callList.calls = new IProtocolAdapter.ForwarderCall[](1);
        callList.calls[0].afterTags = new bytes32[](afterTag == bytes32(0) ? 0 : 1);
        callList.calls[0].beforeTags = new bytes32[](beforeTag == bytes32(0) ? 0 : 1);
        if (afterTag != bytes32(0)) callList.calls[0].afterTags[0] = afterTag;
        if (beforeTag != bytes32(0)) callList.calls[0].beforeTags[0] = beforeTag;
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

    function _payload(bytes memory blob)
        private
        pure
        returns (IProtocolAdapter.ExpirableBlob[] memory externalPayload)
    {
        externalPayload = new IProtocolAdapter.ExpirableBlob[](1);
        externalPayload[0] =
            IProtocolAdapter.ExpirableBlob({deletionCriterion: IProtocolAdapter.DeletionCriterion.Never, blob: blob});
    }
}
