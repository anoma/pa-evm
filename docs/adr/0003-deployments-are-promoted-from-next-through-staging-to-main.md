# Deployments are promoted from next through staging to main

## Context

The protocol adapter runs in two environments, recorded per chain in
`crates/bindings/deployments.json`: **staging**, owned by the deployment wallet
and upgraded instantly, and **production**, owned by a Safe multisig whose
owners queue and execute upgrades.

The deployment tests forked every recorded chain and asserted that the staging
proxy runs the source version exactly. That gate demanded deploy-before-merge: a
pull request bumping `VERSION` was red from its first commit until staging was
upgraded, which can only happen once the change is reviewed and merged. It was
also too weak in the other direction — it compared the version *string*, so a
logic change without a version bump passed while staging ran different bytecode.

Three facts constrain any fix:

- **`VERSION` is bytecode.** It is a `string public constant`, inlined into the
  creation code, so `2.1.0-rc.3` and `2.1.0` are different implementations at
  different CREATE2 addresses. A version bump is a redeploy, not a relabel.
- **An implementation address is per chain, not per environment.** The RISC Zero
  verifier router is a constructor argument and differs per chain. On one chain
  both environments share an implementation until production trails.
- **Version lines are ordered by construction**, so every cross-line comparison
  has a window in which it is false: staging leads main's source between
  releases, and production leads next's source right after one, since
  `2.1.0 > 2.1.0-rc.3`.

## Decision

Environments are bound to branches, and changes flow one way:
`next` → `staging` → `main`.

- **`next`** integrates feature branches. Nothing is asserted about deployments;
  the source version may differ from what is deployed anywhere.
- **`staging`** receives `next`. A pull request into it requires every entry in
  the staging section to run the source version.
- **`main`** is the production line and receives `staging`. A pull request into
  it requires every entry in the production section to run the source version,
  and that version to carry no prerelease suffix.

A deploy happens after the change is reviewed and merged into `next`, and the
promotion pull request is the gate that proves the deployment matches the source.

"Runs the source version" is asserted as an implementation-address comparison
rather than a version-string comparison, which makes it bytecode-sensitive:
`proxy.implementation()` equals the address the source predicts for that chain.
The record is not a term in it. Storing the live implementation alongside the
genesis fields would only interpose a middle term between two ends that already
meet, and it would have to be written at a moment — a proxy owned by a Safe is
upgraded when its signers execute, not when the upgrade is proposed — at which
the record would state either an intention or a staleness rather than a fact.

An entry therefore holds only what the chain cannot answer once the proxy is
upgraded: the genesis fields pinning its address. It is written once, at the
genesis deploy, and never edited.

The comparison is the promotion gate, and runs only on a pull request into the
environment it describes, selected by `github.base_ref` and gated in the tests
with `vm.skip`. What survives on every pull request is what holds independently
of any deployment: genesis CREATE2 reproducibility, which reads only the record,
and the deploy scripts themselves against a fresh chain.

The e2e integration tests drive a deployed proxy with transactions built from
the source, so they are only meaningful where the two agree. They run on
promotion pull requests, against the environment being promoted into.

The release bump happens on `next`, and the identical commit is promoted onward:
`next` bumps `2.1.0-rc.3` to `2.1.0`, that commit is promoted to `staging` and
deployed, then promoted unchanged to `main`. Production therefore runs the exact
implementation staging validated.

## Consequences

- A version-bump pull request is green, and the deploy is a separate mechanical
  step afterwards. The deadlock that motivated this is gone.
- Only `main` constrains the version shape. Staging runs a prerelease most of the
  time but carries the release version during the release window, so "staging
  runs a release candidate" cannot be asserted.
- Rollout is all-or-nothing: a promotion stays red until every chain in the
  section runs the source version. A chain that cannot be upgraded blocks the
  promotion, and the escape is to remove its entry. Section membership is
  therefore a rollout commitment, not documentation.
- Each release costs one extra staging deploy round, since the release-versioned
  implementation is deployed to staging before promotion to `main`.
- Pull requests into `next` no longer fork the recorded chains, so feature CI is
  faster and needs no `ALCHEMY_API_KEY`.
- A proxy upgraded out of band surfaces at the promotion gate rather than when
  it happens, because the gate reads the chain.
- The record is the single source of truth for where each environment is
  deployed, so a genesis deploy owes a pull request into `next`. An upgrade owes
  none, and leaves no artifact in git: the promotion turning green is the
  statement that the environment runs this source, on every chain in its section.
  The per-chain upgrade history stays recoverable from the chain itself, which
  emits `Upgraded(address indexed implementation)` on every upgrade.
- `envs::e2e` resolves the proxy for `Environment::Staging`, which has to become
  configurable so a promotion into `main` exercises production instead. The
  chain it defaults to must be recorded in the section it targets.
- The bindings crate deserializes only an entry's chain ID and proxy address,
  both written at the genesis deploy. An upgrade changes nothing a consumer of
  the published bindings sees, in either environment; a deploy to a new chain
  adds an address under the environment it belongs to, and `Environment::Staging`
  is public API, so that too holds for both.
- Branch names stay `next`, `staging` and `main`. Renaming `main` to
  `production` would match the record's section names but costs a default-branch
  change, branch-protection reconfiguration and four raw-content badge URLs that
  GitHub does not redirect.
