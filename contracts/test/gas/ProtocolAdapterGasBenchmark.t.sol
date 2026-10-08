// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.4/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";
import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {TxGen} from "../libs/TxGen.sol";

interface IRestrictionGasApi {
    enum Policy {
        Unrestricted,
        CreationDenied,
        ConsumptionDenied,
        FullyDenied
    }

    struct PolicyUpdate {
        bytes32 logicRef;
        Policy policy;
    }

    struct Denial {
        bytes32 logicRef;
        bool consumed;
    }

    function setLogicRefPolicies(PolicyUpdate[] calldata updates) external;
    function denyLogicRefs(Denial[] calldata denials) external;
    function logicRefPolicy(bytes32 logicRef) external view returns (Policy policy);
    function isLogicRefDenied(bytes32 logicRef, bool consumed) external view returns (bool denied);
}

/// @dev The same fixture is copied into the base checkout. ABI-local types support both restriction APIs.
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
        IRestrictionGasApi registry = IRestrictionGasApi(address(_pa));
        bool policyApi;
        try registry.logicRefPolicy(logicRef) returns (IRestrictionGasApi.Policy policy) {
            assertEq(uint256(policy), uint256(IRestrictionGasApi.Policy.Unrestricted));
            policyApi = true;
        } catch {
            assertFalse(registry.isLogicRefDenied(logicRef, false));
            assertFalse(registry.isLogicRefDenied(logicRef, true));
        }
        if (policyApi) {
            IRestrictionGasApi.PolicyUpdate[] memory updates = new IRestrictionGasApi.PolicyUpdate[](1);
            updates[0] = IRestrictionGasApi.PolicyUpdate(logicRef, IRestrictionGasApi.Policy.FullyDenied);
            registry.setLogicRefPolicies(updates);
        } else {
            IRestrictionGasApi.Denial[] memory denials = new IRestrictionGasApi.Denial[](2);
            denials[0] = IRestrictionGasApi.Denial(logicRef, false);
            denials[1] = IRestrictionGasApi.Denial(logicRef, true);
            registry.denyLogicRefs(denials);
        }
        vm.snapshotGasLastFrame("OperationGas", "deny_both_empty");
        if (policyApi) {
            assertEq(uint256(registry.logicRefPolicy(logicRef)), uint256(IRestrictionGasApi.Policy.FullyDenied));
        } else {
            assertTrue(registry.isLogicRefDenied(logicRef, false));
            assertTrue(registry.isLogicRefDenied(logicRef, true));
        }
    }
}
