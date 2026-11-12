// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {Address} from "@openzeppelin-contracts-5.7.0/utils/Address.sol";
import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {ForwarderExample} from "anoma-forwarder-bases-3.1.0/test/examples/ForwarderExample.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-2.0.0/test/script/DeployRiscZeroContractsMock.s.sol";
import {Vm} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.1.0/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.1.0/contracts/src/test/RiscZeroMockVerifier.sol";
import {Enum} from "safe-smart-account-1.5.0/contracts/libraries/Enum.sol";
import {Safe as SafeSmartAccount} from "safe-smart-account-1.5.0/contracts/Safe.sol";

import {IKindTreeRoot} from "../src/interfaces/IKindTreeRoot.sol";
import {ILogicRefDenylist} from "../src/interfaces/ILogicRefDenylist.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {SafeFixture} from "./fixtures/SafeFixture.sol";
import {TxGen} from "./libs/TxGen.sol";

/// @dev A Safe owns the protocol adapter, and any account, also a forwarder, can execute its signed transactions.
contract ProtocolAdapterReentrancyTest is SafeFixture {
    using TxGen for Vm;

    bytes32 internal constant _CARRIER_LOGIC_REF = bytes32(uint256(123));

    RiscZeroVerifierRouter internal _router;
    RiscZeroMockVerifier internal _mockVerifier;
    ProtocolAdapter internal _mockPa;
    SafeSmartAccount internal _safe;
    uint256 internal _signerKey;
    address internal _fwd;

    function setUp() public {
        (_router,, _mockVerifier) = new DeployRiscZeroContractsMock().run();

        address signer;
        (signer, _signerKey) = makeAddrAndKey("signer");
        _safe = SafeSmartAccount(payable(_deploySafe(signer)));

        Options memory opts;
        opts.constructorData = abi.encode(_router, _mockVerifier.SELECTOR());

        _mockPa = ProtocolAdapter(
            Upgrades.deployUUPSProxy(
                "ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (address(_safe))), opts
            )
        );

        _fwd = address(new ForwarderExample({protocolAdapter: address(_mockPa), logicRef: _CARRIER_LOGIC_REF}));
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_transaction() public {
        IProtocolAdapter.Transaction memory innerTxn;

        _expectExecuteToRevertOnForwarderCall({
            target: address(_mockPa), payload: abi.encodeCall(IProtocolAdapter.execute, (innerTxn)), output: ""
        });
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_safe_transaction_that_pauses_the_protocol_adapter()
        public
    {
        _expectExecuteToRevertOnSafeTransaction(_execTransactionCall(abi.encodeCall(IProtocolAdapter.pause, ())));
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_safe_transaction_that_unpauses_the_protocol_adapter()
        public
    {
        _expectExecuteToRevertOnSafeTransaction(_execTransactionCall(abi.encodeCall(IProtocolAdapter.unpause, ())));
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_safe_transaction_that_sets_the_kind_tree_root()
        public
    {
        _expectExecuteToRevertOnSafeTransaction(
            _execTransactionCall(abi.encodeCall(IKindTreeRoot.setKindTreeRoot, (bytes32(uint256(1)))))
        );
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_safe_transaction_that_denies_a_logic_ref() public {
        ILogicRefDenylist.DeniedLogicRef[] memory logicRefs = new ILogicRefDenylist.DeniedLogicRef[](1);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: bytes32(uint256(1)), consumed: true});

        _expectExecuteToRevertOnSafeTransaction(
            _execTransactionCall(abi.encodeCall(ILogicRefDenylist.denyLogicRefs, (logicRefs)))
        );
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_safe_transaction_that_upgrades_the_protocol_adapter()
        public
    {
        address newImplementation = address(new ProtocolAdapter(address(_router), _mockVerifier.SELECTOR()));

        _expectExecuteToRevertOnSafeTransaction(
            _execTransactionCall(abi.encodeCall(UUPSUpgradeable.upgradeToAndCall, (newImplementation, bytes(""))))
        );
    }

    function test_execute_leaves_a_safe_transaction_executable_if_a_forwarder_call_tries_it() public {
        bytes32 newKindTreeRoot = bytes32(uint256(1));
        assertNotEq(_mockPa.getKindTreeRoot(), newKindTreeRoot, "the new root must differ from the stored one");
        bytes memory execTransactionCall =
            _execTransactionCall(abi.encodeCall(IKindTreeRoot.setKindTreeRoot, (newKindTreeRoot)));
        uint256 nonce = _safe.nonce();

        _expectExecuteToRevertOnSafeTransaction(execTransactionCall);

        assertEq(_safe.nonce(), nonce, "the nonce of the Safe transaction should stay unused");
        Address.functionCall(address(_safe), execTransactionCall);
        assertEq(_mockPa.getKindTreeRoot(), newKindTreeRoot, "the Safe transaction should still execute");
    }

    /// @dev Executes a transaction whose forwarder call executes the signed Safe transaction in `execTransactionCall`.
    function _expectExecuteToRevertOnSafeTransaction(bytes memory execTransactionCall) internal {
        assertEq(_mockPa.owner(), address(_safe), "the Safe must own the protocol adapter");

        // `execTransaction` returns true on success, so the output check passes if the Safe transaction executes.
        _expectExecuteToRevertOnForwarderCall({
            target: address(_safe), payload: execTransactionCall, output: abi.encode(true)
        });
    }

    /// @dev Executes a transaction whose created resource makes the forwarder call `target` with `payload`. The
    /// protocol adapter requires `output` as the output of the call.
    function _expectExecuteToRevertOnForwarderCall(address target, bytes memory payload, bytes memory output) internal {
        TxGen.ResourceAndAppData[] memory consumed = new TxGen.ResourceAndAppData[](1);
        consumed[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(0), logicRef: _CARRIER_LOGIC_REF, labelRef: _CARRIER_LOGIC_REF, quantity: 1
            }),
            appData: TxGen.emptyAppData()
        });

        TxGen.ResourceAndAppData[] memory created = new TxGen.ResourceAndAppData[](1);
        created[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(uint256(1)), logicRef: _CARRIER_LOGIC_REF, labelRef: _CARRIER_LOGIC_REF, quantity: 1
            }),
            appData: TxGen.emptyAppData()
        });
        created[0].appData.externalPayload = new IProtocolAdapter.ExpirableBlob[](1);
        created[0].appData.externalPayload[0] =
            TxGen.forwarderCall({forwarder: _fwd, input: abi.encode(target, payload), output: output});

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});
        IProtocolAdapter.Transaction memory txn =
            vm.transaction({mockVerifier: _mockVerifier, actionResources: resourceLists});

        vm.expectRevert(ReentrancyGuardTransient.ReentrancyGuardReentrantCall.selector, address(_mockPa));
        _mockPa.execute(txn);
    }

    /// @dev Encodes the `execTransaction` call of a signed Safe transaction that calls the protocol adapter with
    /// `payload`.
    function _execTransactionCall(bytes memory payload) internal view returns (bytes memory execTransactionCall) {
        // With `safeTxGas` 0 and `gasPrice` 0, the Safe transaction reverts if its call reverts.
        bytes32 txHash = _safe.getTransactionHash({
            to: address(_mockPa),
            value: 0,
            data: payload,
            operation: Enum.Operation.Call,
            safeTxGas: 0,
            baseGas: 0,
            gasPrice: 0,
            gasToken: address(0),
            refundReceiver: address(0),
            _nonce: _safe.nonce()
        });
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(_signerKey, txHash);

        execTransactionCall = abi.encodeCall(
            SafeSmartAccount.execTransaction,
            (
                address(_mockPa),
                0,
                payload,
                Enum.Operation.Call,
                0,
                0,
                0,
                address(0),
                payable(address(0)),
                abi.encodePacked(r, s, v)
            )
        );
    }
}
