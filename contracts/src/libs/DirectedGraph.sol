// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LibBit} from "solady-0.1.26/src/utils/LibBit.sol";

/// @title DirectedGraph
/// @author Anoma Foundation, 2026
/// @notice A library for directed graphs whose nodes are indices, and for their topological sort.
/// @custom:security-contact security@anoma.foundation
library DirectedGraph {
    using DirectedGraph for uint256;

    /// @notice A directed graph. Each node is an index below the length of `inNeighbors` and `outNeighbors`, which is
    /// at most `MAX_NODE_COUNT`. A bitmap holds a set of nodes: its bit n is set if node n is in the set. An edge
    /// from a source to a target means that the source comes before the target.
    /// @param inNeighbors The in-neighbors of each node, which have an edge to it, as a bitmap.
    /// @param outNeighbors The out-neighbors of each node, to which it has an edge, as a bitmap.
    struct Graph {
        uint256[] inNeighbors;
        uint256[] outNeighbors;
    }

    /// @notice The maximum number of nodes of a graph, which is the number of bits of a bitmap.
    uint256 public constant MAX_NODE_COUNT = 256;

    /// @notice Thrown if a graph to sort has more than `MAX_NODE_COUNT` nodes.
    error TooManyNodes(uint256 nodeCount);

    /// @notice Adds an edge to a graph.
    /// @param graph The graph.
    /// @param source The node at the start of the edge.
    /// @param target The node at the end of the edge.
    function addEdge(Graph memory graph, uint256 source, uint256 target) internal pure {
        graph.inNeighbors[target] = graph.inNeighbors[target]._add(source);
        graph.outNeighbors[source] = graph.outNeighbors[source]._add(target);
    }

    /// @notice Adds an edge from each node in a range to each node in another range. A range holds the nodes from its
    /// start up to its end, without the end.
    /// @param graph The graph.
    /// @param sourceStart The first node at the start of the edges.
    /// @param sourceEnd The node after the last node at the start of the edges.
    /// @param targetStart The first node at the end of the edges.
    /// @param targetEnd The node after the last node at the end of the edges.
    function addEdges(
        Graph memory graph,
        uint256 sourceStart,
        uint256 sourceEnd,
        uint256 targetStart,
        uint256 targetEnd
    ) internal pure {
        uint256 sourceSet = _range(sourceStart, sourceEnd);
        uint256 targetSet = _range(targetStart, targetEnd);
        for (uint256 node = sourceStart; node < sourceEnd; ++node) {
            graph.outNeighbors[node] = graph.outNeighbors[node]._union(targetSet);
        }
        for (uint256 node = targetStart; node < targetEnd; ++node) {
            graph.inNeighbors[node] = graph.inNeighbors[node]._union(sourceSet);
        }
    }

    /// @notice Sorts the nodes of a graph topologically: the source of each edge comes before its target. A node
    /// is ready when all its in-neighbors are sorted. Of the ready nodes, the one with the lowest index comes
    /// next, so the order is unique.
    /// @dev Kahn's algorithm. Reverts if the graph has more than `MAX_NODE_COUNT` nodes, because a bitmap has no bit
    /// for node `MAX_NODE_COUNT` and above.
    /// @param graph The graph to sort.
    /// @return isAcyclic Whether the edges form no cycle, so that a topological order exists.
    /// @return sortedNodes The nodes in topological order, if `isAcyclic` is true.
    function topologicalSort(Graph memory graph) internal pure returns (bool isAcyclic, uint256[] memory sortedNodes) {
        uint256[] memory inNeighbors = graph.inNeighbors;
        uint256[] memory outNeighbors = graph.outNeighbors;
        uint256 nodeCount = inNeighbors.length;
        require(nodeCount <= MAX_NODE_COUNT, TooManyNodes(nodeCount));

        uint256 readyBitmap = 0;
        for (uint256 node = 0; node < nodeCount; ++node) {
            // A node without in-neighbors is ready.
            if (inNeighbors[node] == 0) {
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

            uint256 remainingOutNeighbors = outNeighbors[node];
            while (remainingOutNeighbors != 0) {
                uint256 outNeighbor = remainingOutNeighbors._lowest();
                remainingOutNeighbors = remainingOutNeighbors._remove(outNeighbor);
                if (inNeighbors[outNeighbor]._isSubsetOf(sortedBitmap)) {
                    readyBitmap = readyBitmap._add(outNeighbor);
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
