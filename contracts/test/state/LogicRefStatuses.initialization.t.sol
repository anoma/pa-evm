// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {LogicRefStatusesMock} from "../mocks/LogicRefStatuses.m.sol";

contract LogicRefStatusesInitializationTest is Test {
    LogicRefStatusesMock internal _logicRefStatuses;

    constructor() {
        _logicRefStatuses = _deployLogicRefStatusesMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_logicRefStatuses));
        _logicRefStatuses.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        LogicRefStatusesMock directMock = new LogicRefStatusesMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_lists_no_logic_ref() public view {
        assertEq(_logicRefStatuses.listedLogicRefCount(), 0, "no logic ref should be listed");
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployLogicRefStatusesMock() internal returns (LogicRefStatusesMock mock) {
        mock = LogicRefStatusesMock(
            address(
                new ERC1967Proxy(
                    address(new LogicRefStatusesMock()), abi.encodeCall(LogicRefStatusesMock.initialize, ())
                )
            )
        );
    }
}
