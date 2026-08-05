// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.1/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test, Vm} from "forge-std-1.16.2/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {UnsafeUpgrades, Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {IRiscZeroVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/IRiscZeroVerifier.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";
import {TxGen} from "./libs/TxGen.sol";
import {ProtocolAdapterResumableMock} from "./mocks/ProtocolAdapterResumable.m.sol";

contract ProtocolAdapterUpgradeTest is Test {
    using TxGen for Vm;

    address internal constant _OWNER = address(uint160(1));
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
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );
    }

    function test_upgrade_allows_replacing_the_risc_zero_verifier_selector_after_an_emergency_stop() public {
        // Execute a transaction proven against the old verifier.
        (IProtocolAdapter.Transaction memory oldTxn, bytes32 nonce) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        _pa.execute(oldTxn);

        bytes32 latestRootBeforeUpgrade = _pa.latestCommitmentTreeRoot();
        uint256 commitmentCountBeforeUpgrade = _pa.commitmentCount();

        // The old verifier gets stopped, halting the protocol adapter.
        vm.prank(_emergencyStop.owner());
        _emergencyStop.estop();

        assertTrue(_pa.isEmergencyStopped(), "PA should be stopped after the verifier emergency stop");

        // A fresh transaction proven against the stopped verifier reverts on proof verification. A fresh
        // transaction is needed because the state transition — rejecting the replayed nullifiers of `oldTxn` —
        // precedes proof verification.
        (IProtocolAdapter.Transaction memory secondOldTxn,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: nonce,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        vm.expectRevert(Pausable.EnforcedPause.selector, address(_emergencyStop));
        _pa.execute(secondOldTxn);

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

        UnsafeUpgrades.upgradeProxy(address(_pa), newImplementation, "", _OWNER);

        // The new selector is in place and the protocol adapter is operational again.
        assertEq(_pa.getRiscZeroVerifierSelector(), _NEW_VERIFIER_SELECTOR, "the new selector should be in place");
        assertFalse(_pa.isEmergencyStopped(), "PA should be operational again after the upgrade");

        // The protocol adapter state survived the upgrade.
        assertEq(_pa.latestCommitmentTreeRoot(), latestRootBeforeUpgrade, "the latest root should survive the upgrade");
        assertEq(_pa.commitmentCount(), commitmentCountBeforeUpgrade, "the commitment count should survive the upgrade");

        // Transactions proven against the new verifier execute.
        (IProtocolAdapter.Transaction memory newTxn,) = vm.transaction({
            mockVerifier: newVerifier,
            nonce: bytes32(uint256(1000)),
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        _pa.execute(newTxn);

        // Proofs generated for the old verifier selector are rejected. The transaction with unconsumed
        // nullifiers is used so that the selector check is reached.
        vm.expectRevert(
            abi.encodeWithSelector(
                ProtocolAdapter.RiscZeroVerifierSelectorMismatch.selector, _NEW_VERIFIER_SELECTOR, _verifier.SELECTOR()
            ),
            address(_pa)
        );
        _pa.execute(secondOldTxn);
    }

    /// @dev `emergencyStop` has no counterpart in the current implementation, so lifting the pause it sets takes an
    /// upgrade to an implementation carrying a recovery path.
    function test_upgrade_allows_lifting_the_pause_set_by_an_emergency_stop() public {
        // Execute a transaction while the protocol adapter is operational.
        (IProtocolAdapter.Transaction memory txnBeforeStop, bytes32 nonce) = vm.transaction({
            mockVerifier: _verifier,
            nonce: 0,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });
        _pa.execute(txnBeforeStop);

        bytes32 latestRootBeforeStop = _pa.latestCommitmentTreeRoot();
        uint256 commitmentCountBeforeStop = _pa.commitmentCount();

        // The owner stops the protocol adapter.
        vm.prank(_OWNER);
        _pa.emergencyStop();

        assertTrue(_pa.isEmergencyStopped(), "PA should be stopped after the emergency stop");

        // Execution is halted. The `whenNotPaused` modifier rejects the transaction before it touches any state, so
        // this very transaction can be replayed once the pause is lifted.
        (IProtocolAdapter.Transaction memory txnAfterStop,) = vm.transaction({
            mockVerifier: _verifier,
            nonce: nonce,
            configs: TxGen.generateActionConfigs({actionCount: 1, consumedCount: 1, createdCount: 1})
        });

        vm.expectRevert(Pausable.EnforcedPause.selector, address(_pa));
        _pa.execute(txnAfterStop);

        // There is no way out while the deployed implementation is in place — its ABI carries no recovery function,
        // so the proxy finds nothing to delegate to and reverts without data.
        vm.prank(_OWNER);
        vm.expectRevert();
        ProtocolAdapterResumableMock(address(_pa)).reinitialize();

        // Upgrade to an implementation carrying a recovery path, lifting the pause in the same transaction. Passing
        // the reinitializer as the upgrade call data leaves no block in which the new implementation is live but
        // still paused.
        Options memory opts;
        opts.constructorData = abi.encode(_router, _verifier.SELECTOR());

        Upgrades.upgradeProxy({
            proxy: address(_pa),
            contractName: "ProtocolAdapterResumable.m.sol:ProtocolAdapterResumableMock",
            data: abi.encodeCall(ProtocolAdapterResumableMock.reinitialize, ()),
            opts: opts,
            tryCaller: _OWNER
        });

        // The protocol adapter is operational again.
        assertFalse(_pa.paused(), "PA should be unpaused after the upgrade");
        assertFalse(_pa.isEmergencyStopped(), "PA should be operational again after the upgrade");

        // The protocol adapter state survived the upgrade.
        assertEq(_pa.latestCommitmentTreeRoot(), latestRootBeforeStop, "the latest root should survive the upgrade");
        assertEq(_pa.commitmentCount(), commitmentCountBeforeStop, "the commitment count should survive the upgrade");

        // The transaction that was rejected while paused now settles.
        _pa.execute(txnAfterStop);

        assertEq(_pa.commitmentCount(), commitmentCountBeforeStop + 1, "the replayed transaction should settle");
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
