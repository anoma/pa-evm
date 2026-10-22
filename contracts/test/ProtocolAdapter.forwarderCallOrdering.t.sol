// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {
    ForwarderTargetExample,
    EXPECTED_OUTPUT
} from "anoma-forwarder-bases-3.0.1/test/examples/ForwarderTargetExample.sol";
import {Vm} from "forge-std-1.17.0/src/Test.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ForwarderCallOrdering} from "../src/libs/ForwarderCallOrdering.sol";
import {ProtocolAdapterMockVerifierFixture} from "./fixtures/ProtocolAdapterMockVerifierFixture.sol";
import {TxGen} from "./libs/TxGen.sol";
import {CommitmentTreeMock} from "./mocks/CommitmentTree.m.sol";

contract ProtocolAdapterForwarderCallOrderingTest is ProtocolAdapterMockVerifierFixture {
    using TxGen for Vm;

    function test_execute_runs_unconstrained_calls_in_the_order_of_the_actions() public {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](2);
        resourceLists[0] = TxGen.ResourceLists({
            consumed: _callCarryingResource({nonce: 0, callId: 1}),
            created: _callCarryingResource({nonce: 1, callId: 2})
        });
        resourceLists[1] = TxGen.ResourceLists({
            consumed: _callCarryingResource({nonce: 2, callId: 3}),
            created: _callCarryingResource({nonce: 3, callId: 4})
        });

        _assertCallOrder({txn: vm.transaction(_mockVerifier, resourceLists), expected: _callIds(1, 2, 3, 4)});
    }

    function test_execute_runs_a_call_after_the_calls_of_its_after_tags() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 2});
        _constrain({data: consumed[0], afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(2, 1)});
    }

    function test_execute_runs_a_call_before_the_calls_of_its_before_tags() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 2});
        _constrain({data: created[0], afterTags: _noTags(), beforeTags: _tags(_nullifier(consumed[0]))});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(2, 1)});
    }

    function test_execute_ignores_an_after_tag_that_names_a_consumed_resource_without_calls() public {
        (TxGen.ResourceAndAppData[] memory consumed, TxGen.ResourceAndAppData[] memory created) =
            _resourcesWithAndWithoutCalls();
        _constrain({data: created[0], afterTags: _tags(_nullifier(consumed[0])), beforeTags: _noTags()});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(1, 2)});
    }

    function test_execute_ignores_an_after_tag_that_names_a_created_resource_without_calls() public {
        (TxGen.ResourceAndAppData[] memory consumed, TxGen.ResourceAndAppData[] memory created) =
            _resourcesWithAndWithoutCalls();
        _constrain({data: created[0], afterTags: _tags(_commitment(created[1])), beforeTags: _noTags()});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(1, 2)});
    }

    function test_execute_ignores_a_before_tag_that_names_a_consumed_resource_without_calls() public {
        (TxGen.ResourceAndAppData[] memory consumed, TxGen.ResourceAndAppData[] memory created) =
            _resourcesWithAndWithoutCalls();
        _constrain({data: created[0], afterTags: _noTags(), beforeTags: _tags(_nullifier(consumed[0]))});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(1, 2)});
    }

    function test_execute_ignores_a_before_tag_that_names_a_created_resource_without_calls() public {
        (TxGen.ResourceAndAppData[] memory consumed, TxGen.ResourceAndAppData[] memory created) =
            _resourcesWithAndWithoutCalls();
        _constrain({data: created[0], afterTags: _noTags(), beforeTags: _tags(_commitment(created[1]))});

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(1, 2)});
    }

    function test_execute_reverts_on_an_after_tag_that_is_not_in_the_transaction() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 2});
        bytes32 tagNotInTransaction = keccak256("not in the transaction");
        _constrain({data: created[0], afterTags: _tags(tagNotInTransaction), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(
            abi.encodeWithSelector(ForwarderCallOrdering.AfterTagNotInTransaction.selector, tagNotInTransaction)
        );
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_a_before_tag_that_is_not_in_the_transaction() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 2});
        bytes32 tagNotInTransaction = keccak256("not in the transaction");
        _constrain({data: created[0], afterTags: _noTags(), beforeTags: _tags(tagNotInTransaction)});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(
            abi.encodeWithSelector(ForwarderCallOrdering.BeforeTagNotInTransaction.selector, tagNotInTransaction)
        );
        _mockPa.execute(txn);
    }

    /// @dev The first call of the consumed resource has no constraints, but it runs together with the second call,
    /// which waits for the created resource.
    function test_execute_applies_the_constraints_of_a_call_to_all_calls_of_its_resource() public {
        TxGen.ResourceAndAppData[] memory consumed =
            _callCarryingResourceWithTwoCalls({nonce: 0, firstCallId: 1, secondCallId: 2});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 3});
        _constrainCall({
            data: consumed[0], callIndex: 1, afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()
        });

        _assertCallOrder({txn: _transaction(consumed, created), expected: _callIds(3, 1, 2)});
    }

    /// @dev The first call of the consumed resource must run before the created resource and the second call after
    /// it, which divides the calls of the consumed resource.
    function test_execute_reverts_on_constraints_that_divide_the_calls_of_a_resource() public {
        TxGen.ResourceAndAppData[] memory consumed =
            _callCarryingResourceWithTwoCalls({nonce: 0, firstCallId: 1, secondCallId: 2});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 3});
        bytes32 createdCommitment = _commitment(created[0]);
        _constrainCall({data: consumed[0], callIndex: 0, afterTags: _noTags(), beforeTags: _tags(createdCommitment)});
        _constrainCall({data: consumed[0], callIndex: 1, afterTags: _tags(createdCommitment), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(ForwarderCallOrdering.CyclicOrderingConstraints.selector);
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_a_call_carrying_resource_that_must_run_after_itself() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 2});
        _constrain({data: created[0], afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(ForwarderCallOrdering.CyclicOrderingConstraints.selector);
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_constraints_that_form_a_cycle() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _callCarryingResource({nonce: 1, callId: 2});
        _constrain({data: consumed[0], afterTags: _tags(_commitment(created[0])), beforeTags: _noTags()});
        _constrain({data: created[0], afterTags: _tags(_nullifier(consumed[0])), beforeTags: _noTags()});

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert(ForwarderCallOrdering.CyclicOrderingConstraints.selector);
        _mockPa.execute(txn);
    }

    /// @dev A constraint can name a resource of a later action. The calls of an action run together, so the whole
    /// second action runs first.
    function test_execute_orders_the_calls_across_actions() public {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](2);
        resourceLists[0] = TxGen.ResourceLists({
            consumed: _callCarryingResource({nonce: 0, callId: 1}),
            created: _callCarryingResource({nonce: 1, callId: 2})
        });
        resourceLists[1] = TxGen.ResourceLists({
            consumed: _callCarryingResource({nonce: 2, callId: 3}),
            created: _callCarryingResource({nonce: 3, callId: 4})
        });
        _constrain({
            data: resourceLists[0].consumed[0],
            afterTags: _tags(_commitment(resourceLists[1].created[0])),
            beforeTags: _noTags()
        });

        _assertCallOrder({txn: vm.transaction(_mockVerifier, resourceLists), expected: _callIds(3, 4, 1, 2)});
    }

    /// @dev The call of the second action must run after the first call of the first action and before its second
    /// call. The calls of an action run together, so no order meets these constraints.
    function test_execute_reverts_on_constraints_that_interleave_the_calls_of_two_actions() public {
        TxGen.ResourceLists[] memory resourceLists = new TxGen.ResourceLists[](2);
        resourceLists[0] = TxGen.ResourceLists({
            consumed: _callCarryingResource({nonce: 0, callId: 1}),
            created: _callCarryingResource({nonce: 1, callId: 2})
        });
        resourceLists[1] = TxGen.ResourceLists({
            consumed: _callCarryingResource({nonce: 2, callId: 3}), created: _exampleResourceAndEmptyAppData({nonce: 3})
        });
        _constrain({
            data: resourceLists[1].consumed[0],
            afterTags: _tags(_nullifier(resourceLists[0].consumed[0])),
            beforeTags: _tags(_commitment(resourceLists[0].created[0]))
        });

        IProtocolAdapter.Transaction memory txn = vm.transaction(_mockVerifier, resourceLists);

        vm.expectRevert(ForwarderCallOrdering.CyclicOrderingConstraints.selector);
        _mockPa.execute(txn);
    }

    /// @dev A swap needs the unwrap (created) before the generic call and the wrap (both consumed). The two lists
    /// run the consumed resources first, so only the constraints set this order.
    function test_execute_runs_the_calls_of_a_swap_in_the_order_of_their_constraints() public {
        TxGen.ResourceAndAppData[] memory consumed = new TxGen.ResourceAndAppData[](3);
        consumed[0] = _exampleResourceAndEmptyAppData({nonce: 0})[0]; // the sold tokens
        consumed[1] = _callCarryingResource({nonce: 1, callId: 10})[0]; // the generic call
        consumed[2] = _callCarryingResource({nonce: 2, callId: 20})[0]; // the wrap of the bought tokens

        TxGen.ResourceAndAppData[] memory created = new TxGen.ResourceAndAppData[](3);
        created[0] = _callCarryingResource({nonce: 3, callId: 30})[0]; // the unwrap of the sold tokens
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
        consumed[0] = _callCarryingResource({nonce: 0, callId: 1})[0];
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

    /// @dev The call checks that the root after the commitment of the created resource is historical. The protocol
    /// adapter must store this root before it executes the calls.
    function test_execute_runs_the_calls_after_storing_the_commitment_tree_root() public {
        TxGen.ResourceAndAppData[] memory consumed = _callCarryingResource({nonce: 0, callId: 1});
        TxGen.ResourceAndAppData[] memory created = _exampleResourceAndEmptyAppData({nonce: 1});

        CommitmentTreeMock commitmentTree = CommitmentTreeMock(
            address(
                new ERC1967Proxy(address(new CommitmentTreeMock()), abi.encodeCall(CommitmentTreeMock.initialize, ()))
            )
        );
        assertEq(
            commitmentTree.latestCommitmentTreeRoot(),
            _mockPa.latestCommitmentTreeRoot(),
            "the protocol adapter and the separate commitment tree start with the same root"
        );
        bytes32 newRoot = commitmentTree.addCommitment(_commitment(created[0]));

        consumed[0].appData.externalPayload[0].blob = _callBlob({
            untrustedForwarder: address(_fwd),
            input: abi.encode(address(_mockPa), abi.encodeCall(_mockPa.isCommitmentTreeRootHistorical, (newRoot))),
            expectedOutput: abi.encode(true)
        });

        _mockPa.execute(_transaction(consumed, created));
    }

    /// @dev The old encoding has no tag lists, so the words where the protocol adapter reads their offsets belong to
    /// the input of the call.
    function test_execute_reverts_on_a_forwarder_call_in_the_encoding_without_tag_lists() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created =
            _exampleCallCarryingResourceAndAppData({nonce: 1, fwdList: _fwdList});
        created[0].appData.externalPayload[0].blob = abi.encode(address(_fwd), _input, EXPECTED_OUTPUT);

        IProtocolAdapter.Transaction memory txn = _transaction(consumed, created);

        vm.expectRevert();
        _mockPa.execute(txn);
    }

    function test_execute_reverts_on_a_forwarder_call_whose_after_tags_offset_is_out_of_bounds() public {
        TxGen.ResourceAndAppData[] memory consumed = _exampleResourceAndEmptyAppData({nonce: 0});
        TxGen.ResourceAndAppData[] memory created =
            _exampleCallCarryingResourceAndAppData({nonce: 1, fwdList: _fwdList});
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

    /// @notice Builds a resource that carries one forwarder call. The call increments `callId`, so that the output
    /// of each call names it.
    function _callCarryingResource(uint256 nonce, uint256 callId)
        private
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        data = _exampleCallCarryingResourceAndAppData({nonce: nonce, fwdList: _fwdList});
        data[0].appData.externalPayload[0].blob = _callBlob({
            untrustedForwarder: address(_fwd),
            input: abi.encode(_fwdTarget, abi.encodeCall(ForwarderTargetExample.increment, (callId))),
            expectedOutput: abi.encode(callId + 1)
        });
    }

    /// @dev Consumed resource 1 carries call 1, created resource 0 carries call 2, and the other two carry no calls.
    function _resourcesWithAndWithoutCalls()
        private
        view
        returns (TxGen.ResourceAndAppData[] memory consumed, TxGen.ResourceAndAppData[] memory created)
    {
        consumed = new TxGen.ResourceAndAppData[](2);
        consumed[0] = _exampleResourceAndEmptyAppData({nonce: 0})[0];
        consumed[1] = _callCarryingResource({nonce: 1, callId: 1})[0];
        created = new TxGen.ResourceAndAppData[](2);
        created[0] = _callCarryingResource({nonce: 2, callId: 2})[0];
        created[1] = _exampleResourceAndEmptyAppData({nonce: 3})[0];
    }

    /// @notice Builds a resource that carries two forwarder calls, which increment `firstCallId` and `secondCallId`.
    function _callCarryingResourceWithTwoCalls(uint256 nonce, uint256 firstCallId, uint256 secondCallId)
        private
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        address[] memory fwdList = new address[](2);
        fwdList[0] = _fwd;
        fwdList[1] = _fwd;
        data = _exampleCallCarryingResourceAndAppData({nonce: nonce, fwdList: fwdList});
        data[0].appData.externalPayload[0].blob =
        _callCarryingResource({nonce: nonce, callId: firstCallId})[0].appData.externalPayload[0].blob;
        data[0].appData.externalPayload[1].blob =
        _callCarryingResource({nonce: nonce, callId: secondCallId})[0].appData.externalPayload[0].blob;
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

    function _tags(bytes32 tag) private pure returns (bytes32[] memory tags) {
        tags = new bytes32[](1);
        tags[0] = tag;
    }

    function _noTags() private pure returns (bytes32[] memory tags) {
        tags = new bytes32[](0);
    }
}
