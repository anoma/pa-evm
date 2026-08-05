// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Logic} from "./libs/proving/Logic.sol";

/// @title Types
/// @author Anoma Foundation, 2025
/// @notice A library containing the type definitions of the protocol adapter.
/// @custom:security-contact security@anoma.foundation
library Types {
    /// @notice The resource object constituting the atomic unit of state in the Anoma protocol.
    /// @param  logicRef The hash of the resource logic function.
    /// @param  labelRef The hash of the resource label, which can contain arbitrary data.
    /// @param  valueRef The hash of the resource value, which can contain arbitrary data.
    /// @param  nullifierKeyCommitment The commitment to the nullifier key.
    /// @param  quantity The quantity that the resource represents.
    /// @param  nonce The nonce guaranteeing the resource's uniqueness.
    /// @param  randSeed The randomness seed that can be used to derive pseudo-randomness for applications.
    /// @param  ephemeral The resource's ephemerality.
    struct Resource {
        bytes32 logicRef;
        bytes32 labelRef;
        bytes32 valueRef;
        bytes32 nullifierKeyCommitment;
        bytes32 nonce;
        bytes32 randSeed;
        uint128 quantity;
        bool ephemeral;
    }

    /// @notice The transaction object containing all required data to conduct a RM state transition
    /// in which resources get consumed and created.
    /// @param actions The list of actions to be executed.
    /// @param deltaProof The proof for the transaction delta value.
    /// @param aggregationProof The recursive proof of all compliance and resource logics in the transaction — the only
    /// RISC Zero proof being verified.
    struct Transaction {
        Action[] actions;
        bytes deltaProof;
        bytes aggregationProof;
    }

    /// @notice The action object providing context separation between non-intersecting sets of resources. An action
    /// corresponds to one compliance unit constraining its consumed and created resources.
    /// @param consumed The public data of the consumed resources.
    /// @param created The public data of the created resources.
    /// @param delta The action's delta value obtained from the underlying compliance unit.
    /// @param actionTreeRoot The root of the tree containing the tags of all resources present in the action.
    struct Action {
        Consumed[] consumed;
        Created[] created;
        Delta delta;
        bytes32 actionTreeRoot;
    }

    /// @notice The public data of a consumed resource.
    /// @param nullifier The nullifier of the resource.
    /// @param logicRef The reference to (the verifying key of) the resource logic.
    /// @param commitmentTreeRoot The historical commitment tree root the resource's inclusion is proven against.
    /// @param appData The application data associated with the resource.
    struct Consumed {
        bytes32 nullifier;
        bytes32 logicRef;
        bytes32 commitmentTreeRoot;
        Logic.AppData appData;
    }

    /// @notice The public data of a created resource.
    /// @param commitment The commitment of the resource.
    /// @param logicRef The reference to (the verifying key of) the resource logic.
    /// @param appData The application data associated with the resource.
    struct Created {
        bytes32 commitment;
        bytes32 logicRef;
        Logic.AppData appData;
    }

    /// @notice An elliptic curve point representing a delta value.
    /// @param x The x component of the point.
    /// @param y The y component of the point.
    struct Delta {
        uint256 x;
        uint256 y;
    }
}
