// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {DeploymentRecords} from "./DeploymentRecords.sol";

/// @title RecordedDeployments
/// @author Anoma Foundation, 2026
/// @notice Reads the protocol adapter deployments each environment records, and the legacy protocol adapters they
/// copy the state from, from `crates/bindings/deployments.json`.
/// @dev The file is the only record. The bindings crate embeds it and checks it against the chains.
/// @custom:security-contact security@anoma.foundation
library RecordedDeployments {
    /// @notice A recorded protocol adapter proxy. The field `addr` holds the address, which is a reserved word.
    /// @dev The genesis fields pin how the address was derived: the creation code and the constructor arguments
    /// determine it together with the environment salt, and none of them can be read from the chain once the proxy is
    /// upgraded. The fields follow the alphabetical order of their keys, in which `parseJson` encodes them.
    struct Proxy {
        address addr;
        bytes creationCode;
        address initialImplementation;
        bytes initializerData;
    }

    /// @notice A recorded protocol adapter deployment.
    struct Deployment {
        uint256 chainId;
        Proxy proxy;
    }

    /// @notice A legacy protocol adapter, which a migrational proxy on the same chain copies the state from.
    struct LegacyDeployment {
        uint256 chainId;
        address protocolAdapter;
    }

    /// @notice The directory of the bindings package that holds the records, relative to the project root.
    string internal constant _DIRECTORY = "../crates/bindings";

    /// @notice The name of the bindings package.
    string internal constant _PACKAGE_NAME = "anoma-pa-evm-bindings";

    /// @notice Returns the protocol adapter proxy an environment records for a chain.
    /// @param isProduction Whether to read the production or the staging environment.
    /// @param chainId The chain ID to look for.
    /// @return proxy The recorded proxy, or the zero address if the environment records none for the chain.
    function protocolAdapterProxy(bool isProduction, uint256 chainId) internal view returns (address proxy) {
        proxy = DeploymentRecords.addressOf({
            json: _records(), key: DeploymentRecords.environment(isProduction), chainId: chainId, field: "proxy.address"
        });
    }

    /// @notice Returns the legacy protocol adapter of a chain.
    /// @param chainId The chain ID to look for.
    /// @return protocolAdapter The legacy protocol adapter, or the zero address if the chain ran none.
    function legacyProtocolAdapter(uint256 chainId) internal view returns (address protocolAdapter) {
        protocolAdapter =
            DeploymentRecords.addressOf({json: _records(), key: "legacy", chainId: chainId, field: "protocolAdapter"});
    }

    /// @notice Returns the deployments an environment records.
    /// @param isProduction Whether to return the production or the staging environment.
    /// @return recorded The recorded deployments.
    function deployments(bool isProduction) internal view returns (Deployment[] memory recorded) {
        bytes memory encoded =
            DeploymentRecords.entries({json: _records(), key: DeploymentRecords.environment(isProduction)});
        recorded = abi.decode(encoded, (Deployment[]));
    }

    /// @notice Returns the legacy protocol adapters, which `anoma-pa-evm-bindings` 2.3.0 records in `addresses.rs`,
    /// except Aurora, which is deprecated.
    /// @return recorded The recorded deployments of the legacy protocol adapters.
    function legacy() internal view returns (LegacyDeployment[] memory recorded) {
        bytes memory encoded = DeploymentRecords.entries({json: _records(), key: "legacy"});
        recorded = abi.decode(encoded, (LegacyDeployment[]));
    }

    /// @notice Returns the content of the records file.
    /// @return json The JSON text of the file.
    function _records() private view returns (string memory json) {
        json = DeploymentRecords.read({directory: _DIRECTORY, packageName: _PACKAGE_NAME});
    }
}
