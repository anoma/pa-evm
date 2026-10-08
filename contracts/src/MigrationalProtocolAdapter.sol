// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SafeCast} from "@openzeppelin-contracts-5.7.0/utils/math/SafeCast.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {INullifierSet as INullifierSetV1} from "anoma-pa-evm-1.1.0/src/interfaces/INullifierSet.sol";

import {ICommitmentTree} from "./interfaces/ICommitmentTree.sol";
import {IMigrational} from "./interfaces/IMigrational.sol";
import {MerkleTree} from "./libs/MerkleTree.sol";
import {SHA256} from "./libs/SHA256.sol";
import {ProtocolAdapter} from "./ProtocolAdapter.sol";

/// @title MigrationalProtocolAdapter
/// @author Anoma Foundation, 2026
/// @notice The protocol adapter implementation used to migrate Anoma Galileo v1 to v2. It starts paused and copies the
/// v1 state in — the commitment tree and the nullifier set — reading every value from the v1 protocol adapter itself.
/// It refuses to unpause until its commitment tree and its nullifier set hold what v1 holds.
/// @dev The contract holds no storage of its own, so upgrading away from it leaves no namespace behind.
/// @custom:security-contact security@anoma.foundation
contract MigrationalProtocolAdapter is IMigrational, ProtocolAdapter {
    using MerkleTree for MerkleTree.Tree;
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
    error MissingHistoricalRoot(bytes32 root);
    error NullifierBatchOutOfRange(uint256 end, uint256 nullifierCount);
    error NullifierBatchLeavesGap(uint256 start);
    error MissingNullifier(bytes32 nullifier);

    /// @notice Reverts unless the v1 protocol adapter is stopped, so that its state cannot change while it is read.
    modifier whenProtocolAdapterV1Stopped() {
        _requireProtocolAdapterV1Stopped();
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

    /// @inheritdoc IMigrational
    function migrateCommitmentTree(bytes32[] calldata sides)
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
        $._merkleTree._zeros = _zeroHashes(treeDepth);

        // The sides are the only value a caller supplies, and this is what binds them to v1.
        bytes32 expectedRoot = ICommitmentTree(_PROTOCOL_ADAPTER_V1).latestCommitmentTreeRoot();
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
        whenProtocolAdapterV1Stopped
    {
        uint256 end = start + count;
        uint256 nullifierCount = INullifierSetV1(_PROTOCOL_ADAPTER_V1).nullifierCount();
        // solhint-disable-next-line gas-strict-inequalities
        require(end <= nullifierCount, NullifierBatchOutOfRange({end: end, nullifierCount: nullifierCount}));

        // The copied nullifiers stay v1's first ones: a later start leaves a gap, an earlier one repeats a nullifier.
        require(
            start == 0 || _isNullifierContained(INullifierSetV1(_PROTOCOL_ADAPTER_V1).nullifierAtIndex(start - 1)),
            NullifierBatchLeavesGap(start)
        );

        // NOTE: v1 exposes no batch getter, and it is a fixed, stopped contract, so the read belongs in the loop.
        // forge-lint: disable-next-item(calls-loop)
        for (uint256 i = start; i < end; ++i) {
            _addNullifier(INullifierSetV1(_PROTOCOL_ADAPTER_V1).nullifierAtIndex(i));
        }

        emit NullifierBatchMigrated({start: start, count: count});
    }

    /// @inheritdoc IMigrational
    function getProtocolAdapterV1() external view override returns (address protocolAdapterV1) {
        protocolAdapterV1 = _PROTOCOL_ADAPTER_V1;
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

    /// @notice Reverts unless this protocol adapter holds the commitment tree and the nullifier set of the v1 protocol
    /// adapter. The historical roots are the one difference that stays: v1 keeps every root it ever had, this contract
    /// keeps two, the empty-tree root and the latest root of the stopped v1 protocol adapter.
    function _checkStateMigrationIsComplete() internal view {
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

        require(_isCommitmentTreeRootContained(SHA256.EMPTY_HASH), MissingHistoricalRoot(SHA256.EMPTY_HASH));
        require(_isCommitmentTreeRootContained(expectedRoot), MissingHistoricalRoot(expectedRoot));

        // The copied nullifiers are v1's first ones, so they are all in once the last one is.
        uint256 nullifierCount = INullifierSetV1(_PROTOCOL_ADAPTER_V1).nullifierCount();
        if (nullifierCount != 0) {
            bytes32 lastNullifier = INullifierSetV1(_PROTOCOL_ADAPTER_V1).nullifierAtIndex(nullifierCount - 1);
            require(_isNullifierContained(lastNullifier), MissingNullifier(lastNullifier));
        }
    }

    /// @notice Reverts unless the v1 protocol adapter is stopped.
    function _requireProtocolAdapterV1Stopped() internal view {
        require(Pausable(_PROTOCOL_ADAPTER_V1).paused(), ProtocolAdapterV1NotStopped(_PROTOCOL_ADAPTER_V1));
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
