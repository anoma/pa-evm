// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.4/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {VerificationFailed} from "risc0-risc0-ethereum-3.0.1/contracts/src/IRiscZeroVerifier.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {ILogicRefDenylist} from "../src/interfaces/ILogicRefDenylist.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {LogicRefDenylist} from "../src/state/LogicRefDenylist.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterDenylistTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _DENIED_LOGIC_REF = bytes32(uint256(0xdead));
    bytes32 internal constant _DEPRECATED_LOGIC_REF = bytes32(uint256(0xdeca));

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

    function testFuzz_denyLogicRefs_reverts_for_non_owners(address caller) public {
        vm.assume(caller != _OWNER);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, caller));
        _mockPa.denyLogicRefs(_denial(_DENIED_LOGIC_REF));
    }

    function test_denyLogicRefs_denies_one_logic_ref_and_deprecates_another_in_one_call() public {
        ILogicRefDenylist.DeniedLogicRef[] memory logicRefs = new ILogicRefDenylist.DeniedLogicRef[](3);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DENIED_LOGIC_REF, consumed: true});
        logicRefs[1] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DENIED_LOGIC_REF, consumed: false});
        logicRefs[2] = ILogicRefDenylist.DeniedLogicRef({logicRef: _DEPRECATED_LOGIC_REF, consumed: false});

        vm.prank(_OWNER);
        _mockPa.denyLogicRefs(logicRefs);

        assertTrue(
            _mockPa.isLogicRefDenied(_DENIED_LOGIC_REF, true),
            "the denied logic ref should be on the denylist for consumed resources"
        );
        assertTrue(
            _mockPa.isLogicRefDenied(_DENIED_LOGIC_REF, false),
            "the denied logic ref should be on the denylist for created resources"
        );
        assertFalse(
            _mockPa.isLogicRefDenied(_DEPRECATED_LOGIC_REF, true),
            "the deprecated logic ref should not be on the denylist for consumed resources"
        );
        assertTrue(
            _mockPa.isLogicRefDenied(_DEPRECATED_LOGIC_REF, false),
            "the deprecated logic ref should be on the denylist for created resources"
        );
    }

    function test_execute_reverts_if_a_created_resource_carries_a_deprecated_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].created[0].logicRef = _DEPRECATED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deprecate(_DEPRECATED_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.ResourceWithDeniedLogicRef.selector, _DEPRECATED_LOGIC_REF, false)
        );
        _mockPa.execute(txn);
    }

    function test_execute_consumes_a_resource_that_carries_a_deprecated_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].consumed[0].logicRef = _DEPRECATED_LOGIC_REF;
        txn = _reaggregate(txn);

        _deprecate(_DEPRECATED_LOGIC_REF);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function test_execute_rolls_back_if_a_later_action_creates_a_deprecated_resource() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 3});
        txn.actions[0].consumed[0].logicRef = _DEPRECATED_LOGIC_REF;
        txn.actions[2].created[0].logicRef = _DEPRECATED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deprecate(_DEPRECATED_LOGIC_REF);

        bytes32 root = _mockPa.latestCommitmentTreeRoot();
        uint256 nullifiers = _mockPa.nullifierCount();
        uint256 commitments = _mockPa.commitmentCount();

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.ResourceWithDeniedLogicRef.selector, _DEPRECATED_LOGIC_REF, false)
        );
        _mockPa.execute(txn);

        assertEq(_mockPa.latestCommitmentTreeRoot(), root);
        assertEq(_mockPa.nullifierCount(), nullifiers);
        assertEq(_mockPa.commitmentCount(), commitments);
    }

    function test_execute_reverts_if_a_deprecated_creation_is_relabelled_without_a_new_proof() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].created[0].logicRef = _DEPRECATED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deprecate(_DEPRECATED_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.ResourceWithDeniedLogicRef.selector, _DEPRECATED_LOGIC_REF, false)
        );
        _mockPa.execute(txn);

        txn.actions[0].created[0].logicRef = txn.actions[0].consumed[0].logicRef;
        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
        _mockPa.execute(txn);
        txn.actions[0].created[0].logicRef = _DEPRECATED_LOGIC_REF;

        IProtocolAdapter.Consumed memory consumed = txn.actions[0].consumed[0];
        IProtocolAdapter.Created memory created = txn.actions[0].created[0];
        txn.actions[0].consumed[0] = IProtocolAdapter.Consumed({
            nullifier: created.commitment,
            logicRef: created.logicRef,
            commitmentTreeRoot: consumed.commitmentTreeRoot,
            appData: created.appData
        });
        txn.actions[0].created[0] = IProtocolAdapter.Created({
            commitment: consumed.nullifier, logicRef: consumed.logicRef, appData: consumed.appData
        });

        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
        _mockPa.execute(txn);
    }

    function test_execute_reverts_if_a_consumed_resource_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].consumed[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.ResourceWithDeniedLogicRef.selector, _DENIED_LOGIC_REF, true)
        );
        _mockPa.execute(txn);
    }

    function test_execute_reverts_if_a_later_action_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 3});
        txn.actions[2].created[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _expectSettlement(txn);
        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.ResourceWithDeniedLogicRef.selector, _DENIED_LOGIC_REF, false)
        );
        _mockPa.execute(txn);
    }

    function test_simulateExecute_reverts_if_a_resource_carries_a_denied_logic_ref() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 1});
        txn.actions[0].consumed[0].logicRef = _DENIED_LOGIC_REF;
        txn = _reaggregate(txn);

        _deny(_DENIED_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.ResourceWithDeniedLogicRef.selector, _DENIED_LOGIC_REF, true)
        );
        _mockPa.simulateExecute({transaction: txn, skipRiscZeroProofVerification: true});
    }

    function test_execute_settles_resources_of_logic_refs_that_are_not_denied() public {
        IProtocolAdapter.Transaction memory txn = _transaction({actionCount: 2});

        _deny(_DENIED_LOGIC_REF);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    /// @dev Adds the logic ref to the denylist for created resources only.
    function _deprecate(bytes32 logicRef) internal {
        vm.prank(_OWNER);
        _mockPa.denyLogicRefs(_deprecation(logicRef));
    }

    /// @dev Adds the logic ref to both denylists.
    function _deny(bytes32 logicRef) internal {
        vm.prank(_OWNER);
        _mockPa.denyLogicRefs(_denial(logicRef));
    }

    /// @dev Checks that the transaction settles before a denylist changes, so that a later revert is the denylist's.
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
            mockVerifier: _mockVerifier, txn: txn, kindTableCommitment: _mockPa.EMPTY_KIND_TABLE_COMMITMENT()
        });
    }

    function _deprecation(bytes32 logicRef)
        internal
        pure
        returns (ILogicRefDenylist.DeniedLogicRef[] memory logicRefs)
    {
        logicRefs = new ILogicRefDenylist.DeniedLogicRef[](1);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: logicRef, consumed: false});
    }

    function _denial(bytes32 logicRef) internal pure returns (ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) {
        logicRefs = new ILogicRefDenylist.DeniedLogicRef[](2);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: logicRef, consumed: true});
        logicRefs[1] = ILogicRefDenylist.DeniedLogicRef({logicRef: logicRef, consumed: false});
    }
}
