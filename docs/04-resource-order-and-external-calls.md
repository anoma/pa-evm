# Resource Order and External Calls

This document specifies how the protocol adapter processes the resources of an action, and how the order of the resources sets the order of the external calls. The glossary [`CONTEXT.md`](../CONTEXT.md#language) defines the terms.

## 1. How the protocol adapter processes an action

The protocol adapter receives each action as one ordered list of resources, `Action.resources`. Consumed and created resources can occur in any order in the list. For each resource of the list, in its order, `execute` does these steps:

1. It checks the denylist of the side of the resource (`ResourceWithDeniedLogicRef`).
2. For a consumed resource, it checks that the stated commitment tree root is a historical root (`NonExistingRoot`) and adds the nullifier to the nullifier set (`PreExistingNullifier`). For a created resource, it adds the commitment to the commitment tree.
3. It makes the external calls of the resource, in the order of the blobs of its external payload. It decodes each blob as a forwarder address, an input and an expected output. It calls `forwardCall(logicRef, input)` on the forwarder, with the logic ref of the resource. The output must be equal to the expected output (`ForwarderCallOutputMismatch`). It emits `ForwarderCallExecuted`.
4. It emits the app data blobs of the resource whose deletion criterion is `Never`.

After the last resource, it emits `ActionExecuted`. After the last action, it verifies the delta proof and the aggregation proof, stores the new commitment tree root if the transaction added commitments, and emits `TransactionExecuted`. So the external calls of a transaction run action by action, resource by resource, and blob by blob.

## 2. Why the order matters

An application can put the resources of several applications into one action and depend on the order of their calls. For example, one action swaps token A for token B through the generic call forwarder:

1. A consumed ERC20 resource of token A, and a created ephemeral ERC20 resource whose external call unwraps token A to the generic call forwarder.
2. A consumed generic call resource whose external call makes the generic call forwarder swap token A for token B on an exchange.
3. A consumed ephemeral ERC20 resource whose external call wraps token B from the generic call forwarder, and a created ERC20 resource of token B.

The swap succeeds only if the unwrap runs before the swap, and the swap before the wrap. A wrap before the unwrap finds no tokens and reverts. The integration test `forwarder_call_order.rs` of the ERC20 forwarder checks both orders, and the integration test `swap_erc20s_on_a_dex.rs` of the generic call forwarder checks this swap.

## 3. What binds the order

- **The compliance proof** covers the resources in the order of the list.
- **The action tree root** is calculated over the tags of the resources in the order of the list. The resource logic proofs and the delta proof cover it, and the ERC20 resource logic requires the signature of the owner over it. So another order invalidates them.
- **The aggregation proof** covers the actions as the protocol adapter receives them, because the protocol adapter calculates the journal from `transaction.actions`. So the protocol adapter processes exactly the proven order.
- **The created nonces** derive from the nullifiers in the order of the list.

## 4. Properties of the external calls

- **External calls run before the proofs are checked.** The protocol adapter verifies the delta proof and the aggregation proof after the last action. A failed proof reverts the transaction, with the effects of all external calls.
- **A forwarder sees a part of the new state.** During an external call, the nullifier set and the commitment tree contain the nullifiers and the commitments of the earlier resources and of the resource that makes the call. The new latest root of the commitment tree is not yet a historical root.
- **A forwarder cannot call `execute` again.** A transient reentrancy guard on `execute` reverts such a call.
- **Each blob of the external payload is a call**, whatever its deletion criterion. A blob that does not decode as a forwarder address, an input and an expected output reverts the transaction.
- **The forwarder checks the logic ref.** The protocol adapter calls any address that a proven external call names, with the logic ref of the resource. So each forwarder needs an out-of-circuit check that accepts only the logic refs of its own resources. The ERC20 forwarder and the generic call forwarder each accept one logic ref.

## 5. Trust assumptions

- **The in-circuit checks cover the rules of the list.** The compliance checks and the in-circuit checks of the aggregation circuit cover, for example, that an action consumes at least one resource and that the action tree root is calculated over the tags in the order of the list. The protocol adapter does not repeat them in out-of-circuit checks. The commitments are unique because of an in-circuit check and an out-of-circuit check: the compliance checks require that each nonce derives from the consumed nullifiers, and the nullifier set rejects a repeated nullifier.
