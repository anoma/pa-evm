| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          4147522 |           19470 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TABLE_ROOT                            |             640 |    640 |    640 |    640 |      11 |
| RISC_ZERO_VERIFIER_ROUTER                        |             378 |    378 |    378 |    378 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             712 |    712 |    712 |    712 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             846 |    846 |    846 |    846 |       2 |
| VERSION                                          |            1107 |   1107 |   1107 |   1107 |       3 |
| commitmentCount                                  |             765 |   1765 |   1765 |   2765 |      12 |
| denyLogicRefs                                    |             752 |  31505 |  28890 |  53030 |      12 |
| execute                                          |            2874 | 131433 | 163030 | 328293 |      35 |
| getImplementation                                |             639 |    805 |    639 |   2639 |      12 |
| getKindTableRoot                                 |            2677 |   2677 |   2677 |   2677 |       2 |
| initialize                                       |            3108 | 150457 | 152979 | 152979 |     111 |
| isLogicRefDenied                                 |            2490 |   2493 |   2493 |   2497 |      12 |
| isNullifierContained                             |            2802 |   2802 |   2802 |   2802 |    2712 |
| latestCommitmentTreeRoot                         |            7011 |  12463 |   9745 |  17912 |      15 |
| migrateNullifierSet                              |             745 |    745 |    745 |    745 |       2 |
| owner                                            |             547 |   2359 |   2547 |   2547 |      32 |
| pause                                            |            2556 |  21639 |  25842 |  25842 |      12 |
| paused                                           |            2443 |   2443 |   2443 |   2443 |      15 |
| proxiableUUID                                    |             333 |    333 |    333 |    333 |      11 |
| riscZeroVerifierPaused                           |           10978 |  10978 |  10978 |  10978 |       5 |
| setKindTableRoot                                 |             341 |   6733 |   8466 |   8466 |       7 |
| simulateExecute                                  |            3194 | 184860 | 176012 | 401357 |      10 |
| transferOwnership                                |            5528 |   5528 |   5528 |   5528 |       3 |
| unpause                                          |            2380 |   6419 |   8439 |   8439 |       3 |
| upgradeToAndCall                                 |            3059 |   8853 |   3288 |  24045 |       5 |

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

