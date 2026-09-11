// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SafeCast} from "@openzeppelin-contracts-5.7.0/utils/math/SafeCast.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";

import {ICommitmentTree} from "./interfaces/ICommitmentTree.sol";
import {INullifierSet} from "./interfaces/INullifierSet.sol";
import {IProtocolAdapterTransition} from "./interfaces/IProtocolAdapterTransition.sol";
import {MerkleTree} from "./libs/MerkleTree.sol";
import {SeededTree} from "./libs/SeededTree.sol";
import {SHA256} from "./libs/SHA256.sol";
import {ProtocolAdapter} from "./ProtocolAdapter.sol";

/// @title ProtocolAdapterTransition
/// @author Anoma Foundation, 2026
/// @notice The protocol adapter for a chain that moves from v1 to v2. It starts paused and copies the v1 state in —
/// the commitment tree and the nullifier set — reading every value from the v1 protocol adapter itself. It refuses to
/// unpause until its commitment tree and its nullifier set hold what v1 holds. The migration script copies the state,
/// unpauses and upgrades the proxy to `ProtocolAdapter` in one run, which is what removes these functions again. A
/// chain deployed fresh uses `ProtocolAdapter` from the start.
/// @dev The contract holds no storage of its own, so upgrading away from it leaves no namespace behind.
/// @custom:security-contact security@anoma.foundation
contract ProtocolAdapterTransition is IProtocolAdapterTransition, ProtocolAdapter {
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using MerkleTree for MerkleTree.Tree;
    using SeededTree for MerkleTree.Tree;
    using SafeCast for uint256;

    /// @notice The v1 protocol adapter this contract copies its state from.
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable
    address internal immutable _PROTOCOL_ADAPTER_V1;

    error ZeroProtocolAdapterV1NotAllowed();
    error ProtocolAdapterV1NotStopped(address protocolAdapterV1);
    error CommitmentTreeNotEmpty();
    error EmptyCommitmentTreeNotAllowed();
    error LeafCountExceedsCapacity(uint256 leafCount, uint256 capacity);
    error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);
    error CommitmentCountMismatch(uint256 expected, uint256 actual);
    error HistoricalRootCountMismatch(uint256 expected, uint256 actual);
    error HistoricalRootMismatch(uint256 index, bytes32 expected, bytes32 actual);
    error NullifierBatchOutOfRange(uint256 available, uint256 requested);
    error NullifierIndexMismatch(uint256 index, bytes32 expected, bytes32 actual);
    error NullifierCountMismatch(uint256 expected, uint256 actual);

    /// @notice Reverts unless the v1 protocol adapter is stopped, so that its state cannot change while it is read.
    modifier whenProtocolAdapterV1Stopped() {
        require(Pausable(_PROTOCOL_ADAPTER_V1).paused(), ProtocolAdapterV1NotStopped(_PROTOCOL_ADAPTER_V1));
        _;
    }

    /// @notice The constructor of the implementation, forwarding the verifier route to the base contract.
    /// @param riscZeroVerifierRouter The RISC Zero verifier router.
    /// @param riscZeroVerifierSelector The selector of the RISC Zero verifier the router must route to.
    /// @param protocolAdapterV1 The v1 protocol adapter of this chain, whose state this contract copies in.
    /// @custom:oz-upgrades-unsafe-allow constructor state-variable-immutable
    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector, address protocolAdapterV1)
        ProtocolAdapter(riscZeroVerifierRouter, riscZeroVerifierSelector)
    {
        require(protocolAdapterV1 != address(0), ZeroProtocolAdapterV1NotAllowed());

        _PROTOCOL_ADAPTER_V1 = protocolAdapterV1;
    }

    /// @notice Initializes the protocol adapter contract and pauses it, so that nothing executes before the state
    /// is in.
    /// @param initialOwner The account receiving ownership, and with it the authority to copy the state in and to
    /// unpause.
    function initialize( /* solhint-disable-line comprehensive-interface*/
        address initialOwner
    )
        external
        override
        initializer
    {
        __ProtocolAdapter_init(initialOwner);
        _pause();
    }

    /// @inheritdoc IProtocolAdapterTransition
    function seedCommitmentTree(bytes32[] calldata sides)
        external
        override
        onlyOwner
        whenPaused
        whenProtocolAdapterV1Stopped
    {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();
        require($._merkleTree.leafCount() == 0, CommitmentTreeNotEmpty());

        uint256 leafCount = ICommitmentTree(_PROTOCOL_ADAPTER_V1).commitmentCount();
        require(leafCount != 0, EmptyCommitmentTreeNotAllowed());

        uint8 treeDepth = sides.length.toUint8();
        uint256 capacity = uint256(1) << treeDepth;
        require(leafCount < capacity, LeafCountExceedsCapacity({leafCount: leafCount, capacity: capacity}));

        $._merkleTree._nextLeafIndex = leafCount;
        $._merkleTree._sides = sides;
        $._merkleTree._zeros = SeededTree.zeroHashes(treeDepth);

        // The sides are the only value a caller supplies, and this is what binds them to v1.
        bytes32 expectedRoot = ICommitmentTree(_PROTOCOL_ADAPTER_V1).latestCommitmentTreeRoot();
        bytes32 root = $._merkleTree.currentRoot();
        require(root == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: root}));

        // The empty-tree root stays. A transaction that consumes an ephemeral resource proves membership against it.
        _addCommitmentTreeRoot(root);

        emit CommitmentTreeSeeded({root: root, leafCount: leafCount});
    }

    /// @inheritdoc IProtocolAdapterTransition
    function seedNullifierSet(uint256 count) external override onlyOwner whenPaused whenProtocolAdapterV1Stopped {
        EnumerableSet.Bytes32Set storage nullifiers = _getNullifierSetStorage()._nullifierSet;

        uint256 start = nullifiers.length();
        uint256 available = INullifierSet(_PROTOCOL_ADAPTER_V1).nullifierCount() - start;
        require(count <= available, NullifierBatchOutOfRange({available: available, requested: count}));

        for (uint256 i = 0; i < count; ++i) {
            uint256 index = start + i;

            // v1 exposes no batch getter, and it is a fixed, stopped contract.
            // slither-disable-next-line calls-loop
            bytes32 nullifier = INullifierSet(_PROTOCOL_ADAPTER_V1).nullifierAtIndex(index);
            _addNullifier(nullifier);

            // The nullifier must occupy the same index here as it does in v1.
            bytes32 stored = nullifiers.at(index);
            require(stored == nullifier, NullifierIndexMismatch({index: index, expected: nullifier, actual: stored}));
        }

        emit NullifierSetSeeded({start: start, count: count});
    }

    /// @inheritdoc IProtocolAdapterTransition
    function getProtocolAdapterV1() external view override returns (address protocolAdapterV1) {
        protocolAdapterV1 = _PROTOCOL_ADAPTER_V1;
    }

    /// @notice Lifts the pause, once this protocol adapter holds the commitment tree and the nullifier set of the v1
    /// protocol adapter. The historical roots are the one difference that stays: v1 keeps every root it ever had, this
    /// contract keeps two — the empty-tree root at index 0, and the latest root of the stopped v1 protocol adapter at
    /// index 1.
    function _unpause() internal override {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        uint256 expectedCommitments = ICommitmentTree(_PROTOCOL_ADAPTER_V1).commitmentCount();
        uint256 actualCommitments = $._merkleTree.leafCount();
        require(
            actualCommitments == expectedCommitments,
            CommitmentCountMismatch({expected: expectedCommitments, actual: actualCommitments})
        );

        bytes32 expectedRoot = ICommitmentTree(_PROTOCOL_ADAPTER_V1).latestCommitmentTreeRoot();
        bytes32 actualRoot = $._merkleTree.currentRoot();
        require(actualRoot == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: actualRoot}));

        uint256 rootCount = $._roots.length();
        require(rootCount == 2, HistoricalRootCountMismatch({expected: 2, actual: rootCount}));

        bytes32 initialRoot = $._roots.at(0);
        require(
            initialRoot == SHA256.EMPTY_HASH,
            HistoricalRootMismatch({index: 0, expected: SHA256.EMPTY_HASH, actual: initialRoot})
        );

        bytes32 copiedRoot = $._roots.at(1);
        require(
            copiedRoot == expectedRoot, HistoricalRootMismatch({index: 1, expected: expectedRoot, actual: copiedRoot})
        );

        uint256 expectedNullifiers = INullifierSet(_PROTOCOL_ADAPTER_V1).nullifierCount();
        uint256 actualNullifiers = _getNullifierSetStorage()._nullifierSet.length();
        require(
            actualNullifiers == expectedNullifiers,
            NullifierCountMismatch({expected: expectedNullifiers, actual: actualNullifiers})
        );

        super._unpause();
    }
}
