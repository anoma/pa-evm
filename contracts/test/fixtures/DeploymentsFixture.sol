// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SupportedNetworks} from "anoma-risc0-deployments-1.2.2/src/SupportedNetworks.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";
import {LibString} from "solady-0.1.26/src/utils/LibString.sol";

import {DeployProtocolAdapterImplementation} from "../../script/DeployProtocolAdapterImplementation.s.sol";
import {DeployProtocolAdapterProxy} from "../../script/DeployProtocolAdapterProxy.s.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";

/// @notice A test fixture providing the protocol adapter deployments recorded per environment in `deployments.json` —
/// the single source of truth for the deterministic deployments.
abstract contract DeploymentsFixture is SupportedNetworks, Test {
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

    /// @notice Checks that every recorded proxy sits at the address its genesis deployment determines under the
    /// environment salt — the check that the first deployment of an environment used the right salt.
    /// @param isProduction Whether to check the production or the staging environment.
    function _expectGenesisDeployments(bool isProduction) internal {
        bytes32 salt = isProduction
            ? new DeployProtocolAdapterProxy().PROXY_SALT_PRODUCTION()
            : new DeployProtocolAdapterProxy().PROXY_SALT_STAGING();

        Deployment[] memory deployments = _recordedDeployments(isProduction);

        for (uint256 i = 0; i < deployments.length; ++i) {
            ProxyData memory recordedProxy = deployments[i].proxy;

            bytes memory constructorArgs =
                abi.encode(recordedProxy.initialImplementation, recordedProxy.initializerData);
            bytes memory initCode = abi.encodePacked(recordedProxy.creationCode, constructorArgs);

            address expectedProxyAddress = vm.computeCreate2Address(salt, keccak256(initCode));

            assertEq(
                expectedProxyAddress,
                recordedProxy.addr,
                string.concat(
                    _deploymentContext({isProduction: isProduction, chainId: deployments[i].chainId}),
                    ": recorded proxy address differs"
                )
            );
        }
    }

    /// @notice Checks that every recorded proxy delegates to the implementation this source version predicts for its
    /// chain, which proves the environment runs this source. The record is not a term in the comparison — the chain
    /// answers what it runs.
    /// @param isProduction Whether to check the production or the staging environment.
    function _expectSourceImplementations(bool isProduction) internal {
        Deployment[] memory deployments = _recordedDeployments(isProduction);

        for (uint256 i = 0; i < deployments.length; ++i) {
            uint256 chainId = deployments[i].chainId;
            ProxyData memory proxy = deployments[i].proxy;
            string memory context = _deploymentContext({isProduction: isProduction, chainId: chainId});

            // Predicted before the fork is selected, because selecting one discards the script deployed here.
            (address sourceImplementation,) = new DeployProtocolAdapterImplementation().predict(chainId);

            _selectForkAt(chainId);
            assertGt(proxy.addr.code.length, 0, string.concat(context, ": deployment missing on-chain"));
            assertEq(
                ProtocolAdapter(proxy.addr).getImplementation(),
                sourceImplementation,
                string.concat(context, ": does not run the source implementation")
            );
        }
    }

    /// @notice Selects a fork of the supported network with the provided chain ID.
    /// @param chainId The chain ID of the supported network to fork.
    function _selectForkAt(uint256 chainId) internal {
        string memory networkName = _supportedNetworks[chainId];
        assertGt(bytes(networkName).length, 0, string.concat(chainId.toString(), ": unsupported network"));

        vm.selectFork(vm.createFork(networkName));
    }

    /// @notice Reads the deployments of an environment recorded in `deployments.json`.
    /// @param isProduction Whether to read the production or the staging environment.
    /// @return deployments The recorded deployments.
    function _recordedDeployments(bool isProduction) internal view returns (Deployment[] memory deployments) {
        string memory environment = string.concat(".", _environmentName(isProduction));

        deployments = abi.decode(vm.parseJson(vm.readFile(_DEPLOYMENTS_PATH), environment), (Deployment[]));
    }

    /// @notice Returns the name of an environment, which keys its deployments in `deployments.json`.
    /// @param isProduction Whether to name the production or the staging environment.
    /// @return name The environment name.
    function _environmentName(bool isProduction) internal pure returns (string memory name) {
        name = isProduction ? "production" : "staging";
    }

    /// @notice Returns the `<environment>, <chain ID>` prefix identifying a recorded deployment in assert messages.
    /// @param isProduction Whether the deployment belongs to the production or the staging environment.
    /// @param chainId The chain ID of the deployment.
    /// @return context The assert message prefix.
    function _deploymentContext(bool isProduction, uint256 chainId) internal pure returns (string memory context) {
        context = string.concat(_environmentName(isProduction), ", ", chainId.toString());
    }

    /// @notice Returns whether a version is a release, i.e. carries no prerelease suffix.
    /// @param version The version to check.
    /// @return isRelease Whether the version is a release.
    function _isRelease(string memory version) internal pure returns (bool isRelease) {
        isRelease = version.indexOf("-") == LibString.NOT_FOUND;
    }

    /// @notice Returns whether a version is a release candidate, i.e. carries an `-rc.<number>` prerelease suffix.
    /// Any other prerelease (`-alpha.1`, `-rc`, `-rc.x`) is not one.
    /// @param version The version to check.
    /// @return isReleaseCandidate Whether the version is a release candidate.
    function _isReleaseCandidate(string memory version) internal pure returns (bool isReleaseCandidate) {
        uint256 separator = version.indexOf("-");
        if (separator == LibString.NOT_FOUND) {
            return false;
        }

        string memory suffix = version.slice(separator + 1);
        if (!suffix.startsWith("rc.")) {
            return false;
        }

        string memory number = suffix.slice(3);
        isReleaseCandidate = bytes(number).length != 0 && number.is7BitASCII(LibString.DIGITS_7_BIT_ASCII);
    }
}
