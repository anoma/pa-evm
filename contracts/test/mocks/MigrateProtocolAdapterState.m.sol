// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {MigrateProtocolAdapterState} from "../../script/migration/MigrateProtocolAdapterState.s.sol";

/// @notice The migration run, reading the given deployments instead of the records. The records are compiled in, so
/// they name no test deployment. A zero address stands for a deployment the records lack.
contract MigrateProtocolAdapterStateMock is MigrateProtocolAdapterState {
    address internal immutable _PROTOCOL_ADAPTER_V1;
    address internal immutable _PROXY;

    constructor(address protocolAdapterV1, address proxy) {
        _PROTOCOL_ADAPTER_V1 = protocolAdapterV1;
        _PROXY = proxy;
    }

    function _recordedDeployments(bool) internal view override returns (address protocolAdapterV1, address proxy) {
        (protocolAdapterV1, proxy) = (_PROTOCOL_ADAPTER_V1, _PROXY);
    }
}
