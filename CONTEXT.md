# Anoma EVM Protocol Adapter

The protocol adapter contract and integration-test layer that executes **Anoma Resource Machine** transactions on EVM-compatible chains.

## Language

**Protocol Adapter (PA)**:
The EVM contract that verifies and executes ARM transactions on-chain — checking the aggregation proof and the delta proof and updating the commitment tree and nullifier set. Use "protocol adapter" (not "verifier" or "settler") for the contract. Without a qualifier, it is the upgradeable protocol adapter of an environment on a chain, behind its proxy; the legacy protocol adapter always carries its qualifier.
_Avoid_: v2, PA v2, new adapter, current adapter, protocol adapter proxy

**Anoma Resource Machine (ARM)**:
The state model of the transactions that the PA executes: state is a set of immutable **resources** that transactions consume and create. The off-chain ARM types live in `anoma-rm-risc0`.

**Resource**:
A unit of ARM state with a quantity, for example an amount of a token that a user owns. For each resource of a transaction, the chain shows the commitment or the nullifier, the logic ref and the app data, which contains the external calls. A **persistent** resource stays until a later transaction consumes it. An **ephemeral** resource exists only in one transaction.

**Action**:
One ordered list of consumed and created resources, proven together by one compliance unit and one resource logic proof per resource. The PA processes the resources, and makes their external calls, in the order of the list.

**Action tree root**:
A hash over the nullifiers and the commitments of an action, in the order of its list. The resource logic proofs and the delta proof cover it, and a resource logic can require the signature of the owner over it.

**Transaction**:
A list of actions plus a delta proof, an aggregation proof and the root of the kind tree that the aggregation proof is proven against, submitted to the PA's `execute` function.

**Resource logic**:
The circuit that decides when a transaction can create or consume a resource. Its proof for a resource is the resource logic proof of that resource.

**Logic ref**:
The hash that identifies the code of a resource logic. A **version** is one release of a resource logic, with its own logic ref.

**Label ref**:
The hash of the data that separates the resources of one resource logic. For an ERC20 resource, the data are the addresses of a forwarder and a token.

**In-circuit check**:
A check that a circuit makes: a proof of the circuit is valid only if the check holds.

**Out-of-circuit check**:
A check outside the circuits, for example a check that the PA makes on chain.

**Compliance unit / Aggregation proof / Delta proof**:
Each action has one compliance unit. Its compliance proof shows the **compliance checks**: the in-circuit checks that the action follows the rules of the ARM. The aggregation proof aggregates the compliance proofs and the resource logic proofs of a transaction into one proof, so the PA verifies only the aggregation proof and the delta proof. The delta proof shows that a transaction balances: for each kind base, the consumed and the created quantities are equal.

**Aggregation journal**:
The public statement the aggregation proof attests to — the compliance verifying key, the kind tree root, and the transaction's actions. The PA reconstructs it from calldata, injecting the compliance verifying key, and verifies the aggregation proof against its digest.

**Kind**:
The hash to curve of the logic ref and the label ref of a resource: a point on an elliptic curve.

**Kind base**:
The point that the compliance circuit multiplies by the quantity of a resource: its kind, or the point that a kind tree gives its kind. Two resources balance against each other only if they have the same kind base.
_Avoid_: kind point

**Kind tree**:
A Merkle tree of entries, each of which gives a kind a kind base. Two kinds with the same kind base are **aliases**.
_Avoid_: kind table

**Kind tree root**:
The root of a kind tree. The PA stores one, which its owner sets, and accepts a transaction proven against the stored kind tree or against the empty kind tree, whose root is the padding leaf `sha256("EMPTY")`.
_Avoid_: kind table root

**Unit Delta**:
The elliptic-curve point carrying the delta value of an action's compliance unit — one per action, summed across the transaction and checked against the delta proof.

**Commitment tree**:
A Merkle tree of all commitments. It only grows. A **historical root** is a root that the PA stores. A transaction proves each resource that it consumes against a historical root.

**Nullifier set**:
The nullifiers of all consumed resources. The PA reverts a transaction with a nullifier that the set contains, so no resource is consumed twice.

**Logic ref denylist**:
Two lists of logic refs, one for consumed and one for created resources: a transaction reverts if it consumes a resource whose logic ref is on the first list or creates a resource whose logic ref is on the second. The owner _deprecates_ a logic ref once a newer circuit version replaces it: it goes on the list for created resources, and transactions still consume its resources. The owner _denies_ a logic ref when its circuit turns out to be broken: it goes on both lists. No function removes an entry. Say "denied" (not "refused" or "blocked").

**Forwarder**:
A contract of an application that the PA calls, for example the ERC20 forwarder or the generic call forwarder. Each lives in its own repository.

**External call**:
A call that the PA makes to a forwarder when it processes a resource. The external payload of the resource carries it, and the resource logic proof covers it. The PA passes the logic ref of the resource. Each forwarder must accept only the logic refs of its own resources.
_Avoid_: forwarder call (identifiers such as `ForwarderCallExecuted` keep it)

**App data / Deletion criterion**:
The app data of a resource holds four lists of blobs: the resource payload, the discovery payload, the external payload and the application payload. The deletion criterion of a blob is `Never` or `Immediately`. The PA emits each blob with `Never` as an event. Each blob of the external payload is an external call, whatever its deletion criterion.

**ERC20 forwarder**:
The forwarder of AnomaPay that holds the tokens that back the ERC20 resources. A **wrap** takes tokens from a user and creates ERC20 resources. An **unwrap** consumes ERC20 resources and sends the tokens that back them.

