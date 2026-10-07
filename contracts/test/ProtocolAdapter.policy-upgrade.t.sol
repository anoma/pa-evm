// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefPolicyRegistry} from "../src/interfaces/ILogicRefPolicyRegistry.sol";
import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {LogicRefPolicyRegistry} from "../src/state/LogicRefPolicyRegistry.sol";
import {LegacyLogicRefDenylistMock} from "./mocks/LegacyLogicRefDenylist.m.sol";

contract ProtocolAdapterPolicyUpgradeTest is Test {
    function test_upgrade_preserves_all_memberships_and_rejects_replay() public {
        LegacyLogicRefDenylistMock legacy = _legacy();
        ProtocolAdapter implementation = new ProtocolAdapter(address(1), bytes4(uint32(1)));
        for (uint256 i = 2; i > 0; --i) {
            vm.expectEmit(address(legacy));
            emit ILogicRefPolicyRegistry.LogicRefPolicyChanged(
                bytes32(i + 1),
                ILogicRefPolicyRegistry.LogicRefPolicy.Unrestricted,
                ILogicRefPolicyRegistry.LogicRefPolicy(i + 1)
            );
        }
        vm.expectEmit(address(legacy));
        emit ILogicRefPolicyRegistry.LogicRefPolicyChanged(
            bytes32(uint256(1)),
            ILogicRefPolicyRegistry.LogicRefPolicy.Unrestricted,
            ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied
        );
        legacy.upgradeToAndCall(
            address(implementation), abi.encodeCall(ILogicRefPolicyRegistry.initializeLogicRefPolicies, ())
        );
        ProtocolAdapter pa = ProtocolAdapter(address(legacy));
        assertEq(pa.owner(), address(this));
        _assertPolicies(pa);
        assertEq(pa.restrictedLogicRefCount(), 3);
        assertEq(pa.restrictedLogicRefAtIndex(0), bytes32(uint256(3)));
        assertEq(pa.restrictedLogicRefAtIndex(1), bytes32(uint256(2)));
        assertEq(pa.restrictedLogicRefAtIndex(2), bytes32(uint256(1)));
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        pa.initializeLogicRefPolicies();
    }

    function test_omitted_import_fails_closed_until_authorized_initialization() public {
        LegacyLogicRefDenylistMock legacy = _legacy();
        legacy.upgradeToAndCall(address(new ProtocolAdapter(address(1), bytes4(uint32(1)))), "");
        ProtocolAdapter pa = ProtocolAdapter(address(legacy));
        vm.expectRevert(LogicRefPolicyRegistry.LogicRefPoliciesNotInitialized.selector);
        pa.logicRefPolicy(bytes32(uint256(1)));
        vm.expectRevert(LogicRefPolicyRegistry.LogicRefPoliciesNotInitialized.selector);
        pa.restrictedLogicRefCount();
        vm.expectRevert(LogicRefPolicyRegistry.LogicRefPoliciesNotInitialized.selector);
        pa.restrictedLogicRefAtIndex(0);
        vm.expectRevert(LogicRefPolicyRegistry.LogicRefPoliciesNotInitialized.selector);
        pa.setLogicRefPolicies(new ILogicRefPolicyRegistry.PolicyUpdate[](0));
        IProtocolAdapter.Transaction memory txn;
        vm.expectRevert(LogicRefPolicyRegistry.LogicRefPoliciesNotInitialized.selector);
        pa.execute(txn);
        vm.expectRevert(LogicRefPolicyRegistry.LogicRefPoliciesNotInitialized.selector);
        pa.simulateExecute(txn, true);
        vm.prank(address(0xbad));
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, address(0xbad)));
        pa.initializeLogicRefPolicies();
        pa.initializeLogicRefPolicies();
        _assertPolicies(pa);
    }

    function _legacy() internal returns (LegacyLogicRefDenylistMock legacy) {
        legacy = LegacyLogicRefDenylistMock(
            address(
                new ERC1967Proxy(
                    address(new LegacyLogicRefDenylistMock()),
                    abi.encodeCall(LegacyLogicRefDenylistMock.initialize, (address(this)))
                )
            )
        );
        legacy.deny(bytes32(uint256(1)), false);
        legacy.deny(bytes32(uint256(3)), true);
        legacy.deny(bytes32(uint256(3)), false);
        legacy.deny(bytes32(uint256(2)), true);
    }

    function _assertPolicies(ProtocolAdapter pa) internal view {
        for (uint256 i = 0; i < 4; ++i) {
            assertEq(uint8(pa.logicRefPolicy(bytes32(i))), i);
        }
    }
}
