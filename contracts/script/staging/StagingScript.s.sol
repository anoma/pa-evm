// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std-1.16.2/src/Script.sol";

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {DeployProtocolAdapterProxy} from "../DeployProtocolAdapterProxy.s.sol";

/// @title StagingScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts acting on the staging environment protocol adapter proxy, which the staging proxy
/// owner drives directly.
/// @custom:security-contact security@anoma.foundation
abstract contract StagingScript is Script {
    /// @notice Thrown if the proxy is not a staging deployment, i.e. not owned by the staging proxy owner.
    error NotAStagingDeployment(address proxy);

    /// @notice Returns the owner of the proxy, and reverts unless the proxy belongs to the staging environment.
    /// @dev The scripts broadcast as this owner, because with `--account` alone forge runs them as its default sender.
    /// @param proxy The staging environment protocol adapter proxy to act on.
    /// @return owner The staging proxy owner, which sends the transactions.
    function _stagingOwner(address proxy) internal returns (address owner) {
        owner = ProtocolAdapter(proxy).owner();
        require(owner == new DeployProtocolAdapterProxy().PROXY_OWNER_STAGING(), NotAStagingDeployment(proxy));
    }
}
