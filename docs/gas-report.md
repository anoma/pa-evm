| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4032530 |           18908 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             313 |    313 |    313 |    313 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             363 |    363 |    363 |    363 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             653 |    653 |    653 |    653 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             831 |    831 |    831 |    831 |       2 |
| VERSION                                          |            1070 |   1070 |   1070 |   1070 |       3 |
| commitmentCount                                  |             706 |   1706 |   1706 |   2706 |      12 |
| denyLogicRefs                                    |             715 |  31706 |  28853 |  52993 |      11 |
| execute                                          |            2881 | 185710 | 194623 | 434286 |      58 |
| getImplementation                                |             624 |    790 |    624 |   2624 |      12 |
| getKindTreeRoot                                  |            2838 |   2838 |   2838 |   2838 |       2 |
| initialize                                       |            3049 | 150662 | 152920 | 152920 |     124 |
| isLogicRefDenied                                 |            2429 |   2432 |   2432 |   2436 |      12 |
| isNullifierContained                             |             765 |   2764 |   2765 |   2765 |    2713 |
| latestCommitmentTreeRoot                         |            6974 |  12426 |   9708 |  17874 |      15 |
| migrateNullifierSet                              |             708 |    708 |    708 |    708 |       2 |
| owner                                            |             554 |   2366 |   2554 |   2554 |      32 |
| pause                                            |            2541 |  21624 |  25827 |  25827 |      12 |
| paused                                           |            2406 |   2406 |   2406 |   2406 |      15 |
| proxiableUUID                                    |             296 |    296 |    296 |    296 |      11 |
| riscZeroVerifierPaused                           |           10963 |  10963 |  10963 |  10963 |       5 |
| setKindTreeRoot                                  |             590 |   6982 |   8715 |   8715 |       7 |
| transferOwnership                                |            5469 |   5469 |   5469 |   5469 |       3 |
| unpause                                          |            2343 |   6382 |   8402 |   8402 |       3 |
| upgradeToAndCall                                 |            3022 |   8784 |   3214 |  23956 |       5 |

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

