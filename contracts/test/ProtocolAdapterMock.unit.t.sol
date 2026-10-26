// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";

import {ForwarderExample} from "anoma-forwarder-bases-3.1.0/test/examples/ForwarderExample.sol";
import {
    ForwarderTargetExample,
    _encodedDefaultInput,
    EXPECTED_OUTPUT
} from "anoma-forwarder-bases-3.1.0/test/examples/ForwarderTargetExample.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.5/test/script/DeployRiscZeroContractsMock.s.sol";
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

    /// @dev The resources are built as consumed 0, consumed 1, created 0, created 1.
    function test_execute_calls_the_forwarders_in_the_order_of_the_resources() public {
        (IProtocolAdapter.Transaction memory txn, address[] memory forwarders) =
            _carrierTransaction({consumedCount: 2, createdCount: 2});

        uint256[] memory order = _order(2, 1, 0, 3);
        txn = _reorderFirstAction(txn, order);

        for (uint256 i = 0; i < order.length; ++i) {
            _expectForwarderCall(forwarders[order[i]]);
        }
        _mockPa.execute(txn);
    }

    /// @dev The resources are built as consumed 0, consumed 1, created 0, created 1.
    function test_execute_emits_the_tags_in_the_order_of_the_resources() public {
        (IProtocolAdapter.Transaction memory txn,) = _carrierTransaction({consumedCount: 2, createdCount: 2});
        IProtocolAdapter.ResourceInstance[] memory built = txn.actions[0].resources;

        txn = _reorderFirstAction(txn, _order(3, 1, 2, 0));

        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ActionExecuted({
            actionTreeRoot: txn.actions[0].actionTreeRoot,
            nullifiers: _pair(built[1].tag, built[0].tag),
            consumedLogicRefs: _pair(_CARRIER_LOGIC_REF, _CARRIER_LOGIC_REF),
            commitments: _pair(built[3].tag, built[2].tag),
            createdLogicRefs: _pair(_CARRIER_LOGIC_REF, _CARRIER_LOGIC_REF)
        });
        _mockPa.execute(txn);
    }

    /// @dev The resources are built as consumed 0, created 0, created 1.
    function test_execute_adds_the_commitments_in_the_order_of_the_resources() public {
        (IProtocolAdapter.Transaction memory txn,) = _carrierTransaction({consumedCount: 1, createdCount: 2});
        IProtocolAdapter.ResourceInstance[] memory built = txn.actions[0].resources;

        uint256[] memory order = new uint256[](3);
        (order[0], order[1], order[2]) = (2, 0, 1);
        txn = _reorderFirstAction(txn, order);

        _mockPa.execute(txn);

        CommitmentTreeMock expectedTree = CommitmentTreeMock(
            address(
                new ERC1967Proxy(address(new CommitmentTreeMock()), abi.encodeCall(CommitmentTreeMock.initialize, ()))
            )
        );
        expectedTree.addCommitment(built[2].tag);
        expectedTree.addCommitment(built[1].tag);
        expectedTree.addCommitmentTreeRoot(expectedTree.latestCommitmentTreeRoot());

        assertEq(_mockPa.latestCommitmentTreeRoot(), expectedTree.latestCommitmentTreeRoot());
    }

    /// @dev The aggregation proof covers the consumed flag, so a consumed resource cannot pass as a created resource.
    function test_execute_reverts_if_a_consumed_resource_is_marked_as_created() public {
        (IProtocolAdapter.Transaction memory txn,) = vm.transaction({
            mockVerifier: _mockVerifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        assertTrue(txn.actions[0].resources[0].consumed, "the fixture lists the consumed resource first");
        txn.actions[0].resources[0].consumed = false;

        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
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

        bytes32 tag = TxGen.actionCommitments(txn.actions[0])[0];
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
        bytes32 preExistingNf = TxGen.actionNullifiers(tx1.actions[0])[0];
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

        // Assign the proposed commitment tree root into the transaction. The consumed resources come first.
        txn.actions[actionIndex].resources[resourceIndex].commitmentTreeRoot = fakeRoot;

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

        bytes32 originalLogicRef = txn.actions[actionIndex].resources[resourceIndex].logicRef;
        txn.actions[actionIndex].resources[resourceIndex].logicRef = SHA256.hash(originalLogicRef, nonce);

        vm.expectRevert(VerificationFailed.selector, address(_mockVerifier));
        _mockPa.execute(txn);
    }

    function testFuzz_execute_reverts_on_unexpected_forwarder_call_output(bytes memory fakeOutput) public {
        vm.assume(keccak256(fakeOutput) != keccak256(EXPECTED_OUTPUT));

        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created = _exampleCarrierResourceAndAppData({nonce: 1, fwdList: _fwdList});

        created[0].appData.externalPayload[0].blob = abi.encode(_fwd, _input, fakeOutput);

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
        for (uint256 i = 0; i < cms.length; ++i) {
            newCmTree.addCommitment(cms[i]);
        }

        newCmTree.addCommitmentTreeRoot(newCmTree.latestCommitmentTreeRoot());

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

    /// @dev Builds a transaction of one action in which every resource calls its own forwarder. The resources are
    /// the consumed resources, then the created resources, and `forwarders` lists their forwarders in this order. Each
    /// consumed resource has quantity `createdCount` and each created resource quantity `consumedCount`, so the action
    /// balances.
    function _carrierTransaction(uint128 consumedCount, uint128 createdCount)
        private
        returns (IProtocolAdapter.Transaction memory txn, address[] memory forwarders)
    {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](1);
        resourceLists[0] = TxGen.ResourceLists({
            consumed: new TxGen.ResourceAndAppData[](consumedCount),
            created: new TxGen.ResourceAndAppData[](createdCount)
        });

        forwarders = new address[](consumedCount + createdCount);
        for (uint256 i = 0; i < forwarders.length; ++i) {
            forwarders[i] =
                address(new ForwarderExample({protocolAdapter: address(_mockPa), logicRef: _CARRIER_LOGIC_REF}));
            if (i < consumedCount) {
                resourceLists[0].consumed[i] =
                    _carrierResourceAndAppData({nonce: i, forwarder: forwarders[i], quantity: createdCount});
            } else {
                resourceLists[0].created[i - consumedCount] =
                    _carrierResourceAndAppData({nonce: i, forwarder: forwarders[i], quantity: consumedCount});
            }
        }

        txn = vm.transaction(_mockVerifier, resourceLists);
    }

    function _expectForwarderCall(address forwarder) private {
        vm.expectEmit(address(_mockPa));
        emit IProtocolAdapter.ForwarderCallExecuted({
            untrustedForwarder: forwarder, input: _input, output: EXPECTED_OUTPUT
        });
    }

    /// @dev Reorders the resources of the first action. The action tree root covers the order, so the delta proof
    /// and the aggregation proof are made again.
    function _reorderFirstAction(IProtocolAdapter.Transaction memory txn, uint256[] memory order)
        private
        returns (IProtocolAdapter.Transaction memory reorderedTxn)
    {
        IProtocolAdapter.ResourceInstance[] memory resources = txn.actions[0].resources;
        IProtocolAdapter.ResourceInstance[] memory reordered = new IProtocolAdapter.ResourceInstance[](order.length);
        for (uint256 i = 0; i < order.length; ++i) {
            reordered[i] = resources[order[i]];
        }
        txn.actions[0].resources = reordered;
        txn.actions[0].actionTreeRoot = TxGen.computeActionTreeRoot(reordered);

        reorderedTxn = vm.transactionFromActions(txn.actions);
        reorderedTxn = TxGen.transactionAggregation({
            mockVerifier: _mockVerifier, txn: reorderedTxn, kindTreeRoot: TxGen.emptyKindTreeRoot()
        });
    }

    function _carrierResourceAndAppData(uint256 nonce, address forwarder, uint128 quantity)
        private
        view
        returns (TxGen.ResourceAndAppData memory data)
    {
        address[] memory fwdList = new address[](1);
        fwdList[0] = forwarder;

        data = _exampleCarrierResourceAndAppData({nonce: nonce, fwdList: fwdList})[0];
        data.resource.quantity = quantity;
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
                blob: abi.encode(address(fwdList[i]), _input, EXPECTED_OUTPUT)
            });
        }
        data[0].appData.externalPayload = externalBlobs;
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

    function _order(uint256 first, uint256 second, uint256 third, uint256 fourth)
        private
        pure
        returns (uint256[] memory order)
    {
        order = new uint256[](4);
        (order[0], order[1], order[2], order[3]) = (first, second, third, fourth);
    }

    function _pair(bytes32 first, bytes32 second) private pure returns (bytes32[] memory pair) {
        pair = new bytes32[](2);
        (pair[0], pair[1]) = (first, second);
    }
}
