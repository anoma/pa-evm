| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4211863 |           19742 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             330 |    330 |    330 |    330 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             380 |    380 |    380 |    380 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             666 |    666 |    666 |    666 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             838 |    838 |    838 |    838 |       2 |
| VERSION                                          |            1077 |   1077 |   1077 |   1077 |       3 |
| commitmentCount                                  |             719 |   1719 |   1719 |   2719 |      12 |
| denyLogicRefs                                    |             720 |  31722 |  28870 |  53010 |      11 |
| execute                                          |            2916 | 185857 | 195745 | 434159 |      60 |
| executeWithCallOrder                             |          161924 | 269956 | 252934 | 431751 |       7 |
| getImplementation                                |             637 |    803 |    637 |   2637 |      12 |
| getKindTreeRoot                                  |            2851 |   2851 |   2851 |   2851 |       2 |
| initialize                                       |            3046 | 150778 | 152883 | 152883 |     133 |
| isCommitmentTreeRootHistorical                   |             921 |    921 |    921 |    921 |       1 |
| isLogicRefDenied                                 |            2446 |   2449 |   2449 |   2453 |      12 |
| isNullifierContained                             |             774 |   2773 |   2774 |   2774 |    2713 |
| latestCommitmentTreeRoot                         |            6967 |  11993 |   9658 |  17684 |      16 |
| migrateNullifierSet                              |             713 |    713 |    713 |    713 |       2 |
| owner                                            |             571 |   2383 |   2571 |   2571 |      32 |
| pause                                            |            2546 |  21639 |  25844 |  25844 |      12 |
| paused                                           |            2423 |   2423 |   2423 |   2423 |      15 |
| proxiableUUID                                    |             313 |    313 |    313 |    313 |      11 |
| riscZeroVerifierPaused                           |           10974 |  10974 |  10974 |  10974 |       5 |
| setKindTreeRoot                                  |             595 |   6987 |   8720 |   8720 |       7 |
| transferOwnership                                |            5465 |   5465 |   5465 |   5465 |       3 |
| unpause                                          |            2348 |   6395 |   8419 |   8419 |       3 |
| upgradeToAndCall                                 |            3039 |   8818 |   3248 |  24007 |       5 |

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

