// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {LibPRNG} from "solady-0.1.26/src/utils/LibPRNG.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ForwarderCallOrder} from "../src/libs/ForwarderCallOrder.sol";

contract ForwarderCallOrderTest is Test {
    using LibPRNG for LibPRNG.PRNG;

    /// @notice Passes the payload in calldata, where the protocol adapter reads it.
    function hasOrderingConstraintsInCalldata(IProtocolAdapter.ExpirableBlob[] calldata externalPayload)
        external
        pure
        returns (bool constrained)
    {
        constrained = ForwarderCallOrder.hasOrderingConstraints(externalPayload);
    }

    /// @dev The offsets of the tag lists are the fourth and fifth words of the encoded call.
    function test_hasOrderingConstraints_reverts_on_a_blob_too_short_for_the_tag_list_offsets() public {
        IProtocolAdapter.ExpirableBlob[] memory externalPayload = _payload(new bytes(3 * 32));

        vm.expectRevert();
        this.hasOrderingConstraintsInCalldata(externalPayload);
    }

    function test_orderByEdges_reverts_on_edges_that_form_a_cycle() public {
        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _order({carrierCount: 3, predecessors: _list(0, 1, 2), successors: _list(1, 2, 0)});
    }

    function test_orderByEdges_reverts_on_an_edge_from_a_carrier_to_itself() public {
        vm.expectRevert(ForwarderCallOrder.CyclicConstraints.selector);
        _order({carrierCount: 2, predecessors: _list(1), successors: _list(1)});
    }

    function testFuzz_hasOrderingConstraints_agrees_with_the_lengths_of_the_tag_lists(
        uint8 afterTagCount,
        uint8 beforeTagCount
    ) public view {
        afterTagCount = uint8(bound(afterTagCount, 0, 3));
        beforeTagCount = uint8(bound(beforeTagCount, 0, 3));
        IProtocolAdapter.ExpirableBlob[] memory externalPayload = _payload(
            abi.encode(address(1), hex"0102", hex"03", new bytes32[](afterTagCount), new bytes32[](beforeTagCount))
        );

        assertEq(
            this.hasOrderingConstraintsInCalldata(externalPayload),
            afterTagCount + beforeTagCount != 0,
            "constrained if a tag list is not empty"
        );
    }

    function test_orderByEdges_keeps_the_order_of_carriers_without_edges() public pure {
        assertEq(_order({carrierCount: 3, predecessors: _list(), successors: _list()}), _list(0, 1, 2), "order");
    }

    function test_orderByEdges_puts_the_predecessor_of_an_edge_before_its_successor() public pure {
        assertEq(_order({carrierCount: 3, predecessors: _list(2), successors: _list(0)}), _list(1, 2, 0), "order");
    }

    /// @dev Once carrier 1 is in the order, carriers 0 and 2 both wait for no carrier, and 0 has the lower index.
    function test_orderByEdges_takes_the_ready_carrier_with_the_lowest_index() public pure {
        assertEq(_order({carrierCount: 3, predecessors: _list(1), successors: _list(0)}), _list(1, 0, 2), "order");
    }

    function test_orderByEdges_counts_each_of_two_equal_edges() public pure {
        assertEq(_order({carrierCount: 2, predecessors: _list(1, 1), successors: _list(0, 0)}), _list(1, 0), "order");
    }

    /// @dev The protocol adapter sizes the edge lists by the number of constraints, so their tail holds zeros that
    /// would read as an edge from carrier 0 to itself.
    function test_orderByEdges_reads_only_the_first_edge_count_entries() public pure {
        uint256[] memory orderedIndices = ForwarderCallOrder.orderByEdges({
            carrierCount: 2, predecessors: _list(1, 0), successors: _list(0, 0), edgeCount: 1
        });

        assertEq(orderedIndices, _list(1, 0), "order");
    }

    function test_orderByEdges_returns_an_empty_order_for_no_carriers() public pure {
        assertEq(_order({carrierCount: 0, predecessors: _list(), successors: _list()}).length, 0, "order length");
    }

    /// @dev Draws a rank for each carrier and adds edges that agree with the ranks, so that an order exists.
    function testFuzz_orderByEdges_puts_the_predecessor_of_every_edge_before_its_successor(
        uint8 carrierCount,
        uint256 seed
    ) public pure {
        carrierCount = uint8(bound(carrierCount, 1, 16));
        LibPRNG.PRNG memory prng = LibPRNG.PRNG({state: seed});

        uint256[] memory ranks = new uint256[](carrierCount);
        for (uint256 i = 0; i < carrierCount; ++i) {
            ranks[i] = i;
        }
        prng.shuffle(ranks);

        uint256 edgeCount = 2 * carrierCount;
        uint256[] memory predecessors = new uint256[](edgeCount);
        uint256[] memory successors = new uint256[](edgeCount);
        uint256 written = 0;
        for (uint256 e = 0; e < edgeCount; ++e) {
            uint256 first = prng.uniform(carrierCount);
            uint256 second = prng.uniform(carrierCount);
            if (ranks[first] != ranks[second]) {
                (predecessors[written], successors[written]) =
                    ranks[first] < ranks[second] ? (first, second) : (second, first);
                ++written;
            }
        }

        uint256[] memory orderedIndices = ForwarderCallOrder.orderByEdges({
            carrierCount: carrierCount, predecessors: predecessors, successors: successors, edgeCount: written
        });

        uint256[] memory positions = new uint256[](carrierCount);
        bool[] memory seen = new bool[](carrierCount);
        assertEq(orderedIndices.length, carrierCount, "every carrier is in the order");
        for (uint256 position = 0; position < carrierCount; ++position) {
            assertFalse(seen[orderedIndices[position]], "no carrier is in the order twice");
            seen[orderedIndices[position]] = true;
            positions[orderedIndices[position]] = position;
        }
        for (uint256 e = 0; e < written; ++e) {
            assertLt(positions[predecessors[e]], positions[successors[e]], "a predecessor comes before its successor");
        }
    }

    function _order(uint256 carrierCount, uint256[] memory predecessors, uint256[] memory successors)
        private
        pure
        returns (uint256[] memory orderedIndices)
    {
        assertEq(predecessors.length, successors.length, "one predecessor and one successor for each edge");
        orderedIndices = ForwarderCallOrder.orderByEdges({
            carrierCount: carrierCount,
            predecessors: predecessors,
            successors: successors,
            edgeCount: predecessors.length
        });
    }

    function _payload(bytes memory blob)
        private
        pure
        returns (IProtocolAdapter.ExpirableBlob[] memory externalPayload)
    {
        externalPayload = new IProtocolAdapter.ExpirableBlob[](1);
        externalPayload[0] =
            IProtocolAdapter.ExpirableBlob({deletionCriterion: IProtocolAdapter.DeletionCriterion.Never, blob: blob});
    }

    function _list() private pure returns (uint256[] memory list) {
        list = new uint256[](0);
    }

    function _list(uint256 a) private pure returns (uint256[] memory list) {
        list = new uint256[](1);
        list[0] = a;
    }

    function _list(uint256 a, uint256 b) private pure returns (uint256[] memory list) {
        list = new uint256[](2);
        list[0] = a;
        list[1] = b;
    }

    function _list(uint256 a, uint256 b, uint256 c) private pure returns (uint256[] memory list) {
        list = new uint256[](3);
        list[0] = a;
        list[1] = b;
        list[2] = c;
    }
}
