// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ExecuteProtocolAdapterUpgrade} from "../../script/staging/ExecuteProtocolAdapterUpgrade.s.sol";

/// @notice The upgrade, acting on the given proxy instead of the recorded one. The records hold only the deployments on
/// the chains, not a test deployment.
contract ExecuteProtocolAdapterUpgradeMock is ExecuteProtocolAdapterUpgrade {
    address internal immutable _PROXY;

    constructor(address proxy) {
        _PROXY = proxy;
    }

    function _recordedProxy(bool) internal view override returns (address proxy) {
        proxy = _PROXY;
    }
}
