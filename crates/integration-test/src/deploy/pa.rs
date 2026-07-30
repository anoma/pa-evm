//! Protocol adapter deployment. The contract is UUPS-upgradeable, so deploying
//! it takes two steps: an implementation, whose constructor disables the
//! initializers, and an ERC-1967 proxy that delegates to it and holds the state.

use alloy::primitives::Address;
use alloy::primitives::FixedBytes;
use alloy::providers::DynProvider;
use alloy::providers::Provider;
use alloy::sol_types::SolCall;
use anoma_pa_evm_bindings::generated::erc1967_proxy::ERC1967Proxy;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter;
use anyhow::Context;

#[inline]
pub fn protocol_adapter<P>(
    address: Address,
    provider: P,
) -> ProtocolAdapter::ProtocolAdapterInstance<P>
where
    P: Provider,
{
    ProtocolAdapter::ProtocolAdapterInstance::new(address, provider)
}

/// Deploys the implementation, returning its address. It is uninitializable by
/// construction and only usable as a proxy's delegation target.
pub async fn deploy_implementation(
    provider: &DynProvider,
    risc_zero_verifier_router: Address,
    risc_zero_verifier_selector: FixedBytes<4>,
) -> anyhow::Result<Address> {
    let implementation = ProtocolAdapter::deploy(
        provider.clone(),
        risc_zero_verifier_router,
        risc_zero_verifier_selector,
    )
    .await
    .context("failed to deploy the protocol adapter implementation")?;

    Ok(*implementation.address())
}

/// Deploys an ERC-1967 proxy delegating to `implementation_address` and
/// initializes it in the same transaction, mirroring
/// `DeployProtocolAdapterProxy.s.sol`. Initializing through the constructor
/// seeds the commitment tree with its initial root and sets the owner, so the
/// protocol adapter is usable the moment it exists.
pub async fn deploy_proxy(
    provider: &DynProvider,
    implementation_address: Address,
    initial_owner: Address,
) -> anyhow::Result<ProtocolAdapter::ProtocolAdapterInstance<DynProvider>> {
    let initializer_data = ProtocolAdapter::initializeCall {
        initialOwner: initial_owner,
    }
    .abi_encode();

    let proxy = ERC1967Proxy::deploy(
        provider.clone(),
        implementation_address,
        initializer_data.into(),
    )
    .await
    .context("failed to deploy the protocol adapter proxy")?;

    Ok(protocol_adapter(*proxy.address(), provider.clone()))
}
