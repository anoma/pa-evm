| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3194819 |           15024 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             310 |    310 |    310 |    310 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             360 |    360 |    360 |    360 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             649 |    649 |    649 |    649 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             796 |    796 |    796 |    796 |       2 |
| VERSION                                          |            1060 |   1060 |   1060 |   1060 |       3 |
| commitmentCount                                  |             677 |   1677 |   1677 |   2677 |      12 |
| denyLogicRefs                                    |             703 |  31702 |  28850 |  52990 |      11 |
| execute                                          |            3025 | 165558 | 192395 | 420581 |      45 |
| getImplementation                                |             595 |    761 |    595 |   2595 |      12 |
| getKindTreeRoot                                  |            2834 |   2834 |   2834 |   2834 |       2 |
| initialize                                       |            3007 | 150325 | 152847 | 152847 |     111 |
| isLogicRefDenied                                 |            2426 |   2429 |   2429 |   2433 |      12 |
| isNullifierContained                             |            2732 |   2732 |   2732 |   2732 |    2712 |
| latestCommitmentTreeRoot                         |            6925 |  12161 |   9951 |  17641 |      16 |
| migrateNullifierSet                              |             696 |    696 |    696 |    696 |       2 |
| owner                                            |             551 |   2363 |   2551 |   2551 |      32 |
| pause                                            |            2529 |  21619 |  25824 |  25824 |      12 |
| paused                                           |            2403 |   2403 |   2403 |   2403 |      15 |
| proxiableUUID                                    |             293 |    293 |    293 |    293 |      11 |
| riscZeroVerifierPaused                           |           10960 |  10960 |  10960 |  10960 |       5 |
| setKindTreeRoot                                  |             578 |   6970 |   8703 |   8703 |       7 |
| transferOwnership                                |            5453 |   5453 |   5453 |   5453 |       3 |
| unpause                                          |            2331 |   6376 |   8399 |   8399 |       3 |
| upgradeToAndCall                                 |            3019 |   8778 |   3208 |  23947 |       5 |

| src/examples/BlockTimeForwarder.sol:BlockTimeForwarder Contract |                 |     |        |     |         |
|-----------------------------------------------------------------|-----------------|-----|--------|-----|---------|
| Deployment Cost                                                 | Deployment Size |     |        |     |         |
|                                                               0 |             570 |     |        |     |         |
|                                                                 |                 |     |        |     |         |
| Function Name                                                   | Min             | Avg | Median | Max | # Calls |
| forwardCall                                                     |             623 | 642 |    651 | 652 |       3 |

| test/mocks/CommitmentTree.m.sol:CommitmentTreeMock Contract |                 |       |        |       |         |
|-------------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                             | Deployment Size |       |        |       |         |
|                                                      831766 |            3699 |       |        |       |         |
|                                                             |                 |       |        |       |         |
| Function Name                                               | Min             | Avg   | Median | Max   | # Calls |
| addCommitment                                               |           17973 | 43715 |  31001 | 97289 |      65 |
| addCommitmentTreeRoot                                       |            2463 | 19421 |  23661 | 23661 |       5 |
| commitmentCount                                             |            2358 |  2358 |   2358 |  2358 |      42 |
| commitmentTreeCapacity                                      |            2455 |  2455 |   2455 |  2455 |       1 |
| commitmentTreeDepth                                         |            2320 |  2320 |   2320 |  2320 |      36 |
| commitmentTreeSides                                         |            2733 | 12117 |  14048 | 16311 |      34 |
| commitmentTreeZeros                                         |            4842 | 14152 |  16157 | 18420 |      35 |
| initialize                                                  |            2525 | 86956 |  93680 | 93680 |      24 |
| isCommitmentTreeRootHistorical                              |            2501 |  2501 |   2501 |  2501 |       4 |
| latestCommitmentTreeRoot                                    |            6628 | 10529 |   9983 | 14680 |      11 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290705 |            1190 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23532 | 38655 |  43696 | 43696 |       4 |
| initialize                                              |            2522 | 18418 |  23653 | 23843 |       4 |
| isNullifierContained                                    |            2363 |  2363 |   2363 |  2363 |       3 |

