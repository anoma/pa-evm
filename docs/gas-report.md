| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4274103 |           20034 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             357 |    357 |    357 |    357 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             407 |    407 |    407 |    407 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             671 |    671 |    671 |    671 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             849 |    849 |    849 |    849 |       2 |
| VERSION                                          |            1088 |   1088 |   1088 |   1088 |       3 |
| commitmentCount                                  |             724 |   1724 |   1724 |   2724 |      12 |
| denyLogicRefs                                    |             733 |  31724 |  28871 |  53011 |      11 |
| execute                                          |            2653 | 190978 | 196695 | 434248 |      66 |
| getImplementation                                |             642 |    808 |    642 |   2642 |      12 |
| getKindTreeRoot                                  |            2856 |   2856 |   2856 |   2856 |       2 |
| initialize                                       |            3067 | 150811 | 152932 | 152932 |     132 |
| isCommitmentTreeRootHistorical                   |             922 |    922 |    922 |    922 |       1 |
| isLogicRefDenied                                 |            2447 |   2450 |   2450 |   2454 |      12 |
| isNullifierContained                             |             783 |   2782 |   2783 |   2783 |    2713 |
| latestCommitmentTreeRoot                         |            6992 |  12103 |   9726 |  17893 |      16 |
| migrateNullifierSet                              |             726 |    726 |    726 |    726 |       2 |
| owner                                            |             598 |   2410 |   2598 |   2598 |      32 |
| pause                                            |            2585 |  21675 |  25880 |  25880 |      12 |
| paused                                           |            2450 |   2450 |   2450 |   2450 |      15 |
| proxiableUUID                                    |             340 |    340 |    340 |    340 |      11 |
| riscZeroVerifierPaused                           |           11001 |  11001 |  11001 |  11001 |       5 |
| setKindTreeRoot                                  |             638 |   7037 |   8771 |   8771 |       7 |
| transferOwnership                                |            5487 |   5487 |   5487 |   5487 |       3 |
| unpause                                          |            2387 |   6432 |   8455 |   8455 |       3 |
| upgradeToAndCall                                 |            3062 |   8888 |   3322 |  24115 |       5 |

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
| addCommitment                                               |           17970 | 43368 |  30998 | 97286 |      64 |
| addCommitmentTreeRoot                                       |            2460 | 18358 |  23658 | 23658 |       4 |
| commitmentCount                                             |            2355 |  2355 |   2355 |  2355 |      42 |
| commitmentTreeCapacity                                      |            2452 |  2452 |   2452 |  2452 |       1 |
| commitmentTreeDepth                                         |            2317 |  2317 |   2317 |  2317 |      36 |
| commitmentTreeSides                                         |            2730 | 12114 |  14045 | 16308 |      34 |
| commitmentTreeZeros                                         |            4839 | 14149 |  16154 | 18417 |      35 |
| initialize                                                  |            2522 | 86953 |  93677 | 93677 |      24 |
| isCommitmentTreeRootHistorical                              |            2498 |  2498 |   2498 |  2498 |       4 |
| latestCommitmentTreeRoot                                    |            6625 | 10221 |   9980 | 14677 |      11 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290489 |            1189 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23529 | 38652 |  43693 | 43693 |       4 |
| initialize                                              |            2519 | 18415 |  23650 | 23840 |       4 |
| isNullifierContained                                    |            2360 |  2360 |   2360 |  2360 |       3 |

