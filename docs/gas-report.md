| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3933699 |           18447 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             276 |    276 |    276 |    276 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             348 |    348 |    348 |    348 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             659 |    659 |    659 |    659 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             811 |    811 |    811 |    811 |       2 |
| VERSION                                          |            1048 |   1048 |   1048 |   1048 |       3 |
| commitmentCount                                  |             690 |   1690 |   1690 |   2690 |      12 |
| denyLogicRefs                                    |             700 |  31691 |  28838 |  52978 |      11 |
| execute                                          |            2868 | 178893 | 196534 | 400000 |      59 |
| getImplementation                                |             608 |    774 |    608 |   2608 |      12 |
| getKindTreeRoot                                  |            2822 |   2822 |   2822 |   2822 |       2 |
| initialize                                       |            3032 | 150628 | 152867 | 152867 |     125 |
| isCommitmentTreeRootHistorical                   |             891 |    891 |    891 |    891 |       1 |
| isLogicRefDenied                                 |            2414 |   2417 |   2417 |   2421 |      12 |
| isNullifierContained                             |             746 |   2745 |   2746 |   2746 |    2713 |
| latestCommitmentTreeRoot                         |            4833 |   9047 |   7999 |  13260 |      16 |
| migrateNullifierSet                              |             693 |    693 |    693 |    693 |       2 |
| owner                                            |             539 |   2351 |   2539 |   2539 |      32 |
| pause                                            |            2526 |  21609 |  25812 |  25812 |      12 |
| paused                                           |            2369 |   2369 |   2369 |   2369 |      15 |
| proxiableUUID                                    |             259 |    259 |    259 |    259 |      11 |
| riscZeroVerifierPaused                           |           10942 |  10942 |  10942 |  10942 |       5 |
| setKindTreeRoot                                  |             575 |   6967 |   8700 |   8700 |       7 |
| transferOwnership                                |            5450 |   5450 |   5450 |   5450 |       3 |
| unpause                                          |            2328 |   6367 |   8387 |   8387 |       3 |
| upgradeToAndCall                                 |            3007 |   8736 |   3162 |  23889 |       5 |

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

