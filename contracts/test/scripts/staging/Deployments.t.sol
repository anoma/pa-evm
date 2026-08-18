// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ProtocolAdapter} from "../../../src/ProtocolAdapter.sol";
import {DeploymentsFixture} from "../../fixtures/DeploymentsFixture.sol";

/// @notice Checks the staging deployments recorded in `deployments.json`. The genesis check needs no chain access and
/// holds everywhere. That the environment runs this source holds only while the deployments are verified, because the
/// source leads them between deployments.
contract DeploymentsStagingTest is DeploymentsFixture {
    /// @notice Skips the test unless the staging deployments are verified against this source.
    modifier onlyStaging() {
        vm.skip(!vm.envOr("VERIFY_STAGING_DEPLOYMENTS", false), "VERIFY_STAGING_DEPLOYMENTS is not set");
        _;
    }

    function test_recorded_deployments_use_the_environment_salt() public {
        _expectGenesisDeployments({isProduction: false});
    }

    function test_recorded_deployments_run_the_source_implementation() public onlyStaging {
        _expectSourceImplementations({isProduction: false});
    }

    function test_recorded_deployments_run_a_release_or_release_candidate_version() public onlyStaging {
        Deployment[] memory deployments = _recordedDeployments({isProduction: false});

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            string memory version = ProtocolAdapter(deployments[i].proxy.addr).VERSION();
            string memory context = _deploymentContext({isProduction: false, chainId: deployments[i].chainId});

            assertTrue(
                _isRelease(version) || _isReleaseCandidate(version),
                string.concat(context, ": version is neither a release nor a release candidate: ", version)
            );
        }
    }
}
