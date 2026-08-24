// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Utils} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Utils.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/utils/PausableUpgradeable.sol";
import {IForwarder} from "anoma-forwarder-bases-2.0.0/src/interfaces/IForwarder.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {IProtocolAdapter} from "./interfaces/IProtocolAdapter.sol";
import {Aggregation} from "./libs/Aggregation.sol";
import {DeltaProof} from "./libs/DeltaProof.sol";
import {VerifyingKeys} from "./libs/VerifyingKeys.sol";
import {CommitmentTree} from "./state/CommitmentTree.sol";
import {NullifierSet} from "./state/NullifierSet.sol";

/// @title ProtocolAdapter
/// @author Anoma Foundation, 2025
/// @notice The protocol adapter contract verifying and executing resource machine transactions.
/// @custom:security-contact security@anoma.foundation
contract ProtocolAdapter is
    IProtocolAdapter,
    Initializable,
    UUPSUpgradeable,
    ReentrancyGuardTransient,
    OwnableUpgradeable,
    PausableUpgradeable,
    CommitmentTree,
    NullifierSet
{
    using Aggregation for Action[];
    using DeltaProof for bytes;
    using DeltaProof for Delta;

    /// @custom:storage-location erc7201:anoma.storage.ProtocolAdapter
    struct ProtocolAdapterStorage {
        bytes32 kindTableCommitment;
    }

    /// @notice The commitment of the empty kind table (the SHA-256 hash of zero bytes of table content), under
    /// which every resource kind is derived via hash-to-curve. Note that this is not `SHA256.EMPTY_HASH`.
    bytes32 internal constant _EMPTY_KIND_TABLE_COMMITMENT =
        0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855;

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.ProtocolAdapter")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _PROTOCOL_ADAPTER_STORAGE_SLOT =
        0x3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a700;

    /// @inheritdoc IProtocolAdapter
    string public constant override VERSION = "2.0.0-rc.0";

    /// @inheritdoc IProtocolAdapter
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable
    address public immutable override RISC_ZERO_VERIFIER_ROUTER;

    /// @inheritdoc IProtocolAdapter
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable
    bytes4 public immutable override RISC_ZERO_VERIFIER_SELECTOR;

    error ZeroRiscZeroVerifierRouterNotAllowed();
    error ZeroRiscZeroVerifierSelectorNotAllowed();
    error ZeroKindTableCommitmentNotAllowed();
    error EmptyTransactionNotAllowed();
    error ForwarderCallOutputMismatch(bytes expected, bytes actual);
    error RiscZeroVerifierSelectorMismatch(bytes4 expected, bytes4 actual);
    error RiscZeroVerifierStopped();
    error Simulated(uint256 gasUsed);

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
    /// @param initialOwner The account receiving ownership, and with it the authority to stop the protocol adapter in
    /// case of a vulnerability, to authorize upgrades, and to set the kind table commitment.
    function initialize( /* solhint-disable-line comprehensive-interface*/
        address initialOwner
    )
        external
        initializer
    {
        __Ownable_init(initialOwner);
        __Pausable_init();

        __CommitmentTree_init();
        __NullifierSet_init();

        // Start with the empty kind table, under which every resource kind is derived via hash-to-curve.
        _getProtocolAdapterStorage().kindTableCommitment = _EMPTY_KIND_TABLE_COMMITMENT;
        emit KindTableCommitmentUpdated({kindTableCommitment: _EMPTY_KIND_TABLE_COMMITMENT});

        // Sanity check that the verifier has not been stopped already.
        require(!isEmergencyStopped(), RiscZeroVerifierStopped());
    }

    /// @inheritdoc IProtocolAdapter
    function execute(Transaction calldata transaction) external override {
        _execute({transaction: transaction, skipRiscZeroProofVerification: false});
    }

    /// @inheritdoc IProtocolAdapter
    function simulateExecute(Transaction calldata transaction, bool skipRiscZeroProofVerification) external override {
        uint256 gasStart = gasleft();

        _execute({transaction: transaction, skipRiscZeroProofVerification: skipRiscZeroProofVerification});

        revert Simulated({gasUsed: gasStart - gasleft()});
    }

    /// @inheritdoc IProtocolAdapter
    function emergencyStop() external override onlyOwner whenNotPaused {
        _pause();
    }

    /// @inheritdoc IProtocolAdapter
    function setKindTableCommitment(bytes32 newKindTableCommitment) external override onlyOwner {
        require(newKindTableCommitment != bytes32(0), ZeroKindTableCommitmentNotAllowed());

        _getProtocolAdapterStorage().kindTableCommitment = newKindTableCommitment;
        emit KindTableCommitmentUpdated({kindTableCommitment: newKindTableCommitment});
    }

    /// @inheritdoc IProtocolAdapter
    function getKindTableCommitment() external view override returns (bytes32 kindTableCommitment) {
        kindTableCommitment = _getProtocolAdapterStorage().kindTableCommitment;
    }

    /// @inheritdoc IProtocolAdapter
    function getImplementation() external view override returns (address current) {
        current = ERC1967Utils.getImplementation();
    }

    /// @inheritdoc IProtocolAdapter
    function isEmergencyStopped() public view override returns (bool isStopped) {
        bool risc0Paused = PausableUpgradeable(
                address(RiscZeroVerifierRouter(RISC_ZERO_VERIFIER_ROUTER).getVerifier(RISC_ZERO_VERIFIER_SELECTOR))
            ).paused();

        isStopped = risc0Paused || paused();
    }

    /// @notice Executes a transaction by adding the commitments and nullifiers to the commitment tree and nullifier
    /// set, respectively.
    /// @param transaction The transaction to execute.
    /// @param skipRiscZeroProofVerification Whether to skip RISC Zero proof verification or not.
    /// @dev This function cannot be called anymore once `emergencyStop()` has been called.
    // NOTE: The state writes and reads after the forwarder calls are protected by the `nonReentrant` modifier.
    // slither-disable-next-line reentrancy-no-eth,reentrancy-benign
    function _execute(Transaction calldata transaction, bool skipRiscZeroProofVerification)
        internal
        nonReentrant
        whenNotPaused
    {
        uint256 actionCount = transaction.actions.length;

        // Reject the empty transaction so that the delta and aggregation proofs are verified unconditionally.
        require(actionCount != 0, EmptyTransactionNotAllowed());

        bytes32[] memory actionTreeRoots = new bytes32[](actionCount);
        Delta memory transactionDelta = DeltaProof.zero();
        bytes32 updatedCommitmentTreeRoot = bytes32(0);

        for (uint256 i = 0; i < actionCount; ++i) {
            Action calldata action = transaction.actions[i];

            bytes32 newRoot = _processAction(action);
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
            transaction: transaction,
            actionTreeRoots: actionTreeRoots,
            transactionDelta: transactionDelta,
            skipRiscZeroProofVerification: skipRiscZeroProofVerification
        });

        // Store the final commitment tree root. A transaction creating no resources leaves the tree untouched,
        // so there is no new root to store.
        if (updatedCommitmentTreeRoot != bytes32(0)) {
            _addCommitmentTreeRoot(updatedCommitmentTreeRoot);
        }

        // NOTE: The event ordering is protected by the `nonReentrant` modifier.
        // slither-disable-next-line reentrancy-events
        emit TransactionExecuted({transactionId: transactionId});
    }

    /// @notice Processes an action by
    /// * checking that the commitment tree roots referenced by the consumed resources are historical roots,
    /// * adding the nullifiers to the nullifier set and the commitments to the commitment tree,
    /// * executing external forwarder calls,
    /// * emitting the app data blobs based on their deletion criterion, and
    /// * emitting the `ActionExecuted` event.
    /// @param action The action to process.
    /// @return updatedCommitmentTreeRoot The commitment tree root after the last added commitment, or zero if the
    /// action created no resources.
    // NOTE: The nullifier and commitment writes around the forwarder calls are protected by the `nonReentrant`
    // modifier on the calling `_execute`.
    // slither-disable-next-line reentrancy-no-eth
    function _processAction(Action calldata action) internal returns (bytes32 updatedCommitmentTreeRoot) {
        uint256 consumedCount = action.consumed.length;
        bytes32[] memory nullifiers = new bytes32[](consumedCount);
        bytes32[] memory consumedLogicRefs = new bytes32[](consumedCount);

        for (uint256 i = 0; i < consumedCount; ++i) {
            Consumed calldata consumed = action.consumed[i];

            // Check that the referenced commitment tree root is part of the historical roots.
            require(
                _isCommitmentTreeRootContained(consumed.commitmentTreeRoot),
                NonExistingRoot(consumed.commitmentTreeRoot)
            );

            // The function reverts if a repeating nullifier is added to the set.
            _addNullifier(consumed.nullifier);

            _executeForwarderCalls({carrierLogicRef: consumed.logicRef, appData: consumed.appData});
            _emitAppDataBlobs({tag: consumed.nullifier, appData: consumed.appData});

            nullifiers[i] = consumed.nullifier;
            consumedLogicRefs[i] = consumed.logicRef;
        }

        uint256 createdCount = action.created.length;
        bytes32[] memory commitments = new bytes32[](createdCount);
        bytes32[] memory createdLogicRefs = new bytes32[](createdCount);

        for (uint256 i = 0; i < createdCount; ++i) {
            Created calldata created = action.created[i];

            // `_addCommitment` does not error if a repeating leaf is added to the tree.
            // Uniqueness of commitments is granted by the compliance circuit, assuming that nullifiers are unique.
            updatedCommitmentTreeRoot = _addCommitment(created.commitment);

            _executeForwarderCalls({carrierLogicRef: created.logicRef, appData: created.appData});
            _emitAppDataBlobs({tag: created.commitment, appData: created.appData});

            commitments[i] = created.commitment;
            createdLogicRefs[i] = created.logicRef;
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
        uint256 nCalls = appData.externalPayload.length;

        for (uint256 i = 0; i < nCalls; ++i) {
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

        // slither-disable-next-line calls-loop
        bytes memory actualOutput =
            IForwarder(untrustedForwarder).forwardCall({logicRef: carrierLogicRef, input: input});

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

    /// @inheritdoc UUPSUpgradeable
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    /// @notice Verifies the global proofs:
    /// * the delta proof ensuring that the transaction is balanced,
    /// * the aggregation proof attesting to all compliance units and resource logics.
    /// @param transaction The transaction to verify the proofs for.
    /// @param actionTreeRoots The action tree roots as ordered in the transaction.
    /// @param transactionDelta The transaction delta obtained from summing the action deltas.
    /// @param skipRiscZeroProofVerification Whether to skip RISC Zero proof verification or not.
    /// @return transactionId The delta verifying key doubling as the transaction ID.
    function _verifyGlobalProofs(
        Transaction calldata transaction,
        bytes32[] memory actionTreeRoots,
        Delta memory transactionDelta,
        bool skipRiscZeroProofVerification
    ) internal view returns (bytes32 transactionId) {
        // The delta proof signs the Keccak-256 hash of the concatenated action tree roots.
        transactionId = DeltaProof.computeVerifyingKey(actionTreeRoots);

        // Check the delta proof.
        transaction.deltaProof.verify({instance: transactionDelta, verifyingKey: transactionId});

        // Reconstruct the aggregation journal, injecting the compliance circuit verifying key and the stored kind
        // table commitment — a transaction proven against any other values is unencodable and fails verification.
        bytes32 journalDigest = sha256(
            transaction.actions
                .toJournal({
                    complianceKey: VerifyingKeys._COMPLIANCE,
                    kindTableCommitment: _getProtocolAdapterStorage().kindTableCommitment
                })
        );

        // Process the aggregation proof.
        _processRiscZeroProof({
            verifyingKey: VerifyingKeys._BATCH_AGGREGATION_EVM,
            instance: journalDigest,
            proof: transaction.aggregationProof,
            skipVerification: skipRiscZeroProofVerification
        });
    }

    /// @notice Processes a RISC Zero proof by checking its selector and verifying it. Optionally, the verification call
    /// can be skipped, which is used in the `simulateExecute` entry point.
    /// @param verifyingKey The image ID of the program to be proven.
    /// @param instance The public inputs to the proof, i.e. the digest of the journal.
    /// @param proof The proof to be verified.
    /// @param skipVerification Whether to skip proof verification or not.
    function _processRiscZeroProof(bytes32 verifyingKey, bytes32 instance, bytes calldata proof, bool skipVerification)
        internal
        view
    {
        _checkSelector(bytes4(proof[0:4]));

        if (!skipVerification) {
            // slither-disable-next-line calls-loop
            RiscZeroVerifierRouter(RISC_ZERO_VERIFIER_ROUTER)
                .verify({seal: proof, imageId: verifyingKey, journalDigest: instance});
        }
    }

    /// @notice Checks that a RISC Zero verifier selector matches the one the protocol adapter is associated with.
    /// @param selector The RISC Zero verifier selector to check.
    function _checkSelector(bytes4 selector) internal view {
        require(
            selector == RISC_ZERO_VERIFIER_SELECTOR,
            RiscZeroVerifierSelectorMismatch({expected: RISC_ZERO_VERIFIER_SELECTOR, actual: selector})
        );
    }

    /// @notice Returns the storage from the protocol adapter storage location.
    /// @return protocolAdapterStorage The data associated with the protocol adapter storage.
    function _getProtocolAdapterStorage()
        internal
        pure
        returns (ProtocolAdapterStorage storage protocolAdapterStorage)
    {
        /* solhint-disable no-inline-assembly */
        // slither-disable-next-line assembly
        assembly {
            protocolAdapterStorage.slot := _PROTOCOL_ADAPTER_STORAGE_SLOT
        }
        /* solhint-enable no-inline-assembly */
    }
}
