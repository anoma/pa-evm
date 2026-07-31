// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.1/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.16.2/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {VerificationFailed} from "risc0-risc0-ethereum-3.0.1/contracts/src/IRiscZeroVerifier.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {Transaction} from "../src/Types.sol";
import {TxGen} from "./libs/TxGen.sol";

contract KindTableCommitmentStorageTest is Test, ProtocolAdapter {
    constructor() ProtocolAdapter(RiscZeroVerifierRouter(address(1)), bytes4(uint32(1))) {}

    function test_storage_slot() public pure {
        assertEq(_PROTOCOL_ADAPTER_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.ProtocolAdapter"));
    }

    function test_empty_kind_table_commitment_is_the_hash_of_empty_bytes() public pure {
        assertEq(_EMPTY_KIND_TABLE_COMMITMENT, sha256(""));
    }
}

contract KindTableCommitmentTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));

    RiscZeroVerifierRouter internal _router;
    RiscZeroMockVerifier internal _mockVerifier;
    ProtocolAdapter internal _mockPa;

    function setUp() public {
        (_router,, _mockVerifier) = new DeployRiscZeroContractsMock().run();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _mockVerifier.SELECTOR());

        _mockPa = ProtocolAdapter(
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );
    }

    function test_initialize_sets_the_empty_kind_table_commitment() public view {
        assertEq(_mockPa.getKindTableCommitment(), sha256(""), "the default should be the empty-table commitment");
    }

    function testFuzz_setKindTableCommitment_reverts_for_non_owners(address caller) public {
        vm.assume(caller != _OWNER);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, caller));
        _mockPa.setKindTableCommitment(bytes32(uint256(1)));
    }

    function test_setKindTableCommitment_reverts_on_the_zero_commitment() public {
        vm.prank(_OWNER);
        vm.expectRevert(ProtocolAdapter.ZeroKindTableCommitmentNotAllowed.selector);
        _mockPa.setKindTableCommitment(bytes32(0));
    }

    function testFuzz_setKindTableCommitment_updates_the_commitment_and_emits_the_event(bytes32 newCommitment) public {
        vm.assume(newCommitment != bytes32(0));

        vm.prank(_OWNER);
        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.KindTableCommitmentUpdated({kindTableCommitment: newCommitment});
        _mockPa.setKindTableCommitment(newCommitment);

        assertEq(_mockPa.getKindTableCommitment(), newCommitment, "the commitment should be updated");
    }

    /// @dev A transaction proven against the previous kind table is unencodable after a rotation: the journal the
    /// protocol adapter reconstructs embeds the stored commitment, so the proven digest no longer matches.
    function test_execute_reverts_for_transactions_proven_against_a_different_kind_table() public {
        (Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });

        vm.prank(_OWNER);
        _mockPa.setKindTableCommitment(bytes32(uint256(42)));

        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
        _mockPa.execute(txn);
    }

    /// @dev After a rotation, transactions aggregated against the new commitment verify.
    function test_execute_accepts_transactions_proven_against_the_rotated_kind_table() public {
        bytes32 newCommitment = bytes32(uint256(42));

        vm.prank(_OWNER);
        _mockPa.setKindTableCommitment(newCommitment);

        (Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        txn = TxGen.transactionAggregation({mockVerifier: _mockVerifier, txn: txn, kindTableCommitment: newCommitment});

        _mockPa.execute(txn);
    }
}
