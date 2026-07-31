// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.6.1/access/Ownable.sol";
import {Pausable} from "@openzeppelin-contracts-5.6.1/utils/Pausable.sol";

import {DeployRiscZeroContracts} from "anoma-risc0-deployments-1.2.0/script/DeployRiscZeroContracts.s.sol";
import {
    DeployRiscZeroContractsMock,
    MOCK_VERIFIER_SELECTOR
} from "anoma-risc0-deployments-1.2.0/test/script/DeployRiscZeroContractsMock.s.sol";

import {Test, Vm} from "forge-std-1.16.1/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.1/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.1/src/Upgrades.sol";
import {RiscZeroGroth16Verifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/groth16/RiscZeroGroth16Verifier.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";
import {SemVerLib} from "solady-0.1.26/src/utils/SemVerLib.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {Transaction} from "../src/Types.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterTest is Test {
    using SemVerLib for bytes32;
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    address internal constant _UNAUTHORIZED_CALLER = address(uint160(2));

    RiscZeroVerifierRouter internal _router;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    RiscZeroVerifierEmergencyStop internal _mockEmergencyStop;
    RiscZeroGroth16Verifier internal _verifier;
    RiscZeroMockVerifier internal _mockVerifier;

    ProtocolAdapter internal _pa;
    bytes4 internal _verifierSelector;

    Transaction internal _emptyTx;

    function setUp() public {
        (_router, _emergencyStop, _verifier) =
            new DeployRiscZeroContracts().run({admin: msg.sender, guardian: msg.sender});

        (, _mockEmergencyStop, _mockVerifier) = new DeployRiscZeroContractsMock().run();
        _router.addVerifier({selector: _mockVerifier.SELECTOR(), verifier: _mockEmergencyStop});

        _verifierSelector = _verifier.SELECTOR();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifierSelector);

        _pa = ProtocolAdapter(
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );
    }

    function test_execute_reverts_if_the_pa_has_been_stopped() public {
        vm.prank(_pa.owner());
        _pa.emergencyStop();

        vm.expectRevert(Pausable.EnforcedPause.selector, address(_pa));
        _pa.execute(_emptyTx);
    }

    function test_execute_reverts_if_the_aggregation_proof_has_been_generated_with_another_unstopped_verifier() public {
        (Transaction memory txnWithMockProof,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, complianceUnitCount: 1})
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapter.RiscZeroVerifierSelectorMismatch.selector, _verifierSelector, MOCK_VERIFIER_SELECTOR
            ),
            address(_pa)
        );
        _pa.execute(txnWithMockProof);
    }

    function test_execute_reverts_on_the_empty_transaction() public {
        vm.expectRevert(ProtocolAdapter.EmptyTransactionNotAllowed.selector, address(_pa));
        _pa.execute(_emptyTx);
    }

    function test_simulateExecute_reverts_on_the_empty_transaction() public {
        vm.expectRevert(ProtocolAdapter.EmptyTransactionNotAllowed.selector, address(_pa));
        _pa.simulateExecute({transaction: _emptyTx, skipRiscZeroProofVerification: true});
    }

    function test_emergencyStop_reverts_if_the_caller_is_not_the_owner() public {
        vm.prank(_UNAUTHORIZED_CALLER);
        vm.expectRevert(
            abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, _UNAUTHORIZED_CALLER), address(_pa)
        );
        _pa.emergencyStop();
    }

    function test_emergencyStop_pauses_the_protocol_adapter() public {
        assertEq(_pa.paused(), false, "PA should not be paused initially");

        vm.prank(_OWNER);
        _pa.emergencyStop();

        assertEq(_pa.paused(), true, "PA should be paused after emergency stop");
    }

    function test_emergencyStop_emits_the_Paused_event() public {
        vm.prank(_OWNER);

        vm.expectEmit(address(_pa));
        emit Pausable.Paused(_OWNER);
        _pa.emergencyStop();
    }

    function test_getRiscZeroVerifierRouter_returns_the_router_address() public view {
        assertEq(_pa.getRiscZeroVerifierRouter(), address(_router), "router address should match");
    }

    function test_getRiscZeroVerifierSelector_returns_the_selector() public view {
        assertEq(_pa.getRiscZeroVerifierSelector(), _verifierSelector, "verifier selector should match");
    }

    function test_check_that_the_current_version_is_a_not_a_major_release() public view {
        int256 lt = -1;
        //int256 eq = 0;
        int256 gt = 1;

        assertEq(SemVerLib.cmp(_pa.getVersion(), "1.0.0"), gt, "version should be greater than 1.0.0");
        assertEq(SemVerLib.cmp(_pa.getVersion(), "2.0.0"), lt, "version should be less than 2.0.0");
    }
}
