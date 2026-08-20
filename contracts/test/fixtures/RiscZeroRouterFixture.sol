// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeployRiscZeroContracts} from "anoma-risc0-deployments-1.2.1/script/DeployRiscZeroContracts.s.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

/// @notice A test fixture providing the RISC Zero stack locally instead of forking a network.
abstract contract RiscZeroRouterFixture is SupportedNetworks, Test {
    /// @notice Deploys the RISC Zero stack locally.
    /// @dev The deploy scripts resolve the router from the chain ID, so the local router — code and storage, which
    /// holds the registered verifier — is cloned onto the router address of a supported network.
    function _deployRiscZeroRouter() internal {
        (RiscZeroVerifierRouter router,,) = new DeployRiscZeroContracts().run({admin: msg.sender, guardian: msg.sender});

        vm.chainId(1); // mainnet
        SupportedNetworks.Data memory data = getRouterData();
        vm.etch(address(data.router), address(router).code);
        vm.copyStorage(address(router), address(data.router));
    }
}
