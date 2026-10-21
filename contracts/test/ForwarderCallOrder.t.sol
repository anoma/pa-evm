// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ForwarderCallOrder} from "../src/libs/ForwarderCallOrder.sol";

contract ForwarderCallOrderTest is Test {
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

    function _payload(bytes memory blob)
        private
        pure
        returns (IProtocolAdapter.ExpirableBlob[] memory externalPayload)
    {
        externalPayload = new IProtocolAdapter.ExpirableBlob[](1);
        externalPayload[0] =
            IProtocolAdapter.ExpirableBlob({deletionCriterion: IProtocolAdapter.DeletionCriterion.Never, blob: blob});
    }
}
