| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3251934 |           15291 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             307 |    307 |    307 |    307 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             357 |    357 |    357 |    357 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             646 |    646 |    646 |    646 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             820 |    820 |    820 |    820 |       2 |
| VERSION                                          |            1057 |   1057 |   1057 |   1057 |       3 |
| commitmentCount                                  |             699 |   1699 |   1699 |   2699 |      12 |
| denyLogicRefs                                    |             721 |  31742 |  28895 |  53059 |      11 |
| execute                                          |            2890 | 165591 | 192432 | 421030 |      45 |
| getImplementation                                |             617 |    783 |    617 |   2617 |      12 |
| getKindTreeRoot                                  |            2831 |   2831 |   2831 |   2831 |       2 |
| initialize                                       |            3032 | 150354 | 152876 | 152876 |     111 |
| isLogicRefDenied                                 |            2423 |   2426 |   2426 |   2430 |      12 |
| isNullifierContained                             |            2755 |   2755 |   2755 |   2755 |    2712 |
| latestCommitmentTreeRoot                         |            6953 |  12219 |  10001 |  17729 |      16 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             548 |   2360 |   2548 |   2548 |      32 |
| pause                                            |            2526 |  21616 |  25821 |  25821 |      12 |
| paused                                           |            2400 |   2400 |   2400 |   2400 |      15 |
| proxiableUUID                                    |             290 |    290 |    290 |    290 |      11 |
| riscZeroVerifierPaused                           |           10951 |  10951 |  10951 |  10951 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6373 |   8396 |   8396 |       3 |
| upgradeToAndCall                                 |            3016 |   8772 |   3202 |  23938 |       5 |

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
| addCommitment                                               |           17970 | 43712 |  30998 | 97286 |      65 |
| addCommitmentTreeRoot                                       |            2460 | 19418 |  23658 | 23658 |       5 |
| commitmentCount                                             |            2355 |  2355 |   2355 |  2355 |      42 |
| commitmentTreeCapacity                                      |            2452 |  2452 |   2452 |  2452 |       1 |
| commitmentTreeDepth                                         |            2317 |  2317 |   2317 |  2317 |      36 |
| commitmentTreeSides                                         |            2730 | 12114 |  14045 | 16308 |      34 |
| commitmentTreeZeros                                         |            4839 | 14149 |  16154 | 18417 |      35 |
| initialize                                                  |            2522 | 86953 |  93677 | 93677 |      24 |
| isCommitmentTreeRootHistorical                              |            2498 |  2498 |   2498 |  2498 |       4 |
| latestCommitmentTreeRoot                                    |            6625 | 10526 |   9980 | 14677 |      11 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290489 |            1189 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23529 | 38652 |  43693 | 43693 |       4 |
| initialize                                              |            2519 | 18415 |  23650 | 23840 |       4 |
| isNullifierContained                                    |            2360 |  2360 |   2360 |  2360 |       3 |

