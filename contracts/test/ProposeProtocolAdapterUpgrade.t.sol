// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC1967} from "@openzeppelin-contracts-5.7.0/interfaces/IERC1967.sol";
import {DeployRiscZeroContracts} from "anoma-risc0-deployments-1.2.1/script/DeployRiscZeroContracts.s.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {Safe as SafeSmartAccount} from "safe-smart-account-1.5.0/contracts/Safe.sol";
import {SafeProxy} from "safe-smart-account-1.5.0/contracts/proxies/SafeProxy.sol";

import {DeployProtocolAdapterProxy} from "../script/DeployProtocolAdapterProxy.s.sol";
import {ProposeProtocolAdapterUpgrade} from "../script/ProposeProtocolAdapterUpgrade.s.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

/// @notice Checks the upgrade proposal script against a Safe-owned proxy. Outside broadcast mode, the script
/// simulates the Safe executing the upgrade, so the proxy must end up on the new implementation.
contract ProposeProtocolAdapterUpgradeTest is SupportedNetworks, Test {
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

    function test_run_upgrades_the_proxy_for_a_test_deployment() public {
        _expectUpgrade({isTestDeployment: true});
    }

    function test_run_upgrades_the_proxy_for_a_deterministic_deployment() public {
        _expectUpgrade({isTestDeployment: false});
    }

    /// @notice Runs the proposal script and checks that the simulated Safe execution upgrades the proxy.
    function _expectUpgrade(bool isTestDeployment) private {
        vm.expectEmit({checkTopic1: false, checkTopic2: false, checkTopic3: false, checkData: false, emitter: _proxy});
        emit IERC1967.Upgraded(address(0));

        address implementation = new ProposeProtocolAdapterUpgrade()
            .run({isTestDeployment: isTestDeployment, proxy: _proxy, safe: _safe, proposer: _owner});

        assertEq(ProtocolAdapter(_proxy).implementation(), implementation, "proxy should run the new implementation");
    }

    /// @notice Deploys a Safe with a single owner and a threshold of one.
    function _deploySafe(address owner) private returns (address safe) {
        address[] memory owners = new address[](1);
        owners[0] = owner;

        safe = address(new SafeProxy(address(new SafeSmartAccount())));
        SafeSmartAccount(payable(safe))
            .setup({
            _owners: owners,
            _threshold: 1,
            to: address(0),
            data: "",
            fallbackHandler: address(0),
            paymentToken: address(0),
            payment: 0,
            paymentReceiver: payable(address(0))
        });
    }

    /// @notice Deploys the RISC Zero stack locally instead of forking a network.
    /// @dev The script resolves the router from the chain ID, so the local router — code and storage, which holds
    /// the registered verifier — is cloned onto the router address of a supported network.
    function _deployRiscZeroRouter() private {
        (RiscZeroVerifierRouter router,,) = new DeployRiscZeroContracts().run({admin: msg.sender, guardian: msg.sender});

        vm.chainId(1); // mainnet
        SupportedNetworks.Data memory data = getRouterData();
        vm.etch(address(data.router), address(router).code);
        vm.copyStorage(address(router), address(data.router));
    }
}
