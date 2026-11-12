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

    /// @notice A resource of an action. `createAction` derives its `IProtocolAdapter.ResourceInstance` from it.
    struct ActionResource {
        bool consumed;
        Resource resource;
        IProtocolAdapter.AppData appData;
    }

    /// @dev Builds an action of the resources in list order. The caller balances the quantities across the transaction.
    function createAction(VmSafe vm, ActionResource[] memory resources)
        internal
        returns (IProtocolAdapter.Action memory action)
    {
        IProtocolAdapter.ResourceInstance[] memory instances = new IProtocolAdapter.ResourceInstance[](resources.length);

        IProtocolAdapter.Delta memory actionDelta;

        for (uint256 i = 0; i < resources.length; ++i) {
            instances[i] = resourceInstance(resources[i]);

            IProtocolAdapter.Delta memory resourceDelta = DeltaGen.generateInstance(
                vm,
                DeltaGen.InstanceInputs({
                    kind: kind(resources[i].resource),
                    quantity: resources[i].resource.quantity,
                    consumed: resources[i].consumed,
                    valueCommitmentRandomness: valueCommitmentRandomness(instances[i].tag)
                })
            );
            actionDelta = (i == 0) ? resourceDelta : DeltaProof.add(actionDelta, resourceDelta);
        }

        action = IProtocolAdapter.Action({
            resources: instances, unitDelta: actionDelta, actionTreeRoot: computeActionTreeRoot(instances)
        });
    }

    function transaction(VmSafe vm, RiscZeroMockVerifier mockVerifier, ActionResource[][] memory actionResources)
        internal
        returns (IProtocolAdapter.Transaction memory txn)
    {
        IProtocolAdapter.Action[] memory actions = new IProtocolAdapter.Action[](actionResources.length);

        for (uint256 i = 0; i < actionResources.length; ++i) {
            actions[i] = createAction({vm: vm, resources: actionResources[i]});
        }

        txn = transactionFromActions({vm: vm, actions: actions});
        txn = transactionAggregation({mockVerifier: mockVerifier, txn: txn, kindTreeRoot: emptyKindTreeRoot()});
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
            actions[i] = createAction({vm: vm, resources: resources});
        }

        txn = transactionFromActions({vm: vm, actions: actions});
        txn = transactionAggregation({mockVerifier: mockVerifier, txn: txn, kindTreeRoot: emptyKindTreeRoot()});
    }

    /// @dev Assembles the transaction and its delta proof.
    function transactionFromActions(VmSafe vm, IProtocolAdapter.Action[] memory actions)
        internal
        returns (IProtocolAdapter.Transaction memory txn)
    {
        bytes memory proof = "";
        if (countResources(actions) != 0) {
            proof = DeltaGen.generateProof(
                vm,
                DeltaGen.ProofInputs({
                    summedValueCommitmentRandomness: summedValueCommitmentRandomness(actions),
                    verifyingKey: DeltaProof.computeVerifyingKey(actionTreeRoots(actions))
                })
            );
        }

        txn = IProtocolAdapter.Transaction({
            actions: actions, deltaProof: proof, aggregationProof: "", kindTreeRoot: emptyKindTreeRoot()
        });
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

    function resourceInstance(ActionResource memory actionResource)
        internal
        pure
        returns (IProtocolAdapter.ResourceInstance memory instance)
    {
        bool consumed = actionResource.consumed;
        instance = IProtocolAdapter.ResourceInstance({
            consumed: consumed,
            tag: consumed ? nullifier(actionResource.resource, 0) : commitment(actionResource.resource),
            logicRef: actionResource.resource.logicRef,
            commitmentTreeRoot: consumed ? initialRoot() : bytes32(0),
            appData: actionResource.appData
        });
    }

    /// @dev `consumedCount` consumed, then `createdCount` created resources of one kind. A consumed resource has
    /// quantity `createdCount` and a created one `consumedCount`, so the action balances.
    function mockResources(bytes32 nonce, uint256 consumedCount, uint256 createdCount)
        internal
        pure
        returns (ActionResource[] memory resources, bytes32 updatedNonce)
    {
        resources = new ActionResource[](consumedCount + createdCount);

        for (uint256 i = 0; i < resources.length; ++i) {
            bool consumed = i < consumedCount;
            resources[i] = ActionResource({
                consumed: consumed,
                resource: mockResource({
                    nonce: bytes32(uint256(nonce) + i),
                    logicRef: nonce,
                    labelRef: nonce,
                    // Counts are small test inputs, so the cast cannot truncate.
                    // forge-lint: disable-next-line(unsafe-typecast)
                    quantity: uint128(consumed ? createdCount : consumedCount)
                }),
                appData: emptyAppData()
            });
        }

        updatedNonce = bytes32(uint256(nonce) + resources.length);
    }

    /// @dev The root of the empty kind tree: its padding leaf, matching the protocol adapter's initialization default.
    function emptyKindTreeRoot() internal pure returns (bytes32 rootOfEmptyTable) {
        rootOfEmptyTable = sha256("EMPTY");
    }

    function transactionId(IProtocolAdapter.Transaction memory txn) internal pure returns (bytes32 id) {
        id = DeltaProof.computeVerifyingKey(actionTreeRoots(txn.actions));
    }

    function actionTreeRoots(IProtocolAdapter.Action[] memory actions) internal pure returns (bytes32[] memory roots) {
        roots = new bytes32[](actions.length);
        for (uint256 i = 0; i < actions.length; ++i) {
            roots[i] = actions[i].actionTreeRoot;
        }
    }

    function countResources(IProtocolAdapter.Action[] memory actions) internal pure returns (uint256 resourceCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            resourceCount += actions[i].resources.length;
        }
    }

    function summedValueCommitmentRandomness(IProtocolAdapter.Action[] memory actions)
        internal
        pure
        returns (uint256 summedRandomness)
    {
        for (uint256 i = 0; i < actions.length; ++i) {
            for (uint256 j = 0; j < actions[i].resources.length; ++j) {
                summedRandomness = addmod(
                    summedRandomness, valueCommitmentRandomness(actions[i].resources[j].tag), DeltaGen.SECP256K1_ORDER
                );
            }
        }
    }

    function collectNullifiers(IProtocolAdapter.Transaction memory txn)
        internal
        pure
        returns (bytes32[] memory nullifiers)
    {
        nullifiers = new bytes32[](countConsumed(txn.actions));

        uint256 n = 0;
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            bytes32[] memory actionTags = actionNullifiers(txn.actions[i]);
            for (uint256 j = 0; j < actionTags.length; ++j) {
                nullifiers[n++] = actionTags[j];
            }
        }
    }

    function collectCommitments(IProtocolAdapter.Transaction memory txn)
        internal
        pure
        returns (bytes32[] memory commitments)
    {
        commitments = new bytes32[](countCreated(txn.actions));

        uint256 n = 0;
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            bytes32[] memory actionTags = actionCommitments(txn.actions[i]);
            for (uint256 j = 0; j < actionTags.length; ++j) {
                commitments[n++] = actionTags[j];
            }
        }
    }

    function countConsumed(IProtocolAdapter.Action[] memory actions) internal pure returns (uint256 consumedCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            consumedCount += actionNullifiers(actions[i]).length;
        }
    }

    function countCreated(IProtocolAdapter.Action[] memory actions) internal pure returns (uint256 createdCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            createdCount += actionCommitments(actions[i]).length;
        }
    }

    function actionNullifiers(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory nullifiers)
    {
        (nullifiers,) = actionTagsAndLogicRefs({action: action, consumed: true});
    }

    function actionConsumedLogicRefs(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory logicRefs)
    {
        (, logicRefs) = actionTagsAndLogicRefs({action: action, consumed: true});
    }

    function actionCommitments(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory commitments)
    {
        (commitments,) = actionTagsAndLogicRefs({action: action, consumed: false});
    }

    function actionCreatedLogicRefs(IProtocolAdapter.Action memory action)
        internal
        pure
        returns (bytes32[] memory logicRefs)
    {
        (, logicRefs) = actionTagsAndLogicRefs({action: action, consumed: false});
    }

    /// @dev The tags and logic references of the consumed or the created resources of an action, in the order of
    /// `resources`, as the protocol adapter emits them.
    function actionTagsAndLogicRefs(IProtocolAdapter.Action memory action, bool consumed)
        internal
        pure
        returns (bytes32[] memory tags, bytes32[] memory logicRefs)
    {
        uint256 count = 0;
        for (uint256 i = 0; i < action.resources.length; ++i) {
            if (action.resources[i].consumed == consumed) {
                ++count;
            }
        }

        tags = new bytes32[](count);
        logicRefs = new bytes32[](count);

        uint256 n = 0;
        for (uint256 i = 0; i < action.resources.length; ++i) {
            if (action.resources[i].consumed == consumed) {
                tags[n] = action.resources[i].tag;
                logicRefs[n] = action.resources[i].logicRef;
                ++n;
            }
        }
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

    function emptyAppData() internal pure returns (IProtocolAdapter.AppData memory appData) {
        appData = IProtocolAdapter.AppData({
            resourcePayload: new IProtocolAdapter.ExpirableBlob[](0),
            discoveryPayload: new IProtocolAdapter.ExpirableBlob[](0),
            externalPayload: new IProtocolAdapter.ExpirableBlob[](0),
            applicationPayload: new IProtocolAdapter.ExpirableBlob[](0)
        });
    }

    function commitment(Resource memory resource) internal pure returns (bytes32 hash) {
        hash = sha256(abi.encode(resource));
    }

    function nullifier(Resource memory resource, bytes32 nullifierKey) internal pure returns (bytes32 hash) {
        hash = sha256(abi.encode(resource, nullifierKey));
    }

    function kind(Resource memory resource) internal pure returns (uint256 hash) {
        hash = uint256(sha256(abi.encode(resource.logicRef, resource.labelRef)));
    }

    /// @dev Derives the randomness from the resource tag, so that each action gets a distinct delta and the delta
    /// proof can sum the randomness from the actions.
    function valueCommitmentRandomness(bytes32 tag) internal pure returns (uint256 randomness) {
        randomness = DeltaGen.modOrder(uint256(tag));
    }

    function expirableBlobs() internal pure returns (IProtocolAdapter.ExpirableBlob[] memory blobs) {
        blobs = new IProtocolAdapter.ExpirableBlob[](2);
        blobs[0] = IProtocolAdapter.ExpirableBlob({
            blob: hex"1f0000003f0000005f0000007f000000",
            deletionCriterion: IProtocolAdapter.DeletionCriterion.Immediately
        });
        blobs[1] = IProtocolAdapter.ExpirableBlob({
            blob: hex"9f000000bf000000df000000ff000000", deletionCriterion: IProtocolAdapter.DeletionCriterion.Never
        });
    }

    function initialRoot() internal pure returns (bytes32 root) {
        root = SHA256.EMPTY_HASH;
    }

    /// @dev The action tree leaves are the tags in the order of the resources.
    function computeActionTreeRoot(IProtocolAdapter.ResourceInstance[] memory resources)
        internal
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
