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
    using Aggregation for Transaction;
    using DeltaProof for bytes;
    using DeltaProof for Delta;

    /// @inheritdoc IVersion
    string public constant override VERSION = "2.0.0-rc.13";

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
        bool addedCommitments = false;

        for (uint256 i = 0; i < actionCount; ++i) {
            Action calldata action = transaction.actions[i];

            bool actionAddedCommitments = _processAction(action);
            if (actionAddedCommitments) {
                addedCommitments = true;
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

        // Store the final commitment tree root. A transaction creating no resources leaves the tree untouched,
        // so there is no new root to store.
        if (addedCommitments) {
            _addCommitmentTreeRoot(latestCommitmentTreeRoot());
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
    /// * executing external forwarder calls,
    /// * emitting the app data blobs based on their deletion criterion, and
    /// * emitting the `ActionExecuted` event.
    /// It processes the resources in the order of `action.resources`.
    /// @param action The action to process.
    /// @return addedCommitments Whether the action added commitments to the commitment tree.
    // NOTE: The nullifier and commitment writes around the forwarder calls are protected by the `nonReentrant`
    // modifier on the calling `execute` function.
    // slither-disable-next-line reentrancy-no-eth
    function _processAction(Action calldata action) internal returns (bool addedCommitments) {
        ResourceInstance[] calldata resources = action.resources;
        uint256 resourceCount = resources.length;

        uint256 consumedCount = 0;
        for (uint256 i = 0; i < resourceCount; ++i) {
            if (resources[i].consumed) {
                ++consumedCount;
            }
        }

        bytes32[] memory nullifiers = new bytes32[](consumedCount);
        bytes32[] memory consumedLogicRefs = new bytes32[](consumedCount);
        bytes32[] memory commitments = new bytes32[](resourceCount - consumedCount);
        bytes32[] memory createdLogicRefs = new bytes32[](resourceCount - consumedCount);

        uint256 consumedIndex = 0;
        uint256 createdIndex = 0;

        // NOTE: Reverting inside the loop is intended: one invalid action aborts the whole transaction.
        for (uint256 i = 0; i < resourceCount; ++i) {
            ResourceInstance calldata resource = resources[i];
            bool consumed = resource.consumed;

            _checkLogicRefNotDenied({logicRef: resource.logicRef, consumed: consumed});

            if (consumed) {
                require(
                    _isCommitmentTreeRootHistorical(resource.commitmentTreeRoot),
                    NonExistingRoot(resource.commitmentTreeRoot)
                );

                // The function reverts if a repeating nullifier is added to the set.
                _addNullifier(resource.tag);

                nullifiers[consumedIndex] = resource.tag;
                consumedLogicRefs[consumedIndex] = resource.logicRef;
                ++consumedIndex;
            } else {
                // `_addCommitment` does not error if a repeating leaf is added to the tree.
                // Uniqueness of commitments is granted by the compliance circuit, assuming that nullifiers are unique.
                _addCommitment(resource.tag);
                addedCommitments = true;

                commitments[createdIndex] = resource.tag;
                createdLogicRefs[createdIndex] = resource.logicRef;
                ++createdIndex;
            }

            _executeForwarderCalls({carrierLogicRef: resource.logicRef, appData: resource.appData});
            _emitAppDataBlobs({tag: resource.tag, appData: resource.appData});
        }

        // NOTE: The event ordering is protected by the `nonReentrant` modifier.
        // slither-disable-next-line reentrancy-events
        emit ActionExecuted({
            actionTreeRoot: action.actionTreeRoot,
            nullifiers: nullifiers,
            consumedLogicRefs: consumedLogicRefs,
            commitments: commitments,
            createdLogicRefs: createdLogicRefs
        });
    }

    /// @notice Processes forwarder calls by verifying and executing them.
    /// @param carrierLogicRef The logic reference of the carrier resource making the calls.
    /// @param appData The application data of the carrier resource containing the external payload.
    function _executeForwarderCalls(bytes32 carrierLogicRef, AppData calldata appData) internal {
        uint256 callCount = appData.externalPayload.length;

        for (uint256 i = 0; i < callCount; ++i) {
            _executeForwarderCall({carrierLogicRef: carrierLogicRef, callBlob: appData.externalPayload[i].blob});
        }
    }

    /// @notice Executes a call to a an external, untrusted forwarder contract.
    /// @param carrierLogicRef The logic reference of the carrier resource.
    /// @param callBlob The blob containing the external call instruction.
    /// @dev This function allows arbitrary code execution through the protocol adapter but is constrained through
    /// the associated carrier resource logic.
    function _executeForwarderCall(bytes32 carrierLogicRef, bytes calldata callBlob) internal {
        (address untrustedForwarder, bytes memory input, bytes memory expectedOutput) =
            abi.decode(callBlob, (address, bytes, bytes));

        // NOTE: Each action carries its own forwarder call, so the call belongs inside the loop over the actions.
        // forge-lint: disable-next-item(calls-loop)
        bytes memory actualOutput =
            IForwarder(untrustedForwarder).forwardCall({logicRef: carrierLogicRef, input: input});

        // NOTE: Reverting inside the loop is intended: one invalid action aborts the whole transaction.
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
                // NOTE: The event ordering is protected by the `nonReentrant` modifier on the `execute` function.
                // slither-disable-next-line reentrancy-events
                emit ResourcePayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = appData.discoveryPayload;
        n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                // NOTE: The event ordering is protected by the `nonReentrant` modifier on the `execute` function.
                // slither-disable-next-line reentrancy-events
                emit DiscoveryPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = appData.externalPayload;
        n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                // NOTE: The event ordering is protected by the `nonReentrant` modifier on the `execute` function.
                // slither-disable-next-line reentrancy-events
                emit ExternalPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = appData.applicationPayload;
        n = payload.length;
        for (uint256 i = 0; i < n; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                // NOTE: The event ordering is protected by the `nonReentrant` modifier on the `execute` function.
                // slither-disable-next-line reentrancy-events
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

        // Reconstruct the aggregation journal from the injected compliance circuit verifying key and the kind tree root
        // the transaction carries.
        bytes32 journalDigest = transaction.computeJournalDigest({complianceKey: VerifyingKeys._COMPLIANCE});

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
