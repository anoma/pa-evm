// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";

import {ForwarderExample} from "anoma-forwarder-bases-3.0.1/test/examples/ForwarderExample.sol";
import {
    ForwarderTargetExample,
    _encodedDefaultInput,
    EXPECTED_OUTPUT
} from "anoma-forwarder-bases-3.0.1/test/examples/ForwarderTargetExample.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.4/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {VerificationFailed} from "risc0-risc0-ethereum-3.0.1/contracts/src/IRiscZeroVerifier.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {ICommitmentTree} from "../src/interfaces/ICommitmentTree.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {DeltaProof} from "../src/libs/DeltaProof.sol";
import {ForwarderCallOrder} from "../src/libs/ForwarderCallOrder.sol";
import {SHA256} from "../src/libs/SHA256.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {CommitmentTree} from "../src/state/CommitmentTree.sol";
import {NullifierSet} from "../src/state/NullifierSet.sol";
import {TxGen} from "./libs/TxGen.sol";
import {CommitmentTreeMock} from "./mocks/CommitmentTree.m.sol";

contract ProtocolAdapterMockVerifierTest is Test {
    using TxGen for IProtocolAdapter.Action[];
    using TxGen for IProtocolAdapter.Action;
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _CARRIER_LOGIC_REF = bytes32(uint256(123));

    RiscZeroVerifierRouter internal _router;
    RiscZeroMockVerifier internal _mockVerifier;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    ProtocolAdapter internal _mockPa;
    address internal _fwd;
    address internal _fwdTarget;
    address[] internal _fwdList;

    bytes internal _input;

    bytes32 internal _carrierLabelRef;

    function setUp() public {
        (_router, _emergencyStop, _mockVerifier) = new DeployRiscZeroContractsMock().run();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _mockVerifier.SELECTOR());

        _mockPa = ProtocolAdapter(
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );

        _fwd = address(new ForwarderExample({protocolAdapter: address(_mockPa), logicRef: _CARRIER_LOGIC_REF}));
        _fwdTarget = address(new ForwarderTargetExample());
        _input = _encodedDefaultInput(_fwdTarget);

        _fwdList = new address[](1);
        _fwdList[0] = _fwd;

        _carrierLabelRef = sha256(abi.encode(_fwd));
    }

    function testFuzz_execute_emits_the_TransactionExecuted_event(uint8 actionCount, uint8 resourcePairCount) public {
        actionCount = uint8(bound(actionCount, 1, 10));
        resourcePairCount = uint8(bound(resourcePairCount, 1, 10));

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.TransactionExecuted({transactionId: TxGen.transactionId(txn)});
        _mockPa.execute(txn);
    }

    function testFuzz_execute_emits_ActionExecuted_events_for_each_action(
        uint8 actionCount,
        uint8 consumedCount,
        uint8 createdCount
    ) public {
        actionCount = uint8(bound(actionCount, 1, 5));
        consumedCount = uint8(bound(consumedCount, 1, 5));
        createdCount = uint8(bound(createdCount, 1, 5));

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: consumedCount, createdCount: createdCount
            })
        });

        for (uint256 i = 0; i < actionCount; ++i) {
            vm.expectEmit(address(_mockPa));
            emit IProtocolAdapter.ActionExecuted({
                actionTreeRoot: txn.actions[i].actionTreeRoot,
                nullifiers: txn.actions[i].actionNullifiers(),
                consumedLogicRefs: txn.actions[i].actionConsumedLogicRefs(),
                commitments: txn.actions[i].actionCommitments(),
                createdLogicRefs: txn.actions[i].actionCreatedLogicRefs()
            });
        }
        _mockPa.execute(txn);
    }

    function test_execute_emits_the_ForwarderCallExecuted_event_on_created_carrier_resource() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleCarrierResourceAndAppData({nonce: 1, fwdList: _fwdList});

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});
        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ForwarderCallExecuted({
            untrustedForwarder: address(_fwd), input: _input, output: EXPECTED_OUTPUT
        });
        _mockPa.execute(txn);
    }

    function test_execute_emits_the_ForwarderCallExecuted_event_on_consumed_carrier_resource() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleCarrierResourceAndAppData({nonce: 0, fwdList: _fwdList});
        TxGen.ResourceAndAppData[] memory created = _exampleResourceAndEmptyAppData({nonce: 1});

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});
        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ForwarderCallExecuted({
            untrustedForwarder: address(_fwd), input: _input, output: EXPECTED_OUTPUT
        });

        _mockPa.execute(txn);
    }

    function test_execute_emits_all_ForwarderCallExecuted_events() public {
        address fwd2 = address(new ForwarderExample({protocolAdapter: address(_mockPa), logicRef: _CARRIER_LOGIC_REF}));
        assertNotEq(_fwd, fwd2, "forwarder addresses should differ");

        address[] memory fwdList = new address[](2);
        fwdList[0] = _fwd;
        fwdList[1] = fwd2;

        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleCarrierResourceAndAppData({nonce: 1, fwdList: fwdList});

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});
        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ForwarderCallExecuted({
            untrustedForwarder: address(_fwd), input: _input, output: EXPECTED_OUTPUT
        });

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ForwarderCallExecuted({
            untrustedForwarder: address(fwd2), input: _input, output: EXPECTED_OUTPUT
        });

        _mockPa.execute(txn);
    }

    function test_execute_runs_unconstrained_calls_in_the_order_of_the_actions() public {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](2);
        resourceLists[0] =
            TxGen.ResourceLists({consumed: _carrier({nonce: 0, callId: 1}), created: _carrier({nonce: 1, callId: 2})});
        resourceLists[1] =
            TxGen.ResourceLists({consumed: _carrier({nonce: 2, callId: 3}), created: _carrier({nonce: 3, callId: 4})});

        _assertCallOrder({txn: vm.transaction(_mockVerifier, resourceLists), expected: _callIds(1, 2, 3, 4)});
    }

    function test_execute_runs_a_call_after_the_calls_of_its_after_tags() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrier({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 2});
        _constrain({data: consumed[0], afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(2, 1)});
    }

    function test_execute_runs_a_call_before_the_calls_of_its_before_tags() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrier({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 2});
        _constrain({data: created[0], afterTags: _noTags(), beforeTags: _tags(_nullifier(consumed[0]))});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(2, 1)});
    }

    /// @dev The protocol adapter changes the state of all resources before any call runs.
    function test_execute_accepts_an_after_tag_that_names_a_resource_without_calls() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 2});
        _constrain({data: created[0], afterTags: _tags(_nullifier(consumed[0])), beforeTags: _noTags()});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(2)});
    }

    function test_execute_reverts_on_an_after_tag_that_is_not_in_the_transaction() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrier({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 2});
        bytes32 tagNotInTransaction = keccak256("not in the transaction");
        _constrain({data: created[0], afterTags: _tags(tagNotInTransaction), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(
            abi.encodeWithSelector(ForwarderCallOrder.AfterTagNotInTransaction.selector, tagNotInTransaction)
        );
        _mockPa.execute(txn);
    }

    /// @dev The call must run before the call of a resource that an earlier transaction consumed, but that call ran
    /// already. A `beforeTags` entry outside the transaction constrains nothing, so the call runs anyway.
    function test_execute_runs_a_call_that_must_precede_a_resource_an_earlier_transaction_consumed() public {
        TxGen.ResourceAndAppData[] memory earlierConsumed = _carrier({nonce: 0, callId: 1});
        _mockPa.execute(_transaction(earlierConsumed, _exampleResourceAndEmptyAppData({nonce: 1})));

        TxGen.ResourceAndAppData[] memory consumed = _carrier({nonce: 2, callId: 2});
        _constrain({data: consumed[0], afterTags: _noTags(), beforeTags: _tags(_nullifier(earlierConsumed[0]))});

        _assertCallOrder({
            txn: _transaction(consumed, _exampleResourceAndEmptyAppData({nonce: 3})), expected: _callIds(2)
        });
    }

    function test_execute_ignores_a_before_tag_that_names_a_resource_without_calls() public {
        TxGen.ResourceAndAppData[] memory consumed = new TxGen.ResourceAndAppData[](2);
        consumed[0] = _exampleResourceAndEmptyAppData({nonce: 0})[0];
        consumed[1] = _carrier({nonce: 1, callId: 1})[0];
        TxGen.ResourceAndAppData[] memory created = new TxGen.ResourceAndAppData[](2);
        created[0] = _carrier({nonce: 2, callId: 2})[0];
        created[1] = _exampleResourceAndEmptyAppData({nonce: 3})[0];
        _constrain({data: created[0], afterTags: _noTags(), beforeTags: _tags(_nullifier(consumed[0]))});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(1, 2)});
    }

    /// @dev The first call of the consumed resource has no constraints, but it runs together with the second call,
    /// which waits for the created resource.
    function test_execute_applies_the_constraints_of_a_call_to_all_calls_of_its_carrier() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrierWithTwoCalls({nonce: 0, firstCallId: 1, secondCallId: 2});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 3});
        _constrainCall({
            data: consumed[0], callIndex: 1, afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()
        });

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(3, 1, 2)});
    }

    /// @dev The first call of the consumed resource must run before the created resource and the second call after
    /// it, which divides the calls of the consumed resource.
    function test_execute_reverts_on_constraints_that_divide_the_calls_of_a_carrier() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrierWithTwoCalls({nonce: 0, firstCallId: 1, secondCallId: 2});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 3});
        bytes32 createdCommitment = _commitment(created[0]);
        _constrainCall({data: consumed[0], callIndex: 0, afterTags: _noTags(), beforeTags: _tags(createdCommitment)});
        _constrainCall({data: consumed[0], callIndex: 1, afterTags: _tags(createdCommitment), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_a_carrier_that_must_run_after_itself() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrier({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 2});
        _constrain({data: created[0], afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_constraints_that_form_a_cycle() public {
        TxGen.ResourceAndAppData[] memory consumed = _carrier({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _carrier({nonce: 1, callId: 2});
        _constrain({data: consumed[0], afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()});
        _constrain({data: created[0], afterTags: _tags(_nullifier(consumed[0])), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _mockPa.execute(txn);
    }

    /// @dev A constraint can name a resource of a later action. Of the calls that can run next, the first in the
    /// order of the actions runs.
    function test_execute_orders_the_calls_across_actions() public {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](2);
        resourceLists[0] =
            TxGen.ResourceLists({consumed: _carrier({nonce: 0, callId: 1}), created: _carrier({nonce: 1, callId: 2})});
        resourceLists[1] =
            TxGen.ResourceLists({consumed: _carrier({nonce: 2, callId: 3}), created: _carrier({nonce: 3, callId: 4})});
        _constrain({
            data: resourceLists[0].consumed[0],
            afterTags: _tags(_commitment(resourceLists[1].created[0])),
            beforeTags: _noTags()
        });

        _assertCallOrder({txn: vm.transaction(_mockVerifier, resourceLists), expected: _callIds(2, 3, 4, 1)});
    }

    /// @dev A swap needs the unwrap (created) before the generic call and the wrap (both consumed). The two lists
    /// run the consumed resources first, so only the constraints set this order.
    function test_execute_runs_the_calls_of_a_swap_in_the_order_of_their_constraints() public {
        TxGen.ResourceAndAppData[] memory consumed = new TxGen.ResourceAndAppData[](3);
        consumed[0] = _exampleResourceAndEmptyAppData({nonce: 0})[0]; // the sold tokens
        consumed[1] = _carrier({nonce: 1, callId: 10})[0]; // the generic call
        consumed[2] = _carrier({nonce: 2, callId: 20})[0]; // the wrap of the bought tokens

        TxGen.ResourceAndAppData[] memory created = new TxGen.ResourceAndAppData[](3);
        created[0] = _carrier({nonce: 3, callId: 30})[0]; // the unwrap of the sold tokens
        created[1] = _exampleResourceAndEmptyAppData({nonce: 4})[0]; // the bought tokens
        created[2] = _exampleResourceAndEmptyAppData({nonce: 5})[0]; // the change

        bytes32 unwrapCommitment = _commitment(created[0]);
        bytes32 genericCallNullifier = _nullifier(consumed[1]);
        _constrain({data: consumed[1], afterTags: _tags(unwrapCommitment), beforeTags: _noTags()});
        _constrain({data: consumed[2], afterTags: _tags(genericCallNullifier), beforeTags: _noTags()});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(30, 10, 20)});
    }

    /// @dev The call of the first consumed resource reads the nullifier of the second one, which the loop adds later.
    /// A call that ran before the nullifier is added would return false and make the transaction revert.
    function test_execute_runs_the_calls_after_adding_the_nullifiers_of_all_consumed_resources() public {
        TxGen.ResourceAndAppData[] memory consumed = new TxGen.ResourceAndAppData[](2);
        consumed[0] = _carrier({nonce: 0, callId: 1})[0];
        consumed[1] = _exampleResourceAndEmptyAppData({nonce: 1})[0];
        TxGen.ResourceAndAppData[] memory created = new TxGen.ResourceAndAppData[](2);
        created[0] = _exampleResourceAndEmptyAppData({nonce: 2})[0];
        created[1] = _exampleResourceAndEmptyAppData({nonce: 3})[0];

        bytes32 secondNullifier = _nullifier(consumed[1]);
        consumed[0].appData.externalPayload[0].blob = _callBlob({
            untrustedForwarder: address(_fwd),
            input: abi.encode(address(_mockPa), abi.encodeCall(_mockPa.isNullifierContained, (secondNullifier))),
            expectedOutput: abi.encode(true)
        });

        _mockPa.execute(_transaction(consumed, created));
    }

    /// @dev The old encoding has no tag lists, so the words where the protocol adapter reads their offsets belong to
    /// the input of the call.
    function test_execute_reverts_on_a_forwarder_call_in_the_encoding_without_tag_lists() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleCarrierResourceAndAppData({nonce: 1, fwdList: _fwdList});
        created[0].appData.externalPayload[0].blob = abi.encode(address(_fwd), _input, EXPECTED_OUTPUT);

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert();
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_a_forwarder_call_whose_after_tags_offset_is_out_of_bounds() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleCarrierResourceAndAppData({nonce: 1, fwdList: _fwdList});
        bytes memory blob = created[0].appData.externalPayload[0].blob;
        // The fourth word of the encoded call is the offset of `afterTags`.
        bytes32 outOfBoundsOffset = bytes32(blob.length);
        for (uint256 k = 0; k < 32; ++k) {
            blob[96 + k] = outOfBoundsOffset[k];
        }
        created[0].appData.externalPayload[0].blob = blob;

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert();
        _mockPa.execute(txn);
    }

    /// @dev `TxGen.expirableBlobs` carries an `Immediately` blob at index 0 and a `Never` blob at index 1, so one
    /// execution exercises both deletion criteria per payload kind.
    function test_execute_emits_the_persistent_app_data_blobs_only() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleResourceAndBlobAppData({nonce: 1});

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});
        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);

        bytes32 tag = txn.actions[0].created[0].commitment;
        bytes memory persistent = TxGen.expirableBlobs()[1].blob;

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ResourcePayload({tag: tag, index: 1, blob: persistent});

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.DiscoveryPayload({tag: tag, index: 1, blob: persistent});

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ApplicationPayload({tag: tag, index: 1, blob: persistent});

        vm.recordLogs();
        _mockPa.execute(txn);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertEq(_countEvents(logs, IProtocolAdapter.ResourcePayload.selector), 1, "the expiring blob should be kept");
        assertEq(_countEvents(logs, IProtocolAdapter.DiscoveryPayload.selector), 1, "the expiring blob should be kept");
        assertEq(
            _countEvents(logs, IProtocolAdapter.ApplicationPayload.selector), 1, "the expiring blob should be kept"
        );
    }

    /// @dev An action without resources carries the zero delta point, which is not on the curve. Such an action is
    /// unprovable anyway — the compliance circuit requires at least one consumed resource.
    function test_execute_reverts_on_actions_without_resources() public {
        TxGen.ActionConfig[] memory configs = new TxGen.ActionConfig[](1);
        configs[0] = TxGen.ActionConfig({consumedCount: 0, createdCount: 0});

        (IProtocolAdapter.Transaction memory txn,) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});

        vm.expectPartialRevert(DeltaProof.PointNotOnCurve.selector);
        _mockPa.execute(txn);
    }

    function testFuzz_execute_1_txn_with_n_actions_and_m_resources(
        uint8 actionCount,
        uint8 consumedCount,
        uint8 createdCount
    ) public {
        TxGen.ActionConfig[] memory configs = TxGen.generateActionConfigs({
            actionCount: uint8(bound(actionCount, 1, 5)),
            consumedCount: uint8(bound(consumedCount, 1, 5)),
            createdCount: uint8(bound(createdCount, 1, 5))
        });

        (IProtocolAdapter.Transaction memory txn,) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});
        _mockPa.execute(txn);
    }

    function testFuzz_execute_2_txns_with_n_actions_and_m_resources(uint8 actionCount, uint8 resourcePairCount) public {
        TxGen.ActionConfig[] memory configs = TxGen.generateActionConfigs({
            actionCount: uint8(bound(actionCount, 1, 5)),
            consumedCount: uint8(bound(resourcePairCount, 1, 5)),
            createdCount: uint8(bound(resourcePairCount, 1, 5))
        });

        (IProtocolAdapter.Transaction memory txn, bytes32 updatedNonce) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});
        _mockPa.execute(txn);

        (txn,) = vm.transaction({mockVerifier: _mockVerifier, nonce: updatedNonce, configs: configs});
        _mockPa.execute(txn);
    }

    function test_execute_executes_consume_only_transactions_without_storing_a_root() public {
        bytes32 rootBefore = _mockPa.latestCommitmentTreeRoot();

        TxGen.ActionConfig[] memory configs = new TxGen.ActionConfig[](1);
        configs[0] = TxGen.ActionConfig({consumedCount: 2, createdCount: 0});

        (IProtocolAdapter.Transaction memory txn, bytes32 updatedNonce) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});
        _mockPa.execute(txn);

        assertEq(_mockPa.latestCommitmentTreeRoot(), rootBefore, "the latest root should be unchanged");
        bytes32[] memory nullifiers = TxGen.collectNullifiers(txn);
        for (uint256 i = 0; i < nullifiers.length; ++i) {
            assertTrue(_mockPa.isNullifierContained(nullifiers[i]), "the nullifiers should have been added");
        }

        // A second consume-only transaction executes as well.
        (txn,) = vm.transaction({mockVerifier: _mockVerifier, nonce: updatedNonce, configs: configs});
        _mockPa.execute(txn);

        assertEq(_mockPa.latestCommitmentTreeRoot(), rootBefore, "the latest root should still be unchanged");
    }

    function test_execute_emits_no_CommitmentTreeRootAdded_event_for_consume_only_transactions() public {
        TxGen.ActionConfig[] memory configs = new TxGen.ActionConfig[](1);
        configs[0] = TxGen.ActionConfig({consumedCount: 1, createdCount: 0});

        (IProtocolAdapter.Transaction memory txn,) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});

        vm.recordLogs();
        _mockPa.execute(txn);

        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; ++i) {
            assertNotEq(
                logs[i].topics[0],
                ICommitmentTree.CommitmentTreeRootAdded.selector,
                "no CommitmentTreeRootAdded event should be emitted"
            );
        }
    }

    /// @dev `execute` must also store the root when the last actions create nothing.
    function testFuzz_execute_stores_the_latest_root(
        uint8 actionCount,
        uint8[5] memory consumedCounts,
        uint8[5] memory createdCounts
    ) public {
        TxGen.ActionConfig[] memory configs = new TxGen.ActionConfig[](bound(actionCount, 1, 5));
        uint256 createdCount = 0;
        for (uint256 i = 0; i < configs.length; ++i) {
            configs[i] = TxGen.ActionConfig({
                consumedCount: bound(consumedCounts[i], 1, 5), createdCount: bound(createdCounts[i], 0, 5)
            });
            createdCount += configs[i].createdCount;
        }
        vm.assume(createdCount != 0);

        (IProtocolAdapter.Transaction memory txn,) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});
        _mockPa.execute(txn);

        assertTrue(
            _mockPa.isCommitmentTreeRootHistorical(_mockPa.latestCommitmentTreeRoot()),
            "the latest root should be a historical root"
        );
    }

    function test_execute_reverts_on_the_empty_transaction() public {
        IProtocolAdapter.Transaction memory txn = IProtocolAdapter.Transaction({
            actions: new IProtocolAdapter.Action[](0),
            deltaProof: "",
            aggregationProof: "",
            kindTreeRoot: _mockPa.EMPTY_KIND_TREE_ROOT()
        });

        vm.expectRevert(ProtocolAdapter.EmptyTransactionNotAllowed.selector, address(_mockPa));
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_pre_existing_nullifier() public {
        TxGen.ActionConfig[] memory configs =
            TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1});

        (IProtocolAdapter.Transaction memory tx1,) =
            vm.transaction({mockVerifier: _mockVerifier, nonce: 0, configs: configs});
        bytes32 preExistingNf = tx1.actions[0].consumed[0].nullifier;
        _mockPa.execute(tx1);

        vm.expectRevert(
            abi.encodeWithSelector(NullifierSet.PreExistingNullifier.selector, preExistingNf), address(_mockPa)
        );
        _mockPa.execute(tx1);
    }

    /// @notice Test that transactions with nonexistent roots fail.
    function testFuzz_execute_reverts_on_non_existing_root(
        uint8 actionCount,
        uint8 resourcePairCount,
        uint8 actionIndex,
        uint8 resourceIndex,
        bytes32 fakeRoot
    ) public {
        // Assume the proposed commitment tree root is not a historical root.
        vm.assume(!_mockPa.isCommitmentTreeRootHistorical(fakeRoot));

        // Choose a random consumed resource among the actions.
        (actionCount, resourcePairCount, actionIndex, resourceIndex) =
            _bindParameters(actionCount, resourcePairCount, actionIndex, resourceIndex);

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        // Assign the proposed commitment tree root into the transaction.
        txn.actions[actionIndex].consumed[resourceIndex].commitmentTreeRoot = fakeRoot;

        vm.expectRevert(abi.encodeWithSelector(CommitmentTree.NonExistingRoot.selector, fakeRoot));
        _mockPa.execute(txn);
    }

    /// @notice Tampering the action tree root changes the delta verifying key, so the delta proof fails first.
    function testFuzz_execute_reverts_on_tampered_action_tree_root(
        uint8 actionCount,
        uint8 resourcePairCount,
        uint8 actionIndex,
        bytes32 nonce
    ) public {
        (actionCount, resourcePairCount, actionIndex,) = _bindParameters(actionCount, resourcePairCount, actionIndex, 0);

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        txn.actions[actionIndex].actionTreeRoot = SHA256.hash(txn.actions[actionIndex].actionTreeRoot, nonce);

        vm.expectPartialRevert(DeltaProof.DeltaMismatch.selector);
        _mockPa.execute(txn);
    }

    /// @notice Tampering a logic reference makes the reconstructed journal mismatch the proven one.
    function testFuzz_execute_reverts_on_tampered_logic_reference(
        uint8 actionCount,
        uint8 resourcePairCount,
        uint8 actionIndex,
        uint8 resourceIndex,
        bytes32 nonce
    ) public {
        (actionCount, resourcePairCount, actionIndex, resourceIndex) =
            _bindParameters(actionCount, resourcePairCount, actionIndex, resourceIndex);

        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        bytes32 originalLogicRef = txn.actions[actionIndex].consumed[resourceIndex].logicRef;
        txn.actions[actionIndex].consumed[resourceIndex].logicRef = SHA256.hash(originalLogicRef, nonce);

        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
        _mockPa.execute(txn);
    }

    function testFuzz_execute_reverts_on_unexpected_forwarder_call_output(bytes memory fakeOutput) public {
        vm.assume(keccak256(fakeOutput) != keccak256(EXPECTED_OUTPUT));

        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleCarrierResourceAndAppData({nonce: 1, fwdList: _fwdList});

        created[0].appData.externalPayload[0].blob =
            _callBlob({untrustedForwarder: address(_fwd), input: _input, expectedOutput: fakeOutput});

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});

        // Create a transaction with two resources, the created calling the forwarder.
        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);

        // Expect output mismatch.
        vm.expectRevert(
            abi.encodeWithSelector(ProtocolAdapter.ForwarderCallOutputMismatch.selector, fakeOutput, EXPECTED_OUTPUT)
        );
        _mockPa.execute(txn);
    }

    function testFuzz_execute_reverts_on_ubalanced_delta(uint128 createdQuantity, uint128 consumedQuantity) public {
        vm.assume(createdQuantity != consumedQuantity);
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleResourceAndEmptyAppData({nonce: 0});

        // Make transaction unbalanced by offsettig the deltas.
        created[0].resource.quantity = createdQuantity;
        consumed[0].resource.quantity = consumedQuantity;

        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});

        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);
        vm.expectPartialRevert(DeltaProof.DeltaMismatch.selector);
        _mockPa.execute(txn);
    }

    function testFuzz_execute_updates_root(uint8 actionCount, uint8 resourcePairCount) public {
        (actionCount, resourcePairCount,,) = _bindParameters(actionCount, resourcePairCount, 0, 0);
        bytes32 oldRoot = _mockPa.latestCommitmentTreeRoot();
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        _mockPa.execute(txn);

        bytes32 newRoot = _mockPa.latestCommitmentTreeRoot();

        assertTrue(oldRoot != newRoot, "commitment tree root should change after execution");
    }

    function testFuzz_execute_updates_commitment_root_exactly_with_desired_commitments(
        uint8 actionCount,
        uint8 resourcePairCount
    ) public {
        (actionCount, resourcePairCount,,) = _bindParameters(actionCount, resourcePairCount, 0, 0);
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        _mockPa.execute(txn);

        CommitmentTreeMock newCmTree = CommitmentTreeMock(
            address(
                new ERC1967Proxy(address(new CommitmentTreeMock()), abi.encodeCall(CommitmentTreeMock.initialize, ()))
            )
        );

        bytes32[] memory cms = TxGen.collectCommitments(txn);
        bytes32 newRoot = newCmTree.latestCommitmentTreeRoot();

        for (uint256 i = 0; i < cms.length; ++i) {
            newRoot = newCmTree.addCommitment(cms[i]);
        }

        newCmTree.addCommitmentTreeRoot(newRoot);

        assertTrue(
            _mockPa.latestCommitmentTreeRoot() == newCmTree.latestCommitmentTreeRoot(),
            "commitment tree roots should match"
        );
    }

    function testFuzz_execute_adds_the_nullifiers_of_the_transaction(uint8 actionCount, uint8 resourcePairCount)
        public
    {
        (actionCount, resourcePairCount,,) = _bindParameters(actionCount, resourcePairCount, 0, 0);
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({
                actionCount: actionCount, consumedCount: resourcePairCount, createdCount: resourcePairCount
            })
        });

        _mockPa.execute(txn);

        bytes32[] memory nlfs = TxGen.collectNullifiers(txn);

        for (uint256 i = 0; i < nlfs.length; ++i) {
            assertTrue(_mockPa.isNullifierContained(nlfs[i]), "nullifier should be contained after execution");
        }
    }

    function test_execute_calls_the_risc_zero_verifier_router_exactly_once() public {
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });

        vm.expectCall({callee: address(_router), data: bytes(""), count: 1});
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_vulnerable_risc_zero_verifier() public {
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });

        vm.prank(_emergencyStop.owner());
        _emergencyStop.estop();

        vm.expectRevert(Pausable.EnforcedPause.selector, address(_emergencyStop));
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_an_invalid_aggregation_proof() public {
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });

        bytes memory proof = txn.aggregationProof;
        proof[5] = _flipBits(proof[5]);
        txn.aggregationProof = proof;

        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
        _mockPa.execute(txn);
    }

    function _transaction(TxGen.ResourceAndAppData[] memory consumed, TxGen.ResourceAndAppData[] memory created)
        private
        returns (IProtocolAdapter.Transaction memory txn)
    {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({consumed: consumed, created: created});
        txn = vm.transaction(_mockVerifier, resourceLists);
    }

    /// @notice Executes the transaction and checks the values of its forwarder calls in the order in which they ran.
    function _assertCallOrder(IProtocolAdapter.Transaction memory txn, uint256[] memory expected) private {
        vm.recordLogs();
        _mockPa.execute(txn);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        uint256[] memory actual = new uint256[](logs.length);
        uint256 count = 0;
        for (uint256 i = 0; i < logs.length; ++i) {
            if (
                logs[i].emitter == address(_mockPa)
                    && logs[i].topics[0] == IProtocolAdapter.ForwarderCallExecuted.selector
            ) {
                (, bytes memory output) = abi.decode(logs[i].data, (bytes, bytes));
                actual[count++] = abi.decode(output, (uint256)) - 1;
            }
        }

        assertEq(count, expected.length, "number of forwarder calls");
        for (uint256 i = 0; i < count; ++i) {
            assertEq(actual[i], expected[i], "value of the forwarder call");
        }
    }

    function _exampleResourceAndEmptyAppData(uint256 nonce)
        private
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        data = new TxGen.ResourceAndAppData[](1);

        data[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(nonce), logicRef: _CARRIER_LOGIC_REF, labelRef: _carrierLabelRef, quantity: 1
            }),
            appData: TxGen.emptyAppData()
        });
    }

    function _exampleResourceAndBlobAppData(uint256 nonce)
        private
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        data = new TxGen.ResourceAndAppData[](1);

        data[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(nonce), logicRef: _CARRIER_LOGIC_REF, labelRef: _carrierLabelRef, quantity: 1
            }),
            appData: IProtocolAdapter.AppData({
                resourcePayload: TxGen.expirableBlobs(),
                discoveryPayload: TxGen.expirableBlobs(),
                externalPayload: new IProtocolAdapter.ExpirableBlob[](0),
                applicationPayload: TxGen.expirableBlobs()
            })
        });
    }

    function _exampleCarrierResourceAndAppData(uint256 nonce, address[] memory fwdList)
        private
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        data = new TxGen.ResourceAndAppData[](1);
        uint256 nCalls = fwdList.length;

        data[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(nonce), logicRef: _CARRIER_LOGIC_REF, labelRef: _carrierLabelRef, quantity: 1
            }),
            appData: IProtocolAdapter.AppData({
                discoveryPayload: new IProtocolAdapter.ExpirableBlob[](0),
                resourcePayload: new IProtocolAdapter.ExpirableBlob[](0),
                externalPayload: new IProtocolAdapter.ExpirableBlob[](nCalls),
                applicationPayload: new IProtocolAdapter.ExpirableBlob[](0)
            })
        });

        IProtocolAdapter.ExpirableBlob[] memory externalBlobs = new IProtocolAdapter.ExpirableBlob[](nCalls);
        for (uint256 i = 0; i < nCalls; ++i) {
            externalBlobs[i] = IProtocolAdapter.ExpirableBlob({
                deletionCriterion: IProtocolAdapter.DeletionCriterion.Never,
                blob: _callBlob({
                    untrustedForwarder: address(fwdList[i]), input: _input, expectedOutput: EXPECTED_OUTPUT
                })
            });
        }
        data[0].appData.externalPayload = externalBlobs;
    }

    /// @notice Builds a resource that carries one forwarder call. The call increments `callId`, so that the output
    /// of each call names it.
    function _carrier(uint256 nonce, uint256 callId) private view returns (TxGen.ResourceAndAppData[] memory data) {
        data = _exampleCarrierResourceAndAppData({nonce: nonce, fwdList: _fwdList});
        data[0].appData.externalPayload[0].blob = _callBlob({
            untrustedForwarder: address(_fwd),
            input: abi.encode(_fwdTarget, abi.encodeCall(ForwarderTargetExample.increment, (callId))),
            expectedOutput: abi.encode(callId + 1)
        });
    }

    /// @notice Builds a resource that carries two forwarder calls, which increment `firstCallId` and `secondCallId`.
    function _carrierWithTwoCalls(uint256 nonce, uint256 firstCallId, uint256 secondCallId)
        private
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        address[] memory fwdList = new address[](2);
        fwdList[0] = _fwd;
        fwdList[1] = _fwd;
        data = _exampleCarrierResourceAndAppData({nonce: nonce, fwdList: fwdList});
        data[0].appData.externalPayload[0].blob =
        _carrier({nonce: nonce, callId: firstCallId})[0].appData.externalPayload[0].blob;
        data[0].appData.externalPayload[1].blob =
        _carrier({nonce: nonce, callId: secondCallId})[0].appData.externalPayload[0].blob;
    }

    /// @notice Rewrites the first call of a resource to carry the constraints.
    function _constrain(TxGen.ResourceAndAppData memory data, bytes32[] memory afterTags, bytes32[] memory beforeTags)
        private
        pure
    {
        _constrainCall({data: data, callIndex: 0, afterTags: afterTags, beforeTags: beforeTags});
    }

    /// @notice Rewrites a call of a resource to carry the constraints. This runs before the transaction is built,
    /// because the aggregation journal covers the actions and therefore the app data.
    function _constrainCall(
        TxGen.ResourceAndAppData memory data,
        uint256 callIndex,
        bytes32[] memory afterTags,
        bytes32[] memory beforeTags
    ) private pure {
        (address untrustedForwarder, bytes memory input, bytes memory expectedOutput,,) = abi.decode(
            data.appData.externalPayload[callIndex].blob, (address, bytes, bytes, bytes32[], bytes32[])
        );
        data.appData.externalPayload[callIndex].blob = _callBlob({
            untrustedForwarder: untrustedForwarder,
            input: input,
            expectedOutput: expectedOutput,
            afterTags: afterTags,
            beforeTags: beforeTags
        });
    }

    function _nullifier(TxGen.ResourceAndAppData memory data) private pure returns (bytes32 tag) {
        tag = TxGen.nullifier(data.resource, 0);
    }

    function _commitment(TxGen.ResourceAndAppData memory data) private pure returns (bytes32 tag) {
        tag = TxGen.commitment(data.resource);
    }

    function _callIds(uint256 a) private pure returns (uint256[] memory callIds) {
        callIds = new uint256[](1);
        callIds[0] = a;
    }

    function _callIds(uint256 a, uint256 b) private pure returns (uint256[] memory callIds) {
        callIds = new uint256[](2);
        callIds[0] = a;
        callIds[1] = b;
    }

    function _callIds(uint256 a, uint256 b, uint256 c) private pure returns (uint256[] memory callIds) {
        callIds = new uint256[](3);
        callIds[0] = a;
        callIds[1] = b;
        callIds[2] = c;
    }

    function _callIds(uint256 a, uint256 b, uint256 c, uint256 d) private pure returns (uint256[] memory callIds) {
        callIds = new uint256[](4);
        callIds[0] = a;
        callIds[1] = b;
        callIds[2] = c;
        callIds[3] = d;
    }

    /// @notice Encodes an unconstrained forwarder call blob.
    function _callBlob(address untrustedForwarder, bytes memory input, bytes memory expectedOutput)
        private
        pure
        returns (bytes memory blob)
    {
        blob = _callBlob({
            untrustedForwarder: untrustedForwarder,
            input: input,
            expectedOutput: expectedOutput,
            afterTags: new bytes32[](0),
            beforeTags: new bytes32[](0)
        });
    }

    /// @notice Encodes a forwarder call blob carrying ordering constraints.
    function _callBlob(
        address untrustedForwarder,
        bytes memory input,
        bytes memory expectedOutput,
        bytes32[] memory afterTags,
        bytes32[] memory beforeTags
    ) private pure returns (bytes memory blob) {
        blob = abi.encode(untrustedForwarder, input, expectedOutput, afterTags, beforeTags);
    }

    function _tags(bytes32 tag) private pure returns (bytes32[] memory tags) {
        tags = new bytes32[](1);
        tags[0] = tag;
    }

    function _noTags() private pure returns (bytes32[] memory tags) {
        tags = new bytes32[](0);
    }

    function _bindParameters(uint8 actionCount, uint8 resourcePairCount, uint8 actionIndex, uint8 resourceIndex)
        private
        pure
        returns (uint8 boundActionCount, uint8 boundResourcePairCount, uint8 boundActionIndex, uint8 boundResourceIndex)
    {
        boundActionCount = uint8(bound(actionCount, 1, 5));
        boundResourcePairCount = uint8(bound(resourcePairCount, 1, 5));
        boundActionIndex = uint8(bound(actionIndex, 0, boundActionCount - 1));
        boundResourceIndex = uint8(bound(resourceIndex, 0, boundResourcePairCount - 1));
    }

    function _countEvents(Vm.Log[] memory logs, bytes32 topic) private pure returns (uint256 count) {
        for (uint256 i = 0; i < logs.length; ++i) {
            if (logs[i].topics[0] == topic) {
                ++count;
            }
        }
    }

    /// @dev Flips all bits of a seal byte to invalidate the proof it belongs to.
    function _flipBits(bytes1 sealByte) private pure returns (bytes1 flipped) {
        flipped = sealByte ^ bytes1(0xff);
    }
}
