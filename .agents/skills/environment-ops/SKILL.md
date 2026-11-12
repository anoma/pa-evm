---
name: environment-ops
description: >-
  How to drive the deploy, upgrade, pause, logic ref denial, kind tree root and migration recipes
  of the pa-evm justfile against the staging and production environments. Use when asked to
  deploy, upgrade, pause, deny a logic ref, set a kind tree root, run the state migration of a chain, verify a
  contract, or check what an environment runs.
---

# Environment operations

Each operation is a step of [RELEASE_CHECKLIST.md](../../../RELEASE_CHECKLIST.md) or
[MIGRATION_CHECKLIST.md](../../../MIGRATION_CHECKLIST.md). Follow the checklist; this file adds
only the rules an agent needs around it.

## You run dry-runs, people broadcast

Recipes come in pairs. The `contracts-simulate-*` one is a dry-run and safe to run. Its partner
broadcasts with `--account <deployer>`:

| Environment | Dry-run                                     | Broadcast (ask first)                        |
| ----------- | ------------------------------------------- | -------------------------------------------- |
| any         | `contracts-simulate-impl`, `-proxy`         | `contracts-deploy-impl`, `-proxy`            |
| staging     | `contracts-simulate-staging-<op>`           | `contracts-execute-staging-<op>`             |
| production  | `contracts-simulate-production-<op>-proposal` | `contracts-propose-production-<op>`        |
| migration   | `contracts-simulate-migration[-completion]` | `contracts-execute-migration[-completion]`   |

`<op>` is `upgrade`, `pause`, `logic-ref-denial` or `kind-tree-root-update`. Run a broadcast only
when the user asks for that exact command, chain and arguments, after its dry-run passed. A
production proposal only queues a Safe transaction; the Safe signers execute it, and nothing
here can.

`contracts-verify-*` submits sources to Sourcify and Etherscan; harmless, but ask too.

Read-only: `contracts-simulate-*`, `contracts-check-migration` and `cast call`. `simulate-impl`
and `simulate-proxy` wipe `out/` first. `VERIFY_STAGING_DEPLOYMENTS=true just bindings-test` (or
the production flag) checks that every chain of an environment runs the source; it forks every
recorded chain, so it is slow.

## Inputs

- Chains are the names in `[rpc_endpoints]` of `contracts/foundry.toml` (`sepolia`,
  `base-sepolia`, ...). They need `ALCHEMY_API_KEY`, which `just` loads from `contracts/.env`.
- Proxy addresses come from `crates/bindings/deployments.json`, never from memory.
- `IS_PRODUCTION` (the `-proxy` and migration recipes) and `IS_MIGRATIONAL` (the `-proxy` recipes and
  `contracts-verify-deployment`) are exported per session. Never put them in `contracts/.env`; ask which values the user means.
- What a proxy runs is read from the chain:
  `cast call <PROXY> "getImplementation()(address)" --rpc-url <CHAIN>`.

## Afterwards

- `contracts-deploy-impl` reverts with `ImplementationAlreadyDeployed` when the chain already
  has that implementation, for example when staging and production share a chain.
- An upgrade writes nothing to git. A genesis deploy adds an entry to `deployments.json`; see the
  checklist's "Record the Genesis Entry".
