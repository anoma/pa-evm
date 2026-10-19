| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3298911 |           15510 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             313 |    313 |    313 |    313 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             363 |    363 |    363 |    363 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             652 |    652 |    652 |    652 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             826 |    826 |    826 |    826 |       2 |
| VERSION                                          |            1063 |   1063 |   1063 |   1063 |       3 |
| commitmentCount                                  |             705 |   1705 |   1705 |   2705 |      12 |
| denyLogicRefs                                    |             703 |  31705 |  28853 |  52993 |      11 |
| execute                                          |            2871 | 151227 | 169571 | 420098 |      41 |
| getImplementation                                |             623 |    789 |    623 |   2623 |      12 |
| getKindTreeRoot                                  |            2837 |   2837 |   2837 |   2837 |       2 |
| initialize                                       |            3035 | 150272 | 152888 | 152888 |     107 |
| isLogicRefDenied                                 |            2429 |   2432 |   2432 |   2436 |      12 |
| isNullifierContained                             |            2761 |   2761 |   2761 |   2761 |    2712 |
| latestCommitmentTreeRoot                         |            6959 |  12350 |   9665 |  17736 |      15 |
| migrateNullifierSet                              |             696 |    696 |    696 |    696 |       2 |
| owner                                            |             554 |   2366 |   2554 |   2554 |      32 |
| pause                                            |            2529 |  21622 |  25827 |  25827 |      12 |
| paused                                           |            2406 |   2406 |   2406 |   2406 |      15 |
| proxiableUUID                                    |             296 |    296 |    296 |    296 |      11 |
| riscZeroVerifierPaused                           |           10963 |  10963 |  10963 |  10963 |       5 |
| setKindTreeRoot                                  |             578 |   6970 |   8703 |   8703 |       7 |
| transferOwnership                                |            5453 |   5453 |   5453 |   5453 |       3 |
| unpause                                          |            2331 |   6378 |   8402 |   8402 |       3 |
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

