// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ProposeProtocolAdapterPause} from "../../script/production/ProposeProtocolAdapterPause.s.sol";

/// @notice The pause proposal, acting on the given proxy instead of the recorded one. The records are compiled in, so they
/// name no test deployment.
contract ProposeProtocolAdapterPauseMock is ProposeProtocolAdapterPause {
    address internal immutable _PROXY;

    constructor(address proxy) {
        _PROXY = proxy;
    }

    function _recordedProxy(bool) internal view override returns (address proxy) {
        proxy = _PROXY;
    }
}