**Generic call forwarder**:
A forwarder that makes calls to other contracts for a transaction, for example a token swap. Its resources are ephemeral.

**Proxy / Implementation**:
The PA is an ERC-1967 proxy in the UUPS pattern. A proxy keeps its address and its state, and runs the code of an implementation. An upgrade sets a new implementation.

**Owner**:
The account that can upgrade a proxy and use its administrative functions. The deployment wallet owns the staging proxies, and a Safe multisig owns the production proxies.

**Environment**:
One of the two protocol adapter deployments the repo maintains, each recorded per chain in the deployment record and tracking a branch. Say "environment" (not "network" or "deployment target") — a chain is where an environment lives, not which one it is.

**Staging / Production**:
The two environments. Staging is owned by the deployment wallet and upgraded directly; production is owned by a Safe multisig whose signers confirm and execute upgrades. The branches tracking them keep their own names, `staging` and `main`.

**Promotion**:
Moving a commit unchanged from `next` to `staging`, or from `staging` to `main`. The pull request opening one carries the gate proving the environment it targets runs that commit's source. Changes only ever flow this way.

**Deployment record**:
`crates/bindings/deployments.json` — the proxy address of each environment on each chain, plus the genesis fields pinning how that address was derived, and, under `legacy`, the legacy protocol adapter of each chain that ran one. Written once per chain at its first deploy and never edited, except that a redeployed release candidate replaces its record; what an environment currently runs is read from the chain, not from here. A replaced release-candidate proxy is short-lived and never becomes legacy.

**Legacy protocol adapter**:
The protocol adapter that a chain ran before its upgradeable protocol adapter: one immutable contract, with no kind tree root, on each chain that ran one. It verifies proofs of the legacy version of arm-risc0. Its owner can stop it, and nothing lifts the stop. No new one is deployed. On a migrated chain, it is stopped, and the migrational implementation copies its state into the protocol adapter.
_Avoid_: v1, PA v1, old adapter, immutable protocol adapter

**Galileo upgrade**:
The replacement of the legacy protocol adapter of a chain with the protocol adapter, once for each chain. After it, each change of the code is an upgrade in place.

**Migrated chain**:
A chain whose protocol adapter copied the state of its legacy protocol adapter.

**Legacy ERC20 forwarder**:
The immutable ERC20 forwarder of the legacy protocol adapter.

**Legacy-forwarder resource**:
An ERC20 resource whose label ref names the legacy ERC20 forwarder.

**Migrational implementation**:
`MigrationalProtocolAdapter`, the implementation that the proxy of a chain that ran a legacy protocol adapter starts on. It is a protocol adapter that begins paused and accepts the state of the legacy protocol adapter. Its proxy is owned by the deployment wallet in both environments; a production one moves to the Safe after the upgrade to the plain implementation.
_Avoid_: transition implementation, migration contract

**Plain implementation**:
`ProtocolAdapter`, the implementation every chain ends on. Its address is deterministic per chain.

**Copy-in**:
Writing the state of the legacy protocol adapter into the proxy: `migrateCommitmentTree` once, then `migrateNullifierSet` per batch. Allowed only while the proxy is paused, and removed by the upgrade to the plain implementation.
_Avoid_: seeding (the function names say it; the act has its own word), import

**Migration run**:
One execution of `MigrateProtocolAdapterState.run` for one chain: the copy-in. The proxy stays paused, so the ERC20 forwarder balances move before anyone can transact. Every step skips once it is done, so a run that stops early is repeated until it reaches the end.

**Completion run**:
One execution of `FinalizeProtocolAdapterStateMigration.run` for one chain, after the ERC20 forwarder balances moved: the unpause, which checks the copied state against the legacy protocol adapter, the upgrade and, in production, the transfer to the production proxy owner. Every step skips once it is done, so a run that stops early is repeated until it reaches the end.

**Left siblings**:
The hashes that a commitment tree stores instead of its leaves: on each level where the leaf count has a one bit, the sibling on that level in the Merkle path of the next leaf; the other levels hold stale values. The legacy protocol adapter stores them as `_sides` and exposes no getter for them, so the script reads them from its storage, and the migrational implementation checks that they reproduce its root.

**State migration**:
The one-time copy of the commitment tree and the nullifier set of a chain's legacy protocol adapter into the protocol adapter. The migrational implementation makes the copy. It changes no resource. In the same period, the asset migration of the forwarder repository moves the tokens of the legacy ERC20 forwarder.
_Avoid_: migration (on its own)

**Conversion**:
A transaction that consumes resources of one kind and creates resources of another kind with the same kind base. The stored kind tree gives one kind the kind base of the other, so the transaction balances. Only a conversion needs the stored kind tree.

**Soft migration**:
The process in which users convert their resources of a deprecated logic ref into resources of the active logic ref. The PA still consumes resources of the deprecated logic ref, so their circuit must still be trusted.
_Avoid_: migration (on its own)

**Hard migration**:
The process in which users replace their unspent resources of a denied logic ref with resources of a new logic ref. The in-circuit checks of the new logic show that the user owns the old resources, and the application makes out-of-circuit checks against the PA that they exist and are unspent. The PA consumes no resource of the denied logic ref. The application stores their nullifiers instead, so no proof of the denied circuit runs again. The ERC20 forwarder and its circuit call it emergency migration.
_Avoid_: migration (on its own), rescue
