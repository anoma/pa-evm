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

    /// @notice Returns whether a graph has an edge.
    /// @param graph The graph.
    /// @return found Whether a node has a successor.
    function hasEdges(Graph memory graph) internal pure returns (bool found) {
        uint256[] memory successors = graph.successors;
        uint256 nodeCount = successors.length;
        for (uint256 node = 0; node < nodeCount; ++node) {
            if (successors[node] != 0) {
                return true;
            }
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

    /// @notice Returns whether each node of a set is also in another set.
    /// @param set The set to check, as a bitmap.
    /// @param superset The other set, as a bitmap.
    /// @return isSubset Whether `superset` holds each node of `set`.
    function _isSubsetOf(uint256 set, uint256 superset) internal pure returns (bool isSubset) {
        isSubset = (set & ~superset) == 0;
    }
}
