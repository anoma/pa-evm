# Migration Checklist

How to move a chain that ran v1 onto v2 and keep its commitment tree and nullifier set. A chain deployed fresh skips all of this and starts on `ProtocolAdapter`. The plan in `anoma-knowledge-base/galileo-v2-transition-and-versioning.md` sets out the whole move; this file covers the protocol adapter's part.

## How it works

A chain that ran v1 cannot start v2 empty. Its proxy starts on [`TransitionalProtocolAdapter`](./contracts/src/TransitionalProtocolAdapter.sol), a protocol adapter that begins paused and lets its owner copy the v1 state in. [`MigrateProtocolAdapterState`](./contracts/script/migration/MigrateProtocolAdapterState.s.sol) copies the state, unpauses, and upgrades the proxy to `ProtocolAdapter`. After that upgrade the chain runs the same code as every other chain, and the copy-in functions are gone.

The transitional implementation adds two functions. Both are owner-only, and both are allowed only while the adapter is paused and the v1 protocol adapter is stopped.

- `seedCommitmentTree` writes the tree in one call. The caller supplies the stored sides, the one part of the tree v1 exposes through no getter. The leaf count and the root come from v1, and the empty-subtree roots are recomputed from the depth. The call reverts unless the result reproduces v1's root, so a wrong set of sides cannot reach storage.
- `seedNullifierSet` copies the next batch of nullifiers. It reads each one from v1 at the index it takes here and reverts unless it lands at that index. A batch starts where the last one stopped, so no batch can be skipped or repeated.

`unpause` proceeds only once the adapter holds what the stopped v1 adapter holds: the same commitment count, the same latest root, and the same nullifier count. The historical roots are the one difference that stays. v1 keeps every root it had. This adapter keeps two: the empty-tree root, which a resource created and consumed in one transaction proves membership against, and v1's latest root.

Neither function is closed by the contract. The upgrade to the plain implementation removes them, so a migration run must always reach the upgrade.

## Before any chain

- [ ] Deploy the plain implementation on the chain. The migration run refuses to start without it.

  ```sh
  just contracts-deploy-impl deployer <CHAIN>
  ```

- [ ] Deploy the proxy on the transitional implementation. It starts paused, and it reads the chain's v1 protocol adapter from the table in [`DeployTransitionalProtocolAdapterImplementation`](./contracts/script/migration/DeployTransitionalProtocolAdapterImplementation.s.sol).

  ```sh
  export IS_PRODUCTION=false
  export IS_TRANSITIONAL=true
  just contracts-simulate-proxy <CHAIN>
  just contracts-deploy-proxy deployer <CHAIN>
  ```

  The proxy gets the environment's owner. The migration run sends its transactions from that owner, so it works as written for staging, whose owner is the deployment wallet. The production owner is a Safe, and the run's calls would have to be proposed there instead.

## Per chain

Steps 2 to 7 leave users unable to transact, so prepare every transaction before step 2.

1. [ ] Read and record v1's `latestCommitmentTreeRoot`, `commitmentCount` and `nullifierCount`. They are what the run is checked against.

2. [ ] Stop the v1 protocol adapter. The production Safe, `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`, owns v1 on every chain, so the stop is a Safe transaction. Simulate the proposal, then propose it:

   ```sh
   just contracts-simulate-v1-stop-proposal <PROTOCOL_ADAPTER_V1> <PROPOSER> <CHAIN>
   just contracts-propose-v1-stop deployer <PROTOCOL_ADAPTER_V1> <PROPOSER> <CHAIN>
   ```

   Ask the Safe signers to confirm and execute it in the [Safe app](https://app.safe.global). The stop cannot be undone: v1 has no function that lifts it.

3. [ ] Simulate the run, with the proxy owner as the sender:

   ```sh
   just contracts-simulate-migration <OWNER> <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

4. [ ] Run it:

   ```sh
   just contracts-execute-migration deployer <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

   The recipe waits for each transaction before it sends the next (`--slow`). Without that, a batch that reverts on chain would not stop the unpause and the upgrade behind it. The run sends one transaction for the tree, one per `NULLIFIERS_PER_BATCH` nullifiers, one to unpause and one to upgrade, so a chain holding 8000 nullifiers sends 43.

5. [ ] If the run stops early, fix the cause and repeat it. A proxy that is short of a batch refuses to unpause, so a half-migrated chain stays paused instead of running on part of the state. The copy-in stays open until the upgrade lands, and a proxy left half-migrated cannot be replaced at its address.

6. [ ] Check the result against v1, reading both from the chain:

   ```sh
   just contracts-check-migration <PROTOCOL_ADAPTER_V1> <PROXY> <CHAIN>
   ```

7. [ ] Confirm through `getImplementation` that the proxy runs the plain implementation.

8. [ ] Move the ERC20 forwarder balances, install the chain's kind table commitment, and transfer ownership, as the plan sets out.
