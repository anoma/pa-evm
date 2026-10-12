// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {MigrateProtocolAdapterState} from "../../script/migration/MigrateProtocolAdapterState.s.sol";

/// @notice The migration run, reading the given deployments instead of the records. The records are compiled in, so
/// they name no test deployment. A zero address stands for a deployment the records lack.
contract MigrateProtocolAdapterStateMock is MigrateProtocolAdapterState {
    address internal immutable _LEGACY_PROTOCOL_ADAPTER;
    address internal immutable _PROXY;

    constructor(address legacyProtocolAdapter, address proxy) {
        _LEGACY_PROTOCOL_ADAPTER = legacyProtocolAdapter;
        _PROXY = proxy;
    }

    function _recordedDeployments(bool) internal view override returns (address legacyProtocolAdapter, address proxy) {
        (legacyProtocolAdapter, proxy) = (_LEGACY_PROTOCOL_ADAPTER, _PROXY);
    }
}
