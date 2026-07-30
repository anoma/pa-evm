# Show commands before running (helps debug failures)
set shell := ["bash", "-euo", "pipefail", "-c"]

# Default recipe
default:
    @just --list

# --- Contracts ---

# Install contract dependencies
contracts-deps:
    cd contracts && forge soldeer install

# Clean contract dependencies
contracts-deps-clean:
    cd contracts && forge soldeer clean

# Clean contracts
contracts-clean:
    cd contracts && forge clean

# Build contracts
contracts-build *args:
    cd contracts && forge build {{ args }}

# Lint contracts (forge lint + solhint)
contracts-lint:
    cd contracts && forge lint --deny warnings
    cd contracts && bunx --bun solhint --config .solhint.json 'src/**/*.sol'
    cd contracts && bunx --bun solhint --config .solhint.other.json 'test/**/*.sol'
    cd contracts && bunx --bun solhint --config .solhint.other.json 'script/**/*.sol'

# Checks that the storage layout of contracts in `src` is empty.
# `skip` is a space-separated list of contract names to ignore (contract-free files).
contracts-storage-check *skip='Types':
    #!/usr/bin/env bash
    set -euo pipefail
    cd contracts
    for sol in $(find src -name '*.sol' | sort); do
        name="$(basename "$sol" .sol)"
        case " {{ skip }} " in *" $name "*) continue ;; esac
        if [ "$(forge inspect "$sol:$name" storageLayout --json | jq '.storage == []')" != true ]; then
            printf '{{RED}}%s has a non-empty storage layout; upgrade-safe contracts must use ERC-7201 namespaced storage.{{NORMAL}}\n' "$sol"
            exit 1
        fi
    done
    printf '{{GREEN}}All contracts in `src` use namespaced storage (empty storage layout).{{NORMAL}}\n'

# Run slither on contracts
contracts-static-analysis:
    cd contracts && slither .
    @echo "Removing slither compilation artifacts..."
    forge clean

# Format contracts
contracts-fmt *args:
    cd contracts && forge fmt {{ args }}

# Check contract formatting
contracts-fmt-check:
    cd contracts && forge fmt --check

# Run contract tests
contracts-test *args:
    cd contracts && forge test --force {{ args }}

# Regenerate Rust bindings from contracts
contracts-gen-bindings:
    # The script directory is built (not skipped) because `ERC1967Proxy` only
    # enters the compilation graph through `DeployProtocolAdapterProxy.s.sol`;
    # `--select` keeps the script contracts themselves out of the bindings.
    # `--skip script` was added in ad1e04e8 against via_ir bytecode churn, but
    # the directory filters provably do not affect a full build: the protocol
    # adapter bytecode is byte-identical with and without it on solc 0.8.33 and
    # 0.8.36, including after mutating the deploy script. Re-check on a solc
    # bump — `bindings-check` catches a regression.
    cd contracts && forge clean && forge bind \
        --skip test \
        --select '^(ProtocolAdapter|IProtocolAdapter|ICommitmentTree|INullifierSet|ERC1967Proxy)$' \
        --bindings-path ../crates/bindings/src/generated/ \
        --module \
        --overwrite

# Simulate deployment (dry-run)
contracts-simulate chain *args:
    @echo "IS_TEST_DEPLOYMENT: $IS_TEST_DEPLOYMENT"
    @echo "EMERGENCY_STOP_CALLER: $EMERGENCY_STOP_CALLER"
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/DeployProtocolAdapterProxy.s.sol:DeployProtocolAdapterProxy \
        --sig "run(bool,address)" $IS_TEST_DEPLOYMENT $EMERGENCY_STOP_CALLER \
        --rpc-url {{chain}} {{ args }}

# Deploy protocol adapter
contracts-deploy deployer chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/DeployProtocolAdapterProxy.s.sol:DeployProtocolAdapterProxy \
        --sig "run(bool,address)" $IS_TEST_DEPLOYMENT $EMERGENCY_STOP_CALLER \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Verify on sourcify
