| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3246882 |           15269 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             282 |    282 |    282 |    282 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             354 |    354 |    354 |    354 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             665 |    665 |    665 |    665 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             817 |    817 |    817 |    817 |       2 |
| VERSION                                          |            1054 |   1054 |   1054 |   1054 |       3 |
| commitmentCount                                  |             696 |   1696 |   1696 |   2696 |      12 |
| denyLogicRefs                                    |             700 |  31696 |  28844 |  52984 |      11 |
| execute                                          |            2880 | 151075 | 168691 | 419403 |      41 |
| getImplementation                                |             614 |    780 |    614 |   2614 |      12 |
| getKindTreeRoot                                  |            2828 |   2828 |   2828 |   2828 |       2 |
| initialize                                       |            3032 | 150257 | 152873 | 152873 |     107 |
| isLogicRefDenied                                 |            2420 |   2423 |   2423 |   2427 |      12 |
| isNullifierContained                             |            2752 |   2752 |   2752 |   2752 |    2712 |
| latestCommitmentTreeRoot                         |            4839 |   9335 |   8005 |  13267 |      15 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             545 |   2357 |   2545 |   2545 |      32 |
| pause                                            |            2526 |  21614 |  25818 |  25818 |      12 |
| paused                                           |            2375 |   2375 |   2375 |   2375 |      15 |
| proxiableUUID                                    |             265 |    265 |    265 |    265 |      11 |
| riscZeroVerifierPaused                           |           10948 |  10948 |  10948 |  10948 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6371 |   8393 |   8393 |       3 |
| upgradeToAndCall                                 |            3013 |   8748 |   3174 |  23907 |       5 |

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
| addCommitment                                               |           12415 | 32775 |  15155 | 96556 |      56 |
| addCommitmentTreeRoot                                       |            2482 | 18380 |  23680 | 23680 |       4 |
| commitmentCount                                             |            2355 |  2355 |   2355 |  2355 |      42 |
| commitmentTreeCapacity                                      |            2452 |  2452 |   2452 |  2452 |       1 |
| commitmentTreeDepth                                         |            2317 |  2317 |   2317 |  2317 |      36 |
| commitmentTreeEmptySubtreeRoots                             |            4971 | 14281 |  16286 | 18549 |      35 |
| commitmentTreeLeftSiblings                                  |            2576 | 11960 |  13891 | 16154 |      34 |
| initialize                                                  |            2522 | 86342 |  93677 | 93677 |      22 |
| isCommitmentTreeRootHistorical                              |            2498 |  2498 |   2498 |  2498 |       4 |
| latestCommitmentTreeRoot                                    |            4485 | 12989 |  13933 | 20263 |      43 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290489 |            1189 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23529 | 38652 |  43693 | 43693 |       4 |
| initialize                                              |            2519 | 18415 |  23650 | 23840 |       4 |
| isNullifierContained                                    |            2360 |  2360 |   2360 |  2360 |       3 |

