// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.6.1/access/Ownable.sol";
import {Pausable} from "@openzeppelin-contracts-5.6.1/utils/Pausable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.0/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.16.1/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.1/src/Options.sol";
import {UnsafeUpgrades, Upgrades} from "openzeppelin-foundry-upgrades-0.4.1/src/Upgrades.sol";
import {IRiscZeroVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/IRiscZeroVerifier.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {Transaction} from "../src/Types.sol";
import {TxGen} from "./libs/TxGen.sol";

contract ProtocolAdapterUpgradeTest is Test {
    using TxGen for Vm;

    address internal constant _EMERGENCY_COMMITTEE = address(uint160(1));
    address internal constant _UNAUTHORIZED_CALLER = address(uint160(2));
    bytes4 internal constant _NEW_VERIFIER_SELECTOR = bytes4(0xdeadbeef);

    RiscZeroVerifierRouter internal _router;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    RiscZeroMockVerifier internal _verifier;
    ProtocolAdapter internal _pa;

    function setUp() public {
        (_router, _emergencyStop, _verifier) = new DeployRiscZeroContractsMock().run();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR());

        _pa = ProtocolAdapter(
            Upgrades.deployUUPSProxy(
                "ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_EMERGENCY_COMMITTEE)), opts
            )
        );
    }

    function test_upgrade_allows_replacing_the_risc_zero_verifier_selector_after_an_emergency_stop() public {
        // Execute a transaction proven against the old verifier.
        (Transaction memory oldTxn,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, complianceUnitCount: 1}),
            isProofAggregated: false
        });
        _pa.execute(oldTxn);

        bytes32 latestRootBeforeUpgrade = _pa.latestCommitmentTreeRoot();
        uint256 commitmentCountBeforeUpgrade = _pa.commitmentCount();

        // The old verifier gets stopped, halting the protocol adapter.
        vm.prank(_emergencyStop.owner());
        _emergencyStop.estop();

        assertTrue(_pa.isEmergencyStopped(), "PA should be stopped after the verifier emergency stop");

        vm.expectRevert(Pausable.EnforcedPause.selector, address(_emergencyStop));
        _pa.execute(oldTxn);

        // Deploy a new verifier and register it on the router under a new selector.
        RiscZeroMockVerifier newVerifier = new RiscZeroMockVerifier(_NEW_VERIFIER_SELECTOR);
        RiscZeroVerifierEmergencyStop newEmergencyStop =
            new RiscZeroVerifierEmergencyStop(IRiscZeroVerifier(address(newVerifier)), address(this));

        vm.prank(_router.owner());
        _router.addVerifier({selector: _NEW_VERIFIER_SELECTOR, verifier: newEmergencyStop});

        // Upgrade the proxy to an implementation bound to the new selector. `UnsafeUpgrades` is used because the
        // upgrade-safety validator rejects same-contract storage layout references and the implementation source
        // is unchanged (it is already validated during the proxy deployment in `setUp`).
        address newImplementation = address(new ProtocolAdapter(_router, _NEW_VERIFIER_SELECTOR));

        UnsafeUpgrades.upgradeProxy(address(_pa), newImplementation, "", _EMERGENCY_COMMITTEE);

        // The new selector is in place and the protocol adapter is operational again.
        assertEq(_pa.getRiscZeroVerifierSelector(), _NEW_VERIFIER_SELECTOR, "the new selector should be in place");
        assertFalse(_pa.isEmergencyStopped(), "PA should be operational again after the upgrade");

        // The protocol adapter state survived the upgrade.
        assertEq(_pa.latestCommitmentTreeRoot(), latestRootBeforeUpgrade, "the latest root should survive the upgrade");
        assertEq(_pa.commitmentCount(), commitmentCountBeforeUpgrade, "the commitment count should survive the upgrade");

        // Transactions proven against the new verifier execute.
        (Transaction memory newTxn,) = vm.transaction({
            mockVerifier: newVerifier,
            nonce: bytes32(uint256(1000)),
            configs: TxGen.generateActionConfigs({actionCount: 1, complianceUnitCount: 1}),
            isProofAggregated: false
        });
        _pa.execute(newTxn);

        // Proofs generated for the old verifier selector are rejected.
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapter.RiscZeroVerifierSelectorMismatch.selector, _NEW_VERIFIER_SELECTOR, _verifier.SELECTOR()
            ),
            address(_pa)
        );
        _pa.execute(oldTxn);
    }

    function test_upgrade_reverts_if_the_caller_is_not_the_owner() public {
        ProtocolAdapter newImplementation = new ProtocolAdapter(_router, _NEW_VERIFIER_SELECTOR);

        vm.prank(_UNAUTHORIZED_CALLER);
        vm.expectRevert(
            abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, _UNAUTHORIZED_CALLER), address(_pa)
        );
        _pa.upgradeToAndCall(address(newImplementation), "");
    }
}
