---
name: environment-ops
description: >-
  Rules for running the pa-evm justfile recipes against the staging and production environments:
  deploy, upgrade, pause, logic ref denial, kind tree root update, state migration, contract
  verification, or checking what an environment runs.
---

# Environment operations

Follow the step in [RELEASE_CHECKLIST.md](../../../RELEASE_CHECKLIST.md) or
[MIGRATION_CHECKLIST.md](../../../MIGRATION_CHECKLIST.md). This adds only the agent rules.

## Dry-runs are yours, broadcasts are not

Each `contracts-simulate-*` dry-run has a broadcasting partner:

| Environment | Dry-run                                       | Broadcast                                  |
| ----------- | --------------------------------------------- | ------------------------------------------ |
| any         | `contracts-simulate-impl`, `-proxy`           | `contracts-deploy-impl`, `-proxy`          |
| staging     | `contracts-simulate-staging-<op>`             | `contracts-execute-staging-<op>`           |
| production  | `contracts-simulate-production-<op>-proposal` | `contracts-propose-production-<op>`        |
| migration   | `contracts-simulate-migration[-completion]`   | `contracts-execute-migration[-completion]` |

`<op>` is `upgrade`, `pause`, `logic-ref-denial` or `kind-tree-root-update`. Run a broadcast only
when the user asks for that exact command, chain and arguments, after its dry-run passed.
`contracts-verify-*` submits sources to Sourcify and Etherscan; ask for that too. A production
proposal only queues a Safe transaction; its signers execute it.

Read-only: `contracts-simulate-*` (`-impl` and `-proxy` wipe `out/` first),
`contracts-check-migration`, `cast call`, and `VERIFY_STAGING_DEPLOYMENTS=true just bindings-test`
(slow: it forks every recorded chain).

## Inputs

- Chains are the `[rpc_endpoints]` names in `contracts/foundry.toml`; they need `ALCHEMY_API_KEY`,
  which `just` loads from `contracts/.env`.
- Proxy addresses come from `crates/bindings/deployments.json`, never from memory. What a proxy
  runs comes from the chain: `cast call <PROXY> "getImplementation()(address)" --rpc-url <CHAIN>`.
- `IS_PRODUCTION` (the `-proxy` and migration recipes) and `IS_MIGRATIONAL` (the `-proxy` recipes
  and `contracts-verify-deployment`) are exported per session, never put in `contracts/.env`. Ask
  which values the user means.
- `contracts-deploy-impl` reverts with `ImplementationAlreadyDeployed` when the chain already has
  that implementation, for example when staging and production share it.
