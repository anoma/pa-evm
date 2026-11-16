// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Vm} from "forge-std-1.17.0/src/Vm.sol";

/// @title DeploymentRecords
/// @author Anoma Foundation, 2026
/// @notice Reads the `deployments.json` file of a bindings package at run time. Under each key, the file holds a list
/// of entries, and each entry names its chain in `chainId`.
/// @custom:security-contact security@anoma.foundation
library DeploymentRecords {
    /// @notice The cheatcode contract that reads and parses the files.
    Vm private constant _VM = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Thrown if the `Cargo.toml` in the directory of the records names another package than the expected one.
    /// @param directory The directory of the records.
    /// @param expectedPackageName The name of the expected package.
    /// @param packageName The name of the package in the directory.
    error RecordsOfAnotherPackage(string directory, string expectedPackageName, string packageName);

    /// @notice Returns the content of the records file of a package.
    /// @param directory The directory of the package, relative to the project root. `fs_permissions` in `foundry.toml`
    /// must allow reads of its `Cargo.toml` and its `deployments.json`.
    /// @param packageName The name of the package, as its `Cargo.toml` states it.
    /// @return json The JSON text of the file.
    /// @dev Forge resolves the directory against the root of the project that runs the reader. Another project can hold
    /// records of the same form at the same path, so the check of the package name keeps a reader from reading them.
    function read(string memory directory, string memory packageName) internal view returns (string memory json) {
        string memory foundName =
            _VM.parseTomlString(_readFile(string.concat(directory, "/Cargo.toml")), ".package.name");
        require(
            keccak256(bytes(foundName)) == keccak256(bytes(packageName)),
            RecordsOfAnotherPackage({directory: directory, expectedPackageName: packageName, packageName: foundName})
        );
        json = _readFile(string.concat(directory, "/deployments.json"));
    }

    /// @notice Returns the chain IDs of the entries under a key, in the order of the list.
    /// @param json The JSON text of the records.
    /// @param key The key of the list.
    /// @return ids The chain IDs.
    function chainIds(string memory json, string memory key) internal view returns (uint256[] memory ids) {
        uint256 count = 0;
        while (_VM.keyExistsJson(json, _entry(key, count))) {
            ++count;
        }

        ids = new uint256[](count);
        for (uint256 i = 0; i < count; ++i) {
            ids[i] = _VM.parseJsonUint(json, string.concat(_entry(key, i), ".chainId"));
        }
    }

    /// @notice Returns an address field of the entry under a key for a chain.
    /// @param json The JSON text of the records.
    /// @param key The key of the list.
    /// @param chainId The chain ID of the entry.
    /// @param field The path of the address field in the entry.
    /// @return value The address, or the zero address if the list holds no entry for the chain.
    function addressOf(string memory json, string memory key, uint256 chainId, string memory field)
        internal
        view
        returns (address value)
    {
        string memory filter = string.concat("[?(@.chainId == ", _VM.toString(chainId), ")]");
        string memory entry = string.concat("$.", key, filter);

        if (_VM.keyExistsJson(json, entry)) {
            value = _VM.parseJsonAddress(json, string.concat(entry, ".", field));
        }
    }

    /// @notice Returns the key under which the records list the deployments of an environment.
    /// @param isProduction Whether to return the key of the production or the staging environment.
    /// @return key The key of the list.
    function environment(bool isProduction) internal pure returns (string memory key) {
        key = isProduction ? "production" : "staging";
    }

    /// @notice Returns the ABI encoding of the list under a key, which the caller decodes into its entry type.
    /// @param json The JSON text of the records.
    /// @param key The key of the list.
    /// @return encoded The ABI-encoded list.
    /// @dev `parseJson` encodes the fields of an entry in the alphabetical order of their keys.
    function entries(string memory json, string memory key) internal pure returns (bytes memory encoded) {
        encoded = _VM.parseJson(json, string.concat("$.", key));
    }

    /// @notice Returns the content of a file.
    /// @param path The path of the file, relative to the project root.
    /// @return content The text of the file.
    function _readFile(string memory path) private view returns (string memory content) {
        // forge-lint: disable-next-line(unsafe-cheatcode) `fs_permissions` allows only reads, and only of the records and their `Cargo.toml`.
        content = _VM.readFile(path);
    }

    /// @notice Returns the path of an entry by its position in the list.
    /// @param key The key of the list.
    /// @param index The position of the entry.
    /// @return path The JSON path of the entry.
    function _entry(string memory key, uint256 index) private pure returns (string memory path) {
        path = string.concat(string.concat("$.", key, "["), _VM.toString(index), "]");
    }
}
