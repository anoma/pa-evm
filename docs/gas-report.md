| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4036765 |           18929 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             307 |    307 |    307 |    307 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             357 |    357 |    357 |    357 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             646 |    646 |    646 |    646 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             820 |    820 |    820 |    820 |       2 |
| VERSION                                          |            1057 |   1057 |   1057 |   1057 |       3 |
| commitmentCount                                  |             699 |   1699 |   1699 |   2699 |      12 |
| denyLogicRefs                                    |             703 |  31699 |  28847 |  52987 |      11 |
| execute                                          |            2871 | 185923 | 194592 | 437965 |      58 |
| getImplementation                                |             617 |    783 |    617 |   2617 |      12 |
| getKindTreeRoot                                  |            2831 |   2831 |   2831 |   2831 |       2 |
| initialize                                       |            3035 | 150625 | 152882 | 152882 |     124 |
| isLogicRefDenied                                 |            2423 |   2426 |   2426 |   2430 |      12 |
| isNullifierContained                             |             755 |   2754 |   2755 |   2755 |    2713 |
| latestCommitmentTreeRoot                         |            6953 |  12344 |   9659 |  17729 |      15 |
| migrateNullifierSet                              |             696 |    696 |    696 |    696 |       2 |
| owner                                            |             548 |   2360 |   2548 |   2548 |      32 |
| pause                                            |            2529 |  21617 |  25821 |  25821 |      12 |
| paused                                           |            2400 |   2400 |   2400 |   2400 |      15 |
| proxiableUUID                                    |             290 |    290 |    290 |    290 |      11 |
| riscZeroVerifierPaused                           |           10957 |  10957 |  10957 |  10957 |       5 |
| setKindTreeRoot                                  |             578 |   6970 |   8703 |   8703 |       7 |
| transferOwnership                                |            5453 |   5453 |   5453 |   5453 |       3 |
| unpause                                          |            2331 |   6374 |   8396 |   8396 |       3 |
| upgradeToAndCall                                 |            3016 |   8772 |   3202 |  23938 |       5 |

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

