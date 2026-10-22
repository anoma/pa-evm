// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ForwarderExample} from "anoma-forwarder-bases-3.1.0/test/examples/ForwarderExample.sol";
import {
    ForwarderTargetExample,
    _encodedDefaultInput,
    EXPECTED_OUTPUT
} from "anoma-forwarder-bases-3.1.0/test/examples/ForwarderTargetExample.sol";
import {DeployRiscZeroContractsMock} from "anoma-risc0-deployments-1.2.5/test/script/DeployRiscZeroContractsMock.s.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";
import {Options} from "openzeppelin-foundry-upgrades-0.4.2/src/Options.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";
import {
    RiscZeroVerifierEmergencyStop
} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierEmergencyStop.sol";
import {RiscZeroVerifierRouter} from "risc0-risc0-ethereum-3.0.1/contracts/src/RiscZeroVerifierRouter.sol";
import {RiscZeroMockVerifier} from "risc0-risc0-ethereum-3.0.1/contracts/src/test/RiscZeroMockVerifier.sol";

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {ProtocolAdapter} from "../../src/ProtocolAdapter.sol";
import {TxGen} from "../libs/TxGen.sol";

/// @notice A test fixture providing a protocol adapter with the RISC Zero mock verifier and a forwarder example.
abstract contract ProtocolAdapterMockVerifierFixture is Test {
    address internal constant _OWNER = address(uint160(1));
    bytes32 internal constant _RESOURCE_LOGIC_REF = bytes32(uint256(123));

    RiscZeroVerifierRouter internal _router;
    RiscZeroMockVerifier internal _mockVerifier;
    RiscZeroVerifierEmergencyStop internal _emergencyStop;
    ProtocolAdapter internal _mockPa;
    address internal _fwd;
    address internal _fwdTarget;
    address[] internal _fwdList;

    bytes internal _input;

    bytes32 internal _resourceLabelRef;

    /// @notice Deploys the RISC Zero mock contracts, the protocol adapter behind a proxy, and a forwarder example.
    function setUp() public {
        (_router, _emergencyStop, _mockVerifier) = new DeployRiscZeroContractsMock().run();

        Options memory opts;
        opts.constructorData = abi.encode(_router, _mockVerifier.SELECTOR());

        _mockPa = ProtocolAdapter(
            Upgrades.deployUUPSProxy("ProtocolAdapter.sol", abi.encodeCall(ProtocolAdapter.initialize, (_OWNER)), opts)
        );

        _fwd = address(new ForwarderExample({protocolAdapter: address(_mockPa), logicRef: _RESOURCE_LOGIC_REF}));
        _fwdTarget = address(new ForwarderTargetExample());
        _input = _encodedDefaultInput(_fwdTarget);

        _fwdList = new address[](1);
        _fwdList[0] = _fwd;

        _resourceLabelRef = sha256(abi.encode(_fwd));
    }

    /// @notice Builds a resource without forwarder calls.
    function _exampleResourceAndEmptyAppData(uint256 nonce)
        internal
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        data = new TxGen.ResourceAndAppData[](1);

        data[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(nonce), logicRef: _RESOURCE_LOGIC_REF, labelRef: _resourceLabelRef, quantity: 1
            }),
            appData: TxGen.emptyAppData()
        });
    }

    /// @notice Builds a resource that carries one forwarder call to each forwarder of `fwdList`.
    function _exampleCallCarryingResourceAndAppData(uint256 nonce, address[] memory fwdList)
        internal
        view
        returns (TxGen.ResourceAndAppData[] memory data)
    {
        data = new TxGen.ResourceAndAppData[](1);
        uint256 nCalls = fwdList.length;

        data[0] = TxGen.ResourceAndAppData({
            resource: TxGen.mockResource({
                nonce: bytes32(nonce), logicRef: _RESOURCE_LOGIC_REF, labelRef: _resourceLabelRef, quantity: 1
            }),
            appData: IProtocolAdapter.AppData({
                discoveryPayload: new IProtocolAdapter.ExpirableBlob[](0),
                resourcePayload: new IProtocolAdapter.ExpirableBlob[](0),
                externalPayload: new IProtocolAdapter.ExpirableBlob[](nCalls),
                applicationPayload: new IProtocolAdapter.ExpirableBlob[](0)
            })
        });

        IProtocolAdapter.ExpirableBlob[] memory externalBlobs = new IProtocolAdapter.ExpirableBlob[](nCalls);
        for (uint256 i = 0; i < nCalls; ++i) {
            externalBlobs[i] = IProtocolAdapter.ExpirableBlob({
                deletionCriterion: IProtocolAdapter.DeletionCriterion.Never,
                blob: _callBlob({
                    untrustedForwarder: address(fwdList[i]), input: _input, expectedOutput: EXPECTED_OUTPUT
                })
            });
        }
        data[0].appData.externalPayload = externalBlobs;
    }

    /// @notice Encodes an unconstrained forwarder call blob.
    function _callBlob(address untrustedForwarder, bytes memory input, bytes memory expectedOutput)
        internal
        pure
        returns (bytes memory blob)
    {
        blob = _callBlob({
            untrustedForwarder: untrustedForwarder,
            input: input,
            expectedOutput: expectedOutput,
            afterTags: new bytes32[](0),
            beforeTags: new bytes32[](0)
        });
    }

    /// @notice Encodes a forwarder call blob carrying ordering constraints.
    function _callBlob(
        address untrustedForwarder,
        bytes memory input,
        bytes memory expectedOutput,
        bytes32[] memory afterTags,
        bytes32[] memory beforeTags
    ) internal pure returns (bytes memory blob) {
        blob = abi.encode(untrustedForwarder, input, expectedOutput, afterTags, beforeTags);
    }
}