contracts-verify-sourcify address chain *args:
    cd contracts && env -u ETHERSCAN_API_KEY forge verify-contract {{address}} \
        src/ProtocolAdapter.sol:ProtocolAdapter \
        --chain {{chain}} --verifier sourcify --watch {{ args }}

# Verify on etherscan
contracts-verify-etherscan address chain *args:
    cd contracts && forge verify-contract {{address}} \
        src/ProtocolAdapter.sol:ProtocolAdapter \
        --chain {{chain}} --verifier etherscan --watch {{ args }}

# Verify on custom explorer
contracts-verify-custom address chain verifier-url *args:
    cd contracts && forge verify-contract {{address}} \
        src/ProtocolAdapter.sol:ProtocolAdapter \
        --chain {{chain}} --verifier-url {{verifier-url}} --watch {{ args }}

# Verify on both sourcify and etherscan
contracts-verify address chain: (contracts-verify-sourcify address chain) (contracts-verify-etherscan address chain)

# Publish contracts
contracts-publish version *args:
    cd contracts && forge soldeer push anoma-pa-evm~{{version}} {{ args }}

# --- Bindings ---

# Clean bindings
bindings-clean:
    cd crates/bindings && cargo clean

# Build bindings
bindings-build *args:
    cd crates/bindings && cargo build {{ args }}

# Test bindings
bindings-test *args:
    cd crates/bindings && cargo test {{ args }}

# Check bindings are up-to-date
bindings-check: contracts-gen-bindings
    git diff --exit-code crates/bindings/src/generated/

# Publish bindings
bindings-publish *args:
    cd crates/bindings && cargo publish {{ args }}

# Lint bindings (clippy)
bindings-lint:
    cd crates/bindings && cargo clippy --no-deps -- -Dwarnings
    cd crates/bindings && cargo clippy --no-deps --tests -- -Dwarnings

# Format bindings
bindings-fmt:
    cargo fmt

# Check bindings formatting
bindings-fmt-check:
    cargo fmt -- --check

# --- Crates (workspace-wide Rust) ---

# Clean all crates
crates-clean:
    cargo clean

# Build all crates
crates-build *args:
    cargo build {{ args }}

# Test all crates
crates-test *args:
    cargo test {{ args }}

# Lint all crates (clippy)
crates-lint:
    cargo clippy --all-targets --no-deps -- -Dwarnings

# Format all crates
crates-fmt *args:
    cargo fmt --all {{ args }}

# Check all crates formatting
crates-fmt-check:
    cargo fmt --all -- --check

# --- All ---

# Lint all (contracts + crates)
all-lint:
    @echo "==> Linting contracts..."
    @just contracts-lint
    @echo "==> Linting crates..."
    @just crates-lint

# Format all (contracts + crates)
all-fmt:
    @echo "==> Formatting contracts..."
    @just contracts-fmt
    @echo "==> Formatting crates..."
    @just crates-fmt

# Check formatting for all (contracts + crates)
all-fmt-check:
    @echo "==> Checking contract formatting..."
    @just contracts-fmt-check
    @echo "==> Checking crates formatting..."
    @just crates-fmt-check

# Build all (contracts + crates)
all-build:
    @echo "==> Building contracts..."
    @just contracts-build
    @echo "==> Building crates..."
    @just crates-build

# Test all (contracts + crates)
all-test:
    @echo "==> Testing contracts..."
    @just contracts-test
    @echo "==> Testing crates..."
    @just crates-test

# Prerequisites check (mirrors CI)
all-check:
    git status
    @echo "==> Checking storage layouts..."
    @just contracts-storage-check
    @echo "==> Static analysis with slither..."
    @just contracts-static-analysis
    @echo "==> Checking formatting..."
    @just all-fmt-check
    @echo "==> Linting..."
    @just all-lint
    @echo "==> Checking bindings are up-to-date..."
    @just bindings-check
