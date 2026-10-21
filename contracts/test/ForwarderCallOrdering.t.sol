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

    /// @dev Passes the actions in calldata, where the protocol adapter reads them.
    function orderInCalldata(
        ForwarderCallOrdering.CallList[] memory callLists,
        IProtocolAdapter.Action[] calldata actions
    ) external pure returns (uint256[] memory orderedIndices) {
        orderedIndices = ForwarderCallOrdering.order(callLists, actions);
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
        this.orderInCalldata(new ForwarderCallOrdering.CallList[](callListCount), new IProtocolAdapter.Action[](0));
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
    function test_order_accepts_as_many_call_lists_as_a_graph_has_nodes() public view {
        uint256 maxNodeCount = DirectedGraph.MAX_NODE_COUNT;

        uint256[] memory orderedIndices =
            this.orderInCalldata(new ForwarderCallOrdering.CallList[](maxNodeCount), new IProtocolAdapter.Action[](0));

        assertEq(orderedIndices.length, maxNodeCount, "number of ordered call lists");
        for (uint256 position = 0; position < maxNodeCount; ++position) {
            assertEq(orderedIndices[position], position, "ordered call list");
        }
    }

    /// @dev The call list of the first action waits for the call list of the last action.
    function test_order_accepts_more_actions_than_a_graph_has_nodes() public view {
        uint256 actionCount = DirectedGraph.MAX_NODE_COUNT + 1;
        ForwarderCallOrdering.CallList[] memory callLists = new ForwarderCallOrdering.CallList[](2);
        callLists[0].resourceTag = keccak256("a resource of the first action");
        callLists[1].resourceTag = keccak256("a resource of the last action");
        callLists[1].actionIndex = actionCount - 1;
        callLists[0].calls = new IProtocolAdapter.ForwarderCall[](1);
        callLists[0].calls[0].afterTags = new bytes32[](1);
        callLists[0].calls[0].afterTags[0] = callLists[1].resourceTag;

        uint256[] memory orderedIndices = this.orderInCalldata(callLists, new IProtocolAdapter.Action[](actionCount));

        assertEq(orderedIndices.length, 2, "number of ordered call lists");
        assertEq(orderedIndices[0], 1, "the call list of the last action runs first");
        assertEq(orderedIndices[1], 0, "the call list of the first action runs second");
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
