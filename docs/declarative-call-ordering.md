# Declarative call ordering

**Status: proposal for PA v2. Nothing here is implemented.**

Design sketch for letting solvers choose the execution order of forwarder calls
without the user having authorized a sequence. Motivated by the forum threads on
[the generic call forwarder][1] and [interoperability between the declarative and
imperative paradigms][2].

[1]: https://forum.anoma.net/t/supporting-arbitrary-evm-application-interactions-using-a-generic-call-forwarder-and-resource/2676
[2]: https://forum.anoma.net/t/interoperability-between-declarative-and-imperative-paradigms-via-pa/2686

## The problem

The ARM is declarative — a transaction states which resources are consumed and
created, not the sequence in which anything happens. The EVM is imperative: a
swap must be preceded by its approval, and two calls touching the same pool give
different results in different orders. Something has to decide the order, and the
choice of *who* decides is the design.

Three candidates from the thread: the user fixes it when constructing the
transaction (available today, sacrifices the declarative benefit), a solver
picks it (wanted, but must not let the solver tamper), or the protocol adapter
derives it on-chain (rejected — the adapter is deliberately more generic than any
application, and ordering heuristics do not belong in it).

## What the current design already fixes

Today the order is not merely conventional. It is authorized end-to-end, in three
places:

- Calls run in lexicographic appearance order — action index, consumed before
  created, resource index, `externalPayload` index (`_processAction`).
- The aggregation journal is `abi.encode(complianceKey, kindTableCommitment,
  actions)`, so every byte of every action, including each call blob and its
  array position, is committed by the aggregation proof.
- `transactionId` is the hash of the action tree root sequence, and the delta
  proof signs it, so action order is committed too.

On top of that, each resource logic's public instance carries its own `app_data`:

```rust
pub struct LogicInstance {
    pub tag: Digest,        // nullifier or commitment
    pub is_consumed: bool,
    pub root: Digest,       // the resource's own action tree root
    pub app_data: AppData,  // external_payload included
}
```

So a resource logic already constrains its own calls byte for byte, in order.
**Array position is a proven total order.** Nothing can be reordered after
proving, and the "user fixes the order" option therefore needs no change at all.

## Options considered

### A. Proven ordering field

Add an explicit position to each call, or an ordering vector to each action, and
commit it with the other proven data.

Inert. Within an action the user already controls the order by choosing array
positions, so an action-local ordering vector only restates what is there. A
global position written inside an action is worse: the resource logic proving it
cannot know how many actions exist, so it asserts a coordinate it cannot compute,
and the aggregation circuit acquires a cross-action permutation check that the
compliance and logic circuits deliberately never perform — breaking exactly the
context separation an action exists to provide.

Generalized: a proven ordering field can only restate array position. Option A is
the status quo wearing a new field.

### B. Unproven schedule

Add a solver-written schedule as a sibling of `actions`, covered by no proof.

Real solver freedom, and cheap. But nothing constrains what the solver writes,
and the `expectedOutput` equality check does not save you: a carrier emitting
`approve(dex, 0)` and `approve(dex, max)` returns `true` either way, so swapping
them passes every check and leaves a live max allowance. Unproven ordering is not
uniformly revert-safe.

### C. Proven constraints, unproven schedule — **chosen**

The user declares *what must hold*, the solver chooses *the sequence*, the
adapter checks the sequence against the declarations.

This is the only option that is actually declarative: A states a sequence, B
states nothing. It escapes A's redundancy objection because a cross-action
dependency is not expressible as an array position. And it dissolves the open
question of two parties both demanding position 0 — nobody writes positions.
Compatible constraints all hold; contradictory ones form a cycle and the
transaction reverts. There is no tiebreak to design.

## Data structures

### The one change to `Transaction`

```solidity
/// @param callSchedule The order in which forwarder calls are executed, given as indices into
/// the transaction's calls in appearance order. Empty means appearance order. Solver-provided
/// and covered by no proof — the constraints it must satisfy are proven, the schedule is not.
struct Transaction {
    Action[] actions;
    bytes deltaProof;
    bytes aggregationProof;
    uint256[] callSchedule; // NEW
}
```

