# Show commands before running (helps debug failures)
set shell := ["bash", "-euo", "pipefail", "-c"]

# Recipes read `ALCHEMY_API_KEY` (fork tests, deploys) from the environment;
# forge does not load this file itself. The file is absent in CI, where the
# values come from secrets instead, so loading it stays optional.
# `IS_PRODUCTION` is deliberately not kept here — see the release
# checklist, which exports it once per deployment session.
set dotenv-path := "contracts/.env"
set dotenv-required := false

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
    cd contracts && forge clean && forge bind \
        --skip test \
        --select '^(ProtocolAdapter|IProtocolAdapter|ICommitmentTree|INullifierSet|ERC1967Proxy)$' \
        --bindings-path ../crates/bindings/src/generated/ \
        --module \
        --overwrite

# Simulate the implementation deployment (dry-run)
contracts-simulate-impl chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/DeployProtocolAdapterImplementation.s.sol:DeployProtocolAdapterImplementation \
        --sig "run()" \
        --rpc-url {{chain}} {{ args }}

# Deploy the implementation shared by the staging and production environments (idempotent)
contracts-deploy-impl deployer chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/DeployProtocolAdapterImplementation.s.sol:DeployProtocolAdapterImplementation \
        --sig "run()" \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Simulate the implementation and proxy deployment (dry-run)
contracts-simulate-proxy chain *args:
    @echo "IS_PRODUCTION: $IS_PRODUCTION"
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/DeployProtocolAdapterProxy.s.sol:DeployProtocolAdapterProxy \
        --sig "run(bool)" $IS_PRODUCTION \
        --rpc-url {{chain}} {{ args }}

# Deploy the protocol adapter implementation and proxy
contracts-deploy-proxy deployer chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/DeployProtocolAdapterProxy.s.sol:DeployProtocolAdapterProxy \
        --sig "run(bool)" $IS_PRODUCTION \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Simulate the staging upgrade (dry-run): validates the upgrade and runs it locally (sender = the staging proxy owner)
contracts-simulate-staging-upgrade sender proxy implementation chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/staging/ExecuteProtocolAdapterUpgrade.s.sol:ExecuteProtocolAdapterUpgrade \
        --sig "run(address,address)" {{proxy}} {{implementation}} \
        --sender {{sender}} --rpc-url {{chain}} {{ args }}

# Execute the staging upgrade to the deployed implementation as the proxy owner
contracts-execute-staging-upgrade deployer proxy implementation chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/staging/ExecuteProtocolAdapterUpgrade.s.sol:ExecuteProtocolAdapterUpgrade \
        --sig "run(address,address)" {{proxy}} {{implementation}} \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Simulate the production upgrade proposal (dry-run): simulates the Safe executing the upgrade
contracts-simulate-production-upgrade-proposal proxy proposer implementation chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/production/ProposeProtocolAdapterUpgrade.s.sol:ProposeProtocolAdapterUpgrade \
        --sig "run(address,address,address)" {{proxy}} {{proposer}} {{implementation}} \
        --rpc-url {{chain}} {{ args }}

# Propose the production upgrade to the deployed implementation to the owning Safe (proposer = unlocked deployer)
contracts-propose-production-upgrade deployer proxy proposer implementation chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/production/ProposeProtocolAdapterUpgrade.s.sol:ProposeProtocolAdapterUpgrade \
        --sig "run(address,address,address)" {{proxy}} {{proposer}} {{implementation}} \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Simulate the staging kind table update (dry-run): impersonates the owner (sender = the staging proxy owner)
contracts-simulate-staging-kind-table-update sender proxy commitment chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/staging/ExecuteKindTableUpdate.s.sol:ExecuteKindTableUpdate \
        --sig "run(address,bytes32)" {{proxy}} {{commitment}} \
        --sender {{sender}} --rpc-url {{chain}} {{ args }}

# Execute the staging kind table update as the proxy owner
contracts-execute-staging-kind-table-update deployer proxy commitment chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/staging/ExecuteKindTableUpdate.s.sol:ExecuteKindTableUpdate \
        --sig "run(address,bytes32)" {{proxy}} {{commitment}} \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Simulate the production kind table update proposal (dry-run): simulates the Safe executing the update
