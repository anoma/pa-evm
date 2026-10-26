// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin-contracts-5.7.0/utils/math/Math.sol";

import {ICommitmentTree} from "../../src/interfaces/ICommitmentTree.sol";
import {ILegacyCommitmentTree} from "../../src/interfaces/ILegacyCommitmentTree.sol";
import {ILegacyNullifierSet} from "../../src/interfaces/ILegacyNullifierSet.sol";
import {INullifierSet} from "../../src/interfaces/INullifierSet.sol";
import {SHA256} from "../../src/libs/SHA256.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";
import {MigrationScript} from "./MigrationScript.s.sol";

/// @title MigrateProtocolAdapterState
/// @author Anoma Foundation, 2026
/// @notice A script to copy one chain's state from the stopped legacy protocol adapter into a proxy running
/// `MigrationalProtocolAdapter`. `run` copies the commitment tree and the nullifier set, and the proxy stays paused,
/// so that the ERC20 forwarder balances move before anyone can transact. `FinalizeProtocolAdapterStateMigration` then
/// unpauses, upgrades the proxy to the plain `ProtocolAdapter` implementation and, in production, transfers it to the
/// production proxy owner. `verify` checks the migrated proxy against the chain. Every step skips once it is done, so a
/// run that stops early is repeated until it reaches the end.
/// @dev The proxy owner sends every transaction, so this serves a chain whose owner is an account. A chain owned by a
/// Safe multisig needs the same calls proposed there instead. The script broadcasts as the owner, because with
/// `--account` alone forge runs it as its default sender.
/// @custom:security-contact security@anoma.foundation
contract MigrateProtocolAdapterState is MigrationScript {
    /// @notice The storage slot of `_merkleTree._nextLeafIndex` in the legacy protocol adapter.
    /// `ReentrancyGuardTransient` holds no persistent state, and `Ownable._owner` and `Pausable._paused` are 20 and 1
    /// bytes, so they share slot 0 and the tree starts at slot 1.
    /// @dev Checked against `commitmentCount()` before anything is read from it.
    uint256 internal constant _LEGACY_PROTOCOL_ADAPTER_NEXT_LEAF_INDEX_SLOT = 1;

    /// @notice The storage slot of `_merkleTree._sides` in the legacy protocol adapter, the one part of the
    /// tree it does not expose through a getter.
    /// @dev Checked against `commitmentTreeDepth()` before anything is read from it.
    uint256 internal constant _LEGACY_PROTOCOL_ADAPTER_SIDES_SLOT = 2;

    /// @notice The number of nullifiers read from the legacy protocol adapter and written to the proxy per
    /// transaction. Adding one nullifier costs two fresh storage writes, about 45000 gas, so a batch of this size
    /// costs roughly 9 million — a third of a 30 million block. A chain holding 8000 nullifiers therefore takes 40
    /// transactions.
    uint256 public constant NULLIFIERS_PER_BATCH = 200;

    /// @notice Thrown if a storage slot of the legacy protocol adapter does not hold what its public getter
    /// reports, i.e. if its storage layout is not the one this script reads.
    error UnexpectedStorageLayout(uint256 slot, uint256 expected, uint256 actual);

    /// @notice Thrown if the copied commitment tree does not reproduce the latest root of the legacy protocol
    /// adapter.
    error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);

    /// @notice Thrown if the proxy holds fewer commitments than the legacy protocol adapter.
    error CommitmentCountMismatch(uint256 expected, uint256 actual);

    /// @notice Thrown if the proxy no longer accepts a root that the copy left it.
    error MissingHistoricalRoot(bytes32 root);

    /// @notice Thrown if a nullifier of the legacy protocol adapter is absent from the copied set.
    error MissingNullifier(bytes32 nullifier);

    /// @notice Thrown if the proxy does not run the plain implementation.
    error ImplementationMismatch(address expected, address actual);

    /// @notice Thrown if the proxy is still paused.
    error ProxyPaused(address proxy);

    /// @notice Thrown if the proxy has another owner than the expected one.
    error OwnerMismatch(address expected, address actual);

    /// @notice Copies the state of the legacy protocol adapter into the proxy. The proxy stays paused, so no
    /// transaction can use the kind table before the ERC20 forwarder balances move. Without `--broadcast` the run is
    /// simulated locally.
    /// @dev Run it with `--slow`, so that each transaction is confirmed before the next is sent. Without it every
    /// transaction goes out at once, and a transaction that reverts on chain does not stop the ones behind it. Every
    /// step skips once it is done, so a run that stops early can be repeated. Run
    /// `FinalizeProtocolAdapterStateMigration` once the ERC20 forwarder balances moved.
    /// @param isProduction Whether to copy the state into the production or the staging proxy.
    function run(bool isProduction) public {
        (address legacyProtocolAdapter, address proxy) = _configuration(isProduction);
        address implementation = _requireDeployedImplementation(new DeployProtocolAdapterImplementation());
        ProtocolAdapter protocolAdapter = ProtocolAdapter(proxy);

        if (protocolAdapter.getImplementation() != implementation) {
            _requireLegacyProtocolAdapter({legacyProtocolAdapter: legacyProtocolAdapter, proxy: proxy});

            if (protocolAdapter.paused()) {
                _migrateCommitmentTree({legacyProtocolAdapter: legacyProtocolAdapter, proxy: proxy});
                _migrateNullifierSet({legacyProtocolAdapter: legacyProtocolAdapter, proxy: proxy});
            }
        }
    }

    /// @notice Checks a migrated proxy against the stopped legacy protocol adapter and against the end state
    /// of the migration, reading both from the chain. Run it after `FinalizeProtocolAdapterStateMigration` has
    /// broadcast, because that script only ever sees the simulated state. Transactions settled since then may have
    /// grown the commitment tree past the copy.
    /// @param isProduction Whether to check the production or the staging proxy. The production proxy owner must own a
    /// production proxy.
    function verify(bool isProduction) public {
        (address legacyProtocolAdapter, address proxy) = _configuration(isProduction);
        _check({legacyProtocolAdapter: legacyProtocolAdapter, proxy: proxy});
        _checkEndState({proxy: proxy, isProduction: isProduction});
    }

    /// @notice Copies the commitment tree of the legacy protocol adapter into the proxy, unless an earlier
    /// run did. The tree's left siblings come from its storage, since it exposes no getter for them; the slot is
    /// checked against its own getters first, and the proxy rejects left siblings that do not reproduce the root.
    /// @param legacyProtocolAdapter The stopped legacy protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    function _migrateCommitmentTree(address legacyProtocolAdapter, address proxy) internal {
        // The proxy refuses to write a tree twice.
        if (ICommitmentTree(proxy).commitmentCount() != 0) {
            return;
        }

        uint256 leafCount = ILegacyCommitmentTree(legacyProtocolAdapter).commitmentCount();
        uint8 treeDepth = ILegacyCommitmentTree(legacyProtocolAdapter).commitmentTreeDepth();

        _checkSlot({
            target: legacyProtocolAdapter, slot: _LEGACY_PROTOCOL_ADAPTER_NEXT_LEAF_INDEX_SLOT, expected: leafCount
        });
        _checkSlot({target: legacyProtocolAdapter, slot: _LEGACY_PROTOCOL_ADAPTER_SIDES_SLOT, expected: treeDepth});

        bytes32[] memory leftSiblings = new bytes32[](treeDepth);
        uint256 firstElementSlot = uint256(keccak256(abi.encode(_LEGACY_PROTOCOL_ADAPTER_SIDES_SLOT)));
        for (uint256 i = 0; i < treeDepth; ++i) {
            leftSiblings[i] = vm.load({target: legacyProtocolAdapter, slot: bytes32(firstElementSlot + i)});
        }

        address owner = MigrationalProtocolAdapter(proxy).owner();
        vm.broadcast(owner);
        MigrationalProtocolAdapter(proxy).migrateCommitmentTree(leftSiblings);
    }

    /// @notice Copies the nullifiers into the proxy, `NULLIFIERS_PER_BATCH` per transaction, from the first one that an
    /// earlier run did not copy. The proxy reads each batch from the legacy protocol adapter itself and
    /// rejects a batch that does not start where the copied nullifiers end, so a batch can neither be skipped nor
    /// repeated.
    /// @param legacyProtocolAdapter The stopped legacy protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    function _migrateNullifierSet(address legacyProtocolAdapter, address proxy) internal {
        uint256 total = ILegacyNullifierSet(legacyProtocolAdapter).nullifierCount();
        address owner = MigrationalProtocolAdapter(proxy).owner();

        for (
            uint256 migrated = _migratedNullifierCount({legacyProtocolAdapter: legacyProtocolAdapter, proxy: proxy});
            migrated < total;
            migrated += NULLIFIERS_PER_BATCH
        ) {
            uint256 size = Math.min(NULLIFIERS_PER_BATCH, total - migrated);

            vm.broadcast(owner);
            MigrationalProtocolAdapter(proxy).migrateNullifierSet({start: migrated, count: size});
        }
    }

    /// @notice Checks what the migration leaves besides the copied state: the plain implementation, the lifted pause
    /// and, in production, the production proxy owner. Reverts on the first difference.
    /// @param proxy The proxy of the migrated protocol adapter.
    /// @param isProduction Whether the production proxy owner must own the proxy.
    function _checkEndState(address proxy, bool isProduction) internal {
        ProtocolAdapter protocolAdapter = ProtocolAdapter(proxy);

        // forge-lint: disable-next-line(unused-return)
        (address expectedImplementation,) = new DeployProtocolAdapterImplementation().predict();
        address actualImplementation = protocolAdapter.getImplementation();
        require(
            actualImplementation == expectedImplementation,
            ImplementationMismatch({expected: expectedImplementation, actual: actualImplementation})
        );

        require(!protocolAdapter.paused(), ProxyPaused(proxy));

        if (isProduction) {
            address expectedOwner = new DeployProtocolAdapterProxy().PROXY_OWNER_PRODUCTION();
            address actualOwner = protocolAdapter.owner();
            require(actualOwner == expectedOwner, OwnerMismatch({expected: expectedOwner, actual: actualOwner}));
        }
    }

    /// @notice Checks the proxy against the stopped legacy protocol adapter: the commitment count, the latest root
    /// while no transaction has added a commitment, the two roots the copy leaves, and every nullifier. Reverts on the
    /// first difference.
    /// @param legacyProtocolAdapter The stopped legacy protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    function _check(address legacyProtocolAdapter, address proxy) internal view {
        uint256 expectedCount = ILegacyCommitmentTree(legacyProtocolAdapter).commitmentCount();
        uint256 actualCount = ICommitmentTree(proxy).commitmentCount();
        require(actualCount >= expectedCount, CommitmentCountMismatch({expected: expectedCount, actual: actualCount}));

        bytes32 expectedRoot = ILegacyCommitmentTree(legacyProtocolAdapter).latestCommitmentTreeRoot();
        if (actualCount == expectedCount) {
            bytes32 actualRoot = ICommitmentTree(proxy).latestCommitmentTreeRoot();
            require(
                actualRoot == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: actualRoot})
            );
        }

        // A settled transaction moves the latest root on, but the copied root and the empty-tree root stay.
        require(
            ICommitmentTree(proxy).isCommitmentTreeRootHistorical(expectedRoot), MissingHistoricalRoot(expectedRoot)
        );
        require(
            ICommitmentTree(proxy).isCommitmentTreeRootHistorical(SHA256.EMPTY_HASH),
            MissingHistoricalRoot(SHA256.EMPTY_HASH)
        );

        uint256 nullifierCount = ILegacyNullifierSet(legacyProtocolAdapter).nullifierCount();
        for (uint256 i = 0; i < nullifierCount; ++i) {
            bytes32 nullifier = ILegacyNullifierSet(legacyProtocolAdapter).nullifierAtIndex(i);
            require(INullifierSet(proxy).isNullifierContained(nullifier), MissingNullifier(nullifier));
        }
    }

    /// @notice Returns the number of nullifiers of the legacy protocol adapter that the proxy holds. The
    /// proxy holds its first nullifiers, so a binary search over its indices finds where they end.
    /// @param legacyProtocolAdapter The stopped legacy protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    /// @return count The number of copied nullifiers, which is also the index of the next one to copy.
    function _migratedNullifierCount(address legacyProtocolAdapter, address proxy)
        internal
        view
        returns (uint256 count)
    {
        uint256 end = ILegacyNullifierSet(legacyProtocolAdapter).nullifierCount();

        while (count < end) {
            uint256 middle = Math.average(count, end);
            bytes32 nullifier = ILegacyNullifierSet(legacyProtocolAdapter).nullifierAtIndex(middle);

            if (INullifierSet(proxy).isNullifierContained(nullifier)) {
                count = middle + 1;
            } else {
                end = middle;
            }
        }
    }

    /// @notice Checks that a storage slot of the legacy protocol adapter holds what its public getter
    /// reports, which pins the layout this script reads the left siblings from.
    /// @param target The legacy protocol adapter.
    /// @param slot The storage slot to read.
    /// @param expected The value the matching getter reports.
    function _checkSlot(address target, uint256 slot, uint256 expected) internal view {
        uint256 actual = uint256(vm.load({target: target, slot: bytes32(slot)}));
        require(actual == expected, UnexpectedStorageLayout({slot: slot, expected: expected, actual: actual}));
    }
}
