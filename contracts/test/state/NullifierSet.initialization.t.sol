// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.6.1/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.6.1/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.16.1/src/Test.sol";

import {NullifierSetMock} from "../mocks/NullifierSet.m.sol";

contract NullifierSetInitializationTest is Test {
    NullifierSetMock internal _nfSet;

    constructor() {
        _nfSet = _deployNullifierSetMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_nfSet));
        _nfSet.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        NullifierSetMock directMock = new NullifierSetMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployNullifierSetMock() internal returns (NullifierSetMock mock) {
        mock = NullifierSetMock(
            address(new ERC1967Proxy(address(new NullifierSetMock()), abi.encodeCall(NullifierSetMock.initialize, ())))
        );
    }
}
