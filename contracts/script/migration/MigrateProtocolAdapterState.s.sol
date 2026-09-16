// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin-contracts-5.7.0/utils/math/Math.sol";
import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ICommitmentTree} from "../../src/interfaces/ICommitmentTree.sol";
import {INullifierSet} from "../../src/interfaces/INullifierSet.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {TransitionalProtocolAdapter} from "../../src/TransitionalProtocolAdapter.sol";
import {DeployProtocolAdapterImplementation} from "../DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title MigrateProtocolAdapterState
/// @author Anoma Foundation, 2026
/// @notice A script to copy one chain's state from the stopped v1 protocol adapter into a v2 proxy running
/// `TransitionalProtocolAdapter`. `run` copies the commitment tree and the nullifier set, and the proxy stays paused,
/// so that the ERC20 forwarder balances move before anyone can transact. `FinalizeMigration` then unpauses, upgrades
/// the proxy to the plain `ProtocolAdapter` implementation and, in production, transfers it to the production proxy
/// owner. `verify` checks the migrated proxy against the chain. Every step skips once it is done, so a run that stops
/// early is repeated until it reaches the end.
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

    /// @notice Thrown if the transitional proxy copies its state from another v1 protocol adapter than the given one.
    error ProtocolAdapterV1Mismatch(address expected, address actual);

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

    /// @notice Thrown if the proxy does not run the plain implementation.
    error ImplementationMismatch(address expected, address actual);

    /// @notice Thrown if the proxy is still paused.
    error ProxyPaused(address proxy);

    /// @notice Thrown if the proxy has another owner than the expected one.
    error OwnerMismatch(address expected, address actual);

    /// @notice Copies v1's state into the proxy. The proxy stays paused, so no transaction can use the kind table
    /// before the ERC20 forwarder balances move. Without `--broadcast` the run is simulated locally.
    /// @dev Run it with `--slow`, so that each transaction is confirmed before the next is sent. Without it every
    /// transaction goes out at once, and a transaction that reverts on chain does not stop the ones behind it. Every
    /// step skips once it is done, so a run that stops early can be repeated. Run `FinalizeMigration` once the ERC20
    /// forwarder balances moved.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter to read the state from.
    /// @param proxy The v2 protocol adapter proxy, running `TransitionalProtocolAdapter`.
    function run(address protocolAdapterV1, address proxy) public {
        address implementation = _requireDeployedImplementation(new DeployProtocolAdapterImplementation());
        ProtocolAdapter protocolAdapter = ProtocolAdapter(proxy);

        if (protocolAdapter.getImplementation() != implementation) {
            _requireProtocolAdapterV1({protocolAdapterV1: protocolAdapterV1, proxy: proxy});

            if (protocolAdapter.paused()) {
                _seedCommitmentTree({protocolAdapterV1: protocolAdapterV1, proxy: proxy});
                _seedNullifierSet({protocolAdapterV1: protocolAdapterV1, proxy: proxy});
            }
        }
    }

    /// @notice Checks a migrated proxy against the stopped v1 protocol adapter and against the end state of the
    /// migration, reading both from the chain. Run it after `FinalizeMigration` has broadcast, because
    /// `FinalizeMigration` itself only ever sees the simulated state.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter.
    /// @param proxy The migrated v2 protocol adapter proxy.
    /// @param isProduction Whether the production proxy owner must own the proxy.
    function verify(address protocolAdapterV1, address proxy, bool isProduction) public {
        _check({protocolAdapterV1: protocolAdapterV1, proxy: proxy});
        _checkEndState({proxy: proxy, isProduction: isProduction});
    }

    /// @notice Copies v1's commitment tree into the proxy, unless an earlier run did. The tree's sides come from v1's
    /// storage, since v1 exposes no getter for them; the slot is checked against v1's own getters first, and the proxy
    /// rejects sides that do not reproduce the root.
    /// @param protocolAdapterV1 The stopped v1 protocol adapter.
    /// @param proxy The v2 protocol adapter proxy.
    function _seedCommitmentTree(address protocolAdapterV1, address proxy) internal {
        // The proxy refuses to write a tree twice.
        if (ICommitmentTree(proxy).commitmentCount() != 0) {
            return;
        }

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

    /// @notice Checks what the migration leaves besides the copied state: the plain implementation, the lifted pause
    /// and, in production, the production proxy owner. Reverts on the first difference.
    /// @param proxy The migrated v2 protocol adapter proxy.
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

    /// @notice Returns the plain implementation the proxy ends on, and reverts unless it is deployed.
    /// @param implementationDeployScript The script that predicts the implementation's deterministic address.
    /// @return implementation The deployed plain implementation.
    function _requireDeployedImplementation(DeployProtocolAdapterImplementation implementationDeployScript)
        internal
        view
        returns (address implementation)
    {
        // forge-lint: disable-next-line(unused-return)
        (implementation,) = implementationDeployScript.predict();
        require(
            implementation.code.length != 0,
            DeployProtocolAdapterImplementation.ImplementationNotDeployed(implementation)
        );
    }

    /// @notice Reverts unless the proxy copies its state from the given v1 protocol adapter. Only the transitional
    /// implementation has the getter, so the call also rejects any other proxy.
    /// @param protocolAdapterV1 The v1 protocol adapter the caller names.
    /// @param proxy The v2 protocol adapter proxy.
    function _requireProtocolAdapterV1(address protocolAdapterV1, address proxy) internal view {
        address copiedFrom = TransitionalProtocolAdapter(proxy).getProtocolAdapterV1();
        require(
            copiedFrom == protocolAdapterV1,
            ProtocolAdapterV1Mismatch({expected: protocolAdapterV1, actual: copiedFrom})
        );
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
