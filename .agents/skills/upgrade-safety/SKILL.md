---
name: upgrade-safety
description: >-
  Checklist for changing contracts/src of the pa-evm protocol adapter, which runs behind a UUPS
  proxy. Use before adding or changing state, an initializer, a constructor, an owner function, an
  inherited base or the ABI, and when reviewing such a change.
---

# Upgrade safety

An upgrade keeps the proxy's storage and swaps its code, so a change to `src` is safe only if the
new code reads the old storage correctly. Passing tests do not show that; check these by hand.

## Storage

- State lives only in ERC-7201 namespaces; `just contracts-storage-check` enforces it.
- A new module copies `src/state/KindTreeRoot.sol`: a `<Module>Storage` struct annotated
  `/// @custom:storage-location erc7201:anoma.storage.<Module>`, a precomputed
  `_<MODULE>_STORAGE_SLOT` with its derivation in a comment, and
  `test/state/<Module>.storage.t.sol` checking it against `SlotDerivation.erc7201Slot`.
- In an existing struct, only append fields. Never reorder, retype, repurpose or delete one, and
  never change a namespace string: deployed proxies hold data there.

## Constructors and initializers

- Constructors only set immutables and call `_disableInitializers()`, annotated
  `@custom:oz-upgrades-unsafe-allow constructor` (and `state-variable-immutable` per immutable).
- Each module has `__<Module>_init() internal onlyInitializing`, called from
  `__ProtocolAdapter_init`. Deployed proxies never run it again, so state a new module needs after
  an upgrade must be set another way. Say how in the PR.
- Constructor arguments and `VERSION` are creation code: they move the implementation's CREATE2
  address on every chain.

## Owner functions

- Owner functions are `nonReentrant onlyOwner`, so they revert while `execute` runs (#776):
  `pause` and `unpause` directly, upgrades, kind tree root and denylist changes through their
  `_authorize*` hooks. A new one gets the same. The copy-in functions of
  `MigrationalProtocolAdapter` are `onlyOwner whenPaused` instead.
- An override of `_authorizeUpgrade` calls `super._authorizeUpgrade`.
- `transferOwnership` and `renounceOwnership` stay open on purpose.

## Validation

- `test/ProtocolAdapter.upgrade.t.sol` validates upgrades with `openzeppelin-foundry-upgrades`; it
  needs `bun install` in `contracts/`. Extend it when an upgrade path changes.
- Fix the source instead of adding an `@custom:oz-upgrades-unsafe-allow` to silence a finding.
- An ABI change of a contract in the `--select` list of `contracts-gen-bindings` needs
  `just contracts-gen-bindings` and a `!` in the commit type.

Whether a change is safe for production is for the maintainers and auditors to decide. Report what
you checked and what you could not; do not call a change "safe".
