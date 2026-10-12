// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin-contracts-5.7.0/utils/math/Math.sol";
import {ICommitmentTree as IImmutableCommitmentTree} from "anoma-pa-evm-1.1.0/src/interfaces/ICommitmentTree.sol";
import {INullifierSet as IImmutableNullifierSet} from "anoma-pa-evm-1.1.0/src/interfaces/INullifierSet.sol";

import {ICommitmentTree} from "../../src/interfaces/ICommitmentTree.sol";
import {INullifierSet} from "../../src/interfaces/INullifierSet.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";
import {MigrationScript} from "./MigrationScript.s.sol";

/// @title MigrateProtocolAdapterState
/// @author Anoma Foundation, 2026
/// @notice A script to copy one chain's state from the stopped legacy immutable protocol adapter into a proxy running
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
    /// @notice The storage slot of `_merkleTree._nextLeafIndex` in the legacy immutable protocol adapter.
    /// `ReentrancyGuardTransient` holds no persistent state, and `Ownable._owner` and `Pausable._paused` are 20 and 1
    /// bytes, so they share slot 0 and the tree starts at slot 1.
    /// @dev Checked against `commitmentCount()` before anything is read from it.
    uint256 internal constant _IMMUTABLE_PROTOCOL_ADAPTER_NEXT_LEAF_INDEX_SLOT = 1;

    /// @notice The storage slot of `_merkleTree._sides` in the legacy immutable protocol adapter, the one part of the
    /// tree it does not expose through a getter.
    /// @dev Checked against `commitmentTreeDepth()` before anything is read from it.
    uint256 internal constant _IMMUTABLE_PROTOCOL_ADAPTER_SIDES_SLOT = 2;

    /// @notice The number of nullifiers read from the legacy immutable protocol adapter and written to the proxy per
    /// transaction. Adding one nullifier costs two fresh storage writes, about 45000 gas, so a batch of this size
    /// costs roughly 9 million — a third of a 30 million block. A chain holding 8000 nullifiers therefore takes 40
    /// transactions.
    uint256 public constant NULLIFIERS_PER_BATCH = 200;

    /// @notice Thrown if a storage slot of the legacy immutable protocol adapter does not hold what its public getter
    /// reports, i.e. if its storage layout is not the one this script reads.
    error UnexpectedStorageLayout(uint256 slot, uint256 expected, uint256 actual);

    /// @notice Thrown if the copied commitment tree does not reproduce the latest root of the legacy immutable protocol
    /// adapter.
    error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);

    /// @notice Thrown if the copied commitment tree holds a different number of commitments than the legacy immutable
    /// protocol adapter.
    error CommitmentCountMismatch(uint256 expected, uint256 actual);

    /// @notice Thrown if a nullifier of the legacy immutable protocol adapter is absent from the copied set.
    error MissingNullifier(bytes32 nullifier);

    /// @notice Thrown if the proxy does not run the plain implementation.
    error ImplementationMismatch(address expected, address actual);

    /// @notice Thrown if the proxy is still paused.
    error ProxyPaused(address proxy);

    /// @notice Thrown if the proxy has another owner than the expected one.
    error OwnerMismatch(address expected, address actual);

    /// @notice Copies the state of the legacy immutable protocol adapter into the proxy. The proxy stays paused, so no
    /// transaction can use the kind table before the ERC20 forwarder balances move. Without `--broadcast` the run is
    /// simulated locally.
    /// @dev Run it with `--slow`, so that each transaction is confirmed before the next is sent. Without it every
    /// transaction goes out at once, and a transaction that reverts on chain does not stop the ones behind it. Every
    /// step skips once it is done, so a run that stops early can be repeated. Run
    /// `FinalizeProtocolAdapterStateMigration` once the ERC20 forwarder balances moved.
    /// @param isProduction Whether to copy the state into the production or the staging proxy.
    function run(bool isProduction) public {
        (address immutableProtocolAdapter, address proxy) = _configuration(isProduction);
        address implementation = _requireDeployedImplementation(new DeployProtocolAdapterImplementation());
        ProtocolAdapter protocolAdapter = ProtocolAdapter(proxy);

        if (protocolAdapter.getImplementation() != implementation) {
            _requireImmutableProtocolAdapter({immutableProtocolAdapter: immutableProtocolAdapter, proxy: proxy});

            if (protocolAdapter.paused()) {
                _migrateCommitmentTree({immutableProtocolAdapter: immutableProtocolAdapter, proxy: proxy});
                _migrateNullifierSet({immutableProtocolAdapter: immutableProtocolAdapter, proxy: proxy});
            }
        }
    }

    /// @notice Checks a migrated proxy against the stopped legacy immutable protocol adapter and against the end state
    /// of the migration, reading both from the chain. Run it after `FinalizeProtocolAdapterStateMigration` has
    /// broadcast, because that script only ever sees the simulated state.
    /// @param isProduction Whether to check the production or the staging proxy. The production proxy owner must own a
    /// production proxy.
    function verify(bool isProduction) public {
        (address immutableProtocolAdapter, address proxy) = _configuration(isProduction);
        _check({immutableProtocolAdapter: immutableProtocolAdapter, proxy: proxy});
        _checkEndState({proxy: proxy, isProduction: isProduction});
    }

    /// @notice Copies the commitment tree of the legacy immutable protocol adapter into the proxy, unless an earlier
    /// run did. The tree's sides come from its storage, since it exposes no getter for them; the slot is checked
    /// against its own getters first, and the proxy rejects sides that do not reproduce the root.
    /// @param immutableProtocolAdapter The stopped immutable protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    function _migrateCommitmentTree(address immutableProtocolAdapter, address proxy) internal {
        // The proxy refuses to write a tree twice.
        if (ICommitmentTree(proxy).commitmentCount() != 0) {
            return;
        }

        uint256 leafCount = IImmutableCommitmentTree(immutableProtocolAdapter).commitmentCount();
        uint8 treeDepth = IImmutableCommitmentTree(immutableProtocolAdapter).commitmentTreeDepth();

        _checkSlot({
            target: immutableProtocolAdapter,
            slot: _IMMUTABLE_PROTOCOL_ADAPTER_NEXT_LEAF_INDEX_SLOT,
            expected: leafCount
        });
        _checkSlot({
            target: immutableProtocolAdapter, slot: _IMMUTABLE_PROTOCOL_ADAPTER_SIDES_SLOT, expected: treeDepth
        });

        bytes32[] memory sides = new bytes32[](treeDepth);
        uint256 firstElementSlot = uint256(keccak256(abi.encode(_IMMUTABLE_PROTOCOL_ADAPTER_SIDES_SLOT)));
        for (uint256 i = 0; i < treeDepth; ++i) {
            sides[i] = vm.load({target: immutableProtocolAdapter, slot: bytes32(firstElementSlot + i)});
        }

        address owner = MigrationalProtocolAdapter(proxy).owner();
        vm.broadcast(owner);
        MigrationalProtocolAdapter(proxy).migrateCommitmentTree(sides);
    }

    /// @notice Copies the nullifiers into the proxy, `NULLIFIERS_PER_BATCH` per transaction, from the first one that an
    /// earlier run did not copy. The proxy reads each batch from the legacy immutable protocol adapter itself and
    /// rejects a batch that does not start where the copied nullifiers end, so a batch can neither be skipped nor
    /// repeated.
    /// @param immutableProtocolAdapter The stopped immutable protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    function _migrateNullifierSet(address immutableProtocolAdapter, address proxy) internal {
        uint256 total = IImmutableNullifierSet(immutableProtocolAdapter).nullifierCount();
        address owner = MigrationalProtocolAdapter(proxy).owner();

        for (
            uint256 migrated = _migratedNullifierCount({
                immutableProtocolAdapter: immutableProtocolAdapter, proxy: proxy
            });
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

    /// @notice Checks the proxy against the stopped legacy immutable protocol adapter: the latest root, the commitment
    /// count, and every nullifier. Reverts on the first difference.
    /// @param immutableProtocolAdapter The stopped immutable protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    function _check(address immutableProtocolAdapter, address proxy) internal view {
        bytes32 expectedRoot = IImmutableCommitmentTree(immutableProtocolAdapter).latestCommitmentTreeRoot();
        bytes32 actualRoot = ICommitmentTree(proxy).latestCommitmentTreeRoot();
        require(actualRoot == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: actualRoot}));

        uint256 expectedCount = IImmutableCommitmentTree(immutableProtocolAdapter).commitmentCount();
        uint256 actualCount = ICommitmentTree(proxy).commitmentCount();
        require(actualCount == expectedCount, CommitmentCountMismatch({expected: expectedCount, actual: actualCount}));

        uint256 nullifierCount = IImmutableNullifierSet(immutableProtocolAdapter).nullifierCount();
        for (uint256 i = 0; i < nullifierCount; ++i) {
            bytes32 nullifier = IImmutableNullifierSet(immutableProtocolAdapter).nullifierAtIndex(i);
            require(INullifierSet(proxy).isNullifierContained(nullifier), MissingNullifier(nullifier));
        }
    }

    /// @notice Returns the number of nullifiers of the legacy immutable protocol adapter that the proxy holds. The
    /// proxy holds its first nullifiers, so a binary search over its indices finds where they end.
    /// @param immutableProtocolAdapter The stopped immutable protocol adapter.
    /// @param proxy The proxy of the protocol adapter.
    /// @return count The number of copied nullifiers, which is also the index of the next one to copy.
    function _migratedNullifierCount(address immutableProtocolAdapter, address proxy)
        internal
        view
        returns (uint256 count)
    {
        uint256 end = IImmutableNullifierSet(immutableProtocolAdapter).nullifierCount();

        while (count < end) {
            uint256 middle = Math.average(count, end);
            bytes32 nullifier = IImmutableNullifierSet(immutableProtocolAdapter).nullifierAtIndex(middle);

            if (INullifierSet(proxy).isNullifierContained(nullifier)) {
                count = middle + 1;
            } else {
                end = middle;
            }
        }
    }

    /// @notice Checks that a storage slot of the legacy immutable protocol adapter holds what its public getter
    /// reports, which pins the layout this script reads the tree sides from.
    /// @param target The immutable protocol adapter.
    /// @param slot The storage slot to read.
    /// @param expected The value the matching getter reports.
    function _checkSlot(address target, uint256 slot, uint256 expected) internal view {
        uint256 actual = uint256(vm.load({target: target, slot: bytes32(slot)}));
        require(actual == expected, UnexpectedStorageLayout({slot: slot, expected: expected, actual: actual}));
    }
}
