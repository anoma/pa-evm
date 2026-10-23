| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3298755 |           15509 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             310 |    310 |    310 |    310 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             360 |    360 |    360 |    360 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             649 |    649 |    649 |    649 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             823 |    823 |    823 |    823 |       2 |
| VERSION                                          |            1060 |   1060 |   1060 |   1060 |       3 |
| commitmentCount                                  |             702 |   1702 |   1702 |   2702 |      12 |
| denyLogicRefs                                    |             700 |  31702 |  28850 |  52990 |      11 |
| execute                                          |            2868 | 151218 | 169559 | 420086 |      41 |
| getImplementation                                |             620 |    786 |    620 |   2620 |      12 |
| getKindTreeRoot                                  |            2834 |   2834 |   2834 |   2834 |       2 |
| initialize                                       |            3032 | 150263 | 152879 | 152879 |     107 |
| isLogicRefDenied                                 |            2426 |   2429 |   2429 |   2433 |      12 |
| isNullifierContained                             |            2758 |   2758 |   2758 |   2758 |    2712 |
| latestCommitmentTreeRoot                         |            6956 |  12347 |   9662 |  17733 |      15 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             551 |   2363 |   2551 |   2551 |      32 |
| pause                                            |            2526 |  21619 |  25824 |  25824 |      12 |
| paused                                           |            2403 |   2403 |   2403 |   2403 |      15 |
| proxiableUUID                                    |             293 |    293 |    293 |    293 |      11 |
| riscZeroVerifierPaused                           |           10954 |  10954 |  10954 |  10954 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6375 |   8399 |   8399 |       3 |
| upgradeToAndCall                                 |            3019 |   8778 |   3208 |  23947 |       5 |

| src/examples/BlockTimeForwarder.sol:BlockTimeForwarder Contract |                 |     |        |     |         |
|-----------------------------------------------------------------|-----------------|-----|--------|-----|---------|
| Deployment Cost                                                 | Deployment Size |     |        |     |         |
|                                                               0 |             569 |     |        |     |         |
|                                                                 |                 |     |        |     |         |
| Function Name                                                   | Min             | Avg | Median | Max | # Calls |
| forwardCall                                                     |             620 | 639 |    648 | 649 |       3 |

| test/mocks/CommitmentTree.m.sol:CommitmentTreeMock Contract |                 |       |        |       |         |
|-------------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                             | Deployment Size |       |        |       |         |
|                                                      831550 |            3698 |       |        |       |         |
|                                                             |                 |       |        |       |         |
| Function Name                                               | Min             | Avg   | Median | Max   | # Calls |
| addCommitment                                               |           17970 | 42512 |  30998 | 97286 |      63 |
| addCommitmentTreeRoot                                       |            2460 | 18358 |  23658 | 23658 |       4 |
| commitmentCount                                             |            2355 |  2355 |   2355 |  2355 |      42 |
| commitmentTreeCapacity                                      |            2452 |  2452 |   2452 |  2452 |       1 |
| commitmentTreeDepth                                         |            2317 |  2317 |   2317 |  2317 |      36 |
| commitmentTreeSides                                         |            2730 | 12114 |  14045 | 16308 |      34 |
| commitmentTreeZeros                                         |            4839 | 14149 |  16154 | 18417 |      35 |
| initialize                                                  |            2522 | 86661 |  93677 | 93677 |      23 |
| isCommitmentTreeRootHistorical                              |            2498 |  2498 |   2498 |  2498 |       4 |
| latestCommitmentTreeRoot                                    |            6625 | 10581 |  10987 | 14677 |      10 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290489 |            1189 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23529 | 38652 |  43693 | 43693 |       4 |
| initialize                                              |            2519 | 18415 |  23650 | 23840 |       4 |
| isNullifierContained                                    |            2360 |  2360 |   2360 |  2360 |       3 |