contracts-simulate-production-kind-table-proposal proxy proposer commitment chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/production/ProposeKindTableUpdate.s.sol:ProposeKindTableUpdate \
        --sig "run(address,address,bytes32)" {{proxy}} {{proposer}} {{commitment}} \
        --rpc-url {{chain}} {{ args }}

# Propose updating the production kind table commitment to the owning Safe (proposer = unlocked deployer)
contracts-propose-production-kind-table-update deployer proxy proposer commitment chain *args:
    @echo "Cleaning contracts to ensure reproducible build..."
    @just contracts-clean
    cd contracts && forge script script/production/ProposeKindTableUpdate.s.sol:ProposeKindTableUpdate \
        --sig "run(address,address,bytes32)" {{proxy}} {{proposer}} {{commitment}} \
        --broadcast --rpc-url {{chain}} --account {{deployer}} {{ args }}

# Verify a contract on sourcify (e.g. contract=src/ProtocolAdapter.sol:ProtocolAdapter)
contracts-verify-sourcify address contract chain *args:
    cd contracts && env -u ETHERSCAN_API_KEY forge verify-contract {{address}} {{contract}} \
        --chain {{chain}} --verifier sourcify --watch {{ args }}

# Verify a contract on etherscan (e.g. contract=src/ProtocolAdapter.sol:ProtocolAdapter). Reads the constructor
# args from the on-chain creation code and forces submission past a prior similar match.
contracts-verify-etherscan address contract chain *args:
    cd contracts && forge verify-contract {{address}} {{contract}} \
        --chain {{chain}} --verifier etherscan --watch \
        --rpc-url {{chain}} --guess-constructor-args --skip-is-verified-check {{ args }}

# Verify a contract on a custom explorer
contracts-verify-custom address contract chain verifier-url *args:
    cd contracts && forge verify-contract {{address}} {{contract}} \
        --chain {{chain}} --verifier-url {{verifier-url}} --watch {{ args }}

# Verify a contract on both sourcify and etherscan
contracts-verify address contract chain: (contracts-verify-sourcify address contract chain) (contracts-verify-etherscan address contract chain)

# Verify the protocol adapter implementation on both explorers
contracts-verify-impl implementation chain: (contracts-verify implementation "src/ProtocolAdapter.sol:ProtocolAdapter" chain)

# Verify the ERC-1967 proxy — which carries the proxy bytecode, not the implementation's — on both explorers
contracts-verify-proxy proxy chain: \
    (contracts-verify proxy "dependencies/@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol:ERC1967Proxy" chain)

# Verify a deployment — the protocol adapter implementation and the ERC-1967 proxy pointing at it — on both explorers
contracts-verify-deployment implementation proxy chain: \
    (contracts-verify-impl implementation chain) \
    (contracts-verify-proxy proxy chain)

# Publish contracts at the version `ProtocolAdapter` compiles to
contracts-publish *args:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Cleaning contracts to ensure reproducible build..."
    just contracts-clean
    just contracts-build
    cd contracts
    version="$(forge script script/PrintProtocolAdapterVersion.s.sol:PrintProtocolAdapterVersion --sig 'run()(string)' --json \
        | jq -ser '[.[] | select(has("returns")) | .returns.version.value] | if length == 1 then .[0] else error("expected one version, found \(length)") end')"
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
        printf '{{RED}}The protocol adapter reports "%s", which is not a version.{{NORMAL}}\n' "$version"
        exit 1
    fi
    printf '{{GREEN}}Publishing anoma-pa-evm~%s{{NORMAL}}\n' "$version"
    forge soldeer push "anoma-pa-evm~$version" {{ args }}

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

# Convert a tx-gen transaction (bincode) into `execute` calldata beside it; pass the kind table as second argument to verify first
bindings-execute-calldata transaction *args:
    cargo run -p anoma-pa-evm-bindings --example execute_calldata -- {{transaction}} {{ args }}

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
