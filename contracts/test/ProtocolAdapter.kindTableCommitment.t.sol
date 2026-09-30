// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.2/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.16.2/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {KindTableCommitment} from "../src/state/KindTableCommitment.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterKindTableCommitmentTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _STORED_COMMITMENT = bytes32(uint256(42));
    bytes32 internal constant _REPLACED_COMMITMENT = bytes32(uint256(_STORED_COMMITMENT) - 1);

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

    function testFuzz_setKindTableCommitment_reverts_for_non_owners(address caller) public {
        vm.assume(caller != _OWNER);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, caller));
        _mockPa.setKindTableCommitment(bytes32(uint256(1)));
    }

    function test_execute_executes_a_transaction_proven_against_the_stored_kind_table() public {
        _setKindTableCommitment(_STORED_COMMITMENT);

        IProtocolAdapter.Transaction memory txn = _transaction(_STORED_COMMITMENT);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function test_execute_executes_a_transaction_proven_against_the_empty_kind_table() public {
        assertNotEq(_STORED_COMMITMENT, TxGen.emptyKindTableCommitment(), "the stored kind table must not be empty");
        _setKindTableCommitment(_STORED_COMMITMENT);

        IProtocolAdapter.Transaction memory txn = _transaction(TxGen.emptyKindTableCommitment());

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function test_execute_reverts_for_a_transaction_proven_against_a_replaced_kind_table() public {
        assertNotEq(_REPLACED_COMMITMENT, TxGen.emptyKindTableCommitment(), "the replaced kind table must not be empty");
        _setKindTableCommitment(_REPLACED_COMMITMENT);
        IProtocolAdapter.Transaction memory txn = _transaction(_REPLACED_COMMITMENT);

        _setKindTableCommitment(_STORED_COMMITMENT);

        vm.expectRevert(
            abi.encodeWithSelector(KindTableCommitment.UnacceptedKindTableCommitment.selector, _REPLACED_COMMITMENT),
            address(_mockPa)
        );
        _mockPa.execute(txn);
    }

    function _setKindTableCommitment(bytes32 kindTableCommitment) internal {
        vm.prank(_OWNER);
        _mockPa.setKindTableCommitment(kindTableCommitment);
    }

    function _transaction(bytes32 kindTableCommitment) internal returns (IProtocolAdapter.Transaction memory txn) {
        (txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        txn = TxGen.transactionAggregation({
            mockVerifier: _mockVerifier, txn: txn, kindTableCommitment: kindTableCommitment
        });
    }
}
