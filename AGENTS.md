# AGENTS.md

The protocol adapter (PA): an upgradeable Solidity contract that settles Anoma Resource Machine
transactions on EVM chains, plus its Rust bindings and integration tests. This file holds only
what the docs do not make obvious. Read these rather than guess:

- [CONTEXT.md](CONTEXT.md), the glossary. Use its terms, never the ones it rules out (kind table,
  v1/v2, "migration" on its own).
- [docs/](docs/README.md), the specification, and [docs/adr/](docs/adr/), the decisions.
- [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) and [MIGRATION_CHECKLIST.md](MIGRATION_CHECKLIST.md)
  for every deploy, upgrade, pause, denial and kind tree root update.

## Layout

- `contracts/src/`: `ProtocolAdapter`, `MigrationalProtocolAdapter`, and `state/`, one ERC-7201
  module per piece of state.
- `contracts/script/`: `staging/` executes as the owner, `production/` proposes to the Safe,
  `migration/` runs the state migration.
- `contracts/test/` mirrors `src/` and `script/`; `libs/TxGen.sol` and `libs/DeltaGen.sol` build
  transactions.
- `crates/bindings/`: the published crate. `deployments.json` is the deployment record, and the
  crate's tests are the promotion gate.
- `crates/integration-test/`: local and e2e tests with risc0 proofs.
- Generated, never edited by hand: `crates/bindings/src/generated/`,
  `contracts/generated/RecordedDeployments.sol`, `docs/gas-report.md`. Never edit
  `contracts/dependencies/` either.

## Checks

```bash
just contracts-deps && (cd contracts && bun install)   # soldeer deps, solhint, upgrades-core
just all-test    # forge, then cargo
just all-check   # CI's static checks; needs slither; regenerates then diffs, so use a clean tree
```

PRs into `staging` and `main` also run the promotion gate (`VERIFY_*_DEPLOYMENTS`, forks every
recorded chain) and `just crates-test-e2e`. Without the flags the gate tests skip and
`cargo test` still exits 0, so say which suites ran.

## Finishing a change

CI fails on stale generated files. It does not notice stale prose or a missing test, and those
are what follow-up PRs keep fixing (#772, #773, #777, #778). Before opening a PR:

1. Regenerate what CI diffs: `just contracts-gen` (bindings and `RecordedDeployments.sol`) after
   an ABI or `deployments.json` change, `just contracts-gen-gas-report` after a gas change.
2. Sweep every renamed identifier or term: `git grep -in` the old name in each spelling
   (`kindTable`, `kind_table`, `kind table`) and fix every hit, including NatSpec, comments, test
   names, assertion messages, justfile recipes and comments, `docs/`, ADRs, `CONTEXT.md`, the
   checklists and the READMEs.
3. A behavior change brings a test that fails on `next` and passes with the change, and updates
   the NatSpec and the `docs/NN-*.md` section that states the behavior. A new term goes in
   `CONTEXT.md`.
4. A changed recipe or script: update each checklist step that runs it.
5. New or changed state, initializers, constructors or owner functions: read
   [upgrade-safety](.agents/skills/upgrade-safety/SKILL.md). A `VERSION` bump: RELEASE_CHECKLIST
   step 1. A soldeer dependency: `[dependencies]`, the versioned `remappings`, every import of the
   old path, and `soldeer.lock`.

In the PR description, say which of these applied and what you swept for.

## Never without asking

These broadcast transactions or publish packages: `contracts-deploy-impl`,
`contracts-deploy-proxy`, `contracts-execute-*`, `contracts-propose-*`, `contracts-publish`,
`bindings-publish`, and any `forge script --broadcast` or `cast send`. Dry-runs are fine; see
[environment-ops](.agents/skills/environment-ops/SKILL.md). Never edit an existing entry of
`deployments.json`, and never commit `contracts/.env` or an RPC URL with a key in it.

## Branches and PRs

Branch from `next` and target `next`. Changes reach `staging` and `main` only through promotion
PRs ([ADR-0003](docs/adr/0003-deployments-are-promoted-from-next-through-staging-to-main.md)).
PRs merge with a merge commit, so each commit should build on its own.

## Naming

- Branches: `<github-user>/<imperative-kebab-summary>`.
- Commits: Conventional Commits with scopes `contracts`, `crates`, `bindings`, comma-joined
  (`refactor(contracts,crates)!:`), and `!` when the ABI or the bindings API breaks. Lowercase,
  imperative, plain English with articles: `fix(contracts,crates): revert the owner functions
  while execute runs`. One topic per commit.
- Tags: `contracts/vX.Y.Z[-rc.N]`, equal to `VERSION`, and `bindings/vA.B.C[-rc.N]`.
- Solidity files: one contract per file, named after it. `.t.sol` tests, `.s.sol` scripts,
  `.m.sol` mocks (`<Name>Mock`), `.e.sol` test examples. One aspect per test file:
  `ProtocolAdapter.reentrancy.t.sol`.
- Tests: `test_<function>_<behavior_in_snake_case>` and `testFuzz_`, keeping identifiers as
  spelled: `test_pause_emits_the_Paused_event`.
- Scripts: `Execute*` (staging), `Propose*` (production), `Deploy*`.
- Code: interfaces `I*`; non-public members `_camelCase`; constants `_SCREAMING_SNAKE`, or without
  the underscore when public; custom errors only; namespaces `erc7201:anoma.storage.<Module>`;
  initializers `__<Module>_init`; named imports from versioned remappings.
- Docs: `docs/NN-<topic>.md`; ADRs `docs/adr/NNNN-<the-decision-as-a-sentence>.md`.

`forge fmt`, `cargo fmt`, solhint, `forge lint` and clippy settle everything else.
