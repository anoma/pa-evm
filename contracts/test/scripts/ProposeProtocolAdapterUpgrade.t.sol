// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";

import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {ProposeProtocolAdapterUpgrade} from "../../script/ProposeProtocolAdapterUpgrade.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../fixtures/SafeFixture.sol";

/// @notice Checks the upgrade proposal script against a Safe-owned proxy. Outside broadcast mode, the script
/// simulates the Safe executing the upgrade, so the proxy must end up on the new implementation.
contract ProposeProtocolAdapterUpgradeTest is RiscZeroRouterFixture, SafeFixture {
    address internal _owner;
    address internal _safe;
    address internal _proxy;

    function setUp() public {
        // Keep the script on the simulation branch regardless of the shell environment.
        vm.setEnv("SAFE_BROADCAST", "false");

        _deployRiscZeroRouter();

        _owner = makeAddr("safe owner");
        _safe = _deploySafe(_owner);

        (_proxy,) = new DeployProtocolAdapterProxy().run({isTestDeployment: true, initialOwner: _safe});
    }

    function test_run_upgrades_the_proxy() public {
        vm.expectEmit({checkTopic1: false, checkTopic2: false, checkTopic3: false, checkData: false, emitter: _proxy});
        emit IERC1967.Upgraded(address(0));

        address implementation = new ProposeProtocolAdapterUpgrade().run({proxy: _proxy, safe: _safe, proposer: _owner});

        assertEq(ProtocolAdapter(_proxy).implementation(), implementation, "proxy should run the new implementation");
    }
}
