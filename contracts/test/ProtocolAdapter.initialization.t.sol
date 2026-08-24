// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.2/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

contract ProtocolAdapterInitializationTest is Test {
    address internal constant _OWNER = address(uint160(1));

    RiscZeroVerifierRouter internal _router;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    RiscZeroMockVerifier internal _verifier;
    ProtocolAdapter internal _pa;

    function setUp() public {
        (_router, _emergencyStop, _verifier) = new DeployRiscZeroContractsMock().run();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR());

        _pa = ProtocolAdapter(
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );
    }

    function test_constructor_reverts_on_zero_risc_zero_verifier_router() public {
        bytes4 verifierSelector = _verifier.SELECTOR();

        vm.expectRevert(ProtocolAdapter.ZeroRiscZeroVerifierRouterNotAllowed.selector);
        new ProtocolAdapter(address(0), verifierSelector);
    }

    function test_constructor_reverts_on_zero_risc_zero_verifier_selector() public {
        vm.expectRevert(ProtocolAdapter.ZeroRiscZeroVerifierSelectorNotAllowed.selector);
        new ProtocolAdapter(address(_router), bytes4(0));
    }

    function test_initialize_reverts_on_vulnerable_risc_zero_verifier() public {
        ProtocolAdapter implementation = new ProtocolAdapter(address(_router), _verifier.SELECTOR());

        vm.prank(_emergencyStop.owner());
        _emergencyStop.estop();

        vm.expectRevert(ProtocolAdapter.RiscZeroVerifierStopped.selector);
        new ERC1967Proxy(address(implementation), abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)));
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_pa));
        _pa.initialize(_OWNER);
    }

    function test_initialize_reverts_on_implementation_contract() public {
        ProtocolAdapter implementation = new ProtocolAdapter(address(_router), _verifier.SELECTOR());

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(implementation));
        implementation.initialize(_OWNER);
    }
}
