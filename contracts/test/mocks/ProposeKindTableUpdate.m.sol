// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ProposeKindTableUpdate} from "../../script/production/ProposeKindTableUpdate.s.sol";

/// @notice The kind tree root update proposal, acting on the given proxy instead of the recorded one. The records are compiled in, so they
/// name no test deployment.
contract ProposeKindTableUpdateMock is ProposeKindTableUpdate {
    address internal immutable _PROXY;

    constructor(address proxy) {
        _PROXY = proxy;
    }

    function _recordedProxy(bool) internal view override returns (address proxy) {
        proxy = _PROXY;
    }
}
