# Release Checklist

Releases of the packages contained in this monorepo follow the [SemVer convention](https://semver.org/spec/v2.0.0.html).

> [!NOTE]
> The `contracts` and `bindings` are independently versioned with `X.Y.Z` and `A.B.C`, respectively. Both versions can include release candidates (suffixed with `-rc.N`).

We distinguish between three release cases:

- Releasing a **new** protocol adapter version resulting in a new
  - `contracts/vX.Y.Z` version
  - `bindings/vA.0.0` version

- Deploying an **existing** protocol adapter version to a chain new to an environment resulting in a new
  - `bindings/vA.B.0` version

- Maintaining the bindings resulting in a new
  - `bindings/vA.B.C` version

## Branches and Environments

The protocol adapter runs in two environments, recorded per chain in [`./crates/bindings/deployments.json`](./crates/bindings/deployments.json):

| Environment  | Proxy owner                                                        | Upgraded by                     | Branch    |
| ------------ | ------------------------------------------------------------------ | ------------------------------- | --------- |
| `staging`    | the deployment wallet `0x61462bE56782568376f9cB069382EFa72764a407` | the deployment wallet, directly | `staging` |
| `production` | the Safe multisig `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10`     | its signers, from a proposal    | `main`    |

Changes flow one way, `next` → `staging` → `main`, and the promotion pull request is the gate:

- **`next`** integrates feature branches. Nothing is asserted about deployments, so a version bump is green before anything is deployed.
- **`staging`** receives `next`. A pull request into it requires every entry in the staging section to run the source version, checked with `VERIFY_STAGING_DEPLOYMENTS`.
- **`main`** receives `staging`. A pull request into it requires every entry in the production section to run the source version, carry no prerelease suffix, and be owned by a Safe, checked with `VERIFY_PRODUCTION_DEPLOYMENTS`.

Deploy or upgrade **every** chain of an environment before opening its promotion pull request — one chain left behind blocks the promotion for all of them.

`VERSION` is a `string public constant`, so it is part of the creation code and every version is a different implementation at a different address. Bumping it is a redeploy, and stripping an `-rc.N` suffix is a redeploy too, which is why a release costs one extra staging deploy round.

Write the deployment record **once** per chain per environment, at the genesis deploy, and never edit it afterwards. It pins the proxy address and nothing else; what an environment currently runs is read from the chain.

## Prerequisites

These apply to all three cases and are done once per session.

- [ ] Visit https://www.soliditylang.org/ and check that the Solidity compiler version used in [`./contracts/foundry.toml`](./contracts/foundry.toml) has no [known vulnerabilities](https://docs.soliditylang.org/en/latest/bugs.html).

- [ ] Install the dependencies with

  ```sh
  just contracts-deps
  ```

- [ ] Check that the dependencies are up-to-date and have no known vulnerabilities.

- [ ] Check that the bindings are up-to-date with

  ```sh
  just bindings-check
  ```

- [ ] Check out a new git branch branching off from `next`, and check that there are no staged or unstaged changes by running `git status`.

- [ ] Check that the deployer wallet is funded and add it to `cast` with

  ```sh
  cast wallet import deployer --private-key <PRIVATE_KEY>
  ```

  or

  ```sh
  cast wallet import deployer --mnemonic <MNEMONIC>
  ```

- [ ] Set the Alchemy RPC provider by exporting

  ```sh
  export ALCHEMY_API_KEY=<KEY>
  ```

  Deploying to `aurora` needs `NOWNODES_API_KEY` instead. Both can live in `contracts/.env`, which `just` loads.

- [ ] Set the Etherscan key

  ```sh
  export ETHERSCAN_API_KEY=<KEY>
  ```

- [ ] Select the environment. It picks the CREATE2 salt and the proxy owner in [`DeployProtocolAdapterProxy.s.sol`](./contracts/script/DeployProtocolAdapterProxy.s.sol), and is deliberately kept out of `contracts/.env` so that it is a conscious choice per session.

  ```sh
  export IS_PRODUCTION=false
  ```

  Only `just contracts-simulate-proxy` and `just contracts-deploy-proxy` read it; every other recipe takes its addresses as arguments.

## Releasing a new Protocol Adapter Version

A release candidate and a release go through the same cycle. Steps 1 to 5 are repeated for each release candidate; steps 6 to 9 lift the last candidate to a release and carry it to production.

### 1. Bump the Version

- [ ] Bump `VERSION` in [`./contracts/src/ProtocolAdapter.sol`](./contracts/src/ProtocolAdapter.sol) following [SemVer](https://semver.org/spec/v2.0.0.html).

- [ ] Bump the `bindings` package version in [`./crates/bindings/Cargo.toml`](./crates/bindings/Cargo.toml) to `A.0.0-rc.N`, where `A` is the last `MAJOR` version number incremented by 1.

- [ ] Regenerate the bindings with `just contracts-gen-bindings`, then run `just bindings-build` and check that the `Cargo.lock` file reflects the version number change.

- [ ] Open a pull request into `next` and merge it once green. The deploy is a separate mechanical step afterwards.

### 2. Test the Contracts

- [ ] Run the checks CI runs with

  ```sh
  just all-check
  ```

- [ ] Run the test suites with

  ```sh
  just all-test
  ```

### 3. Deploy the Implementation and Upgrade Staging

For each chain in the `staging` section of the record:

- [ ] **Simulate** the implementation deployment by running

  ```sh
  just contracts-simulate-impl <CHAIN>
  ```

- [ ] After successful simulation, **deploy** it by running

  ```sh
  just contracts-deploy-impl deployer <CHAIN>
  ```

- [ ] Export the address of the newly deployed implementation, and the proxy address recorded for this chain, with

  ```sh
  export IMPL_ADDRESS=<ADDRESS>
  export PROXY_ADDRESS=<ADDRESS>
  ```

- [ ] Verify the implementation on sourcify and Etherscan by running

  ```sh
  just contracts-verify-impl $IMPL_ADDRESS <CHAIN>
  ```

  and check that the verification worked (e.g. on https://sourcify.dev/#/lookup). The proxy was verified at its genesis deploy and carries the ERC-1967 bytecode, not the implementation's, so it needs no reverification.

- [ ] **Simulate** the upgrade, with the staging proxy owner as the sender, by running

  ```sh
  just contracts-simulate-staging-upgrade 0x61462bE56782568376f9cB069382EFa72764a407 $PROXY_ADDRESS $IMPL_ADDRESS <CHAIN>
  ```

- [ ] After successful simulation, **execute** it by running

  ```sh
  just contracts-execute-staging-upgrade deployer $PROXY_ADDRESS $IMPL_ADDRESS <CHAIN>
  ```

- [ ] Confirm the proxy now delegates to the new implementation with

  ```sh
  cast call $PROXY_ADDRESS "getImplementation()(address)" --rpc-url <CHAIN>
  ```

> [!NOTE]
> An upgrade adds nothing to `deployments.json` and owes no pull request. Only a genesis deploy writes to the record.

### 4. Promote `next` into `staging`

- [ ] Open a pull request from `next` into `staging`. CI sets `VERIFY_STAGING_DEPLOYMENTS`, so the deployment tests fork every chain in the staging section and check that it runs the implementation this source predicts.

- [ ] Merge it once green.

### 5. Tag and Publish the Release Candidate

- [ ] Create the tags on the promoted commit:
  - [ ] `contracts/vX.Y.Z-rc.N`, where `X.Y.Z-rc.N` must match the protocol adapter `VERSION`, and
  - [ ] `bindings/vA.0.0-rc.N`.

- [ ] Create new [GH releases](https://github.com/anoma/pa-evm/releases) for both packages.

- [ ] Publish the `contracts` package on https://soldeer.xyz/ with

  ```sh
  just contracts-publish --dry-run
  ```

  and check the resulting `contracts.zip` file. If everything is correct, remove the `--dry-run` flag and publish the package.

- [ ] Publish the `anoma-pa-evm-bindings` package on https://crates.io/ with

  ```sh
  just bindings-publish --dry-run
  ```

  and check the result. If everything is correct, remove the `--dry-run` flag and publish the package.

> [!IMPORTANT]
> A prerelease of the bindings describes **staging only**. Production trails on the previous release until the candidate cycle ends, so the generated ABI need not match what production runs.

### 6. Lift the Release Candidate to a Release

- [ ] On a branch off `next`, strip the `-rc.N` suffix from `VERSION` and from the `bindings` package version, and merge it into `next`.

- [ ] Repeat steps 2 to 4. The release version is a different implementation at a different address, so it has to be deployed to staging and promoted like any other candidate. This is the extra staging deploy round a release costs, and it is what lets production run the exact implementation staging validated.

### 7. Upgrade Production

For each chain in the `production` section of the record:

- [ ] **Deploy** the implementation, unless the chain is also a staging chain — step 6 deployed it there already, and `just contracts-deploy-impl` reverts with `ImplementationAlreadyDeployed` if it exists. The implementation is shared by both environments.

  ```sh
  just contracts-deploy-impl deployer <CHAIN>
  ```

- [ ] Export the addresses and verify the implementation as in step 3.

- [ ] **Simulate** the proposal, which simulates the Safe executing the upgrade, by running

  ```sh
  just contracts-simulate-production-upgrade-proposal $PROXY_ADDRESS <PROPOSER> $IMPL_ADDRESS <CHAIN>
  ```

- [ ] After successful simulation, **propose** it to the owning Safe by running

  ```sh
  just contracts-propose-production-upgrade deployer $PROXY_ADDRESS <PROPOSER> $IMPL_ADDRESS <CHAIN>
  ```

  where `<PROPOSER>` is a Safe owner or delegate.

> [!IMPORTANT]
> The procedure hands over to the Safe signers here. Everything below waits on people outside this checklist.

- [ ] Ask the signers of `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10` to confirm and execute the queued transaction in the [Safe app](https://app.safe.global/home?safe=0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10).

- [ ] Once executed, confirm the upgrade with

  ```sh
  cast call $PROXY_ADDRESS "getImplementation()(address)" --rpc-url <CHAIN>
  ```

  Signers execute chain by chain, so a rollout can span days. Nothing is recorded in the meantime, and the promotion below stays red until the last chain is done.

### 8. Promote `staging` into `main`

- [ ] Open a pull request from `staging` into `main`. CI sets `VERIFY_PRODUCTION_DEPLOYMENTS`, so the deployment tests check that every chain in the production section runs this source, carries no prerelease suffix, and is owned by a Safe.

- [ ] Merge it once green.

### 9. Tag and Publish the Release

- [ ] Create the tags on the promoted commit:
  - [ ] `contracts/vX.Y.Z`, where `X.Y.Z` must match the protocol adapter `VERSION`, and
  - [ ] `bindings/vA.0.0`, where `A` is the last `MAJOR` version number incremented by 1.

- [ ] Create new [GH releases](https://github.com/anoma/pa-evm/releases) for both packages, and publish both as in step 5.

## Deploying a Version to a Chain new to an Environment

A chain can be new to one environment and not to the other. The implementation is shared by both, so where the other environment already runs this version only the proxy is deployed.

### 1. Deploy and Verify the Protocol Adapter

For **staging**:

- [ ] Select the environment with

  ```sh
  export IS_PRODUCTION=false
  ```

For **production**:

- [ ] Check that `VERSION` carries no `-rc.N` suffix. Recording a production entry arms the release check on the next promotion into `main`.

- [ ] Select the environment with

  ```sh
  export IS_PRODUCTION=true
  ```

  The proxy is owned by the Safe from its constructor, so there is no ownership transfer. This is the only production step that needs no signer action.

For **both**:

- [ ] Run the test suites as in step 2 of the release cycle.

- [ ] **Simulate** the deployment by running

  ```sh
  just contracts-simulate-proxy <CHAIN>
  ```

- [ ] After successful simulation, **deploy** the contracts by running

  ```sh
  just contracts-deploy-proxy deployer <CHAIN>
  ```

- [ ] Export the addresses of the implementation and proxy with

  ```sh
  export IMPL_ADDRESS=<ADDRESS>
  export PROXY_ADDRESS=<ADDRESS>
  ```

- [ ] Verify the implementation and proxy on sourcify and Etherscan by running

  ```sh
  just contracts-verify-deployment $IMPL_ADDRESS $PROXY_ADDRESS <CHAIN>
  ```

  and check that the verification worked (e.g. on https://sourcify.dev/#/lookup).

### 2. Record the Genesis Entry

- [ ] Add an entry to the environment's section of [`./crates/bindings/deployments.json`](./crates/bindings/deployments.json), built from the four values the deploy run returns:

  ```json
  {
    "chainId": <CHAIN_ID>,
    "proxy": {
      "address": "<PROXY_ADDRESS>",
      "initialImplementation": "<IMPL_ADDRESS>",
      "initializerData": "<INITIALIZER_DATA>",
      "creationCode": "<CREATION_CODE>"
    }
  }
  ```

  The genesis fields pin how the address was derived and cannot be recovered from the chain once the proxy is upgraded. They are written once and never edited.

- [ ] Bump the `bindings` package version in [`./crates/bindings/Cargo.toml`](./crates/bindings/Cargo.toml) to `A.B.0`, where `A` is the last `MAJOR` version and `B` is the last `MINOR` version number incremented by 1.

- [ ] Run `just bindings-build` and check that the `Cargo.lock` file reflects the version number change, then run the tests with `just bindings-test`.

- [ ] Open a pull request into `next` and merge it once green. The CREATE2 derivation of the new entry is checked on every pull request, so a wrong salt or a mistyped genesis field fails here rather than at a promotion.

### 3. Promote and Publish

- [ ] Promote `next` into `staging`, then `staging` into `main`, as in the release cycle. Each gate now includes the new chain, so the section it was added to has become a rollout commitment.

- [ ] Create a `bindings/vA.B.0` tag on the commit promoted to `main`, create a new [GH release](https://github.com/anoma/pa-evm/releases), and publish the package as in step 5 of the release cycle.

## Maintaining the Bindings

For changes that touch only the bindings crate and leave `VERSION` alone.

### 1. Bump the Version

- [ ] Change the `bindings` package version number in [`./crates/bindings/Cargo.toml`](./crates/bindings/Cargo.toml) to `A.B.C`, where `A` and `B` are the last `MAJOR` and `MINOR` version numbers and `C` is the last `PATCH` version number incremented by 1.

- [ ] Run `just bindings-build` and check that the `Cargo.lock` file reflects the version number change.

- [ ] Run the tests with `just bindings-test`.

- [ ] Open a pull request into `next` and merge it once green.

### 2. Promote

- [ ] Promote `next` into `staging`, then `staging` into `main`. Neither promotion needs a deploy round: `VERSION` did not change, so both environments already run the source implementation and both gates are satisfied as they stand.

### 3. Tag and Publish a new `bindings` Package

- [ ] Create a new `bindings/vA.B.C` tag on the commit promoted to `main` and a new [GH release](https://github.com/anoma/pa-evm/releases).

- [ ] Publish the `anoma-pa-evm-bindings` package on https://crates.io/ with

  ```sh
  just bindings-publish --dry-run
  ```

  and check the result. If everything is correct, remove the `--dry-run` flag and publish the package.

## Updating the Kind Table Commitment

Not a release. The kind table commitment is rotated on a live proxy without changing `VERSION`, so no promotion, tag or publish is involved.

For **staging**:

- [ ] **Simulate** the update, with the staging proxy owner as the sender, by running

  ```sh
  just contracts-simulate-staging-kind-table-update 0x61462bE56782568376f9cB069382EFa72764a407 <PROXY> <COMMITMENT> <CHAIN>
  ```

- [ ] After successful simulation, **execute** it by running

  ```sh
  just contracts-execute-staging-kind-table-update deployer <PROXY> <COMMITMENT> <CHAIN>
  ```

For **production**:

- [ ] **Simulate** the proposal, which simulates the Safe executing the update, by running

  ```sh
  just contracts-simulate-production-kind-table-proposal <PROXY> <PROPOSER> <COMMITMENT> <CHAIN>
  ```

- [ ] After successful simulation, **propose** it to the owning Safe by running

  ```sh
  just contracts-propose-production-kind-table-update deployer <PROXY> <PROPOSER> <COMMITMENT> <CHAIN>
  ```

- [ ] Ask the signers of `0xE9082Ac8Aa2Fb27DEfDBAC604921C196b884Da10` to confirm and execute the queued transaction in the [Safe app](https://app.safe.global).
