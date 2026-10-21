// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../interfaces/IProtocolAdapter.sol";

/// @title ForwarderCallOrder
/// @author Anoma Foundation, 2026
/// @notice A library decoding the forwarder calls of a transaction into call lists and ordering the call lists by the
/// ordering constraints of the calls.
/// @custom:security-contact security@anoma.foundation
library ForwarderCallOrder {
    /// @notice The forwarder calls in the external payload of a resource. They run together and in their order.
    /// @param resourceTag The tag of the resource.
    /// @param resourceLogicRef The logic reference of the resource.
    /// @param actionIndex The index of the action of the resource.
    /// @param calls The forwarder calls, in the order of the external payload.
    struct CallList {
        bytes32 resourceTag;
        bytes32 resourceLogicRef;
        uint256 actionIndex;
        ForwarderCall[] calls;
    }

    /// @notice A forwarder call and its ordering constraints, which hold for all calls of its call list.
    /// @param untrustedForwarder The forwarder contract to call.
    /// @param input The input of the call.
    /// @param expectedOutput The output that the call must return.
    /// @param afterTags The tags of the resources whose calls run before the calls of the call list of this call.
    /// @param beforeTags The tags of the resources whose calls run after the calls of the call list of this call.
    struct ForwarderCall {
        address untrustedForwarder;
        bytes input;
        bytes expectedOutput;
        bytes32[] afterTags;
        bytes32[] beforeTags;
    }

    /// @notice Edges between call lists or between actions. An edge is a pair of items whose forwarder calls run in
    /// this order.
    /// @param predecessors The item at the start of each edge.
    /// @param successors The item at the end of each edge.
    /// @param count The number of edges.
    struct Edges {
        uint256[] predecessors;
        uint256[] successors;
        uint256 count;
    }

    /// @notice The incoming edge count of an item that is in the order.
    uint256 private constant _ORDERED = type(uint256).max;

    /// @notice Thrown if an `afterTags` entry names no resource of the transaction.
    error AfterTagNotInTransaction(bytes32 tag);

    /// @notice Thrown if a `beforeTags` entry names no resource of the transaction.
    error BeforeTagNotInTransaction(bytes32 tag);

    /// @notice Thrown if the ordering constraints form a cycle, so that no order meets them.
    error CyclicConstraints();

    /// @notice Returns whether a forwarder call of a resource carries ordering constraints.
    /// @dev Reads only the lengths of `afterTags` and `beforeTags`, whose offsets are the fourth and fifth words of
    /// the encoded call. The slicing reverts if the blob is too short or an offset points outside it.
    /// @param externalPayload The external payload of the resource.
    /// @return constrained Whether a call has an `afterTags` or a `beforeTags` entry.
    function hasOrderingConstraints(IProtocolAdapter.ExpirableBlob[] calldata externalPayload)
        internal
        pure
        returns (bool constrained)
    {
        uint256 callCount = externalPayload.length;
        for (uint256 i = 0; i < callCount; ++i) {
            bytes calldata blob = externalPayload[i].blob;
            if (_wordAt(blob, _wordAt(blob, 96)) != 0 || _wordAt(blob, _wordAt(blob, 128)) != 0) {
                return true;
            }
        }
    }

    /// @notice Decodes the call lists of a transaction, in the order of the actions, with the consumed resources of an
    /// action before its created resources.
    /// @param actions The actions of the transaction.
    /// @param callListCount The number of resources that carry forwarder calls.
    /// @return callLists The call lists of the resources that carry forwarder calls.
    /// @return constraintCount The number of `afterTags` and `beforeTags` entries of all calls.
    function decodeCallLists(IProtocolAdapter.Action[] calldata actions, uint256 callListCount)
        internal
        pure
        returns (CallList[] memory callLists, uint256 constraintCount)
    {
        callLists = new CallList[](callListCount);
        uint256 callListIndex = 0;

        uint256 actionCount = actions.length;
        for (uint256 i = 0; i < actionCount; ++i) {
            IProtocolAdapter.Consumed[] calldata consumedResources = actions[i].consumed;
            uint256 consumedCount = consumedResources.length;
            for (uint256 j = 0; j < consumedCount; ++j) {
                IProtocolAdapter.Consumed calldata consumed = consumedResources[j];
                IProtocolAdapter.ExpirableBlob[] calldata externalPayload = consumed.appData.externalPayload;
                if (externalPayload.length != 0) {
                    CallList memory callList = callLists[callListIndex++];
                    callList.actionIndex = i;
                    constraintCount += _decodeCallList({
                        callList: callList,
                        resourceTag: consumed.nullifier,
                        resourceLogicRef: consumed.logicRef,
                        externalPayload: externalPayload
                    });
                }
            }

            IProtocolAdapter.Created[] calldata createdResources = actions[i].created;
            uint256 createdCount = createdResources.length;
            for (uint256 j = 0; j < createdCount; ++j) {
                IProtocolAdapter.Created calldata created = createdResources[j];
                IProtocolAdapter.ExpirableBlob[] calldata externalPayload = created.appData.externalPayload;
                if (externalPayload.length != 0) {
                    CallList memory callList = callLists[callListIndex++];
                    callList.actionIndex = i;
                    constraintCount += _decodeCallList({
                        callList: callList,
                        resourceTag: created.commitment,
                        resourceLogicRef: created.logicRef,
                        externalPayload: externalPayload
                    });
                }
            }
        }
    }

    /// @notice Orders the call lists so that their forwarder calls meet the ordering constraints, and checks that
    /// each `afterTags` and `beforeTags` entry names a resource of the transaction. The calls of an action run
    /// together: a constraint between two call lists of one action orders the call lists, and a constraint between
    /// call lists of two actions orders the actions. Of the actions that can run next, the first runs, and so does the
    /// first call list within an action.
    /// @param callLists The call lists of the transaction.
    /// @param actions The actions of the transaction.
    /// @param constraintCount The number of `afterTags` and `beforeTags` entries of all calls.
    /// @return orderedIndices The indices of `callLists`, in the order in which their calls run.
    function order(CallList[] memory callLists, IProtocolAdapter.Action[] calldata actions, uint256 constraintCount)
        internal
        pure
        returns (uint256[] memory orderedIndices)
    {
        // Each ordering constraint adds at most one edge.
        Edges memory callListEdges =
            Edges({predecessors: new uint256[](constraintCount), successors: new uint256[](constraintCount), count: 0});
        Edges memory actionEdges =
            Edges({predecessors: new uint256[](constraintCount), successors: new uint256[](constraintCount), count: 0});
        _addEdges({callLists: callLists, actions: actions, callListEdges: callListEdges, actionEdges: actionEdges});

        uint256 callListCount = callLists.length;
        uint256[] memory callListOrder = orderByEdges({
            itemCount: callListCount,
            predecessors: callListEdges.predecessors,
            successors: callListEdges.successors,
            edgeCount: callListEdges.count
        });

        // The call lists of an action have lower indices than those of later actions, and their edges stay within an
        // action, so without edges between actions their order runs the actions one after another.
        if (actionEdges.count == 0) {
            return callListOrder;
        }

        uint256[] memory actionOrder = orderByEdges({
            itemCount: actions.length,
            predecessors: actionEdges.predecessors,
            successors: actionEdges.successors,
            edgeCount: actionEdges.count
        });

        // The edges between call lists stay within an action, so their order restricted to an action is the order of
        // the call lists of that action.
        orderedIndices = new uint256[](callListCount);
        uint256 position = 0;
        uint256 actionCount = actionOrder.length;
        for (uint256 i = 0; i < actionCount; ++i) {
            for (uint256 k = 0; k < callListCount; ++k) {
                if (callLists[callListOrder[k]].actionIndex == actionOrder[i]) {
                    orderedIndices[position++] = callListOrder[k];
                }
            }
        }
    }

    /// @notice Orders the items, which are call lists or actions, so that the predecessor of each edge comes before its
    /// successor. An edge is a pair of items whose forwarder calls run in this order. Of the items that wait for no
    /// other item, the item with the lowest index comes next.
    /// @param itemCount The number of items.
    /// @param predecessors The item at the start of each edge.
    /// @param successors The item at the end of each edge.
    /// @param edgeCount The number of edges in `predecessors` and `successors`.
    /// @return orderedIndices The indices of the items, in the order in which their calls run.
    function orderByEdges(
        uint256 itemCount,
        uint256[] memory predecessors,
        uint256[] memory successors,
        uint256 edgeCount
    ) internal pure returns (uint256[] memory orderedIndices) {
        uint256[] memory incomingEdgeCounts = new uint256[](itemCount);
        for (uint256 e = 0; e < edgeCount; ++e) {
            ++incomingEdgeCounts[successors[e]];
        }

        orderedIndices = new uint256[](itemCount);
        for (uint256 position = 0; position < itemCount; ++position) {
            uint256 ready = _firstReady(incomingEdgeCounts);
            require(ready != itemCount, CyclicConstraints());

            // No edge from an item that is not in the order ends at one that is, so no decrement reaches this value.
            incomingEdgeCounts[ready] = _ORDERED;
            orderedIndices[position] = ready;
            for (uint256 e = 0; e < edgeCount; ++e) {
                if (predecessors[e] == ready) {
                    --incomingEdgeCounts[successors[e]];
                }
            }
        }
    }

    /// @notice Writes the tag and the logic reference of a resource to its call list and decodes its forwarder calls.
    /// @param callList The call list to fill.
    /// @param resourceTag The tag of the resource.
    /// @param resourceLogicRef The logic reference of the resource.
    /// @param externalPayload The external payload of the resource.
    /// @return constraintCount The number of `afterTags` and `beforeTags` entries of the calls.
    function _decodeCallList(
        CallList memory callList,
        bytes32 resourceTag,
        bytes32 resourceLogicRef,
        IProtocolAdapter.ExpirableBlob[] calldata externalPayload
    ) private pure returns (uint256 constraintCount) {
        callList.resourceTag = resourceTag;
        callList.resourceLogicRef = resourceLogicRef;

        uint256 callCount = externalPayload.length;
        callList.calls = new ForwarderCall[](callCount);
        for (uint256 i = 0; i < callCount; ++i) {
            ForwarderCall memory forwarderCall = callList.calls[i];
            (
                forwarderCall.untrustedForwarder,
                forwarderCall.input,
                forwarderCall.expectedOutput,
                forwarderCall.afterTags,
                forwarderCall.beforeTags
            ) = abi.decode(externalPayload[i].blob, (address, bytes, bytes, bytes32[], bytes32[]));
            constraintCount += forwarderCall.afterTags.length + forwarderCall.beforeTags.length;
        }
    }

    /// @notice Writes one edge for each ordering constraint that names the resource of a call list, and checks that
    /// each `afterTags` and `beforeTags` entry names a resource of the transaction.
    /// @param callLists The call lists of the transaction.
    /// @param actions The actions of the transaction.
    /// @param callListEdges The edges between call lists of one action.
    /// @param actionEdges The edges between actions.
    function _addEdges(
        CallList[] memory callLists,
        IProtocolAdapter.Action[] calldata actions,
        Edges memory callListEdges,
        Edges memory actionEdges
    ) private pure {
        uint256 callListCount = callLists.length;

        for (uint256 callListIndex = 0; callListIndex < callListCount; ++callListIndex) {
            ForwarderCall[] memory calls = callLists[callListIndex].calls;
            uint256 callCount = calls.length;

            for (uint256 c = 0; c < callCount; ++c) {
                bytes32[] memory afterTags = calls[c].afterTags;
                uint256 afterTagCount = afterTags.length;
                for (uint256 a = 0; a < afterTagCount; ++a) {
                    bytes32 afterTag = afterTags[a];
                    uint256 predecessor = _indexOf({callLists: callLists, tag: afterTag});
                    if (predecessor == callListCount) {
                        // A resource without calls has no calls to wait for.
                        require(
                            _isTagOfTransaction({actions: actions, tag: afterTag}), AfterTagNotInTransaction(afterTag)
                        );
                        continue;
                    }
                    _addEdge({
                        callLists: callLists,
                        callListEdges: callListEdges,
                        actionEdges: actionEdges,
                        predecessor: predecessor,
                        successor: callListIndex
                    });
                }

                bytes32[] memory beforeTags = calls[c].beforeTags;
                uint256 beforeTagCount = beforeTags.length;
                for (uint256 b = 0; b < beforeTagCount; ++b) {
                    bytes32 beforeTag = beforeTags[b];
                    uint256 successor = _indexOf({callLists: callLists, tag: beforeTag});
                    if (successor == callListCount) {
                        // A resource without calls has no calls to run before.
                        require(
                            _isTagOfTransaction({actions: actions, tag: beforeTag}),
                            BeforeTagNotInTransaction(beforeTag)
                        );
                        continue;
                    }
                    _addEdge({
                        callLists: callLists,
                        callListEdges: callListEdges,
                        actionEdges: actionEdges,
                        predecessor: callListIndex,
                        successor: successor
                    });
                }
            }
        }
    }

    /// @notice Writes the edge between two call lists if they belong to one action, and else the edge between their
    /// actions.
    /// @param callLists The call lists of the transaction.
    /// @param callListEdges The edges between call lists of one action.
    /// @param actionEdges The edges between actions.
    /// @param predecessor The call list whose calls run first.
    /// @param successor The call list whose calls run second.
    function _addEdge(
        CallList[] memory callLists,
        Edges memory callListEdges,
        Edges memory actionEdges,
        uint256 predecessor,
        uint256 successor
    ) private pure {
        Edges memory edges = callListEdges;
        uint256 predecessorAction = callLists[predecessor].actionIndex;
        uint256 successorAction = callLists[successor].actionIndex;
        if (predecessorAction != successorAction) {
            (edges, predecessor, successor) = (actionEdges, predecessorAction, successorAction);
        }
        edges.predecessors[edges.count] = predecessor;
        edges.successors[edges.count] = successor;
        ++edges.count;
    }

    /// @notice Returns the index of the call list of the resource with a tag.
    /// @param callLists The call lists of the transaction.
    /// @param tag The tag to look for.
    /// @return index The index of the call list, or the number of call lists if no call list has the resource tag.
    function _indexOf(CallList[] memory callLists, bytes32 tag) private pure returns (uint256 index) {
        uint256 callListCount = callLists.length;
        for (index = 0; index < callListCount; ++index) {
            if (callLists[index].resourceTag == tag) {
                return index;
            }
        }
    }

    /// @notice Returns whether a tag is the nullifier or the commitment of a resource of a transaction.
    /// @param actions The actions of the transaction.
    /// @param tag The tag to look for.
    /// @return found Whether a resource of the transaction has the tag.
    function _isTagOfTransaction(IProtocolAdapter.Action[] calldata actions, bytes32 tag)
        private
        pure
        returns (bool found)
    {
        uint256 actionCount = actions.length;
        for (uint256 i = 0; i < actionCount; ++i) {
            IProtocolAdapter.Consumed[] calldata consumedResources = actions[i].consumed;
            uint256 consumedCount = consumedResources.length;
            for (uint256 j = 0; j < consumedCount; ++j) {
                if (consumedResources[j].nullifier == tag) {
                    return true;
                }
            }

            IProtocolAdapter.Created[] calldata createdResources = actions[i].created;
            uint256 createdCount = createdResources.length;
            for (uint256 j = 0; j < createdCount; ++j) {
                if (createdResources[j].commitment == tag) {
                    return true;
                }
            }
        }
    }

    /// @notice Returns the lowest index of an item that is not in the order and waits for no other item.
    /// @param incomingEdgeCounts The number of edges to each item from items that are not in the order.
    /// @return ready The index of the item, or the number of items if no item is ready.
    function _firstReady(uint256[] memory incomingEdgeCounts) private pure returns (uint256 ready) {
        uint256 itemCount = incomingEdgeCounts.length;
        for (ready = 0; ready < itemCount; ++ready) {
            if (incomingEdgeCounts[ready] == 0) {
                return ready;
            }
        }
    }

    /// @notice Returns the 32-byte word of a blob at an offset.
    /// @param blob The blob to read from.
    /// @param offset The offset of the word in the blob.
    /// @return word The word.
    function _wordAt(bytes calldata blob, uint256 offset) private pure returns (uint256 word) {
        word = uint256(bytes32(blob[offset:offset + 32]));
    }
}
