// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RecordedDeployments} from "../../generated/RecordedDeployments.sol";
import {MigrationalProtocolAdapter} from "../../src/MigrationalProtocolAdapter.sol";
import {RecordedDeploymentsScript} from "../RecordedDeploymentsScript.s.sol";

/// @title MigrationScript
/// @author Anoma Foundation, 2026
/// @notice The base of the scripts that move one chain's state from the stopped legacy protocol adapter into
/// the proxy. Both come from the recorded deployments, so `--rpc-url` alone names the chain.
/// @custom:security-contact security@anoma.foundation
abstract contract MigrationScript is RecordedDeploymentsScript {
    /// @notice Thrown if the chain records no legacy protocol adapter, i.e. it has no state to migrate.
    error LegacyProtocolAdapterNotRecorded(uint256 chainId);

    /// @notice Thrown if the migrational proxy copies its state from another legacy protocol adapter than the
    /// recorded one.
    error LegacyProtocolAdapterMismatch(address expected, address actual);

    /// @notice Returns the chain's recorded legacy protocol adapter and the environment's recorded proxy, and
    /// reverts unless both are recorded.
    /// @param isProduction Whether to read the production or the staging proxy.
    /// @return legacyProtocolAdapter The legacy protocol adapter, which holds the state.
    /// @return proxy The proxy of the protocol adapter, which receives it.
    function _configuration(bool isProduction) internal returns (address legacyProtocolAdapter, address proxy) {
        legacyProtocolAdapter = _recordedLegacyProtocolAdapter();
        require(legacyProtocolAdapter != address(0), LegacyProtocolAdapterNotRecorded(block.chainid));
        proxy = _requireRecordedProxy(isProduction);
    }

    /// @notice Returns the legacy protocol adapter the records name for the chain `--rpc-url` names.
    /// @return legacyProtocolAdapter The recorded legacy protocol adapter, or the zero address if the chain records
    /// none.
    function _recordedLegacyProtocolAdapter() internal view virtual returns (address legacyProtocolAdapter) {
        legacyProtocolAdapter = RecordedDeployments.legacyProtocolAdapter(block.chainid);
    }

    /// @notice Reverts unless the proxy copies its state from the recorded legacy protocol adapter. Only the
    /// migrational implementation has the getter, so the call also rejects any other proxy.
    /// @param legacyProtocolAdapter The legacy protocol adapter the records name.
    /// @param proxy The proxy of the protocol adapter.
    function _requireLegacyProtocolAdapter(address legacyProtocolAdapter, address proxy) internal view {
        address copiedFrom = MigrationalProtocolAdapter(proxy).getLegacyProtocolAdapter();
        require(
            copiedFrom == legacyProtocolAdapter,
            LegacyProtocolAdapterMismatch({expected: legacyProtocolAdapter, actual: copiedFrom})
        );
    }
}
