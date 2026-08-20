// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {NullifierSet} from "../../src/state/NullifierSet.sol";

contract NullifierSetMock is NullifierSet {
    function initialize() external initializer {
        __NullifierSet_init();
    }

    function addNullifier(bytes32 nullifier) external {
        _addNullifier(nullifier);
    }
}
