// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {Parameters} from "../Parameters.sol";
import {RecordedDeploymentsScript} from "../RecordedDeploymentsScript.s.sol";

/// @title StagingScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts that the staging proxy owner executes directly on a recorded proxy it owns: the
/// staging proxy, or a production proxy before its migration completes. `IS_PRODUCTION` selects the record.
/// @custom:security-contact security@anoma.foundation
abstract contract StagingScript is RecordedDeploymentsScript {
    /// @notice Thrown if the proxy is not a staging deployment, i.e. not owned by the staging proxy owner.
    error NotAStagingDeployment(address proxy);

    /// @notice Returns the owner of the proxy, and reverts unless the proxy belongs to the staging environment.
    /// @param proxy The staging environment protocol adapter proxy to act on.
    /// @return owner The staging proxy owner, which sends the transactions.
    /// @dev The scripts broadcast as this owner, because with `--account` alone forge runs them as its default sender.
    function _stagingOwner(address proxy) internal view returns (address owner) {
        owner = ProtocolAdapter(proxy).owner();
        require(owner == Parameters.DEPLOYMENT_WALLET, NotAStagingDeployment(proxy));
    }
}
