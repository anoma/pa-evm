// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title StagingScript
/// @author Anoma Foundation, 2025
/// @notice The base of the scripts acting on the staging environment protocol adapter proxy, which the staging proxy
/// owner drives directly.
/// @custom:security-contact security@anoma.foundation
abstract contract StagingScript is Script {
    /// @notice Thrown if the proxy is not a staging deployment, i.e. not owned by the staging proxy owner.
    error NotAStagingDeployment(address proxy);

    /// @notice Thrown if the sender is not the proxy owner.
    error UnauthorizedSender(address sender);

    /// @notice Checks that the proxy belongs to the staging environment and that the sender owns it.
    /// @param proxy The staging environment protocol adapter proxy to act on.
    function _checkSenderAuthorization(address proxy) internal {
        address owner = ProtocolAdapter(proxy).owner();
        require(owner == new DeployProtocolAdapterProxy().PROXY_OWNER_STAGING(), NotAStagingDeployment(proxy));
        require(msg.sender == owner, UnauthorizedSender(msg.sender));
    }
}
