| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3414423 |           16047 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TABLE_ROOT                            |             602 |    602 |    602 |    602 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             363 |    363 |    363 |    363 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             674 |    674 |    674 |    674 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             804 |    804 |    804 |    804 |       2 |
| VERSION                                          |            1063 |   1063 |   1063 |   1063 |       3 |
| commitmentCount                                  |             727 |   1727 |   1727 |   2727 |      12 |
| denyLogicRefs                                    |             725 |  31727 |  28875 |  53015 |      11 |
| execute                                          |            2849 | 157043 | 170958 | 421303 |      48 |
| getImplementation                                |             601 |    767 |    601 |   2601 |      12 |
| getKindTableRoot                                 |            2639 |   2639 |   2639 |   2639 |       2 |
| initialize                                       |            3057 | 150455 | 152910 | 152910 |     114 |
| isLogicRefDenied                                 |            2451 |   2454 |   2454 |   2458 |      12 |
| isNullifierContained                             |            2761 |   2761 |   2761 |   2761 |    2712 |
| latestCommitmentTreeRoot                         |            6959 |  12350 |   9665 |  17736 |      15 |
| migrateNullifierSet                              |             696 |    696 |    696 |    696 |       2 |
| owner                                            |             532 |   2344 |   2532 |   2532 |      32 |
| pause                                            |            2529 |  21622 |  25827 |  25827 |      12 |
| paused                                           |            2428 |   2428 |   2428 |   2428 |      15 |
| proxiableUUID                                    |             318 |    318 |    318 |    318 |      11 |
| riscZeroVerifierPaused                           |           10963 |  10963 |  10963 |  10963 |       5 |
| setKindTableRoot                                 |             314 |   6706 |   8439 |   8439 |       7 |
| transferOwnership                                |            5475 |   5475 |   5475 |   5475 |       3 |
| unpause                                          |            2353 |   6400 |   8424 |   8424 |       3 |
| upgradeToAndCall                                 |            3044 |   8823 |   3258 |  24000 |       5 |

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

