| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4011716 |           18812 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             304 |    304 |    304 |    304 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             354 |    354 |    354 |    354 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             643 |    643 |    643 |    643 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             817 |    817 |    817 |    817 |       2 |
| VERSION                                          |            1054 |   1054 |   1054 |   1054 |       3 |
| commitmentCount                                  |             696 |   1696 |   1696 |   2696 |      12 |
| denyLogicRefs                                    |             703 |  31697 |  28844 |  52984 |      11 |
| execute                                          |            2871 | 189017 | 196568 | 434000 |      59 |
| getImplementation                                |             614 |    780 |    614 |   2614 |      12 |
| getKindTreeRoot                                  |            2828 |   2828 |   2828 |   2828 |       2 |
| initialize                                       |            3035 | 150640 | 152879 | 152879 |     125 |
| isCommitmentTreeRootHistorical                   |             897 |    897 |    897 |    897 |       1 |
| isLogicRefDenied                                 |            2420 |   2423 |   2423 |   2427 |      12 |
| isNullifierContained                             |             752 |   2751 |   2752 |   2752 |    2713 |
| latestCommitmentTreeRoot                         |            6950 |  12004 |   9656 |  17726 |      16 |
| migrateNullifierSet                              |             696 |    696 |    696 |    696 |       2 |
| owner                                            |             545 |   2357 |   2545 |   2545 |      32 |
| pause                                            |            2529 |  21614 |  25818 |  25818 |      12 |
| paused                                           |            2397 |   2397 |   2397 |   2397 |      15 |
| proxiableUUID                                    |             287 |    287 |    287 |    287 |      11 |
| riscZeroVerifierPaused                           |           10954 |  10954 |  10954 |  10954 |       5 |
| setKindTreeRoot                                  |             578 |   6970 |   8703 |   8703 |       7 |
| transferOwnership                                |            5453 |   5453 |   5453 |   5453 |       3 |
| unpause                                          |            2331 |   6372 |   8393 |   8393 |       3 |
| upgradeToAndCall                                 |            3013 |   8766 |   3196 |  23929 |       5 |

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
| addCommitment                                               |           17973 | 43371 |  31001 | 97289 |      64 |
| addCommitmentTreeRoot                                       |            2463 | 18361 |  23661 | 23661 |       4 |
| commitmentCount                                             |            2358 |  2358 |   2358 |  2358 |      42 |
| commitmentTreeCapacity                                      |            2455 |  2455 |   2455 |  2455 |       1 |
| commitmentTreeDepth                                         |            2320 |  2320 |   2320 |  2320 |      36 |
| commitmentTreeSides                                         |            2733 | 12117 |  14048 | 16311 |      34 |
| commitmentTreeZeros                                         |            4842 | 14152 |  16157 | 18420 |      35 |
| initialize                                                  |            2525 | 86956 |  93680 | 93680 |      24 |
| isCommitmentTreeRootHistorical                              |            2501 |  2501 |   2501 |  2501 |       4 |
| latestCommitmentTreeRoot                                    |            6628 | 10224 |   9983 | 14680 |      11 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290705 |            1190 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23532 | 38655 |  43696 | 43696 |       4 |
| initialize                                              |            2522 | 18418 |  23653 | 23843 |       4 |
| isNullifierContained                                    |            2363 |  2363 |   2363 |  2363 |       3 |

