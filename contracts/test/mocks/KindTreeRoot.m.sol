// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {KindTreeRoot} from "../../src/state/KindTreeRoot.sol";

contract KindTreeRootMock is KindTreeRoot {
    function initialize() external initializer {
        __KindTreeRoot_init();
    }

    function isKindTreeRootAccepted(bytes32 kindTreeRoot) external view returns (bool isAccepted) {
        isAccepted = _isKindTreeRootAccepted(kindTreeRoot);
    }

    // The mock lets anyone set the root; `ProtocolAdapter` allows only the owner.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeKindTreeRootChange() internal override {}
}
