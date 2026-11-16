// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {VmSafe} from "forge-std-1.17.0/src/Vm.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.1.0/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {DeltaProof} from "../../src/libs/DeltaProof.sol";
import {SHA256} from "../../src/libs/SHA256.sol";
import {VerifyingKeys} from "../../src/libs/VerifyingKeys.sol";
import {DeltaGen} from "./DeltaGen.sol";
import {MerkleTreeReference} from "./MerkleTreeReference.sol";

library TxGen {
    using MerkleTreeReference for bytes32[];

    /// @notice The resource object constituting the atomic unit of state in the Anoma protocol.
    /// @param  logicRef The hash of the resource logic function.
    /// @param  labelRef The hash of the resource label, which can contain arbitrary data.
    /// @param  valueRef The hash of the resource value, which can contain arbitrary data.
    /// @param  nullifierKeyCommitment The commitment to the nullifier key.
    /// @param  quantity The quantity that the resource represents.
    /// @param  nonce The nonce guaranteeing the resource's uniqueness.
    /// @param  randSeed The randomness seed that can be used to derive pseudo-randomness for applications.
    /// @param  ephemeral The resource's ephemerality.
    struct Resource {
        bytes32 logicRef;
        bytes32 labelRef;
        bytes32 valueRef;
        bytes32 nullifierKeyCommitment;
        bytes32 nonce;
        bytes32 randSeed;
        uint128 quantity;
        bool ephemeral;
    }

    /// @notice A resource of an action. `transaction` derives its `IProtocolAdapter.ResourceInstance` from it.
    struct ActionResource {
        bool consumed;
        Resource resource;
        IProtocolAdapter.AppData appData;
    }

    /// @dev Builds one action of each resource list, with the resources in list order. The caller balances the
    /// quantities across the transaction.
    function transaction(VmSafe vm, RiscZeroMockVerifier mockVerifier, ActionResource[][] memory actionResources)
        internal
        returns (IProtocolAdapter.Transaction memory txn)
    {
        IProtocolAdapter.Action[] memory actions = new IProtocolAdapter.Action[](actionResources.length);

        for (uint256 i = 0; i < actionResources.length; ++i) {
            actions[i] = _createAction({vm: vm, resources: actionResources[i]});
        }

        txn = _transactionFromActions({vm: vm, actions: actions});
        txn = transactionAggregation({mockVerifier: mockVerifier, txn: txn, kindTreeRoot: _emptyKindTreeRoot()});
    }

    /// @dev Builds a transaction of `actionCount` mock actions, each of the resources that `mockResources` gives.
    function transaction(
        VmSafe vm,
        RiscZeroMockVerifier mockVerifier,
        bytes32 nonce,
        uint256 actionCount,
        uint256 consumedCount,
        uint256 createdCount
    ) internal returns (IProtocolAdapter.Transaction memory txn, bytes32 updatedNonce) {
        updatedNonce = nonce;

        IProtocolAdapter.Action[] memory actions = new IProtocolAdapter.Action[](actionCount);
        for (uint256 i = 0; i < actionCount; ++i) {
            ActionResource[] memory resources;
            (resources, updatedNonce) =
                mockResources({nonce: updatedNonce, consumedCount: consumedCount, createdCount: createdCount});
            actions[i] = _createAction({vm: vm, resources: resources});
        }

        txn = _transactionFromActions({vm: vm, actions: actions});
        txn = transactionAggregation({mockVerifier: mockVerifier, txn: txn, kindTreeRoot: _emptyKindTreeRoot()});
    }

    /// @dev Mock-proves the aggregation against a kind tree: the transaction carries its root, and the seal
    /// commits to the journal the protocol adapter reconstructs with it and the compliance verifying key.
    function transactionAggregation(
        RiscZeroMockVerifier mockVerifier,
        IProtocolAdapter.Transaction memory txn,
        bytes32 kindTreeRoot
    ) internal view returns (IProtocolAdapter.Transaction memory aggregatedTxn) {
        bytes32 journalDigest = sha256(abi.encode(VerifyingKeys._COMPLIANCE, kindTreeRoot, txn.actions));

        aggregatedTxn = txn;
        aggregatedTxn.kindTreeRoot = kindTreeRoot;
        aggregatedTxn.aggregationProof =
        mockVerifier.mockProve({imageId: VerifyingKeys._BATCH_AGGREGATION_EVM, journalDigest: journalDigest}).seal;
    }

    /// @dev The resource lists of a transaction with one action of `resources`.
    function oneAction(ActionResource[] memory resources)
        internal
        pure
        returns (ActionResource[][] memory actionResources)
    {
        actionResources = new ActionResource[][](1);
        actionResources[0] = resources;
    }

    function consumed(Resource memory resource, IProtocolAdapter.AppData memory appData)
        internal
        pure
        returns (ActionResource memory actionResource)
    {
        actionResource = ActionResource({consumed: true, resource: resource, appData: appData});
    }

    function created(Resource memory resource, IProtocolAdapter.AppData memory appData)
        internal
        pure
        returns (ActionResource memory actionResource)
    {
        actionResource = ActionResource({consumed: false, resource: resource, appData: appData});
    }

    /// @dev `consumedCount` consumed, then `createdCount` created resources of one kind. A consumed resource has
    /// quantity `createdCount` and a created one `consumedCount`, so the action balances. `nonce` is also the logic
    /// ref and the label ref, so it sets the kind.
    function mockResources(bytes32 nonce, uint256 consumedCount, uint256 createdCount)
        internal
        pure
        returns (ActionResource[] memory resources, bytes32 updatedNonce)
    {
        resources = new ActionResource[](consumedCount + createdCount);

        for (uint256 i = 0; i < resources.length; ++i) {
            bool isConsumed = i < consumedCount;
            resources[i] = ActionResource({
                consumed: isConsumed,
                resource: mockResource({
                    nonce: bytes32(uint256(nonce) + i),
                    logicRef: nonce,
                    labelRef: nonce,
                    // Counts are small test inputs, so the cast cannot truncate.
                    // forge-lint: disable-next-line(unsafe-typecast)
                    quantity: uint128(isConsumed ? createdCount : consumedCount)
                }),
                appData: emptyAppData()
            });
        }

        updatedNonce = bytes32(uint256(nonce) + resources.length);
    }

    function mockResource(bytes32 nonce, bytes32 logicRef, bytes32 labelRef, uint128 quantity)
        internal
        pure
        returns (Resource memory mock)
    {
        mock = Resource({
            logicRef: logicRef,
            labelRef: labelRef,
            valueRef: bytes32(0),
            nullifierKeyCommitment: bytes32(0),
            quantity: quantity,
            nonce: nonce,
            randSeed: 0,
            ephemeral: true
        });
    }

    /// @dev The external payload blob of a forwarder call: the protocol adapter calls `forwarder` with `input` and
    /// requires `output` as the output of the call.
    function forwarderCall(address forwarder, bytes memory input, bytes memory output)
        internal
        pure
        returns (IProtocolAdapter.ExpirableBlob memory call)
    {
        call = IProtocolAdapter.ExpirableBlob({
            deletionCriterion: IProtocolAdapter.DeletionCriterion.Never, blob: abi.encode(forwarder, input, output)
        });
    }

    /// @dev App data whose only blob is the `forwarderCall` blob of `forwarder`, `input` and `output`.
    function forwarderCallAppData(address forwarder, bytes memory input, bytes memory output)
        internal
        pure
        returns (IProtocolAdapter.AppData memory appData)
    {
        appData = emptyAppData();
        appData.externalPayload = new IProtocolAdapter.ExpirableBlob[](1);
        appData.externalPayload[0] = forwarderCall({forwarder: forwarder, input: input, output: output});
    }

    function emptyAppData() internal pure returns (IProtocolAdapter.AppData memory appData) {
        appData = IProtocolAdapter.AppData({
            resourcePayload: new IProtocolAdapter.ExpirableBlob[](0),
            discoveryPayload: new IProtocolAdapter.ExpirableBlob[](0),
            externalPayload: new IProtocolAdapter.ExpirableBlob[](0),
            applicationPayload: new IProtocolAdapter.ExpirableBlob[](0)
        });
    }

    function transactionId(IProtocolAdapter.Transaction memory txn) internal pure returns (bytes32 id) {
        id = DeltaProof.computeVerifyingKey(_actionTreeRoots(txn.actions));
    }

    function collectNullifiers(IProtocolAdapter.Transaction memory txn)
        internal
        pure
        returns (bytes32[] memory nullifiers)
    {
        nullifiers = _collectTags({txn: txn, isConsumed: true});
    }

    function collectCommitments(IProtocolAdapter.Transaction memory txn)
        internal
        pure
        returns (bytes32[] memory commitments)
    {
        commitments = _collectTags({txn: txn, isConsumed: false});
    }

    function actionNullifiers(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory nullifiers)
    {
        (nullifiers,) = _actionTagsAndLogicRefs({action: action, isConsumed: true});
    }

    function actionConsumedLogicRefs(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory logicRefs)
    {
        (, logicRefs) = _actionTagsAndLogicRefs({action: action, isConsumed: true});
    }

    function actionCommitments(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory commitments)
    {
        (commitments,) = _actionTagsAndLogicRefs({action: action, isConsumed: false});
    }

    function actionCreatedLogicRefs(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory logicRefs)
    {
        (, logicRefs) = _actionTagsAndLogicRefs({action: action, isConsumed: false});
    }

    function commitment(Resource memory resource) internal pure returns (bytes32 hash) {
        hash = sha256(abi.encode(resource));
    }

    function nullifier(Resource memory resource, bytes32 nullifierKey) internal pure returns (bytes32 hash) {
        hash = sha256(abi.encode(resource, nullifierKey));
    }

    /// @dev Builds an action of the resources in list order.
    function _createAction(VmSafe vm, ActionResource[] memory resources)
        private
        returns (IProtocolAdapter.Action memory action)
    {
        IProtocolAdapter.ResourceInstance[] memory instances = new IProtocolAdapter.ResourceInstance[](resources.length);

        IProtocolAdapter.Delta memory actionDelta;

        for (uint256 i = 0; i < resources.length; ++i) {
            instances[i] = _resourceInstance(resources[i]);

            IProtocolAdapter.Delta memory resourceDelta = DeltaGen.generateInstance(
                vm,
                DeltaGen.InstanceInputs({
                    kind: _kind(resources[i].resource),
                    quantity: resources[i].resource.quantity,
                    consumed: resources[i].consumed,
                    valueCommitmentRandomness: _valueCommitmentRandomness(instances[i].tag)
                })
            );
            actionDelta = (i == 0) ? resourceDelta : DeltaProof.add(actionDelta, resourceDelta);
        }

        action = IProtocolAdapter.Action({
            resources: instances, unitDelta: actionDelta, actionTreeRoot: _computeActionTreeRoot(instances)
        });
    }

    /// @dev Assembles the transaction and its delta proof.
    function _transactionFromActions(VmSafe vm, IProtocolAdapter.Action[] memory actions)
        private
        returns (IProtocolAdapter.Transaction memory txn)
    {
        bytes memory proof = "";
        if (_countResources(actions) != 0) {
            proof = DeltaGen.generateProof(
                vm,
                DeltaGen.ProofInputs({
                    summedValueCommitmentRandomness: _summedValueCommitmentRandomness(actions),
                    verifyingKey: DeltaProof.computeVerifyingKey(_actionTreeRoots(actions))
                })
            );
        }

        txn = IProtocolAdapter.Transaction({
            actions: actions, deltaProof: proof, aggregationProof: "", kindTreeRoot: _emptyKindTreeRoot()
        });
    }

    function _resourceInstance(ActionResource memory actionResource)
        private
        pure
        returns (IProtocolAdapter.ResourceInstance memory instance)
    {
        bool isConsumed = actionResource.consumed;
        instance = IProtocolAdapter.ResourceInstance({
            consumed: isConsumed,
            tag: isConsumed ? nullifier(actionResource.resource, 0) : commitment(actionResource.resource),
            logicRef: actionResource.resource.logicRef,
            commitmentTreeRoot: isConsumed ? _initialRoot() : bytes32(0),
            appData: actionResource.appData
        });
    }

    function _actionTreeRoots(IProtocolAdapter.Action[] memory actions) private pure returns (bytes32[] memory roots) {
        roots = new bytes32[](actions.length);
        for (uint256 i = 0; i < actions.length; ++i) {
            roots[i] = actions[i].actionTreeRoot;
        }
    }

    function _countResources(IProtocolAdapter.Action[] memory actions) private pure returns (uint256 resourceCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            resourceCount += actions[i].resources.length;
        }
    }

    function _summedValueCommitmentRandomness(IProtocolAdapter.Action[] memory actions)
        private
        pure
        returns (uint256 summedRandomness)
    {
        for (uint256 i = 0; i < actions.length; ++i) {
            for (uint256 j = 0; j < actions[i].resources.length; ++j) {
                summedRandomness = addmod(
                    summedRandomness, _valueCommitmentRandomness(actions[i].resources[j].tag), DeltaGen.SECP256K1_ORDER
                );
            }
        }
    }

    /// @dev The tags of the consumed or the created resources of a transaction, in the order of the actions and of
    /// their resources.
    function _collectTags(IProtocolAdapter.Transaction memory txn, bool isConsumed)
        private
        pure
        returns (bytes32[] memory tags)
    {
        uint256 count = 0;
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            for (uint256 j = 0; j < txn.actions[i].resources.length; ++j) {
                if (txn.actions[i].resources[j].consumed == isConsumed) {
                    ++count;
                }
            }
        }

        tags = new bytes32[](count);

        uint256 n = 0;
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            for (uint256 j = 0; j < txn.actions[i].resources.length; ++j) {
                if (txn.actions[i].resources[j].consumed == isConsumed) {
                    tags[n++] = txn.actions[i].resources[j].tag;
                }
            }
        }
    }

    /// @dev The tags and logic references of the consumed or the created resources of an action, in the order of
    /// `resources`, as the protocol adapter emits them.
    function _actionTagsAndLogicRefs(IProtocolAdapter.Action memory action, bool isConsumed)
        private
        pure
        returns (bytes32[] memory tags, bytes32[] memory logicRefs)
    {
        uint256 count = 0;
        for (uint256 i = 0; i < action.resources.length; ++i) {
            if (action.resources[i].consumed == isConsumed) {
                ++count;
            }
        }

        tags = new bytes32[](count);
        logicRefs = new bytes32[](count);

        uint256 n = 0;
        for (uint256 i = 0; i < action.resources.length; ++i) {
            if (action.resources[i].consumed == isConsumed) {
                tags[n] = action.resources[i].tag;
                logicRefs[n] = action.resources[i].logicRef;
                ++n;
            }
        }
    }

    function _kind(Resource memory resource) private pure returns (uint256 hash) {
        hash = uint256(sha256(abi.encode(resource.logicRef, resource.labelRef)));
    }

    /// @dev Derives the randomness from the resource tag, so that each action gets a distinct delta and the delta
    /// proof can sum the randomness from the actions.
    function _valueCommitmentRandomness(bytes32 tag) private pure returns (uint256 randomness) {
        randomness = DeltaGen.modOrder(uint256(tag));
    }

    /// @dev The root of the kind tree without entries: its padding leaf, as in `KindTreeRoot.EMPTY_KIND_TREE_ROOT`.
    function _emptyKindTreeRoot() private pure returns (bytes32 root) {
        root = SHA256.EMPTY_HASH;
    }

    function _initialRoot() private pure returns (bytes32 root) {
        root = SHA256.EMPTY_HASH;
    }

    /// @dev The action tree leaves are the tags in the order of the resources.
    function _computeActionTreeRoot(IProtocolAdapter.ResourceInstance[] memory resources)
        private
        pure
        returns (bytes32 root)
    {
        bytes32[] memory tags = new bytes32[](resources.length);
        for (uint256 i = 0; i < resources.length; ++i) {
            tags[i] = resources[i].tag;
        }

        root = tags.computeRoot();
    }
}
