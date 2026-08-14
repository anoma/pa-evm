// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {RiscZeroVerifierSelectors} from "anoma-risc0-deployments-1.2.1/src/RiscZeroVerifierSelectors.sol";
import {SupportedNetworks} from "anoma-risc0-deployments-1.2.1/src/SupportedNetworks.sol";
import {LibString} from "solady-0.1.26/src/utils/LibString.sol";
import {SemVerLib} from "solady-0.1.26/src/utils/SemVerLib.sol";

import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {RiscZeroRouterFixture} from "../fixtures/RiscZeroRouterFixture.sol";
import {SafeFixture} from "../fixtures/SafeFixture.sol";

/// @notice Checks the proxy deploy script and checks the deployments recorded per environment in
/// `deployments.json` — the single source of truth for the deterministic deployments — against the chain state.
/// Every recorded proxy must sit at the address its genesis deployment and environment salt determine; the staging
/// environment must run the source version exactly; the production environment may trail it, must run a release
/// version, and must be owned by a Safe. An environment without recorded deployments passes vacuously.
contract DeployProtocolAdapterProxyTest is RiscZeroRouterFixture, SafeFixture {
    using LibString for *;

    /// @notice A protocol adapter proxy recorded in `deployments.json`.
    /// @dev The genesis fields pin the first deployment: the ERC-1967 proxy creation code and constructor arguments
    /// determine the address together with the environment salt, and none of them can be recovered from the chain
    /// once the proxy is upgraded.
    struct ProxyData {
        address addr;
        bytes creationCode;
        address initialImplementation;
        bytes initializerData;
    }

    /// @notice A protocol adapter deployment recorded in `deployments.json`.
    /// @dev Fields are ordered alphabetically by their JSON key so the struct decodes from `vm.parseJson`, which
    /// encodes object values in that order — the Solidity names themselves are irrelevant.
    struct Deployment {
        uint256 chainId;
        ProxyData proxy;
    }

    string internal constant _DEPLOYMENTS_PATH = "../crates/bindings/deployments.json";

    function test_run_succeeds_for_a_staging_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: false});
    }

    function test_run_succeeds_for_a_production_deployment() public {
        _deployRiscZeroRouter();

        _expectDeployment({isProduction: true});
    }

    function test_run_deploys_distinct_proxies_sharing_the_implementation() public {
        _deployRiscZeroRouter();

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address stagingProxy, address stagingImplementation) = script.run({isProduction: false});
        (address productionProxy, address productionImplementation) = script.run({isProduction: true});

        assertNotEq(stagingProxy, productionProxy, "staging and production proxy addresses are equal");
        assertEq(stagingImplementation, productionImplementation, "staging and production implementations differ");
    }

    function test_recorded_staging_deployments_use_the_environment_salt() public {
        _expectGenesisDeployments({isProduction: false});
    }

    function test_recorded_production_deployments_use_the_environment_salt() public {
        _expectGenesisDeployments({isProduction: true});
    }

    function test_recorded_staging_deployments_at_the_source_version_are_reproducible() public {
        _expectReproducibleDeployments({isProduction: false});
    }

    function test_recorded_production_deployments_at_the_source_version_are_reproducible() public {
        _expectReproducibleDeployments({isProduction: true});
    }

    function test_recorded_staging_deployments_run_the_source_version() public {
        Deployment[] memory deployments = _recordedDeployments({isProduction: false});

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            address recordedProxy = deployments[i].proxy.addr;
            string memory context = _deploymentContext({isProduction: false, chainId: deployments[i].chainId});

            assertGt(recordedProxy.code.length, 0, string.concat(context, ": deployment missing on-chain"));
            assertEq(
                ProtocolAdapter(recordedProxy).VERSION(),
                _sourceVersion(),
                string.concat(context, ": version differs from the source version")
            );
        }
    }

    function test_recorded_production_deployments_trail_the_source_version_and_are_safe_owned() public {
        Deployment[] memory deployments = _recordedDeployments({isProduction: true});

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            address recordedProxy = deployments[i].proxy.addr;
            string memory context = _deploymentContext({isProduction: true, chainId: deployments[i].chainId});

            assertGt(recordedProxy.code.length, 0, string.concat(context, ": deployment missing on-chain"));

            bytes32 deployedVersion = ProtocolAdapter(recordedProxy).VERSION().toSmallString();
            assertLe(
                SemVerLib.cmp(deployedVersion, _sourceVersion().toSmallString()),
                0,
                string.concat(context, ": version leads the source version")
            );
            assertFalse(_hasPrereleaseSuffix(deployedVersion), string.concat(context, ": version is a prerelease"));

            assertTrue(
                _isSafe(ProtocolAdapter(recordedProxy).owner()),
                string.concat(context, ": proxy is not owned by a Safe")
            );
        }
    }

    /// @notice Selects a fork of the supported network with the provided chain ID.
    function _selectForkAt(uint256 chainId) internal {
        string memory networkName = _supportedNetworks[chainId];
        assertGt(bytes(networkName).length, 0, string.concat(chainId.toString(), ": unsupported network"));

        vm.selectFork(vm.createFork(networkName));
    }

    /// @notice Returns the version of this source tree by constructing a fresh implementation on the fork.
    function _sourceVersion() internal returns (string memory sourceVersion) {
        SupportedNetworks.Data memory data = getRouterData();
        sourceVersion =
            new ProtocolAdapter(address(data.router), RiscZeroVerifierSelectors._GROTH16_VERIFIER_SELECTOR).VERSION();
    }

    /// @notice Reads the deployments of an environment recorded in `deployments.json`.
    /// @return deployments The recorded deployments.
    function _recordedDeployments(bool isProduction) internal view returns (Deployment[] memory deployments) {
        string memory environment = string.concat(".", _environmentName(isProduction));

        deployments = abi.decode(vm.parseJson(vm.readFile(_DEPLOYMENTS_PATH), environment), (Deployment[]));
    }

    /// @notice Returns the name of an environment, which keys its deployments in `deployments.json`.
    function _environmentName(bool isProduction) internal pure returns (string memory name) {
        name = isProduction ? "production" : "staging";
    }

    /// @notice Returns the `<environment>, <chain ID>` prefix identifying a recorded deployment in assert messages.
    function _deploymentContext(bool isProduction, uint256 chainId) internal pure returns (string memory context) {
        context = string.concat(_environmentName(isProduction), ", ", chainId.toString());
    }

    /// @notice Returns whether a version carries a prerelease suffix (e.g. `2.0.0-rc.1`).
    function _hasPrereleaseSuffix(bytes32 version) internal pure returns (bool hasSuffix) {
        for (uint256 i = 0; i < 32 && version[i] != 0; ++i) {
            if (version[i] == "-") return true;
        }
    }

    /// @notice Checks that the deploy script still reproduces every recorded deployment running the source version.
    /// A deployment of an earlier version is skipped: its proxy address commits to the implementation it was
    /// initialized with, so the current source deploys a new proxy instead of reproducing it.
    function _expectReproducibleDeployments(bool isProduction) private {
        Deployment[] memory deployments = _recordedDeployments(isProduction);

        for (uint256 i = 0; i < deployments.length; ++i) {
            _selectForkAt(deployments[i].chainId);
            address recordedProxy = deployments[i].proxy.addr;
            string memory context = _deploymentContext({isProduction: isProduction, chainId: deployments[i].chainId});

            assertGt(recordedProxy.code.length, 0, string.concat(context, ": deployment missing on-chain"));

            if (keccak256(bytes(ProtocolAdapter(recordedProxy).VERSION())) == keccak256(bytes(_sourceVersion()))) {
                (address sourceProxy,) = new DeployProtocolAdapterProxy().run({isProduction: isProduction});

                assertEq(sourceProxy, recordedProxy, string.concat(context, ": recorded proxy address differs"));
            }
        }
    }

    /// @notice Runs the deploy script for the environment and checks that the proxy lands at the predicted
    /// deterministic address, delegates to a deployed implementation, and is initialized with the environment owner.
    function _expectDeployment(bool isProduction) private {
        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        (address proxy, address implementation) = script.run({isProduction: isProduction});

        address owner = isProduction ? script.PROXY_OWNER_PRODUCTION() : script.PROXY_OWNER_STAGING();
        address predicted = vm.computeCreate2Address(
            isProduction ? script.PROXY_SALT_PRODUCTION() : script.PROXY_SALT_STAGING(),
            keccak256(
                abi.encodePacked(
                    type(ERC1967Proxy).creationCode,
                    abi.encode(implementation, abi.encodeCall(ProtocolAdapter.initialize, (owner)))
                )
            )
        );

        string memory environment = _environmentName(isProduction);

        assertEq(proxy, predicted, string.concat(environment, ": proxy address differs from the prediction"));
        assertGt(implementation.code.length, 0, string.concat(environment, ": implementation is not deployed"));
        assertEq(
            ProtocolAdapter(proxy).implementation(),
            implementation,
            string.concat(environment, ": proxy does not delegate to the implementation")
        );
        assertEq(ProtocolAdapter(proxy).owner(), owner, string.concat(environment, ": proxy owner differs"));
    }

    /// @notice Checks that every recorded proxy sits at the address its genesis deployment determines under the
    /// environment salt — the check that the first deployment of an environment used the right salt.
    function _expectGenesisDeployments(bool isProduction) private {
        Deployment[] memory deployments = _recordedDeployments(isProduction);

        DeployProtocolAdapterProxy script = new DeployProtocolAdapterProxy();
        bytes32 salt = isProduction ? script.PROXY_SALT_PRODUCTION() : script.PROXY_SALT_STAGING();

        for (uint256 i = 0; i < deployments.length; ++i) {
            ProxyData memory proxy = deployments[i].proxy;

            bytes memory constructorArgs = abi.encode(proxy.initialImplementation, proxy.initializerData);
            string memory context = _deploymentContext({isProduction: isProduction, chainId: deployments[i].chainId});

            assertEq(
                vm.computeCreate2Address(salt, keccak256(abi.encodePacked(proxy.creationCode, constructorArgs))),
                proxy.addr,
                string.concat(context, ": recorded proxy address differs")
            );
        }
    }
}
