// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Utils} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Utils.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/utils/PausableUpgradeable.sol";
import {IForwarder} from "anoma-forwarder-bases-3.0.1/src/interfaces/IForwarder.sol";
import {IImplementation} from "anoma-forwarder-bases-3.0.1/src/interfaces/IImplementation.sol";
import {IVersion} from "anoma-forwarder-bases-3.0.1/src/interfaces/IVersion.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {IProtocolAdapter} from "./interfaces/IProtocolAdapter.sol";
import {Aggregation} from "./libs/Aggregation.sol";
import {DeltaProof} from "./libs/DeltaProof.sol";
import {ForwarderCallOrder} from "./libs/ForwarderCallOrder.sol";
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

    /// @notice A forwarder call from the external payload of a resource.
    /// @param carrierLogicRef The logic reference of the resource carrying the call.
    /// @param untrustedForwarder The forwarder contract to call.
    /// @param input The input of the call.
    /// @param expectedOutput The output that the call must return.
    struct ForwarderCall {
        bytes32 carrierLogicRef;
        address untrustedForwarder;
        bytes input;
        bytes expectedOutput;
    }

    /// @notice The tags and the forwarder calls of the resources of a transaction, in the order of the actions, with
    /// the consumed resources of an action before its created resources.
    /// @param tags The tags of all resources.
    /// @param tagCount The number of tags collected so far.
    /// @param constraints The constraints of the forwarder calls.
    /// @param calls The forwarder calls, at the indices of their constraints.
    /// @param callCount The number of forwarder calls collected so far.
    struct CollectedCalls {
        bytes32[] tags;
        uint256 tagCount;
        ForwarderCallOrder.Constraints[] constraints;
        ForwarderCall[] calls;
        uint256 callCount;
    }

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

    // NOTE: The state writes and reads after the forwarder calls are protected by the `nonReentrant` modifier.
    // slither-disable-start reentrancy-no-eth,reentrancy-benign
    /// @inheritdoc IProtocolAdapter
    function execute(Transaction calldata transaction) external override nonReentrant whenNotPaused {
        uint256 actionCount = transaction.actions.length;

        // Reject the empty transaction so that the delta and aggregation proofs are verified unconditionally.
        require(actionCount != 0, EmptyTransactionNotAllowed());

        require(_isKindTreeRootAccepted(transaction.kindTreeRoot), UnacceptedKindTreeRoot(transaction.kindTreeRoot));

        bytes32[] memory actionTreeRoots = new bytes32[](actionCount);
        Delta memory transactionDelta = DeltaProof.zero();
        bytes32 updatedCommitmentTreeRoot = bytes32(0);
        CollectedCalls memory collected = _allocateCollectedCalls(transaction.actions);

        for (uint256 i = 0; i < actionCount; ++i) {
            Action calldata action = transaction.actions[i];

            bytes32 newRoot = _processAction({action: action, collected: collected});
            if (newRoot != bytes32(0)) {
                updatedCommitmentTreeRoot = newRoot;
            }

            // Add the action delta to the transaction delta.
            transactionDelta = transactionDelta.add(action.unitDelta);

            actionTreeRoots[i] = action.actionTreeRoot;
        }

        // Verify the delta and aggregation proofs. The empty transaction is rejected above,
        // so both proofs are guaranteed to be checked.
        bytes32 transactionId = _verifyGlobalProofs({
            transaction: transaction, actionTreeRoots: actionTreeRoots, transactionDelta: transactionDelta
        });

        // Run the forwarder calls after the state changes of all actions and after the proofs.
        _executeForwarderCalls(collected);

        // Store the final commitment tree root. A transaction creating no resources leaves the tree untouched,
        // so there is no new root to store.
        if (updatedCommitmentTreeRoot != bytes32(0)) {
            _addCommitmentTreeRoot(updatedCommitmentTreeRoot);
        }

        // NOTE: The event ordering is protected by the `nonReentrant` modifier.
        // slither-disable-next-line reentrancy-events
        emit TransactionExecuted({transactionId: transactionId});
    }

    // slither-disable-end reentrancy-no-eth,reentrancy-benign

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
    /// * collecting the tags and the forwarder calls of the resources,
    /// * emitting the app data blobs based on their deletion criterion, and
    /// * emitting the `ActionExecuted` event.
    /// @param action The action to process.
    /// @param collected The tags and forwarder calls collected so far, to which the action adds its own.
    /// @return updatedCommitmentTreeRoot The commitment tree root after the last added commitment, or zero if the
    /// action created no resources.
    function _processAction(Action calldata action, CollectedCalls memory collected)
        internal
        returns (bytes32 updatedCommitmentTreeRoot)
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

            _collectResource({
                tag: consumed.nullifier,
                carrierLogicRef: consumed.logicRef,
                appData: consumed.appData,
                collected: collected
            });
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

            _collectResource({
                tag: created.commitment,
                carrierLogicRef: created.logicRef,
                appData: created.appData,
                collected: collected
            });
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

    /// @notice Executes the collected forwarder calls in the order that their constraints set.
    /// @param collected The tags and forwarder calls of the transaction.
    function _executeForwarderCalls(CollectedCalls memory collected) internal {
        uint256[] memory sequence = ForwarderCallOrder.order({tags: collected.tags, calls: collected.constraints});
        uint256 callCount = sequence.length;

        for (uint256 i = 0; i < callCount; ++i) {
            _executeForwarderCall(collected.calls[sequence[i]]);
        }
    }

    /// @notice Executes a call to a an external, untrusted forwarder contract.
    /// @param forwarderCall The call to execute.
    /// @dev This function allows arbitrary code execution through the protocol adapter but is constrained through
    /// the associated carrier resource logic.
    function _executeForwarderCall(ForwarderCall memory forwarderCall) internal {
        // NOTE: Each resource carries its own forwarder calls, so the call belongs inside the loop over the calls.
        // forge-lint: disable-next-item(calls-loop)
        bytes memory actualOutput = IForwarder(forwarderCall.untrustedForwarder).forwardCall({
            logicRef: forwarderCall.carrierLogicRef, input: forwarderCall.input
        });

        // NOTE: Reverting inside the loop is intended: one invalid call aborts the whole transaction.
        require(
            keccak256(actualOutput) == keccak256(forwarderCall.expectedOutput),
            ForwarderCallOutputMismatch({expected: forwarderCall.expectedOutput, actual: actualOutput})
        );

        // NOTE: The event ordering is protected by the `nonReentrant` modifier on the `execute` function.
        // slither-disable-next-line reentrancy-events
        emit ForwarderCallExecuted({
            untrustedForwarder: forwarderCall.untrustedForwarder, input: forwarderCall.input, output: actualOutput
        });
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

    /// @notice Allocates the arrays for the tags and the forwarder calls of a transaction.
    /// @param actions The actions of the transaction.
    /// @return collected The empty collection.
    function _allocateCollectedCalls(Action[] calldata actions)
        internal
        pure
        returns (CollectedCalls memory collected)
    {
        uint256 resourceCount = 0;
        uint256 callCount = 0;
        uint256 actionCount = actions.length;

        for (uint256 i = 0; i < actionCount; ++i) {
            Consumed[] calldata consumed = actions[i].consumed;
            Created[] calldata created = actions[i].created;
            resourceCount += consumed.length + created.length;

            for (uint256 j = 0; j < consumed.length; ++j) {
                callCount += consumed[j].appData.externalPayload.length;
            }
            for (uint256 j = 0; j < created.length; ++j) {
                callCount += created[j].appData.externalPayload.length;
            }
        }

        collected.tags = new bytes32[](resourceCount);
        collected.constraints = new ForwarderCallOrder.Constraints[](callCount);
        collected.calls = new ForwarderCall[](callCount);
    }

    /// @notice Collects the tag of a resource and the forwarder calls in its external payload.
    /// @param tag The tag of the resource.
    /// @param carrierLogicRef The logic reference of the resource.
    /// @param appData The application data of the resource.
    /// @param collected The collection to add the tag and the calls to.
    function _collectResource(
        bytes32 tag,
        bytes32 carrierLogicRef,
        AppData calldata appData,
        CollectedCalls memory collected
    ) internal pure {
        collected.tags[collected.tagCount++] = tag;

        uint256 blobCount = appData.externalPayload.length;
        for (uint256 i = 0; i < blobCount; ++i) {
            (
                address untrustedForwarder,
                bytes memory input,
                bytes memory expectedOutput,
                bytes32[] memory afterTags,
                bytes32[] memory beforeTags
            ) = abi.decode(appData.externalPayload[i].blob, (address, bytes, bytes, bytes32[], bytes32[]));

            // The arrays hold zeroed structs already, so the fields are written in place.
            uint256 index = collected.callCount++;
            ForwarderCallOrder.Constraints memory constraints = collected.constraints[index];
            constraints.tag = tag;
            constraints.afterTags = afterTags;
            constraints.beforeTags = beforeTags;

            ForwarderCall memory forwarderCall = collected.calls[index];
            forwarderCall.carrierLogicRef = carrierLogicRef;
            forwarderCall.untrustedForwarder = untrustedForwarder;
            forwarderCall.input = input;
            forwarderCall.expectedOutput = expectedOutput;
        }
    }
}
