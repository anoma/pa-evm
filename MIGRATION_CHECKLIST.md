# Migration Checklist

How to move a chain from its legacy protocol adapter to the protocol adapter and keep its commitment tree and nullifier set. A chain deployed fresh skips all of this and starts on `ProtocolAdapter`. [`docs/01-upgrades-and-state-migration.md`](./docs/01-upgrades-and-state-migration.md#6-state-migration) explains the whole move and the order of its steps; this file covers the protocol adapter's part.

## How it works

A chain that ran a legacy protocol adapter cannot start the protocol adapter empty. Its proxy starts on [`MigrationalProtocolAdapter`](./contracts/src/MigrationalProtocolAdapter.sol), a protocol adapter that begins paused and lets its owner copy the state of the legacy protocol adapter in. The state moves in two runs. In the migration run, [`MigrateProtocolAdapterState`](./contracts/script/migration/MigrateProtocolAdapterState.s.sol) copies the state, and the proxy stays paused. In the completion run, [`FinalizeProtocolAdapterStateMigration`](./contracts/script/migration/FinalizeProtocolAdapterStateMigration.s.sol) unpauses, upgrades the proxy to `ProtocolAdapter` and, in production, transfers the proxy to the production proxy owner. After that upgrade the chain runs the same code as every other chain, and the copy-in functions are gone.

The ERC20 forwarder balances move between the two runs. The kind table on the proxy carries members under the label of the legacy ERC20 forwarder, which let a legacy-forwarder resource unwrap from the environment's ERC20 forwarder. A paused proxy executes nothing, so no legacy-forwarder resource can unwrap before the environment's ERC20 forwarder holds the tokens of the legacy ERC20 forwarder.

The migrational implementation adds two functions. Both are owner-only, and both are allowed only while the adapter is paused and the legacy protocol adapter is stopped.

- `migrateCommitmentTree` writes the tree in one call. The caller supplies the stored sides, the one part of the tree that the legacy protocol adapter exposes through no getter. The leaf count and the root come from the legacy protocol adapter, and the empty-subtree roots are recomputed from the depth. The call reverts unless the result reproduces the root of the legacy protocol adapter, so a wrong set of sides cannot reach storage.
- `migrateNullifierSet` copies a batch of nullifiers, reading each one from the legacy protocol adapter. The caller gives the index in the legacy protocol adapter that the batch starts at. The call reverts unless the batch starts where the copied nullifiers end, so no nullifier can be skipped or copied twice.

`unpause` proceeds only once the adapter holds what the stopped legacy protocol adapter holds: the same commitment count, the same latest root, and every nullifier. The batches copy the nullifiers of the legacy protocol adapter in order, so the adapter checks only the last one. The historical roots are the one difference that stays. The legacy protocol adapter keeps every root it had. This adapter keeps two: the empty-tree root, which a resource created and consumed in one transaction proves membership against, and the latest root of the legacy protocol adapter.

Neither function is closed by the contract. The upgrade to the plain implementation removes them, so a migration must always reach the end of the completion run.

## Before any chain

- [ ] Deploy the plain implementation on the chain. The migration and completion runs refuse to start without it.

  ```sh
  just contracts-deploy-impl deployer <CHAIN>
  ```

- [ ] Verify the plain implementation on sourcify and Etherscan, with the address the deployment printed:

  ```sh
  just contracts-verify-impl <IMPLEMENTATION> <CHAIN>
  ```

- [ ] Deploy the proxy on the migrational implementation. It starts paused. It reads the chain's legacy protocol adapter from [`RecordedDeployments`](./contracts/generated/RecordedDeployments.sol), which is generated from the `legacy` entries in [`deployments.json`](./crates/bindings/deployments.json).

  ```sh
  export IS_PRODUCTION=<true|false>
  export IS_MIGRATIONAL=true
  just contracts-simulate-proxy <CHAIN>
  just contracts-deploy-proxy deployer <CHAIN>
  ```

  The proxy gets the staging proxy owner, the deployment wallet, in both environments, because the migration and completion runs send their calls from the owner. A production proxy moves to the production proxy owner at the end of the completion run. Until then, the staging recipes act on it too, because they check the owner and not the environment.

- [ ] Verify the migrational implementation and the proxy on sourcify and Etherscan, with the addresses the deployment printed. With `IS_MIGRATIONAL=true`, the recipe verifies the implementation as `MigrationalProtocolAdapter`.

  ```sh
  just contracts-verify-deployment <MIGRATIONAL_IMPLEMENTATION> <PROXY> <CHAIN>
  ```

- [ ] Record the proxy in `deployments.json`, regenerate `RecordedDeployments` and the bindings with `just contracts-gen`, and release the bindings, as for a chain new to an environment in [`RELEASE_CHECKLIST.md`](./RELEASE_CHECKLIST.md). The migration and completion runs read the proxy and the legacy protocol adapter from the regenerated library, so they refuse to start without this record. The kind tables take their chains from it too. On both promotion gates, the bindings tests accept a recorded proxy that runs the migrational implementation this source predicts, and print an info line for the chain. The production gate also requires a Safe to own the proxy, and the deployment wallet owns it until the completion run transfers it, so that gate fails for this chain until then. Do not promote production before the check at the end.

- [ ] Deploy the ERC20 forwarder and the generic call forwarder against the proxy, and record them in their repositories together with the chain's legacy ERC20 forwarder.

- [ ] Install the chain's kind tree root on the proxy. Generate the table after the forwarders are recorded, so that it carries the members under the label of the legacy ERC20 forwarder. The proxy can take the root while paused, so install it before the stop of the legacy protocol adapter. The migration and completion runs do not change it. Simulate, run, and read it back:

  ```sh
  just contracts-simulate-staging-kind-table-update <PROXY> <KIND_TREE_ROOT> <CHAIN>
  just contracts-execute-staging-kind-table-update deployer <PROXY> <KIND_TREE_ROOT> <CHAIN>
  cast call <PROXY> "getKindTreeRoot()(bytes32)" --rpc-url <CHAIN>
  ```

- [ ] Deprecate the logic reference that the chain's legacy ERC20 forwarder accepts: add it to the proxy's denylist for created resources. The members under the label of the legacy ERC20 forwarder make its resources fungible with those of the environment's ERC20 forwarder, in both directions, so without this entry a transaction could convert resources of the environment's ERC20 forwarder into new legacy-forwarder resources. Legacy-forwarder resources stay consumable, so they still convert and unwrap. Take the logic reference from the legacy ERC20 forwarder that the forwarder repository records, and check that the environment's ERC20 forwarder on the chain accepts a different one: no wrap can create resources of a deprecated logic reference. The proxy can take the entry while paused, so add it before the stop of the legacy protocol adapter. Simulate, run, and read it back:

  ```sh
  just contracts-simulate-staging-logic-ref-denial <PROXY> "[(<LEGACY_ERC20_FORWARDER_LOGIC_REF>,false)]" <CHAIN>
  just contracts-execute-staging-logic-ref-denial deployer <PROXY> "[(<LEGACY_ERC20_FORWARDER_LOGIC_REF>,false)]" <CHAIN>
  cast call <PROXY> "isLogicRefDenied(bytes32,bool)(bool)" <LEGACY_ERC20_FORWARDER_LOGIC_REF> false --rpc-url <CHAIN>
  ```

## Per chain

The stop in step 2 leaves users unable to transact, and the unpause in step 7 lets them transact again. Prepare every transaction before the stop.

1. [ ] Read and record the `latestCommitmentTreeRoot`, `commitmentCount` and `nullifierCount` of the legacy protocol adapter. They are what the runs are checked against.

2. [ ] Transfer the legacy protocol adapter to the deployment wallet, then stop it from there. The production Safe, `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`, owns every legacy protocol adapter, so the transfer is a Safe transaction. Besides the `Ownable` functions, its owner can only call `emergencyStop`, so the transfer gives the deployment wallet nothing but the stop. Simulate the proposal, then propose it:

   ```sh
   just contracts-simulate-legacy-protocol-adapter-ownership-transfer-proposal <LEGACY_PROTOCOL_ADAPTER> <PROPOSER> <CHAIN>
   just contracts-propose-legacy-protocol-adapter-ownership-transfer deployer <LEGACY_PROTOCOL_ADAPTER> <PROPOSER> <CHAIN>
   ```

   Ask the Safe signers to confirm and execute it in the [Safe app](https://app.safe.global). The legacy protocol adapter keeps running, so do this before the planned stop and read the new owner back:

   ```sh
   cast call <LEGACY_PROTOCOL_ADAPTER> "owner()(address)" --rpc-url <CHAIN>
   ```

   Then stop the legacy protocol adapter from the deployment wallet. The stop and the migration run of step 4 follow each other without a Safe transaction between them, which is why the ownership moves at all:

   ```sh
   cast send <LEGACY_PROTOCOL_ADAPTER> "emergencyStop()" --rpc-url <CHAIN> --account deployer
   ```

   The stop cannot be undone: the legacy protocol adapter has no function that lifts it. Nothing transfers it back, and no step needs to.

3. [ ] Simulate the migration run. `IS_PRODUCTION` selects the environment whose recorded proxy the runs and the check act on:

   ```sh
   export IS_PRODUCTION=<true|false>
   just contracts-simulate-migration <CHAIN>
   ```

4. [ ] Run it:

   ```sh
   just contracts-execute-migration deployer <CHAIN>
   ```

   The run copies the commitment tree and the nullifier set, and the proxy stays paused. The recipe waits for each transaction before it sends the next (`--slow`). Without that, a transaction that reverts on chain would not stop the ones behind it. The run sends one transaction for the tree and one per `NULLIFIERS_PER_BATCH` nullifiers. A chain holding 8000 nullifiers sends 41 transactions.

   If the run stops early, fix the cause and repeat it. Every step skips once it is done, so the repeat continues where the run stopped. The copy-in stays open until the upgrade lands, and a proxy left half-migrated cannot be replaced at its address.

5. [ ] Move the balances of the legacy ERC20 forwarder to the environment's ERC20 forwarder, as `anomapay-erc20-forwarder` sets out. The proxy must stay paused until they moved: the members under the label of the legacy ERC20 forwarder let legacy-forwarder resources unwrap from the environment's ERC20 forwarder.

6. [ ] Simulate the completion run. `IS_PRODUCTION` decides whether the run ends with the transfer to the production proxy owner:

   ```sh
   export IS_PRODUCTION=<true|false>
   just contracts-simulate-migration-completion <CHAIN>
   ```

7. [ ] Run it:

   ```sh
   just contracts-execute-migration-completion deployer <CHAIN>
   ```

   The run unpauses, upgrades to the plain implementation and, in production, transfers the proxy to the production proxy owner, the Safe `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`. The transfer takes effect at once, and only a Safe transaction can move ownership back. The run sends one transaction each for the unpause, the upgrade and, in production, the transfer.

   A proxy that is short of a batch refuses to unpause, so a half-migrated chain stays paused instead of running on part of the state. If the run stops early, fix the cause and repeat it.

8. [ ] Check the result, reading from the chain:

   ```sh
   just contracts-check-migration <CHAIN>
   ```

   It compares the copied state with the legacy protocol adapter, and it checks that the proxy runs the plain implementation and is unpaused. In production, it also checks that the production proxy owner owns the proxy.
