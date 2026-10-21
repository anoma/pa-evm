// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {LibPRNG} from "solady-0.1.26/src/utils/LibPRNG.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ForwarderCallOrder} from "../src/libs/ForwarderCallOrder.sol";

contract ForwarderCallOrderTest is Test {
    using LibPRNG for LibPRNG.PRNG;

    /// @notice Passes the payload in calldata, where the protocol adapter reads it.
    function hasOrderingConstraintsInCalldata(IProtocolAdapter.ExpirableBlob[] calldata externalPayload)
        external
        pure
        returns (bool constrained)
    {
        constrained = ForwarderCallOrder.hasOrderingConstraints(externalPayload);
    }

    /// @dev The offsets of the tag lists are the fourth and fifth words of the encoded call.
    function test_hasOrderingConstraints_reverts_on_a_blob_too_short_for_the_tag_list_offsets() public {
        IProtocolAdapter.ExpirableBlob[] memory externalPayload = _payload(new bytes(3 * 32));

        vm.expectRevert();
        this.hasOrderingConstraintsInCalldata(externalPayload);
    }

    function test_topologicalSort_reverts_on_edges_that_form_a_cycle() public {
        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _sort({nodeCount: 3, predecessors: _list(0, 1, 2), successors: _list(1, 2, 0)});
    }

    function test_topologicalSort_reverts_on_an_edge_from_a_node_to_itself() public {
        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _sort({nodeCount: 2, predecessors: _list(1), successors: _list(1)});
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

    function test_topologicalSort_sorts_a_graph_without_edges_by_index() public pure {
        assertEq(_sort({nodeCount: 3, predecessors: _list(), successors: _list()}), _list(0, 1, 2), "sorted nodes");
    }

    function test_topologicalSort_puts_the_predecessor_of_an_edge_before_its_successor() public pure {
        assertEq(_sort({nodeCount: 3, predecessors: _list(2), successors: _list(0)}), _list(1, 2, 0), "sorted nodes");
    }

    /// @dev Once node 1 is sorted, nodes 0 and 2 are both ready, and 0 has the lower index.
    function test_topologicalSort_takes_the_ready_node_with_the_lowest_index() public pure {
        assertEq(_sort({nodeCount: 3, predecessors: _list(1), successors: _list(0)}), _list(1, 0, 2), "sorted nodes");
    }

    function test_topologicalSort_counts_each_of_two_equal_edges() public pure {
        assertEq(_sort({nodeCount: 2, predecessors: _list(1, 1), successors: _list(0, 0)}), _list(1, 0), "sorted nodes");
    }

    /// @dev `order` sizes `predecessors` and `successors` by the number of constraints, so their tail holds zeros that
    /// would read as an edge from node 0 to itself.
    function test_topologicalSort_reads_only_the_first_edge_count_entries() public pure {
        uint256[] memory sortedNodes = ForwarderCallOrder.topologicalSort(
            ForwarderCallOrder.Graph({nodeCount: 2, predecessors: _list(1, 0), successors: _list(0, 0), edgeCount: 1})
        );

        assertEq(sortedNodes, _list(1, 0), "sorted nodes");
    }

    function test_topologicalSort_returns_no_nodes_for_a_graph_without_nodes() public pure {
        assertEq(_sort({nodeCount: 0, predecessors: _list(), successors: _list()}).length, 0, "number of sorted nodes");
    }

    /// @dev Draws a rank for each node and adds edges that agree with the ranks, so that the graph has no cycle.
    function testFuzz_topologicalSort_puts_the_predecessor_of_every_edge_before_its_successor(
        uint8 nodeCount,
        uint256 seed
    ) public pure {
        nodeCount = uint8(bound(nodeCount, 1, 16));
        LibPRNG.PRNG memory prng = LibPRNG.PRNG({state: seed});

        uint256[] memory ranks = new uint256[](nodeCount);
        for (uint256 i = 0; i < nodeCount; ++i) {
            ranks[i] = i;
        }
        prng.shuffle(ranks);

        uint256 drawCount = 2 * nodeCount;
        uint256[] memory predecessors = new uint256[](drawCount);
        uint256[] memory successors = new uint256[](drawCount);
        uint256 edgeCount = 0;
        for (uint256 d = 0; d < drawCount; ++d) {
            uint256 first = prng.uniform(nodeCount);
            uint256 second = prng.uniform(nodeCount);
            if (ranks[first] != ranks[second]) {
                (predecessors[edgeCount], successors[edgeCount]) =
                    ranks[first] < ranks[second] ? (first, second) : (second, first);
                ++edgeCount;
            }
        }

        uint256[] memory sortedNodes = ForwarderCallOrder.topologicalSort(
            ForwarderCallOrder.Graph({
                nodeCount: nodeCount, predecessors: predecessors, successors: successors, edgeCount: edgeCount
            })
        );

        uint256[] memory positions = new uint256[](nodeCount);
        bool[] memory seen = new bool[](nodeCount);
        assertEq(sortedNodes.length, nodeCount, "every node is sorted");
        for (uint256 position = 0; position < nodeCount; ++position) {
            assertFalse(seen[sortedNodes[position]], "no node is sorted twice");
            seen[sortedNodes[position]] = true;
            positions[sortedNodes[position]] = position;
        }
        for (uint256 e = 0; e < edgeCount; ++e) {
            assertLt(positions[predecessors[e]], positions[successors[e]], "a predecessor comes before its successor");
        }
    }

    function _sort(uint256 nodeCount, uint256[] memory predecessors, uint256[] memory successors)
        private
        pure
        returns (uint256[] memory sortedNodes)
    {
        assertEq(predecessors.length, successors.length, "one predecessor and one successor for each edge");
        sortedNodes = ForwarderCallOrder.topologicalSort(
            ForwarderCallOrder.Graph({
                nodeCount: nodeCount, predecessors: predecessors, successors: successors, edgeCount: predecessors.length
            })
        );
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

    function _list() private pure returns (uint256[] memory list) {
        list = new uint256[](0);
    }

    function _list(uint256 a) private pure returns (uint256[] memory list) {
        list = new uint256[](1);
        list[0] = a;
    }

    function _list(uint256 a, uint256 b) private pure returns (uint256[] memory list) {
        list = new uint256[](2);
        list[0] = a;
        list[1] = b;
    }

    function _list(uint256 a, uint256 b, uint256 c) private pure returns (uint256[] memory list) {
        list = new uint256[](3);
        list[0] = a;
        list[1] = b;
        list[2] = c;
    }
}
