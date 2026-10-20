// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ForwarderCallOrder
/// @author Anoma Foundation, 2026
/// @notice A library ordering the forwarder calls of a transaction by the constraints that the calls carry.
/// @custom:security-contact security@anoma.foundation
library ForwarderCallOrder {
    /// @notice The constraints of a forwarder call.
    /// @param tag The tag of the resource carrying the call.
    /// @param afterTags The tags of the resources whose calls must run before the call.
    /// @param beforeTags The tags of the resources whose calls must run after the call.
    struct Constraints {
        bytes32 tag;
        bytes32[] afterTags;
        bytes32[] beforeTags;
    }

    /// @notice Thrown if a call must run after the calls of a resource that is not in the transaction.
    error AfterTagNotInTransaction(bytes32 tag);

    /// @notice Thrown if the constraints form a cycle, so that no order meets them.
    error CyclicConstraints();

    /// @notice Orders the forwarder calls of a transaction. Each call runs after the calls of the resources in its
    /// `afterTags` and before the calls of the resources in its `beforeTags`, and the calls of a resource keep their
    /// order. Of the calls that can run next, the first in `calls` runs, so calls without constraints keep their
    /// order.
    /// @dev The protocol adapter changes the state of all resources before any call runs, so a resource without
    /// calls counts as done. An `afterTags` entry must name a resource of the transaction. A `beforeTags` entry that
    /// names no call constrains nothing.
    /// @param tags The tags of all resources of the transaction.
    /// @param calls The constraints of the calls, with the calls of a resource next to each other.
    /// @return sequence The indices of `calls` in the order in which the calls run.
    function order(bytes32[] memory tags, Constraints[] memory calls)
        internal
        pure
        returns (uint256[] memory sequence)
    {
        uint256 callCount = calls.length;
        sequence = new uint256[](callCount);

        // Without constraints, the calls run in their order.
        if (!_hasConstraints(calls)) {
            for (uint256 k = 0; k < callCount; ++k) {
                sequence[k] = k;
            }
            return sequence;
        }

        // An edge from call `earlier[e]` to call `later[e]` means that the first call runs before the second.
        uint256 edgeCount = _edges({tags: tags, calls: calls, earlier: new uint256[](0), later: new uint256[](0)});
        uint256[] memory earlier = new uint256[](edgeCount);
        uint256[] memory later = new uint256[](edgeCount);
        _edges({tags: tags, calls: calls, earlier: earlier, later: later});

        uint256[] memory predecessorCounts = new uint256[](callCount);
        for (uint256 e = 0; e < edgeCount; ++e) {
            ++predecessorCounts[later[e]];
        }

        bool[] memory done = new bool[](callCount);
        for (uint256 k = 0; k < callCount; ++k) {
            uint256 next = _firstReady({predecessorCounts: predecessorCounts, done: done});
            require(next != callCount, CyclicConstraints());

            done[next] = true;
            sequence[k] = next;
            for (uint256 e = 0; e < edgeCount; ++e) {
                if (earlier[e] == next) {
                    --predecessorCounts[later[e]];
                }
            }
        }
    }

    /// @notice Lists the edges that the constraints set. Writes them only while `earlier` and `later` have room, so
    /// that empty arrays only count them.
    /// @param tags The tags of all resources of the transaction.
    /// @param calls The constraints of the calls.
    /// @param earlier The calls that run first, one per edge.
    /// @param later The calls that run second, one per edge.
    /// @return count The number of edges.
    function _edges(bytes32[] memory tags, Constraints[] memory calls, uint256[] memory earlier, uint256[] memory later)
        private
        pure
        returns (uint256 count)
    {
        uint256 callCount = calls.length;

        for (uint256 j = 0; j < callCount; ++j) {
            // The calls of a resource keep their order.
            if (j != 0 && calls[j - 1].tag == calls[j].tag) {
                count = _addEdge({earlier: earlier, later: later, count: count, first: j - 1, second: j});
            }

            bytes32[] memory afterTags = calls[j].afterTags;
            uint256 afterTagCount = afterTags.length;
            for (uint256 a = 0; a < afterTagCount; ++a) {
                uint256 countBefore = count;
                for (uint256 i = 0; i < callCount; ++i) {
                    if (calls[i].tag == afterTags[a]) {
                        count = _addEdge({earlier: earlier, later: later, count: count, first: i, second: j});
                    }
                }
                require(
                    count != countBefore || _contains({tags: tags, tag: afterTags[a]}),
                    AfterTagNotInTransaction(afterTags[a])
                );
            }

            bytes32[] memory beforeTags = calls[j].beforeTags;
            uint256 beforeTagCount = beforeTags.length;
            for (uint256 b = 0; b < beforeTagCount; ++b) {
                for (uint256 i = 0; i < callCount; ++i) {
                    if (calls[i].tag == beforeTags[b]) {
                        count = _addEdge({earlier: earlier, later: later, count: count, first: j, second: i});
                    }
                }
            }
        }
    }

    /// @notice Writes an edge if the arrays have room for it.
    /// @param earlier The calls that run first.
    /// @param later The calls that run second.
    /// @param count The number of edges so far.
    /// @param first The call that runs first.
    /// @param second The call that runs second.
    /// @return newCount The number of edges with this one.
    function _addEdge(uint256[] memory earlier, uint256[] memory later, uint256 count, uint256 first, uint256 second)
        private
        pure
        returns (uint256 newCount)
    {
        if (count < earlier.length) {
            earlier[count] = first;
            later[count] = second;
        }
        newCount = count + 1;
    }

    /// @notice Returns the first call that has not run and waits for no other call.
    /// @param predecessorCounts The number of calls that must still run before each call.
    /// @param done Whether each call has run.
    /// @return next The index of the call, or the number of calls if no call can run.
    function _firstReady(uint256[] memory predecessorCounts, bool[] memory done) private pure returns (uint256 next) {
        uint256 callCount = done.length;
        for (next = 0; next < callCount; ++next) {
            if (!done[next] && predecessorCounts[next] == 0) {
                return next;
            }
        }
    }

    /// @notice Returns whether any call carries a constraint.
    /// @param calls The constraints of the calls.
    /// @return constrained Whether a call has an `afterTags` or a `beforeTags` entry.
    function _hasConstraints(Constraints[] memory calls) private pure returns (bool constrained) {
        uint256 callCount = calls.length;
        for (uint256 i = 0; i < callCount; ++i) {
            if (calls[i].afterTags.length != 0 || calls[i].beforeTags.length != 0) {
                return true;
            }
        }
    }

    /// @notice Returns whether a list of tags contains a tag.
    /// @param tags The list of tags.
    /// @param tag The tag to look for.
    /// @return found Whether the list contains the tag.
    function _contains(bytes32[] memory tags, bytes32 tag) private pure returns (bool found) {
        uint256 tagCount = tags.length;
        for (uint256 i = 0; i < tagCount; ++i) {
            if (tags[i] == tag) {
                return true;
            }
        }
    }
}
