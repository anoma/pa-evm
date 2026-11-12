---
name: upgrade-safety
description: >-
  Checklist for changing anything in contracts/src of the pa-evm protocol adapter, which runs
  behind a UUPS proxy. Use before adding or changing state, an initializer, a constructor, an
  owner function, an inherited base, or the ABI, and when reviewing such a change.
---

# Upgrade safety

Every chain runs the protocol adapter behind an ERC-1967 proxy in the UUPS pattern. An upgrade
keeps the proxy's storage and swaps the code, so a change to `src` is safe only if the new code
reads the old storage correctly. `forge test` passing does not show that. Check these by hand.

## Storage

- State lives only in ERC-7201 namespaces. `just contracts-storage-check` fails on any other
  storage in `src`.
- One state module per file in `src/state/`, following `KindTreeRoot.sol`:
  - a `<Module>Storage` struct annotated `/// @custom:storage-location erc7201:anoma.storage.<Module>`;
  - `_<MODULE>_STORAGE_SLOT`, the precomputed slot with the derivation formula in a comment above it;
  - `test/state/<Module>.storage.t.sol` asserting the slot equals
    `SlotDerivation.erc7201Slot("anoma.storage.<Module>")`.
- In an existing struct, only append fields. Never reorder, retype, rename-and-repurpose or
  delete one: deployed proxies hold data at those offsets.
- Never change an existing namespace string; it moves the slot and orphans the state.

## Constructors and initializers

- Constructors only set immutables and call `_disableInitializers()`. Annotate them
  `/// @custom:oz-upgrades-unsafe-allow constructor`, and each immutable
  `/// @custom:oz-upgrades-unsafe-allow state-variable-immutable` (combined on a constructor that
  sets them: `constructor state-variable-immutable`).
- Each module has `__<Module>_init() internal onlyInitializing`, called from
  `__ProtocolAdapter_init`. A new module there only initializes fresh proxies; a deployed proxy
  never runs it again, so state it needs after an upgrade must be set some other way. Say so in the PR.
- Constructor arguments and `VERSION` are part of the creation code: they change the CREATE2
  address of the implementation on every chain.

## Owner functions

- Owner functions are `nonReentrant onlyOwner`, so they revert while `execute` runs (PR #776):
  `pause` and `unpause` directly, the upgrade, kind tree root and denylist changes through their
  `_authorize*` hooks in `ProtocolAdapter`. A new owner function gets the same. The copy-in
  functions of `MigrationalProtocolAdapter` are `onlyOwner whenPaused` instead.
- An override of `_authorizeUpgrade` calls `super._authorizeUpgrade`, as
  `MigrationalProtocolAdapter` does, so it keeps the guard and the owner check.
- `transferOwnership` and `renounceOwnership` stay open on purpose.

## Validation and tests

- `test/ProtocolAdapter.upgrade.t.sol` validates upgrades with `openzeppelin-foundry-upgrades`
  (`Upgrades.deployUUPSProxy`, `Upgrades.upgradeProxy`). It needs `ffi`, `ast`, `build_info` and
  `storageLayout`, which `foundry.toml` sets, and `bun install` in `contracts/`. Extend it when an
  upgrade path changes.
- Fix the source rather than adding an `@custom:oz-upgrades-unsafe-allow` to silence a finding.
- A change to the ABI of a contract in the `--select` list of `contracts-gen-bindings` needs
  `just contracts-gen-bindings` and a `!` in the commit type.

## Out of scope

Deciding whether a change is safe for production is for the maintainers and the auditors. Report
what you checked and what you could not, and do not call a change "safe".