`Action`, `Consumed`, `Created`, `AppData` and `ExpirableBlob` are untouched.
That is deliberate and load-bearing — see [Blast radius](#blast-radius).

### The widened call blob

An external-payload blob is `bytes`, opaque to the ARM, and decoded only by the
adapter. It is the one extension point that is already proven twice — by the
logic instance and by the aggregation journal — and costs nothing upstream to
widen. Naming the decoded shape puts it where the house style keeps ABI types:

```solidity
/// @notice The decoded content of an external-payload blob — one forwarder call.
/// @param untrustedForwarder The forwarder contract forwarding the call.
/// @param input The input data for the forwarded call.
/// @param expectedOutput The expected output data from the forwarded call.
/// @param predecessors Tags whose calls must all have run before this call.
struct ForwarderCall {
    address untrustedForwarder;
    bytes input;
    bytes expectedOutput;
    bytes32[] predecessors; // NEW
}
```

`predecessors` are **tags** — nullifiers or commitments. Tags are forced: a logic
cannot know a global call index (A's objection), but it can know a counterparty's
tag, because composition precedes proving. A logic cannot verify the referenced
resource is present either — it only sees its own `root` — so the adapter enforces
presence and reverts on a dangling edge.

The dependency is asymmetric on purpose: fine-grained on the dependent side (this
one call waits), coarse on the depended-upon side (a whole resource, never another
application's internal call index).

### Event attribution

```solidity
event ForwarderCallExecuted(
    bytes32 indexed tag,               // NEW — the carrier resource
    address indexed untrustedForwarder,
    bytes input,
    bytes output
);
```

Today a call is attributed to its resource by *event position* — it is emitted
between that action's state writes and its `ActionExecuted`. A global schedule
destroys that nesting, so the attribution has to move into the event itself.

## Execution and validation

A global schedule cannot be executed inside the per-action loop, so `execute`
splits into two passes:

```
pass 1  per action: check roots, add nullifiers, add commitments,
                    emit payload blobs, emit ActionExecuted
pass 2  validate the schedule, then run the calls in schedule order
```

Validation:

```
calls     = enumerated in appearance order: action → consumed, then created → payload index
schedule  = callSchedule.length == 0 ? identity : callSchedule

require schedule is a permutation of 0..callCount-1        // seen-bitmap
require calls of one resource keep their relative order    // implicit, from array position
for each call c, for each tag t in c.predecessors:
    require t is a tag in this transaction                 // dangling edge → revert
    require pos(last call of t) < pos(c)                   // cycles fail here naturally
```

The implicit intra-resource rule is not an extra constraint to encode — it is the
order the logic already proved via array position. Honoring it costs one
comparison per call and keeps every invariant a single logic can express,
including the inseparable `approve(0)` / `approve(max)` pair that sank option B.

`callSchedule` is all-or-nothing: either a complete permutation or empty. This
retires the two-phase rule floated in the thread (ordered calls first, unordered
after) — under C there is nothing to place "after", and the same edge check runs
in both cases.

### Worked example

Action 0 consumes an unwrap carrier `[release]` and creates a wrap carrier
`[pull]`; action 1 consumes a swap carrier `[approve(100), swap, approve(0)]`.

```
appearance order:  0 release   1 pull   2 approve(100)   3 swap   4 approve(0)

proven edges:      swap.predecessors = [unwrapCarrierNullifier]
                   pull.predecessors = [swapCarrierNullifier]

callSchedule = [2, 0, 3, 4, 1]   →   approve(100), release, swap, approve(0), pull
```

Valid: the swap carrier's calls hold their relative order (positions 0 < 2 < 3);
`release` precedes `swap` (1 < 2); every swap carrier call precedes `pull`
(0, 2, 3 < 4). The solver got its interleave — `release` landing between the
approval and the swap — without the user ever having written a sequence.

## Blast radius

Where a proven dependency *could* have lived, and what each choice would cost:

| Location | What changes |
|---|---|
| New field on `Action` | Solidity + aggregation circuit + journal encoding |
| New field or fifth payload kind in `AppData` | the above + `arm-risc0::AppData` + every resource logic's instance layout |
| **Widened call blob** (chosen) | Solidity decode + the resource-logic libraries that build blobs |

`arm-risc0` ships prebuilt circuit ELFs and is shared across every target chain,
so the third row is the only one that keeps this an EVM-adapter change.

Concretely:

- `arm-risc0` — unchanged. Blobs stay `Vec<u32>`; only the adapter interprets them.
- `generic_call_library`, `transfer_library` — emit the four-tuple.
- pa-evm crates — fill `call_schedule` in the conversion, default empty.
- The envio indexer — new `ForwarderCallExecuted` signature, and call ordering no
  longer nests inside actions.

## Consequences

- **State is fully settled before any external call.** Pass 1 writes every
  nullifier and commitment before pass 2 makes its first call, which is
  checks-effects-interactions by construction rather than by `nonReentrant`
  alone.
- **Widening the blob is a breaking format change** for every application that
  emits one, since a three-tuple no longer decodes. v2 is an ABI break anyway,
  but the break propagates into the resource-logic repos and has to be
  coordinated there.
- **`transactionId` does not cover `callSchedule`.** The same authorized
  transaction executed under two different schedules carries the same ID. Only
  one can ever land — the nullifier set sees to that — so the ID identifies the
  authorization, not the execution.
- **Two passes over the resources instead of one**, plus permutation and edge
  validation. The tag → position lookup wants transient storage (`evm_version =
  "osaka"` already allows `tstore`) rather than a linear scan per edge.

## Open questions

- **May a resource impose on a resource that did not agree?** Declaring `t` as a
  predecessor constrains `t`'s position as much as one's own. The damage is
  bounded to a revert, since an unsatisfiable graph fails validation and the
  whole transaction reverts atomically. Confirm that revert-only is the accepted
  failure mode.
- **Predecessors only, or successors too?** With predecessors only, "my call must
  run *before* yours" is declarable solely by the other party. Fully symmetric
  expressiveness would need a `successors` field, at the cost of letting a
  resource push others later. The one-sided case looks weak in practice — you
  rarely know at proving time about a call you want to precede — so the proposal
  ships predecessors only.
- **Should a partial schedule be allowed after all?** All-or-nothing is the
  simpler invariant, but a prefix-plus-appearance-order form would let a solver
  pin only the calls it cares about and shrink calldata.
