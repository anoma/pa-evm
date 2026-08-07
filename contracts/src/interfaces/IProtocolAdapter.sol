// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IProtocolAdapter
/// @author Anoma Foundation, 2025
/// @notice The interface of the protocol adapter contract verifying and executing resource machine transactions.
/// @custom:security-contact security@anoma.foundation
interface IProtocolAdapter {
    /// @notice An enum representing the supported blob deletion criteria.
    enum DeletionCriterion {
        Immediately,
        Never
    }

    /// @notice The transaction object containing all required data to conduct a RM state transition
    /// in which resources get consumed and created.
    /// @param actions The list of actions to be executed.
    /// @param deltaProof The proof for the transaction delta value.
    /// @param aggregationProof The recursive proof of all compliance and resource logics in the transaction — the only
    /// RISC Zero proof being verified.
    struct Transaction {
        Action[] actions;
        bytes deltaProof;
        bytes aggregationProof;
    }

    /// @notice The action object providing context separation between non-intersecting sets of resources. An action
    /// corresponds to one compliance unit constraining its consumed and created resources.
    /// @param consumed The public data of the consumed resources.
    /// @param created The public data of the created resources.
    /// @param unitDelta The action's delta value obtained from the underlying compliance unit.
    /// @param actionTreeRoot The root of the tree containing the tags of all resources present in the action.
    struct Action {
        Consumed[] consumed;
        Created[] created;
        Delta unitDelta;
        bytes32 actionTreeRoot;
    }

    /// @notice The public data of a consumed resource.
    /// @param nullifier The nullifier of the resource.
    /// @param logicRef The reference to (the verifying key of) the resource logic.
    /// @param commitmentTreeRoot The historical commitment tree root the resource's inclusion is proven against.
    /// @param appData The application data associated with the resource.
    struct Consumed {
        bytes32 nullifier;
        bytes32 logicRef;
        bytes32 commitmentTreeRoot;
        AppData appData;
    }

    /// @notice The public data of a created resource.
    /// @param commitment The commitment of the resource.
    /// @param logicRef The reference to (the verifying key of) the resource logic.
    /// @param appData The application data associated with the resource.
    struct Created {
        bytes32 commitment;
        bytes32 logicRef;
        AppData appData;
    }

    /// @notice The elliptic curve point representing the delta value of an action's compliance unit.
    /// @param x The x component of the point.
    /// @param y The y component of the point.
    struct Delta {
        uint256 x;
        uint256 y;
    }

    /// @notice A struct containing payloads of different kinds.
    /// @param resourcePayload A list of blobs for encoding plaintext info connected to resources.
    /// @param discoveryPayload A list of blobs for encoding data with public keys for discovery.
    /// @param externalPayload A list of blobs for encoding data connected with external calls.
    /// @param applicationPayload A list of blobs for application-specific purposes.
    struct AppData {
        ExpirableBlob[] resourcePayload;
        ExpirableBlob[] discoveryPayload;
        ExpirableBlob[] externalPayload;
        ExpirableBlob[] applicationPayload;
    }

    /// @notice A blob with a deletion criterion attached.
    /// @param deletionCriterion The deletion criterion.
    /// @param blob The bytes-encoded blob data.
    struct ExpirableBlob {
        DeletionCriterion deletionCriterion;
        bytes blob;
    }

    /// @notice Emitted when a transaction is executed.
    /// @param transactionId The Keccak-256 hash of the concatenated action tree roots — the message the delta proof
    /// signs, unique per transaction and known to the sender before submission.
    event TransactionExecuted(bytes32 indexed transactionId);

    /// @notice Emitted when an action is executed.
    /// @param actionTreeRoot The action tree root.
    /// @param nullifiers The nullifiers of the consumed resources.
    /// @param consumedLogicRefs The logic references of the consumed resources.
    /// @param commitments The commitments of the created resources.
    /// @param createdLogicRefs The logic references of the created resources.
    event ActionExecuted(
        bytes32 actionTreeRoot,
        bytes32[] nullifiers,
        bytes32[] consumedLogicRefs,
        bytes32[] commitments,
        bytes32[] createdLogicRefs
    );

    /// @notice Emitted when the kind table commitment is set.
    /// @param kindTableCommitment The commitment (SHA-256 hash) of the kind table transactions must be proven
    /// against.
    event KindTableCommitmentUpdated(bytes32 indexed kindTableCommitment);

    /// @notice Emitted when a forwarder call is executed.
    /// @param untrustedForwarder The forwarder contract forwarding the call.
    /// @param input The input data for the forwarded call.
    /// @param output The expected output data from the forwarded call.
    event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);

    /// @notice Emitted to store a resource payload blob persistently.
    /// @param tag The tag of the resource this blob belongs to.
    /// @param index The index of the blob in the payload array.
    /// @param blob The blob.
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Emitted to store a discovery payload blob persistently.
    /// @param tag The tag of the resource this blob belongs to.
    /// @param index The index of the blob in the payload array.
    /// @param blob The blob.
    event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Emitted to store a external payload blob persistently.
    /// @param tag The tag of the resource this blob belongs to.
    /// @param index The index of the blob in the payload array.
    /// @param blob The blob.
    event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Emitted to store an application payload blob persistently.
    /// @param tag The tag of the resource this blob belongs to.
    /// @param index The index of the blob in the payload array.
    /// @param blob The blob.
    event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Executes a transaction by adding the commitments and nullifiers to the commitment tree and nullifier
    /// set, respectively.
    /// @param transaction The transaction to execute.
    function execute(Transaction calldata transaction) external;

    /// @notice Simulates a transaction and returns the gas after reverting.
    /// @param transaction The transaction to simulate execution for.
    /// @param skipRiscZeroProofVerification Whether to skip RISC Zero proof verification or not.
    /// @dev This transaction will always revert.
    function simulateExecute(Transaction calldata transaction, bool skipRiscZeroProofVerification) external;

    /// @notice Stops the protocol adapter permanently in case of an emergency.
    function emergencyStop() external;

    /// @notice Sets the kind table commitment that transactions must be proven against.
    /// @param newKindTableCommitment The commitment (SHA-256 hash) of the new kind table.
    /// @dev The commitment changes whenever the set of supported resource kinds changes.
    function setKindTableCommitment(bytes32 newKindTableCommitment) external;

    /// @notice Returns whether the protocol adapter has been stopped or not. This can have two reasons:
    /// 1. The RISC Zero verifier associated with the protocol adapter has been stopped.
    /// 2. The protocol adapter itself was stopped by the owner.
    /// @return isStopped Whether the protocol adapter has been stopped or not.
    function isEmergencyStopped() external view returns (bool isStopped);

    /// @notice Returns the kind table commitment that transactions must be proven against.
    /// @return kindTableCommitment The commitment (SHA-256 hash) of the current kind table.
    function getKindTableCommitment() external view returns (bytes32 kindTableCommitment);

    /// @notice Returns the RISC Zero verifier router associated with the protocol adapter.
    /// @return verifierRouter The RISC Zero verifier router.
    function getRiscZeroVerifierRouter() external view returns (address verifierRouter);

    /// @notice Returns the RISC Zero verifier selector associated with the protocol adapter.
    /// @return verifierSelector The RISC Zero verifier selector.
    function getRiscZeroVerifierSelector() external view returns (bytes4 verifierSelector);

    /// @notice Returns the current implementation contract the calls are delegated to.
    /// @return current The current implementation contract.
    function implementation() external view returns (address current);
}
