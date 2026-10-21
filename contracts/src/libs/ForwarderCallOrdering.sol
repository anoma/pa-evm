// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../interfaces/IProtocolAdapter.sol";
import {DirectedGraph} from "../libs/DirectedGraph.sol";

/// @title ForwarderCallOrdering
/// @author Anoma Foundation, 2026
/// @notice A library decoding the forwarder calls of a transaction into call lists and ordering the call lists by the
/// ordering constraints of the calls.
/// @dev `order` builds one graph whose nodes are the call lists. An edge means that the calls of its source are
/// executed before those of its target. A constraint between call lists of two actions adds an edge from each call list
/// of the earlier action to each call list of the later one, so that the calls of an action are executed together.
/// `order` sorts the graph with `DirectedGraph.topologicalSort`.
/// @custom:security-contact security@anoma.foundation
library ForwarderCallOrdering {
    using DirectedGraph for DirectedGraph.Graph;

    /// @notice The forwarder calls in the external payload of a resource. They are executed together and in their
    /// order.
    /// @param resourceTag The tag of the resource.
    /// @param resourceLogicRef The logic reference of the resource.
    /// @param actionIndex The index of the action of the resource.
    /// @param calls The forwarder calls, in the order of the external payload.
    struct CallList {
        bytes32 resourceTag;
        bytes32 resourceLogicRef;
        uint256 actionIndex;
        IProtocolAdapter.ForwarderCall[] calls;
    }

    /// @notice The position of the offset of `afterTags` in an encoded forwarder call, which is its fourth word.
    uint256 private constant _AFTER_TAGS_OFFSET_POSITION = 3 * 32;

    /// @notice The position of the offset of `beforeTags` in an encoded forwarder call, which is its fifth word.
    uint256 private constant _BEFORE_TAGS_OFFSET_POSITION = 4 * 32;

    /// @notice Thrown if an `afterTags` entry names no resource of the transaction.
    error AfterTagNotInTransaction(bytes32 tag);

    /// @notice Thrown if a `beforeTags` entry names no resource of the transaction.
    error BeforeTagNotInTransaction(bytes32 tag);

    /// @notice Thrown if the ordering constraints form a cycle, so that no order meets them.
    error CyclicOrderingConstraints();

    /// @notice Thrown if a transaction with ordering constraints has more than `DirectedGraph.MAX_NODE_COUNT`
    /// call-carrying resources.
    error TooManyCallCarryingResources(uint256 count);

    /// @notice Returns whether a forwarder call of a resource carries ordering constraints.
    /// @dev Reads only the lengths of `afterTags` and `beforeTags`. The slicing reverts if the blob is too short or an
    /// offset points outside it.
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
            if (_tagCount(blob, _AFTER_TAGS_OFFSET_POSITION) != 0 || _tagCount(blob, _BEFORE_TAGS_OFFSET_POSITION) != 0)
            {
                return true;
            }
        }
    }

    /// @notice Decodes the call lists of a transaction, in the order of the actions, with the consumed resources of an
    /// action before its created resources.
    /// @param actions The actions of the transaction.
    /// @param callListCount The number of resources that carry forwarder calls.
    /// @return callLists The call lists of the resources that carry forwarder calls.
    function decodeCallLists(IProtocolAdapter.Action[] calldata actions, uint256 callListCount)
        internal
        pure
        returns (CallList[] memory callLists)
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
                    _decodeCallList({
                        callList: callLists[callListIndex++],
                        actionIndex: i,
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
                    _decodeCallList({
                        callList: callLists[callListIndex++],
                        actionIndex: i,
                        resourceTag: created.commitment,
                        resourceLogicRef: created.logicRef,
                        externalPayload: externalPayload
                    });
                }
            }
        }
    }

    /// @notice Orders the call lists so that their forwarder calls meet the ordering constraints, and checks that
    /// each `afterTags` and `beforeTags` entry names a resource of the transaction. The NatSpec of
    /// `ProtocolAdapter._executeForwarderCalls` states the rules of the order.
    /// @param callLists The call lists of the transaction.
    /// @param actions The actions of the transaction.
    /// @return orderedIndices The indices of `callLists`, in the order in which their calls are executed.
    function order(CallList[] memory callLists, IProtocolAdapter.Action[] calldata actions)
        internal
        pure
        returns (uint256[] memory orderedIndices)
    {
        uint256 callListCount = callLists.length;
        require(callListCount <= DirectedGraph.MAX_NODE_COUNT, TooManyCallCarryingResources(callListCount));

        DirectedGraph.Graph memory constraints = _buildConstraints({callLists: callLists, actions: actions});

        bool isAcyclic;
        (isAcyclic, orderedIndices) = constraints.topologicalSort();
        require(isAcyclic, CyclicOrderingConstraints());
    }

    /// @notice Writes the action index, the tag and the logic reference of a resource to its call list and decodes its
    /// forwarder calls.
    /// @param callList The call list to fill.
    /// @param actionIndex The index of the action of the resource.
    /// @param resourceTag The tag of the resource.
    /// @param resourceLogicRef The logic reference of the resource.
    /// @param externalPayload The external payload of the resource.
    function _decodeCallList(
        CallList memory callList,
        uint256 actionIndex,
        bytes32 resourceTag,
        bytes32 resourceLogicRef,
        IProtocolAdapter.ExpirableBlob[] calldata externalPayload
    ) private pure {
        callList.actionIndex = actionIndex;
        callList.resourceTag = resourceTag;
        callList.resourceLogicRef = resourceLogicRef;

        uint256 callCount = externalPayload.length;
        callList.calls = new IProtocolAdapter.ForwarderCall[](callCount);
        for (uint256 i = 0; i < callCount; ++i) {
            IProtocolAdapter.ForwarderCall memory forwarderCall = callList.calls[i];
            (
                forwarderCall.untrustedForwarder,
                forwarderCall.input,
                forwarderCall.expectedOutput,
                forwarderCall.afterTags,
                forwarderCall.beforeTags
            ) = abi.decode(externalPayload[i].blob, (address, bytes, bytes, bytes32[], bytes32[]));
        }
    }

    /// @notice Builds a graph whose nodes are the call lists, with an edge for each ordering constraint that names the
    /// resource of a call list. Checks that each `afterTags` and `beforeTags` entry names a resource of the
    /// transaction.
    /// @param callLists The call lists of the transaction.
    /// @param actions The actions of the transaction.
    /// @return constraints The ordering constraints, as a graph whose nodes are the call lists.
    function _buildConstraints(CallList[] memory callLists, IProtocolAdapter.Action[] calldata actions)
        private
        pure
        returns (DirectedGraph.Graph memory constraints)
    {
        uint256 callListCount = callLists.length;
        constraints.inNeighbors = new uint256[](callListCount);
        constraints.outNeighbors = new uint256[](callListCount);

        for (uint256 callListIndex = 0; callListIndex < callListCount; ++callListIndex) {
            IProtocolAdapter.ForwarderCall[] memory calls = callLists[callListIndex].calls;
            uint256 callCount = calls.length;

            for (uint256 c = 0; c < callCount; ++c) {
                IProtocolAdapter.ForwarderCall memory forwarderCall = calls[c];
                bytes32[] memory afterTags = forwarderCall.afterTags;
                uint256 afterTagCount = afterTags.length;
                for (uint256 a = 0; a < afterTagCount; ++a) {
                    bytes32 afterTag = afterTags[a];
                    uint256 source = _indexOf({callLists: callLists, tag: afterTag});
                    if (source == callListCount) {
                        // A resource without calls has no calls to wait for.
                        require(
                            _isTagInTransaction({actions: actions, tag: afterTag}), AfterTagNotInTransaction(afterTag)
                        );
                        continue;
                    }
                    _addEdge({callLists: callLists, constraints: constraints, source: source, target: callListIndex});
                }

                bytes32[] memory beforeTags = forwarderCall.beforeTags;
                uint256 beforeTagCount = beforeTags.length;
                for (uint256 b = 0; b < beforeTagCount; ++b) {
                    bytes32 beforeTag = beforeTags[b];
                    uint256 target = _indexOf({callLists: callLists, tag: beforeTag});
                    if (target == callListCount) {
                        // A resource without calls has no calls that wait for these.
                        require(
                            _isTagInTransaction({actions: actions, tag: beforeTag}),
                            BeforeTagNotInTransaction(beforeTag)
                        );
                        continue;
                    }
                    _addEdge({callLists: callLists, constraints: constraints, source: callListIndex, target: target});
                }
            }
        }
    }

    /// @notice Adds the edge between two call lists if they belong to one action, and else an edge from each call list
    /// of the action of `source` to each call list of the action of `target`.
    /// @param callLists The call lists of the transaction.
    /// @param constraints The ordering constraints, as a graph whose nodes are the call lists.
    /// @param source The call list whose calls are executed before those of `target`.
    /// @param target The call list whose calls are executed after those of `source`.
    function _addEdge(
        CallList[] memory callLists,
        DirectedGraph.Graph memory constraints,
        uint256 source,
        uint256 target
    ) private pure {
        if (callLists[source].actionIndex == callLists[target].actionIndex) {
            constraints.addEdge({source: source, target: target});
            return;
        }
        (uint256 sourceStart, uint256 sourceEnd) = _callListsOfAction(callLists, source);
        (uint256 targetStart, uint256 targetEnd) = _callListsOfAction(callLists, target);
        constraints.addEdges({
            sourceStart: sourceStart, sourceEnd: sourceEnd, targetStart: targetStart, targetEnd: targetEnd
        });
    }

    /// @notice Returns the range of the call lists of the action of a call list. The call lists of an action have
    /// consecutive indices.
    /// @param callLists The call lists of the transaction.
    /// @param index The index of a call list of the action.
    /// @return start The index of the first call list of the action.
    /// @return end The index after the last call list of the action.
    function _callListsOfAction(CallList[] memory callLists, uint256 index)
        private
        pure
        returns (uint256 start, uint256 end)
    {
        uint256 actionIndex = callLists[index].actionIndex;
        start = index;
        while (start != 0 && callLists[start - 1].actionIndex == actionIndex) {
            --start;
        }
        end = index + 1;
        uint256 callListCount = callLists.length;
        while (end != callListCount && callLists[end].actionIndex == actionIndex) {
            ++end;
        }
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
    function _isTagInTransaction(IProtocolAdapter.Action[] calldata actions, bytes32 tag)
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

    /// @notice Returns the number of entries of a tag list of an encoded forwarder call.
    /// @param blob The encoded forwarder call.
    /// @param offsetPosition The position of the offset of the tag list in the blob.
    /// @return count The number of tags.
    function _tagCount(bytes calldata blob, uint256 offsetPosition) private pure returns (uint256 count) {
        count = _wordAt(blob, _wordAt(blob, offsetPosition));
    }

    /// @notice Returns the 32-byte word of a blob at an offset.
    /// @param blob The blob to read from.
    /// @param offset The offset of the word in the blob.
    /// @return word The word.
    function _wordAt(bytes calldata blob, uint256 offset) private pure returns (uint256 word) {
        word = uint256(bytes32(blob[offset:offset + 32]));
    }
}
