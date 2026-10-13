// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.4/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {KindTableRoot} from "../src/state/KindTableRoot.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterKindTableRootTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _STORED_ROOT = bytes32(uint256(42));
    bytes32 internal constant _REPLACED_ROOT = bytes32(uint256(_STORED_ROOT) - 1);

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

    function testFuzz_setKindTableRoot_reverts_for_non_owners(address caller) public {
        vm.assume(caller != _OWNER);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, caller));
        _mockPa.setKindTableRoot(bytes32(uint256(1)));
    }

    function test_execute_executes_a_transaction_proven_against_the_stored_kind_table() public {
        _setKindTableRoot(_STORED_ROOT);

        IProtocolAdapter.Transaction memory txn = _transaction(_STORED_ROOT);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function test_execute_executes_a_transaction_proven_against_the_empty_kind_table() public {
        assertNotEq(_STORED_ROOT, _mockPa.EMPTY_KIND_TABLE_ROOT(), "the stored kind table must not be empty");
        _setKindTableRoot(_STORED_ROOT);

        IProtocolAdapter.Transaction memory txn = _transaction(_mockPa.EMPTY_KIND_TABLE_ROOT());

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function test_execute_reverts_for_a_transaction_proven_against_a_replaced_kind_table() public {
        assertNotEq(_REPLACED_ROOT, _mockPa.EMPTY_KIND_TABLE_ROOT(), "the replaced kind table must not be empty");
        _setKindTableRoot(_REPLACED_ROOT);
        IProtocolAdapter.Transaction memory txn = _transaction(_REPLACED_ROOT);

        _setKindTableRoot(_STORED_ROOT);

        vm.expectRevert(
            abi.encodeWithSelector(KindTableRoot.UnacceptedKindTableRoot.selector, _REPLACED_ROOT), address(_mockPa)
        );
        _mockPa.execute(txn);
    }

    function _setKindTableRoot(bytes32 kindTableRoot) internal {
        vm.prank(_OWNER);
        _mockPa.setKindTableRoot(kindTableRoot);
    }

    function _transaction(bytes32 kindTableRoot) internal returns (IProtocolAdapter.Transaction memory txn) {
        (txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        txn = TxGen.transactionAggregation({mockVerifier: _mockVerifier, txn: txn, kindTableRoot: kindTableRoot});
    }
}
