| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3210847 |           15101 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             285 |    285 |    285 |    285 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             357 |    357 |    357 |    357 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             668 |    668 |    668 |    668 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             820 |    820 |    820 |    820 |       2 |
| VERSION                                          |            1057 |   1057 |   1057 |   1057 |       3 |
| commitmentCount                                  |             699 |   1699 |   1699 |   2699 |      12 |
| denyLogicRefs                                    |             721 |  31742 |  28895 |  53059 |      11 |
| execute                                          |            2890 | 165595 | 190997 | 420597 |      45 |
| getImplementation                                |             617 |    783 |    617 |   2617 |      12 |
| getKindTreeRoot                                  |            2831 |   2831 |   2831 |   2831 |       2 |
| initialize                                       |            3032 | 150354 | 152876 | 152876 |     111 |
| isLogicRefDenied                                 |            2423 |   2426 |   2426 |   2430 |      12 |
| isNullifierContained                             |            2755 |   2755 |   2755 |   2755 |    2712 |
| latestCommitmentTreeRoot                         |            4842 |   9320 |   8533 |  13270 |      16 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             548 |   2360 |   2548 |   2548 |      32 |
| pause                                            |            2526 |  21616 |  25821 |  25821 |      12 |
| paused                                           |            2378 |   2378 |   2378 |   2378 |      15 |
| proxiableUUID                                    |             268 |    268 |    268 |    268 |      11 |
| riscZeroVerifierPaused                           |           10951 |  10951 |  10951 |  10951 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6373 |   8396 |   8396 |       3 |
| upgradeToAndCall                                 |            3016 |   8754 |   3180 |  23916 |       5 |

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
|                                                      787633 |            3495 |       |        |       |         |
|                                                             |                 |       |        |       |         |
| Function Name                                               | Min             | Avg   | Median | Max   | # Calls |
| addCommitment                                               |           12415 | 34432 |  15155 | 96556 |      58 |
| addCommitmentTreeRoot                                       |            2482 | 19440 |  23680 | 23680 |       5 |
| commitmentCount                                             |            2355 |  2355 |   2355 |  2355 |      42 |
| commitmentTreeCapacity                                      |            2452 |  2452 |   2452 |  2452 |       1 |
| commitmentTreeDepth                                         |            2317 |  2317 |   2317 |  2317 |      36 |
| commitmentTreeEmptySubtreeRoots                             |            4971 | 14281 |  16286 | 18549 |      35 |
| commitmentTreeLeftSiblings                                  |            2576 | 11960 |  13891 | 16154 |      34 |
| initialize                                                  |            2522 | 86661 |  93677 | 93677 |      23 |
| isCommitmentTreeRootHistorical                              |            2498 |  2498 |   2498 |  2498 |       4 |
| latestCommitmentTreeRoot                                    |            4485 | 12798 |  12888 | 20263 |      45 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290489 |            1189 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23529 | 38652 |  43693 | 43693 |       4 |
| initialize                                              |            2519 | 18415 |  23650 | 23840 |       4 |
| isNullifierContained                                    |            2360 |  2360 |   2360 |  2360 |       3 |

