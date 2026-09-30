// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";

import {LogicRefDenylistMock} from "../mocks/LogicRefDenylist.m.sol";

contract LogicRefDenylistInitializationTest is Test {
    LogicRefDenylistMock internal _denylist;

    constructor() {
        _denylist = _deployLogicRefDenylistMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_denylist));
        _denylist.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        LogicRefDenylistMock directMock = new LogicRefDenylistMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_starts_with_an_empty_denylist() public view {
        assertEq(_denylist.deniedLogicRefCount(), 0, "the denylist should start empty");
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployLogicRefDenylistMock() internal returns (LogicRefDenylistMock mock) {
        mock = LogicRefDenylistMock(
            address(
                new ERC1967Proxy(
                    address(new LogicRefDenylistMock()), abi.encodeCall(LogicRefDenylistMock.initialize, ())
                )
            )
        );
    }
}
