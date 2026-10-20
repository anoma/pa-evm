// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ForwarderCallOrder
/// @author Anoma Foundation, 2026
/// @notice A library ordering the carriers of a transaction by the edges between them. An edge is a pair of carriers
/// whose forwarder calls run in this order.
/// @custom:security-contact security@anoma.foundation
library ForwarderCallOrder {
    /// @notice The incoming edge count of a carrier that is in the order.
    uint256 private constant _ORDERED = type(uint256).max;

    /// @notice Thrown if the edges form a cycle, so that no order meets them.
    error CyclicConstraints();

    /// @notice Orders the carriers so that the predecessor of each edge comes before its successor. Of the carriers
    /// that wait for no other carrier, the carrier with the lowest index comes next.
    /// @param carrierCount The number of carriers.
    /// @param predecessors The carrier at the start of each edge.
    /// @param successors The carrier at the end of each edge.
    /// @param edgeCount The number of edges in `predecessors` and `successors`.
    /// @return orderedIndices The indices of the carriers, in the order in which their calls run.
    function order(uint256 carrierCount, uint256[] memory predecessors, uint256[] memory successors, uint256 edgeCount)
        internal
        pure
        returns (uint256[] memory orderedIndices)
    {
        uint256[] memory incomingEdgeCounts = new uint256[](carrierCount);
        for (uint256 e = 0; e < edgeCount; ++e) {
            ++incomingEdgeCounts[successors[e]];
        }

        orderedIndices = new uint256[](carrierCount);
        for (uint256 position = 0; position < carrierCount; ++position) {
            uint256 ready = _firstReady(incomingEdgeCounts);
            require(ready != carrierCount, CyclicConstraints());

            // No edge from a carrier that is not in the order ends at one that is, so no decrement reaches this value.
            incomingEdgeCounts[ready] = _ORDERED;
            orderedIndices[position] = ready;
            for (uint256 e = 0; e < edgeCount; ++e) {
                if (predecessors[e] == ready) {
                    --incomingEdgeCounts[successors[e]];
                }
            }
        }
    }

    /// @notice Returns the lowest index of a carrier that is not in the order and waits for no other carrier.
    /// @param incomingEdgeCounts The number of edges to each carrier from carriers that are not in the order.
    /// @return ready The index of the carrier, or the number of carriers if no carrier is ready.
    function _firstReady(uint256[] memory incomingEdgeCounts) private pure returns (uint256 ready) {
        uint256 carrierCount = incomingEdgeCounts.length;
        for (ready = 0; ready < carrierCount; ++ready) {
            if (incomingEdgeCounts[ready] == 0) {
                return ready;
            }
        }
    }
}
