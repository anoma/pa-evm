| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4151194 |           19487 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TABLE_COMMITMENT                      |             662 |    662 |    662 |    662 |      11 |
| RISC_ZERO_VERIFIER_ROUTER                        |             356 |    356 |    356 |    356 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             690 |    690 |    690 |    690 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             824 |    824 |    824 |    824 |       2 |
| VERSION                                          |            1088 |   1088 |   1088 |   1088 |       3 |
| commitmentCount                                  |             721 |   1721 |   1721 |   2721 |      12 |
| denyLogicRefs                                    |             730 |  31483 |  28868 |  53008 |      12 |
| execute                                          |            2852 | 131411 | 163008 | 328271 |      35 |
| getImplementation                                |             617 |    783 |    617 |   2617 |      12 |
| getKindTableCommitment                           |            2916 |   2916 |   2916 |   2916 |       2 |
| initialize                                       |            3064 | 150413 | 152935 | 152935 |     111 |
| isLogicRefDenied                                 |            2468 |   2471 |   2471 |   2475 |      12 |
| isNullifierContained                             |            2780 |   2780 |   2780 |   2780 |    2712 |
| latestCommitmentTreeRoot                         |            6967 |  12419 |   9701 |  17868 |      15 |
| migrateNullifierSet                              |             745 |    745 |    745 |    745 |       2 |
| owner                                            |             525 |   2337 |   2525 |   2525 |      32 |
| pause                                            |            2534 |  21617 |  25820 |  25820 |      12 |
| paused                                           |            2421 |   2421 |   2421 |   2421 |      15 |
| proxiableUUID                                    |             311 |    311 |    311 |    311 |      11 |
| riscZeroVerifierPaused                           |           10956 |  10956 |  10956 |  10956 |       5 |
| setKindTableCommitment                           |             745 |   7144 |   8878 |   8878 |       7 |
| simulateExecute                                  |            3172 | 184838 | 175990 | 401335 |      10 |
| transferOwnership                                |            5506 |   5506 |   5506 |   5506 |       3 |
| unpause                                          |            2358 |   6397 |   8417 |   8417 |       3 |
| upgradeToAndCall                                 |            3037 |   8809 |   3244 |  23979 |       5 |

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
| addCommitment                                               |           17973 | 42515 |  31001 | 97289 |      63 |
| addCommitmentTreeRoot                                       |            2463 | 18361 |  23661 | 23661 |       4 |
| commitmentCount                                             |            2358 |  2358 |   2358 |  2358 |      42 |
| commitmentTreeCapacity                                      |            2455 |  2455 |   2455 |  2455 |       1 |
| commitmentTreeDepth                                         |            2320 |  2320 |   2320 |  2320 |      36 |
| commitmentTreeSides                                         |            2733 | 12117 |  14048 | 16311 |      34 |
| commitmentTreeZeros                                         |            4842 | 14152 |  16157 | 18420 |      35 |
| initialize                                                  |            2525 | 86664 |  93680 | 93680 |      23 |
| isCommitmentTreeRootHistorical                              |            2501 |  2501 |   2501 |  2501 |       4 |
| latestCommitmentTreeRoot                                    |            6628 | 10584 |  10990 | 14680 |      10 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290705 |            1190 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23532 | 38655 |  43696 | 43696 |       4 |
| initialize                                              |            2522 | 18418 |  23653 | 23843 |       4 |
| isNullifierContained                                    |            2363 |  2363 |   2363 |  2363 |       3 |

