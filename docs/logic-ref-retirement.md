# Retiring a logic ref

Status: proposed rollout requirement. The conversion-and-withdrawal scenario below still needs an integration test.

## Deprecation and withdrawal

The statuses distinguish creation from consumption as follows. Status changes are irreversible through the status setters. [PA statuses (2026/10), setters and checks][pa-statuses]

| Status | Creation | Consumption |
| --- | --- | --- |
| Active | Allowed | Allowed |
| Deprecated | Rejected | Allowed |
| Denied | Rejected | Rejected |

The ERC20 unwrap fixture consumes a persistent resource and creates an ephemeral carrier for the withdrawal call. That carrier uses the transfer circuit's logic ref. Consequently, deprecating that ref rejects this unwrap's created resource even when its consumed resource is permitted. [Unwrap fixture (2026/10), action][unwrap-action] [Unwrap carrier (2026/10), logic ref][unwrap-resource] [PA statuses (2026/10), creation check][pa-statuses]

The forwarder separately requires the carrier's logic ref to equal its configured ref. A successor circuit therefore needs a compatible forwarder configuration as well as adapter acceptance. [Forwarder base (2026/10), `forwardCall`][forwarder-base]

## Proposed acceptance test

Before deprecating an old ref `L`, test the following against the intended adapter, forwarder, prover, and kind-table versions:

1. Wrap tokens under `L` and retain the resulting persistent resource. Confirm the original unwrap succeeds in simulation before changing statuses, keeping the resource unspent.
2. Prepare a distinct active successor `M`, the kind-table aliases needed for conversion, and a forwarder upgrade or migration that accepts `M` and can release the original backing tokens.
3. Deprecate `L`. As a negative control, verify that the original unwrap fails with `DeprecatedLogicRef(L)` and leaves token balances and adapter state unchanged.
4. Consume the retained `L` resource and create an `M` resource, using proofs and the accepted kind-table commitment for that conversion.
5. Withdraw through `M`. Assert the recipient's token balance increases by the expected amount and the backing balance decreases accordingly.
6. Verify that the reverse conversion cannot create another `L` resource, and that denying `L` prevents its consumption and conversion too.

Run the negative controls from snapshots with unspent inputs so failures come from the intended checks. Use real circuit proofs for the exit-path acceptance test. Mocked verifier tests can cover adapter rejection and rollback separately. Keep the pinned circuit image IDs, table commitment, forwarder implementation/configuration, and test command with the result.

Do not mark the exit path verified until steps 4 and 5 pass. Treat moving `L` from Deprecated to Denied as a separate decision about preventing consumption of remaining resources.

## Sources

Dates identify the reviewed snapshots, not deployment dates.

[PA statuses, 2026/10] Anoma Foundation. `LogicRefStatuses.sol`, PR #660 at `a0e82fd`. [Source][pa-statuses].

[Unwrap fixture, 2026/10] Anoma Foundation. ERC20 forwarder unwrap action and resource fixtures at `c9063ad`. [Action][unwrap-action], [carrier][unwrap-resource].

[Forwarder base, 2026/10] Anoma Foundation. `ForwarderBaseUpgradeable.sol`, contracts v3.0.1 at `ae34c69`. [Source][forwarder-base].

[pa-statuses]: https://github.com/anoma/pa-evm/blob/a0e82fd77d910af25c0a0905b9f48561039b7998/contracts/src/state/LogicRefStatuses.sol
[unwrap-action]: https://github.com/anoma/anomapay-erc20-forwarder/blob/c9063adb9f018a2a1779031c2a3f573540602a60/crates/integration-test/src/fixtures/unwrap/action.rs
[unwrap-resource]: https://github.com/anoma/anomapay-erc20-forwarder/blob/c9063adb9f018a2a1779031c2a3f573540602a60/crates/integration-test/src/fixtures/unwrap/resource.rs
[forwarder-base]: https://github.com/anoma/forwarder-bases/blob/ae34c69656556afba41762f467141adc52a647c0/contracts/src/ForwarderBaseUpgradeable.sol
