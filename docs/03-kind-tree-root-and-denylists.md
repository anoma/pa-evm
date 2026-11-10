# Kind Tree Root and Denylists

This document specifies two controls of the protocol adapter: the kind tree root, which decides which kinds are aliases, and the two denylists, which stop the creation or the consumption of resources with a logic ref. Only the owner changes them. Both changes are possible while the protocol adapter is paused, but not while `execute` runs. The soft migration and the hard migration use these controls. The glossary [`CONTEXT.md`](../CONTEXT.md#language) defines the terms.

## 1. The kind tree root

The compliance circuit can read the kind base of a resource from a kind tree instead of calculating it from the kind. Its proof shows the root of the kind tree that it used. The repository risc0-kind-trees makes the kind trees and publishes their roots.

The protocol adapter stores one kind tree root. A transaction states the kind tree root of its proofs in `Transaction.kindTreeRoot`, and the aggregation proof covers this value. The protocol adapter accepts two values ([ADR-0004](./adr/0004-the-transaction-carries-the-kind-tree-root.md)):

- the stored root, and
- `EMPTY_KIND_TREE_ROOT`, the root of the kind tree without entries.

It makes this out-of-circuit check before it processes the actions. On any other value, it reverts with `UnacceptedKindTreeRoot`.

With the empty kind tree, each kind base comes from its kind. So a transaction that balances with the empty kind tree also balances with the stored kind tree. Only a conversion between aliases needs the stored kind tree. For this reason, a kind tree can make kinds aliases, but it cannot stop a kind. A prover that converts nothing can always use the empty kind tree, and its proofs stay valid when the owner sets a new root.

`setKindTreeRoot(root)` sets the stored root and emits `KindTreeRootUpdated`. It reverts with `ZeroKindTreeRootNotAllowed` for zero. `getKindTreeRoot()` returns the stored root.

## 2. The denylists

The protocol adapter keeps two denylists of logic refs: one for consumed resources and one for created resources. A transaction reverts with `ResourceWithDeniedLogicRef(logicRef, consumed)` if it consumes a resource whose logic ref is on the list for consumed resources, or creates a resource whose logic ref is on the list for created resources.

| Status of a logic ref | Consumption denied | Creation denied |
| --------------------- | ------------------ | --------------- |
| Allowed               | no                 | no              |
| Deprecated            | no                 | yes             |
| Denied                | yes                | yes             |

`denyLogicRefs(entries)` adds entries. Each entry is a `DeniedLogicRef{logicRef, consumed}`: `consumed = true` selects the list for consumed resources, and `false` the list for created resources. The function reverts with `ZeroLogicRefNotAllowed` for zero and with `LogicRefAlreadyDenied` for an entry that its list already contains. It emits `LogicRefDenied` for each entry. `isLogicRefDenied(logicRef, consumed)` returns whether a list contains a logic ref. No function removes an entry.

An entry only on the list for consumed resources is also possible. Then transactions can create resources that no transaction can consume.

The lists contain logic refs, not kinds. So an entry applies to the resources of that logic ref with any label ref, also to the resources of other applications.

## 3. Use in the soft and the hard migration

|           | Soft migration                                                                                              | Hard migration (draft)                                                                                                                                                                                                                      |
| --------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose   | Users replace resources of a resource logic that is still trusted.                                          | Users replace resources of a resource logic with a flaw.                                                                                                                                                                                    |
| Denylists | The owner deprecates the old logic ref. No transaction creates old resources.                               | The owner denies the old logic ref. No transaction consumes or creates old resources.                                                                                                                                                       |
| Kind tree | Each old kind is an alias of a new kind. A conversion consumes old resources with a proof of the old logic. | Not used for the old resources. The in-circuit checks of the new logic show that the user owns them. The application makes out-of-circuit checks against the protocol adapter that they exist and are unspent, and stores their nullifiers. |

## 4. Trust assumptions

- **The owner sets only reviewed kind tree roots.** The stored kind tree decides which kinds are aliases. An entry that makes two kinds aliases lets users convert one into the other at the same quantity, so a wrong entry can let users take tokens that back other resources. The kind trees repository publishes roots after a review.
- **No upgrade removes a denylist entry.** No function removes an entry, but a new implementation can change the lists.
- **The owner deprecates or denies only replaced or flawed logic refs.** The denylists apply to all applications, so the owner can stop the resources of any resource logic.

## 5. Parameters

| Name                   | Value                                                                                            | Use                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| `EMPTY_KIND_TREE_ROOT` | `0xcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06`, which is `sha256("EMPTY")` | Root of the kind tree without entries, which the protocol adapter always accepts |
