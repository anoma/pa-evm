// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ECDSA} from "@openzeppelin-contracts-5.7.0/utils/cryptography/ECDSA.sol";
import {EllipticCurve} from "elliptic-curve-solidity-0.2.5/contracts/EllipticCurve.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {DeltaProof} from "../../src/libs/DeltaProof.sol";

import {DeltaGen} from "../libs/DeltaGen.sol";

library DeltaFuzzing {
    struct InstanceInputsExceptKind {
        uint256 valueCommitmentRandomness;
        uint128 quantity;
        bool consumed;
    }

    /// @dev This function exposes `Delta.verify` for the fuzzer.
    function verify(bytes memory proof, IProtocolAdapter.Delta memory instance, bytes32 verifyingKey) public pure {
        DeltaProof.verify({proof: proof, instance: instance, verifyingKey: verifyingKey});
    }
}

contract DeltaProofTest is Test {
    using DeltaProof for IProtocolAdapter.Delta;
    using DeltaProof for DeltaProof.Jacobian;
    using DeltaGen for DeltaGen.InstanceInputs[];
    using DeltaGen for DeltaGen.InstanceInputs;
    using DeltaGen for uint256;

    function testFuzz_add_adds_deltas(
        uint256 kind,
        DeltaFuzzing.InstanceInputsExceptKind memory input1,
        DeltaFuzzing.InstanceInputsExceptKind memory input2
    ) public {
        kind = bound(kind, 1, DeltaGen.SECP256K1_ORDER - 1);
        input1.valueCommitmentRandomness = bound(input1.valueCommitmentRandomness, 1, DeltaGen.SECP256K1_ORDER - 1);
        input2.valueCommitmentRandomness = bound(input2.valueCommitmentRandomness, 1, DeltaGen.SECP256K1_ORDER - 1);

        // Construct delta instance inputs from the above parameters
        DeltaGen.InstanceInputs memory deltaInputs1 = DeltaGen.InstanceInputs({
            kind: kind,
            quantity: input1.quantity,
            consumed: input1.consumed,
            valueCommitmentRandomness: input1.valueCommitmentRandomness
        });
        vm.assume(deltaInputs1.computePreDelta() != 0);
        DeltaGen.InstanceInputs memory deltaInputs2 = DeltaGen.InstanceInputs({
            kind: kind,
            quantity: input2.quantity,
            consumed: input2.consumed,
            valueCommitmentRandomness: input2.valueCommitmentRandomness
        });
        vm.assume(deltaInputs2.computePreDelta() != 0);

        // The expected summed delta is the curve point derived from the sum of the two pre-delta scalars: scalar
        // addition modulo the curve order matches point addition on the curve.
        uint256 expectedPreDelta =
            addmod(deltaInputs1.computePreDelta(), deltaInputs2.computePreDelta(), DeltaGen.SECP256K1_ORDER);
        vm.assume(expectedPreDelta != 0);

        // Generate a delta proof and instance from the above tags and preimage
        IProtocolAdapter.Delta memory instance1 = DeltaGen.generateInstance(vm, deltaInputs1);
        IProtocolAdapter.Delta memory instance2 = DeltaGen.generateInstance(vm, deltaInputs2);
        IProtocolAdapter.Delta memory expectedDelta = DeltaGen.generateInstanceFromPreDelta(vm, expectedPreDelta);

        // Verify that the deltas add correctly
        IProtocolAdapter.Delta memory computedDelta = DeltaGen.add(instance1, instance2);

        assertEq(computedDelta.x, expectedDelta.x, "computed delta x should match expected");
        assertEq(computedDelta.y, expectedDelta.y, "computed delta y should match expected");
    }

    function testFuzz_verify_reverts_if_the_delta_is_non_zero(
        DeltaGen.InstanceInputs memory deltaInstanceInputs,
        bytes32 verifyingKey
    ) public {
        deltaInstanceInputs.kind = bound(deltaInstanceInputs.kind, 1, DeltaGen.SECP256K1_ORDER - 1);
        deltaInstanceInputs.valueCommitmentRandomness =
            bound(deltaInstanceInputs.valueCommitmentRandomness, 1, DeltaGen.SECP256K1_ORDER - 1);

        // Ensure that the quantity and pre-delta are non-zero.
        vm.assume(DeltaGen.signedQuantityModOrder(deltaInstanceInputs.consumed, deltaInstanceInputs.quantity) != 0);
        vm.assume(deltaInstanceInputs.computePreDelta() != 0);

        // Generate a delta proof and instance from the above tags and preimage
        IProtocolAdapter.Delta memory instance = DeltaGen.generateInstance(vm, deltaInstanceInputs);

        // Construct delta proof inputs from the above parameters
        DeltaGen.ProofInputs memory deltaProofInputs = DeltaGen.ProofInputs({
            summedValueCommitmentRandomness: deltaInstanceInputs.valueCommitmentRandomness, verifyingKey: verifyingKey
        });
        bytes memory proof = DeltaGen.generateProof(vm, deltaProofInputs);

        address expected = instance.toAccount();
        address actual = ECDSA.recover(deltaProofInputs.verifyingKey, proof);
        vm.expectRevert(abi.encodeWithSelector(DeltaProof.DeltaMismatch.selector, expected, actual));
        DeltaFuzzing.verify({proof: proof, instance: instance, verifyingKey: deltaProofInputs.verifyingKey});
    }

    function testFuzz_verify_reverts_if_value_commitment_randomness_of_instance_and_proof_mismatch(
        uint256 kind,
        bool consumed,
        bytes32 verifyingKey,
        uint256 valueCommitmentRandomness1,
        uint256 valueCommitmentRandomness2
    ) public {
        kind = bound(kind, 1, DeltaGen.SECP256K1_ORDER - 1);

        valueCommitmentRandomness1 = bound(valueCommitmentRandomness1, 1, DeltaGen.SECP256K1_ORDER - 1);
        valueCommitmentRandomness2 = bound(valueCommitmentRandomness2, 1, DeltaGen.SECP256K1_ORDER - 1);
        vm.assume(valueCommitmentRandomness1.modOrder() != valueCommitmentRandomness2.modOrder());

        DeltaGen.InstanceInputs memory deltaInputs = DeltaGen.InstanceInputs({
            kind: kind, quantity: 0, consumed: consumed, valueCommitmentRandomness: valueCommitmentRandomness1
        });
        vm.assume(deltaInputs.computePreDelta() != 0);
        IProtocolAdapter.Delta memory instanceRcv1 = DeltaGen.generateInstance(vm, deltaInputs);

        bytes memory proofRcv2 = DeltaGen.generateProof(
            vm,
            DeltaGen.ProofInputs({
                summedValueCommitmentRandomness: valueCommitmentRandomness2, verifyingKey: verifyingKey
            })
        );

        address expected = instanceRcv1.toAccount();
        address actual = ECDSA.recover(verifyingKey, proofRcv2);
        vm.expectRevert(abi.encodeWithSelector(DeltaProof.DeltaMismatch.selector, expected, actual));
        DeltaFuzzing.verify({proof: proofRcv2, instance: instanceRcv1, verifyingKey: verifyingKey});
    }

    function testFuzz_verify_reverts_if_verifying_keys_mismatch(
        uint256 kind,
        uint256 valueCommitmentRandomness,
        bool consumed,
        bytes32 verifyingKey1,
        bytes32 verifyingKey2
    ) public {
        kind = bound(kind, 1, DeltaGen.SECP256K1_ORDER - 1);
        valueCommitmentRandomness = bound(valueCommitmentRandomness, 1, DeltaGen.SECP256K1_ORDER - 1);
        vm.assume(uint256(verifyingKey1).modOrder() != uint256(verifyingKey2).modOrder());

        DeltaGen.InstanceInputs memory deltaInputs = DeltaGen.InstanceInputs({
            kind: kind, quantity: 0, consumed: consumed, valueCommitmentRandomness: valueCommitmentRandomness
        });
        vm.assume(deltaInputs.computePreDelta() != 0);
        IProtocolAdapter.Delta memory instance = DeltaGen.generateInstance(vm, deltaInputs);

        bytes memory proofForVk1 = DeltaGen.generateProof(
            vm,
            DeltaGen.ProofInputs({
                summedValueCommitmentRandomness: valueCommitmentRandomness, verifyingKey: verifyingKey1
            })
        );

        vm.expectPartialRevert(DeltaProof.DeltaMismatch.selector);
        DeltaFuzzing.verify({proof: proofForVk1, instance: instance, verifyingKey: verifyingKey2});
    }

    function testFuzz_verify_passes_if_delta_is_zero(
        uint256 kind,
        uint256 valueCommitmentRandomness,
        bool consumed,
        bytes32 verifyingKey
    ) public {
        kind = bound(kind, 1, DeltaGen.SECP256K1_ORDER - 1);
        valueCommitmentRandomness = bound(valueCommitmentRandomness, 1, DeltaGen.SECP256K1_ORDER - 1);

        // Construct delta instance inputs from the above parameters for quantity zero.
        DeltaGen.InstanceInputs memory deltaInstanceInputs = DeltaGen.InstanceInputs({
            kind: kind, quantity: 0, consumed: consumed, valueCommitmentRandomness: valueCommitmentRandomness
        });
        vm.assume(deltaInstanceInputs.computePreDelta() != 0);

        // Construct delta proof inputs from the above parameters
        DeltaGen.ProofInputs memory deltaProofInputs = DeltaGen.ProofInputs({
            summedValueCommitmentRandomness: valueCommitmentRandomness, verifyingKey: verifyingKey
        });

        // Generate a delta instance from the above inputs
        IProtocolAdapter.Delta memory instance = DeltaGen.generateInstance(vm, deltaInstanceInputs);

        // Generate a delta proof from the above inputs
        bytes memory proof = DeltaGen.generateProof(vm, deltaProofInputs);

        // Verify that the generated delta proof is valid
        DeltaFuzzing.verify({proof: proof, instance: instance, verifyingKey: deltaProofInputs.verifyingKey});
    }

    function testFuzz_verify_passes_if_balanced_kind_and_quantity_pairs_accumulate_to_a_zero_delta(
        uint256 kind,
        DeltaFuzzing.InstanceInputsExceptKind[] memory fuzzerInputs,
        bytes32 verifyingKey
    ) public {
        // Bound kind to valid range (non-zero, less than curve order)
        kind = bound(kind, 1, DeltaGen.SECP256K1_ORDER - 1);
        DeltaGen.InstanceInputs[] memory baseInputs = _getBoundedDeltaInstances(kind, fuzzerInputs);
        vm.assume(baseInputs.length > 0);

        // Create balanced pairs: for each input with quantity q, creates +q and -q entries
        // This guarantees the quantities sum to zero by construction
        (DeltaGen.InstanceInputs[] memory pairedInputs, uint256 totalRandomness) =
            DeltaGen.createBalancedPairs(baseInputs);

        // Ensure accumulated randomness is valid for proof generation
        totalRandomness = totalRandomness.modOrder();
        vm.assume(totalRandomness != 0);

        // Generate and accumulate delta instances
        IProtocolAdapter.Delta memory deltaAcc = DeltaGen.zero();
        for (uint256 i = 0; i < pairedInputs.length; i++) {
            // Normalize and validate each input
            pairedInputs[i].valueCommitmentRandomness = pairedInputs[i].valueCommitmentRandomness.modOrder();
            vm.assume(pairedInputs[i].computePreDelta() != 0);

            IProtocolAdapter.Delta memory instance = DeltaGen.generateInstance(vm, pairedInputs[i]);
            deltaAcc = DeltaGen.add(deltaAcc, instance);
        }

        // Generate proof with the total accumulated randomness
        bytes memory proof = DeltaGen.generateProof(
            vm, DeltaGen.ProofInputs({summedValueCommitmentRandomness: totalRandomness, verifyingKey: verifyingKey})
        );

        // Verify: should succeed because quantities are balanced (sum to zero)
        DeltaFuzzing.verify({proof: proof, instance: deltaAcc, verifyingKey: verifyingKey});
    }

    function testFuzz_add_reverts_when_adding_a_non_curve_from_the_right(
        uint32 k,
        IProtocolAdapter.Delta memory rhs,
        bool empty
    ) public {
        // Ensure that `rhs` is not on the curve.
        vm.assume(
            !EllipticCurve.isOnCurve({
                _x: rhs.x, _y: rhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            })
        );

        // Generate a random point on the curve.
        IProtocolAdapter.Delta memory lhs =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k});
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: lhs.x, _y: lhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Left-hand side point must be on the curve."
        );

        DeltaProof.Jacobian memory acc;
        if (!empty) acc.add(lhs);

        // Attempt to add a point being not on the curve.
        vm.expectRevert(abi.encodeWithSelector(DeltaProof.PointNotOnCurve.selector, rhs), address(this));
        acc.add(rhs);
    }

    function testFuzz_add_reverts_when_adding_zero_from_the_right(uint32 k, bool empty) public {
        // Generate a random point on the curve.
        IProtocolAdapter.Delta memory lhs =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k});
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: lhs.x, _y: lhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Left-hand side point must be on the curve."
        );

        IProtocolAdapter.Delta memory zero = DeltaGen.zero();
        DeltaProof.Jacobian memory acc;
        if (!empty) acc.add(lhs);

        // Add the two points.
        vm.expectRevert(abi.encodeWithSelector(DeltaProof.PointNotOnCurve.selector, zero), address(this));
        acc.add(zero);
    }

    /// @dev Each step adds a fresh multiple of the generator, a repeat of the previous point, or the running sum or its
    /// negation (doubling and cancellation against a running sum with z != 1), so that sums pass through the zero
    /// delta and restart from it.
    function testFuzz_add_matches_the_chained_affine_sum(uint256[6] memory seeds, uint8 count) public view {
        IProtocolAdapter.Delta[] memory points = new IProtocolAdapter.Delta[](1 + (count % 6));
        uint256 running = 0;
        uint256 k = 0;
        for (uint256 i = 0; i < points.length; i++) {
            uint256 mode = running == 0 ? 0 : seeds[i] % 4;
            if (mode == 0) k = 1 + (seeds[i] >> 2) % (DeltaGen.SECP256K1_ORDER - 1);
            if (mode == 2) k = running;
            if (mode == 3) k = DeltaGen.SECP256K1_ORDER - running;
            points[i] = _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k});
            running = addmod(running, k, DeltaGen.SECP256K1_ORDER);
        }

        _assertJacobianSumMatchesChainedSum(points);
    }

    function test_add_handles_cancellation_and_repeated_doubling_with_a_running_sum_that_is_not_normalized()
        public
        view
    {
        IProtocolAdapter.Delta memory g =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: 1});

        // G + (-G) is the zero delta.
        IProtocolAdapter.Delta[] memory points = new IProtocolAdapter.Delta[](2);
        points[0] = g;
        points[1] = IProtocolAdapter.Delta({x: g.x, y: DeltaProof._PP - g.y});
        IProtocolAdapter.Delta memory sum = _jacobianSum(points);
        assertEq(sum.x, 0, "G + (-G) x should be zero");
        assertEq(sum.y, 0, "G + (-G) y should be zero");

        // G + G doubles, and the sum continues from a running sum with z != 1.
        points = new IProtocolAdapter.Delta[](4);
        points[0] = g;
        points[1] = g;
        points[2] = g;
        points[3] = g;
        _assertJacobianSumMatchesChainedSum(points);
    }

    function testFuzz_add_adding_zero_from_the_left_produces_a_curve_point(uint32 k) public pure {
        IProtocolAdapter.Delta memory zero = DeltaGen.zero();

        // Generate a random point on the curve.
        IProtocolAdapter.Delta memory rhs =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k});
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: rhs.x, _y: rhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Right-hand side point must be on the curve."
        );

        // Add the two points and check that the sum is a curve point.
        IProtocolAdapter.Delta memory sum = DeltaGen.add(zero, rhs);
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: sum.x, _y: sum.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Sum must be on the curve."
        );
    }

    function testFuzz_add_adding_zero_from_the_left_is_the_identity_operation(uint32 k) public pure {
        IProtocolAdapter.Delta memory zero = DeltaGen.zero();

        // Generate a random point on the curve.
        IProtocolAdapter.Delta memory rhs =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k});
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: rhs.x, _y: rhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Right-hand side point must be on the curve."
        );

        // Add the two points and check that the sum is a curve point.
        IProtocolAdapter.Delta memory sum = DeltaGen.add(zero, rhs);
        assertEq(sum.x, rhs.x, "zero + rhs x should equal rhs x");
        assertEq(sum.y, rhs.y, "zero + rhs y should equal rhs y");
    }

    function testFuzz_add_adding_two_curve_points_produces_a_curve_point(uint32 k1, uint32 k2) public pure {
        // Generate two random points on the curve.
        IProtocolAdapter.Delta memory lhs =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k1});
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: lhs.x, _y: lhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Left-hand side point must be on the curve."
        );

        IProtocolAdapter.Delta memory rhs =
            _mul({p: IProtocolAdapter.Delta({x: DeltaProof._GX, y: DeltaProof._GY}), k: k2});
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: rhs.x, _y: rhs.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Right-hand side must be on the curve."
        );

        // Add the two points and check that the sum is a curve point.
        IProtocolAdapter.Delta memory sum = DeltaGen.add(lhs, rhs);
        assertTrue(
            EllipticCurve.isOnCurve({
                _x: sum.x, _y: sum.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "Sum must be on the curve."
        );
    }

    function test_zero_is_not_on_the_curve() public pure {
        IProtocolAdapter.Delta memory p2 = DeltaGen.zero();
        assertFalse(
            EllipticCurve.isOnCurve({
                _x: p2.x, _y: p2.y, _aa: DeltaProof._AA, _bb: DeltaProof._BB, _pp: DeltaProof._PP
            }),
            "zero point should not be on the curve"
        );
    }

    function _jacobianSum(IProtocolAdapter.Delta[] memory points)
        internal
        view
        returns (IProtocolAdapter.Delta memory sum)
    {
        DeltaProof.Jacobian memory acc;
        for (uint256 i = 0; i < points.length; i++) {
            acc.add(points[i]);
        }
        sum = acc.toAffine();
    }

    /// @dev The reference is the affine `add` chained from the zero delta.
    function _assertJacobianSumMatchesChainedSum(IProtocolAdapter.Delta[] memory points) internal view {
        IProtocolAdapter.Delta memory chained = DeltaGen.zero();
        for (uint256 i = 0; i < points.length; i++) {
            chained = DeltaGen.add(chained, points[i]);
        }

        IProtocolAdapter.Delta memory sum = _jacobianSum(points);
        assertEq(sum.x, chained.x, "Jacobian sum x should match the chained sum");
        assertEq(sum.y, chained.y, "Jacobian sum y should match the chained sum");
    }

    function _getBoundedDeltaInstances(uint256 kind, DeltaFuzzing.InstanceInputsExceptKind[] memory fuzzerInputs)
        internal
        pure
        returns (DeltaGen.InstanceInputs[] memory deltaInputs)
    {
        // Array length
        uint256 boundedLength = bound(fuzzerInputs.length, 0, 10);

        // solhint-disable-next-line no-inline-assembly
        assembly {
            mstore(fuzzerInputs, boundedLength)
        }

        deltaInputs = new DeltaGen.InstanceInputs[](fuzzerInputs.length);
        for (uint256 i = 0; i < fuzzerInputs.length; i++) {
            deltaInputs[i] = DeltaGen.InstanceInputs({
                kind: kind,
                valueCommitmentRandomness: fuzzerInputs[i].valueCommitmentRandomness,
                quantity: fuzzerInputs[i].quantity,
                consumed: fuzzerInputs[i].consumed
            });
        }
    }

    function _mul(IProtocolAdapter.Delta memory p, uint256 k)
        internal
        pure
        returns (IProtocolAdapter.Delta memory product)
    {
        (product.x, product.y) =
            EllipticCurve.ecMul({_k: k, _x: p.x, _y: p.y, _aa: DeltaProof._AA, _pp: DeltaProof._PP});
    }
}
