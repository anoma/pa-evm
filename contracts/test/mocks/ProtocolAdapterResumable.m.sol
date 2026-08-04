// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";

import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";

/// @notice A mock protocol adapter upgrade implementation to that resumes operations by unpausing.
/// @custom:oz-upgrades-from ProtocolAdapter
/// @custom:oz-upgrades-unsafe-allow missing-initializer
contract ProtocolAdapterResumableMock is ProtocolAdapter {
    /// @custom:oz-upgrades-unsafe-allow constructor state-variable-immutable
    constructor(RiscZeroVerifierRouter riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector)
        ProtocolAdapter(riscZeroVerifierRouter, riscZeroVerifierSelector)
    {}

    /// @notice Reinitializes the contract after the upgrade, lifting the pause to resume transaction execution.
    /// @dev Passed as the `upgradeToAndCall` data so it runs atomically with the upgrade.
    function reinitialize() external reinitializer(2) onlyOwner {
        _unpause();
    }
}
