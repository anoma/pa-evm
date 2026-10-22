// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SafeCast} from "@openzeppelin-contracts-5.7.0/utils/math/SafeCast.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";

import {ILegacyCommitmentTree} from "./interfaces/ILegacyCommitmentTree.sol";
import {ILegacyNullifierSet} from "./interfaces/ILegacyNullifierSet.sol";
import {IMigrational} from "./interfaces/IMigrational.sol";
import {MerkleTree} from "./libs/MerkleTree.sol";
import {SHA256} from "./libs/SHA256.sol";
import {ProtocolAdapter} from "./ProtocolAdapter.sol";

/// @title MigrationalProtocolAdapter
/// @author Anoma Foundation, 2026
/// @notice The protocol adapter implementation that takes over the state of a chain's legacy protocol
/// adapter. It starts paused and copies the state in — the commitment tree and the nullifier set — reading every value
/// from the legacy protocol adapter itself. It refuses to unpause until its commitment tree and its nullifier set
/// hold what the legacy protocol adapter holds.
/// @dev The contract holds no storage of its own, so upgrading away from it leaves no namespace behind.
/// @custom:security-contact security@anoma.foundation
contract MigrationalProtocolAdapter is IMigrational, ProtocolAdapter {
    using MerkleTree for MerkleTree.Tree;
    using SafeCast for uint256;

    /// @notice The legacy protocol adapter this contract copies its state from.
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable
    address internal immutable _LEGACY_PROTOCOL_ADAPTER;

    error ZeroLegacyProtocolAdapterNotAllowed();
    error LegacyProtocolAdapterNotStopped(address legacyProtocolAdapter);
    error CommitmentTreeNotEmpty();
    error EmptyCommitmentTreeNotAllowed();
    error LeafCountExceedsCapacity(uint256 leafCount, uint256 capacity);
    error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);
    error CommitmentCountMismatch(uint256 expected, uint256 actual);
    error MissingHistoricalRoot(bytes32 root);
    error NullifierBatchOutOfRange(uint256 end, uint256 nullifierCount);
    error NullifierBatchLeavesGap(uint256 start);
    error MissingNullifier(bytes32 nullifier);

    /// @notice Reverts unless the legacy protocol adapter is stopped, so that its state cannot change while
    /// it is read.
    modifier whenLegacyProtocolAdapterStopped() {
        _requireLegacyProtocolAdapterStopped();
        _;
    }

    /// @notice The constructor of the implementation, forwarding the verifier route to the base contract.
    /// @param riscZeroVerifierRouter The RISC Zero verifier router.
    /// @param riscZeroVerifierSelector The selector of the RISC Zero verifier the router must route to.
    /// @param legacyProtocolAdapter The legacy protocol adapter of this chain, whose state this contract
    /// copies in.
    /// @custom:oz-upgrades-unsafe-allow constructor state-variable-immutable
    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector, address legacyProtocolAdapter)
        ProtocolAdapter(riscZeroVerifierRouter, riscZeroVerifierSelector)
    {
        require(legacyProtocolAdapter != address(0), ZeroLegacyProtocolAdapterNotAllowed());

        _LEGACY_PROTOCOL_ADAPTER = legacyProtocolAdapter;
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

    /// @inheritdoc IMigrational
    function migrateCommitmentTree(bytes32[] calldata sides)
        external
        override
        onlyOwner
        whenPaused
        whenLegacyProtocolAdapterStopped
    {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();
        require($._merkleTree.leafCount() == 0, CommitmentTreeNotEmpty());

        uint256 leafCount = ILegacyCommitmentTree(_LEGACY_PROTOCOL_ADAPTER).commitmentCount();
        require(leafCount != 0, EmptyCommitmentTreeNotAllowed());

        uint8 treeDepth = sides.length.toUint8();
        uint256 capacity = uint256(1) << treeDepth;
        require(leafCount < capacity, LeafCountExceedsCapacity({leafCount: leafCount, capacity: capacity}));

        $._merkleTree._nextLeafIndex = leafCount;
        $._merkleTree._sides = sides;
        $._merkleTree._zeros = _zeroHashes(treeDepth);

        // The sides are the only value a caller supplies, and this is what binds them to the legacy protocol
        // adapter.
        bytes32 expectedRoot = ILegacyCommitmentTree(_LEGACY_PROTOCOL_ADAPTER).latestCommitmentTreeRoot();
        bytes32 root = $._merkleTree.currentRoot();
        require(root == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: root}));

        // The empty-tree root stays. A transaction that consumes an ephemeral resource proves membership against it.
        _addCommitmentTreeRoot(root);

        emit CommitmentTreeMigrated({root: root, leafCount: leafCount});
    }

    /// @inheritdoc IMigrational
    function migrateNullifierSet(uint256 start, uint256 count)
        external
        override
        onlyOwner
        whenPaused
        whenLegacyProtocolAdapterStopped
    {
        uint256 end = start + count;
        uint256 nullifierCount = ILegacyNullifierSet(_LEGACY_PROTOCOL_ADAPTER).nullifierCount();
        require(end <= nullifierCount, NullifierBatchOutOfRange({end: end, nullifierCount: nullifierCount}));

        // The copied nullifiers stay the first ones of the legacy protocol adapter: a later start leaves a
        // gap, an earlier one repeats a nullifier.
        require(
            start == 0
                || _isNullifierContained(ILegacyNullifierSet(_LEGACY_PROTOCOL_ADAPTER).nullifierAtIndex(start - 1)),
            NullifierBatchLeavesGap(start)
        );

        // NOTE: The legacy protocol adapter exposes no batch getter and is stopped, so the read belongs in
        // the loop.
        // forge-lint: disable-next-item(calls-loop)
        for (uint256 i = start; i < end; ++i) {
            _addNullifier(ILegacyNullifierSet(_LEGACY_PROTOCOL_ADAPTER).nullifierAtIndex(i));
        }

        emit NullifierBatchMigrated({start: start, count: count});
    }

    /// @inheritdoc IMigrational
    function getLegacyProtocolAdapter() external view override returns (address legacyProtocolAdapter) {
        legacyProtocolAdapter = _LEGACY_PROTOCOL_ADAPTER;
    }

    /// @notice Lifts the pause, once the copy-in is complete.
    function _unpause() internal override {
        _checkStateMigrationIsComplete();
        super._unpause();
    }

    /// @notice Authorizes an upgrade of the proxy and reports the migration as finished.
    /// @dev The upgrade away from this implementation is the last step of the migration, so it is what the event
    /// marks. The base authorizes the same way and emits nothing.
    function _authorizeUpgrade(address) internal override onlyOwner {
        emit MigrationFinalized();
    }

    /// @notice Reverts unless this protocol adapter holds the commitment tree and the nullifier set of the legacy
    /// protocol adapter. The historical roots are the one difference that stays: the legacy protocol
    /// adapter keeps every root it ever had, this contract keeps two, the empty-tree root and its latest root.
    function _checkStateMigrationIsComplete() internal view {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        uint256 expectedCommitments = ILegacyCommitmentTree(_LEGACY_PROTOCOL_ADAPTER).commitmentCount();
        uint256 actualCommitments = $._merkleTree.leafCount();
        require(
            actualCommitments == expectedCommitments,
            CommitmentCountMismatch({expected: expectedCommitments, actual: actualCommitments})
        );

        bytes32 expectedRoot = ILegacyCommitmentTree(_LEGACY_PROTOCOL_ADAPTER).latestCommitmentTreeRoot();
        bytes32 actualRoot = $._merkleTree.currentRoot();
        require(actualRoot == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: actualRoot}));

        require(_isCommitmentTreeRootHistorical(SHA256.EMPTY_HASH), MissingHistoricalRoot(SHA256.EMPTY_HASH));
        require(_isCommitmentTreeRootHistorical(expectedRoot), MissingHistoricalRoot(expectedRoot));

        // The copied nullifiers are the first ones of the legacy protocol adapter, so all are in once its
        // last one is.
        uint256 nullifierCount = ILegacyNullifierSet(_LEGACY_PROTOCOL_ADAPTER).nullifierCount();
        if (nullifierCount != 0) {
            bytes32 lastNullifier = ILegacyNullifierSet(_LEGACY_PROTOCOL_ADAPTER).nullifierAtIndex(nullifierCount - 1);
            require(_isNullifierContained(lastNullifier), MissingNullifier(lastNullifier));
        }
    }

    /// @notice Reverts unless the legacy protocol adapter is stopped.
    function _requireLegacyProtocolAdapterStopped() internal view {
        require(Pausable(_LEGACY_PROTOCOL_ADAPTER).paused(), LegacyProtocolAdapterNotStopped(_LEGACY_PROTOCOL_ADAPTER));
    }

    /// @notice Returns the roots of the empty subtrees, one per level, for a tree of the given depth.
    /// @param treeDepth The depth of the tree.
    /// @return hashes The empty-subtree roots, from level 0 up to `treeDepth`.
    /// @dev `MerkleTree.setup` stores the first and `MerkleTree.push` appends one per level it adds, so this
    /// reproduces what a tree of that depth holds.
    function _zeroHashes(uint8 treeDepth) internal pure returns (bytes32[] memory hashes) {
        hashes = new bytes32[](uint256(treeDepth) + 1);
        hashes[0] = SHA256.EMPTY_HASH;

        for (uint256 i = 0; i < treeDepth; ++i) {
            hashes[i + 1] = SHA256.hash(hashes[i], hashes[i]);
        }
    }
}
