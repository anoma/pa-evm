// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LibBit} from "solady-0.1.26/src/utils/LibBit.sol";

/// @title DirectedGraph
/// @author Anoma Foundation, 2026
/// @notice A library for directed graphs whose nodes are indices, and for their topological sort.
/// @custom:security-contact security@anoma.foundation
library DirectedGraph {
    using DirectedGraph for uint256;

    /// @notice A directed graph. Each node is an index below the length of `predecessors` and `successors`, which is
    /// at most `MAX_NODE_COUNT`. A bitmap holds a set of nodes: its bit n is set if node n is in the set. An edge
    /// from a predecessor to a successor means that the predecessor comes before the successor.
    /// @param predecessors The predecessors of each node, as a bitmap.
    /// @param successors The successors of each node, as a bitmap.
    struct Graph {
        uint256[] predecessors;
        uint256[] successors;
    }

    /// @notice The maximum number of nodes of a graph, which is the number of bits of a bitmap.
    uint256 public constant MAX_NODE_COUNT = 256;

    /// @notice Adds an edge to a graph.
    /// @param graph The graph.
    /// @param predecessor The node at the start of the edge.
    /// @param successor The node at the end of the edge.
    function addEdge(Graph memory graph, uint256 predecessor, uint256 successor) internal pure {
        graph.predecessors[successor] = graph.predecessors[successor]._add(predecessor);
        graph.successors[predecessor] = graph.successors[predecessor]._add(successor);
    }

    /// @notice Adds an edge from each node in a range to each node in another range. A range holds the nodes from its
    /// start up to its end, without the end.
    /// @param graph The graph.
    /// @param predecessorStart The first node at the start of the edges.
    /// @param predecessorEnd The node after the last node at the start of the edges.
    /// @param successorStart The first node at the end of the edges.
    /// @param successorEnd The node after the last node at the end of the edges.
    function addEdges(
        Graph memory graph,
        uint256 predecessorStart,
        uint256 predecessorEnd,
        uint256 successorStart,
        uint256 successorEnd
    ) internal pure {
        uint256 predecessorSet = _range(predecessorStart, predecessorEnd);
        uint256 successorSet = _range(successorStart, successorEnd);
        for (uint256 node = predecessorStart; node < predecessorEnd; ++node) {
            graph.successors[node] = graph.successors[node]._union(successorSet);
        }
        for (uint256 node = successorStart; node < successorEnd; ++node) {
            graph.predecessors[node] = graph.predecessors[node]._union(predecessorSet);
        }
    }

    /// @notice Sorts the nodes of a graph topologically: the predecessor of each edge comes before its successor. Of
    /// the nodes that wait for no other node, the node with the lowest index comes next.
    /// @dev Kahn's algorithm. A node is ready when all its predecessors are sorted. Taking the ready node with the
    /// lowest index at each step makes the order unique.
    /// @param graph The graph to sort.
    /// @return isAcyclic Whether the edges form no cycle, so that a topological order exists.
    /// @return sortedNodes The nodes in topological order, if `isAcyclic` is true.
    function topologicalSort(Graph memory graph) internal pure returns (bool isAcyclic, uint256[] memory sortedNodes) {
        uint256[] memory predecessors = graph.predecessors;
        uint256[] memory successors = graph.successors;
        uint256 nodeCount = predecessors.length;

        uint256 readyBitmap = 0;
        for (uint256 node = 0; node < nodeCount; ++node) {
            // A node without predecessors is ready.
            if (predecessors[node] == 0) {
                readyBitmap = readyBitmap._add(node);
            }
        }

        sortedNodes = new uint256[](nodeCount);
        uint256 sortedBitmap = 0;
        for (uint256 position = 0; position < nodeCount; ++position) {
            // No node is ready while some are not sorted, so the edges form a cycle.
            if (readyBitmap == 0) {
                return (false, sortedNodes);
            }
            uint256 node = readyBitmap._lowest();
            readyBitmap = readyBitmap._remove(node);
            sortedBitmap = sortedBitmap._add(node);
            sortedNodes[position] = node;

            uint256 remainingSuccessors = successors[node];
            while (remainingSuccessors != 0) {
                uint256 successor = remainingSuccessors._lowest();
                remainingSuccessors = remainingSuccessors._remove(successor);
                if (predecessors[successor]._isSubsetOf(sortedBitmap)) {
                    readyBitmap = readyBitmap._add(successor);
                }
            }
        }
        isAcyclic = true;
    }

    /// @notice Returns a set with a node added.
    /// @param set The set, as a bitmap.
    /// @param node The node to add.
    /// @return result The set with the node.
    function _add(uint256 set, uint256 node) internal pure returns (uint256 result) {
        result = set | (1 << node);
    }

    /// @notice Returns a set without a node.
    /// @param set The set, as a bitmap.
    /// @param node The node to remove.
    /// @return result The set without the node.
    function _remove(uint256 set, uint256 node) internal pure returns (uint256 result) {
        result = set & ~(1 << node);
    }

    /// @notice Returns the node with the lowest index in a set, or 256 for the empty set.
    /// @param set The set, as a bitmap.
    /// @return node The node.
    function _lowest(uint256 set) internal pure returns (uint256 node) {
        node = LibBit.ffs(set);
    }

    /// @notice Returns the nodes that are in one set or in another.
    /// @param set The first set, as a bitmap.
    /// @param other The second set, as a bitmap.
    /// @return result The nodes of both sets.
    function _union(uint256 set, uint256 other) internal pure returns (uint256 result) {
        result = set | other;
    }

    /// @notice Returns the nodes of a range, which must not be empty.
    /// @param start The first node of the range.
    /// @param end The node after the last node of the range.
    /// @return set The set, as a bitmap.
    function _range(uint256 start, uint256 end) internal pure returns (uint256 set) {
        set = (type(uint256).max >> (MAX_NODE_COUNT - end)) & ~((1 << start) - 1);
    }

    /// @notice Returns whether each node of a set is also in another set.
    /// @param set The set to check, as a bitmap.
    /// @param superset The other set, as a bitmap.
    /// @return isSubset Whether `superset` holds each node of `set`.
    function _isSubsetOf(uint256 set, uint256 superset) internal pure returns (bool isSubset) {
        isSubset = (set & ~superset) == 0;
    }
}
