// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../interfaces/IProtocolAdapter.sol";

/// @title ForwarderCallOrder
/// @author Anoma Foundation, 2026
/// @notice A library decoding the forwarder calls of a transaction and ordering their carriers by the ordering
/// constraints of the calls.
/// @custom:security-contact security@anoma.foundation
library ForwarderCallOrder {
    /// @notice A resource with forwarder calls in its external payload.
    /// @param tag The tag of the resource.
    /// @param logicRef The logic reference of the resource.
    /// @param calls The forwarder calls of the resource, in the order of its external payload.
    struct Carrier {
        bytes32 tag;
        bytes32 logicRef;
        ForwarderCall[] calls;
    }

    /// @notice A forwarder call and its ordering constraints, which hold for all calls of its carrier.
    /// @param untrustedForwarder The forwarder contract to call.
    /// @param input The input of the call.
    /// @param expectedOutput The output that the call must return.
    /// @param afterTags The tags of the resources whose calls run before the calls of the carrier.
    /// @param beforeTags The tags of the resources whose calls run after the calls of the carrier.
    struct ForwarderCall {
        address untrustedForwarder;
        bytes input;
        bytes expectedOutput;
        bytes32[] afterTags;
        bytes32[] beforeTags;
    }

    /// @notice The incoming edge count of a carrier that is in the order.
    uint256 private constant _ORDERED = type(uint256).max;

    /// @notice Thrown if an `afterTags` entry names no resource of the transaction.
    error AfterTagNotInTransaction(bytes32 tag);

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

    /// @notice Decodes the forwarder calls of the carriers of a transaction, in the order of the actions, with the
    /// consumed resources of an action before its created resources.
    /// @param actions The actions of the transaction.
    /// @param carrierCount The number of resources that carry forwarder calls.
    /// @return carriers The resources that carry forwarder calls, with their decoded calls.
    /// @return constraintCount The number of `afterTags` and `beforeTags` entries of all calls.
    function decodeCarriers(IProtocolAdapter.Action[] calldata actions, uint256 carrierCount)
        internal
        pure
        returns (Carrier[] memory carriers, uint256 constraintCount)
    {
        carriers = new Carrier[](carrierCount);
        uint256 carrierIndex = 0;

        uint256 actionCount = actions.length;
        for (uint256 i = 0; i < actionCount; ++i) {
            IProtocolAdapter.Consumed[] calldata consumedResources = actions[i].consumed;
            uint256 consumedCount = consumedResources.length;
            for (uint256 j = 0; j < consumedCount; ++j) {
                IProtocolAdapter.Consumed calldata consumed = consumedResources[j];
                IProtocolAdapter.ExpirableBlob[] calldata externalPayload = consumed.appData.externalPayload;
                if (externalPayload.length != 0) {
                    constraintCount += _decodeCarrier({
                        carrier: carriers[carrierIndex++],
                        tag: consumed.nullifier,
                        logicRef: consumed.logicRef,
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
                    constraintCount += _decodeCarrier({
                        carrier: carriers[carrierIndex++],
                        tag: created.commitment,
                        logicRef: created.logicRef,
                        externalPayload: externalPayload
                    });
                }
            }
        }
    }

    /// @notice Orders the carriers so that their forwarder calls meet the ordering constraints, and checks that each
    /// `afterTags` entry names a resource of the transaction.
    /// @param carriers The carriers of the transaction.
    /// @param actions The actions of the transaction.
    /// @param constraintCount The number of `afterTags` and `beforeTags` entries of all calls.
    /// @return orderedIndices The indices of `carriers`, in the order in which their calls run.
    function order(Carrier[] memory carriers, IProtocolAdapter.Action[] calldata actions, uint256 constraintCount)
        internal
        pure
        returns (uint256[] memory orderedIndices)
    {
        // Each ordering constraint adds at most one edge.
        uint256[] memory predecessors = new uint256[](constraintCount);
        uint256[] memory successors = new uint256[](constraintCount);
        uint256 edgeCount =
            _addEdges({carriers: carriers, actions: actions, predecessors: predecessors, successors: successors});

        orderedIndices = orderByEdges({
            carrierCount: carriers.length, predecessors: predecessors, successors: successors, edgeCount: edgeCount
        });
    }

    /// @notice Orders the carriers so that the predecessor of each edge comes before its successor. An edge is a pair
    /// of carriers whose forwarder calls run in this order. Of the carriers that wait for no other carrier, the
    /// carrier with the lowest index comes next.
    /// @param carrierCount The number of carriers.
    /// @param predecessors The carrier at the start of each edge.
    /// @param successors The carrier at the end of each edge.
    /// @param edgeCount The number of edges in `predecessors` and `successors`.
    /// @return orderedIndices The indices of the carriers, in the order in which their calls run.
    function orderByEdges(
        uint256 carrierCount,
        uint256[] memory predecessors,
        uint256[] memory successors,
        uint256 edgeCount
    ) internal pure returns (uint256[] memory orderedIndices) {
        // The edges out of each carrier form a list: `lastEdge` holds one more than the index of the last edge out of
        // a carrier, and `previousEdge` one more than the index of the edge out of the same carrier before an edge.
        // Zero ends the list.
        uint256[] memory incomingEdgeCounts = new uint256[](carrierCount);
        uint256[] memory lastEdge = new uint256[](carrierCount);
        uint256[] memory previousEdge = new uint256[](edgeCount);
        for (uint256 e = 0; e < edgeCount; ++e) {
            ++incomingEdgeCounts[successors[e]];
            previousEdge[e] = lastEdge[predecessors[e]];
            lastEdge[predecessors[e]] = e + 1;
        }

        orderedIndices = new uint256[](carrierCount);
        for (uint256 position = 0; position < carrierCount; ++position) {
            uint256 ready = _firstReady(incomingEdgeCounts);
            require(ready != carrierCount, CyclicConstraints());

            // No edge from a carrier that is not in the order ends at one that is, so no decrement reaches this value.
            incomingEdgeCounts[ready] = _ORDERED;
            orderedIndices[position] = ready;
            for (uint256 e = lastEdge[ready]; e != 0; e = previousEdge[e - 1]) {
                --incomingEdgeCounts[successors[e - 1]];
            }
        }
    }

    /// @notice Writes the tag and the logic reference of a carrier and decodes its forwarder calls.
    /// @param carrier The carrier to fill.
    /// @param tag The tag of the resource.
    /// @param logicRef The logic reference of the resource.
    /// @param externalPayload The external payload of the resource.
    /// @return constraintCount The number of `afterTags` and `beforeTags` entries of the calls.
    function _decodeCarrier(
        Carrier memory carrier,
        bytes32 tag,
        bytes32 logicRef,
        IProtocolAdapter.ExpirableBlob[] calldata externalPayload
    ) private pure returns (uint256 constraintCount) {
        carrier.tag = tag;
        carrier.logicRef = logicRef;

        uint256 callCount = externalPayload.length;
        carrier.calls = new ForwarderCall[](callCount);
        for (uint256 i = 0; i < callCount; ++i) {
            ForwarderCall memory forwarderCall = carrier.calls[i];
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

    /// @notice Writes one edge for each ordering constraint that names a carrier, and checks that each `afterTags`
    /// entry names a resource of the transaction.
    /// @param carriers The carriers of the transaction.
    /// @param actions The actions of the transaction.
    /// @param predecessors The carrier at the start of each edge.
    /// @param successors The carrier at the end of each edge.
    /// @return edgeCount The number of edges.
    function _addEdges(
        Carrier[] memory carriers,
        IProtocolAdapter.Action[] calldata actions,
        uint256[] memory predecessors,
        uint256[] memory successors
    ) private pure returns (uint256 edgeCount) {
        uint256 carrierCount = carriers.length;

        for (uint256 carrierIndex = 0; carrierIndex < carrierCount; ++carrierIndex) {
            ForwarderCall[] memory calls = carriers[carrierIndex].calls;
            uint256 callCount = calls.length;

            for (uint256 c = 0; c < callCount; ++c) {
                bytes32[] memory afterTags = calls[c].afterTags;
                uint256 afterTagCount = afterTags.length;
                for (uint256 a = 0; a < afterTagCount; ++a) {
                    bytes32 afterTag = afterTags[a];
                    uint256 predecessor = _indexOf({carriers: carriers, tag: afterTag});
                    if (predecessor == carrierCount) {
                        // A resource without calls has no calls to wait for.
                        require(
                            _isTagOfTransaction({actions: actions, tag: afterTag}), AfterTagNotInTransaction(afterTag)
                        );
                        continue;
                    }
                    predecessors[edgeCount] = predecessor;
                    successors[edgeCount] = carrierIndex;
                    ++edgeCount;
                }

                bytes32[] memory beforeTags = calls[c].beforeTags;
                uint256 beforeTagCount = beforeTags.length;
                for (uint256 b = 0; b < beforeTagCount; ++b) {
                    uint256 successor = _indexOf({carriers: carriers, tag: beforeTags[b]});
                    if (successor != carrierCount) {
                        predecessors[edgeCount] = carrierIndex;
                        successors[edgeCount] = successor;
                        ++edgeCount;
                    }
                }
            }
        }
    }

    /// @notice Returns the index of the carrier with a tag.
    /// @param carriers The carriers of the transaction.
    /// @param tag The tag to look for.
    /// @return index The index of the carrier, or the number of carriers if no carrier has the tag.
    function _indexOf(Carrier[] memory carriers, bytes32 tag) private pure returns (uint256 index) {
        uint256 carrierCount = carriers.length;
        for (index = 0; index < carrierCount; ++index) {
            if (carriers[index].tag == tag) {
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

    /// @notice Returns the 32-byte word of a blob at an offset.
    /// @param blob The blob to read from.
    /// @param offset The offset of the word in the blob.
    /// @return word The word.
    function _wordAt(bytes calldata blob, uint256 offset) private pure returns (uint256 word) {
        word = uint256(bytes32(blob[offset:offset + 32]));
    }
}
