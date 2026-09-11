// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {Math} from "@openzeppelin-contracts-5.7.0/utils/math/Math.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ICommitmentTree} from "../../src/interfaces/ICommitmentTree.sol";
import {INullifierSet} from "../../src/interfaces/INullifierSet.sol";
import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";

/// @title MigrateProtocolAdapterState
/// @author Anoma Foundation, 2026
/// @notice A script to move one chain's state from the stopped v1 protocol adapter into a v2 proxy running
/// `TransitionalProtocolAdapter`, and to leave that proxy on the plain `ProtocolAdapter` implementation. It copies the
/// commitment tree and the nullifier set, unpauses, and upgrades — in that order, in one run. The unpause compares
/// the copied state against v1 and reverts on a difference, so no run reaches the upgrade with the wrong state. The
/// copy-in is only removed by the upgrade, so a run that stops early must be repeated until it reaches the end.
/// @dev The proxy owner sends every transaction, so this serves a chain whose owner is an account. A chain owned by a
/// Safe multisig needs the same calls proposed there instead.
/// @custom:security-contact security@anoma.foundation
contract MigrateProtocolAdapterState is Script {
    /// @notice The storage slot of `_merkleTree._nextLeafIndex` in the v1 protocol adapter. `ReentrancyGuardTransient`
    /// holds no persistent state, and `Ownable._owner` and `Pausable._paused` are 20 and 1 bytes, so they share slot
    /// 0 and the tree starts at slot 1.
    /// @dev Checked against `commitmentCount()` before anything is read from it.
    uint256 internal constant _V1_NEXT_LEAF_INDEX_SLOT = 1;

    /// @notice The storage slot of `_merkleTree._sides` in the v1 protocol adapter, the one part of the tree it does
    /// not expose through a getter.
    /// @dev Checked against `commitmentTreeDepth()` before anything is read from it.
    uint256 internal constant _V1_SIDES_SLOT = 2;

    /// @notice The number of nullifiers read from v1 and written to the proxy per transaction. Adding one nullifier
    /// costs two fresh storage writes, about 45000 gas, so a batch of this size costs roughly 9 million — a third of
    /// a 30 million block. A chain holding 8000 nullifiers therefore takes 40 transactions.
    uint256 public constant NULLIFIERS_PER_BATCH = 200;

    /// @notice Thrown if the proxy is not paused, i.e. if it is not a freshly initialized transitional deployment.
    error ProxyNotPaused(address proxy);

    /// @notice Thrown if a v1 storage slot does not hold what its public getter reports, i.e. if v1's storage layout
    /// is not the one this script reads.
    error UnexpectedStorageLayout(uint256 slot, uint256 expected, uint256 actual);

    /// @notice Thrown if the copied commitment tree does not reproduce v1's latest root.
    error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);

    /// @notice Thrown if the copied commitment tree holds a different number of commitments than v1.
    error CommitmentCountMismatch(uint256 expected, uint256 actual);

    /// @notice Thrown if the copied nullifier set holds a different number of nullifiers than v1.
    error NullifierCountMismatch(uint256 expected, uint256 actual);

    /// @notice Thrown if a nullifier of v1 is absent from the copied set.
    error MissingNullifier(bytes32 nullifier);

    /// @notice Copies v1's state into the proxy, unpauses, and upgrades to the plain implementation. The unpause is
    /// what checks the copied state against v1. Without `--broadcast` the whole run is simulated locally.
    /// @dev Run it with `--slow`, so that each transaction is confirmed before the next is sent. Without it every
    /// transaction goes out at once, and a batch that reverts on chain does not stop the unpause and the upgrade
    /// behind it. The proxy refuses to unpause until it holds v1's commitment tree and nullifier set, so the run
    /// cannot reach the upgrade with the wrong state; run `verify` afterwards to assert the same against the chain.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter to read the state from.
    /// @param proxy The v2 protocol adapter proxy, running `TransitionalProtocolAdapter` and still paused.
    function run(address protocolAdapterV1, address proxy) public {
        require(Pausable(proxy).paused(), ProxyNotPaused(proxy));

        // The implementation the proxy ends on must already be deployed, by `DeployProtocolAdapterImplementation`.
        DeployProtocolAdapterImplementation implementationDeployScript = new DeployProtocolAdapterImplementation();
        // forge-lint: disable-next-line(unused-return)
        (address implementation,) = implementationDeployScript.predict();
        require(
            implementation.code.length != 0,
            DeployProtocolAdapterImplementation.ImplementationNotDeployed(implementation)
        );

        _seedCommitmentTree({protocolAdapterV1: protocolAdapterV1, proxy: proxy});
        _seedNullifierSet({protocolAdapterV1: protocolAdapterV1, proxy: proxy});

        vm.startBroadcast();
        TransitionalProtocolAdapter(proxy).unpause();
        UUPSUpgradeable(proxy).upgradeToAndCall(implementation, implementationDeployScript.INITIALIZATION_DATA());
        vm.stopBroadcast();
    }

    /// @notice Checks a migrated proxy against the stopped v1 protocol adapter, reading both from the chain. Run it
    /// after `run` has broadcast, because `run` itself only ever sees the simulated state.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter.
    /// @param proxy The migrated v2 protocol adapter proxy.
    function verify(address protocolAdapterV1, address proxy) public view {
        _check({protocolAdapterV1: protocolAdapterV1, proxy: proxy});
    }

    /// @notice Copies v1's commitment tree into the proxy. The tree's sides come from v1's storage, since v1 exposes
    /// no getter for them; the slot is checked against v1's own getters first, and the proxy rejects sides that do
    /// not reproduce the root.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter.
    /// @param proxy The v2 protocol adapter proxy.
    function _seedCommitmentTree(address protocolAdapterV1, address proxy) internal {
        uint256 leafCount = ICommitmentTree(protocolAdapterV1).commitmentCount();
        uint8 treeDepth = ICommitmentTree(protocolAdapterV1).commitmentTreeDepth();

        _checkSlot({target: protocolAdapterV1, slot: _V1_NEXT_LEAF_INDEX_SLOT, expected: leafCount});
        _checkSlot({target: protocolAdapterV1, slot: _V1_SIDES_SLOT, expected: treeDepth});

        bytes32[] memory sides = new bytes32[](treeDepth);
        uint256 firstElementSlot = uint256(keccak256(abi.encode(_V1_SIDES_SLOT)));
        for (uint256 i = 0; i < treeDepth; ++i) {
            sides[i] = vm.load({target: protocolAdapterV1, slot: bytes32(firstElementSlot + i)});
        }

        vm.broadcast();
        TransitionalProtocolAdapter(proxy).seedCommitmentTree(sides);
    }

    /// @notice Copies the nullifiers into the proxy, `NULLIFIERS_PER_BATCH` per transaction. The proxy reads each
    /// batch from the v1 protocol adapter itself and resumes at the index it has reached, so a batch can neither be
    /// skipped nor repeated.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter.
    /// @param proxy The v2 protocol adapter proxy.
    function _seedNullifierSet(address protocolAdapterV1, address proxy) internal {
        uint256 total = INullifierSet(protocolAdapterV1).nullifierCount();

        for (uint256 seeded = INullifierSet(proxy).nullifierCount(); seeded < total; seeded += NULLIFIERS_PER_BATCH) {
            uint256 size = Math.min(NULLIFIERS_PER_BATCH, total - seeded);

            vm.broadcast();
            TransitionalProtocolAdapter(proxy).seedNullifierSet(size);
        }
    }

    /// @notice Checks the proxy against the stopped v1 protocol adapter: the latest root, the commitment count, the
    /// nullifier count, and every nullifier. Reverts on the first difference.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter.
    /// @param proxy The v2 protocol adapter proxy.
    function _check(address protocolAdapterV1, address proxy) internal view {
        bytes32 expectedRoot = ICommitmentTree(protocolAdapterV1).latestCommitmentTreeRoot();
        bytes32 actualRoot = ICommitmentTree(proxy).latestCommitmentTreeRoot();
        require(actualRoot == expectedRoot, CommitmentTreeRootMismatch({expected: expectedRoot, actual: actualRoot}));

        uint256 expectedCount = ICommitmentTree(protocolAdapterV1).commitmentCount();
        uint256 actualCount = ICommitmentTree(proxy).commitmentCount();
        require(actualCount == expectedCount, CommitmentCountMismatch({expected: expectedCount, actual: actualCount}));

        uint256 expectedNullifiers = INullifierSet(protocolAdapterV1).nullifierCount();
        uint256 actualNullifiers = INullifierSet(proxy).nullifierCount();
        require(
            actualNullifiers == expectedNullifiers,
            NullifierCountMismatch({expected: expectedNullifiers, actual: actualNullifiers})
        );

        for (uint256 i = 0; i < expectedNullifiers; ++i) {
            bytes32 nullifier = INullifierSet(protocolAdapterV1).nullifierAtIndex(i);
            require(INullifierSet(proxy).isNullifierContained(nullifier), MissingNullifier(nullifier));
        }
    }

    /// @notice Checks that a v1 storage slot holds what its public getter reports, which pins the layout this script
    /// reads the tree sides from.
    /// @param target The v1 protocol adapter.
    /// @param slot The storage slot to read.
    /// @param expected The value the matching getter reports.
    function _checkSlot(address target, uint256 slot, uint256 expected) internal view {
        uint256 actual = uint256(vm.load({target: target, slot: bytes32(slot)}));
        require(actual == expected, UnexpectedStorageLayout({slot: slot, expected: expected, actual: actual}));
    }
}
