// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {LibPRNG} from "solady-0.1.26/src/utils/LibPRNG.sol";

import {DirectedGraph} from "../src/libs/DirectedGraph.sol";

contract DirectedGraphTest is Test {
    using DirectedGraph for DirectedGraph.Graph;
    using LibPRNG for LibPRNG.PRNG;

    function test_topologicalSort_reports_edges_that_form_a_cycle() public pure {
        (bool isAcyclic,) =
            _graph({nodeCount: 3, predecessors: _list(0, 1, 2), successors: _list(1, 2, 0)}).topologicalSort();

        assertFalse(isAcyclic, "the edges form a cycle");
    }

    function test_topologicalSort_reports_an_edge_from_a_node_to_itself_as_a_cycle() public pure {
        (bool isAcyclic,) = _graph({nodeCount: 2, predecessors: _list(1), successors: _list(1)}).topologicalSort();

        assertFalse(isAcyclic, "the edge forms a cycle");
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

    /// @dev The memory of the second edge holds zeros, which would read as an edge from node 0 to itself.
    function test_topologicalSort_reads_only_the_added_edges() public pure {
        DirectedGraph.Graph memory graph = DirectedGraph.Graph({
            nodeCount: 2, predecessors: new uint256[](2), successors: new uint256[](2), edgeCount: 0
        });
        graph.addEdge({predecessor: 1, successor: 0});

        (bool isAcyclic, uint256[] memory sortedNodes) = graph.topologicalSort();

        assertTrue(isAcyclic, "the edges form no cycle");
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
        DirectedGraph.Graph memory graph = DirectedGraph.Graph({
            nodeCount: nodeCount,
            predecessors: new uint256[](drawCount),
            successors: new uint256[](drawCount),
            edgeCount: 0
        });
        for (uint256 d = 0; d < drawCount; ++d) {
            uint256 first = prng.uniform(nodeCount);
            uint256 second = prng.uniform(nodeCount);
            if (ranks[first] != ranks[second]) {
                (uint256 predecessor, uint256 successor) =
                    ranks[first] < ranks[second] ? (first, second) : (second, first);
                graph.addEdge({predecessor: predecessor, successor: successor});
            }
        }

        (bool isAcyclic, uint256[] memory sortedNodes) = graph.topologicalSort();

        assertTrue(isAcyclic, "the edges form no cycle");
        uint256[] memory positions = new uint256[](nodeCount);
        bool[] memory seen = new bool[](nodeCount);
        assertEq(sortedNodes.length, nodeCount, "every node is sorted");
        for (uint256 position = 0; position < nodeCount; ++position) {
            assertFalse(seen[sortedNodes[position]], "no node is sorted twice");
            seen[sortedNodes[position]] = true;
            positions[sortedNodes[position]] = position;
        }
        for (uint256 e = 0; e < graph.edgeCount; ++e) {
            assertLt(
                positions[graph.predecessors[e]],
                positions[graph.successors[e]],
                "a predecessor comes before its successor"
            );
        }
    }

    function _sort(uint256 nodeCount, uint256[] memory predecessors, uint256[] memory successors)
        private
        pure
        returns (uint256[] memory sortedNodes)
    {
        bool isAcyclic;
        (isAcyclic, sortedNodes) =
            _graph({nodeCount: nodeCount, predecessors: predecessors, successors: successors}).topologicalSort();
        assertTrue(isAcyclic, "the edges form no cycle");
    }

    function _graph(uint256 nodeCount, uint256[] memory predecessors, uint256[] memory successors)
        private
        pure
        returns (DirectedGraph.Graph memory graph)
    {
        assertEq(predecessors.length, successors.length, "one predecessor and one successor for each edge");
        graph = DirectedGraph.Graph({
            nodeCount: nodeCount, predecessors: predecessors, successors: successors, edgeCount: predecessors.length
        });
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
