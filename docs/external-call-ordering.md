# External call ordering

This document describes the order in which the protocol adapter (PA) executes the external calls of a transaction, and the reasons for this design. An external call is a call from the PA to a forwarder contract. A resource carries it in the external payload of its app data. The code and the [glossary](../CONTEXT.md) name such a call a _forwarder call_, and this document uses that name. A resource with forwarder calls is a _call-carrying resource_, and its forwarder calls are its _call list_.

## The call execution is decoupled from the ARM execution

The PA executes a transaction in two parts:

1. The resource machine (ARM) execution. For each action, in the order of the transaction, the PA checks the resources, adds the nullifiers of the consumed resources to the nullifier set, adds the commitments of the created resources to the commitment tree, and emits the events of the action. Then it verifies the delta proof and the aggregation proof, and it stores the new root of the commitment tree.
2. The call execution. The PA executes the forwarder calls of all resources, in the order that this document describes.

The PA decouples the two parts on purpose.

- The ARM is designed to be independent of order. It is declarative: a transaction states which resources it consumes and which it creates.
- The EVM is imperative: a call changes the state that later calls read. So in a transaction, only the external calls, which give the ARM access to EVM contracts, can depend on an order.
- Some applications have external calls that depend on each other, and they must be able to state these dependencies. A swap, for example, must unwrap the sold tokens before it swaps them, and swap them before it wraps the bought tokens. The ordering constraints state such dependencies.

Because the call execution comes after the ARM execution, a call that reads the nullifier set finds the nullifiers of all consumed resources of the transaction. This is a side effect of the design, not a goal.

## Ordering constraints

Each blob in the external payload of a resource encodes one forwarder call, as `abi.encode` of the five fields of `IProtocolAdapter.ForwarderCall`:

```solidity
abi.encode(address untrustedForwarder, bytes input, bytes expectedOutput, bytes32[] afterTags, bytes32[] beforeTags)
```

The PA calls `forwardCall(logicRef, input)` on `untrustedForwarder`, with the logic reference of the resource. If the output of the call differs from `expectedOutput`, the transaction reverts with `ForwarderCallOutputMismatch`. After each call, the PA emits a `ForwarderCallExecuted` event, so the events show the order of execution.

`afterTags` and `beforeTags` hold the ordering constraints of the call. Each entry is the tag of a call-carrying resource of the transaction: the nullifier of a consumed resource or the commitment of a created resource.

- An `afterTags` entry makes the calls of the named resource come before the calls of this resource.
- A `beforeTags` entry makes the calls of the named resource come after the calls of this resource.

The constraints of a call hold for all calls of its call list.

Either of the two resources can carry a constraint. A `beforeTags` entry of resource A that names resource B and an `afterTags` entry of B that names A give the same order: the calls of A before the calls of B. The choice matters for the resource logic. The compliance circuit derives the nonce of each created resource from the nullifiers of the consumed resources of its action. So the commitment of a created resource depends on each consumed resource of its action, and such a consumed resource cannot hold that commitment in its label or value. If a resource logic requires its tags in the label or value of its resource, a constraint between a consumed and a created resource of one action goes on the created resource.

The aggregation proof covers the actions with their app data. So the resource logic of a call-carrying resource can check its forwarder calls and their tags. The PA does not check whether an order suits the application. It executes the order that the tags set.

## The legacy call encoding

The legacy protocol adapter accepted forwarder calls without the two tag lists, as `abi.encode(address untrustedForwarder, bytes input, bytes expectedOutput)`. The PA cannot decode a call in this encoding: it reads the offsets of the two tag lists from words that hold the input of such a call. No transaction for the PA needs this encoding:

- Only ephemeral resources carry forwarder calls. In the ERC20 logic, these are the wrap and the unwrap, and the generic call logic accepts only ephemeral resources. The ERC20 logic and the logic of the legacy ERC20 forwarder both give a persistent resource an empty external payload.
- A client makes each ephemeral resource new for its transaction. The logics for the PA use arm-risc0 v2.0.0-rc.7.1 or later, which encodes the two tag lists.

So the resources of the legacy protocol adapter that a migrated chain still holds carry no forwarder calls: they are persistent.

## Boundaries

- Only call-carrying resources take part in the order. A tag that names a resource without forwarder calls sets no order, so the client most likely named the wrong resource. Such a tag, and a tag that names no resource of the transaction, make the transaction revert with `AfterTagNamesNoCallCarryingResource` or `BeforeTagNamesNoCallCarryingResource`.
- The constraints apply inside one transaction. The PA does not look up the nullifier set or the commitment tree for a tag.
- The calls of one resource stay together. The PA executes them in the order of the external payload. Constraints that divide them have no order.
- The calls of one action stay together. The PA never interleaves them with the calls of another action. So a constraint between resources of two actions puts all calls of one action before all calls of the other. Constraints that interleave two actions have no order.
- The order does not change the ARM execution, which ends before the first call.

## The graph problem

Without ordering constraints, the PA executes the call lists in the order of the transaction: action by action, and in each action the call lists of the consumed resources before those of the created resources.

With ordering constraints, the PA solves a graph problem. A graph has nodes and edges, and an edge from node A to node B means that A comes before B. The in-neighbors of a node are the nodes with an edge to it, and its out-neighbors are the nodes that it has an edge to.

