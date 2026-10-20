| src/ProtocolAdapter.sol:ProtocolAdapter Contract |                 |        |        |        |         |
|--------------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                                  | Deployment Size |        |        |        |         |
|                                          3783727 |           17754 |        |        |        |         |
|                                                  |                 |        |        |        |         |
| Function Name                                    | Min             | Avg    | Median | Max    | # Calls |
| EMPTY_KIND_TREE_ROOT                             |             301 |    301 |    301 |    301 |      10 |
| RISC_ZERO_VERIFIER_ROUTER                        |             351 |    351 |    351 |    351 |       1 |
| RISC_ZERO_VERIFIER_SELECTOR                      |             640 |    640 |    640 |    640 |       2 |
| UPGRADE_INTERFACE_VERSION                        |             814 |    814 |    814 |    814 |       2 |
| VERSION                                          |            1051 |   1051 |   1051 |   1051 |       3 |
| commitmentCount                                  |             693 |   1693 |   1693 |   2693 |      12 |
| denyLogicRefs                                    |             703 |  31694 |  28841 |  52981 |      11 |
| execute                                          |            2871 | 183139 | 198707 | 435848 |      51 |
| getImplementation                                |             611 |    777 |    611 |   2611 |      12 |
| getKindTreeRoot                                  |            2825 |   2825 |   2825 |   2825 |       2 |
| initialize                                       |            3035 | 150484 | 152876 | 152876 |     117 |
| isLogicRefDenied                                 |            2417 |   2420 |   2420 |   2424 |      12 |
| isNullifierContained                             |             749 |   2748 |   2749 |   2749 |    2713 |
| latestCommitmentTreeRoot                         |            6947 |  12338 |   9653 |  17723 |      15 |
| migrateNullifierSet                              |             696 |    696 |    696 |    696 |       2 |
| owner                                            |             542 |   2354 |   2542 |   2542 |      32 |
| pause                                            |            2529 |  21612 |  25815 |  25815 |      12 |
| paused                                           |            2394 |   2394 |   2394 |   2394 |      15 |
| proxiableUUID                                    |             284 |    284 |    284 |    284 |      11 |
| riscZeroVerifierPaused                           |           10951 |  10951 |  10951 |  10951 |       5 |
| setKindTreeRoot                                  |             578 |   6970 |   8703 |   8703 |       7 |
| transferOwnership                                |            5453 |   5453 |   5453 |   5453 |       3 |
| unpause                                          |            2331 |   6370 |   8390 |   8390 |       3 |
| upgradeToAndCall                                 |            3010 |   8760 |   3190 |  23920 |       5 |

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

