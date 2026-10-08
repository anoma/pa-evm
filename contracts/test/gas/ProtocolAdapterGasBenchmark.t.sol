// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.4/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";
import {ILogicRefDenylist} from "../../src/interfaces/ILogicRefDenylist.sol";
import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {TxGen} from "../libs/TxGen.sol";

contract ProtocolAdapterGasBenchmark is Test {
    using TxGen for Vm;

    ProtocolAdapter internal _pa;
    RiscZeroMockVerifier internal _verifier;

    function setUp() public {
        (RiscZeroVerifierRouter router,, RiscZeroMockVerifier verifier) = new DeployRiscZeroContractsMock().run();
        _verifier = verifier;
        _pa = ProtocolAdapter(
            address(
                new ERC1967Proxy(
                    address(new ProtocolAdapter(address(router), verifier.SELECTOR())),
                    abi.encodeCall(ProtocolAdapter.initialize, (address(this)))
                )
            )
        );
    }

    function test_execute_populated_two_actions() public {
        TxGen.ActionConfig[] memory configs =
            TxGen.generateActionConfigs({actionCount: 2, consumedCount: 1, createdCount: 1});
        (IProtocolAdapter.Transaction memory txn, bytes32 nonce) =
            vm.transaction({mockVerifier: _verifier, nonce: 0, configs: configs});
        _pa.execute(txn);
        (txn,) = vm.transaction({mockVerifier: _verifier, nonce: nonce, configs: configs});
        _pa.execute(txn);
        vm.snapshotGasLastFrame("OperationGas", "execute_populated_two_actions");
        assertTrue(_pa.isNullifierContained(txn.actions[0].consumed[0].nullifier));
        assertTrue(_pa.isNullifierContained(txn.actions[1].consumed[0].nullifier));
    }

    function test_latest_root_populated() public {
        TxGen.ActionConfig[] memory configs =
            TxGen.generateActionConfigs({actionCount: 2, consumedCount: 1, createdCount: 1});
        (IProtocolAdapter.Transaction memory txn,) =
            vm.transaction({mockVerifier: _verifier, nonce: 0, configs: configs});
        _pa.execute(txn);

        bytes32 root = _pa.latestCommitmentTreeRoot();
        vm.snapshotGasLastFrame("OperationGas", "latest_root_populated");
        assertTrue(_pa.isCommitmentTreeRootContained(root));
    }

    function test_deny_both_empty() public {
        bytes32 logicRef = bytes32(uint256(123));
        ILogicRefDenylist.DeniedLogicRef[] memory denials = new ILogicRefDenylist.DeniedLogicRef[](2);
        denials[0] = ILogicRefDenylist.DeniedLogicRef(logicRef, false);
        denials[1] = ILogicRefDenylist.DeniedLogicRef(logicRef, true);
        _pa.denyLogicRefs(denials);
        vm.snapshotGasLastFrame("OperationGas", "deny_both_empty");
        assertTrue(_pa.isLogicRefDenied(logicRef, false));
        assertTrue(_pa.isLogicRefDenied(logicRef, true));
    }
}
