// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {ForwarderExample} from "anoma-forwarder-bases-3.1.0/test/examples/ForwarderExample.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-2.0.0/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.1.0/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.1.0/contracts/src/test/RiscZeroMockVerifier.sol";

import {IKindTreeRoot} from "../src/interfaces/IKindTreeRoot.sol";
import {ILogicRefDenylist} from "../src/interfaces/ILogicRefDenylist.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterReentrancyTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _CARRIER_LOGIC_REF = bytes32(uint256(123));

    RiscZeroVerifierRouter internal _router;
    RiscZeroMockVerifier internal _mockVerifier;
    ProtocolAdapter internal _mockPa;
    address internal _fwd;

    function setUp() public {
        (_router,, _mockVerifier) = new DeployRiscZeroContractsMock().run();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _mockVerifier.SELECTOR());

        _mockPa = ProtocolAdapter(
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );

        // The forwarder owns the protocol adapter, so a forwarder call passes `onlyOwner`, as a Safe transaction that
        // an external call executes does.
        _fwd = address(new ForwarderExample({protocolAdapter: address(_mockPa), logicRef: _CARRIER_LOGIC_REF}));
        vm.prank(_OWNER);
        _mockPa.transferOwnership(_fwd);
    }

    function test_execute_reverts_if_a_forwarder_call_executes_a_transaction() public {
        IProtocolAdapter.Transaction memory innerTxn;

        _expectExecuteToRevertOnCallBack(abi.encodeCall(IProtocolAdapter.execute, (innerTxn)));
    }

    function test_execute_reverts_if_a_forwarder_call_pauses_the_protocol_adapter() public {
        _expectExecuteToRevertOnCallBack(abi.encodeCall(IProtocolAdapter.pause, ()));
    }

    function test_execute_reverts_if_a_forwarder_call_unpauses_the_protocol_adapter() public {
        _expectExecuteToRevertOnCallBack(abi.encodeCall(IProtocolAdapter.unpause, ()));
    }

    function test_execute_reverts_if_a_forwarder_call_sets_the_kind_tree_root() public {
        _expectExecuteToRevertOnCallBack(abi.encodeCall(IKindTreeRoot.setKindTreeRoot, (bytes32(uint256(1)))));
    }

    function test_execute_reverts_if_a_forwarder_call_denies_a_logic_ref() public {
        ILogicRefDenylist.DeniedLogicRef[] memory logicRefs = new ILogicRefDenylist.DeniedLogicRef[](1);
        logicRefs[0] = ILogicRefDenylist.DeniedLogicRef({logicRef: bytes32(uint256(1)), consumed: true});

        _expectExecuteToRevertOnCallBack(abi.encodeCall(ILogicRefDenylist.denyLogicRefs, (logicRefs)));
    }

    function test_execute_reverts_if_a_forwarder_call_upgrades_the_protocol_adapter() public {
        address newImplementation = address(new ProtocolAdapter(address(_router), _mockVerifier.SELECTOR()));

        _expectExecuteToRevertOnCallBack(
            abi.encodeCall(UUPSUpgradeable.upgradeToAndCall, (newImplementation, bytes("")))
        );
    }

    /// @dev Executes a transaction whose created resource makes the forwarder call the protocol adapter with `payload`.
    function _expectExecuteToRevertOnCallBack(bytes memory payload) internal {
        assertEq(_mockPa.owner(), _fwd, "the forwarder must own the protocol adapter");

        IProtocolAdapter.Transaction memory txn = vm.transactionWithForwarderCall({
            mockVerifier: _mockVerifier,
            forwarder: _fwd,
            logicRef: _CARRIER_LOGIC_REF,
            input: abi.encode(address(_mockPa), payload),
            output: ""
        });

        vm.expectRevert(ReentrancyGuardTransient.ReentrancyGuardReentrantCall.selector, address(_mockPa));
        _mockPa.execute(txn);
    }
}
