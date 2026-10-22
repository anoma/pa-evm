// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Utils} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Utils.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/utils/PausableUpgradeable.sol";
import {IForwarder} from "anoma-forwarder-bases-3.1.0/src/interfaces/IForwarder.sol";
import {IImplementation} from "anoma-forwarder-bases-3.1.0/src/interfaces/IImplementation.sol";
import {IVersion} from "anoma-forwarder-bases-3.1.0/src/interfaces/IVersion.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {IProtocolAdapter} from "./interfaces/IProtocolAdapter.sol";
import {Aggregation} from "./libs/Aggregation.sol";
import {DeltaProof} from "./libs/DeltaProof.sol";
import {ForwarderCallOrdering} from "./libs/ForwarderCallOrdering.sol";
import {VerifyingKeys} from "./libs/VerifyingKeys.sol";
import {CommitmentTree} from "./state/CommitmentTree.sol";
import {KindTreeRoot} from "./state/KindTreeRoot.sol";
import {LogicRefDenylist} from "./state/LogicRefDenylist.sol";
import {NullifierSet} from "./state/NullifierSet.sol";

/// @title ProtocolAdapter
/// @author Anoma Foundation, 2025
/// @notice The protocol adapter contract verifying and executing resource machine transactions.
/// @custom:security-contact security@anoma.foundation
contract ProtocolAdapter is
    IProtocolAdapter,
    IVersion,
    IImplementation,
    Initializable,
    UUPSUpgradeable,
    ReentrancyGuardTransient,
    OwnableUpgradeable,
    PausableUpgradeable,
    CommitmentTree,
    NullifierSet,
    LogicRefDenylist,
    KindTreeRoot
{
    using Aggregation for Action[];
    using DeltaProof for bytes;
    using DeltaProof for Delta;
    using ForwarderCallOrdering for Action[];
    using ForwarderCallOrdering for ExpirableBlob[];
    using ForwarderCallOrdering for ForwarderCallOrdering.CallList[];

    /// @inheritdoc IVersion
    string public constant override VERSION = "2.0.0-rc.11";

    /// @inheritdoc IProtocolAdapter
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable
    address public immutable override RISC_ZERO_VERIFIER_ROUTER;

    /// @inheritdoc IProtocolAdapter
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable
    bytes4 public immutable override RISC_ZERO_VERIFIER_SELECTOR;

    error ZeroRiscZeroVerifierRouterNotAllowed();
    error ZeroRiscZeroVerifierSelectorNotAllowed();
    error EmptyTransactionNotAllowed();
    error ForwarderCallOutputMismatch(bytes expected, bytes actual);
    error RiscZeroVerifierSelectorMismatch(bytes4 expected, bytes4 actual);
    error RiscZeroVerifierPaused();

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @param riscZeroVerifierRouter The RISC Zero verifier router contract address.
    /// @param riscZeroVerifierSelector The RISC Zero verifier selector this protocol adapter is associated with.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector) {
        require(riscZeroVerifierRouter != address(0), ZeroRiscZeroVerifierRouterNotAllowed());
        require(riscZeroVerifierSelector != bytes4(0), ZeroRiscZeroVerifierSelectorNotAllowed());

        RISC_ZERO_VERIFIER_ROUTER = riscZeroVerifierRouter;
        RISC_ZERO_VERIFIER_SELECTOR = riscZeroVerifierSelector;

        _disableInitializers();
    }

    /// @notice Initializes the protocol adapter contract.
    /// @param initialOwner The account receiving ownership, and with it the authority to pause the protocol adapter,
    /// to authorize upgrades, to set the kind tree root, and to deny logic references.
    function initialize( /* solhint-disable-line comprehensive-interface*/
        address initialOwner
    )
        external
        virtual
        initializer
    {
        __ProtocolAdapter_init(initialOwner);
    }

    /// @inheritdoc IProtocolAdapter
    function execute(Transaction calldata transaction) external override nonReentrant whenNotPaused {
        uint256 actionCount = transaction.actions.length;

        // Reject the empty transaction so that the delta and aggregation proofs are verified unconditionally.
        require(actionCount != 0, EmptyTransactionNotAllowed());

        require(_isKindTreeRootAccepted(transaction.kindTreeRoot), UnacceptedKindTreeRoot(transaction.kindTreeRoot));

        bytes32[] memory actionTreeRoots = new bytes32[](actionCount);
        Delta memory transactionDelta = DeltaProof.zero();
        bytes32 updatedCommitmentTreeRoot = bytes32(0);
        uint256 callListCount = 0;
        bool hasOrderingConstraints = false;

        for (uint256 i = 0; i < actionCount; ++i) {
            Action calldata action = transaction.actions[i];

            (bytes32 newRoot, uint256 actionCallListCount, bool actionHasOrderingConstraints) = _processAction(action);
            if (newRoot != bytes32(0)) {
                updatedCommitmentTreeRoot = newRoot;
            }
            if (actionCallListCount != 0) {
                callListCount += actionCallListCount;
                if (actionHasOrderingConstraints) {
                    hasOrderingConstraints = true;
                }
            }

            transactionDelta = transactionDelta.add(action.unitDelta);

            actionTreeRoots[i] = action.actionTreeRoot;
        }

        bytes32 transactionId = _verifyGlobalProofs({
            transaction: transaction, actionTreeRoots: actionTreeRoots, transactionDelta: transactionDelta
        });

        // Store the final commitment tree root. A transaction creating no resources leaves the tree untouched,
        // so there is no new root to store.
        if (updatedCommitmentTreeRoot != bytes32(0)) {
            _addCommitmentTreeRoot(updatedCommitmentTreeRoot);
        }

        // Execute the forwarder calls after all state changes of the transaction.
        if (callListCount != 0) {
            _executeForwarderCalls({
                actions: transaction.actions,
                callListCount: callListCount,
                hasOrderingConstraints: hasOrderingConstraints
            });
        }

        // NOTE: The event ordering is protected by the `nonReentrant` modifier.
        // slither-disable-next-line reentrancy-events
        emit TransactionExecuted({transactionId: transactionId});
    }

    /// @inheritdoc IProtocolAdapter
    function pause() external override onlyOwner {
        _pause();
    }

    /// @inheritdoc IProtocolAdapter
    function unpause() external override onlyOwner {
        _unpause();
    }

    /// @inheritdoc IImplementation
    function getImplementation() external view override returns (address current) {
        current = ERC1967Utils.getImplementation();
    }

    /// @inheritdoc IProtocolAdapter
    function paused() public view override(IProtocolAdapter, PausableUpgradeable) returns (bool isPaused) {
        isPaused = super.paused();
    }

    /// @inheritdoc IProtocolAdapter
    function riscZeroVerifierPaused() public view override returns (bool isPaused) {
        isPaused = PausableUpgradeable(
                address(RiscZeroVerifierRouter(RISC_ZERO_VERIFIER_ROUTER).getVerifier(RISC_ZERO_VERIFIER_SELECTOR))
            )
            .paused();
    }

    /// @notice Processes an action by
    /// * checking the logic references of the consumed and created resources against the denylists,
    /// * checking that the commitment tree roots referenced by the consumed resources are historical roots,
    /// * adding the nullifiers to the nullifier set and the commitments to the commitment tree,
    /// * counting the resources that carry forwarder calls and checking the calls for ordering constraints,
    /// * emitting the app data blobs based on their deletion criterion, and
    /// * emitting the `ActionExecuted` event.
    /// @param action The action to process.
    /// @return updatedCommitmentTreeRoot The commitment tree root after the last added commitment, or zero if the
    /// action created no resources.
    /// @return callListCount The number of resources of the action that carry forwarder calls.
    /// @return hasOrderingConstraints Whether a forwarder call of the action carries ordering constraints.
    function _processAction(Action calldata action)
        internal
        returns (bytes32 updatedCommitmentTreeRoot, uint256 callListCount, bool hasOrderingConstraints)
    {
        uint256 consumedCount = action.consumed.length;
        bytes32[] memory nullifiers = new bytes32[](consumedCount);
        bytes32[] memory consumedLogicRefs = new bytes32[](consumedCount);

        // NOTE: Reverting inside the loop is intended: one invalid action aborts the whole transaction.
        for (uint256 i = 0; i < consumedCount; ++i) {
            Consumed calldata consumed = action.consumed[i];

            _checkLogicRefNotDenied({logicRef: consumed.logicRef, consumed: true});

            require(
                _isCommitmentTreeRootHistorical(consumed.commitmentTreeRoot),
                NonExistingRoot(consumed.commitmentTreeRoot)
            );

            // The function reverts if a repeating nullifier is added to the set.
            _addNullifier(consumed.nullifier);

            ExpirableBlob[] calldata externalPayload = consumed.appData.externalPayload;
            if (externalPayload.length != 0) {
                ++callListCount;
                if (!hasOrderingConstraints) {
                    hasOrderingConstraints = externalPayload.hasOrderingConstraints();
                }
            }
            _emitAppDataBlobs({tag: consumed.nullifier, appData: consumed.appData});

            nullifiers[i] = consumed.nullifier;
            consumedLogicRefs[i] = consumed.logicRef;
        }

        uint256 createdCount = action.created.length;
        bytes32[] memory commitments = new bytes32[](createdCount);
        bytes32[] memory createdLogicRefs = new bytes32[](createdCount);

        for (uint256 i = 0; i < createdCount; ++i) {
            Created calldata created = action.created[i];

            _checkLogicRefNotDenied({logicRef: created.logicRef, consumed: false});

            // `_addCommitment` does not error if a repeating leaf is added to the tree.
            // Uniqueness of commitments is granted by the compliance circuit, assuming that nullifiers are unique.
            updatedCommitmentTreeRoot = _addCommitment(created.commitment);

            ExpirableBlob[] calldata externalPayload = created.appData.externalPayload;
            if (externalPayload.length != 0) {
                ++callListCount;
                if (!hasOrderingConstraints) {
                    hasOrderingConstraints = externalPayload.hasOrderingConstraints();
                }
            }
            _emitAppDataBlobs({tag: created.commitment, appData: created.appData});

            commitments[i] = created.commitment;
            createdLogicRefs[i] = created.logicRef;
        }

        emit ActionExecuted({
            actionTreeRoot: action.actionTreeRoot,
            nullifiers: nullifiers,
            consumedLogicRefs: consumedLogicRefs,
            commitments: commitments,
            createdLogicRefs: createdLogicRefs
        });
    }

    /// @notice Executes the forwarder calls of a transaction.
    /// @dev Executes the calls of one resource together and in their order: after the calls of each resource that their
    /// `afterTags` name, and before the calls of each resource that their `beforeTags` name. Also executes the calls of
    /// one action together. Of the resources whose calls can come next, takes the first in the transaction, with the
    /// consumed resources of an action before its created resources. With ordering constraints, it reverts if a tag in
    /// `afterTags` or `beforeTags` names no call-carrying resource of the transaction, if no order meets the
    /// constraints, or if more than 256 resources carry calls.
    /// @param actions The actions of the transaction.
    /// @param callListCount The number of resources that carry forwarder calls.
    /// @param hasOrderingConstraints Whether a forwarder call carries ordering constraints.
    function _executeForwarderCalls(Action[] calldata actions, uint256 callListCount, bool hasOrderingConstraints)
        internal
    {
        if (hasOrderingConstraints) {
            _executeForwarderCallsInConstraintOrder({actions: actions, callListCount: callListCount});
        } else {
            _executeForwarderCallsInActionOrder(actions);
        }
    }

    /// @notice Executes the forwarder calls of a transaction in the order of the actions, with the consumed resources
    /// of an action before its created resources.
    /// @param actions The actions of the transaction.
    function _executeForwarderCallsInActionOrder(Action[] calldata actions) internal {
        uint256 actionCount = actions.length;
        for (uint256 i = 0; i < actionCount; ++i) {
            Consumed[] calldata consumedResources = actions[i].consumed;
            uint256 consumedCount = consumedResources.length;
            for (uint256 j = 0; j < consumedCount; ++j) {
                Consumed calldata consumed = consumedResources[j];
                _executeForwarderCallsOfResource({
                    logicRef: consumed.logicRef, externalPayload: consumed.appData.externalPayload
                });
            }

            Created[] calldata createdResources = actions[i].created;
            uint256 createdCount = createdResources.length;
            for (uint256 j = 0; j < createdCount; ++j) {
                Created calldata created = createdResources[j];
                _executeForwarderCallsOfResource({
                    logicRef: created.logicRef, externalPayload: created.appData.externalPayload
                });
            }
        }
    }

    /// @notice Decodes and executes the forwarder calls of a resource, which carry no ordering constraints.
    /// @param logicRef The logic reference of the call-carrying resource.
    /// @param externalPayload The external payload of the resource.
    function _executeForwarderCallsOfResource(bytes32 logicRef, ExpirableBlob[] calldata externalPayload) internal {
        uint256 callCount = externalPayload.length;
        for (uint256 i = 0; i < callCount; ++i) {
            // `ForwarderCallOrdering.hasOrderingConstraints` checked that the tag lists after these fields are empty.
            (address untrustedForwarder, bytes memory input, bytes memory expectedOutput) =
                abi.decode(externalPayload[i].blob, (address, bytes, bytes));
            _executeForwarderCall({
                logicRef: logicRef, untrustedForwarder: untrustedForwarder, input: input, expectedOutput: expectedOutput
            });
        }
    }

    /// @notice Executes the forwarder calls of a transaction in the order that their ordering constraints set.
    /// @param actions The actions of the transaction.
    /// @param callListCount The number of resources that carry forwarder calls.
    function _executeForwarderCallsInConstraintOrder(Action[] calldata actions, uint256 callListCount) internal {
        ForwarderCallOrdering.CallList[] memory callLists = actions.decodeCallLists(callListCount);
        uint256[] memory orderedIndices = callLists.order();

        for (uint256 position = 0; position < callListCount; ++position) {
            ForwarderCallOrdering.CallList memory callList = callLists[orderedIndices[position]];
            uint256 callCount = callList.calls.length;
            for (uint256 i = 0; i < callCount; ++i) {
                ForwarderCall memory forwarderCall = callList.calls[i];
                _executeForwarderCall({
                    logicRef: callList.resourceLogicRef,
                    untrustedForwarder: forwarderCall.untrustedForwarder,
                    input: forwarderCall.input,
                    expectedOutput: forwarderCall.expectedOutput
                });
            }
        }
    }

    /// @notice Executes a call to an external, untrusted forwarder contract.
    /// @param logicRef The logic reference of the call-carrying resource.
    /// @param untrustedForwarder The forwarder contract to call.
    /// @param input The input of the call.
    /// @param expectedOutput The output that the call must return.
    /// @dev This function allows arbitrary code execution through the protocol adapter but is constrained through
    /// the logic of the call-carrying resource.
    function _executeForwarderCall(
        bytes32 logicRef,
        address untrustedForwarder,
        bytes memory input,
        bytes memory expectedOutput
    ) internal {
        // NOTE: Each resource carries its own forwarder calls, so the call belongs inside the loop over the calls.
        // forge-lint: disable-next-item(calls-loop)
        bytes memory actualOutput = IForwarder(untrustedForwarder).forwardCall({logicRef: logicRef, input: input});

        // NOTE: Reverting inside the loop is intended: one invalid call aborts the whole transaction.
        require(
            keccak256(actualOutput) == keccak256(expectedOutput),
            ForwarderCallOutputMismatch({expected: expectedOutput, actual: actualOutput})
        );

        // NOTE: The event ordering is protected by the `nonReentrant` modifier on the `execute` function.
        // slither-disable-next-line reentrancy-events
        emit ForwarderCallExecuted({untrustedForwarder: untrustedForwarder, input: input, output: actualOutput});
    }

    /// @notice Emits app data blobs together with the associated resource tag based on their deletion criterion.
    /// @param tag The tag of the resource the app data belongs to.
    /// @param appData The application data to emit the blobs from.
    function _emitAppDataBlobs(bytes32 tag, AppData calldata appData) internal {
        ExpirableBlob[] calldata payload = appData.resourcePayload;
        uint256 n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit ResourcePayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = appData.discoveryPayload;
        n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit DiscoveryPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = appData.externalPayload;
        n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit ExternalPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = appData.applicationPayload;
        n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit ApplicationPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }
    }

    /// @notice Initializes the protocol adapter state: ownership, the pause, the commitment tree, the nullifier set,
    /// the empty logic reference denylists and the empty kind table.
    /// @param initialOwner The account receiving ownership.
    // forge-lint: disable-next-line(mixed-case-function)
    function __ProtocolAdapter_init(address initialOwner) internal onlyInitializing {
        __Ownable_init(initialOwner);
        __Pausable_init();
        __CommitmentTree_init();
        __NullifierSet_init();
        __LogicRefDenylist_init();
        __KindTreeRoot_init();

        // Sanity check that the verifier is not paused already.
        require(!riscZeroVerifierPaused(), RiscZeroVerifierPaused());
    }

    /// @inheritdoc UUPSUpgradeable
    // slither-disable-next-line dead-code
    function _authorizeUpgrade(address newImplementation) internal virtual override onlyOwner {}

    /// @notice Allows only the owner to set the kind tree root.
    function _authorizeKindTreeRootChange() internal override onlyOwner {}

    /// @notice Allows only the owner to add logic references to the denylists.
    function _authorizeLogicRefDenylistChange() internal override onlyOwner {}

    /// @notice Verifies the global proofs:
    /// * the delta proof ensuring that the transaction is balanced,
    /// * the aggregation proof attesting to all compliance units and resource logics.
    /// @param transaction The transaction to verify the proofs for.
    /// @param actionTreeRoots The action tree roots as ordered in the transaction.
    /// @param transactionDelta The transaction delta obtained from summing the action deltas.
    /// @return transactionId The delta verifying key doubling as the transaction ID.
    function _verifyGlobalProofs(
        Transaction calldata transaction,
        bytes32[] memory actionTreeRoots,
        Delta memory transactionDelta
    ) internal view returns (bytes32 transactionId) {
        // The delta proof signs the Keccak-256 hash of the concatenated action tree roots.
        transactionId = DeltaProof.computeVerifyingKey(actionTreeRoots);

        // Check the delta proof.
        transaction.deltaProof.verify({instance: transactionDelta, verifyingKey: transactionId});

        // Reconstruct the aggregation journal from the injected compliance circuit verifying key and the kind table
        // root the transaction carries.
        bytes32 journalDigest = sha256(
            transaction.actions
                .toJournal({complianceKey: VerifyingKeys._COMPLIANCE, kindTreeRoot: transaction.kindTreeRoot})
        );

        // Process the aggregation proof.
        _processRiscZeroProof({
            verifyingKey: VerifyingKeys._BATCH_AGGREGATION_EVM,
            instance: journalDigest,
            proof: transaction.aggregationProof
        });
    }

    /// @notice Processes a RISC Zero proof by checking its selector and verifying it.
    /// @param verifyingKey The image ID of the program to be proven.
    /// @param instance The public inputs to the proof, i.e. the digest of the journal.
    /// @param proof The proof to be verified.
    function _processRiscZeroProof(bytes32 verifyingKey, bytes32 instance, bytes calldata proof) internal view {
        _checkSelector(bytes4(proof[0:4]));

        RiscZeroVerifierRouter(RISC_ZERO_VERIFIER_ROUTER).verify({
            seal: proof, imageId: verifyingKey, journalDigest: instance
        });
    }

    /// @notice Checks that a RISC Zero verifier selector matches the one the protocol adapter is associated with.
    /// @param selector The RISC Zero verifier selector to check.
    function _checkSelector(bytes4 selector) internal view {
        require(
            selector == RISC_ZERO_VERIFIER_SELECTOR,
            RiscZeroVerifierSelectorMismatch({expected: RISC_ZERO_VERIFIER_SELECTOR, actual: selector})
        );
    }
}
