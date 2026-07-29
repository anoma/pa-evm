// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployRiscZeroContracts} from "anoma-risc0-deployments-1.2.0/script/DeployRiscZeroContracts.s.sol";
import {Test} from "forge-std-1.16.1/src/Test.sol";
import {RiscZeroGroth16Verifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/groth16/RiscZeroGroth16Verifier.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {Transaction} from "../src/Types.sol";

uint256 constant UPPER_EMPTY_TX_GAS_COST_BOUND = 7256;

contract Benchmark is Test {
    RiscZeroVerifierRouter internal _router;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    ProtocolAdapter internal _pa;

    Transaction internal _txnEmpty;

    function setUp() public {
        RiscZeroGroth16Verifier verifier;

        (_router, _emergencyStop, verifier) =
            new DeployRiscZeroContracts().run({admin: msg.sender, guardian: msg.sender});

        _pa = new ProtocolAdapter(_router, verifier.SELECTOR(), msg.sender);
    }

    function test_empty_transaction_gas_cost_is_fixed() public {
        uint256 gasWithoutProofs = _executionGasCost({transaction: _txnEmpty, skipRiscZeroProofVerification: true});
        uint256 gasWithProofs = _executionGasCost({transaction: _txnEmpty, skipRiscZeroProofVerification: false});

        assertEq(gasWithProofs, gasWithoutProofs, "empty tx gas cost should be the same with and without proofs");
        assertLe(gasWithoutProofs, UPPER_EMPTY_TX_GAS_COST_BOUND, "empty tx gas cost should be within upper bound");
    }

    function _executionGasCost(Transaction memory transaction, bool skipRiscZeroProofVerification)
        internal
        returns (uint256 gasUsed)
    {
        (bool success, bytes memory data) =
            address(_pa).call(abi.encodeCall(_pa.simulateExecute, (transaction, skipRiscZeroProofVerification))); // solhint-disable-line avoid-low-level-calls
        assertFalse(success, "call should revert");

        bytes4 selector;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            // Load first 32 bytes
            let word := mload(add(data, 32))
            // Selector is high-order 4 bytes
            selector := shl(224, shr(224, word))
            // Gas used is next 32 bytes (starting at offset 4)
            gasUsed := mload(add(data, 36))
        }

        assertEq(selector, ProtocolAdapter.Simulated.selector, "selector should match Simulated");
    }
}
