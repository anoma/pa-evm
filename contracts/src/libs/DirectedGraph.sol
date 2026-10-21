// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title DirectedGraph
/// @author Anoma Foundation, 2026
/// @notice A library for directed graphs whose nodes are indices, and for their topological sort.
/// @custom:security-contact security@anoma.foundation
library DirectedGraph {
    /// @notice A directed graph. Each node is an index below `nodeCount`. An edge from a predecessor to a successor
    /// means that the predecessor comes before the successor.
    /// @param nodeCount The number of nodes.
    /// @param predecessors The node at the start of each edge.
    /// @param successors The node at the end of each edge.
    /// @param edgeCount The number of edges, in the first entries of `predecessors` and `successors`.
    struct Graph {
        uint256 nodeCount;
        uint256[] predecessors;
        uint256[] successors;
        uint256 edgeCount;
    }

    /// @notice The value that marks a sorted node in place of its in-degree, so that the node is never ready again.
    uint256 private constant _SORTED = type(uint256).max;

    /// @notice Adds an edge to a graph, which must have room for it in `predecessors` and `successors`.
    /// @param graph The graph.
    /// @param predecessor The node at the start of the edge.
    /// @param successor The node at the end of the edge.
    function addEdge(Graph memory graph, uint256 predecessor, uint256 successor) internal pure {
        uint256 edgeCount = graph.edgeCount;
        graph.predecessors[edgeCount] = predecessor;
        graph.successors[edgeCount] = successor;
        graph.edgeCount = edgeCount + 1;
    }

    /// @notice Sorts the nodes of a graph topologically: the predecessor of each edge comes before its successor. Of
    /// the nodes that wait for no other node, the node with the lowest index comes next.
    /// @dev Kahn's algorithm. The in-degree of a node is the number of edges to it from nodes that are not sorted yet,
    /// and a node is ready when its in-degree is zero. Taking the ready node with the lowest index at each step makes
    /// the order unique.
    /// @param graph The graph to sort.
    /// @return isAcyclic Whether the edges form no cycle, so that a topological order exists.
    /// @return sortedNodes The nodes in topological order, if `isAcyclic` is true.
    function topologicalSort(Graph memory graph) internal pure returns (bool isAcyclic, uint256[] memory sortedNodes) {
        uint256 nodeCount = graph.nodeCount;
        uint256[] memory predecessors = graph.predecessors;
        uint256[] memory successors = graph.successors;
        uint256 edgeCount = graph.edgeCount;

        uint256[] memory inDegrees = new uint256[](nodeCount);
        for (uint256 e = 0; e < edgeCount; ++e) {
            ++inDegrees[successors[e]];
        }

        sortedNodes = new uint256[](nodeCount);
        for (uint256 position = 0; position < nodeCount; ++position) {
            uint256 node = _firstReadyNode(inDegrees);
            if (node == nodeCount) {
                return (false, sortedNodes);
            }

            // Each edge from this node ends at a node that is not sorted yet, so no decrement reaches this value.
            inDegrees[node] = _SORTED;
            sortedNodes[position] = node;
            for (uint256 e = 0; e < edgeCount; ++e) {
                if (predecessors[e] == node) {
                    --inDegrees[successors[e]];
                }
            }
        }
        isAcyclic = true;
    }

    /// @notice Returns the ready node with the lowest index.
    /// @param inDegrees The in-degree of each node, or `_SORTED` for a sorted node.
    /// @return node The node, or the number of nodes if no node is ready.
    function _firstReadyNode(uint256[] memory inDegrees) private pure returns (uint256 node) {
        uint256 nodeCount = inDegrees.length;
        for (node = 0; node < nodeCount; ++node) {
            if (inDegrees[node] == 0) {
                return node;
            }
        }
    }
}
