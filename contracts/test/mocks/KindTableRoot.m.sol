// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {KindTableRoot} from "../../src/state/KindTableRoot.sol";

contract KindTableRootMock is KindTableRoot {
    function initialize() external initializer {
        __KindTableRoot_init();
    }

    function isKindTableRootAccepted(bytes32 kindTableRoot) external view returns (bool isAccepted) {
        isAccepted = _isKindTableRootAccepted(kindTableRoot);
    }

    // The mock lets anyone set the root; `ProtocolAdapter` allows only the owner.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeKindTableRootChange() internal override {}
}
