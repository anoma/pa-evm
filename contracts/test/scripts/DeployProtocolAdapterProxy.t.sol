// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {SemVerLib} from "solady-0.1.26/src/utils/SemVerLib.sol";

import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../fixtures/SafeFixture.sol";

/// @notice Checks the proxy deploy script and checks the deployments recorded per environment in
/// `deployments.json` — the single source of truth for the deterministic deployments — against the chain state.
/// The test environment must run the source version exactly; the prod environment may trail it, must run a release
/// version, and must be owned by a Safe.
contract DeployProtocolAdapterProxyTest is RiscZeroRouterFixture, SafeFixture {
    /// @notice A protocol adapter deployment recorded in `deployments.json`.
    /// @dev Fields are ordered alphabetically so the struct decodes from `vm.parseJson`.
    struct Deployment {
        uint256 chainId;
        address proxy;
    }

    string internal constant _DEPLOYMENTS_PATH = "../crates/bindings/deployments.json";

    function test_run_succeeds_for_a_test_deployment() public {
        _deployRiscZeroRouter();

        new DeployProtocolAdapterProxy().run({isTestDeployment: true, initialOwner: msg.sender});
    }

    function test_run_succeeds_for_a_prod_deployment() public {
        _deployRiscZeroRouter();

        (address proxy, address implementation) =
            new DeployProtocolAdapterProxy().run({isTestDeployment: false, initialOwner: msg.sender});

        assertGt(implementation.code.length, 0, "implementation should be deployed");
        assertEq(ProtocolAdapter(proxy).implementation(), implementation, "proxy should delegate to the implementation");
        assertEq(ProtocolAdapter(proxy).owner(), msg.sender, "proxy should be initialized with the owner");
    }

    function test_recorded_test_deployments_run_the_source_version() public {
        Deployment[] memory deployments = _recordedDeployments(".test");

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            address proxy = deployments[i].proxy;

            assertGt(proxy.code.length, 0, "recorded test deployment missing on-chain");
            assertEq(
                ProtocolAdapter(proxy).getVersion(),
                _sourceVersion(),
                "recorded test deployment must run the source version"
            );
        }
    }

    function test_recorded_prod_deployments_trail_the_source_version_and_are_safe_owned() public {
        Deployment[] memory deployments = _recordedDeployments(".prod");

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            address proxy = deployments[i].proxy;

            assertGt(proxy.code.length, 0, "recorded prod deployment missing on-chain");

            bytes32 deployedVersion = ProtocolAdapter(proxy).getVersion();
            assertLe(
                SemVerLib.cmp(deployedVersion, _sourceVersion()),
                0,
                "recorded prod deployment must not lead the source version"
            );
            assertFalse(_hasPrereleaseSuffix(deployedVersion), "recorded prod deployment must run a release version");

            assertTrue(_isSafe(ProtocolAdapter(proxy).owner()), "recorded prod deployment must be owned by a Safe");
        }
    }

    /// @notice Reads the deployments of an environment (`".test"` or `".prod"`) recorded in `deployments.json`.
    /// @return deployments The recorded deployments.
    function _recordedDeployments(string memory environment) internal view returns (Deployment[] memory deployments) {
        deployments = abi.decode(vm.parseJson(vm.readFile(_DEPLOYMENTS_PATH), environment), (Deployment[]));
    }

    /// @notice Selects a fork of the supported network with the provided chain ID.
    function _selectForkAt(uint256 chainId) internal {
        string memory networkName = _supportedNetworks[chainId];
        assertGt(bytes(networkName).length, 0, "recorded deployment on an unsupported network");

        vm.selectFork(vm.createFork(networkName));
    }

    /// @notice Returns the version of this source tree by constructing a fresh implementation on the fork.
    function _sourceVersion() internal returns (bytes32 sourceVersion) {
        SupportedNetworks.Data memory data = getRouterData();
        sourceVersion =
            new ProtocolAdapter(data.router, RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR).getVersion();
    }

    /// @notice Returns whether a version carries a prerelease suffix (e.g. `2.0.0-rc.1`).
    function _hasPrereleaseSuffix(bytes32 version) internal pure returns (bool hasSuffix) {
        for (uint256 i = 0; i < 32 && version[i] != 0; ++i) {
            if (version[i] == "-") return true;
        }
    }
}
