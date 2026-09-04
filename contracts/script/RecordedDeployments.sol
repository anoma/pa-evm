// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title RecordedDeployments
/// @author Anoma Foundation, 2026
/// @notice The chains each environment already records a protocol adapter deployment for.
/// @dev Generated from `crates/bindings/deployments.json`, the single source of truth, which the bindings crate
/// embeds and checks against the chains. Do not edit by hand: run `just contracts-gen-deployments`, which CI
/// reruns and fails on any diff. The records live with the bindings because that crate publishes them, and this
/// library carries the part the deploy script needs so the contracts package reads nothing outside itself.
/// @custom:security-contact security@anoma.foundation
library RecordedDeployments {
    /// @notice Returns whether the environment already records a deployment for the chain.
    /// @param isProduction Whether to check the production or the staging environment.
    /// @param chainId The chain ID to look for.
    /// @return recorded Whether the environment records a deployment for the chain.
    function isRecorded(bool isProduction, uint256 chainId) internal pure returns (bool recorded) {
        uint256[] memory chainIds = isProduction ? productionChainIds() : stagingChainIds();

        for (uint256 i = 0; i < chainIds.length; ++i) {
            if (chainIds[i] == chainId) {
                return true;
            }
        }
    }

    /// @notice Returns the chains the staging environment records a deployment for.
    /// @return chainIds The recorded staging chain IDs.
    function stagingChainIds() internal pure returns (uint256[] memory chainIds) {
        chainIds = new uint256[](2);
        chainIds[0] = 11155111;
        chainIds[1] = 84532;
    }

    /// @notice Returns the chains the production environment records a deployment for.
    /// @return chainIds The recorded production chain IDs.
    function productionChainIds() internal pure returns (uint256[] memory chainIds) {
        chainIds = new uint256[](0);
    }
}
