// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {KindTableCommitment} from "../../src/state/KindTableCommitment.sol";

contract KindTableCommitmentMock is KindTableCommitment {
    function initialize() external initializer {
        __KindTableCommitment_init();
    }

    function isKindTableCommitmentAccepted(bytes32 kindTableCommitment) external view returns (bool isAccepted) {
        isAccepted = _isKindTableCommitmentAccepted(kindTableCommitment);
    }

    // The mock lets anyone set the commitment; `ProtocolAdapter` allows only the owner.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeKindTableCommitmentChange() internal override {}
}
