// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {VmSafe} from "forge-std-1.16.1/src/Vm.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {MerkleTree} from "../../src/libs/MerkleTree.sol";
import {Delta} from "../../src/libs/proving/Delta.sol";
import {Logic} from "../../src/libs/proving/Logic.sol";
import {VerifyingKeys} from "../../src/libs/proving/VerifyingKeys.sol";
import {RiscZeroUtils} from "../../src/libs/RiscZeroUtils.sol";
import {SHA256} from "../../src/libs/SHA256.sol";
import {
    Transaction,
    Action,
    ConsumedResourcePublicData,
    CreatedResourcePublicData,
    Resource
} from "./../../src/Types.sol";
import {DeltaGen} from "./DeltaGen.sol";

library TxGen {
    using MerkleTree for bytes32[];
    using RiscZeroUtils for Transaction;

    struct ActionConfig {
        uint256 consumedCount;
        uint256 createdCount;
    }

    struct ResourceAndAppData {
        Resource resource;
        Logic.AppData appData;
    }

    struct ResourceLists {
        ResourceAndAppData[] consumed;
        ResourceAndAppData[] created;
    }

    /// @dev Builds an action from resource lists. The action delta is the sum of the per-resource deltas, each
    /// generated with a value commitment randomness of 1 — the caller is responsible for quantity balance across
    /// the transaction.
    function createAction(VmSafe vm, ResourceAndAppData[] memory consumed, ResourceAndAppData[] memory created)
        internal
        returns (Action memory action)
    {
        uint256 consumedCount = consumed.length;
        uint256 createdCount = created.length;

        ConsumedResourcePublicData[] memory consumedData = new ConsumedResourcePublicData[](consumedCount);
        CreatedResourcePublicData[] memory createdData = new CreatedResourcePublicData[](createdCount);

        Delta.Point memory actionDelta;

        for (uint256 i = 0; i < consumedCount; ++i) {
            consumedData[i] = ConsumedResourcePublicData({
                nullifier: nullifier(consumed[i].resource, 0),
                logicRef: consumed[i].resource.logicRef,
                commitmentTreeRoot: initialRoot(),
                appData: consumed[i].appData
            });

            Delta.Point memory resourceDelta = DeltaGen.generateInstance(
                vm,
                DeltaGen.InstanceInputs({
                    kind: kind(consumed[i].resource),
                    quantity: consumed[i].resource.quantity,
                    consumed: true,
                    valueCommitmentRandomness: 1
                })
            );
            actionDelta = (i == 0) ? resourceDelta : Delta.add(actionDelta, resourceDelta);
        }

        for (uint256 i = 0; i < createdCount; ++i) {
            createdData[i] = CreatedResourcePublicData({
                commitment: commitment(created[i].resource),
                logicRef: created[i].resource.logicRef,
                appData: created[i].appData
            });

            Delta.Point memory resourceDelta = DeltaGen.generateInstance(
                vm,
                DeltaGen.InstanceInputs({
                    kind: kind(created[i].resource),
                    quantity: created[i].resource.quantity,
                    consumed: false,
                    valueCommitmentRandomness: 1
                })
            );
            actionDelta = (consumedCount == 0 && i == 0) ? resourceDelta : Delta.add(actionDelta, resourceDelta);
        }

        action = Action({
            consumed: consumedData,
            created: createdData,
            delta: actionDelta,
            actionTreeRoot: computeActionTreeRoot(consumedData, createdData)
        });
    }

    /// @dev Builds an action of `consumedCount` consumed and `createdCount` created mock resources sharing one
    /// kind. Quantities are chosen so the action balances: each consumed resource has quantity `createdCount`,
    /// each created resource quantity `consumedCount`.
    function createDefaultAction(VmSafe vm, bytes32 nonce, uint256 consumedCount, uint256 createdCount)
        internal
        returns (Action memory action, bytes32 updatedNonce)
    {
        updatedNonce = nonce;

        ResourceAndAppData[] memory consumed = new ResourceAndAppData[](consumedCount);
        ResourceAndAppData[] memory created = new ResourceAndAppData[](createdCount);

        bytes32 kindRef = nonce;

        for (uint256 i = 0; i < consumedCount; ++i) {
            consumed[i] = ResourceAndAppData({
                resource: mockResource({
                    nonce: updatedNonce,
                    logicRef: kindRef,
                    labelRef: kindRef,
                    // Counts are small test inputs, so the cast cannot truncate.
                    // forge-lint: disable-next-line(unsafe-typecast)
                    quantity: uint128(createdCount)
                }),
                appData: emptyAppData()
            });
            updatedNonce = bytes32(uint256(updatedNonce) + 1);
        }

        for (uint256 i = 0; i < createdCount; ++i) {
            created[i] = ResourceAndAppData({
                resource: mockResource({
                    nonce: updatedNonce,
                    logicRef: kindRef,
                    labelRef: kindRef,
                    // Counts are small test inputs, so the cast cannot truncate.
                    // forge-lint: disable-next-line(unsafe-typecast)
                    quantity: uint128(consumedCount)
                }),
                appData: emptyAppData()
            });
            updatedNonce = bytes32(uint256(updatedNonce) + 1);
        }

        action = createAction({vm: vm, consumed: consumed, created: created});
    }

    function transaction(VmSafe vm, RiscZeroMockVerifier mockVerifier, ResourceLists[] memory actionResources)
        internal
        returns (Transaction memory txn)
    {
        Action[] memory actions = new Action[](actionResources.length);

        uint256 resourceCount = 0;
        for (uint256 i = 0; i < actionResources.length; ++i) {
            actions[i] =
                createAction({vm: vm, consumed: actionResources[i].consumed, created: actionResources[i].created});
            resourceCount += actionResources[i].consumed.length + actionResources[i].created.length;
        }

        txn = transactionFromActions({vm: vm, actions: actions, resourceCount: resourceCount});
        txn = transactionAggregation({
            mockVerifier: mockVerifier, txn: txn, kindTableCommitment: emptyKindTableCommitment()
        });
    }

    function transaction(VmSafe vm, RiscZeroMockVerifier mockVerifier, bytes32 nonce, ActionConfig[] memory configs)
        internal
        returns (Transaction memory txn, bytes32 updatedNonce)
    {
        updatedNonce = nonce;

        Action[] memory actions = new Action[](configs.length);
        uint256 resourceCount = 0;
        for (uint256 i = 0; i < configs.length; ++i) {
            (actions[i], updatedNonce) = createDefaultAction({
                vm: vm,
                nonce: updatedNonce,
                consumedCount: configs[i].consumedCount,
                createdCount: configs[i].createdCount
            });
            resourceCount += configs[i].consumedCount + configs[i].createdCount;
        }

        txn = transactionFromActions({vm: vm, actions: actions, resourceCount: resourceCount});
        txn = transactionAggregation({
            mockVerifier: mockVerifier, txn: txn, kindTableCommitment: emptyKindTableCommitment()
        });
    }

    /// @dev Assembles the transaction and its delta proof. Each resource contributed a value commitment randomness
    /// of 1, so the summed randomness is the resource count.
    function transactionFromActions(VmSafe vm, Action[] memory actions, uint256 resourceCount)
        internal
        returns (Transaction memory txn)
    {
        bytes memory proof = "";
        if (resourceCount != 0) {
            proof = DeltaGen.generateProof(
                vm,
                DeltaGen.ProofInputs({
                    summedValueCommitmentRandomness: resourceCount,
                    verifyingKey: Delta.computeVerifyingKey(actionTreeRoots(actions))
                })
            );
        }

        txn = Transaction({actions: actions, deltaProof: proof, aggregationProof: ""});
    }

    /// @dev Mock-proves the aggregation: the seal commits to the journal reconstructed with the same compliance key
    /// and kind table commitment the protocol adapter injects.
    function transactionAggregation(
        RiscZeroMockVerifier mockVerifier,
        Transaction memory txn,
        bytes32 kindTableCommitment
    ) internal view returns (Transaction memory aggregatedTxn) {
        aggregatedTxn = txn;

        aggregatedTxn.aggregationProof =
        mockVerifier.mockProve({
            imageId: VerifyingKeys._BATCH_AGGREGATION,
            journalDigest: sha256(
                txn.toJournal({complianceKey: VerifyingKeys._COMPLIANCE, kindTableCommitment: kindTableCommitment})
            )
        }).seal;
    }

    function generateActionConfigs(uint256 actionCount, uint256 consumedCount, uint256 createdCount)
        internal
        pure
        returns (ActionConfig[] memory configs)
    {
        configs = new TxGen.ActionConfig[](actionCount);
        for (uint256 i = 0; i < actionCount; ++i) {
            configs[i] = TxGen.ActionConfig({consumedCount: consumedCount, createdCount: createdCount});
        }
    }

    /// @dev The commitment of the empty kind table, matching the protocol adapter's initialization default.
    function emptyKindTableCommitment() internal pure returns (bytes32 commitmentOfEmptyTable) {
        commitmentOfEmptyTable = sha256("");
    }

    function transactionId(Transaction memory txn) internal pure returns (bytes32 id) {
        id = Delta.computeVerifyingKey(actionTreeRoots(txn.actions));
    }

    function actionTreeRoots(Action[] memory actions) internal pure returns (bytes32[] memory roots) {
        roots = new bytes32[](actions.length);
        for (uint256 i = 0; i < actions.length; ++i) {
            roots[i] = actions[i].actionTreeRoot;
        }
    }

    function countResources(Action[] memory actions) internal pure returns (uint256 resourceCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            resourceCount += actions[i].consumed.length + actions[i].created.length;
        }
    }

    function collectNullifiers(Transaction memory txn) internal pure returns (bytes32[] memory nullifiers) {
        nullifiers = new bytes32[](countConsumed(txn.actions));

        uint256 n = 0;
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            ConsumedResourcePublicData[] memory consumed = txn.actions[i].consumed;
            for (uint256 j = 0; j < consumed.length; ++j) {
                nullifiers[n++] = consumed[j].nullifier;
            }
        }
    }

    function collectCommitments(Transaction memory txn) internal pure returns (bytes32[] memory commitments) {
        commitments = new bytes32[](countCreated(txn.actions));

        uint256 n = 0;
        for (uint256 i = 0; i < txn.actions.length; ++i) {
            CreatedResourcePublicData[] memory created = txn.actions[i].created;
            for (uint256 j = 0; j < created.length; ++j) {
                commitments[n++] = created[j].commitment;
            }
        }
    }

    function countConsumed(Action[] memory actions) internal pure returns (uint256 consumedCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            consumedCount += actions[i].consumed.length;
        }
    }

    function countCreated(Action[] memory actions) internal pure returns (uint256 createdCount) {
        for (uint256 i = 0; i < actions.length; ++i) {
            createdCount += actions[i].created.length;
        }
    }

    function actionNullifiers(Action memory action) internal pure returns (bytes32[] memory nullifiers) {
        nullifiers = new bytes32[](action.consumed.length);
        for (uint256 i = 0; i < action.consumed.length; ++i) {
            nullifiers[i] = action.consumed[i].nullifier;
        }
    }

    function actionConsumedLogicRefs(Action memory action) internal pure returns (bytes32[] memory logicRefs) {
        logicRefs = new bytes32[](action.consumed.length);
        for (uint256 i = 0; i < action.consumed.length; ++i) {
            logicRefs[i] = action.consumed[i].logicRef;
        }
    }

    function actionCommitments(Action memory action) internal pure returns (bytes32[] memory commitments) {
        commitments = new bytes32[](action.created.length);
        for (uint256 i = 0; i < action.created.length; ++i) {
            commitments[i] = action.created[i].commitment;
        }
    }

    function actionCreatedLogicRefs(Action memory action) internal pure returns (bytes32[] memory logicRefs) {
        logicRefs = new bytes32[](action.created.length);
        for (uint256 i = 0; i < action.created.length; ++i) {
            logicRefs[i] = action.created[i].logicRef;
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

    function emptyAppData() internal pure returns (Logic.AppData memory appData) {
        appData = Logic.AppData({
            resourcePayload: new Logic.ExpirableBlob[](0),
            discoveryPayload: new Logic.ExpirableBlob[](0),
            externalPayload: new Logic.ExpirableBlob[](0),
            applicationPayload: new Logic.ExpirableBlob[](0)
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

    function expirableBlobs() internal pure returns (Logic.ExpirableBlob[] memory blobs) {
        blobs = new Logic.ExpirableBlob[](2);
        blobs[0] = Logic.ExpirableBlob({
            blob: hex"1f0000003f0000005f0000007f000000", deletionCriterion: Logic.DeletionCriterion.Immediately
        });
        blobs[1] = Logic.ExpirableBlob({
            blob: hex"9f000000bf000000df000000ff000000", deletionCriterion: Logic.DeletionCriterion.Never
        });
    }

    function initialRoot() internal pure returns (bytes32 root) {
        root = SHA256.EMPTY_HASH;
    }

    /// @dev The action tree leaves are the consumed nullifiers followed by the created commitments — the canonical
    /// tag order.
    function computeActionTreeRoot(
        ConsumedResourcePublicData[] memory consumed,
        CreatedResourcePublicData[] memory created
    ) internal pure returns (bytes32 root) {
        bytes32[] memory tags = new bytes32[](consumed.length + created.length);

        uint256 n = 0;
        for (uint256 i = 0; i < consumed.length; ++i) {
            tags[n++] = consumed[i].nullifier;
        }
        for (uint256 i = 0; i < created.length; ++i) {
            tags[n++] = created[i].commitment;
        }

        root = tags.computeRoot();
    }
}
