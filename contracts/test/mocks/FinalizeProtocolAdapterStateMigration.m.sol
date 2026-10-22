// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {
    FinalizeProtocolAdapterStateMigration
} from "../../script/migration/FinalizeProtocolAdapterStateMigration.s.sol";

/// @notice The completion run, reading the given deployments instead of the records. The records are compiled in, so
/// they name no test deployment. A zero address stands for a deployment the records lack.
contract FinalizeProtocolAdapterStateMigrationMock is FinalizeProtocolAdapterStateMigration {
    address internal immutable _LEGACY_PROTOCOL_ADAPTER;
    address internal immutable _PROXY;

    constructor(address legacyProtocolAdapter, address proxy) {
        _LEGACY_PROTOCOL_ADAPTER = legacyProtocolAdapter;
        _PROXY = proxy;
    }

    function _recordedLegacyProtocolAdapter() internal view override returns (address legacyProtocolAdapter) {
        legacyProtocolAdapter = _LEGACY_PROTOCOL_ADAPTER;
    }

    function _recordedProxy(bool) internal view override returns (address proxy) {
        proxy = _PROXY;
    }
}
