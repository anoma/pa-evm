# Migration Checklist

How to move a chain that ran v1 onto v2 and keep its commitment tree and nullifier set. A chain deployed fresh skips all of this and starts on `ProtocolAdapter`. The plan in `anoma-knowledge-base/galileo-v2-transition-and-versioning.md` sets out the whole move; this file covers the protocol adapter's part.

## How it works

A chain that ran v1 cannot start v2 empty. Its proxy starts on [`TransitionalProtocolAdapter`](./contracts/src/TransitionalProtocolAdapter.sol), a protocol adapter that begins paused and lets its owner copy the v1 state in. The state moves in two runs. In the migration run, [`MigrateProtocolAdapterState`](./contracts/script/migration/MigrateProtocolAdapterState.s.sol) copies the state, and the proxy stays paused. In the completion run, [`FinalizeProtocolAdapterStateMigration`](./contracts/script/migration/FinalizeProtocolAdapterStateMigration.s.sol) unpauses, upgrades the proxy to `ProtocolAdapter` and, in production, transfers the proxy to the production proxy owner. After that upgrade the chain runs the same code as every other chain, and the copy-in functions are gone.

The ERC20 forwarder balances move between the two runs. The kind table on the proxy carries V1 members, which let a V1 resource unwrap from the V2 ERC20 forwarder. A paused proxy executes nothing, so no V1 resource can unwrap before the V2 forwarder holds the V1 tokens.

The transitional implementation adds two functions. Both are owner-only, and both are allowed only while the adapter is paused and the v1 protocol adapter is stopped.

- `seedCommitmentTree` writes the tree in one call. The caller supplies the stored sides, the one part of the tree v1 exposes through no getter. The leaf count and the root come from v1, and the empty-subtree roots are recomputed from the depth. The call reverts unless the result reproduces v1's root, so a wrong set of sides cannot reach storage.
- `seedNullifierSet` copies the next batch of nullifiers. It reads each one from v1 at the index it takes here and reverts unless it lands at that index. A batch starts where the last one stopped, so no batch can be skipped or repeated.

`unpause` proceeds only once the adapter holds what the stopped v1 adapter holds: the same commitment count, the same latest root, and the same nullifier count. The historical roots are the one difference that stays. v1 keeps every root it had. This adapter keeps two: the empty-tree root, which a resource created and consumed in one transaction proves membership against, and v1's latest root.

Neither function is closed by the contract. The upgrade to the plain implementation removes them, so a migration must always reach the end of the completion run.

## Before any chain

- [ ] Deploy the plain implementation on the chain. The migration and completion runs refuse to start without it.

  ```sh
  just contracts-deploy-impl deployer <CHAIN>
  ```

- [ ] Deploy the proxy on the transitional implementation. It starts paused. It reads the chain's v1 protocol adapter from [`RecordedDeployments`](./contracts/generated/RecordedDeployments.sol), which is generated from the `v1` entries in [`deployments.json`](./crates/bindings/deployments.json).

  ```sh
  export IS_PRODUCTION=<true|false>
  export IS_TRANSITIONAL=true
  just contracts-simulate-proxy <CHAIN>
  just contracts-deploy-proxy deployer <CHAIN>
  ```

  The proxy gets the staging proxy owner, the deployment wallet, in both environments, because the migration and completion runs send their calls from the owner. A production proxy moves to the production proxy owner at the end of the completion run. Until then, the staging recipes act on it too, because they check the owner and not the environment.

- [ ] Record the proxy in `deployments.json` and release the bindings, as for a chain new to an environment in [`RELEASE_CHECKLIST.md`](./RELEASE_CHECKLIST.md). The kind tables take their chains from this record. The bindings tests on the promotion gate require a recorded proxy to run the plain implementation this source predicts and, in production, to be owned by a Safe, so they fail for this chain until the completion run. Do not promote the environment before the check at the end.

- [ ] Deploy the ERC20 forwarder and the generic call forwarder against the proxy, and record them in their repositories together with the chain's V1 ERC20 forwarder.

- [ ] Install the chain's kind table commitment on the proxy. Generate the table after the forwarders are recorded, so that it carries the V1 members. The proxy can take the commitment while paused, so install it before the v1 stop. The migration and completion runs do not change it. Simulate, run, and read it back:

  ```sh
  just contracts-simulate-staging-kind-table-update <DEPLOYMENT_WALLET> <PROXY> <KIND_TABLE_COMMITMENT> <CHAIN>
  just contracts-execute-staging-kind-table-update deployer <PROXY> <KIND_TABLE_COMMITMENT> <CHAIN>
  cast call <PROXY> "getKindTableCommitment()(bytes32)" --rpc-url <CHAIN>
  ```

## Per chain

Steps 2 to 8 leave users unable to transact, so prepare every transaction before step 2.

1. [ ] Read and record v1's `latestCommitmentTreeRoot`, `commitmentCount` and `nullifierCount`. They are what the runs are checked against.

2. [ ] Stop the v1 protocol adapter. The production Safe, `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`, owns v1 on every chain, so the stop is a Safe transaction. Simulate the proposal, then propose it:

   ```sh
   just contracts-simulate-v1-stop-proposal <PROTOCOL_ADAPTER_V1> <PROPOSER> <CHAIN>
   just contracts-propose-v1-stop deployer <PROTOCOL_ADAPTER_V1> <PROPOSER> <CHAIN>
   ```

   Ask the Safe signers to confirm and execute it in the [Safe app](https://app.safe.global). The stop cannot be undone: v1 has no function that lifts it.

3. [ ] Simulate the migration run, with the proxy owner as the sender:

   ```sh
   just contracts-simulate-migration <OWNER> <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

4. [ ] Run it:

   ```sh
   just contracts-execute-migration deployer <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

   The run copies the commitment tree and the nullifier set, and the proxy stays paused. The recipe waits for each transaction before it sends the next (`--slow`). Without that, a transaction that reverts on chain would not stop the ones behind it. The run sends one transaction for the tree and one per `NULLIFIERS_PER_BATCH` nullifiers. A chain holding 8000 nullifiers sends 41 transactions.

   If the run stops early, fix the cause and repeat it. Every step skips once it is done, so the repeat continues where the run stopped. The copy-in stays open until the upgrade lands, and a proxy left half-migrated cannot be replaced at its address.

5. [ ] Move the V1 ERC20 forwarder balances to the V2 ERC20 forwarder, as `anomapay-erc20-forwarder` sets out. The proxy must stay paused until they moved: the kind table's V1 members let V1 resources unwrap from the V2 forwarder.

6. [ ] Simulate the completion run, with the proxy owner as the sender. `IS_PRODUCTION` decides whether the run ends with the transfer to the production proxy owner:

   ```sh
   export IS_PRODUCTION=<true|false>
   just contracts-simulate-migration-completion <OWNER> <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

7. [ ] Run it:

   ```sh
   just contracts-execute-migration-completion deployer <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

   The run unpauses, upgrades to the plain implementation and, in production, transfers the proxy to the production proxy owner, the Safe `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`. The transfer takes effect at once, and only a Safe transaction can move ownership back. The run sends one transaction each for the unpause, the upgrade and, in production, the transfer.

   A proxy that is short of a batch refuses to unpause, so a half-migrated chain stays paused instead of running on part of the state. If the run stops early, fix the cause and repeat it.

8. [ ] Check the result, reading from the chain:

   ```sh
   just contracts-check-migration <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

   It compares the copied state with v1, and it checks that the proxy runs the plain implementation and is unpaused. In production, it also checks that the production proxy owner owns the proxy.
