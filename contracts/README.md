[![Contracts Tests](https://github.com/anoma/pa-evm/actions/workflows/contracts.yml/badge.svg)](https://github.com/anoma/pa-evm/actions/workflows/contracts.yml) [![soldeer.xyz](https://img.shields.io/badge/soldeer.xyz-anoma--pa--evm-blue?logo=ethereum)](https://soldeer.xyz/project/anoma-pa-evm) [![License](https://img.shields.io/badge/license-MIT-blue)](https://raw.githubusercontent.com/anoma/pa-evm/refs/heads/main/contracts/LICENSE)

# Anoma EVM Protocol Adapter Contract

The protocol adapter contract written in Solidity enabling Anoma Resource Machine transaction settlement on EVM-compatible chains.

## Prerequisites

1. Get an up-to-date version of [Foundry](https://github.com/foundry-rs/foundry) with

   ```sh
   curl -L https://foundry.paradigm.xyz | sh
   foundryup
   ```

2. Optionally, to lint the contracts, install [solhint](https://github.com/protofire/solhint) using a JS package manager such as [Bun](https://bun.com/) with

   ```sh
   curl -fsSL https://bun.sh/install | sh
   bun install
   ```

3. Optionally, for static analysis, install [Slither](https://github.com/crytic/slither) with

   ```sh
   python3 -m pip install slither-analyzer
   ```

   or brew

   ```sh
   brew install slither-analyzer
   ```

## Usage

#### Installation

Change the directory to the `contracts` folder with `cd contracts` and run

```sh
forge soldeer install
```

#### Build

To compile the contracts, run

```sh
forge build
```

#### Tests & Coverage

To run the tests, run

```sh
forge test
```

To show the coverage report, run

```sh
forge coverage
```

Append the

- `--no-match-coverage "(script|test|draft)"` to exclude scripts, tests, and drafts,
- `--report lcov` to generate the `lcov.info` file that can be used by code review tooling.

#### Gas Report

[`docs/gas-report.md`](../docs/gas-report.md) lists the gas use of each external function of the contracts that `gas_reports` in `foundry.toml` names. The numbers come from the calls in the unit tests, not from fuzz runs. CI fails if the file is out of date, so the diff of a pull request shows its gas changes. To update the file, run from the repository root

```sh
just contracts-gen-gas-report
```

#### Linting & Static Analysis

As a prerequisite, install the

- `solhint` linter (see https://github.com/protofire/solhint)
- `slither` static analyzer (see https://github.com/crytic/slither)

To run the linters and static analyzer, run

```sh
forge lint --deny notes && \
bunx solhint --config .solhint.json 'src/**/*.sol' && \
bunx solhint --config .solhint.other.json 'script/**/*.sol' 'test/**/*.sol' && \
slither .
```

`forge lint` runs its full rule set on `src` only. solhint checks `src` for the rules that `forge lint` lacks, and checks `script` and `test` with the relaxed `.solhint.other.json`. slither skips `assembly`, `calls-loop` and `locked-ether`, because `forge lint` reports every site that slither reports for them.

#### Rust Bindings

To regenerate the Rust bindings (see the [forge bind](https://getfoundry.sh/forge/reference/bind/) documentation), run

```sh
forge clean && forge build --skip test && forge bind \
  --skip-build \
  --select '^(ProtocolAdapter|IProtocolAdapter|MigrationalProtocolAdapter|IMigrational|ICommitmentTree|INullifierSet|ERC1967Proxy|DeploymentParameters)$' \
  --bindings-path ../crates/bindings/src/generated/ \
  --module \
  --overwrite
```

#### Documentation

Run

```sh
forge doc
```

#### Deployment

To simulate deployment on sepolia, run

```sh
forge script script/DeployProtocolAdapterProxy.s.sol:DeployProtocolAdapterProxy \
  --sig "run(bool,bool)" <IS_PRODUCTION> <IS_MIGRATIONAL> \
  --rpc-url sepolia
```

`<IS_MIGRATIONAL>` is `true` only for a chain that ran v1: its proxy then starts on the migrational implementation, which copies the v1 state in. See [`MIGRATION_CHECKLIST.md`](../MIGRATION_CHECKLIST.md).

Append the

- `--broadcast` flag to deploy on sepolia
- `--verify` flag for subsequent contract verification (Sourcify by default; set `ETHERSCAN_API_KEY` to also verify on Etherscan)
- `--slow` flag to add 15 seconds of waiting time between verification attempts
- `--account <ACCOUNT_NAME>` flag to use a previously imported keystore (see
  `cast wallet --help` for more info)

#### Block Explorer Verification

For post-deployment verification on **Sourcify** run

```sh
forge verify-contract \
   <ADDRESS> \
   src/ProtocolAdapter.sol:ProtocolAdapter \
   --chain sepolia \
   --verifier sourcify
```

For **Etherscan** (requires `ETHERSCAN_API_KEY`) run

```sh
forge verify-contract \
   <ADDRESS> \
   src/ProtocolAdapter.sol:ProtocolAdapter \
   --chain sepolia \
   --verifier etherscan
```
