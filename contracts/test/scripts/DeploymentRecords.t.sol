// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {DeploymentRecords} from "../../script/DeploymentRecords.sol";
import {RecordedDeployments} from "../../script/RecordedDeployments.sol";

/// @notice Checks that a reader reads only the records of its own package.
contract DeploymentRecordsTest is Test {
    /// @notice A reader of another repository that reads `../crates/bindings` reaches the records of this project.
    function test_read_reverts_if_the_directory_holds_the_records_of_another_package() public {
        string memory otherPackageName = "other-bindings";

        vm.expectRevert(
            abi.encodeWithSelector(
                DeploymentRecords.RecordsOfAnotherPackage.selector,
                RecordedDeployments._DIRECTORY,
                otherPackageName,
                RecordedDeployments._PACKAGE_NAME
            )
        );
        DeploymentRecords.read({directory: RecordedDeployments._DIRECTORY, packageName: otherPackageName});
    }
}
