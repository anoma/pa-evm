| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3742050 |           17567 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             304 |    304 |    304 |    304 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             354 |    354 |    354 |    354 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             643 |    643 |    643 |    643 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             817 |    817 |    817 |    817 |       2 |
| VERSION                                          |            1054 |   1054 |   1054 |   1054 |       3 |
| commitmentCount                                  |             696 |   1696 |   1696 |   2696 |      12 |
| denyLogicRefs                                    |             700 |  31696 |  28844 |  52984 |      11 |
| execute                                          |            2880 | 181999 | 190724 | 424298 |      61 |
| getImplementation                                |             614 |    780 |    614 |   2614 |      12 |
| getKindTreeRoot                                  |            2828 |   2828 |   2828 |   2828 |       2 |
| initialize                                       |            3032 | 150652 | 152873 | 152873 |     126 |
| isCommitmentTreeRootHistorical                   |             897 |    897 |    897 |    897 |       1 |
| isLogicRefDenied                                 |            2420 |   2423 |   2423 |   2427 |      12 |
| isNullifierContained                             |             752 |   2751 |   2752 |   2752 |    2713 |
| latestCommitmentTreeRoot                         |            6950 |  12004 |   9656 |  17726 |      16 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             545 |   2357 |   2545 |   2545 |      32 |
| pause                                            |            2526 |  21614 |  25818 |  25818 |      12 |
| paused                                           |            2397 |   2397 |   2397 |   2397 |      15 |
| proxiableUUID                                    |             287 |    287 |    287 |    287 |      11 |
| riscZeroVerifierPaused                           |           10948 |  10948 |  10948 |  10948 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6371 |   8393 |   8393 |       3 |
| upgradeToAndCall                                 |            3013 |   8766 |   3196 |  23929 |       5 |

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

