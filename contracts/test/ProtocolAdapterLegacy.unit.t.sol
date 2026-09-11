// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.1/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm, console} from "forge-std-1.16.2/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {NullifierSet} from "../src/state/NullifierSet.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterLegacyTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));

    RiscZeroVerifierRouter internal _legacyRouter;
    RiscZeroVerifierEmergencyStop internal _legacyEmergencyStop;
    RiscZeroMockVerifier internal _legacyVerifier;
    ProtocolAdapter internal _legacyPa;
    bytes32 internal _legacyRoot;
    bytes32 internal _legacyNullifier;

    RiscZeroVerifierRouter internal _v2Router;
    RiscZeroMockVerifier internal _v2Verifier;
    ProtocolAdapter internal _v2Pa;

    function setUp() public {
        (_legacyRouter, _legacyEmergencyStop, _legacyVerifier) = new DeployRiscZeroContractsMock().run();
        _legacyPa = _deployProtocolAdapter(_legacyRouter, _legacyVerifier);

        (IProtocolAdapter.Transaction memory seedTransaction,) = vm.transaction({
            mockVerifier: _legacyVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        _legacyPa.execute(seedTransaction);
        _legacyRoot = _legacyPa.latestCommitmentTreeRoot();
        _legacyNullifier = seedTransaction.actions[0].consumed[0].nullifier;

        vm.prank(_legacyEmergencyStop.owner());
        _legacyEmergencyStop.estop();

        (_v2Router,, _v2Verifier) = new DeployRiscZeroContractsMock().run();
        _v2Pa = _deployProtocolAdapter(_v2Router, _v2Verifier);
    }

    function test_configureLegacyProtocolAdapter_snapshots_the_frozen_adapter_and_root() public {
        vm.expectEmit(address(_v2Pa));
        emit IProtocolAdapter.LegacyProtocolAdapterConfigured({
            legacyProtocolAdapter: address(_legacyPa), legacyCommitmentTreeRoot: _legacyRoot
        });

        _configureLegacy();

        assertEq(_v2Pa.legacyProtocolAdapter(), address(_legacyPa), "legacy adapter should match");
        assertEq(_v2Pa.legacyCommitmentTreeRoot(), _legacyRoot, "legacy root should match");
    }

    function test_configureLegacyProtocolAdapter_reverts_for_an_unfrozen_adapter() public {
        ProtocolAdapter activeAdapter = _deployProtocolAdapter(_v2Router, _v2Verifier);

        vm.prank(_OWNER);
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolAdapter.LegacyProtocolAdapterNotFrozen.selector, address(activeAdapter))
        );
        _v2Pa.configureLegacyProtocolAdapter(address(activeAdapter));
    }

    function test_configureLegacyProtocolAdapter_reverts_after_v2_state_changes() public {
        _v2Pa.execute(_transactionWithRoot(_v2Pa.latestCommitmentTreeRoot(), bytes32(uint256(100))));

        vm.prank(_OWNER);
        vm.expectRevert(ProtocolAdapter.ProtocolAdapterStateNotEmpty.selector);
        _v2Pa.configureLegacyProtocolAdapter(address(_legacyPa));
    }

    function test_execute_legacy_input_checks_v1_and_records_the_nullifier_in_v2() public {
        _configureLegacy();
        IProtocolAdapter.Transaction memory transaction = _transactionWithRoot(_legacyRoot, bytes32(uint256(100)));
        bytes32 nullifier = transaction.actions[0].consumed[0].nullifier;

        _v2Pa.execute(transaction);

        assertFalse(_legacyPa.isNullifierContained(nullifier), "legacy nullifier should not be written to v1");
        assertTrue(_v2Pa.isNullifierContained(nullifier), "legacy nullifier should be written to v2");
    }

    function test_execute_legacy_input_reverts_when_v1_already_contains_its_nullifier() public {
        _configureLegacy();
        IProtocolAdapter.Transaction memory transaction = _transactionWithRoot(_legacyRoot, bytes32(uint256(100)));
        transaction.actions[0].consumed[0].nullifier = _legacyNullifier;
        transaction = _aggregate(transaction);

        vm.expectRevert(abi.encodeWithSelector(ProtocolAdapter.LegacyNullifierAlreadySpent.selector, _legacyNullifier));
        _v2Pa.execute(transaction);
    }

    function test_execute_legacy_input_reverts_when_v2_already_contains_its_nullifier() public {
        _configureLegacy();
        IProtocolAdapter.Transaction memory transaction = _transactionWithRoot(_legacyRoot, bytes32(uint256(100)));

        _v2Pa.execute(transaction);

        vm.expectRevert(
            abi.encodeWithSelector(
                NullifierSet.PreExistingNullifier.selector, transaction.actions[0].consumed[0].nullifier
            )
        );
        _v2Pa.execute(transaction);
    }

    function test_execute_native_v2_input_needs_only_a_v2_root() public {
        _configureLegacy();
        IProtocolAdapter.Transaction memory transaction =
            _transactionWithRoot(_v2Pa.latestCommitmentTreeRoot(), bytes32(uint256(100)));

        _v2Pa.execute(transaction);

        assertTrue(
            _v2Pa.isNullifierContained(transaction.actions[0].consumed[0].nullifier),
            "native nullifier should be written to v2"
        );
    }

    function test_gas_execute_legacy_input() public {
        _configureLegacy();
        IProtocolAdapter.Transaction memory transaction = _transactionWithRoot(_legacyRoot, bytes32(uint256(100)));

        uint256 gasBefore = gasleft();
        _v2Pa.execute(transaction);
        console.log("legacy input execute gas:", gasBefore - gasleft());
    }

    function test_gas_execute_native_v2_input() public {
        _configureLegacy();
        IProtocolAdapter.Transaction memory transaction =
            _transactionWithRoot(_v2Pa.latestCommitmentTreeRoot(), bytes32(uint256(100)));

        uint256 gasBefore = gasleft();
        _v2Pa.execute(transaction);
        console.log("native v2 input execute gas:", gasBefore - gasleft());
    }

    function _configureLegacy() internal {
        vm.prank(_OWNER);
        _v2Pa.configureLegacyProtocolAdapter(address(_legacyPa));
    }

    function _deployProtocolAdapter(RiscZeroVerifierRouter router, RiscZeroMockVerifier verifier)
        internal
        returns (ProtocolAdapter protocolAdapter)
    {
        Options memory options;
        options.constructorData = abi.encode(router, verifier.SELECTOR());
        protocolAdapter = ProtocolAdapter(
            Upgrades.deployUUPSProxy(
                "ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), options
            )
        );
    }

    function _transactionWithRoot(bytes32 root, bytes32 nonce)
        internal
        returns (IProtocolAdapter.Transaction memory transaction)
    {
        (IProtocolAdapter.Action memory action,) =
            TxGen.createDefaultAction({vm: vm, nonce: nonce, consumedCount: 1, createdCount: 1});
        action.consumed[0].commitmentTreeRoot = root;
        action.actionTreeRoot = TxGen.computeActionTreeRoot(action.consumed, action.created);

        IProtocolAdapter.Action[] memory actions = new IProtocolAdapter.Action[](1);
        actions[0] = action;
        transaction = TxGen.transactionFromActions({vm: vm, actions: actions, resourceCount: 2});
        transaction = _aggregate(transaction);
    }

    function _aggregate(IProtocolAdapter.Transaction memory transaction)
        internal
        view
        returns (IProtocolAdapter.Transaction memory)
    {
        return TxGen.transactionAggregation({
            mockVerifier: _v2Verifier, txn: transaction, kindTableCommitment: _v2Pa.getKindTableCommitment()
        });
    }
}
