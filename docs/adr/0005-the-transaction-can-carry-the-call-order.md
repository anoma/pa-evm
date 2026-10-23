# The transaction can carry the call order

## Context

The protocol adapter executes the call lists of a transaction in an order that it computes, as [the design of the external call ordering](../external-call-ordering.md) describes. Without ordering constraints, this is the order of the transaction. With ordering constraints, it is a topological sort that takes the call list with the lowest number first. So where the constraints leave two call lists unordered, their order depends on their numbers, and an action numbers its consumed resources before its created ones.

Whoever composes a transaction orders its actions, but not the resources inside an action, since each logic proof of an action commits to the action tree root. So among the orders that the constraints allow, the composer cannot choose one that runs a created call list before a consumed call list of the same action. If only the generic call resource states ordering constraints, such an order needs a generic call between the two call lists. When no generic call belongs there, as when an unwrap must run before a wrap of the same action, it takes an extra generic call resource whose call does nothing.

Ordering information in the label or value of a resource does not solve this. The commitment of a created resource depends on the nullifiers of the consumed resources of its action, so the label or value of such a consumed resource cannot name it. And a persistent consumed resource got its value from the transaction that created it, before the transaction that consumes it existed.

The sort also holds two sets of 256 bits for each call list, so a transaction with ordering constraints can have at most 256 call-carrying resources.

## Decision

The protocol adapter gets a second entry point, `executeWithCallOrder(Transaction calldata transaction, uint256[] calldata callOrder)`, and `execute` does not change. An entry of `callOrder` is the number of a call list: the call lists are numbered from 0 in the order of the transaction, as `ForwarderCallOrdering.decodeCallLists` returns them. The delta proof signs the order.

- If `callOrder` is empty, `executeWithCallOrder` does what `execute` does. The delta proof signs `transactionId`, and the protocol adapter computes the order.
- Otherwise, the delta proof signs `keccak256(abi.encode(transactionId, callOrder))` instead of `transactionId`. After the proofs and before any forwarder call, the protocol adapter decodes the call lists as it does for ordering constraints, whether or not a call carries one, checks `callOrder`, and executes the call lists in this order.

The check has three steps, in this order. A step runs only if the steps before it passed.

1. `callOrder` names each call list exactly once: its length is the number of call lists, and each entry is below this number and appears once. Otherwise the protocol adapter reverts with `CallOrderNotAPermutation()`. A transaction without call-carrying resources reverts with this error if its `callOrder` is not empty.
2. The action indices of the call lists never decrease along `callOrder`. So the actions keep the order of the transaction, and the call lists of each action stay together. Otherwise it reverts with `CallOrderNotInActionOrder(uint256 actionIndex)`, which names the action index of the first entry where they decrease.
3. Every ordering constraint holds. The protocol adapter visits the constraints as `_buildConstraints` does: call list by call list, call by call, the `afterTags` of a call before its `beforeTags`. As now, each entry must name a call-carrying resource, found with the first-match rule of `_indexOf`. A call list that an `afterTags` entry names must have a lower position in `callOrder` than the call list of the call, and one that a `beforeTags` entry names a higher position. Otherwise it reverts with `CallOrderViolatesConstraint(bytes32 tag)`, which names the entry. A tag that names the resource of its own call fails this comparison.

The check keeps the position of each call list in an array of one word per call list, and it builds no graph. With `_indexOf`, it costs O(n + n·t) for n call lists and t tag entries, which the submitter pays.

## Consequences

- The delta proof binds the order. A relayer that changes `callOrder`, or submits the transaction to `execute` without it, makes the delta proof fail, so it cannot copy the transaction and run it first with another order. The holder of the delta witness chooses the order: the user when the user builds the transaction alone, and the composer when one composes it. Partial transactions carry their delta witnesses until they are composed, so anyone who saw the pieces can sign an order, as they can already order the actions.
- The composer chooses the order of the call lists inside each action, within what the constraints allow, which the provers of the action fix now. A user who relies on the positions of the resources in its action for the order of two call lists must state that order with an ordering constraint. A resource whose logic states no constraints, such as one of the token transfer, cannot keep its position.
- An order that passes the check meets the rules of the design document. With every tag naming a call-carrying resource and at most 256 call-carrying resources, the sort finds an order exactly when such an order exists. A carried order is narrower in one way: it keeps the actions in the order of the transaction. A transaction whose constraints run a later action first therefore cannot carry its order, but its actions can be put in the order of the constraints instead.
- Above 256 call-carrying resources, only a transaction that carries its order can have ordering constraints.
- A cycle, or a tag that names the resource of its own call, reverts with `CallOrderViolatesConstraint` instead of `CyclicOrderingConstraints`.
- `transactionId` and the `TransactionExecuted` event do not depend on the order. The `ForwarderCallExecuted` events show the order in which the calls ran.
- `execute`, its selector and `Transaction` do not change, so clients, bindings and indexers keep working, and only a transaction that carries an order pays for it in calldata: 64 bytes, and 32 bytes for each call list. A deployed protocol adapter gets the new function with an implementation upgrade.
- A client that carries an order calls `executeWithCallOrder` and signs the new delta message. The ARM signs and verifies a delta message built from the action tree roots alone, so it needs a change before a client built on it can carry an order.

## Alternatives

- The transaction does not carry its order. The numbers of the call lists keep deciding the order of the call lists that no constraint orders, and the limit of 256 call-carrying resources stays. An order that the composer cannot reach needs a generic call resource whose call does nothing, with its proofs and a forwarder call.
- The order is a new last field of `Transaction`. This changes the ABI and the selector of `execute`, so every client needs a release at the time of the upgrade, and every transaction pays 64 bytes of calldata for an empty order.
- The transaction carries its order, and nothing signs it. Then whoever relays the transaction can replace or drop the order, or copy the transaction and run it first with another order, after which the original reverts on its nullifiers.
- The carried order must equal the order that the sort computes. This keeps the sort and its limit, and it executes the same order as now, so it solves neither problem above.
- The aggregation journal includes `callOrder`. This also binds the order, but it changes the guest and the statement of the aggregation proof, which is a larger change than the message of the delta proof.
- Each action carries the order of its own call lists, in a new field of `Action`, numbered within the action. The numbers then do not change when actions are composed. But the aggregation journal encodes the actions, so the guest must commit to the new field. Otherwise the delta proof signs it, and the composer chooses the order, as with `callOrder`.
- The generic call resource carries the order of the call lists of its action in its app data, numbered within the action. The logic proof of the resource binds the order to the user who proves it, so the composer cannot change it, and the order depends on no tag and on no other action. But the generic call logic must accept the order, the protocol adapter needs a place in the app data to find it, two resources of an action can carry different orders, and an action without a generic call cannot carry one.
