// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LibString} from "solady-0.1.26/src/utils/LibString.sol";

import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {DeploymentsFixture} from "../../fixtures/DeploymentsFixture.sol";
import {SafeFixture} from "../../fixtures/SafeFixture.sol";

/// @notice Checks the production deployments recorded in `deployments.json`. The genesis check needs no chain access
/// and holds everywhere. That the environment runs this source, as a release owned by a Safe, holds only while the
/// deployments are verified, because the source leads them between deployments.
contract DeploymentsProductionTest is DeploymentsFixture, SafeFixture {
    using LibString for *;

    /// @notice Skips the test unless the production deployments are verified against this source.
    modifier onlyProduction() {
        vm.skip(!vm.envOr("VERIFY_PRODUCTION_DEPLOYMENTS", false), "VERIFY_PRODUCTION_DEPLOYMENTS is not set");
        _;
    }

    function test_recorded_deployments_use_the_environment_salt() public {
        _expectGenesisDeployments({isProduction: true});
    }

    function test_recorded_deployments_run_the_source_implementation() public onlyProduction {
        _expectSourceImplementations({isProduction: true});
    }

    function test_recorded_deployments_run_a_release_version_and_are_safe_owned() public onlyProduction {
        Deployment[] memory deployments = _recordedDeployments({isProduction: true});

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            address recordedProxy = deployments[i].proxy.addr;
            string memory context = _deploymentContext({isProduction: true, chainId: deployments[i].chainId});

            assertFalse(
                _hasPrereleaseSuffix(ProtocolAdapter(recordedProxy).VERSION().toSmallString()),
                string.concat(context, ": version is a prerelease")
            );
            assertTrue(
                _isSafe(ProtocolAdapter(recordedProxy).owner()),
                string.concat(context, ": proxy is not owned by a Safe")
            );
        }
    }

    /// @notice Returns whether a version carries a prerelease suffix (e.g. `2.0.0-rc.1`).
    /// @param version The version to check.
    /// @return hasSuffix Whether the version carries a prerelease suffix.
    function _hasPrereleaseSuffix(bytes32 version) private pure returns (bool hasSuffix) {
        for (uint256 i = 0; i < 32 && version[i] != 0; ++i) {
            if (version[i] == "-") return true;
        }
    }
}