1. Each call list is a node. The nodes are numbered in the order of the transaction.
2. Each tag gives an edge from the call list that comes first to the call list that comes second. If the two call lists belong to one action, the PA adds this edge. If they belong to two actions, it adds an edge from each call list of the first action to each call list of the second action. These edges keep the calls of each action together.
3. The PA sorts the nodes so that each node comes after its in-neighbors. This is a topological sort, made with Kahn's algorithm. Of the nodes whose in-neighbors are all sorted, the node with the lowest number comes next.
4. If the edges form a cycle, no order meets the constraints, and the transaction reverts with `CyclicOrderingConstraints`. An edge from a call list to itself is a cycle too, so a resource cannot name its own tag.

The lowest-number rule makes the order unique. A client that builds or simulates a transaction computes the same order as the PA, from the transaction alone. A graph without edges gives the order of the transaction, so a transaction without ordering constraints follows the same rule.

For each node, the graph holds the in-neighbors and the out-neighbors, each as a set of 256 bits. So a transaction with ordering constraints can have at most 256 call-carrying resources, and with more it reverts with `TooManyCallCarryingResources`. A transaction without ordering constraints builds no graph and has no such limit. The actions are not nodes, so the graph sets no limit on their number.

Without ordering constraints, the PA reads only the lengths of the two tag lists of each call. With ordering constraints, it also decodes all calls into memory, and it builds and sorts the graph.

## A carried call order

A transaction can also come with the order itself, through `executeWithCallOrder`, whose `callOrder` lists the numbers of its call lists in the order in which their calls run. `execute`, and `executeWithCallOrder` with an empty `callOrder`, compute the order as described above. Otherwise:

- The delta proof signs `keccak256(abi.encode(transactionId, callOrder))` instead of the transaction ID, so whoever relays the transaction cannot change or drop the order. Whoever holds the delta witness, such as the composer, chooses it.
- The PA builds no graph. It checks that `callOrder` names each call list exactly once (`CallOrderNotAPermutation`), that it runs the call lists of an action together (`CallOrderSplitsAction`), and that every ordering constraint holds (`CallOrderViolatesConstraint`), in this order. Each tag must still name a call-carrying resource.
- As with the computed order, the calls of an action run together, but the actions can run in any order that the constraints allow. So every order that `execute` computes is also a valid `callOrder`.
- The same limit of 256 call-carrying resources applies once a call carries ordering constraints, which bounds the search for each tag.

The ordering constraints stay the rules of the applications, which the logic proof of each resource binds through its app data, and the call order is the choice of whoever holds the delta witness within them.

## Examples

### A swap in one action

| Node | Resource                      | Side     | Calls                    | Ordering constraints                            |
| ---: | ----------------------------- | -------- | ------------------------ | ----------------------------------------------- |
|      | the sold tokens               | consumed | none                     | none                                            |
|    0 | the generic call              | consumed | the swap on the exchange | `afterTags`: the unwrap, `beforeTags`: the wrap |
|    1 | the wrap of the bought tokens | consumed | the wrap                 | none                                            |
|    2 | the unwrap of the sold tokens | created  | the unwrap               | none                                            |
|      | the bought tokens             | created  | none                     | none                                            |
|      | the change                    | created  | none                     | none                                            |

The edges are 2 → 0 and 0 → 1. Only node 2 has no in-neighbor, so the order is 2, 0, 1. The unwrap moves the sold tokens to the generic call forwarder, the generic call swaps them for the bought tokens, and the wrap moves the bought tokens to the ERC20 forwarder. In the order of the transaction, the generic call would come first, before the generic call forwarder has the sold tokens.

The generic call carries both constraints, so the wrap and the unwrap carry none. Its `afterTags` entry names the unwrap, a created resource of the same action, whose commitment depends on the nullifier of the generic call resource. So the generic call logic must not require its tags in the label or value of its resource.

### Constraints that interleave two actions

Action 0 has a consumed resource A and a created resource B, both with calls. Action 1 has a consumed resource C with calls, which names A in its `afterTags` and B in its `beforeTags`. These constraints need the order A, C, B, which interleaves the calls of the two actions. The `afterTags` entry adds edges from A and from B to C, and the `beforeTags` entry adds edges from C to A and to B. These edges form a cycle, so the transaction reverts.

## Code

- [`ProtocolAdapter.execute` and `executeWithCallOrder`](../contracts/src/ProtocolAdapter.sol) execute the two parts. Their function `_executeForwarderCalls` executes the calls in the call order that comes with the transaction, or else in the order of the transaction, or in the order of the constraints if a call has an ordering constraint.
- [`ForwarderCallOrdering`](../contracts/src/libs/ForwarderCallOrdering.sol) decodes the call lists, checks the tags, and builds and sorts the graph. Its function `checkCallOrder` checks a carried call order.
- [`DirectedGraph`](../contracts/src/libs/DirectedGraph.sol) holds the graph and its topological sort.
- [`call_order`](../crates/bindings/src/call_order.rs) signs a call order with the delta witness, for `executeWithCallOrder`.
- Tests: [`ProtocolAdapter.forwarderCallOrdering.t.sol`](../contracts/test/ProtocolAdapter.forwarderCallOrdering.t.sol), [`ForwarderCallOrdering.t.sol`](../contracts/test/ForwarderCallOrdering.t.sol) and [`DirectedGraph.t.sol`](../contracts/test/DirectedGraph.t.sol).
