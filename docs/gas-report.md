| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3973324 |           18636 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             301 |    301 |    301 |    301 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             351 |    351 |    351 |    351 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             640 |    640 |    640 |    640 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             814 |    814 |    814 |    814 |       2 |
| VERSION                                          |            1051 |   1051 |   1051 |   1051 |       3 |
| commitmentCount                                  |             693 |   1550 |    693 |   2693 |      14 |
| denyLogicRefs                                    |             700 |  31694 |  28841 |  52981 |      11 |
| execute                                          |            2868 | 186086 | 196556 | 433894 |      59 |
| getImplementation                                |             611 |    736 |    611 |   2611 |      16 |
| getKindTreeRoot                                  |            2825 |   2825 |   2825 |   2825 |       2 |
| initialize                                       |            3032 | 150631 | 152870 | 152870 |     125 |
| isCommitmentTreeRootHistorical                   |             894 |    894 |    894 |    894 |       1 |
| isLogicRefDenied                                 |            2417 |   2420 |   2420 |   2424 |      12 |
| isNullifierContained                             |             749 |   2748 |   2749 |   2749 |    3163 |
| latestCommitmentTreeRoot                         |            6947 |  11439 |   9653 |  17723 |      18 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             542 |   2360 |   2542 |   2542 |      33 |
| pause                                            |            2526 |  21611 |  25815 |  25815 |      12 |
| paused                                           |            2394 |   2394 |   2394 |   2394 |      17 |
| proxiableUUID                                    |             284 |    284 |    284 |    284 |      13 |
| riscZeroVerifierPaused                           |           10945 |  10945 |  10945 |  10945 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6369 |   8390 |   8390 |       3 |
| upgradeToAndCall                                 |            3010 |   8760 |   3190 |  23920 |       5 |

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

