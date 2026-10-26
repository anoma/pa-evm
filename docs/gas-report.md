| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3985009 |           18690 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             279 |    279 |    279 |    279 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             351 |    351 |    351 |    351 |       2 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             662 |    662 |    662 |    662 |       4 |
| UPGRADE_INTERFACE_VERSION                        |             814 |    814 |    814 |    814 |       2 |
| VERSION                                          |            1051 |   1051 |   1051 |   1051 |       3 |
| commitmentCount                                  |            2693 |   2693 |   2693 |   2693 |      16 |
| denyLogicRefs                                    |             700 |  31694 |  28841 |  52981 |      11 |
| execute                                          |            2868 | 185129 | 196242 | 434923 |      60 |
| getImplementation                                |             611 |    764 |    611 |   2611 |      13 |
| getKindTreeRoot                                  |            2825 |   2825 |   2825 |   2825 |       2 |
| initialize                                       |            3032 | 150631 | 152870 | 152870 |     125 |
| isCommitmentTreeRootHistorical                   |             894 |   2782 |   2894 |   2894 |      18 |
| isLogicRefDenied                                 |            2417 |   2420 |   2420 |   2424 |      12 |
| isNullifierContained                             |             749 |   2748 |   2749 |   2749 |    3388 |
| latestCommitmentTreeRoot                         |            4802 |   8572 |   7975 |  13256 |      17 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             542 |   2354 |   2542 |   2542 |      32 |
| pause                                            |            2526 |  21611 |  25815 |  25815 |      12 |
| paused                                           |            2372 |   2372 |   2372 |   2372 |      16 |
| proxiableUUID                                    |             262 |    262 |    262 |    262 |      15 |
| riscZeroVerifierPaused                           |           10945 |  10945 |  10945 |  10945 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6369 |   8390 |   8390 |       3 |
| upgradeToAndCall                                 |            3010 |   8742 |   3168 |  23898 |       5 |

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
| addCommitment                                               |           12415 | 33894 |  15155 | 96556 |      57 |
| addCommitmentTreeRoot                                       |            2482 | 18380 |  23680 | 23680 |       4 |
| commitmentCount                                             |            2355 |  2355 |   2355 |  2355 |      42 |
| commitmentTreeCapacity                                      |            2452 |  2452 |   2452 |  2452 |       1 |
| commitmentTreeDepth                                         |            2317 |  2317 |   2317 |  2317 |      36 |
| commitmentTreeEmptySubtreeRoots                             |            4971 | 14281 |  16286 | 18549 |      35 |
| commitmentTreeLeftSiblings                                  |            2576 | 11960 |  13891 | 16154 |      34 |
| initialize                                                  |            2522 | 86661 |  93677 | 93677 |      23 |
| isCommitmentTreeRootHistorical                              |            2498 |  2498 |   2498 |  2498 |       4 |
| latestCommitmentTreeRoot                                    |            4485 | 12681 |  12888 | 20263 |      45 |

| test/mocks/NullifierSet.m.sol:NullifierSetMock Contract |                 |       |        |       |         |
|---------------------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                                         | Deployment Size |       |        |       |         |
|                                                  290489 |            1189 |       |        |       |         |
|                                                         |                 |       |        |       |         |
| Function Name                                           | Min             | Avg   | Median | Max   | # Calls |
| addNullifier                                            |           23529 | 38652 |  43693 | 43693 |       4 |
| initialize                                              |            2519 | 18415 |  23650 | 23840 |       4 |
| isNullifierContained                                    |            2360 |  2360 |   2360 |  2360 |       3 |

