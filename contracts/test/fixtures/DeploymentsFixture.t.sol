// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeploymentsFixture} from "./DeploymentsFixture.sol";

/// @notice Checks the version classification of the fixture, which the promotion gates rely on but only reach for the
/// versions that happen to be deployed.
contract DeploymentsFixtureTest is DeploymentsFixture {
    function test_isReleaseOrCandidate_accepts_a_release() public pure {
        assertTrue(_isReleaseOrCandidate("2.0.0"), "a version without a suffix should be accepted");
    }

    function test_isReleaseOrCandidate_accepts_a_release_candidate() public pure {
        assertTrue(_isReleaseOrCandidate("2.0.0-rc.1"), "a single-digit candidate should be accepted");
        assertTrue(_isReleaseOrCandidate("2.0.0-rc.12"), "a multi-digit candidate should be accepted");
    }

    function test_isReleaseOrCandidate_rejects_another_prerelease() public pure {
        assertFalse(_isReleaseOrCandidate("2.0.0-alpha.6"), "an alpha should be rejected");
        assertFalse(_isReleaseOrCandidate("2.0.0-beta.1"), "a beta should be rejected");
    }

    function test_isReleaseOrCandidate_rejects_a_candidate_without_a_number() public pure {
        assertFalse(_isReleaseOrCandidate("2.0.0-rc"), "a candidate without a number should be rejected");
        assertFalse(_isReleaseOrCandidate("2.0.0-rc."), "a candidate without digits should be rejected");
        assertFalse(_isReleaseOrCandidate("2.0.0-rc.x"), "a non-numeric candidate should be rejected");
    }
}
