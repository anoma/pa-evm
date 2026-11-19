// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";

/// @notice The proxy deployment, finding no recorded proxy. The tests deploy a test proxy on the ID of a chain that the
/// records can hold.
contract DeployProtocolAdapterProxyMock is DeployProtocolAdapterProxy {
    function _recordedProxy(bool) internal pure override returns (address proxy) {
        proxy = address(0);
    }
}
