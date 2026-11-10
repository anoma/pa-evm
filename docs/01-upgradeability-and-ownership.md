# Upgradeability and Ownership

This document specifies how the protocol adapter can change after its deployment: the proxy, its owners, the actions of the owner and the controls for a flaw. The glossary [`CONTEXT.md`](../CONTEXT.md#language) defines the terms.

## 1. Contracts

- **Proxy.** An `ERC1967Proxy` at a CREATE2 address. The address depends on the salt of the environment and on the deployment code, which contains the first implementation and the first owner. Each upgrade keeps the address, the commitment tree, the nullifier set, the kind tree root and the denylists.
- **Implementation.** `ProtocolAdapter`. Its constructor stores the address of the RISC Zero router and the verifier selector as the immutables `RISC_ZERO_VERIFIER_ROUTER` and `RISC_ZERO_VERIFIER_SELECTOR`. The code also contains `VERSION` and the verifying keys of the compliance circuit and the aggregation circuit (`VerifyingKeys._COMPLIANCE`, `VerifyingKeys._BATCH_AGGREGATION_EVM`). So a change of any of these values needs an upgrade. After an upgrade that changes a verifying key or the verifier selector, each proof for the old values fails. Both environments deploy an implementation with the same CREATE2 salt, so they use the same implementation of a version on a chain.
- **State.** The four state contracts keep their data in the ERC-7201 namespaces `anoma.storage.CommitmentTree`, `anoma.storage.NullifierSet`, `anoma.storage.KindTreeRoot` and `anoma.storage.LogicRefDenylist`. The recipe `contracts-storage-check` fails if a contract uses a normal storage slot.
- **RISC Zero router.** It sends each proof to the verifier of the selector in the first four bytes of the proof. The protocol adapter accepts only proofs with its own selector, else it reverts with `RiscZeroVerifierSelectorMismatch`.

## 2. Owners and their actions

| Environment | Owner                                                              |
| ----------- | ------------------------------------------------------------------ |
| Staging     | The deployment wallet `0x61462bE56782568376f9cB069382EFa72764a407` |
| Production  | The PA Safe `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`           |

The owner can do these actions. All of them except `pause()` also work while the protocol adapter is paused. All of them except `transferOwnership` revert while `execute` runs, so that they cannot change a transaction in progress ([`04-resource-order-and-external-calls.md`](./04-resource-order-and-external-calls.md#4-properties-of-the-external-calls)).

- `upgradeToAndCall(newImplementation, data)` replaces the implementation.
- `pause()` stops `execute`, and `unpause()` lets it run again.
- `setKindTreeRoot(root)` sets the kind tree root.
- `denyLogicRefs(entries)` deprecates or denies logic refs.
- `transferOwnership(newOwner)` gives the ownership to another account in one step.

## 3. Controls for a flaw

The protocol adapter accepts every valid proof, from any prover. So only contracts on the chain can stop the use of a flaw:

- **Pause.** The protocol adapter executes no transaction until the owner lifts the pause.
- **Denial of a logic ref.** The protocol adapter reverts each transaction that consumes or creates a resource with that logic ref ([`03-kind-tree-root-and-denylists.md`](./03-kind-tree-root-and-denylists.md)).
- **Upgrade.** A new implementation replaces flawed code, a flawed verifying key or the verifier selector.
- **Emergency stop of a RISC Zero verifier.** For the selector of the protocol adapter, the RISC Zero router sends each proof through an emergency stop contract. Its guardian can stop the verifier, and so can anyone who shows a valid proof of a false claim. No function restarts a stopped verifier. From then on, `execute` reverts, and `riscZeroVerifierPaused()` returns `true`.

| Flaw in                                                                 | Controls                                                                                                                                                                                    |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| The protocol adapter, the compliance circuit or the aggregation circuit | The pause, and an upgrade of the protocol adapter                                                                                                                                           |
| The RISC Zero verifier                                                  | The pause or the emergency stop of the verifier. The owner of the RISC Zero router adds a correct verifier with a new selector, and an upgrade of the protocol adapter takes that selector. |
| A resource logic or a forwarder                                         | The pause, the denial of the logic ref of a flawed resource logic, and new contracts of the application                                                                                     |

## 4. Trust assumptions

- **The owner upgrades only to reviewed code.** An upgrade replaces all code, also the code that reads and writes the state, so the owner can change every rule. In production, the PA Safe signers decide each upgrade.
- **The owner transfers the ownership only to a correct address.** `transferOwnership` completes in one step, so nobody can undo a transfer to a wrong address. `renounceOwnership()` removes the owner, and without an owner nobody can upgrade, pause or deny.
- **The PA Safe signers respond to a flaw in time.** In production, each response is a Safe transaction, except the emergency stop of a RISC Zero verifier by a guardian that is not the PA Safe. On each chain where the team deployed the RISC Zero router, for example BSC, the PA Safe owns the router and is the guardian.
- **The owner of the RISC Zero router removes a verifier only after a flaw.** The owner can remove the verifier of a selector, and then `execute` and `riscZeroVerifierPaused()` revert with `SelectorRemoved`. The router does not let its owner replace the verifier of a selector in use (`SelectorInUse`), so the owner cannot make the protocol adapter accept a false proof. The package `anoma-risc0-deployments` records for each chain whether the team deployed the router (`isAnomaDeployed`). RISC Zero deployed the others.
- **The PA Safe signs each Safe transaction with `safeTxGas` 0 and `gasPrice` 0.** With these values, if the call of a Safe transaction reverts, the Safe transaction reverts too, and its nonce stays unused. Any account can execute a Safe transaction that has enough signatures, also during an external call, and there `upgradeToAndCall`, `pause`, `unpause`, `setKindTreeRoot` and `denyLogicRefs` revert. So with other values, any account can make such a Safe transaction fail and use up its nonce, and the signers must sign a new one.

## 5. Parameters

| Name                               | Value                                        | Use                                                       |
| ---------------------------------- | -------------------------------------------- | --------------------------------------------------------- |
| `Parameters.PROXY_SALT_STAGING`    | `"ProtocolAdapterProxyStaging"`              | CREATE2 salt of the staging proxies                       |
| `Parameters.PROXY_SALT_PRODUCTION` | `"ProtocolAdapterProxyProduction"`           | CREATE2 salt of the production proxies                    |
| `Parameters.IMPLEMENTATION_SALT`   | `"ProtocolAdapterImpl"`                      | CREATE2 salt of the implementations, in both environments |
| `Parameters.DEPLOYMENT_WALLET`     | `0x61462bE56782568376f9cB069382EFa72764a407` | Owner in staging                                          |
| `Parameters.PA_MULTISIG`           | `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10` | Owner in production                                       |

The RISC Zero router of each chain and the Groth16 verifier selector, which is the same on all chains, come from the package `anoma-risc0-deployments`. The deployment record, [`crates/bindings/deployments.json`](../crates/bindings/deployments.json), lists the protocol adapters of each environment and the legacy protocol adapter of each chain.
