// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {LogicRefRegistryMock} from "../mocks/LogicRefRegistry.m.sol";

contract LogicRefRegistryInitializationTest is Test {
    LogicRefRegistryMock internal _logicRefRegistry;

    constructor() {
        _logicRefRegistry = _deployLogicRefRegistryMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_logicRefRegistry));
        _logicRefRegistry.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        LogicRefRegistryMock directMock = new LogicRefRegistryMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_has_no_non_active_logic_refs() public view {
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 0, "no logic ref should be non-active");
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployLogicRefRegistryMock() internal returns (LogicRefRegistryMock mock) {
        mock = LogicRefRegistryMock(
            address(
                new ERC1967Proxy(
                    address(new LogicRefRegistryMock()), abi.encodeCall(LogicRefRegistryMock.initialize, ())
                )
            )
        );
    }
}
