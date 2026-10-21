// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {LibPRNG} from "solady-0.1.26/src/utils/LibPRNG.sol";

import {DirectedGraph} from "../src/libs/DirectedGraph.sol";

contract DirectedGraphTest is Test {
    using DirectedGraph for DirectedGraph.Graph;
    using LibPRNG for LibPRNG.PRNG;

    function test_addEdges_puts_each_node_of_one_range_before_each_node_of_another() public pure {
        DirectedGraph.Graph memory graph = _graphWithoutEdges(3);
        graph.addEdges({sourceStart: 1, sourceEnd: 3, targetStart: 0, targetEnd: 1});

        (bool isAcyclic, uint256[] memory sortedNodes) = graph.topologicalSort();

        assertTrue(isAcyclic, "the edges form no cycle");
        assertEq(sortedNodes, _list(1, 2, 0), "sorted nodes");
    }

    /// @dev The source range holds the node whose bit is the last bit of a bitmap.
    function test_addEdges_reaches_the_last_node_of_a_bitmap() public pure {
        uint256 nodeCount = DirectedGraph.MAX_NODE_COUNT;
        DirectedGraph.Graph memory graph = _graphWithoutEdges(nodeCount);
        graph.addEdges({sourceStart: nodeCount - 1, sourceEnd: nodeCount, targetStart: 0, targetEnd: nodeCount - 1});

        (bool isAcyclic, uint256[] memory sortedNodes) = graph.topologicalSort();

        assertTrue(isAcyclic, "the edges form no cycle");
        assertEq(sortedNodes[0], nodeCount - 1, "the last node comes first");
        assertEq(sortedNodes[nodeCount - 1], nodeCount - 2, "the other nodes follow in the order of their indices");
    }

    function test_topologicalSort_reports_edges_that_form_a_cycle() public pure {
        (bool isAcyclic,) =
            _graph({nodeCount: 3, edgeSources: _list(0, 1, 2), edgeTargets: _list(1, 2, 0)}).topologicalSort();

        assertFalse(isAcyclic, "the edges form a cycle");
    }

    function test_topologicalSort_reports_an_edge_from_a_node_to_itself_as_a_cycle() public pure {
        (bool isAcyclic,) = _graph({nodeCount: 2, edgeSources: _list(1), edgeTargets: _list(1)}).topologicalSort();

        assertFalse(isAcyclic, "the edge forms a cycle");
    }

    function test_topologicalSort_sorts_a_graph_without_edges_by_index() public pure {
        assertEq(_sort({nodeCount: 3, edgeSources: _list(), edgeTargets: _list()}), _list(0, 1, 2), "sorted nodes");
    }

    function test_topologicalSort_puts_the_source_of_an_edge_before_its_target() public pure {
        assertEq(_sort({nodeCount: 3, edgeSources: _list(2), edgeTargets: _list(0)}), _list(1, 2, 0), "sorted nodes");
    }

    /// @dev Once node 1 is sorted, nodes 0 and 2 are both ready, and 0 has the lower index.
    function test_topologicalSort_takes_the_ready_node_with_the_lowest_index() public pure {
        assertEq(_sort({nodeCount: 3, edgeSources: _list(1), edgeTargets: _list(0)}), _list(1, 0, 2), "sorted nodes");
    }

    function test_topologicalSort_sorts_two_equal_edges_like_one() public pure {
        assertEq(_sort({nodeCount: 2, edgeSources: _list(1, 1), edgeTargets: _list(0, 0)}), _list(1, 0), "sorted nodes");
    }

    /// @dev Each node waits for the node above it, so the highest node, whose bit is the last bit of a bitmap, comes
    /// first.
    function test_topologicalSort_sorts_a_graph_of_the_maximum_node_count() public pure {
        uint256 nodeCount = DirectedGraph.MAX_NODE_COUNT;
        DirectedGraph.Graph memory graph = _graphWithoutEdges(nodeCount);
        for (uint256 node = 0; node + 1 < nodeCount; ++node) {
            graph.addEdge({source: node + 1, target: node});
        }

        (bool isAcyclic, uint256[] memory sortedNodes) = graph.topologicalSort();

        assertTrue(isAcyclic, "the edges form no cycle");
        assertEq(sortedNodes.length, nodeCount, "number of sorted nodes");
        for (uint256 position = 0; position < nodeCount; ++position) {
            assertEq(sortedNodes[position], nodeCount - 1 - position, "sorted node");
        }
    }

    function test_topologicalSort_returns_no_nodes_for_a_graph_without_nodes() public pure {
        assertEq(_sort({nodeCount: 0, edgeSources: _list(), edgeTargets: _list()}).length, 0, "number of sorted nodes");
    }

    /// @dev Draws a rank for each node and adds edges that agree with the ranks, so that the graph has no cycle.
    function testFuzz_topologicalSort_puts_the_source_of_every_edge_before_its_target(uint256 nodeCount, uint256 seed)
        public
        pure
    {
        nodeCount = bound(nodeCount, 1, DirectedGraph.MAX_NODE_COUNT);
        LibPRNG.PRNG memory prng = LibPRNG.PRNG({state: seed});

        uint256[] memory ranks = new uint256[](nodeCount);
        for (uint256 i = 0; i < nodeCount; ++i) {
            ranks[i] = i;
        }
        prng.shuffle(ranks);

        uint256 drawCount = 2 * nodeCount;
        DirectedGraph.Graph memory graph = _graphWithoutEdges(nodeCount);
        uint256[] memory edgeSources = new uint256[](drawCount);
        uint256[] memory edgeTargets = new uint256[](drawCount);
        uint256 edgeCount = 0;
        for (uint256 d = 0; d < drawCount; ++d) {
            uint256 first = prng.uniform(nodeCount);
            uint256 second = prng.uniform(nodeCount);
            if (ranks[first] != ranks[second]) {
                (edgeSources[edgeCount], edgeTargets[edgeCount]) =
                    ranks[first] < ranks[second] ? (first, second) : (second, first);
                graph.addEdge({source: edgeSources[edgeCount], target: edgeTargets[edgeCount]});
                ++edgeCount;
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
        for (uint256 e = 0; e < edgeCount; ++e) {
            assertLt(positions[edgeSources[e]], positions[edgeTargets[e]], "a source comes before its target");
        }
    }

    function _sort(uint256 nodeCount, uint256[] memory edgeSources, uint256[] memory edgeTargets)
        private
        pure
        returns (uint256[] memory sortedNodes)
    {
        bool isAcyclic;
        (isAcyclic, sortedNodes) =
            _graph({nodeCount: nodeCount, edgeSources: edgeSources, edgeTargets: edgeTargets}).topologicalSort();
        assertTrue(isAcyclic, "the edges form no cycle");
    }

    function _graph(uint256 nodeCount, uint256[] memory edgeSources, uint256[] memory edgeTargets)
        private
        pure
        returns (DirectedGraph.Graph memory graph)
    {
        assertEq(edgeSources.length, edgeTargets.length, "one start and one end for each edge");
        graph = _graphWithoutEdges(nodeCount);
        for (uint256 e = 0; e < edgeSources.length; ++e) {
            graph.addEdge({source: edgeSources[e], target: edgeTargets[e]});
        }
    }

    function _graphWithoutEdges(uint256 nodeCount) private pure returns (DirectedGraph.Graph memory graph) {
        graph = DirectedGraph.Graph({inNeighbors: new uint256[](nodeCount), outNeighbors: new uint256[](nodeCount)});
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
