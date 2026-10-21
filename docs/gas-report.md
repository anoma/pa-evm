| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3891290 |           18253 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             316 |    316 |    316 |    316 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             366 |    366 |    366 |    366 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             656 |    656 |    656 |    656 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             834 |    834 |    834 |    834 |       2 |
| VERSION                                          |            1073 |   1073 |   1073 |   1073 |       3 |
| commitmentCount                                  |             709 |   1709 |   1709 |   2709 |      12 |
| denyLogicRefs                                    |             718 |  31709 |  28856 |  52996 |      11 |
| execute                                          |            2884 | 183733 | 194642 | 433622 |      58 |
| getImplementation                                |             627 |    793 |    627 |   2627 |      12 |
| getKindTreeRoot                                  |            2841 |   2841 |   2841 |   2841 |       2 |
| initialize                                       |            3052 | 150647 | 152923 | 152923 |     123 |
| isLogicRefDenied                                 |            2432 |   2435 |   2435 |   2439 |      12 |
| isNullifierContained                             |             768 |   2767 |   2768 |   2768 |    2713 |
| latestCommitmentTreeRoot                         |            6977 |  12429 |   9711 |  17877 |      15 |
| migrateNullifierSet                              |             711 |    711 |    711 |    711 |       2 |
| owner                                            |             557 |   2369 |   2557 |   2557 |      32 |
| pause                                            |            2544 |  21627 |  25830 |  25830 |      12 |
| paused                                           |            2409 |   2409 |   2409 |   2409 |      15 |
| proxiableUUID                                    |             299 |    299 |    299 |    299 |      11 |
| riscZeroVerifierPaused                           |           10966 |  10966 |  10966 |  10966 |       5 |
| setKindTreeRoot                                  |             593 |   6985 |   8718 |   8718 |       7 |
| transferOwnership                                |            5472 |   5472 |   5472 |   5472 |       3 |
| unpause                                          |            2346 |   6385 |   8405 |   8405 |       3 |
| upgradeToAndCall                                 |            3025 |   8790 |   3220 |  23965 |       5 |

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

