// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.1/src/Test.sol";

import {DeployProtocolAdapterProxy} from "../script/DeployProtocolAdapterProxy.s.sol";

contract DeployProtocolAdapterProxyTest is Test {
    struct TestCase {
        string name;
    }

    // forge-lint: disable-next-line(mixed-case-function)
    function tableNetworksTest_DeployProtocolAdapterProxy_test_deployment_succeeds_on_all_supported_networks(TestCase memory network)
        public
    {
        vm.selectFork(vm.createFork(network.name));

        new DeployProtocolAdapterProxy().run({isTestDeployment: true, emergencyStopCaller: msg.sender});
    }

    // forge-lint: disable-next-line(mixed-case-function)
    function tableNetworksTest_DeployProtocolAdapterProxy_prod_deployment_succeeds_on_all_supported_networks(TestCase memory network)
        public
    {
        vm.selectFork(vm.createFork(network.name));

        new DeployProtocolAdapterProxy().run({isTestDeployment: false, emergencyStopCaller: msg.sender});
    }

    function fixtureNetwork() public pure returns (TestCase[] memory network) {
        network = new TestCase[](8);
        network[0] = TestCase({name: "sepolia"});
        network[1] = TestCase({name: "mainnet"});

        network[2] = TestCase({name: "arbitrum-sepolia"});
        network[3] = TestCase({name: "arbitrum"});

        network[4] = TestCase({name: "base-sepolia"});
        network[5] = TestCase({name: "base"});

        network[6] = TestCase({name: "optimism-sepolia"});
        network[7] = TestCase({name: "optimism"});

        return network;
    }
}
