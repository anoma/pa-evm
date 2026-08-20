// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeploymentsFixture} from "./DeploymentsFixture.sol";

/// @notice Checks the version classification of the fixture, which the promotion gates rely on but only reach for the
/// versions that happen to be deployed.
contract DeploymentsFixtureTest is DeploymentsFixture {
    function test_isRelease_accepts_a_version_without_a_suffix() public pure {
        assertTrue(_isRelease("2.0.0"), "a version without a suffix should be a release");
    }

    function test_isRelease_rejects_a_prerelease() public pure {
        assertFalse(_isRelease("2.0.0-rc.1"), "a candidate should not be a release");
        assertFalse(_isRelease("2.0.0-alpha.6"), "an alpha should not be a release");
    }

    function test_isReleaseCandidate_accepts_a_numbered_candidate() public pure {
        assertTrue(_isReleaseCandidate("2.0.0-rc.0"), "a zero-numbered candidate should be accepted");
        assertTrue(_isReleaseCandidate("2.0.0-rc.12"), "a multi-digit candidate should be accepted");
    }

    function test_isReleaseCandidate_rejects_another_prerelease() public pure {
        assertFalse(_isReleaseCandidate("2.0.0-alpha.6"), "an alpha should be rejected");
        assertFalse(_isReleaseCandidate("2.0.0-beta.1"), "a beta should be rejected");
    }

    function test_isReleaseCandidate_rejects_a_candidate_without_a_number() public pure {
        assertFalse(_isReleaseCandidate("2.0.0-rc"), "a candidate without a number should be rejected");
        assertFalse(_isReleaseCandidate("2.0.0-rc."), "a candidate without digits should be rejected");
        assertFalse(_isReleaseCandidate("2.0.0-rc.x"), "a non-numeric candidate should be rejected");
    }

    function test_isReleaseCandidate_rejects_a_version_without_a_suffix() public pure {
        assertFalse(_isReleaseCandidate("2.0.0"), "a release should not be a candidate");
    }
}
