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

    /// @notice A v1 protocol adapter, which a migrational proxy on the same chain copies the state from.
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
        deployments = new Deployment[](1);
        deployments[0] = Deployment({
            chainId: 11155111,
            proxy: Proxy({
                addr: 0xca62980d4dbb3a6368C575061c824a902017c7ac,
                initialImplementation: 0x346e7735fC15bFCbD3E896a8c3E661CE2625EB17,
                initializerData: hex"c4d66de800000000000000000000000061462be56782568376f9cb069382efa72764a407",
                creationCode: hex"6080604052610284803803806100148161016e565b9283398101604082820312610156578151916001600160a01b03831690818403610156576020810151906001600160401b038211610156570182601f82011215610156578051906001600160401b03821161015a5761007c601f8301601f191660200161016e565b938285526020838301011161015657815f9260208093018387015e8401015281511561014757823b15610135577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc80546001600160a01b031916821790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a280511561011e5761010e91610193565b505b604051606490816102208239f35b505034156101105763b398979f60e01b5f5260045ffd5b634c9c8ce360e01b5f5260045260245ffd5b6330a289cf60e21b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f191682016001600160401b0381118382101761015a57604052565b905f8091602081519101845af4808061020c575b156101c75750506040513d81523d5f602083013e60203d82010160405290565b156101ec57639996b31560e01b5f9081526001600160a01b0391909116600452602490fd5b3d156101fd576040513d5f823e3d90fd5b63d6bda27560e01b5f5260045ffd5b503d1515806101a75750813b15156101a756fe60806040525f8073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416368280378136915af43d5f803e156053573d5ff35b3d5ffdfea164736f6c6343000824000a"
            })
        });
    }

    /// @notice Returns the deployments the production environment records.
    /// @return deployments The recorded production deployments.
    function production() internal pure returns (Deployment[] memory deployments) {
        deployments = new Deployment[](0);
    }

    /// @notice Returns the v1 protocol adapters that `anoma-pa-evm-bindings` 2.3.0 records in `addresses.rs`, except
    /// Aurora, which is deprecated.
    /// @return deployments The recorded v1 deployments.
    function v1() internal pure returns (DeploymentV1[] memory deployments) {
        deployments = new DeploymentV1[](10);
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
    }
}
