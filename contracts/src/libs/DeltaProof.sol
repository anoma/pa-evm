// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ECDSA} from "@openzeppelin-contracts-5.7.0/utils/cryptography/ECDSA.sol";
import {EllipticCurve} from "elliptic-curve-solidity-0.2.5/contracts/EllipticCurve.sol";
import {EfficientHashLib} from "solady-0.1.26/src/utils/EfficientHashLib.sol";

import {IProtocolAdapter} from "../interfaces/IProtocolAdapter.sol";

/// @title DeltaProof
/// @author Anoma Foundation, 2025
/// @notice A library containing methods of the delta proving system.
/// @custom:security-contact security@anoma.foundation
library DeltaProof {
    /// @notice A curve point in Jacobian coordinates, where the all-zero value is the zero delta.
    struct Jacobian {
        uint256 x;
        uint256 y;
        uint256 z;
    }

    /// @notice The x-coordinate of the curve generator point.
    uint256 internal constant _GX = 0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798;

    /// @notice The y-coordinate of the curve generator point.
    uint256 internal constant _GY = 0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8;

    // @notice The coefficient a of th secp256k1 (K-256) elliptic curve (y² = x³ + ax + b).
    uint256 internal constant _AA = 0;

    // @notice The coefficient b of th secp256k1 (K-256) elliptic curve (y² = x³ + ax + b).
    uint256 internal constant _BB = 7;

    /// @notice The field prime modulus (2^256 - 2^32 - 977) of the secp256k1 (K-256) elliptic curve (y² = x³ + ax + b).
    uint256 internal constant _PP = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F;

    /// @notice Thrown if the recovered delta public key doesn't match the delta instance.
    error DeltaMismatch(address expected, address actual);

    /// @notice Thrown when a provided point is not on the curve.
    error PointNotOnCurve(IProtocolAdapter.Delta point);

    /// @notice Converts a Jacobian sum to affine coordinates with one modular inverse, computed as z^(p-2) by the
    /// modexp precompile.
    /// @param acc The Jacobian sum.
    /// @return sum The affine point, which is the zero delta (0, 0) for z = 0, as 0^(p-2) = 0.
    function toAffine(Jacobian memory acc) internal view returns (IProtocolAdapter.Delta memory sum) {
        if (acc.z == 1) {
            return IProtocolAdapter.Delta({x: acc.x, y: acc.y});
        }

        // forge-lint: disable-start(low-level-calls, custom-errors, literal-instead-of-constant)
        // slither-disable-next-line low-level-calls
        (bool ok, bytes memory out) = address(0x05).staticcall(abi.encode(32, 32, 32, acc.z, _PP - 2, _PP));
        require(ok);
        // forge-lint: disable-end(low-level-calls, custom-errors, literal-instead-of-constant)
        uint256 zInv = abi.decode(out, (uint256));

        uint256 zInv2 = mulmod(zInv, zInv, _PP);
        sum.x = mulmod(acc.x, zInv2, _PP);
        sum.y = mulmod(acc.y, mulmod(zInv, zInv2, _PP), _PP);
    }

    /// @notice Adds a delta point to a running Jacobian sum in place.
    /// @param acc The running sum, starting from the zero delta.
    /// @param rhs The point to add that must be a curve point.
    /// @dev Only the added point is checked, as the running sum is built from checked points. The conversion to affine
    /// coordinates is deferred to `toAffine`, so that summing the action deltas in the `ProtocolAdapter.execute()`
    /// function costs one modular inverse instead of one per action.
    function add(Jacobian memory acc, IProtocolAdapter.Delta memory rhs) internal pure {
        require(EllipticCurve.isOnCurve({_x: rhs.x, _y: rhs.y, _aa: _AA, _bb: _BB, _pp: _PP}), PointNotOnCurve(rhs));

        if (acc.z == 0) {
            (acc.x, acc.y, acc.z) = (rhs.x, rhs.y, 1);
            return;
        }

        // The Jacobian addition is undefined for equal x-coordinates, so the doubling and the cancellation to the zero
        // delta are picked first by comparing the added point scaled to the running sum.
        uint256 zz = mulmod(acc.z, acc.z, _PP);
        if (acc.x == mulmod(rhs.x, zz, _PP)) {
            if (acc.y == mulmod(rhs.y, mulmod(acc.z, zz, _PP), _PP)) {
                (acc.x, acc.y, acc.z) = EllipticCurve.jacDouble({_x: acc.x, _y: acc.y, _z: acc.z, _aa: _AA, _pp: _PP});
            } else {
                (acc.x, acc.y, acc.z) = (0, 0, 0);
            }
            return;
        }

        (acc.x, acc.y, acc.z) =
            EllipticCurve.jacAdd({_x1: acc.x, _y1: acc.y, _z1: acc.z, _x2: rhs.x, _y2: rhs.y, _z2: 1, _pp: _PP});
    }

    /// @notice Converts an elliptic curve point to an Ethereum account address.
    /// @param delta The elliptic curve point.
    /// @return account The associated account.
    function toAccount(IProtocolAdapter.Delta memory delta) internal pure returns (address account) {
        // Hash the public key with Keccak-256.
        bytes32 hashedKey = EfficientHashLib.hash(delta.x, delta.y);

        // Take the last 20 bytes to obtain an Ethereum address.
        account = address(uint160(uint256(hashedKey)));
    }

    /// @notice Computes the delta verifying key as the Keccak-256 hash of all action tree roots as ordered in the
    /// transaction. The verifying key doubles as the transaction ID.
    /// @param actionTreeRoots The action tree roots to compute the verifying key from.
    /// @return verifyingKey The verifying key obtained from hashing the action tree roots.
    function computeVerifyingKey(bytes32[] memory actionTreeRoots) internal pure returns (bytes32 verifyingKey) {
        verifyingKey = EfficientHashLib.hash(actionTreeRoots);
    }

    /// @notice Verifies a delta proof.
    /// @param proof The delta proof.
    /// @param instance The transaction delta.
    /// @param verifyingKey The Keccak-256 hash of all action tree roots as ordered in the transaction.
    function verify(bytes memory proof, IProtocolAdapter.Delta memory instance, bytes32 verifyingKey) internal pure {
        // Verify the delta proof using the ECDSA.recover API to obtain the address
        address recovered = ECDSA.recover({hash: verifyingKey, signature: proof});

        // Convert the public key to an address
        address expected = toAccount(instance);

        // Compare it with the recovered address
        require(recovered == expected, DeltaMismatch({expected: expected, actual: recovered}));
    }
}
