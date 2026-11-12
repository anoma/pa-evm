# AGENTS.md

The protocol adapter (PA): an upgradeable Solidity contract that settles Anoma Resource Machine
transactions on EVM chains, plus the Rust bindings and integration tests around it. This file
covers what an agent needs that the human docs do not make obvious. Read these instead of
guessing:

- [CONTEXT.md](CONTEXT.md): the glossary. Use its terms and none of the ones it rules out
  (kind table, v1/v2, "migration" on its own, "blocked" for a denied logic ref).
- [docs/](docs/README.md): the specification. [docs/adr/](docs/adr/) records the decisions.
- [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) and [MIGRATION_CHECKLIST.md](MIGRATION_CHECKLIST.md):
  every deploy, upgrade, pause, denial and kind tree root update.

## Layout

| Path                            | Holds                                                                  |
| ------------------------------- | ---------------------------------------------------------------------- |
| `contracts/src/`                | `ProtocolAdapter`, `MigrationalProtocolAdapter`; `state/` (one ERC-7201 module per state), `libs/`, `interfaces/`, `examples/` |
| `contracts/script/`             | forge scripts; `staging/` executes as the owner, `production/` proposes to the Safe, `migration/` runs the state migration |
| `contracts/test/`               | forge tests; `state/` per state module, `scripts/` mirrors `script/`, `libs/` (`TxGen`, `DeltaGen`), `mocks/`, `fixtures/` |
| `contracts/generated/`          | `RecordedDeployments.sol`, generated from the deployment record        |
| `crates/bindings/`              | the `anoma-pa-evm-bindings` crate; `deployments.json` is the deployment record; its tests are the promotion gate |
| `crates/integration-test/`      | local and e2e tests that deploy the PA and run risc0-proven transactions |
| `scripts/`                      | `generate-recorded-deployments.sh`                                     |

Everything runs through the root `justfile` (`just --list`). Recipes are prefixed `contracts-`,
`bindings-`, `crates-` or `all-`.

## Setup and verification

```bash
just contracts-deps            # soldeer installs into contracts/dependencies/
(cd contracts && bun install)  # solhint, and upgrades-core for the upgrade validation in tests
just all-test                  # forge tests, then cargo tests
just all-check                 # CI's static checks: storage, slither, fmt, lint, the three drift checks
```

`all-check` needs `slither` installed and regenerates files before diffing them, so run it on a
clean tree. CI runs two more suites only on PRs into `staging` and `main`: the promotion gate
(`VERIFY_STAGING_DEPLOYMENTS` / `VERIFY_PRODUCTION_DEPLOYMENTS`, forks every recorded chain) and
`just crates-test-e2e` (needs `QUEUE_BASE_URL`). Without the flags the gate tests skip and
`cargo test` still exits 0, so say which suites ran rather than reporting the exit code.

## When you change X, also do Y

| Change                                                    | Then                                                            |
| --------------------------------------------------------- | --------------------------------------------------------------- |
| ABI of a contract in the `--select` list of `contracts-gen-bindings` | `just contracts-gen-bindings`, commit `crates/bindings/src/generated/` |
| `crates/bindings/deployments.json`                        | `just contracts-gen-deployments`, commit `RecordedDeployments.sol` |
| gas of `ProtocolAdapter` or a contract in `gas_reports`   | `just contracts-gen-gas-report`, commit `docs/gas-report.md`    |
| state in `src/`                                           | ERC-7201 namespaced storage only; read [upgrade-safety](.agents/skills/upgrade-safety/SKILL.md) |
| `VERSION` in `ProtocolAdapter.sol`                        | it is bytecode: a bump is a redeploy; follow RELEASE_CHECKLIST step 1 |
| a soldeer dependency                                      | `[dependencies]`, the versioned `remappings` in `foundry.toml`, every import of the old path, `soldeer.lock` |
| behavior or a term                                        | the matching `docs/NN-*.md` and NatSpec; a new term goes in `CONTEXT.md` |

CI fails on any drift in these generated outputs; never edit them by hand. Never edit
`contracts/dependencies/` either.

## Never without asking

The PA and its forwarders run on mainnets. These recipes broadcast transactions or publish packages:
`contracts-deploy-impl`, `contracts-deploy-proxy`, `contracts-execute-*`, `contracts-propose-*`,
`contracts-publish`, `bindings-publish`, and any `forge script` or `cast send` with `--broadcast` or a private key.
Run only the matching `contracts-simulate-*` dry-run on your own, and see
[environment-ops](.agents/skills/environment-ops/SKILL.md).

Do not edit an existing entry of `deployments.json`: it is written once per chain at the genesis
deploy (only a redeployed release candidate replaces its own). Never commit `contracts/.env` or an RPC URL with a key in it.

## Branches and pull requests

Feature branches start from `next` and their PRs target `next`. Changes flow `next` → `staging`
→ `main` only by promotion PRs ([ADR-0003](docs/adr/0003-deployments-are-promoted-from-next-through-staging-to-main.md));
never open a feature PR into `staging` or `main`. PRs merge with a merge commit, so every
commit lands in history and should build on its own.

## Naming conventions

- Branches: usually `<github-user>/<imperative-kebab-summary>`, e.g. `heueristik/keep-the-gates-during-execute`.
- Commits: Conventional Commits. Types `feat`, `fix`, `refactor`, `test`, `docs`, `build`, `ci`,
  `chore`, `perf`, `revert`. Scopes `contracts`, `crates`, `bindings`, comma-joined when a commit
  spans several: `refactor(contracts,crates)!:`. `!` when the ABI or the bindings API breaks.
  Subjects are lowercase, imperative, plain English with articles:
  `fix(contracts,crates): revert the owner functions while execute runs`. One topic per commit.
- Tags: `contracts/vX.Y.Z[-rc.N]` (must equal `VERSION`) and `bindings/vA.B.C[-rc.N]`, versioned
  independently.
- Solidity files: one contract per file, named after it. Suffixes: `.t.sol` tests, `.s.sol`
  scripts, `.m.sol` mocks (contract `<Name>Mock`), `.e.sol` test examples. Tests of one aspect go in
  `<Contract>.<aspect>.t.sol` (`ProtocolAdapter.reentrancy.t.sol`, `NullifierSet.storage.t.sol`).
- Test functions: `test_<function>_<behavior_in_snake_case>`, `testFuzz_` for fuzz tests, keeping
  identifiers as spelled: `test_execute_reverts_on_the_empty_transaction`, `test_pause_emits_the_Paused_event`.
- Scripts: verb then object. `Execute*` for staging (owner acts directly), `Propose*` for
  production (Safe proposal), `Deploy*` for deployments.
- Solidity code: interfaces `I*`; private and internal members `_camelCase`; constants
  `_SCREAMING_SNAKE`, public ones without the underscore; custom errors, no revert strings;
  storage namespaces `erc7201:anoma.storage.<Module>`; initializers `__<Module>_init`; named
  imports from versioned remappings (`@openzeppelin-contracts-5.7.0/...`). Sources declare
  `pragma solidity ^0.8.30`; `foundry.toml` pins the compiler.
- Docs: numbered `docs/NN-<topic>.md`; ADRs `docs/adr/NNNN-<the-decision-as-a-sentence>.md`.

`forge fmt`, `cargo fmt`, solhint (`contracts/.solhint.json` for `src`, `.solhint.other.json` elsewhere),
`forge lint` and clippy are authoritative for everything else.
