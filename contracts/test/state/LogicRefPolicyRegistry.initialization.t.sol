// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefPolicyRegistry} from "../../src/interfaces/ILogicRefPolicyRegistry.sol";

import {LogicRefPolicyRegistryMock} from "../mocks/LogicRefPolicyRegistry.m.sol";

contract LogicRefPolicyRegistryInitializationTest is Test {
    LogicRefPolicyRegistryMock internal _registry;

    constructor() {
        _registry = _deployLogicRefPolicyRegistryMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_registry));
        _registry.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        LogicRefPolicyRegistryMock directMock = new LogicRefPolicyRegistryMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_starts_with_an_empty_registry() public view {
        assertEq(
            uint8(_registry.logicRefPolicy(bytes32(uint256(1)))),
            uint8(ILogicRefPolicyRegistry.LogicRefPolicy.Unrestricted)
        );
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployLogicRefPolicyRegistryMock() internal returns (LogicRefPolicyRegistryMock mock) {
        mock = LogicRefPolicyRegistryMock(
            address(
                new ERC1967Proxy(
                    address(new LogicRefPolicyRegistryMock()), abi.encodeCall(LogicRefPolicyRegistryMock.initialize, ())
                )
            )
        );
    }
}
