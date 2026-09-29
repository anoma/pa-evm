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
import {LogicRefDenylist} from "../src/state/LogicRefDenylist.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterDenylistTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _DENIED_LOGIC_REF = bytes32(uint256(0xdead));

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

    function testFuzz_denyLogicRef_reverts_for_non_owners(address caller) public {
        vm.assume(caller != _OWNER);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, caller));
        _mockPa.denyLogicRef(_DENIED_LOGIC_REF);
    }

    function test_execute_reverts_if_a_consumed_resource_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].consumed[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(abi.encodeWithSelector(LogicRefDenylist.DeniedLogicRef.selector, _DENIED_LOGIC_REF));
        _mockPa.execute(txn);
    }

    function test_execute_reverts_if_a_created_resource_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].created[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(abi.encodeWithSelector(LogicRefDenylist.DeniedLogicRef.selector, _DENIED_LOGIC_REF));
        _mockPa.execute(txn);
    }

    function test_execute_reverts_if_a_later_action_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 3});
        txn.actions[2].created[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(abi.encodeWithSelector(LogicRefDenylist.DeniedLogicRef.selector, _DENIED_LOGIC_REF));
        _mockPa.execute(txn);
    }

    function test_simulateExecute_reverts_if_a_resource_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].consumed[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(abi.encodeWithSelector(LogicRefDenylist.DeniedLogicRef.selector, _DENIED_LOGIC_REF));
        _mockPa.simulateExecute({transaction: txn, skipRiscZeroProofVerification: true});
    }

    function test_execute_settles_resources_of_logic_refs_that_are_not_denied() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 2});

        _deny(_DENIED_LOGIC_REF);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function _deny(bytes32 logicRef) internal {
        vm.prank(_OWNER);
        _mockPa.denyLogicRef(logicRef);
    }

    /// @dev Checks that the transaction settles before the logic ref is denied, so that a later revert is the denylist's.
    function _expectSettlement(IProtocolAdapter.Transaction memory txn) internal {
        vm.expectPartialRevert(ProtocolAdapter.Simulated.selector, address(_mockPa));
        _mockPa.simulateExecute({transaction: txn, skipRiscZeroProofVerification: false});
    }

    function _transaction(uint256 actionCount) internal returns (IProtocolAdapter.Transaction memory txn) {
        (txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: bytes32(uint256(1)),
            configs: TxGen.generateActionConfigs({actionCount: actionCount, consumedCount: 1, createdCount: 1})
        });
    }

    /// @dev Proves the aggregation again after a logic ref changed, since the journal covers every logic ref.
    function _reaggregate(IProtocolAdapter.Transaction memory txn)
        internal
        view
        returns (IProtocolAdapter.Transaction memory aggregatedTxn)
    {
        aggregatedTxn = TxGen.transactionAggregation({
            mockVerifier: _mockVerifier, txn: txn, kindTableCommitment: TxGen.emptyKindTableCommitment()
        });
    }
}
