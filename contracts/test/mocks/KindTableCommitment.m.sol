// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {KindTableCommitment} from "../../src/state/KindTableCommitment.sol";

contract KindTableCommitmentMock is KindTableCommitment {
    function initialize() external initializer {
        __KindTableCommitment_init();
    }

    function setKindTableCommitment(bytes32 newKindTableCommitment) external {
        _setKindTableCommitment(newKindTableCommitment);
    }
}
