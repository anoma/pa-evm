# Forwarder call order

`execute` runs the forwarder calls of a transaction after it has added the nullifiers and commitments of all actions and verified the proofs. This page defines which orders of those calls are valid, and which one the protocol adapter executes.

## Terms

- The tag of a resource is its nullifier if it is consumed, and its commitment if it is created.
- The call list of a resource is its forwarder calls, in the order of its external payload.
- The transaction order of the call lists follows the actions, and within an action puts the consumed resources before the created ones.

## Valid orders

An order of the call lists of a transaction is valid if:

1. The calls of each call list run together and in their order.
2. The call lists of each action run together: no call of another action runs between two calls of one action.
3. If a call lists the tag of a resource with calls in its `afterTags`, the calls of that resource run before the call list of the call. If it lists the tag in its `beforeTags`, they run after it.

So a constraint between call lists of two actions puts one whole action before the other. A tag of a resource without calls constrains nothing, and a call that lists the tag of its own resource leaves no valid order.

## The executed order

The protocol adapter executes the first valid order in transaction order: of two valid orders, it takes the one with the earlier call list at the first position where they differ. Without ordering constraints, that is the transaction order itself.

If a call carries an ordering constraint, `execute` reverts when:

| Condition                                         | Error                          |
|---------------------------------------------------|--------------------------------|
| an `afterTags` entry is no tag of the transaction | `AfterTagNotInTransaction`     |
| a `beforeTags` entry is no tag of the transaction | `BeforeTagNotInTransaction`    |
| more than 256 resources carry calls               | `TooManyCallCarryingResources` |
| no valid order exists                             | `CyclicConstraints`            |

If none of the first three holds, `execute` reverts with `CyclicConstraints` exactly when no valid order exists.

## Example

A swap action has three resources with calls, an unwrap, a generic call and a wrap, and the generic call names the unwrap in `afterTags` and the wrap in `beforeTags`. The only valid order of the action is unwrap, generic call, wrap. A call of another action that names the unwrap in `afterTags` and the generic call in `beforeTags` would run inside the swap, which rule 2 forbids, so the transaction reverts with `CyclicConstraints`. That call can still run before or after the whole swap.
