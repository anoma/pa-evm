// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

// forge-lint: disable-next-item(literal-instead-of-constant)
/// @title RecordedDeployments
/// @author Anoma Foundation, 2026
/// @notice The protocol adapter deployments each environment records, and the v1 protocol adapters they copy the state
/// from.
/// @dev Generated from `crates/bindings/deployments.json`, the single source of truth, which the bindings crate
/// embeds and checks against the chains. Do not edit by hand: run `just contracts-gen-deployments`, which CI reruns
/// and fails on any diff. The records live with the bindings because that crate publishes them; this library carries
/// them into Solidity so the contracts package reads nothing outside itself.
/// @custom:security-contact security@anoma.foundation
library RecordedDeployments {
    /// @notice A recorded protocol adapter proxy. The field `addr` holds the address, which is a reserved word.
    /// @dev The genesis fields pin how the address was derived: the creation code and the constructor arguments
    /// determine it together with the environment salt, and none of them can be read from the chain once the proxy is
    /// upgraded.
    struct Proxy {
        address addr;
        address initialImplementation;
        bytes initializerData;
        bytes creationCode;
    }

    /// @notice A recorded protocol adapter deployment.
    struct Deployment {
        uint256 chainId;
        Proxy proxy;
    }

    /// @notice A v1 protocol adapter, which a transitional proxy on the same chain copies the state from.
    struct DeploymentV1 {
        uint256 chainId;
        address protocolAdapter;
    }

    /// @notice Returns whether the environment records a deployment for the chain.
    /// @param isProduction Whether to check the production or the staging environment.
    /// @param chainId The chain ID to look for.
    /// @return recorded Whether the environment records a deployment for the chain.
    function isRecorded(bool isProduction, uint256 chainId) internal pure returns (bool recorded) {
        Deployment[] memory deployments = isProduction ? production() : staging();

        for (uint256 i = 0; i < deployments.length; ++i) {
            if (deployments[i].chainId == chainId) {
                return true;
            }
        }
    }

    /// @notice Returns the v1 protocol adapter of a chain.
    /// @param chainId The chain ID to look for.
    /// @return protocolAdapter The v1 protocol adapter, or the zero address if the chain ran none.
    function protocolAdapterV1(uint256 chainId) internal pure returns (address protocolAdapter) {
        DeploymentV1[] memory deployments = v1();

        for (uint256 i = 0; i < deployments.length; ++i) {
            if (deployments[i].chainId == chainId) {
                return deployments[i].protocolAdapter;
            }
        }
    }

    /// @notice Returns the deployments the staging environment records.
    /// @return deployments The recorded staging deployments.
    function staging() internal pure returns (Deployment[] memory deployments) {
        deployments = new Deployment[](0);
    }

    /// @notice Returns the deployments the production environment records.
    /// @return deployments The recorded production deployments.
    function production() internal pure returns (Deployment[] memory deployments) {
        deployments = new Deployment[](0);
    }

    /// @notice Returns the v1 protocol adapters, as `anoma-pa-evm-bindings` 2.3.0 records them in `addresses.rs`.
    /// @return deployments The recorded v1 deployments.
    function v1() internal pure returns (DeploymentV1[] memory deployments) {
        deployments = new DeploymentV1[](11);
        deployments[0] = DeploymentV1({chainId: 11155111, protocolAdapter: 0xf152BBA809d6cba122579cee997A54B8F3FBa417});
        deployments[1] = DeploymentV1({chainId: 1, protocolAdapter: 0x0eA3B55b68A3f307c8FE3fe66E443247c95F0CfF});
        deployments[2] = DeploymentV1({chainId: 84532, protocolAdapter: 0x094FCC095323080e71a037b2B1e3519c07dd84F8});
        deployments[3] = DeploymentV1({chainId: 8453, protocolAdapter: 0x094FCC095323080e71a037b2B1e3519c07dd84F8});
        deployments[4] = DeploymentV1({chainId: 10, protocolAdapter: 0x094FCC095323080e71a037b2B1e3519c07dd84F8});
        deployments[5] = DeploymentV1({chainId: 42161, protocolAdapter: 0x094FCC095323080e71a037b2B1e3519c07dd84F8});
        deployments[6] = DeploymentV1({chainId: 56, protocolAdapter: 0xFC44b66a39fe6923Ad8d3c93bFeC369728862B68});
        deployments[7] = DeploymentV1({chainId: 143, protocolAdapter: 0x2D2Fa19aFdbb20DC73737ca5f075cfAE00Cd90C2});
        deployments[8] = DeploymentV1({chainId: 988, protocolAdapter: 0x2D2Fa19aFdbb20DC73737ca5f075cfAE00Cd90C2});
        deployments[9] = DeploymentV1({chainId: 4326, protocolAdapter: 0x2D2Fa19aFdbb20DC73737ca5f075cfAE00Cd90C2});
        deployments[10] =
            DeploymentV1({chainId: 1313161554, protocolAdapter: 0x2D2Fa19aFdbb20DC73737ca5f075cfAE00Cd90C2});
    }
}
