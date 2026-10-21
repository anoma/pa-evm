// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LibBit} from "solady-0.1.26/src/utils/LibBit.sol";

/// @title DirectedGraph
/// @author Anoma Foundation, 2026
/// @notice A library for directed graphs whose nodes are indices, and for their topological sort.
/// @custom:security-contact security@anoma.foundation
library DirectedGraph {
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
        // `x |= 1 << n` sets bit n of `x`: node n joins the set.
        graph.predecessors[successor] |= 1 << predecessor;
        graph.successors[predecessor] |= 1 << successor;
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
                readyBitmap |= 1 << node;
            }
        }

        sortedNodes = new uint256[](nodeCount);
        uint256 sortedBitmap = 0;
        for (uint256 position = 0; position < nodeCount; ++position) {
            // No node is ready while some are not sorted, so the edges form a cycle.
            if (readyBitmap == 0) {
                return (false, sortedNodes);
            }
            // `LibBit.ffs` returns the index of the lowest set bit: the ready node with the lowest index.
            uint256 node = LibBit.ffs(readyBitmap);
            // `x &= ~(1 << n)` clears bit n of `x`: the node leaves the ready set and joins the sorted set.
            readyBitmap &= ~(1 << node);
            sortedBitmap |= 1 << node;
            sortedNodes[position] = node;

            // Takes the successors one by one, from the lowest bit, and clears each bit after use.
            uint256 remainingSuccessors = successors[node];
            while (remainingSuccessors != 0) {
                uint256 successor = LibBit.ffs(remainingSuccessors);
                remainingSuccessors &= ~(1 << successor);
                // `& ~sortedBitmap` clears the sorted predecessors, so a zero result means that all are sorted.
                if ((predecessors[successor] & ~sortedBitmap) == 0) {
                    readyBitmap |= 1 << successor;
                }
            }
        }
        isAcyclic = true;
    }
}
