// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.2/src/Test.sol";
import {SafeProxy} from "safe-smart-account-1.5.0/contracts/proxies/SafeProxy.sol";
import {Safe as SafeSmartAccount} from "safe-smart-account-1.5.0/contracts/Safe.sol";

/// @notice A test fixture providing a Safe deployment.
abstract contract SafeFixture is Test {
    /// @notice Deploys a Safe with a single owner and a threshold of one.
    function _deploySafe(address owner) internal returns (address safe) {
        address[] memory owners = new address[](1);
        owners[0] = owner;

        safe = address(new SafeProxy(address(new SafeSmartAccount())));
        SafeSmartAccount(payable(safe))
            .setup({
            _owners: owners,
            _threshold: 1,
            to: address(0),
            data: "",
            fallbackHandler: address(0),
            paymentToken: address(0),
            payment: 0,
            paymentReceiver: payable(address(0))
        });
    }
}
