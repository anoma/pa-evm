use alloy::primitives::Address;
use alloy::providers::Provider;
use anoma_pa_evm_bindings::generated::protocol_adapter::ProtocolAdapter;

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
