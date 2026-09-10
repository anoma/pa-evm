#!/usr/bin/env bash
# Generates the Solidity mirror of the recorded protocol adapter deployments.
#
# Reads the deployment records, the single source of truth, and writes them as a
# library the contracts package can read without leaving its own directory. Run
# it through `just contracts-gen-deployments`; CI reruns it and fails on a diff.
#
# The records store checksummed addresses, which Solidity address literals
# require. A wrong checksum fails the bindings tests and the contract build, so
# this script passes them through unchanged.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
records="${root}/crates/bindings/deployments.json"
output="${root}/contracts/generated/RecordedDeployments.sol"

# Emits the statements that fill the `deployments` array of one environment.
entries() {
    jq --exit-status --raw-output --arg environment "$1" '
        .[$environment] as $records
        | "        deployments = new Deployment[](\($records | length));",
          ( $records
            | to_entries[]
            | "        deployments[\(.key)] = Deployment({",
              "            chainId: \(.value.chainId),",
              "            proxy: Proxy({",
              "                addr: \(.value.proxy.address),",
              "                initialImplementation: \(.value.proxy.initialImplementation),",
              "                initializerData: hex\"\(.value.proxy.initializerData[2:])\",",
              "                creationCode: hex\"\(.value.proxy.creationCode[2:])\"",
              "            })",
              "        });"
          )
    ' "$records"
}

{
    cat <<'SOLIDITY'
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title RecordedDeployments
/// @author Anoma Foundation, 2026
/// @notice The protocol adapter deployments each environment records.
/// @dev Generated from `crates/bindings/deployments.json`, the single source of truth, which the bindings crate
/// embeds and checks against the chains. Do not edit by hand: run `just contracts-gen-deployments`, which CI reruns
/// and fails on any diff. The records live with the bindings because that crate publishes them; this library carries
/// them into Solidity so the contracts package reads nothing outside itself.
/// @custom:security-contact security@anoma.foundation
library RecordedDeployments {
    /// @notice A recorded protocol adapter proxy. The field `addr` holds the address, which is a reserved word.
    /// @dev The genesis fields pin how the address was derived: the creation code and the constructor arguments
    /// determine it together with the environment salt, and none of them can be read from the chain once the proxy is
    /// upgraded.
    struct Proxy {
        address addr;
        address initialImplementation;
        bytes initializerData;
        bytes creationCode;
    }

    /// @notice A recorded protocol adapter deployment.
    struct Deployment {
        uint256 chainId;
        Proxy proxy;
    }

    /// @notice Returns whether the environment records a deployment for the chain.
    /// @param isProduction Whether to check the production or the staging environment.
    /// @param chainId The chain ID to look for.
    /// @return recorded Whether the environment records a deployment for the chain.
    function isRecorded(bool isProduction, uint256 chainId) internal pure returns (bool recorded) {
        Deployment[] memory deployments = isProduction ? production() : staging();

        for (uint256 i = 0; i < deployments.length; ++i) {
            if (deployments[i].chainId == chainId) {
                return true;
            }
        }
    }

    /// @notice Returns the deployments the staging environment records.
    /// @return deployments The recorded staging deployments.
    function staging() internal pure returns (Deployment[] memory deployments) {
SOLIDITY

    entries staging

    cat <<'SOLIDITY'
    }

    /// @notice Returns the deployments the production environment records.
    /// @return deployments The recorded production deployments.
    function production() internal pure returns (Deployment[] memory deployments) {
SOLIDITY

    entries production

    cat <<'SOLIDITY'
    }
}
SOLIDITY
} >"$output"

(cd "${root}/contracts" && forge fmt generated/RecordedDeployments.sol)

echo "generated ${output}"
