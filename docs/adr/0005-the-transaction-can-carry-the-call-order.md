# The transaction can carry the call order

## Context

The protocol adapter executes the call lists of a transaction in an order that it computes, as [the design of the external call ordering](../external-call-ordering.md) describes. Without ordering constraints, this is the order of the transaction. With ordering constraints, it is a topological sort that takes the call list with the lowest number first. So where the constraints leave two call lists unordered, their order depends on their numbers, and an action numbers its consumed resources before its created ones.

If only the generic call resource states ordering constraints, a created call list can run before a consumed call list of its action, when neither of them is a generic call, only if a generic call runs between them. An application that needs an unwrap to run before a wrap of the same action, with no generic call between them, cannot state it.

Ordering information in the label or value of a resource does not solve this. The commitment of a created resource depends on the nullifiers of the consumed resources of its action, so the label or value of such a consumed resource cannot name it. And a persistent consumed resource got its value from the transaction that created it, before the transaction that consumes it existed.

The sort also holds two sets of 256 bits for each call list, so a transaction with ordering constraints can have at most 256 call-carrying resources.

## Decision

The transaction can carry the order of its call lists, in a new last field `Transaction.callOrder` of type `uint256[]`. An entry is the number of a call list: the call lists are numbered from 0 in the order of the transaction, as `ForwarderCallOrdering.decodeCallLists` returns them. The delta proof signs the carried order.

- If `callOrder` is empty, nothing changes. The delta proof signs `transactionId`, and the protocol adapter computes the order as it does now.
- Otherwise, the delta proof signs `keccak256(abi.encode(transactionId, callOrder))` instead of `transactionId`. After the proofs and before any forwarder call, the protocol adapter decodes the call lists as it does for ordering constraints, whether or not a call carries one, checks `callOrder`, and executes the call lists in this order.

The check has three steps, in this order. A step runs only if the steps before it passed.

1. `callOrder` names each call list exactly once: its length is the number of call lists, and each entry is below this number and appears once. Otherwise the protocol adapter reverts with `CallOrderNotAPermutation()`. A transaction without call-carrying resources reverts with this error if its `callOrder` is not empty.
2. The action indices of the call lists never decrease along `callOrder`. So the actions keep the order of the transaction, and the call lists of each action stay together. Otherwise it reverts with `CallOrderNotInActionOrder(uint256 actionIndex)`, which names the action index of the first entry where they decrease.
3. Every ordering constraint holds. The protocol adapter visits the constraints as `_buildConstraints` does: call list by call list, call by call, the `afterTags` of a call before its `beforeTags`. As now, each entry must name a call-carrying resource, found with the first-match rule of `_indexOf`. A call list that an `afterTags` entry names must have a lower position in `callOrder` than the call list of the call, and one that a `beforeTags` entry names a higher position. Otherwise it reverts with `CallOrderViolatesConstraint(bytes32 tag)`, which names the entry. A tag that names the resource of its own call fails this comparison.

The check keeps the position of each call list in an array of one word per call list, and it builds no graph. With `_indexOf`, it costs O(n + n·t) for n call lists and t tag entries, which the submitter pays.

## Consequences

- The delta proof binds the order. A relayer that changes or drops `callOrder` makes the delta proof fail, so it cannot copy the transaction and run it first with another order. The holder of the delta key chooses the order: the user when the user builds the transaction alone, and the solver when a solver composes it.
- In a transaction that a solver composes, the solver chooses the order of the call lists of each action, within what the constraints allow. A user who relies on the positions of the resources in its action for the order of two call lists must state that order with an ordering constraint.
- An order that passes the check meets the rules of the design document. With every tag naming a call-carrying resource and at most 256 call-carrying resources, the sort finds an order exactly when such an order exists. A carried order is narrower in one way: it keeps the actions in the order of the transaction. A transaction whose constraints run a later action first therefore cannot carry its order, but its actions can be put in the order of the constraints instead.
- Above 256 call-carrying resources, only a transaction that carries its order can have ordering constraints.
- A cycle, or a tag that names the resource of its own call, reverts with `CallOrderViolatesConstraint` instead of `CyclicOrderingConstraints`.
- `transactionId` and the `TransactionExecuted` event do not depend on the order. The `ForwarderCallExecuted` events show the order in which the calls ran.
- The new field changes the ABI and the selector of `execute`. A deployed protocol adapter needs an implementation upgrade, and its clients need a release that sends the field, at the same time. A client that carries an order also signs the new delta message, and tools that decode `execute` calldata, such as indexers, need the new ABI.
- The field adds 64 bytes of calldata to each transaction, and 32 bytes for each call list when the transaction carries its order.

## Alternatives

- The transaction does not carry its order. The numbers of the call lists keep deciding the order of the call lists that no constraint orders, and the limit of 256 call-carrying resources stays.
- The transaction carries its order, and nothing signs it. Then whoever relays the transaction can replace or drop the order, or copy the transaction and run it first with another order, after which the original reverts on its nullifiers.
- The carried order must equal the order that the sort computes. This keeps the sort and its limit, and it executes the same order as now, so it solves neither problem above.
- The aggregation journal includes `callOrder`. This also binds the order, but it changes the guest and the statement of the aggregation proof, which is a larger change than the message of the delta proof.
- The generic call resource carries the order in its app data, and its resource logic checks a signature of the user. Then the user, and not the composer of the transaction, chooses the order. But the order names the call lists of other resources and other actions, and two resources can carry different orders.
