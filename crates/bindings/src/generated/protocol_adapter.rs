///Module containing a contract's types and functions.
/**

```solidity
library ILogicRefPolicyRegistry {
    type LogicRefPolicy is uint8;
    struct PolicyUpdate { bytes32 logicRef; LogicRefPolicy policy; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ILogicRefPolicyRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LogicRefPolicy(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<LogicRefPolicy> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl LogicRefPolicy {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(LogicRefPolicy);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for LogicRefPolicy {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<LogicRefPolicy> for u8 {
            fn from(value: LogicRefPolicy) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for LogicRefPolicy {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for LogicRefPolicy {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PolicyUpdate { bytes32 logicRef; LogicRefPolicy policy; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PolicyUpdate {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub policy: <LogicRefPolicy as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            LogicRefPolicy,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <LogicRefPolicy as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PolicyUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: PolicyUpdate) -> Self {
                (value.logicRef, value.policy)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PolicyUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    logicRef: tuple.0,
                    policy: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PolicyUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PolicyUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                    <LogicRefPolicy as alloy_sol_types::SolType>::tokenize(&self.policy),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PolicyUpdate {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PolicyUpdate {
            const NAME: &'static str = "PolicyUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PolicyUpdate(bytes32 logicRef,uint8 policy)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.logicRef)
                        .0,
                    <LogicRefPolicy as alloy_sol_types::SolType>::eip712_data_word(
                            &self.policy,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PolicyUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.logicRef,
                    )
                    + <LogicRefPolicy as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.policy,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.logicRef,
                    out,
                );
                <LogicRefPolicy as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.policy,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ILogicRefPolicyRegistry`](self) contract instance.

See the [wrapper's documentation](`ILogicRefPolicyRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ILogicRefPolicyRegistryInstance<P, N> {
        ILogicRefPolicyRegistryInstance::<P, N>::new(address, __provider)
    }
    /**A [`ILogicRefPolicyRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ILogicRefPolicyRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ILogicRefPolicyRegistryInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ILogicRefPolicyRegistryInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ILogicRefPolicyRegistryInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ILogicRefPolicyRegistryInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ILogicRefPolicyRegistry`](self) contract instance.

See the [wrapper's documentation](`ILogicRefPolicyRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ILogicRefPolicyRegistryInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ILogicRefPolicyRegistryInstance<P, N> {
            ILogicRefPolicyRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ILogicRefPolicyRegistryInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ILogicRefPolicyRegistryInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IProtocolAdapter {
    type DeletionCriterion is uint8;
    struct Action { Consumed[] consumed; Created[] created; Delta unitDelta; bytes32 actionTreeRoot; }
    struct AppData { ExpirableBlob[] resourcePayload; ExpirableBlob[] discoveryPayload; ExpirableBlob[] externalPayload; ExpirableBlob[] applicationPayload; }
    struct Consumed { bytes32 nullifier; bytes32 logicRef; bytes32 commitmentTreeRoot; AppData appData; }
    struct Created { bytes32 commitment; bytes32 logicRef; AppData appData; }
    struct Delta { uint256 x; uint256 y; }
    struct ExpirableBlob { DeletionCriterion deletionCriterion; bytes blob; }
    struct Transaction { Action[] actions; bytes deltaProof; bytes aggregationProof; bytes32 kindTableCommitment; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IProtocolAdapter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeletionCriterion(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<DeletionCriterion> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl DeletionCriterion {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(DeletionCriterion);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for DeletionCriterion {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<DeletionCriterion> for u8 {
            fn from(value: DeletionCriterion) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DeletionCriterion {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeletionCriterion {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Action { Consumed[] consumed; Created[] created; Delta unitDelta; bytes32 actionTreeRoot; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Action {
        #[allow(missing_docs)]
        pub consumed: alloy::sol_types::private::Vec<
            <Consumed as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub created: alloy::sol_types::private::Vec<
            <Created as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub unitDelta: <Delta as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub actionTreeRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<Consumed>,
            alloy::sol_types::sol_data::Array<Created>,
            Delta,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <Consumed as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <Created as alloy::sol_types::SolType>::RustType,
            >,
            <Delta as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Action> for UnderlyingRustTuple<'_> {
            fn from(value: Action) -> Self {
                (value.consumed, value.created, value.unitDelta, value.actionTreeRoot)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Action {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    consumed: tuple.0,
                    created: tuple.1,
                    unitDelta: tuple.2,
                    actionTreeRoot: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Action {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Action {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        Consumed,
                    > as alloy_sol_types::SolType>::tokenize(&self.consumed),
                    <alloy::sol_types::sol_data::Array<
                        Created,
                    > as alloy_sol_types::SolType>::tokenize(&self.created),
                    <Delta as alloy_sol_types::SolType>::tokenize(&self.unitDelta),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionTreeRoot),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Action {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Action {
            const NAME: &'static str = "Action";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Action(Consumed[] consumed,Created[] created,Delta unitDelta,bytes32 actionTreeRoot)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components
                    .push(<Consumed as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Consumed as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(<Created as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Created as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(<Delta as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Delta as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        Consumed,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.consumed)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        Created,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.created)
                        .0,
                    <Delta as alloy_sol_types::SolType>::eip712_data_word(
                            &self.unitDelta,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.actionTreeRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Action {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        Consumed,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.consumed,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        Created,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.created,
                    )
                    + <Delta as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unitDelta,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.actionTreeRoot,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    Consumed,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.consumed,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    Created,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.created,
                    out,
                );
                <Delta as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unitDelta,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.actionTreeRoot,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct AppData { ExpirableBlob[] resourcePayload; ExpirableBlob[] discoveryPayload; ExpirableBlob[] externalPayload; ExpirableBlob[] applicationPayload; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AppData {
        #[allow(missing_docs)]
        pub resourcePayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub discoveryPayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub externalPayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub applicationPayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AppData> for UnderlyingRustTuple<'_> {
            fn from(value: AppData) -> Self {
                (
                    value.resourcePayload,
                    value.discoveryPayload,
                    value.externalPayload,
                    value.applicationPayload,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AppData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    resourcePayload: tuple.0,
                    discoveryPayload: tuple.1,
                    externalPayload: tuple.2,
                    applicationPayload: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AppData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AppData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.resourcePayload),
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.discoveryPayload),
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.externalPayload),
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.applicationPayload),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for AppData {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for AppData {
            const NAME: &'static str = "AppData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AppData(ExpirableBlob[] resourcePayload,ExpirableBlob[] discoveryPayload,ExpirableBlob[] externalPayload,ExpirableBlob[] applicationPayload)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.resourcePayload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.discoveryPayload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.externalPayload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.applicationPayload,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AppData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.resourcePayload,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.discoveryPayload,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.externalPayload,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.applicationPayload,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.resourcePayload,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.discoveryPayload,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.externalPayload,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.applicationPayload,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Consumed { bytes32 nullifier; bytes32 logicRef; bytes32 commitmentTreeRoot; AppData appData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Consumed {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub commitmentTreeRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub appData: <AppData as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            AppData,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            <AppData as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Consumed> for UnderlyingRustTuple<'_> {
            fn from(value: Consumed) -> Self {
                (
                    value.nullifier,
                    value.logicRef,
                    value.commitmentTreeRoot,
                    value.appData,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Consumed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nullifier: tuple.0,
                    logicRef: tuple.1,
                    commitmentTreeRoot: tuple.2,
                    appData: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Consumed {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Consumed {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifier),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitmentTreeRoot),
                    <AppData as alloy_sol_types::SolType>::tokenize(&self.appData),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Consumed {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Consumed {
            const NAME: &'static str = "Consumed";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Consumed(bytes32 nullifier,bytes32 logicRef,bytes32 commitmentTreeRoot,AppData appData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<AppData as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <AppData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nullifier)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.logicRef)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.commitmentTreeRoot,
                        )
                        .0,
                    <AppData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.appData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Consumed {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nullifier,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.logicRef,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitmentTreeRoot,
                    )
                    + <AppData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.appData,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nullifier,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.logicRef,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitmentTreeRoot,
                    out,
                );
                <AppData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.appData,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Created { bytes32 commitment; bytes32 logicRef; AppData appData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Created {
        #[allow(missing_docs)]
        pub commitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub appData: <AppData as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            AppData,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            <AppData as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Created> for UnderlyingRustTuple<'_> {
            fn from(value: Created) -> Self {
                (value.commitment, value.logicRef, value.appData)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Created {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    commitment: tuple.0,
                    logicRef: tuple.1,
                    appData: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Created {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Created {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitment),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                    <AppData as alloy_sol_types::SolType>::tokenize(&self.appData),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Created {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Created {
            const NAME: &'static str = "Created";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Created(bytes32 commitment,bytes32 logicRef,AppData appData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<AppData as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <AppData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.commitment)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.logicRef)
                        .0,
                    <AppData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.appData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Created {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitment,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.logicRef,
                    )
                    + <AppData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.appData,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitment,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.logicRef,
                    out,
                );
                <AppData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.appData,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Delta { uint256 x; uint256 y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Delta {
        #[allow(missing_docs)]
        pub x: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Delta> for UnderlyingRustTuple<'_> {
            fn from(value: Delta) -> Self {
                (value.x, value.y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Delta {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { x: tuple.0, y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Delta {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Delta {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Delta {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Delta {
            const NAME: &'static str = "Delta";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Delta(uint256 x,uint256 y)")
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Delta {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y, out);
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ExpirableBlob { DeletionCriterion deletionCriterion; bytes blob; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpirableBlob {
        #[allow(missing_docs)]
        pub deletionCriterion: <DeletionCriterion as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            DeletionCriterion,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <DeletionCriterion as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpirableBlob> for UnderlyingRustTuple<'_> {
            fn from(value: ExpirableBlob) -> Self {
                (value.deletionCriterion, value.blob)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpirableBlob {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    deletionCriterion: tuple.0,
                    blob: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ExpirableBlob {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ExpirableBlob {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <DeletionCriterion as alloy_sol_types::SolType>::tokenize(
                        &self.deletionCriterion,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ExpirableBlob {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ExpirableBlob {
            const NAME: &'static str = "ExpirableBlob";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ExpirableBlob(uint8 deletionCriterion,bytes blob)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <DeletionCriterion as alloy_sol_types::SolType>::eip712_data_word(
                            &self.deletionCriterion,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.blob,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ExpirableBlob {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <DeletionCriterion as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deletionCriterion,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blob,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <DeletionCriterion as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deletionCriterion,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blob,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Transaction { Action[] actions; bytes deltaProof; bytes aggregationProof; bytes32 kindTableCommitment; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Transaction {
        #[allow(missing_docs)]
        pub actions: alloy::sol_types::private::Vec<
            <Action as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub deltaProof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub aggregationProof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kindTableCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<Action>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <Action as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Transaction> for UnderlyingRustTuple<'_> {
            fn from(value: Transaction) -> Self {
                (
                    value.actions,
                    value.deltaProof,
                    value.aggregationProof,
                    value.kindTableCommitment,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Transaction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    actions: tuple.0,
                    deltaProof: tuple.1,
                    aggregationProof: tuple.2,
                    kindTableCommitment: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Transaction {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Transaction {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        Action,
                    > as alloy_sol_types::SolType>::tokenize(&self.actions),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.deltaProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.aggregationProof,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.kindTableCommitment),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Transaction {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Transaction {
            const NAME: &'static str = "Transaction";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Transaction(Action[] actions,bytes deltaProof,bytes aggregationProof,bytes32 kindTableCommitment)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Action as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Action as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        Action,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.actions)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.deltaProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aggregationProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kindTableCommitment,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Transaction {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        Action,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.actions,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deltaProof,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aggregationProof,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kindTableCommitment,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    Action,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.actions,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deltaProof,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aggregationProof,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kindTableCommitment,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IProtocolAdapter`](self) contract instance.

See the [wrapper's documentation](`IProtocolAdapterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IProtocolAdapterInstance<P, N> {
        IProtocolAdapterInstance::<P, N>::new(address, __provider)
    }
    /**A [`IProtocolAdapter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IProtocolAdapter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IProtocolAdapterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IProtocolAdapterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IProtocolAdapterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolAdapterInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IProtocolAdapter`](self) contract instance.

See the [wrapper's documentation](`IProtocolAdapterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IProtocolAdapterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IProtocolAdapterInstance<P, N> {
            IProtocolAdapterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolAdapterInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IProtocolAdapterInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ILogicRefPolicyRegistry {
    type LogicRefPolicy is uint8;
    struct PolicyUpdate {
        bytes32 logicRef;
        LogicRefPolicy policy;
    }
}

library IProtocolAdapter {
    type DeletionCriterion is uint8;
    struct Action {
        Consumed[] consumed;
        Created[] created;
        Delta unitDelta;
        bytes32 actionTreeRoot;
    }
    struct AppData {
        ExpirableBlob[] resourcePayload;
        ExpirableBlob[] discoveryPayload;
        ExpirableBlob[] externalPayload;
        ExpirableBlob[] applicationPayload;
    }
    struct Consumed {
        bytes32 nullifier;
        bytes32 logicRef;
        bytes32 commitmentTreeRoot;
        AppData appData;
    }
    struct Created {
        bytes32 commitment;
        bytes32 logicRef;
        AppData appData;
    }
    struct Delta {
        uint256 x;
        uint256 y;
    }
    struct ExpirableBlob {
        DeletionCriterion deletionCriterion;
        bytes blob;
    }
    struct Transaction {
        Action[] actions;
        bytes deltaProof;
        bytes aggregationProof;
        bytes32 kindTableCommitment;
    }
}

interface ProtocolAdapter {
    error AddressEmptyCode(address target);
    error DeltaMismatch(address expected, address actual);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyTransactionNotAllowed();
    error EnforcedPause();
    error ExpectedPause();
    error FailedCall();
    error ForwarderCallOutputMismatch(bytes expected, bytes actual);
    error InvalidInitialization();
    error InvalidLogicRefPolicyTransition(bytes32 logicRef, ILogicRefPolicyRegistry.LogicRefPolicy previousPolicy, ILogicRefPolicyRegistry.LogicRefPolicy newPolicy);
    error LogicRefPoliciesNotInitialized();
    error NonExistingRoot(bytes32 root);
    error NotInitializing();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error PointNotOnCurve(IProtocolAdapter.Delta point);
    error PreExistingNullifier(bytes32 nullifier);
    error PreExistingRoot(bytes32 root);
    error ReentrancyGuardReentrantCall();
    error ResourceWithDeniedLogicRef(bytes32 logicRef, bool consumed);
    error RiscZeroVerifierPaused();
    error RiscZeroVerifierSelectorMismatch(bytes4 expected, bytes4 actual);
    error Simulated(uint256 gasUsed);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnacceptedKindTableCommitment(bytes32 kindTableCommitment);
    error ZeroKindTableCommitmentNotAllowed();
    error ZeroLogicRefNotAllowed();
    error ZeroRiscZeroVerifierRouterNotAllowed();
    error ZeroRiscZeroVerifierSelectorNotAllowed();

    event ActionExecuted(bytes32 actionTreeRoot, bytes32[] nullifiers, bytes32[] consumedLogicRefs, bytes32[] commitments, bytes32[] createdLogicRefs);
    event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event CommitmentTreeRootAdded(bytes32 root);
    event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);
    event Initialized(uint64 version);
    event KindTableCommitmentUpdated(bytes32 indexed kindTableCommitment);
    event LogicRefPolicyChanged(bytes32 indexed logicRef, ILogicRefPolicyRegistry.LogicRefPolicy previousPolicy, ILogicRefPolicyRegistry.LogicRefPolicy newPolicy);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address account);
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
    event TransactionExecuted(bytes32 indexed transactionId);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);

    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector);

    function EMPTY_KIND_TABLE_COMMITMENT() external view returns (bytes32);
    function RISC_ZERO_VERIFIER_ROUTER() external view returns (address);
    function RISC_ZERO_VERIFIER_SELECTOR() external view returns (bytes4);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function VERSION() external view returns (string memory);
    function commitmentCount() external view returns (uint256 count);
    function commitmentTreeCapacity() external view returns (uint256 capacity);
    function commitmentTreeDepth() external view returns (uint8 depth);
    function commitmentTreeRootAtIndex(uint256 index) external view returns (bytes32 root);
    function commitmentTreeRootCount() external view returns (uint256 count);
    function commitmentTreeSides() external view returns (bytes32[] memory sides);
    function commitmentTreeZeros() external view returns (bytes32[] memory zeros);
    function execute(IProtocolAdapter.Transaction memory transaction) external;
    function getImplementation() external view returns (address current);
    function getKindTableCommitment() external view returns (bytes32 kindTableCommitment);
    function initialize(address initialOwner) external;
    function initializeLogicRefPolicies() external;
    function isCommitmentTreeRootContained(bytes32 root) external view returns (bool isContained);
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
    function latestCommitmentTreeRoot() external view returns (bytes32 root);
    function logicRefPolicy(bytes32 logicRef) external view returns (ILogicRefPolicyRegistry.LogicRefPolicy policy);
    function nullifierAtIndex(uint256 index) external view returns (bytes32 nullifier);
    function nullifierCount() external view returns (uint256 count);
    function owner() external view returns (address);
    function pause() external;
    function paused() external view returns (bool isPaused);
    function proxiableUUID() external view returns (bytes32);
    function renounceOwnership() external;
    function restrictedLogicRefAtIndex(uint256 index) external view returns (bytes32 logicRef);
    function restrictedLogicRefCount() external view returns (uint256 count);
    function riscZeroVerifierPaused() external view returns (bool isPaused);
    function setKindTableCommitment(bytes32 newKindTableCommitment) external;
    function setLogicRefPolicies(ILogicRefPolicyRegistry.PolicyUpdate[] memory updates) external;
    function simulateExecute(IProtocolAdapter.Transaction memory transaction, bool skipRiscZeroProofVerification) external;
    function transferOwnership(address newOwner) external;
    function unpause() external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "riscZeroVerifierRouter",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "riscZeroVerifierSelector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "EMPTY_KIND_TABLE_COMMITMENT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "RISC_ZERO_VERIFIER_ROUTER",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "RISC_ZERO_VERIFIER_SELECTOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UPGRADE_INTERFACE_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentTreeCapacity",
    "inputs": [],
    "outputs": [
      {
        "name": "capacity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentTreeDepth",
    "inputs": [],
    "outputs": [
      {
        "name": "depth",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentTreeRootAtIndex",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentTreeRootCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentTreeSides",
    "inputs": [],
    "outputs": [
      {
        "name": "sides",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentTreeZeros",
    "inputs": [],
    "outputs": [
      {
        "name": "zeros",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execute",
    "inputs": [
      {
        "name": "transaction",
        "type": "tuple",
        "internalType": "struct IProtocolAdapter.Transaction",
        "components": [
          {
            "name": "actions",
            "type": "tuple[]",
            "internalType": "struct IProtocolAdapter.Action[]",
            "components": [
              {
                "name": "consumed",
                "type": "tuple[]",
                "internalType": "struct IProtocolAdapter.Consumed[]",
                "components": [
                  {
                    "name": "nullifier",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "logicRef",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "commitmentTreeRoot",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "appData",
                    "type": "tuple",
                    "internalType": "struct IProtocolAdapter.AppData",
                    "components": [
                      {
                        "name": "resourcePayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "discoveryPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "externalPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "applicationPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "created",
                "type": "tuple[]",
                "internalType": "struct IProtocolAdapter.Created[]",
                "components": [
                  {
                    "name": "commitment",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "logicRef",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "appData",
                    "type": "tuple",
                    "internalType": "struct IProtocolAdapter.AppData",
                    "components": [
                      {
                        "name": "resourcePayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "discoveryPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "externalPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "applicationPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "unitDelta",
                "type": "tuple",
                "internalType": "struct IProtocolAdapter.Delta",
                "components": [
                  {
                    "name": "x",
                    "type": "uint256",
                    "internalType": "uint256"
                  },
                  {
                    "name": "y",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "actionTreeRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "deltaProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "aggregationProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "kindTableCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "current",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKindTableCommitment",
    "inputs": [],
    "outputs": [
      {
        "name": "kindTableCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initializeLogicRefPolicies",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isCommitmentTreeRootContained",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "isContained",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isNullifierContained",
    "inputs": [
      {
        "name": "nullifier",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "isContained",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "latestCommitmentTreeRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "logicRefPolicy",
    "inputs": [
      {
        "name": "logicRef",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "policy",
        "type": "uint8",
        "internalType": "enum ILogicRefPolicyRegistry.LogicRefPolicy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nullifierAtIndex",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "nullifier",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nullifierCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "isPaused",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proxiableUUID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "restrictedLogicRefAtIndex",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "logicRef",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "restrictedLogicRefCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "riscZeroVerifierPaused",
    "inputs": [],
    "outputs": [
      {
        "name": "isPaused",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setKindTableCommitment",
    "inputs": [
      {
        "name": "newKindTableCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setLogicRefPolicies",
    "inputs": [
      {
        "name": "updates",
        "type": "tuple[]",
        "internalType": "struct ILogicRefPolicyRegistry.PolicyUpdate[]",
        "components": [
          {
            "name": "logicRef",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "policy",
            "type": "uint8",
            "internalType": "enum ILogicRefPolicyRegistry.LogicRefPolicy"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "simulateExecute",
    "inputs": [
      {
        "name": "transaction",
        "type": "tuple",
        "internalType": "struct IProtocolAdapter.Transaction",
        "components": [
          {
            "name": "actions",
            "type": "tuple[]",
            "internalType": "struct IProtocolAdapter.Action[]",
            "components": [
              {
                "name": "consumed",
                "type": "tuple[]",
                "internalType": "struct IProtocolAdapter.Consumed[]",
                "components": [
                  {
                    "name": "nullifier",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "logicRef",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "commitmentTreeRoot",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "appData",
                    "type": "tuple",
                    "internalType": "struct IProtocolAdapter.AppData",
                    "components": [
                      {
                        "name": "resourcePayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "discoveryPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "externalPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "applicationPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "created",
                "type": "tuple[]",
                "internalType": "struct IProtocolAdapter.Created[]",
                "components": [
                  {
                    "name": "commitment",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "logicRef",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "appData",
                    "type": "tuple",
                    "internalType": "struct IProtocolAdapter.AppData",
                    "components": [
                      {
                        "name": "resourcePayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "discoveryPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "externalPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "applicationPayload",
                        "type": "tuple[]",
                        "internalType": "struct IProtocolAdapter.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IProtocolAdapter.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "unitDelta",
                "type": "tuple",
                "internalType": "struct IProtocolAdapter.Delta",
                "components": [
                  {
                    "name": "x",
                    "type": "uint256",
                    "internalType": "uint256"
                  },
                  {
                    "name": "y",
                    "type": "uint256",
                    "internalType": "uint256"
                  }
                ]
              },
              {
                "name": "actionTreeRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "deltaProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "aggregationProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "kindTableCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "skipRiscZeroProofVerification",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgradeToAndCall",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "event",
    "name": "ActionExecuted",
    "inputs": [
      {
        "name": "actionTreeRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "nullifiers",
        "type": "bytes32[]",
        "indexed": false,
        "internalType": "bytes32[]"
      },
      {
        "name": "consumedLogicRefs",
        "type": "bytes32[]",
        "indexed": false,
        "internalType": "bytes32[]"
      },
      {
        "name": "commitments",
        "type": "bytes32[]",
        "indexed": false,
        "internalType": "bytes32[]"
      },
      {
        "name": "createdLogicRefs",
        "type": "bytes32[]",
        "indexed": false,
        "internalType": "bytes32[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ApplicationPayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CommitmentTreeRootAdded",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DiscoveryPayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExternalPayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ForwarderCallExecuted",
    "inputs": [
      {
        "name": "untrustedForwarder",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "input",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "output",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KindTableCommitmentUpdated",
    "inputs": [
      {
        "name": "kindTableCommitment",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LogicRefPolicyChanged",
    "inputs": [
      {
        "name": "logicRef",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousPolicy",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum ILogicRefPolicyRegistry.LogicRefPolicy"
      },
      {
        "name": "newPolicy",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum ILogicRefPolicyRegistry.LogicRefPolicy"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ResourcePayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionExecuted",
    "inputs": [
      {
        "name": "transactionId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AddressEmptyCode",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "DeltaMismatch",
    "inputs": [
      {
        "name": "expected",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "actual",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967InvalidImplementation",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967NonPayable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyTransactionNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ForwarderCallOutputMismatch",
    "inputs": [
      {
        "name": "expected",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "actual",
        "type": "bytes",
        "internalType": "bytes"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLogicRefPolicyTransition",
    "inputs": [
      {
        "name": "logicRef",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "previousPolicy",
        "type": "uint8",
        "internalType": "enum ILogicRefPolicyRegistry.LogicRefPolicy"
      },
      {
        "name": "newPolicy",
        "type": "uint8",
        "internalType": "enum ILogicRefPolicyRegistry.LogicRefPolicy"
      }
    ]
  },
  {
    "type": "error",
    "name": "LogicRefPoliciesNotInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonExistingRoot",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "PointNotOnCurve",
    "inputs": [
      {
        "name": "point",
        "type": "tuple",
        "internalType": "struct IProtocolAdapter.Delta",
        "components": [
          {
            "name": "x",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ]
  },
  {
    "type": "error",
    "name": "PreExistingNullifier",
    "inputs": [
      {
        "name": "nullifier",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "PreExistingRoot",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ReentrancyGuardReentrantCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ResourceWithDeniedLogicRef",
    "inputs": [
      {
        "name": "logicRef",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "consumed",
        "type": "bool",
        "internalType": "bool"
      }
    ]
  },
  {
    "type": "error",
    "name": "RiscZeroVerifierPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RiscZeroVerifierSelectorMismatch",
    "inputs": [
      {
        "name": "expected",
        "type": "bytes4",
        "internalType": "bytes4"
      },
      {
        "name": "actual",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ]
  },
  {
    "type": "error",
    "name": "Simulated",
    "inputs": [
      {
        "name": "gasUsed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "UUPSUnauthorizedCallContext",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnsupportedProxiableUUID",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnacceptedKindTableCommitment",
    "inputs": [
      {
        "name": "kindTableCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroKindTableCommitmentNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroLogicRefNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroRiscZeroVerifierRouterNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroRiscZeroVerifierSelectorNotAllowed",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ProtocolAdapter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e03461011e57601f61557238819003918201601f19168301916001600160401b0383118484101761012257808492604094855283398101031261011e5780516001600160a01b0381169182820361011e5760200151916001600160e01b031983169081840361011e5730608052610075610136565b61007d610136565b610085610136565b61008d610136565b1561010f57156101005760a05260c0526100a5610136565b60405161538590816101cd8239608051818181612d260152612e7e015260a0518181816111eb01528181611ffe01528181612c440152613921015260c05181818161042f0152818161109101528181611fbe01526138df0152f35b63b1b25aaf60e01b5f5260045ffd5b63536c150160e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f5160206155525f395f51905f525460ff8160401c166101bd576002600160401b03196001600160401b0382160161016b5750565b6001600160401b0319166001600160401b039081175f5160206155525f395f51905f52556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a1565b63f92ee8a960e01b5f5260045ffdfe6101606040526004361015610012575f80fd5b5f610120525f3560e01c806331ee62421461336c578063328c2433146131fb5780633f4ba83a1461313e57806340f34d42146131025780634f1ef28614612e3257806351945b0614612d9e57806352d1902d14612cff57806359ba925814612cc35780635c975abb14612c825780636735312214612c685780636bf0a04b14612c18578063715018a614612b5c5780638456cb5914612ac55780638da5cb5b14612a735780638ee0a26114612a2f5780639239ac0c14611d225780639ad91d4c14611c9e578063a06056f714611c5c578063a600458b14610dc7578063aaf10f4214610d72578063ad3cb1cc14610d0f578063bdeb442d14610c82578063c025302314610bde578063c1b0bed714610b88578063c37d757a14610aca578063c44956d114610a8b578063c4d66de814610594578063c75f304d146104e3578063c879dbe414610490578063e1c5095814610453578063e35a5d2f146103f4578063e5edfd2214610391578063ed83cdc7146102d8578063f2fde38b146102a8578063fe18ab9114610263578063ffa1ad74146101fc5763ffc33f72146101b6575f80fd5b346101f557610120516003193601126101f55760207f54765cac2cb330b12e843854496f3e05fcf80a7a08eec821362177619185190054604051908152f35b6101205180fd5b346101f557610120516003193601126101f5576040805161025f9161022190826133f6565b600a81527f322e302e302d72632e380000000000000000000000000000000000000000000060208201526040519182916020835260208301906134bc565b0390f35b346101f557610120516003193601126101f5576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b346101f55760206003193601126101f5576102d16102c461338a565b6102cc613ed7565b613aba565b6101205180f35b346101f557610120516003193601126101f55761012051506040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90161012051526020610120512090610120515b81811061037b5761025f85610367818703826133f6565b604051918291602083526020830190613489565b8254845260209093019260019283019201610350565b346101f55760206003193601126101f5576103aa613f43565b60043561012051527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052602060ff604061012051205416604051906103f0816134e1565b8152f35b346101f557610120516003193601126101f55760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101f557610120516003193601126101f55760206040517fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8558152f35b346101f55760206003193601126101f55760206104d96004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b6040519015158152f35b346101f55760206003193601126101f55760043567ffffffffffffffff81116101f557366023820112156101f557806004013567ffffffffffffffff81116101f5576024820191602436918360061b0101116101f557610541613ed7565b610549613f43565b610120515b81811061055c576101205180f35b610567818385613aaa565b35906020610576828587613aaa565b01359160048310156101f55760019261058e91613d68565b0161054e565b346101f55760206003193601126101f5576105ad61338a565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549067ffffffffffffffff60ff8360401c1615921680159081610a83575b6001149081610a79575b159081610a70575b50610a425761068e908260017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000007ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005416177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00556109cc575b61067e614137565b610686614137565b6102cc614137565b610696614137565b61069e614137565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156109995780600161072192017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902613a95565b81549060031b905f197fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06831b921b1916179055610120517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005561078261505a565b507f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a16107d5614137565b6107dd614137565b600160ff197f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3025416177f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30255610830614137565b7fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f54765cac2cb330b12e843854496f3e05fcf80a7a08eec8213621776191851900557fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26108c6613896565b61096b576108d5576101205180f35b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a16102d1565b7f25ae6eaa000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526041600452602461012051fd5b680100000000000000007fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005416177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610676565b7ff92ee8a9000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b905015836105fe565b303b1591506105f6565b8391506105ec565b346101f557610120516003193601126101f55760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b346101f55760206003193601126101f557600435610ae6613f43565b7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30154811015610b55576020907f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3016101205152816101205120016101205150546101205160031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526032600452602461012051fd5b346101f55760206003193601126101f557610120515060043561012051527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc01602052602060406101205120541515604051908152f35b346101f55760206003193601126101f557600435610bfa613ed7565b8015610c5457807f54765cac2cb330b12e843854496f3e05fcf80a7a08eec8213621776191851900557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26101205180f35b7f774d5de1000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b346101f557610120516003193601126101f5577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903545f198101908111610cdc57610ccd602091613a3f565b90549060031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526011600452602461012051fd5b346101f557610120516003193601126101f5576040805161025f91610d3490826133f6565b600581527f352e302e3000000000000000000000000000000000000000000000000000000060208201526040519182916020835260208301906134bc565b346101f557610120516003193601126101f557602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b346101f55760406003193601126101f55767ffffffffffffffff600435116101f5576080600319600435360301126101f55760243560c05260c051151560c051036101f5575a60e0527f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c611c2e5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d610e62614048565b610e6a613f43565b610e78600480350180613f97565b80915015611c00577f54765cac2cb330b12e843854496f3e05fcf80a7a08eec82136217761918519005460646004350135148015611bd1575b15611b9957610ebf81614003565b608052610eca61409b565b50604051610ed7816133ad565b610120518152610120516020820152906101205161010052610120515b8181106115495782610f3e6080515160051b6020608051012091610f35610f2f610f286024600435016004356004016140e6565b3691613435565b846151e8565b90939193615222565b6020815191015190610120515260205273ffffffffffffffffffffffffffffffffffffffff8060406101205120169116908082036115155782610f85600480350180613f97565b6040929192519080608083017f406c60f87a5bb542a7fc7301ba5c01fe7724b5b3c9e335214d092a10b405f5a060208501526064600435013560408501526060808501525260a082019060a08160051b84010194809261012051915b8383106112f55786602087610fff818c03601f1981018352826133f6565b8160405191805191829101835e810190610120518252806101205192039060025afa156112b357610120515161103f6044600435016004356004016140e6565b806004939293116101f5577fffffffff000000000000000000000000000000000000000000000000000000008335167fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168082036112c157505060c051156111d4575b5050506101005161115c575b7f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d61112b5a60e051613a32565b7f6f149831000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b61116861010051614faa565b1561119f577f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d26020604051610100518152a16110cf565b7fdb788c2b00000000000000000000000000000000000000000000000000000000610120515261010051600452602461012051fd5b73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b156101f55761124f6040519485937fab750e7500000000000000000000000000000000000000000000000000000000855260606004860152606485019161458b565b927f858be23ecbd24b70efdfacada11c2f0471f33982e33758b8861e5d462576dc46602484015260448301528180610120519403915afa80156112b357611298575b80806110c3565b610120516112a5916133f6565b610120516101f55781611291565b6040513d61012051823e3d90fd5b7f78a2221c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b90919293967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608682030183528735907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61833603018212156101f55760a0810191839061136381830180614ad8565b809560a086525260c0840160c08660051b8601019582610120515b82811061148e57505050505061139b602083830101838301614ad8565b94908482036020860152858252602082019060208760051b84010196819361012051927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b8381106114285750505050505091016040818101359084015260608082013590840152608090810135920191909152509660209081019493600101920190610fe1565b91939599909294969750601f198482030187528935868112156101f557602061147a600193606061146c888596018035845285810135868501526040810190614b2b565b918160408201520190614c4c565b9b0197019101918a979695939194926113e5565b9193976001919395965060206115038260806114f56114d58e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f889903018d528a614b2b565b803584528581013586850152604081013560408501526060810190614b2b565b918160608201520190614c4c565b9901950191019188959493919261137e565b7fe6d44b4c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b611557600480350180613f97565b9060a052811015610b55578060051b60a0510135927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6160a0513603018412156101f5576101205160a05185906115af90820180613f97565b90506115ba81614003565b6115c382614003565b91610120515b818110611a5657505060a0516115e59084016020810190613f97565b9290506115f183614003565b916115fb84614003565b9361012051905b8082106117915750509261167a7f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32959361166c6116889461165e608098604051998a9960a051010135895260a060208a015260a0890190613489565b908782036040890152613489565b908582036060870152613489565b908382036080850152613489565b0390a180611787575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08560a05101360301126101f557604051906116cf826133ad565b60408560a05101013582526020820160608660a0510101358152604051926116f6846133ad565b5f84525f602085015261170c81518351906147e6565b1561174f5782600195949261172e926020608096519301519051915192614848565b602083015281529460a05101013561174882608051614034565b5201610ef4565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b6101005284611691565b91929394956117ae91975060209060a05101018b60a05101613f97565b8791971015610b55578060051b8701357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1883603018112156101f5576117f96020828a0101356142c7565b60ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f198114610cdc57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005590828a013591610120515b82811061199557505091602091600194937f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548660ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b1461193b575b50996119126119096040848401016119016118f6828787016140b3565b878787010135614378565b8484016140b3565b838301356145c2565b81810135611920858b614034565b5201013561192e8289614034565b52018a9594939291611602565b61198a61198361198f9261194e85613ba7565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9026101205152866101205120015480946151a5565b92806151a5565b613c40565b8f6118d9565b9092600190818516611a1757611a0c907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901610120515280846020610120512001557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902610120515283602061012051200154906151a5565b935b811c9101611879565b611a50907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9016101205152836020610120512001546151a5565b93611a0e565b90919293611a6c9060a051018a60a05101613f97565b821015610b5557611a83908260051b8101906140b3565b611a906020820135614255565b6040810135611ac9815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b15611b685750611ad98135614ef5565b15611b365790602082611b11611b0a606060019601611b04611afb82866140b3565b86860135614378565b836140b3565b82356145c2565b8035611b1d8488614034565b520135611b2a8287614034565b520190889392916115c9565b7f39a940c500000000000000000000000000000000000000000000000000000000610120515235600452602461012051fd5b7ff9849ea3000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b7fd6d8891b00000000000000000000000000000000000000000000000000000000610120515260646004350135600452602461012051fd5b507fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8556064600435013514610eb1565b7fcf1b093c000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b346101f557610120516003193601126101f557602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b346101f55760206003193601126101f5577f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc005460043590811015610b55576020907f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc006101205152816101205120016101205150546101205160031b1c604051908152f35b346121935760206003193601126121935767ffffffffffffffff6004351161219357608060031960043536030112612193577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c612a075760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d611da6614048565b611dae613f43565b611dbc600480350180613f97565b809150156129df577f54765cac2cb330b12e843854496f3e05fcf80a7a08eec821362177619185190054606460043501351480156129b0575b1561297e57611e0381614003565b90611e0c61409b565b50604051611e19816133ad565b5f81525f6020820152915f915f5b818110612394575050806020611e5f925160051b91012092610f35611e59610f286024600435016004356004016140e6565b856151e8565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f20169116908082036123665750611ea29050600480350180613f97565b60405190602082019281608084017f406c60f87a5bb542a7fc7301ba5c01fe7724b5b3c9e335214d092a10b405f5a086526064600435013560408601526060808601525260a0830160a08360051b85010192825f907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603015b8383106121c5578a8a60205f8c8c611f3e818e03601f1981018352826133f6565b604051918291518091835e8101838152039060025afa15612188575f51611f6f6044600435016004356004016140e6565b80600411612193577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000001680820361219757505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15612193575f9261206992604051958694859384937fab750e7500000000000000000000000000000000000000000000000000000000855260606004860152606485019161458b565b907f858be23ecbd24b70efdfacada11c2f0471f33982e33758b8861e5d462576dc466024840152604483015203915afa801561218857612173575b5080612103575b507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d6101205180f35b61210c81614faa565b156121425760207f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d291604051908152a1816120ab565b7fdb788c2b000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b5f61217d916133f6565b5f61012052826120a4565b6040513d5f823e3d90fd5b5f80fd5b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9091929394957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608882030186528635828112156121935783019060a081019161220e8180614ad8565b809460a085525260c0830160c08560051b85010194825f5b82811061231557505050505061223f6020820182614ad8565b93908382036020850152848252602082019060208660051b8401019581935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b8381106122c557505050506040808501359086015250505060608082013590830152608090810135910152956020908101950193926001019190611f1d565b90919293949598601f19848203018752893586811215612193576020612306600193606061146c888596018035845285810135868501526040810190614b2b565b9b019701959493929101612286565b90919293966020806123598360806114f56114d58e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f60019a03018d528a614b2b565b9901950193929101612226565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b6123a2600480350180613f97565b82101561283c578160051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff618136030182121561219357015f956123ea8280613f97565b9790506123f688614003565b9761240081614003565b905f5b8181106128695750506124196020850185613f97565b905061242481614003565b6101405261243181614003565b905f905b808210612567575050987f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32916124ae61249d9b61167a60808901359d8e9461248f604051978897885260a0602089015260a0880190613489565b908682036040880152613489565b848103606086015261014051613489565b0390a18061255f575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0833603011261219357604051916124f1836133ad565b60408101358352606060208401910135815260405192612510846133ad565b5f84525f602085015261252681518351906147e6565b1561174f5791612546916001959493602083519301519051915192614848565b60208301528152956125588286614034565b5201611e27565b9550876124b7565b9093506125776020870187613f97565b859195101561283c578060051b8501357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa186360301811215612193576125c2602082880101356142c7565b60ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f19811461280f57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900559082880135915f5b82811061272b57505091602091600194937f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548660ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b146126e3575b50976126bc6119096040848401016119016118f6828787016140b3565b818101356126cd8561014051614034565b520101356126db8286614034565b520190612435565b61198a611983612725926126f685613ba7565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f52865f20015480946151a5565b5f61269f565b90926001908185166127b8577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301546127ad916151a5565b935b811c910161263f565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83015461280991906151a5565b936127af565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6128738680613f97565b82101561283c5761288a908260051b8101906140b3565b60208101359061289982614255565b60408101356128d2815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b1561295357506128e28135614ef5565b1561292757908c612915848461290e611b0a60606001999801611b0461290882866140b3565b89614378565b3592614034565b526129208286614034565b5201612403565b7f39a940c5000000000000000000000000000000000000000000000000000000005f523560045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fd6d8891b000000000000000000000000000000000000000000000000000000005f526064600435013560045260245ffd5b507fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8556064600435013514611df5565b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34612193575f60031936011261219357612a47613f43565b60207f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30154604051908152f35b34612193575f60031936011261219357602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b34612193575f60031936011261219357612add613ed7565b612ae5614048565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b34612193575f60031936011261219357612b74613ed7565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34612193575f60031936011261219357602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34612193575f6003193601126121935760206104d9613896565b34612193575f60031936011261219357602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b34612193575f6003193601126121935760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354604051908152f35b34612193575f6003193601126121935773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163003612d765760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b34612193575f600319360112612193576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f20905f5b818110612e1c5761025f85610367818703826133f6565b8254845260209093019260019283019201612e05565b604060031936011261219357612e4661338a565b60243567ffffffffffffffff811161219357612e6690369060040161346b565b9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168030149081156130c0575b50612d7657612eb6613ed7565b73ffffffffffffffffffffffffffffffffffffffff8116916040517f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f918161308c575b50612f3657837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8592036130615750823b1561303657807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2805115613005576130039161418e565b005b50503461300e57005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d6020116130b8575b816130a8602093836133f6565b8101031261219357519085612f05565b3d915061309b565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416141583612ea9565b34612193575f6003193601126121935760207f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054604051908152f35b34612193575f60031936011261219357613156613ed7565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff8116156131d35760ff19167fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b34612193575f600319360112612193577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c16908115613356575b5061332e577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0080547fffffffffffffffffffffffffffffffffffffffffffffff000000000000000000166801000000000000000217905561329b613518565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160028152a1005b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002915067ffffffffffffffff1610158161323c565b34612193576020600319360112612193576020610ccd600435613a3f565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361219357565b6040810190811067ffffffffffffffff8211176133c957604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff8211176133c957604052565b67ffffffffffffffff81116133c957601f01601f191660200190565b92919261344182613419565b9161344f60405193846133f6565b829481845281830111612193578281602093845f960137010152565b9080601f830112156121935781602061348693359101613435565b90565b90602080835192838152019201905f5b8181106134a65750505090565b8251845260209384019390920191600101613499565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b600411156134eb57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b613520613ed7565b60ff7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3025416613894577f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900545f5b8181106137f75750507f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62902545f5b8181106135f0575050600160ff197f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3025416177f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30255565b7f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629025481101561283c577f9769a3c2a2ce2c693e1c889e63a7e62f739218367bbc5aa6faed32d6a7cfb3e98101545f8181527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052604090205490919060ff16613679816134e1565b15613689575b600191500161359a565b81156137cf57815f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30060205260405f20549160ff8316926136ca846134e1565b836001141590816137af575b5015613772575f8181527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30060205260409020600160ff1982541617905561371c836134e1565b8215613764575b60405192613730816134e1565b83527f1470948b32386703e3756d8e14d51fad3baf8f25d8248c248df7af5360e380c56040846001602081970152a261367f565b61376d81613cc3565b613723565b7fee80efae000000000000000000000000000000000000000000000000000000005f52600452506137a2816134e1565b602452600160445260645ffd5b9050600184916137be836134e1565b6137c7836134e1565b16145f6136d6565b7f7af62341000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629005481101561283c577fb2f9f85961e18e7e2d332eaa0571b488c6acba1ca66a6b2cd2d41964cac670f88101545f8181527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62903602052604090205460019291613886911561388c57600390613d68565b0161356d565b600290613d68565b565b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115612188575f916139e2575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa908115612188575f916139aa575090565b90506020813d6020116139da575b816139c5602093836133f6565b81010312612193575180151581036121935790565b3d91506139b8565b90506020813d602011613a2a575b816139fd602093836133f6565b81010312612193575173ffffffffffffffffffffffffffffffffffffffff81168103612193576020613951565b3d91506139f0565b9190820391821161280f57565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9035481101561283c577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9035f5260205f2001905f90565b805482101561283c575f5260205f2001905f90565b919081101561283c5760061b0190565b73ffffffffffffffffffffffffffffffffffffffff168015613b7b5773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154680100000000000000008110156133c957806001613c2a92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901613a95565b5f19829392549160031b92831b921b1916179055565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156133c957806001613c2a92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902613a95565b7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30154680100000000000000008110156133c957806001613c2a92017f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e301557f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e301613a95565b8054680100000000000000008110156133c957613c2a91600182018155613a95565b9081156137cf57815f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30060205260405f20549060ff821691613daa836134e1565b613db3826134e1565b82808314159182613eac575b505015613e68577f1470948b32386703e3756d8e14d51fad3baf8f25d8248c248df7af5360e380c591604091845f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052825f20613e1e826134e1565b60ff1981541660ff8316179055613e34826134e1565b8115613e5a575b825191613e47816134e1565b8252613e52816134e1565b6020820152a2565b613e6385613cc3565b613e3b565b917fee80efae000000000000000000000000000000000000000000000000000000005f52600452613e98816134e1565b602452613ea4816134e1565b60445260645ffd5b60ff919250613eba846134e1565b613ec3836134e1565b613ecc836134e1565b83161614825f613dbf565b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054163303613f1757565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e302541615613f6f57565b7f27a41336000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215612193570180359067ffffffffffffffff821161219357602001918160051b3603831361219357565b67ffffffffffffffff81116133c95760051b60200190565b9061400d82613feb565b61401a60405191826133f6565b828152601f1961402a8294613feb565b0190602036910137565b805182101561283c5760209160051b010190565b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541661407357565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b604051906140a8826133ad565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215612193570190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215612193570180359067ffffffffffffffff82116121935760200191813603831361219357565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c161561416657565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b905f8091602081519101845af48080614242575b156141c25750506040513d81523d5f602083013e60203d82010160405290565b156142095773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d1561421a576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d1515806141a25750813b15156141a2565b805f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052600260405f205461428f60ff82166134e1565b166142975750565b7f597abd20000000000000000000000000000000000000000000000000000000005f52600452600160245260445ffd5b805f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052600160405f205461430160ff82166134e1565b166143095750565b7f597abd20000000000000000000000000000000000000000000000000000000005f526004525f60245260445ffd5b919081101561283c5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215612193570190565b90604081016143878183613f97565b9290505f5b83811061439a575050505050565b6143bb6143b1826143ab8686613f97565b90614338565b60208101906140e6565b81016060828203126121935781359173ffffffffffffffffffffffffffffffffffffffff831680930361219357602081013567ffffffffffffffff8111612193578261440891830161346b565b91604082013567ffffffffffffffff811161219357614427920161346b565b90604051917f33a89203000000000000000000000000000000000000000000000000000000008352886004840152604060248401525f838061446c60448201866134bc565b038183885af1928315612188575f9361450f575b508251602084012081516020830120036144d4575060019392917fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf1916144cb60405192839283614cc3565b0390a20161438c565b905061450b6040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401614cc3565b0390fd5b9092503d805f833e61452181836133f6565b8101906020818303126121935780519067ffffffffffffffff8211612193570181601f820112156121935780519061455882613419565b9261456660405194856133f6565b8284526020838301011161219357815f9260208093018386015e83010152915f614480565b601f8260209493601f1993818652868601375f8582860101520116010190565b60409061348694928152816020820152019161458b565b906145cd8180613f97565b5f5b818110614777575050506145e66020820182613f97565b5f5b818110614708575050506145ff6040820182613f97565b5f5b81811061469957505050806060614619920190613f97565b90915f5b82811061462a5750505050565b614635818486614338565b35906002821015612193576001809214614650575b0161461d565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c6146806143b184888a614338565b9061469160405192839287846145ab565b0390a261464a565b6146a4818385614338565b359060028210156121935760018092146146bf575b01614601565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596146ef6143b1848789614338565b9061470060405192839287846145ab565b0390a26146b9565b614713818385614338565b3590600282101561219357600180921461472e575b016145e8565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f361475e6143b1848789614338565b9061476f60405192839287846145ab565b0390a2614728565b614782818385614338565b3590600282101561219357600180921461479d575b016145cf565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66147cd6143b1848789614338565b906147de60405192839287846145ab565b0390a2614797565b80158015614838575b8015614830575b8015614820575b61481a576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d0198210156147fd565b5081156147f6565b506401000003d0198110156147ef565b92939290915f90808303614abe5750506401000003d0195f94830861487157505090505f905f90565b6401000003d019808080858009818080808060018009988180808080808b87096004099d80095f09928009600309088180808b80086148b09082613a32565b8184800908996148c08b83613a32565b900890099280096008096148d49083613a32565b9008936001900960020991905b8215158381614a80575b5080614a78575b15614a1a579084939293906001936401000003d0199187965b80156149c75780840496809861499a576401000003d0199088096401000003d019036401000003d019811161280f576401000003d019908a96089793819782810292818404149015171561496d579061496391613a32565b959293969561490b565b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b50945094505093806149ed5750906401000003d019809281808780098092099509900990565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b5060016148f2565b6401000003d019915014155f6148eb565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b614ad2936401000003d01993969296614ce8565b916148e1565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561219357016020813591019167ffffffffffffffff8211612193578160051b3603831361219357565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8182360301811215612193570190565b906020838281520160208260051b85010193835f917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc182360301905b858410614baa575050505050505090565b90919293949596601f198282030186528735838112156121935784018035600281101561219357825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561219357016020813591019067ffffffffffffffff811161219357803603821361219357614c3d602092839260408681866001990152019161458b565b99019796019401929190614b99565b61348691614cb5614caa614c8f614c74614c668680614ad8565b608087526080870191614b5d565b614c816020870187614ad8565b908683036020880152614b5d565b614c9c6040860186614ad8565b908583036040870152614b5d565b926060810190614ad8565b916060818503910152614b5d565b9091614cda613486936040845260408401906134bc565b9160208184039101526134bc565b9492909391851580614eed575b614ee257801580614eda575b614ed257604051608091614d1583836133f6565b823683378415614a915784600180098083529385856001099860208401998a52604084019286845287845160010993606086019485526040519a878c01958c871067ffffffffffffffff8811176133c9578a80979581969482956040525190098d525190099460208b019586525190099860408901998a52519009606087019081528651885114801590614ec6575b15614e6857849283808093816040519c85614dc08f97886133f6565b368737518c51614dd09083613a32565b90088452518551614de19083613a32565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a525180098851614e1e9083613a32565b90088180875185519009600209614e359083613a32565b90089c51935190519009614e498c83613a32565b90089009925190519009614e5d9083613a32565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b50815181511415614da4565b505050600190565b508115614d01565b945092506001919050565b508415614cf5565b805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2054155f14614fa557614f52817f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00613d46565b7f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054905f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2055600190565b505f90565b805f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054155f14614fa557615007817f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613d46565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354905f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2055600190565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e546151a1576151117fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613d46565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903547fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e55600190565b5f90565b5f90602092604051908482019283526040820152604081526151c86060826133f6565b604051918291518091835e8101838152039060025afa15612188575f5190565b8151919060418303615218576152119250602082015190606060408401519301515f1a906152e9565b9192909190565b50505f9160029190565b61522b816134e1565b80615234575050565b61523d816134e1565b6001810361526d577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b615276816134e1565b600281036152aa57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6003906152b6816134e1565b146152be5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161536d579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15612188575f5173ffffffffffffffffffffffffffffffffffffffff81161561536357905f905f90565b505f906001905f90565b5050505f916003919056fea164736f6c6343000825000af0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE04a\x01\x1EW`\x1FaUr8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\"W\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x01\x1EW\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x82\x82\x03a\x01\x1EW` \x01Q\x91`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x90\x81\x84\x03a\x01\x1EW0`\x80Ra\0ua\x016V[a\0}a\x016V[a\0\x85a\x016V[a\0\x8Da\x016V[\x15a\x01\x0FW\x15a\x01\0W`\xA0R`\xC0Ra\0\xA5a\x016V[`@QaS\x85\x90\x81a\x01\xCD\x829`\x80Q\x81\x81\x81a-&\x01Ra.~\x01R`\xA0Q\x81\x81\x81a\x11\xEB\x01R\x81\x81a\x1F\xFE\x01R\x81\x81a,D\x01Ra9!\x01R`\xC0Q\x81\x81\x81a\x04/\x01R\x81\x81a\x10\x91\x01R\x81\x81a\x1F\xBE\x01Ra8\xDF\x01R\xF3[c\xB1\xB2Z\xAF`\xE0\x1B_R`\x04_\xFD[cSl\x15\x01`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_Q` aUR_9_Q\x90_RT`\xFF\x81`@\x1C\x16a\x01\xBDW`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01a\x01kWPV[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17_Q` aUR_9_Q\x90_RU`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD\xFEa\x01``@R`\x046\x10\x15a\0\x12W_\x80\xFD[_a\x01 R_5`\xE0\x1C\x80c1\xEEbB\x14a3lW\x80c2\x8C$3\x14a1\xFBW\x80c?K\xA8:\x14a1>W\x80c@\xF3MB\x14a1\x02W\x80cO\x1E\xF2\x86\x14a.2W\x80cQ\x94[\x06\x14a-\x9EW\x80cR\xD1\x90-\x14a,\xFFW\x80cY\xBA\x92X\x14a,\xC3W\x80c\\\x97Z\xBB\x14a,\x82W\x80cg51\"\x14a,hW\x80ck\xF0\xA0K\x14a,\x18W\x80cqP\x18\xA6\x14a+\\W\x80c\x84V\xCBY\x14a*\xC5W\x80c\x8D\xA5\xCB[\x14a*sW\x80c\x8E\xE0\xA2a\x14a*/W\x80c\x929\xAC\x0C\x14a\x1D\"W\x80c\x9A\xD9\x1DL\x14a\x1C\x9EW\x80c\xA0`V\xF7\x14a\x1C\\W\x80c\xA6\0E\x8B\x14a\r\xC7W\x80c\xAA\xF1\x0FB\x14a\rrW\x80c\xAD<\xB1\xCC\x14a\r\x0FW\x80c\xBD\xEBD-\x14a\x0C\x82W\x80c\xC0%0#\x14a\x0B\xDEW\x80c\xC1\xB0\xBE\xD7\x14a\x0B\x88W\x80c\xC3}uz\x14a\n\xCAW\x80c\xC4IV\xD1\x14a\n\x8BW\x80c\xC4\xD6m\xE8\x14a\x05\x94W\x80c\xC7_0M\x14a\x04\xE3W\x80c\xC8y\xDB\xE4\x14a\x04\x90W\x80c\xE1\xC5\tX\x14a\x04SW\x80c\xE3Z]/\x14a\x03\xF4W\x80c\xE5\xED\xFD\"\x14a\x03\x91W\x80c\xED\x83\xCD\xC7\x14a\x02\xD8W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xA8W\x80c\xFE\x18\xAB\x91\x14a\x02cW\x80c\xFF\xA1\xADt\x14a\x01\xFCWc\xFF\xC3?r\x14a\x01\xB6W_\x80\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` \x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0T`@Q\x90\x81R\xF3[a\x01 Q\x80\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W`@\x80Qa\x02_\x91a\x02!\x90\x82a3\xF6V[`\n\x81R\x7F2.0.0-rc.8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xBCV[\x03\x90\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x02\xD1a\x02\xC4a3\x8AV[a\x02\xCCa>\xD7V[a:\xBAV[a\x01 Q\x80\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5Wa\x01 QP`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR` a\x01 Q \x90a\x01 Q[\x81\x81\x10a\x03{Wa\x02_\x85a\x03g\x81\x87\x03\x82a3\xF6V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\x89V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03PV[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x03\xAAa?CV[`\x045a\x01 QR\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R` `\xFF`@a\x01 Q T\x16`@Q\x90a\x03\xF0\x81a4\xE1V[\x81R\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `@Q\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W` a\x04\xD9`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xF5W6`#\x82\x01\x12\x15a\x01\xF5W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xF5W`$\x82\x01\x91`$6\x91\x83`\x06\x1B\x01\x01\x11a\x01\xF5Wa\x05Aa>\xD7V[a\x05Ia?CV[a\x01 Q[\x81\x81\x10a\x05\\Wa\x01 Q\x80\xF3[a\x05g\x81\x83\x85a:\xAAV[5\x90` a\x05v\x82\x85\x87a:\xAAV[\x015\x91`\x04\x83\x10\x15a\x01\xF5W`\x01\x92a\x05\x8E\x91a=hV[\x01a\x05NV[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x05\xADa3\x8AV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x83`@\x1C\x16\x15\x92\x16\x80\x15\x90\x81a\n\x83W[`\x01\x14\x90\x81a\nyW[\x15\x90\x81a\npW[Pa\nBWa\x06\x8E\x90\x82`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\t\xCCW[a\x06~aA7V[a\x06\x86aA7V[a\x02\xCCaA7V[a\x06\x96aA7V[a\x06\x9EaA7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\t\x99W\x80`\x01a\x07!\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a:\x95V[\x81T\x90`\x03\x1B\x90_\x19\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x83\x1B\x92\x1B\x19\x16\x17\x90Ua\x01 Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ua\x07\x82aPZV[P\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x07\xD5aA7V[a\x07\xDDaA7V[`\x01`\xFF\x19\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16\x17\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02Ua\x080aA7V[\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0U\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x08\xC6a8\x96V[a\tkWa\x08\xD5Wa\x01 Q\x80\xF3[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1a\x02\xD1V[\x7F%\xAEn\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`A`\x04R`$a\x01 Q\xFD[h\x01\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x06vV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x90P\x15\x83a\x05\xFEV[0;\x15\x91Pa\x05\xF6V[\x83\x91Pa\x05\xECV[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045a\n\xE6a?CV[\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01T\x81\x10\x15a\x0BUW` \x90\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01a\x01 QR\x81a\x01 Q \x01a\x01 QPTa\x01 Q`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`2`\x04R`$a\x01 Q\xFD[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x01 QP`\x045a\x01 QR\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R` `@a\x01 Q T\x15\x15`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045a\x0B\xFAa>\xD7V[\x80\x15a\x0CTW\x80\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x01 Q\x80\xF3[\x7FwM]\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T_\x19\x81\x01\x90\x81\x11a\x0C\xDCWa\x0C\xCD` \x91a:?V[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x11`\x04R`$a\x01 Q\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W`@\x80Qa\x02_\x91a\r4\x90\x82a3\xF6V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xBCV[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[4a\x01\xF5W`@`\x03\x196\x01\x12a\x01\xF5Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x01\xF5W`\x80`\x03\x19`\x0456\x03\x01\x12a\x01\xF5W`$5`\xC0R`\xC0Q\x15\x15`\xC0Q\x03a\x01\xF5WZ`\xE0R\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x1C.W`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x0Eba@HV[a\x0Eja?CV[a\x0Ex`\x04\x805\x01\x80a?\x97V[\x80\x91P\x15a\x1C\0W\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0T`d`\x045\x015\x14\x80\x15a\x1B\xD1W[\x15a\x1B\x99Wa\x0E\xBF\x81a@\x03V[`\x80Ra\x0E\xCAa@\x9BV[P`@Qa\x0E\xD7\x81a3\xADV[a\x01 Q\x81Ra\x01 Q` \x82\x01R\x90a\x01 Qa\x01\0Ra\x01 Q[\x81\x81\x10a\x15IW\x82a\x0F>`\x80QQ`\x05\x1B` `\x80Q\x01 \x91a\x0F5a\x0F/a\x0F(`$`\x045\x01`\x045`\x04\x01a@\xE6V[6\x91a45V[\x84aQ\xE8V[\x90\x93\x91\x93aR\"V[` \x81Q\x91\x01Q\x90a\x01 QR` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@a\x01 Q \x16\x91\x16\x90\x80\x82\x03a\x15\x15W\x82a\x0F\x85`\x04\x805\x01\x80a?\x97V[`@\x92\x91\x92Q\x90\x80`\x80\x83\x01\x7F@l`\xF8z[\xB5B\xA7\xFCs\x01\xBA\\\x01\xFEw$\xB5\xB3\xC9\xE35!M\t*\x10\xB4\x05\xF5\xA0` \x85\x01R`d`\x045\x015`@\x85\x01R``\x80\x85\x01RR`\xA0\x82\x01\x90`\xA0\x81`\x05\x1B\x84\x01\x01\x94\x80\x92a\x01 Q\x91[\x83\x83\x10a\x12\xF5W\x86` \x87a\x0F\xFF\x81\x8C\x03`\x1F\x19\x81\x01\x83R\x82a3\xF6V[\x81`@Q\x91\x80Q\x91\x82\x91\x01\x83^\x81\x01\x90a\x01 Q\x82R\x80a\x01 Q\x92\x03\x90`\x02Z\xFA\x15a\x12\xB3Wa\x01 QQa\x10?`D`\x045\x01`\x045`\x04\x01a@\xE6V[\x80`\x04\x93\x92\x93\x11a\x01\xF5W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x835\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x12\xC1WPP`\xC0Q\x15a\x11\xD4W[PPPa\x01\0Qa\x11\\W[\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x11+Z`\xE0Qa:2V[\x7Fo\x14\x981\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[a\x11ha\x01\0QaO\xAAV[\x15a\x11\x9FW\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Qa\x01\0Q\x81R\xA1a\x10\xCFV[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QRa\x01\0Q`\x04R`$a\x01 Q\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x01\xF5Wa\x12O`@Q\x94\x85\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91aE\x8BV[\x92\x7F\x85\x8B\xE2>\xCB\xD2Kp\xEF\xDF\xAC\xAD\xA1\x1C/\x04q\xF39\x82\xE37X\xB8\x86\x1E]F%v\xDCF`$\x84\x01R`D\x83\x01R\x81\x80a\x01 Q\x94\x03\x91Z\xFA\x80\x15a\x12\xB3Wa\x12\x98W[\x80\x80a\x10\xC3V[a\x01 Qa\x12\xA5\x91a3\xF6V[a\x01 Qa\x01\xF5W\x81a\x12\x91V[`@Q=a\x01 Q\x82>=\x90\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[\x90\x91\x92\x93\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x86\x82\x03\x01\x83R\x875\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x82\x12\x15a\x01\xF5W`\xA0\x81\x01\x91\x83\x90a\x13c\x81\x83\x01\x80aJ\xD8V[\x80\x95`\xA0\x86RR`\xC0\x84\x01`\xC0\x86`\x05\x1B\x86\x01\x01\x95\x82a\x01 Q[\x82\x81\x10a\x14\x8EWPPPPPa\x13\x9B` \x83\x83\x01\x01\x83\x83\x01aJ\xD8V[\x94\x90\x84\x82\x03` \x86\x01R\x85\x82R` \x82\x01\x90` \x87`\x05\x1B\x84\x01\x01\x96\x81\x93a\x01 Q\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x14(WPPPPPP\x91\x01`@\x81\x81\x015\x90\x84\x01R``\x80\x82\x015\x90\x84\x01R`\x80\x90\x81\x015\x92\x01\x91\x90\x91RP\x96` \x90\x81\x01\x94\x93`\x01\x01\x92\x01\x90a\x0F\xE1V[\x91\x93\x95\x99\x90\x92\x94\x96\x97P`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x01\xF5W` a\x14z`\x01\x93``a\x14l\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90aK+V[\x91\x81`@\x82\x01R\x01\x90aLLV[\x9B\x01\x97\x01\x91\x01\x91\x8A\x97\x96\x95\x93\x91\x94\x92a\x13\xE5V[\x91\x93\x97`\x01\x91\x93\x95\x96P` a\x15\x03\x82`\x80a\x14\xF5a\x14\xD5\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F\x88\x99\x03\x01\x8DR\x8AaK+V[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90aK+V[\x91\x81``\x82\x01R\x01\x90aLLV[\x99\x01\x95\x01\x91\x01\x91\x88\x95\x94\x93\x91\x92a\x13~V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[a\x15W`\x04\x805\x01\x80a?\x97V[\x90`\xA0R\x81\x10\x15a\x0BUW\x80`\x05\x1B`\xA0Q\x015\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa`\xA0Q6\x03\x01\x84\x12\x15a\x01\xF5Wa\x01 Q`\xA0Q\x85\x90a\x15\xAF\x90\x82\x01\x80a?\x97V[\x90Pa\x15\xBA\x81a@\x03V[a\x15\xC3\x82a@\x03V[\x91a\x01 Q[\x81\x81\x10a\x1AVWPP`\xA0Qa\x15\xE5\x90\x84\x01` \x81\x01\x90a?\x97V[\x92\x90Pa\x15\xF1\x83a@\x03V[\x91a\x15\xFB\x84a@\x03V[\x93a\x01 Q\x90[\x80\x82\x10a\x17\x91WPP\x92a\x16z\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a\x16la\x16\x88\x94a\x16^`\x80\x98`@Q\x99\x8A\x99`\xA0Q\x01\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a4\x89V[\x90\x87\x82\x03`@\x89\x01Ra4\x89V[\x90\x85\x82\x03``\x87\x01Ra4\x89V[\x90\x83\x82\x03`\x80\x85\x01Ra4\x89V[\x03\x90\xA1\x80a\x17\x87W[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x85`\xA0Q\x016\x03\x01\x12a\x01\xF5W`@Q\x90a\x16\xCF\x82a3\xADV[`@\x85`\xA0Q\x01\x015\x82R` \x82\x01``\x86`\xA0Q\x01\x015\x81R`@Q\x92a\x16\xF6\x84a3\xADV[_\x84R_` \x85\x01Ra\x17\x0C\x81Q\x83Q\x90aG\xE6V[\x15a\x17OW\x82`\x01\x95\x94\x92a\x17.\x92` `\x80\x96Q\x93\x01Q\x90Q\x91Q\x92aHHV[` \x83\x01R\x81R\x94`\xA0Q\x01\x015a\x17H\x82`\x80Qa@4V[R\x01a\x0E\xF4V[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[a\x01\0R\x84a\x16\x91V[\x91\x92\x93\x94\x95a\x17\xAE\x91\x97P` \x90`\xA0Q\x01\x01\x8B`\xA0Q\x01a?\x97V[\x87\x91\x97\x10\x15a\x0BUW\x80`\x05\x1B\x87\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x886\x03\x01\x81\x12\x15a\x01\xF5Wa\x17\xF9` \x82\x8A\x01\x015aB\xC7V[`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a\x0C\xDCW`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x82\x8A\x015\x91a\x01 Q[\x82\x81\x10a\x19\x95WPP\x91` \x91`\x01\x94\x93\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x86`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\x19;W[P\x99a\x19\x12a\x19\t`@\x84\x84\x01\x01a\x19\x01a\x18\xF6\x82\x87\x87\x01a@\xB3V[\x87\x87\x87\x01\x015aCxV[\x84\x84\x01a@\xB3V[\x83\x83\x015aE\xC2V[\x81\x81\x015a\x19 \x85\x8Ba@4V[R\x01\x015a\x19.\x82\x89a@4V[R\x01\x8A\x95\x94\x93\x92\x91a\x16\x02V[a\x19\x8Aa\x19\x83a\x19\x8F\x92a\x19N\x85a;\xA7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR\x86a\x01 Q \x01T\x80\x94aQ\xA5V[\x92\x80aQ\xA5V[a<@V[\x8Fa\x18\xD9V[\x90\x92`\x01\x90\x81\x85\x16a\x1A\x17Wa\x1A\x0C\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x80\x84` a\x01 Q \x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR\x83` a\x01 Q \x01T\x90aQ\xA5V[\x93[\x81\x1C\x91\x01a\x18yV[a\x1AP\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x83` a\x01 Q \x01TaQ\xA5V[\x93a\x1A\x0EV[\x90\x91\x92\x93a\x1Al\x90`\xA0Q\x01\x8A`\xA0Q\x01a?\x97V[\x82\x10\x15a\x0BUWa\x1A\x83\x90\x82`\x05\x1B\x81\x01\x90a@\xB3V[a\x1A\x90` \x82\x015aBUV[`@\x81\x015a\x1A\xC9\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a\x1BhWPa\x1A\xD9\x815aN\xF5V[\x15a\x1B6W\x90` \x82a\x1B\x11a\x1B\n```\x01\x96\x01a\x1B\x04a\x1A\xFB\x82\x86a@\xB3V[\x86\x86\x015aCxV[\x83a@\xB3V[\x825aE\xC2V[\x805a\x1B\x1D\x84\x88a@4V[R\x015a\x1B*\x82\x87a@4V[R\x01\x90\x88\x93\x92\x91a\x15\xC9V[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR5`\x04R`$a\x01 Q\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[\x7F\xD6\xD8\x89\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`d`\x045\x015`\x04R`$a\x01 Q\xFD[P\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U`d`\x045\x015\x14a\x0E\xB1V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`\x045\x90\x81\x10\x15a\x0BUW` \x90\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a\x01 QR\x81a\x01 Q \x01a\x01 QPTa\x01 Q`\x03\x1B\x1C`@Q\x90\x81R\xF3[4a!\x93W` `\x03\x196\x01\x12a!\x93Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a!\x93W`\x80`\x03\x19`\x0456\x03\x01\x12a!\x93W\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a*\x07W`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x1D\xA6a@HV[a\x1D\xAEa?CV[a\x1D\xBC`\x04\x805\x01\x80a?\x97V[\x80\x91P\x15a)\xDFW\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0T`d`\x045\x015\x14\x80\x15a)\xB0W[\x15a)~Wa\x1E\x03\x81a@\x03V[\x90a\x1E\x0Ca@\x9BV[P`@Qa\x1E\x19\x81a3\xADV[_\x81R_` \x82\x01R\x91_\x91_[\x81\x81\x10a#\x94WPP\x80` a\x1E_\x92Q`\x05\x1B\x91\x01 \x92a\x0F5a\x1EYa\x0F(`$`\x045\x01`\x045`\x04\x01a@\xE6V[\x85aQ\xE8V[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a#fWPa\x1E\xA2\x90P`\x04\x805\x01\x80a?\x97V[`@Q\x90` \x82\x01\x92\x81`\x80\x84\x01\x7F@l`\xF8z[\xB5B\xA7\xFCs\x01\xBA\\\x01\xFEw$\xB5\xB3\xC9\xE35!M\t*\x10\xB4\x05\xF5\xA0\x86R`d`\x045\x015`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01`\xA0\x83`\x05\x1B\x85\x01\x01\x92\x82_\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01[\x83\x83\x10a!\xC5W\x8A\x8A` _\x8C\x8Ca\x1F>\x81\x8E\x03`\x1F\x19\x81\x01\x83R\x82a3\xF6V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a!\x88W_Qa\x1Fo`D`\x045\x01`\x045`\x04\x01a@\xE6V[\x80`\x04\x11a!\x93W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a!\x97WPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a!\x93W_\x92a i\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91aE\x8BV[\x90\x7F\x85\x8B\xE2>\xCB\xD2Kp\xEF\xDF\xAC\xAD\xA1\x1C/\x04q\xF39\x82\xE37X\xB8\x86\x1E]F%v\xDCF`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a!\x88Wa!sW[P\x80a!\x03W[P\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x01 Q\x80\xF3[a!\x0C\x81aO\xAAV[\x15a!BW` \x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x91`@Q\x90\x81R\xA1\x81a \xABV[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[_a!}\x91a3\xF6V[_a\x01 R\x82a \xA4V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x88\x82\x03\x01\x86R\x865\x82\x81\x12\x15a!\x93W\x83\x01\x90`\xA0\x81\x01\x91a\"\x0E\x81\x80aJ\xD8V[\x80\x94`\xA0\x85RR`\xC0\x83\x01`\xC0\x85`\x05\x1B\x85\x01\x01\x94\x82_[\x82\x81\x10a#\x15WPPPPPa\"?` \x82\x01\x82aJ\xD8V[\x93\x90\x83\x82\x03` \x85\x01R\x84\x82R` \x82\x01\x90` \x86`\x05\x1B\x84\x01\x01\x95\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\"\xC5WPPPP`@\x80\x85\x015\x90\x86\x01RPPP``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01R\x95` \x90\x81\x01\x95\x01\x93\x92`\x01\x01\x91\x90a\x1F\x1DV[\x90\x91\x92\x93\x94\x95\x98`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a!\x93W` a#\x06`\x01\x93``a\x14l\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90aK+V[\x9B\x01\x97\x01\x95\x94\x93\x92\x91\x01a\"\x86V[\x90\x91\x92\x93\x96` \x80a#Y\x83`\x80a\x14\xF5a\x14\xD5\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F`\x01\x9A\x03\x01\x8DR\x8AaK+V[\x99\x01\x95\x01\x93\x92\x91\x01a\"&V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[a#\xA2`\x04\x805\x01\x80a?\x97V[\x82\x10\x15a(<W\x81`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a!\x93W\x01_\x95a#\xEA\x82\x80a?\x97V[\x97\x90Pa#\xF6\x88a@\x03V[\x97a$\0\x81a@\x03V[\x90_[\x81\x81\x10a(iWPPa$\x19` \x85\x01\x85a?\x97V[\x90Pa$$\x81a@\x03V[a\x01@Ra$1\x81a@\x03V[\x90_\x90[\x80\x82\x10a%gWPP\x98\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x91a$\xAEa$\x9D\x9Ba\x16z`\x80\x89\x015\x9D\x8E\x94a$\x8F`@Q\x97\x88\x97\x88R`\xA0` \x89\x01R`\xA0\x88\x01\x90a4\x89V[\x90\x86\x82\x03`@\x88\x01Ra4\x89V[\x84\x81\x03``\x86\x01Ra\x01@Qa4\x89V[\x03\x90\xA1\x80a%_W[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x836\x03\x01\x12a!\x93W`@Q\x91a$\xF1\x83a3\xADV[`@\x81\x015\x83R``` \x84\x01\x91\x015\x81R`@Q\x92a%\x10\x84a3\xADV[_\x84R_` \x85\x01Ra%&\x81Q\x83Q\x90aG\xE6V[\x15a\x17OW\x91a%F\x91`\x01\x95\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92aHHV[` \x83\x01R\x81R\x95a%X\x82\x86a@4V[R\x01a\x1E'V[\x95P\x87a$\xB7V[\x90\x93Pa%w` \x87\x01\x87a?\x97V[\x85\x91\x95\x10\x15a(<W\x80`\x05\x1B\x85\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x866\x03\x01\x81\x12\x15a!\x93Wa%\xC2` \x82\x88\x01\x015aB\xC7V[`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a(\x0FW`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x82\x88\x015\x91_[\x82\x81\x10a'+WPP\x91` \x91`\x01\x94\x93\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x86`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a&\xE3W[P\x97a&\xBCa\x19\t`@\x84\x84\x01\x01a\x19\x01a\x18\xF6\x82\x87\x87\x01a@\xB3V[\x81\x81\x015a&\xCD\x85a\x01@Qa@4V[R\x01\x015a&\xDB\x82\x86a@4V[R\x01\x90a$5V[a\x19\x8Aa\x19\x83a'%\x92a&\xF6\x85a;\xA7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x86_ \x01T\x80\x94aQ\xA5V[_a&\x9FV[\x90\x92`\x01\x90\x81\x85\x16a'\xB8W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta'\xAD\x91aQ\xA5V[\x93[\x81\x1C\x91\x01a&?V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta(\t\x91\x90aQ\xA5V[\x93a'\xAFV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a(s\x86\x80a?\x97V[\x82\x10\x15a(<Wa(\x8A\x90\x82`\x05\x1B\x81\x01\x90a@\xB3V[` \x81\x015\x90a(\x99\x82aBUV[`@\x81\x015a(\xD2\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a)SWPa(\xE2\x815aN\xF5V[\x15a)'W\x90\x8Ca)\x15\x84\x84a)\x0Ea\x1B\n```\x01\x99\x98\x01a\x1B\x04a)\x08\x82\x86a@\xB3V[\x89aCxV[5\x92a@4V[Ra) \x82\x86a@4V[R\x01a$\x03V[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R5`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xD6\xD8\x89\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`d`\x045\x015`\x04R`$_\xFD[P\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U`d`\x045\x015\x14a\x1D\xF5V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa*Ga?CV[` \x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01T`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa*\xDDa>\xD7V[a*\xE5a@HV[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa+ta>\xD7V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a!\x93W_`\x03\x196\x01\x12a!\x93W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93W` a\x04\xD9a8\x96V[4a!\x93W_`\x03\x196\x01\x12a!\x93W` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a-vW` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a!\x93W_`\x03\x196\x01\x12a!\x93W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x90_[\x81\x81\x10a.\x1CWa\x02_\x85a\x03g\x81\x87\x03\x82a3\xF6V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a.\x05V[`@`\x03\x196\x01\x12a!\x93Wa.Fa3\x8AV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93Wa.f\x906\x90`\x04\x01a4kV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a0\xC0W[Pa-vWa.\xB6a>\xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a0\x8CW[Pa/6W\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a0aWP\x82;\x15a06W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a0\x05Wa0\x03\x91aA\x8EV[\0[PP4a0\x0EW\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a0\xB8W[\x81a0\xA8` \x93\x83a3\xF6V[\x81\x01\x03\x12a!\x93WQ\x90\x85a/\x05V[=\x91Pa0\x9BV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a.\xA9V[4a!\x93W_`\x03\x196\x01\x12a!\x93W` \x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa1Va>\xD7V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x81\x16\x15a1\xD3W`\xFF\x19\x16\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a!\x93W_`\x03\x196\x01\x12a!\x93W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16\x90\x81\x15a3VW[Pa3.W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x02\x17\x90Ua2\x9Ba5\x18V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x02\x81R\xA1\0[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x81a2<V[4a!\x93W` `\x03\x196\x01\x12a!\x93W` a\x0C\xCD`\x045a:?V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a!\x93WV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3\xC9W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3\xC9W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3\xC9W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a4A\x82a4\x19V[\x91a4O`@Q\x93\x84a3\xF6V[\x82\x94\x81\x84R\x81\x83\x01\x11a!\x93W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a!\x93W\x81` a4\x86\x935\x91\x01a45V[\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a4\xA6WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4\x99V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x04\x11\x15a4\xEBWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a5 a>\xD7V[`\xFF\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16a8\x94W\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0T_[\x81\x81\x10a7\xF7WPP\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\x02T_[\x81\x81\x10a5\xF0WPP`\x01`\xFF\x19\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16\x17\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02UV[\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\x02T\x81\x10\x15a(<W\x7F\x97i\xA3\xC2\xA2\xCE,i>\x1C\x88\x9Ec\xA7\xE6/s\x92\x186{\xBCZ\xA6\xFA\xED2\xD6\xA7\xCF\xB3\xE9\x81\x01T_\x81\x81R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@\x90 T\x90\x91\x90`\xFF\x16a6y\x81a4\xE1V[\x15a6\x89W[`\x01\x91P\x01a5\x9AV[\x81\x15a7\xCFW\x81_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@_ T\x91`\xFF\x83\x16\x92a6\xCA\x84a4\xE1V[\x83`\x01\x14\x15\x90\x81a7\xAFW[P\x15a7rW_\x81\x81R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@\x90 `\x01`\xFF\x19\x82T\x16\x17\x90Ua7\x1C\x83a4\xE1V[\x82\x15a7dW[`@Q\x92a70\x81a4\xE1V[\x83R\x7F\x14p\x94\x8B28g\x03\xE3um\x8E\x14\xD5\x1F\xAD;\xAF\x8F%\xD8$\x8C$\x8D\xF7\xAFS`\xE3\x80\xC5`@\x84`\x01` \x81\x97\x01R\xA2a6\x7FV[a7m\x81a<\xC3V[a7#V[\x7F\xEE\x80\xEF\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RPa7\xA2\x81a4\xE1V[`$R`\x01`DR`d_\xFD[\x90P`\x01\x84\x91a7\xBE\x83a4\xE1V[a7\xC7\x83a4\xE1V[\x16\x14_a6\xD6V[\x7Fz\xF6#A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0T\x81\x10\x15a(<W\x7F\xB2\xF9\xF8Ya\xE1\x8E~-3.\xAA\x05q\xB4\x88\xC6\xAC\xBA\x1C\xA6jk,\xD2\xD4\x19d\xCA\xC6p\xF8\x81\x01T_\x81\x81R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\x03` R`@\x90 T`\x01\x92\x91a8\x86\x91\x15a8\x8CW`\x03\x90a=hV[\x01a5mV[`\x02\x90a=hV[V[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a!\x88W_\x91a9\xE2W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a!\x88W_\x91a9\xAAWP\x90V[\x90P` \x81=` \x11a9\xDAW[\x81a9\xC5` \x93\x83a3\xF6V[\x81\x01\x03\x12a!\x93WQ\x80\x15\x15\x81\x03a!\x93W\x90V[=\x91Pa9\xB8V[\x90P` \x81=` \x11a:*W[\x81a9\xFD` \x93\x83a3\xF6V[\x81\x01\x03\x12a!\x93WQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a!\x93W` a9QV[=\x91Pa9\xF0V[\x91\x90\x82\x03\x91\x82\x11a(\x0FWV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x81\x10\x15a(<W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03_R` _ \x01\x90_\x90V[\x80T\x82\x10\x15a(<W_R` _ \x01\x90_\x90V[\x91\x90\x81\x10\x15a(<W`\x06\x1B\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a;{Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9W\x80`\x01a<*\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a:\x95V[_\x19\x82\x93\x92T\x91`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90UV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9W\x80`\x01a<*\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a:\x95V[\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9W\x80`\x01a<*\x92\x01\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01U\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01a:\x95V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9Wa<*\x91`\x01\x82\x01\x81Ua:\x95V[\x90\x81\x15a7\xCFW\x81_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@_ T\x90`\xFF\x82\x16\x91a=\xAA\x83a4\xE1V[a=\xB3\x82a4\xE1V[\x82\x80\x83\x14\x15\x91\x82a>\xACW[PP\x15a>hW\x7F\x14p\x94\x8B28g\x03\xE3um\x8E\x14\xD5\x1F\xAD;\xAF\x8F%\xD8$\x8C$\x8D\xF7\xAFS`\xE3\x80\xC5\x91`@\x91\x84_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R\x82_ a>\x1E\x82a4\xE1V[`\xFF\x19\x81T\x16`\xFF\x83\x16\x17\x90Ua>4\x82a4\xE1V[\x81\x15a>ZW[\x82Q\x91a>G\x81a4\xE1V[\x82Ra>R\x81a4\xE1V[` \x82\x01R\xA2V[a>c\x85a<\xC3V[a>;V[\x91\x7F\xEE\x80\xEF\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Ra>\x98\x81a4\xE1V[`$Ra>\xA4\x81a4\xE1V[`DR`d_\xFD[`\xFF\x91\x92Pa>\xBA\x84a4\xE1V[a>\xC3\x83a4\xE1V[a>\xCC\x83a4\xE1V[\x83\x16\x16\x14\x82_a=\xBFV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a?\x17WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16\x15a?oWV[\x7F'\xA4\x136\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a!\x93W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a!\x93WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3\xC9W`\x05\x1B` \x01\x90V[\x90a@\r\x82a?\xEBV[a@\x1A`@Q\x91\x82a3\xF6V[\x82\x81R`\x1F\x19a@*\x82\x94a?\xEBV[\x01\x90` 6\x91\x017V[\x80Q\x82\x10\x15a(<W` \x91`\x05\x1B\x01\x01\x90V[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a@sWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@Q\x90a@\xA8\x82a3\xADV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a!\x93W\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a!\x93W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W` \x01\x91\x816\x03\x83\x13a!\x93WV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15aAfWV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80aBBW[\x15aA\xC2WPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15aB\tWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15aB\x1AW`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80aA\xA2WP\x81;\x15\x15aA\xA2V[\x80_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`\x02`@_ TaB\x8F`\xFF\x82\x16a4\xE1V[\x16aB\x97WPV[\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`\x01`$R`D_\xFD[\x80_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`\x01`@_ TaC\x01`\xFF\x82\x16a4\xE1V[\x16aC\tWPV[\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R_`$R`D_\xFD[\x91\x90\x81\x10\x15a(<W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a!\x93W\x01\x90V[\x90`@\x81\x01aC\x87\x81\x83a?\x97V[\x92\x90P_[\x83\x81\x10aC\x9AWPPPPPV[aC\xBBaC\xB1\x82aC\xAB\x86\x86a?\x97V[\x90aC8V[` \x81\x01\x90a@\xE6V[\x81\x01``\x82\x82\x03\x12a!\x93W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a!\x93W` \x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93W\x82aD\x08\x91\x83\x01a4kV[\x91`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93WaD'\x92\x01a4kV[\x90`@Q\x91\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01R`@`$\x84\x01R_\x83\x80aDl`D\x82\x01\x86a4\xBCV[\x03\x81\x83\x88Z\xF1\x92\x83\x15a!\x88W_\x93aE\x0FW[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03aD\xD4WP`\x01\x93\x92\x91\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91aD\xCB`@Q\x92\x83\x92\x83aL\xC3V[\x03\x90\xA2\x01aC\x8CV[\x90PaE\x0B`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01aL\xC3V[\x03\x90\xFD[\x90\x92P=\x80_\x83>aE!\x81\x83a3\xF6V[\x81\x01\x90` \x81\x83\x03\x12a!\x93W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W\x01\x81`\x1F\x82\x01\x12\x15a!\x93W\x80Q\x90aEX\x82a4\x19V[\x92aEf`@Q\x94\x85a3\xF6V[\x82\x84R` \x83\x83\x01\x01\x11a!\x93W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_aD\x80V[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a4\x86\x94\x92\x81R\x81` \x82\x01R\x01\x91aE\x8BV[\x90aE\xCD\x81\x80a?\x97V[_[\x81\x81\x10aGwWPPPaE\xE6` \x82\x01\x82a?\x97V[_[\x81\x81\x10aG\x08WPPPaE\xFF`@\x82\x01\x82a?\x97V[_[\x81\x81\x10aF\x99WPPP\x80``aF\x19\x92\x01\x90a?\x97V[\x90\x91_[\x82\x81\x10aF*WPPPPV[aF5\x81\x84\x86aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aFPW[\x01aF\x1DV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8CaF\x80aC\xB1\x84\x88\x8AaC8V[\x90aF\x91`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aFJV[aF\xA4\x81\x83\x85aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aF\xBFW[\x01aF\x01V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYaF\xEFaC\xB1\x84\x87\x89aC8V[\x90aG\0`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aF\xB9V[aG\x13\x81\x83\x85aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aG.W[\x01aE\xE8V[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3aG^aC\xB1\x84\x87\x89aC8V[\x90aGo`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aG(V[aG\x82\x81\x83\x85aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aG\x9DW[\x01aE\xCFV[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6aG\xCDaC\xB1\x84\x87\x89aC8V[\x90aG\xDE`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aG\x97V[\x80\x15\x80\x15aH8W[\x80\x15aH0W[\x80\x15aH W[aH\x1AWd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15aG\xFDV[P\x81\x15aG\xF6V[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15aG\xEFV[\x92\x93\x92\x90\x91_\x90\x80\x83\x03aJ\xBEWPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08aHqWPP\x90P_\x90_\x90V[d\x01\0\0\x03\xD0\x19\x80\x80\x80\x85\x80\t\x81\x80\x80\x80\x80`\x01\x80\t\x98\x81\x80\x80\x80\x80\x80\x8B\x87\t`\x04\t\x9D\x80\t_\t\x92\x80\t`\x03\t\x08\x81\x80\x80\x8B\x80\x08aH\xB0\x90\x82a:2V[\x81\x84\x80\t\x08\x99aH\xC0\x8B\x83a:2V[\x90\x08\x90\t\x92\x80\t`\x08\taH\xD4\x90\x83a:2V[\x90\x08\x93`\x01\x90\t`\x02\t\x91\x90[\x82\x15\x15\x83\x81aJ\x80W[P\x80aJxW[\x15aJ\x1AW\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15aI\xC7W\x80\x84\x04\x96\x80\x98aI\x9AWd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a(\x0FWd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15aImW\x90aIc\x91a:2V[\x95\x92\x93\x96\x95aI\x0BV[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80aI\xEDWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01aH\xF2V[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_aH\xEBV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[aJ\xD2\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96aL\xE8V[\x91aH\xE1V[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a!\x93W\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W\x81`\x05\x1B6\x03\x83\x13a!\x93WV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a!\x93W\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x826\x03\x01\x90[\x85\x84\x10aK\xAAWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`\x1F\x19\x82\x82\x03\x01\x86R\x875\x83\x81\x12\x15a!\x93W\x84\x01\x805`\x02\x81\x10\x15a!\x93W\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a!\x93W\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93W\x806\x03\x82\x13a!\x93WaL=` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91aE\x8BV[\x99\x01\x97\x96\x01\x94\x01\x92\x91\x90aK\x99V[a4\x86\x91aL\xB5aL\xAAaL\x8FaLtaLf\x86\x80aJ\xD8V[`\x80\x87R`\x80\x87\x01\x91aK]V[aL\x81` \x87\x01\x87aJ\xD8V[\x90\x86\x83\x03` \x88\x01RaK]V[aL\x9C`@\x86\x01\x86aJ\xD8V[\x90\x85\x83\x03`@\x87\x01RaK]V[\x92``\x81\x01\x90aJ\xD8V[\x91``\x81\x85\x03\x91\x01RaK]V[\x90\x91aL\xDAa4\x86\x93`@\x84R`@\x84\x01\x90a4\xBCV[\x91` \x81\x84\x03\x91\x01Ra4\xBCV[\x94\x92\x90\x93\x91\x85\x15\x80aN\xEDW[aN\xE2W\x80\x15\x80aN\xDAW[aN\xD2W`@Q`\x80\x91aM\x15\x83\x83a3\xF6V[\x826\x837\x84\x15aJ\x91W\x84`\x01\x80\t\x80\x83R\x93\x85\x85`\x01\t\x98` \x84\x01\x99\x8AR`@\x84\x01\x92\x86\x84R\x87\x84Q`\x01\t\x93``\x86\x01\x94\x85R`@Q\x9A\x87\x8C\x01\x95\x8C\x87\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x11\x17a3\xC9W\x8A\x80\x97\x95\x81\x96\x94\x82\x95`@RQ\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90aN\xC6W[\x15aNhW\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85aM\xC0\x8F\x97\x88a3\xF6V[6\x877Q\x8CQaM\xD0\x90\x83a:2V[\x90\x08\x84RQ\x85QaM\xE1\x90\x83a:2V[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88QaN\x1E\x90\x83a:2V[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\taN5\x90\x83a:2V[\x90\x08\x9CQ\x93Q\x90Q\x90\taNI\x8C\x83a:2V[\x90\x08\x90\t\x92Q\x90Q\x90\taN]\x90\x83a:2V[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15aM\xA4V[PPP`\x01\x90V[P\x81\x15aM\x01V[\x94P\x92P`\x01\x91\x90PV[P\x84\x15aL\xF5V[\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ T\x15_\x14aO\xA5WaOR\x81\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a=FV[\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T\x90_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ U`\x01\x90V[P_\x90V[\x80_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15_\x14aO\xA5WaP\x07\x81\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a=FV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x90_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ U`\x01\x90V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nTaQ\xA1WaQ\x11\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a=FV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nU`\x01\x90V[_\x90V[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81RaQ\xC8``\x82a3\xF6V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a!\x88W_Q\x90V[\x81Q\x91\x90`A\x83\x03aR\x18WaR\x11\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aR\xE9V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[aR+\x81a4\xE1V[\x80aR4WPPV[aR=\x81a4\xE1V[`\x01\x81\x03aRmW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[aRv\x81a4\xE1V[`\x02\x81\x03aR\xAAWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x90aR\xB6\x81a4\xE1V[\x14aR\xBEWPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aSmW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a!\x88W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aScW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V\xFE\xA1dsolcC\0\x08%\0\n\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6101606040526004361015610012575f80fd5b5f610120525f3560e01c806331ee62421461336c578063328c2433146131fb5780633f4ba83a1461313e57806340f34d42146131025780634f1ef28614612e3257806351945b0614612d9e57806352d1902d14612cff57806359ba925814612cc35780635c975abb14612c825780636735312214612c685780636bf0a04b14612c18578063715018a614612b5c5780638456cb5914612ac55780638da5cb5b14612a735780638ee0a26114612a2f5780639239ac0c14611d225780639ad91d4c14611c9e578063a06056f714611c5c578063a600458b14610dc7578063aaf10f4214610d72578063ad3cb1cc14610d0f578063bdeb442d14610c82578063c025302314610bde578063c1b0bed714610b88578063c37d757a14610aca578063c44956d114610a8b578063c4d66de814610594578063c75f304d146104e3578063c879dbe414610490578063e1c5095814610453578063e35a5d2f146103f4578063e5edfd2214610391578063ed83cdc7146102d8578063f2fde38b146102a8578063fe18ab9114610263578063ffa1ad74146101fc5763ffc33f72146101b6575f80fd5b346101f557610120516003193601126101f55760207f54765cac2cb330b12e843854496f3e05fcf80a7a08eec821362177619185190054604051908152f35b6101205180fd5b346101f557610120516003193601126101f5576040805161025f9161022190826133f6565b600a81527f322e302e302d72632e380000000000000000000000000000000000000000000060208201526040519182916020835260208301906134bc565b0390f35b346101f557610120516003193601126101f5576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b346101f55760206003193601126101f5576102d16102c461338a565b6102cc613ed7565b613aba565b6101205180f35b346101f557610120516003193601126101f55761012051506040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90161012051526020610120512090610120515b81811061037b5761025f85610367818703826133f6565b604051918291602083526020830190613489565b8254845260209093019260019283019201610350565b346101f55760206003193601126101f5576103aa613f43565b60043561012051527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052602060ff604061012051205416604051906103f0816134e1565b8152f35b346101f557610120516003193601126101f55760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101f557610120516003193601126101f55760206040517fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8558152f35b346101f55760206003193601126101f55760206104d96004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b6040519015158152f35b346101f55760206003193601126101f55760043567ffffffffffffffff81116101f557366023820112156101f557806004013567ffffffffffffffff81116101f5576024820191602436918360061b0101116101f557610541613ed7565b610549613f43565b610120515b81811061055c576101205180f35b610567818385613aaa565b35906020610576828587613aaa565b01359160048310156101f55760019261058e91613d68565b0161054e565b346101f55760206003193601126101f5576105ad61338a565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549067ffffffffffffffff60ff8360401c1615921680159081610a83575b6001149081610a79575b159081610a70575b50610a425761068e908260017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000007ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005416177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00556109cc575b61067e614137565b610686614137565b6102cc614137565b610696614137565b61069e614137565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156109995780600161072192017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902613a95565b81549060031b905f197fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06831b921b1916179055610120517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005561078261505a565b507f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a16107d5614137565b6107dd614137565b600160ff197f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3025416177f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30255610830614137565b7fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f54765cac2cb330b12e843854496f3e05fcf80a7a08eec8213621776191851900557fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26108c6613896565b61096b576108d5576101205180f35b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a16102d1565b7f25ae6eaa000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526041600452602461012051fd5b680100000000000000007fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005416177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610676565b7ff92ee8a9000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b905015836105fe565b303b1591506105f6565b8391506105ec565b346101f557610120516003193601126101f55760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b346101f55760206003193601126101f557600435610ae6613f43565b7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30154811015610b55576020907f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3016101205152816101205120016101205150546101205160031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526032600452602461012051fd5b346101f55760206003193601126101f557610120515060043561012051527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc01602052602060406101205120541515604051908152f35b346101f55760206003193601126101f557600435610bfa613ed7565b8015610c5457807f54765cac2cb330b12e843854496f3e05fcf80a7a08eec8213621776191851900557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26101205180f35b7f774d5de1000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b346101f557610120516003193601126101f5577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903545f198101908111610cdc57610ccd602091613a3f565b90549060031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526011600452602461012051fd5b346101f557610120516003193601126101f5576040805161025f91610d3490826133f6565b600581527f352e302e3000000000000000000000000000000000000000000000000000000060208201526040519182916020835260208301906134bc565b346101f557610120516003193601126101f557602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b346101f55760406003193601126101f55767ffffffffffffffff600435116101f5576080600319600435360301126101f55760243560c05260c051151560c051036101f5575a60e0527f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c611c2e5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d610e62614048565b610e6a613f43565b610e78600480350180613f97565b80915015611c00577f54765cac2cb330b12e843854496f3e05fcf80a7a08eec82136217761918519005460646004350135148015611bd1575b15611b9957610ebf81614003565b608052610eca61409b565b50604051610ed7816133ad565b610120518152610120516020820152906101205161010052610120515b8181106115495782610f3e6080515160051b6020608051012091610f35610f2f610f286024600435016004356004016140e6565b3691613435565b846151e8565b90939193615222565b6020815191015190610120515260205273ffffffffffffffffffffffffffffffffffffffff8060406101205120169116908082036115155782610f85600480350180613f97565b6040929192519080608083017f406c60f87a5bb542a7fc7301ba5c01fe7724b5b3c9e335214d092a10b405f5a060208501526064600435013560408501526060808501525260a082019060a08160051b84010194809261012051915b8383106112f55786602087610fff818c03601f1981018352826133f6565b8160405191805191829101835e810190610120518252806101205192039060025afa156112b357610120515161103f6044600435016004356004016140e6565b806004939293116101f5577fffffffff000000000000000000000000000000000000000000000000000000008335167fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168082036112c157505060c051156111d4575b5050506101005161115c575b7f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d61112b5a60e051613a32565b7f6f149831000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b61116861010051614faa565b1561119f577f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d26020604051610100518152a16110cf565b7fdb788c2b00000000000000000000000000000000000000000000000000000000610120515261010051600452602461012051fd5b73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b156101f55761124f6040519485937fab750e7500000000000000000000000000000000000000000000000000000000855260606004860152606485019161458b565b927f858be23ecbd24b70efdfacada11c2f0471f33982e33758b8861e5d462576dc46602484015260448301528180610120519403915afa80156112b357611298575b80806110c3565b610120516112a5916133f6565b610120516101f55781611291565b6040513d61012051823e3d90fd5b7f78a2221c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b90919293967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608682030183528735907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61833603018212156101f55760a0810191839061136381830180614ad8565b809560a086525260c0840160c08660051b8601019582610120515b82811061148e57505050505061139b602083830101838301614ad8565b94908482036020860152858252602082019060208760051b84010196819361012051927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b8381106114285750505050505091016040818101359084015260608082013590840152608090810135920191909152509660209081019493600101920190610fe1565b91939599909294969750601f198482030187528935868112156101f557602061147a600193606061146c888596018035845285810135868501526040810190614b2b565b918160408201520190614c4c565b9b0197019101918a979695939194926113e5565b9193976001919395965060206115038260806114f56114d58e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f889903018d528a614b2b565b803584528581013586850152604081013560408501526060810190614b2b565b918160608201520190614c4c565b9901950191019188959493919261137e565b7fe6d44b4c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b611557600480350180613f97565b9060a052811015610b55578060051b60a0510135927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6160a0513603018412156101f5576101205160a05185906115af90820180613f97565b90506115ba81614003565b6115c382614003565b91610120515b818110611a5657505060a0516115e59084016020810190613f97565b9290506115f183614003565b916115fb84614003565b9361012051905b8082106117915750509261167a7f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32959361166c6116889461165e608098604051998a9960a051010135895260a060208a015260a0890190613489565b908782036040890152613489565b908582036060870152613489565b908382036080850152613489565b0390a180611787575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08560a05101360301126101f557604051906116cf826133ad565b60408560a05101013582526020820160608660a0510101358152604051926116f6846133ad565b5f84525f602085015261170c81518351906147e6565b1561174f5782600195949261172e926020608096519301519051915192614848565b602083015281529460a05101013561174882608051614034565b5201610ef4565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b6101005284611691565b91929394956117ae91975060209060a05101018b60a05101613f97565b8791971015610b55578060051b8701357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1883603018112156101f5576117f96020828a0101356142c7565b60ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f198114610cdc57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005590828a013591610120515b82811061199557505091602091600194937f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548660ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b1461193b575b50996119126119096040848401016119016118f6828787016140b3565b878787010135614378565b8484016140b3565b838301356145c2565b81810135611920858b614034565b5201013561192e8289614034565b52018a9594939291611602565b61198a61198361198f9261194e85613ba7565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9026101205152866101205120015480946151a5565b92806151a5565b613c40565b8f6118d9565b9092600190818516611a1757611a0c907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901610120515280846020610120512001557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902610120515283602061012051200154906151a5565b935b811c9101611879565b611a50907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9016101205152836020610120512001546151a5565b93611a0e565b90919293611a6c9060a051018a60a05101613f97565b821015610b5557611a83908260051b8101906140b3565b611a906020820135614255565b6040810135611ac9815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b15611b685750611ad98135614ef5565b15611b365790602082611b11611b0a606060019601611b04611afb82866140b3565b86860135614378565b836140b3565b82356145c2565b8035611b1d8488614034565b520135611b2a8287614034565b520190889392916115c9565b7f39a940c500000000000000000000000000000000000000000000000000000000610120515235600452602461012051fd5b7ff9849ea3000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b7fd6d8891b00000000000000000000000000000000000000000000000000000000610120515260646004350135600452602461012051fd5b507fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8556064600435013514610eb1565b7fcf1b093c000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b346101f557610120516003193601126101f557602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b346101f55760206003193601126101f5577f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc005460043590811015610b55576020907f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc006101205152816101205120016101205150546101205160031b1c604051908152f35b346121935760206003193601126121935767ffffffffffffffff6004351161219357608060031960043536030112612193577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c612a075760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d611da6614048565b611dae613f43565b611dbc600480350180613f97565b809150156129df577f54765cac2cb330b12e843854496f3e05fcf80a7a08eec821362177619185190054606460043501351480156129b0575b1561297e57611e0381614003565b90611e0c61409b565b50604051611e19816133ad565b5f81525f6020820152915f915f5b818110612394575050806020611e5f925160051b91012092610f35611e59610f286024600435016004356004016140e6565b856151e8565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f20169116908082036123665750611ea29050600480350180613f97565b60405190602082019281608084017f406c60f87a5bb542a7fc7301ba5c01fe7724b5b3c9e335214d092a10b405f5a086526064600435013560408601526060808601525260a0830160a08360051b85010192825f907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603015b8383106121c5578a8a60205f8c8c611f3e818e03601f1981018352826133f6565b604051918291518091835e8101838152039060025afa15612188575f51611f6f6044600435016004356004016140e6565b80600411612193577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000001680820361219757505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15612193575f9261206992604051958694859384937fab750e7500000000000000000000000000000000000000000000000000000000855260606004860152606485019161458b565b907f858be23ecbd24b70efdfacada11c2f0471f33982e33758b8861e5d462576dc466024840152604483015203915afa801561218857612173575b5080612103575b507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d6101205180f35b61210c81614faa565b156121425760207f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d291604051908152a1816120ab565b7fdb788c2b000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b5f61217d916133f6565b5f61012052826120a4565b6040513d5f823e3d90fd5b5f80fd5b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9091929394957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608882030186528635828112156121935783019060a081019161220e8180614ad8565b809460a085525260c0830160c08560051b85010194825f5b82811061231557505050505061223f6020820182614ad8565b93908382036020850152848252602082019060208660051b8401019581935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b8381106122c557505050506040808501359086015250505060608082013590830152608090810135910152956020908101950193926001019190611f1d565b90919293949598601f19848203018752893586811215612193576020612306600193606061146c888596018035845285810135868501526040810190614b2b565b9b019701959493929101612286565b90919293966020806123598360806114f56114d58e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f60019a03018d528a614b2b565b9901950193929101612226565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b6123a2600480350180613f97565b82101561283c578160051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff618136030182121561219357015f956123ea8280613f97565b9790506123f688614003565b9761240081614003565b905f5b8181106128695750506124196020850185613f97565b905061242481614003565b6101405261243181614003565b905f905b808210612567575050987f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32916124ae61249d9b61167a60808901359d8e9461248f604051978897885260a0602089015260a0880190613489565b908682036040880152613489565b848103606086015261014051613489565b0390a18061255f575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0833603011261219357604051916124f1836133ad565b60408101358352606060208401910135815260405192612510846133ad565b5f84525f602085015261252681518351906147e6565b1561174f5791612546916001959493602083519301519051915192614848565b60208301528152956125588286614034565b5201611e27565b9550876124b7565b9093506125776020870187613f97565b859195101561283c578060051b8501357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa186360301811215612193576125c2602082880101356142c7565b60ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f19811461280f57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900559082880135915f5b82811061272b57505091602091600194937f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548660ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b146126e3575b50976126bc6119096040848401016119016118f6828787016140b3565b818101356126cd8561014051614034565b520101356126db8286614034565b520190612435565b61198a611983612725926126f685613ba7565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f52865f20015480946151a5565b5f61269f565b90926001908185166127b8577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301546127ad916151a5565b935b811c910161263f565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83015461280991906151a5565b936127af565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6128738680613f97565b82101561283c5761288a908260051b8101906140b3565b60208101359061289982614255565b60408101356128d2815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b1561295357506128e28135614ef5565b1561292757908c612915848461290e611b0a60606001999801611b0461290882866140b3565b89614378565b3592614034565b526129208286614034565b5201612403565b7f39a940c5000000000000000000000000000000000000000000000000000000005f523560045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fd6d8891b000000000000000000000000000000000000000000000000000000005f526064600435013560045260245ffd5b507fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8556064600435013514611df5565b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34612193575f60031936011261219357612a47613f43565b60207f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30154604051908152f35b34612193575f60031936011261219357602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b34612193575f60031936011261219357612add613ed7565b612ae5614048565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b34612193575f60031936011261219357612b74613ed7565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34612193575f60031936011261219357602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34612193575f6003193601126121935760206104d9613896565b34612193575f60031936011261219357602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b34612193575f6003193601126121935760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354604051908152f35b34612193575f6003193601126121935773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163003612d765760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b34612193575f600319360112612193576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f20905f5b818110612e1c5761025f85610367818703826133f6565b8254845260209093019260019283019201612e05565b604060031936011261219357612e4661338a565b60243567ffffffffffffffff811161219357612e6690369060040161346b565b9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168030149081156130c0575b50612d7657612eb6613ed7565b73ffffffffffffffffffffffffffffffffffffffff8116916040517f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f918161308c575b50612f3657837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8592036130615750823b1561303657807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2805115613005576130039161418e565b005b50503461300e57005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d6020116130b8575b816130a8602093836133f6565b8101031261219357519085612f05565b3d915061309b565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416141583612ea9565b34612193575f6003193601126121935760207f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054604051908152f35b34612193575f60031936011261219357613156613ed7565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff8116156131d35760ff19167fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b34612193575f600319360112612193577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c16908115613356575b5061332e577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0080547fffffffffffffffffffffffffffffffffffffffffffffff000000000000000000166801000000000000000217905561329b613518565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160028152a1005b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002915067ffffffffffffffff1610158161323c565b34612193576020600319360112612193576020610ccd600435613a3f565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361219357565b6040810190811067ffffffffffffffff8211176133c957604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff8211176133c957604052565b67ffffffffffffffff81116133c957601f01601f191660200190565b92919261344182613419565b9161344f60405193846133f6565b829481845281830111612193578281602093845f960137010152565b9080601f830112156121935781602061348693359101613435565b90565b90602080835192838152019201905f5b8181106134a65750505090565b8251845260209384019390920191600101613499565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b600411156134eb57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b613520613ed7565b60ff7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3025416613894577f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900545f5b8181106137f75750507f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62902545f5b8181106135f0575050600160ff197f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e3025416177f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30255565b7f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629025481101561283c577f9769a3c2a2ce2c693e1c889e63a7e62f739218367bbc5aa6faed32d6a7cfb3e98101545f8181527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052604090205490919060ff16613679816134e1565b15613689575b600191500161359a565b81156137cf57815f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30060205260405f20549160ff8316926136ca846134e1565b836001141590816137af575b5015613772575f8181527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30060205260409020600160ff1982541617905561371c836134e1565b8215613764575b60405192613730816134e1565b83527f1470948b32386703e3756d8e14d51fad3baf8f25d8248c248df7af5360e380c56040846001602081970152a261367f565b61376d81613cc3565b613723565b7fee80efae000000000000000000000000000000000000000000000000000000005f52600452506137a2816134e1565b602452600160445260645ffd5b9050600184916137be836134e1565b6137c7836134e1565b16145f6136d6565b7f7af62341000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629005481101561283c577fb2f9f85961e18e7e2d332eaa0571b488c6acba1ca66a6b2cd2d41964cac670f88101545f8181527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62903602052604090205460019291613886911561388c57600390613d68565b0161356d565b600290613d68565b565b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115612188575f916139e2575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa908115612188575f916139aa575090565b90506020813d6020116139da575b816139c5602093836133f6565b81010312612193575180151581036121935790565b3d91506139b8565b90506020813d602011613a2a575b816139fd602093836133f6565b81010312612193575173ffffffffffffffffffffffffffffffffffffffff81168103612193576020613951565b3d91506139f0565b9190820391821161280f57565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9035481101561283c577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9035f5260205f2001905f90565b805482101561283c575f5260205f2001905f90565b919081101561283c5760061b0190565b73ffffffffffffffffffffffffffffffffffffffff168015613b7b5773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154680100000000000000008110156133c957806001613c2a92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901613a95565b5f19829392549160031b92831b921b1916179055565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156133c957806001613c2a92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902613a95565b7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30154680100000000000000008110156133c957806001613c2a92017f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e301557f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e301613a95565b8054680100000000000000008110156133c957613c2a91600182018155613a95565b9081156137cf57815f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e30060205260405f20549060ff821691613daa836134e1565b613db3826134e1565b82808314159182613eac575b505015613e68577f1470948b32386703e3756d8e14d51fad3baf8f25d8248c248df7af5360e380c591604091845f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052825f20613e1e826134e1565b60ff1981541660ff8316179055613e34826134e1565b8115613e5a575b825191613e47816134e1565b8252613e52816134e1565b6020820152a2565b613e6385613cc3565b613e3b565b917fee80efae000000000000000000000000000000000000000000000000000000005f52600452613e98816134e1565b602452613ea4816134e1565b60445260645ffd5b60ff919250613eba846134e1565b613ec3836134e1565b613ecc836134e1565b83161614825f613dbf565b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054163303613f1757565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e302541615613f6f57565b7f27a41336000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215612193570180359067ffffffffffffffff821161219357602001918160051b3603831361219357565b67ffffffffffffffff81116133c95760051b60200190565b9061400d82613feb565b61401a60405191826133f6565b828152601f1961402a8294613feb565b0190602036910137565b805182101561283c5760209160051b010190565b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541661407357565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b604051906140a8826133ad565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215612193570190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215612193570180359067ffffffffffffffff82116121935760200191813603831361219357565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c161561416657565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b905f8091602081519101845af48080614242575b156141c25750506040513d81523d5f602083013e60203d82010160405290565b156142095773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d1561421a576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d1515806141a25750813b15156141a2565b805f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052600260405f205461428f60ff82166134e1565b166142975750565b7f597abd20000000000000000000000000000000000000000000000000000000005f52600452600160245260445ffd5b805f527f95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300602052600160405f205461430160ff82166134e1565b166143095750565b7f597abd20000000000000000000000000000000000000000000000000000000005f526004525f60245260445ffd5b919081101561283c5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215612193570190565b90604081016143878183613f97565b9290505f5b83811061439a575050505050565b6143bb6143b1826143ab8686613f97565b90614338565b60208101906140e6565b81016060828203126121935781359173ffffffffffffffffffffffffffffffffffffffff831680930361219357602081013567ffffffffffffffff8111612193578261440891830161346b565b91604082013567ffffffffffffffff811161219357614427920161346b565b90604051917f33a89203000000000000000000000000000000000000000000000000000000008352886004840152604060248401525f838061446c60448201866134bc565b038183885af1928315612188575f9361450f575b508251602084012081516020830120036144d4575060019392917fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf1916144cb60405192839283614cc3565b0390a20161438c565b905061450b6040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401614cc3565b0390fd5b9092503d805f833e61452181836133f6565b8101906020818303126121935780519067ffffffffffffffff8211612193570181601f820112156121935780519061455882613419565b9261456660405194856133f6565b8284526020838301011161219357815f9260208093018386015e83010152915f614480565b601f8260209493601f1993818652868601375f8582860101520116010190565b60409061348694928152816020820152019161458b565b906145cd8180613f97565b5f5b818110614777575050506145e66020820182613f97565b5f5b818110614708575050506145ff6040820182613f97565b5f5b81811061469957505050806060614619920190613f97565b90915f5b82811061462a5750505050565b614635818486614338565b35906002821015612193576001809214614650575b0161461d565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c6146806143b184888a614338565b9061469160405192839287846145ab565b0390a261464a565b6146a4818385614338565b359060028210156121935760018092146146bf575b01614601565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596146ef6143b1848789614338565b9061470060405192839287846145ab565b0390a26146b9565b614713818385614338565b3590600282101561219357600180921461472e575b016145e8565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f361475e6143b1848789614338565b9061476f60405192839287846145ab565b0390a2614728565b614782818385614338565b3590600282101561219357600180921461479d575b016145cf565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66147cd6143b1848789614338565b906147de60405192839287846145ab565b0390a2614797565b80158015614838575b8015614830575b8015614820575b61481a576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d0198210156147fd565b5081156147f6565b506401000003d0198110156147ef565b92939290915f90808303614abe5750506401000003d0195f94830861487157505090505f905f90565b6401000003d019808080858009818080808060018009988180808080808b87096004099d80095f09928009600309088180808b80086148b09082613a32565b8184800908996148c08b83613a32565b900890099280096008096148d49083613a32565b9008936001900960020991905b8215158381614a80575b5080614a78575b15614a1a579084939293906001936401000003d0199187965b80156149c75780840496809861499a576401000003d0199088096401000003d019036401000003d019811161280f576401000003d019908a96089793819782810292818404149015171561496d579061496391613a32565b959293969561490b565b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b50945094505093806149ed5750906401000003d019809281808780098092099509900990565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b5060016148f2565b6401000003d019915014155f6148eb565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b614ad2936401000003d01993969296614ce8565b916148e1565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561219357016020813591019167ffffffffffffffff8211612193578160051b3603831361219357565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8182360301811215612193570190565b906020838281520160208260051b85010193835f917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc182360301905b858410614baa575050505050505090565b90919293949596601f198282030186528735838112156121935784018035600281101561219357825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561219357016020813591019067ffffffffffffffff811161219357803603821361219357614c3d602092839260408681866001990152019161458b565b99019796019401929190614b99565b61348691614cb5614caa614c8f614c74614c668680614ad8565b608087526080870191614b5d565b614c816020870187614ad8565b908683036020880152614b5d565b614c9c6040860186614ad8565b908583036040870152614b5d565b926060810190614ad8565b916060818503910152614b5d565b9091614cda613486936040845260408401906134bc565b9160208184039101526134bc565b9492909391851580614eed575b614ee257801580614eda575b614ed257604051608091614d1583836133f6565b823683378415614a915784600180098083529385856001099860208401998a52604084019286845287845160010993606086019485526040519a878c01958c871067ffffffffffffffff8811176133c9578a80979581969482956040525190098d525190099460208b019586525190099860408901998a52519009606087019081528651885114801590614ec6575b15614e6857849283808093816040519c85614dc08f97886133f6565b368737518c51614dd09083613a32565b90088452518551614de19083613a32565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a525180098851614e1e9083613a32565b90088180875185519009600209614e359083613a32565b90089c51935190519009614e498c83613a32565b90089009925190519009614e5d9083613a32565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b50815181511415614da4565b505050600190565b508115614d01565b945092506001919050565b508415614cf5565b805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2054155f14614fa557614f52817f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00613d46565b7f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054905f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2055600190565b505f90565b805f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054155f14614fa557615007817f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613d46565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354905f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2055600190565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e546151a1576151117fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613d46565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903547fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e55600190565b5f90565b5f90602092604051908482019283526040820152604081526151c86060826133f6565b604051918291518091835e8101838152039060025afa15612188575f5190565b8151919060418303615218576152119250602082015190606060408401519301515f1a906152e9565b9192909190565b50505f9160029190565b61522b816134e1565b80615234575050565b61523d816134e1565b6001810361526d577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b615276816134e1565b600281036152aa57507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6003906152b6816134e1565b146152be5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161536d579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15612188575f5173ffffffffffffffffffffffffffffffffffffffff81161561536357905f905f90565b505f906001905f90565b5050505f916003919056fea164736f6c6343000825000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R`\x046\x10\x15a\0\x12W_\x80\xFD[_a\x01 R_5`\xE0\x1C\x80c1\xEEbB\x14a3lW\x80c2\x8C$3\x14a1\xFBW\x80c?K\xA8:\x14a1>W\x80c@\xF3MB\x14a1\x02W\x80cO\x1E\xF2\x86\x14a.2W\x80cQ\x94[\x06\x14a-\x9EW\x80cR\xD1\x90-\x14a,\xFFW\x80cY\xBA\x92X\x14a,\xC3W\x80c\\\x97Z\xBB\x14a,\x82W\x80cg51\"\x14a,hW\x80ck\xF0\xA0K\x14a,\x18W\x80cqP\x18\xA6\x14a+\\W\x80c\x84V\xCBY\x14a*\xC5W\x80c\x8D\xA5\xCB[\x14a*sW\x80c\x8E\xE0\xA2a\x14a*/W\x80c\x929\xAC\x0C\x14a\x1D\"W\x80c\x9A\xD9\x1DL\x14a\x1C\x9EW\x80c\xA0`V\xF7\x14a\x1C\\W\x80c\xA6\0E\x8B\x14a\r\xC7W\x80c\xAA\xF1\x0FB\x14a\rrW\x80c\xAD<\xB1\xCC\x14a\r\x0FW\x80c\xBD\xEBD-\x14a\x0C\x82W\x80c\xC0%0#\x14a\x0B\xDEW\x80c\xC1\xB0\xBE\xD7\x14a\x0B\x88W\x80c\xC3}uz\x14a\n\xCAW\x80c\xC4IV\xD1\x14a\n\x8BW\x80c\xC4\xD6m\xE8\x14a\x05\x94W\x80c\xC7_0M\x14a\x04\xE3W\x80c\xC8y\xDB\xE4\x14a\x04\x90W\x80c\xE1\xC5\tX\x14a\x04SW\x80c\xE3Z]/\x14a\x03\xF4W\x80c\xE5\xED\xFD\"\x14a\x03\x91W\x80c\xED\x83\xCD\xC7\x14a\x02\xD8W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xA8W\x80c\xFE\x18\xAB\x91\x14a\x02cW\x80c\xFF\xA1\xADt\x14a\x01\xFCWc\xFF\xC3?r\x14a\x01\xB6W_\x80\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` \x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0T`@Q\x90\x81R\xF3[a\x01 Q\x80\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W`@\x80Qa\x02_\x91a\x02!\x90\x82a3\xF6V[`\n\x81R\x7F2.0.0-rc.8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xBCV[\x03\x90\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x02\xD1a\x02\xC4a3\x8AV[a\x02\xCCa>\xD7V[a:\xBAV[a\x01 Q\x80\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5Wa\x01 QP`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR` a\x01 Q \x90a\x01 Q[\x81\x81\x10a\x03{Wa\x02_\x85a\x03g\x81\x87\x03\x82a3\xF6V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\x89V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03PV[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x03\xAAa?CV[`\x045a\x01 QR\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R` `\xFF`@a\x01 Q T\x16`@Q\x90a\x03\xF0\x81a4\xE1V[\x81R\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `@Q\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W` a\x04\xD9`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xF5W6`#\x82\x01\x12\x15a\x01\xF5W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xF5W`$\x82\x01\x91`$6\x91\x83`\x06\x1B\x01\x01\x11a\x01\xF5Wa\x05Aa>\xD7V[a\x05Ia?CV[a\x01 Q[\x81\x81\x10a\x05\\Wa\x01 Q\x80\xF3[a\x05g\x81\x83\x85a:\xAAV[5\x90` a\x05v\x82\x85\x87a:\xAAV[\x015\x91`\x04\x83\x10\x15a\x01\xF5W`\x01\x92a\x05\x8E\x91a=hV[\x01a\x05NV[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x05\xADa3\x8AV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x83`@\x1C\x16\x15\x92\x16\x80\x15\x90\x81a\n\x83W[`\x01\x14\x90\x81a\nyW[\x15\x90\x81a\npW[Pa\nBWa\x06\x8E\x90\x82`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\t\xCCW[a\x06~aA7V[a\x06\x86aA7V[a\x02\xCCaA7V[a\x06\x96aA7V[a\x06\x9EaA7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\t\x99W\x80`\x01a\x07!\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a:\x95V[\x81T\x90`\x03\x1B\x90_\x19\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x83\x1B\x92\x1B\x19\x16\x17\x90Ua\x01 Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ua\x07\x82aPZV[P\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x07\xD5aA7V[a\x07\xDDaA7V[`\x01`\xFF\x19\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16\x17\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02Ua\x080aA7V[\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0U\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x08\xC6a8\x96V[a\tkWa\x08\xD5Wa\x01 Q\x80\xF3[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1a\x02\xD1V[\x7F%\xAEn\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`A`\x04R`$a\x01 Q\xFD[h\x01\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x06vV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x90P\x15\x83a\x05\xFEV[0;\x15\x91Pa\x05\xF6V[\x83\x91Pa\x05\xECV[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045a\n\xE6a?CV[\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01T\x81\x10\x15a\x0BUW` \x90\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01a\x01 QR\x81a\x01 Q \x01a\x01 QPTa\x01 Q`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`2`\x04R`$a\x01 Q\xFD[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5Wa\x01 QP`\x045a\x01 QR\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R` `@a\x01 Q T\x15\x15`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W`\x045a\x0B\xFAa>\xD7V[\x80\x15a\x0CTW\x80\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x01 Q\x80\xF3[\x7FwM]\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T_\x19\x81\x01\x90\x81\x11a\x0C\xDCWa\x0C\xCD` \x91a:?V[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x11`\x04R`$a\x01 Q\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W`@\x80Qa\x02_\x91a\r4\x90\x82a3\xF6V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xBCV[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[4a\x01\xF5W`@`\x03\x196\x01\x12a\x01\xF5Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x01\xF5W`\x80`\x03\x19`\x0456\x03\x01\x12a\x01\xF5W`$5`\xC0R`\xC0Q\x15\x15`\xC0Q\x03a\x01\xF5WZ`\xE0R\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x1C.W`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x0Eba@HV[a\x0Eja?CV[a\x0Ex`\x04\x805\x01\x80a?\x97V[\x80\x91P\x15a\x1C\0W\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0T`d`\x045\x015\x14\x80\x15a\x1B\xD1W[\x15a\x1B\x99Wa\x0E\xBF\x81a@\x03V[`\x80Ra\x0E\xCAa@\x9BV[P`@Qa\x0E\xD7\x81a3\xADV[a\x01 Q\x81Ra\x01 Q` \x82\x01R\x90a\x01 Qa\x01\0Ra\x01 Q[\x81\x81\x10a\x15IW\x82a\x0F>`\x80QQ`\x05\x1B` `\x80Q\x01 \x91a\x0F5a\x0F/a\x0F(`$`\x045\x01`\x045`\x04\x01a@\xE6V[6\x91a45V[\x84aQ\xE8V[\x90\x93\x91\x93aR\"V[` \x81Q\x91\x01Q\x90a\x01 QR` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@a\x01 Q \x16\x91\x16\x90\x80\x82\x03a\x15\x15W\x82a\x0F\x85`\x04\x805\x01\x80a?\x97V[`@\x92\x91\x92Q\x90\x80`\x80\x83\x01\x7F@l`\xF8z[\xB5B\xA7\xFCs\x01\xBA\\\x01\xFEw$\xB5\xB3\xC9\xE35!M\t*\x10\xB4\x05\xF5\xA0` \x85\x01R`d`\x045\x015`@\x85\x01R``\x80\x85\x01RR`\xA0\x82\x01\x90`\xA0\x81`\x05\x1B\x84\x01\x01\x94\x80\x92a\x01 Q\x91[\x83\x83\x10a\x12\xF5W\x86` \x87a\x0F\xFF\x81\x8C\x03`\x1F\x19\x81\x01\x83R\x82a3\xF6V[\x81`@Q\x91\x80Q\x91\x82\x91\x01\x83^\x81\x01\x90a\x01 Q\x82R\x80a\x01 Q\x92\x03\x90`\x02Z\xFA\x15a\x12\xB3Wa\x01 QQa\x10?`D`\x045\x01`\x045`\x04\x01a@\xE6V[\x80`\x04\x93\x92\x93\x11a\x01\xF5W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x835\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x12\xC1WPP`\xC0Q\x15a\x11\xD4W[PPPa\x01\0Qa\x11\\W[\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x11+Z`\xE0Qa:2V[\x7Fo\x14\x981\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[a\x11ha\x01\0QaO\xAAV[\x15a\x11\x9FW\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Qa\x01\0Q\x81R\xA1a\x10\xCFV[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QRa\x01\0Q`\x04R`$a\x01 Q\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x01\xF5Wa\x12O`@Q\x94\x85\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91aE\x8BV[\x92\x7F\x85\x8B\xE2>\xCB\xD2Kp\xEF\xDF\xAC\xAD\xA1\x1C/\x04q\xF39\x82\xE37X\xB8\x86\x1E]F%v\xDCF`$\x84\x01R`D\x83\x01R\x81\x80a\x01 Q\x94\x03\x91Z\xFA\x80\x15a\x12\xB3Wa\x12\x98W[\x80\x80a\x10\xC3V[a\x01 Qa\x12\xA5\x91a3\xF6V[a\x01 Qa\x01\xF5W\x81a\x12\x91V[`@Q=a\x01 Q\x82>=\x90\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[\x90\x91\x92\x93\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x86\x82\x03\x01\x83R\x875\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x82\x12\x15a\x01\xF5W`\xA0\x81\x01\x91\x83\x90a\x13c\x81\x83\x01\x80aJ\xD8V[\x80\x95`\xA0\x86RR`\xC0\x84\x01`\xC0\x86`\x05\x1B\x86\x01\x01\x95\x82a\x01 Q[\x82\x81\x10a\x14\x8EWPPPPPa\x13\x9B` \x83\x83\x01\x01\x83\x83\x01aJ\xD8V[\x94\x90\x84\x82\x03` \x86\x01R\x85\x82R` \x82\x01\x90` \x87`\x05\x1B\x84\x01\x01\x96\x81\x93a\x01 Q\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x14(WPPPPPP\x91\x01`@\x81\x81\x015\x90\x84\x01R``\x80\x82\x015\x90\x84\x01R`\x80\x90\x81\x015\x92\x01\x91\x90\x91RP\x96` \x90\x81\x01\x94\x93`\x01\x01\x92\x01\x90a\x0F\xE1V[\x91\x93\x95\x99\x90\x92\x94\x96\x97P`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x01\xF5W` a\x14z`\x01\x93``a\x14l\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90aK+V[\x91\x81`@\x82\x01R\x01\x90aLLV[\x9B\x01\x97\x01\x91\x01\x91\x8A\x97\x96\x95\x93\x91\x94\x92a\x13\xE5V[\x91\x93\x97`\x01\x91\x93\x95\x96P` a\x15\x03\x82`\x80a\x14\xF5a\x14\xD5\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F\x88\x99\x03\x01\x8DR\x8AaK+V[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90aK+V[\x91\x81``\x82\x01R\x01\x90aLLV[\x99\x01\x95\x01\x91\x01\x91\x88\x95\x94\x93\x91\x92a\x13~V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[a\x15W`\x04\x805\x01\x80a?\x97V[\x90`\xA0R\x81\x10\x15a\x0BUW\x80`\x05\x1B`\xA0Q\x015\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa`\xA0Q6\x03\x01\x84\x12\x15a\x01\xF5Wa\x01 Q`\xA0Q\x85\x90a\x15\xAF\x90\x82\x01\x80a?\x97V[\x90Pa\x15\xBA\x81a@\x03V[a\x15\xC3\x82a@\x03V[\x91a\x01 Q[\x81\x81\x10a\x1AVWPP`\xA0Qa\x15\xE5\x90\x84\x01` \x81\x01\x90a?\x97V[\x92\x90Pa\x15\xF1\x83a@\x03V[\x91a\x15\xFB\x84a@\x03V[\x93a\x01 Q\x90[\x80\x82\x10a\x17\x91WPP\x92a\x16z\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a\x16la\x16\x88\x94a\x16^`\x80\x98`@Q\x99\x8A\x99`\xA0Q\x01\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a4\x89V[\x90\x87\x82\x03`@\x89\x01Ra4\x89V[\x90\x85\x82\x03``\x87\x01Ra4\x89V[\x90\x83\x82\x03`\x80\x85\x01Ra4\x89V[\x03\x90\xA1\x80a\x17\x87W[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x85`\xA0Q\x016\x03\x01\x12a\x01\xF5W`@Q\x90a\x16\xCF\x82a3\xADV[`@\x85`\xA0Q\x01\x015\x82R` \x82\x01``\x86`\xA0Q\x01\x015\x81R`@Q\x92a\x16\xF6\x84a3\xADV[_\x84R_` \x85\x01Ra\x17\x0C\x81Q\x83Q\x90aG\xE6V[\x15a\x17OW\x82`\x01\x95\x94\x92a\x17.\x92` `\x80\x96Q\x93\x01Q\x90Q\x91Q\x92aHHV[` \x83\x01R\x81R\x94`\xA0Q\x01\x015a\x17H\x82`\x80Qa@4V[R\x01a\x0E\xF4V[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[a\x01\0R\x84a\x16\x91V[\x91\x92\x93\x94\x95a\x17\xAE\x91\x97P` \x90`\xA0Q\x01\x01\x8B`\xA0Q\x01a?\x97V[\x87\x91\x97\x10\x15a\x0BUW\x80`\x05\x1B\x87\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x886\x03\x01\x81\x12\x15a\x01\xF5Wa\x17\xF9` \x82\x8A\x01\x015aB\xC7V[`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a\x0C\xDCW`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x82\x8A\x015\x91a\x01 Q[\x82\x81\x10a\x19\x95WPP\x91` \x91`\x01\x94\x93\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x86`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\x19;W[P\x99a\x19\x12a\x19\t`@\x84\x84\x01\x01a\x19\x01a\x18\xF6\x82\x87\x87\x01a@\xB3V[\x87\x87\x87\x01\x015aCxV[\x84\x84\x01a@\xB3V[\x83\x83\x015aE\xC2V[\x81\x81\x015a\x19 \x85\x8Ba@4V[R\x01\x015a\x19.\x82\x89a@4V[R\x01\x8A\x95\x94\x93\x92\x91a\x16\x02V[a\x19\x8Aa\x19\x83a\x19\x8F\x92a\x19N\x85a;\xA7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR\x86a\x01 Q \x01T\x80\x94aQ\xA5V[\x92\x80aQ\xA5V[a<@V[\x8Fa\x18\xD9V[\x90\x92`\x01\x90\x81\x85\x16a\x1A\x17Wa\x1A\x0C\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x80\x84` a\x01 Q \x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR\x83` a\x01 Q \x01T\x90aQ\xA5V[\x93[\x81\x1C\x91\x01a\x18yV[a\x1AP\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x83` a\x01 Q \x01TaQ\xA5V[\x93a\x1A\x0EV[\x90\x91\x92\x93a\x1Al\x90`\xA0Q\x01\x8A`\xA0Q\x01a?\x97V[\x82\x10\x15a\x0BUWa\x1A\x83\x90\x82`\x05\x1B\x81\x01\x90a@\xB3V[a\x1A\x90` \x82\x015aBUV[`@\x81\x015a\x1A\xC9\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a\x1BhWPa\x1A\xD9\x815aN\xF5V[\x15a\x1B6W\x90` \x82a\x1B\x11a\x1B\n```\x01\x96\x01a\x1B\x04a\x1A\xFB\x82\x86a@\xB3V[\x86\x86\x015aCxV[\x83a@\xB3V[\x825aE\xC2V[\x805a\x1B\x1D\x84\x88a@4V[R\x015a\x1B*\x82\x87a@4V[R\x01\x90\x88\x93\x92\x91a\x15\xC9V[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR5`\x04R`$a\x01 Q\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[\x7F\xD6\xD8\x89\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`d`\x045\x015`\x04R`$a\x01 Q\xFD[P\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U`d`\x045\x015\x14a\x0E\xB1V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x01\xF5Wa\x01 Q`\x03\x196\x01\x12a\x01\xF5W` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[4a\x01\xF5W` `\x03\x196\x01\x12a\x01\xF5W\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`\x045\x90\x81\x10\x15a\x0BUW` \x90\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a\x01 QR\x81a\x01 Q \x01a\x01 QPTa\x01 Q`\x03\x1B\x1C`@Q\x90\x81R\xF3[4a!\x93W` `\x03\x196\x01\x12a!\x93Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a!\x93W`\x80`\x03\x19`\x0456\x03\x01\x12a!\x93W\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a*\x07W`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x1D\xA6a@HV[a\x1D\xAEa?CV[a\x1D\xBC`\x04\x805\x01\x80a?\x97V[\x80\x91P\x15a)\xDFW\x7FTv\\\xAC,\xB30\xB1.\x848TIo>\x05\xFC\xF8\nz\x08\xEE\xC8!6!wa\x91\x85\x19\0T`d`\x045\x015\x14\x80\x15a)\xB0W[\x15a)~Wa\x1E\x03\x81a@\x03V[\x90a\x1E\x0Ca@\x9BV[P`@Qa\x1E\x19\x81a3\xADV[_\x81R_` \x82\x01R\x91_\x91_[\x81\x81\x10a#\x94WPP\x80` a\x1E_\x92Q`\x05\x1B\x91\x01 \x92a\x0F5a\x1EYa\x0F(`$`\x045\x01`\x045`\x04\x01a@\xE6V[\x85aQ\xE8V[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a#fWPa\x1E\xA2\x90P`\x04\x805\x01\x80a?\x97V[`@Q\x90` \x82\x01\x92\x81`\x80\x84\x01\x7F@l`\xF8z[\xB5B\xA7\xFCs\x01\xBA\\\x01\xFEw$\xB5\xB3\xC9\xE35!M\t*\x10\xB4\x05\xF5\xA0\x86R`d`\x045\x015`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01`\xA0\x83`\x05\x1B\x85\x01\x01\x92\x82_\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01[\x83\x83\x10a!\xC5W\x8A\x8A` _\x8C\x8Ca\x1F>\x81\x8E\x03`\x1F\x19\x81\x01\x83R\x82a3\xF6V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a!\x88W_Qa\x1Fo`D`\x045\x01`\x045`\x04\x01a@\xE6V[\x80`\x04\x11a!\x93W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a!\x97WPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a!\x93W_\x92a i\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91aE\x8BV[\x90\x7F\x85\x8B\xE2>\xCB\xD2Kp\xEF\xDF\xAC\xAD\xA1\x1C/\x04q\xF39\x82\xE37X\xB8\x86\x1E]F%v\xDCF`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a!\x88Wa!sW[P\x80a!\x03W[P\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x01 Q\x80\xF3[a!\x0C\x81aO\xAAV[\x15a!BW` \x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x91`@Q\x90\x81R\xA1\x81a \xABV[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[_a!}\x91a3\xF6V[_a\x01 R\x82a \xA4V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x88\x82\x03\x01\x86R\x865\x82\x81\x12\x15a!\x93W\x83\x01\x90`\xA0\x81\x01\x91a\"\x0E\x81\x80aJ\xD8V[\x80\x94`\xA0\x85RR`\xC0\x83\x01`\xC0\x85`\x05\x1B\x85\x01\x01\x94\x82_[\x82\x81\x10a#\x15WPPPPPa\"?` \x82\x01\x82aJ\xD8V[\x93\x90\x83\x82\x03` \x85\x01R\x84\x82R` \x82\x01\x90` \x86`\x05\x1B\x84\x01\x01\x95\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\"\xC5WPPPP`@\x80\x85\x015\x90\x86\x01RPPP``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01R\x95` \x90\x81\x01\x95\x01\x93\x92`\x01\x01\x91\x90a\x1F\x1DV[\x90\x91\x92\x93\x94\x95\x98`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a!\x93W` a#\x06`\x01\x93``a\x14l\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90aK+V[\x9B\x01\x97\x01\x95\x94\x93\x92\x91\x01a\"\x86V[\x90\x91\x92\x93\x96` \x80a#Y\x83`\x80a\x14\xF5a\x14\xD5\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F`\x01\x9A\x03\x01\x8DR\x8AaK+V[\x99\x01\x95\x01\x93\x92\x91\x01a\"&V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[a#\xA2`\x04\x805\x01\x80a?\x97V[\x82\x10\x15a(<W\x81`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a!\x93W\x01_\x95a#\xEA\x82\x80a?\x97V[\x97\x90Pa#\xF6\x88a@\x03V[\x97a$\0\x81a@\x03V[\x90_[\x81\x81\x10a(iWPPa$\x19` \x85\x01\x85a?\x97V[\x90Pa$$\x81a@\x03V[a\x01@Ra$1\x81a@\x03V[\x90_\x90[\x80\x82\x10a%gWPP\x98\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x91a$\xAEa$\x9D\x9Ba\x16z`\x80\x89\x015\x9D\x8E\x94a$\x8F`@Q\x97\x88\x97\x88R`\xA0` \x89\x01R`\xA0\x88\x01\x90a4\x89V[\x90\x86\x82\x03`@\x88\x01Ra4\x89V[\x84\x81\x03``\x86\x01Ra\x01@Qa4\x89V[\x03\x90\xA1\x80a%_W[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x836\x03\x01\x12a!\x93W`@Q\x91a$\xF1\x83a3\xADV[`@\x81\x015\x83R``` \x84\x01\x91\x015\x81R`@Q\x92a%\x10\x84a3\xADV[_\x84R_` \x85\x01Ra%&\x81Q\x83Q\x90aG\xE6V[\x15a\x17OW\x91a%F\x91`\x01\x95\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92aHHV[` \x83\x01R\x81R\x95a%X\x82\x86a@4V[R\x01a\x1E'V[\x95P\x87a$\xB7V[\x90\x93Pa%w` \x87\x01\x87a?\x97V[\x85\x91\x95\x10\x15a(<W\x80`\x05\x1B\x85\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x866\x03\x01\x81\x12\x15a!\x93Wa%\xC2` \x82\x88\x01\x015aB\xC7V[`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a(\x0FW`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x82\x88\x015\x91_[\x82\x81\x10a'+WPP\x91` \x91`\x01\x94\x93\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x86`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a&\xE3W[P\x97a&\xBCa\x19\t`@\x84\x84\x01\x01a\x19\x01a\x18\xF6\x82\x87\x87\x01a@\xB3V[\x81\x81\x015a&\xCD\x85a\x01@Qa@4V[R\x01\x015a&\xDB\x82\x86a@4V[R\x01\x90a$5V[a\x19\x8Aa\x19\x83a'%\x92a&\xF6\x85a;\xA7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x86_ \x01T\x80\x94aQ\xA5V[_a&\x9FV[\x90\x92`\x01\x90\x81\x85\x16a'\xB8W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta'\xAD\x91aQ\xA5V[\x93[\x81\x1C\x91\x01a&?V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta(\t\x91\x90aQ\xA5V[\x93a'\xAFV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a(s\x86\x80a?\x97V[\x82\x10\x15a(<Wa(\x8A\x90\x82`\x05\x1B\x81\x01\x90a@\xB3V[` \x81\x015\x90a(\x99\x82aBUV[`@\x81\x015a(\xD2\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a)SWPa(\xE2\x815aN\xF5V[\x15a)'W\x90\x8Ca)\x15\x84\x84a)\x0Ea\x1B\n```\x01\x99\x98\x01a\x1B\x04a)\x08\x82\x86a@\xB3V[\x89aCxV[5\x92a@4V[Ra) \x82\x86a@4V[R\x01a$\x03V[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R5`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xD6\xD8\x89\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`d`\x045\x015`\x04R`$_\xFD[P\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U`d`\x045\x015\x14a\x1D\xF5V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa*Ga?CV[` \x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01T`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa*\xDDa>\xD7V[a*\xE5a@HV[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa+ta>\xD7V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a!\x93W_`\x03\x196\x01\x12a!\x93W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93W` a\x04\xD9a8\x96V[4a!\x93W_`\x03\x196\x01\x12a!\x93W` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a-vW` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a!\x93W_`\x03\x196\x01\x12a!\x93W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x90_[\x81\x81\x10a.\x1CWa\x02_\x85a\x03g\x81\x87\x03\x82a3\xF6V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a.\x05V[`@`\x03\x196\x01\x12a!\x93Wa.Fa3\x8AV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93Wa.f\x906\x90`\x04\x01a4kV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a0\xC0W[Pa-vWa.\xB6a>\xD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a0\x8CW[Pa/6W\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a0aWP\x82;\x15a06W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a0\x05Wa0\x03\x91aA\x8EV[\0[PP4a0\x0EW\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a0\xB8W[\x81a0\xA8` \x93\x83a3\xF6V[\x81\x01\x03\x12a!\x93WQ\x90\x85a/\x05V[=\x91Pa0\x9BV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a.\xA9V[4a!\x93W_`\x03\x196\x01\x12a!\x93W` \x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`@Q\x90\x81R\xF3[4a!\x93W_`\x03\x196\x01\x12a!\x93Wa1Va>\xD7V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x81\x16\x15a1\xD3W`\xFF\x19\x16\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a!\x93W_`\x03\x196\x01\x12a!\x93W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16\x90\x81\x15a3VW[Pa3.W\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x02\x17\x90Ua2\x9Ba5\x18V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x02\x81R\xA1\0[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x81a2<V[4a!\x93W` `\x03\x196\x01\x12a!\x93W` a\x0C\xCD`\x045a:?V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a!\x93WV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3\xC9W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a3\xC9W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3\xC9W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a4A\x82a4\x19V[\x91a4O`@Q\x93\x84a3\xF6V[\x82\x94\x81\x84R\x81\x83\x01\x11a!\x93W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a!\x93W\x81` a4\x86\x935\x91\x01a45V[\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a4\xA6WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4\x99V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`\x04\x11\x15a4\xEBWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a5 a>\xD7V[`\xFF\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16a8\x94W\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0T_[\x81\x81\x10a7\xF7WPP\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\x02T_[\x81\x81\x10a5\xF0WPP`\x01`\xFF\x19\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16\x17\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02UV[\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\x02T\x81\x10\x15a(<W\x7F\x97i\xA3\xC2\xA2\xCE,i>\x1C\x88\x9Ec\xA7\xE6/s\x92\x186{\xBCZ\xA6\xFA\xED2\xD6\xA7\xCF\xB3\xE9\x81\x01T_\x81\x81R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@\x90 T\x90\x91\x90`\xFF\x16a6y\x81a4\xE1V[\x15a6\x89W[`\x01\x91P\x01a5\x9AV[\x81\x15a7\xCFW\x81_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@_ T\x91`\xFF\x83\x16\x92a6\xCA\x84a4\xE1V[\x83`\x01\x14\x15\x90\x81a7\xAFW[P\x15a7rW_\x81\x81R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@\x90 `\x01`\xFF\x19\x82T\x16\x17\x90Ua7\x1C\x83a4\xE1V[\x82\x15a7dW[`@Q\x92a70\x81a4\xE1V[\x83R\x7F\x14p\x94\x8B28g\x03\xE3um\x8E\x14\xD5\x1F\xAD;\xAF\x8F%\xD8$\x8C$\x8D\xF7\xAFS`\xE3\x80\xC5`@\x84`\x01` \x81\x97\x01R\xA2a6\x7FV[a7m\x81a<\xC3V[a7#V[\x7F\xEE\x80\xEF\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RPa7\xA2\x81a4\xE1V[`$R`\x01`DR`d_\xFD[\x90P`\x01\x84\x91a7\xBE\x83a4\xE1V[a7\xC7\x83a4\xE1V[\x16\x14_a6\xD6V[\x7Fz\xF6#A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0T\x81\x10\x15a(<W\x7F\xB2\xF9\xF8Ya\xE1\x8E~-3.\xAA\x05q\xB4\x88\xC6\xAC\xBA\x1C\xA6jk,\xD2\xD4\x19d\xCA\xC6p\xF8\x81\x01T_\x81\x81R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\x03` R`@\x90 T`\x01\x92\x91a8\x86\x91\x15a8\x8CW`\x03\x90a=hV[\x01a5mV[`\x02\x90a=hV[V[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a!\x88W_\x91a9\xE2W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a!\x88W_\x91a9\xAAWP\x90V[\x90P` \x81=` \x11a9\xDAW[\x81a9\xC5` \x93\x83a3\xF6V[\x81\x01\x03\x12a!\x93WQ\x80\x15\x15\x81\x03a!\x93W\x90V[=\x91Pa9\xB8V[\x90P` \x81=` \x11a:*W[\x81a9\xFD` \x93\x83a3\xF6V[\x81\x01\x03\x12a!\x93WQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a!\x93W` a9QV[=\x91Pa9\xF0V[\x91\x90\x82\x03\x91\x82\x11a(\x0FWV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x81\x10\x15a(<W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03_R` _ \x01\x90_\x90V[\x80T\x82\x10\x15a(<W_R` _ \x01\x90_\x90V[\x91\x90\x81\x10\x15a(<W`\x06\x1B\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a;{Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9W\x80`\x01a<*\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a:\x95V[_\x19\x82\x93\x92T\x91`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90UV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9W\x80`\x01a<*\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a:\x95V[\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9W\x80`\x01a<*\x92\x01\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01U\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x01a:\x95V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a3\xC9Wa<*\x91`\x01\x82\x01\x81Ua:\x95V[\x90\x81\x15a7\xCFW\x81_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`@_ T\x90`\xFF\x82\x16\x91a=\xAA\x83a4\xE1V[a=\xB3\x82a4\xE1V[\x82\x80\x83\x14\x15\x91\x82a>\xACW[PP\x15a>hW\x7F\x14p\x94\x8B28g\x03\xE3um\x8E\x14\xD5\x1F\xAD;\xAF\x8F%\xD8$\x8C$\x8D\xF7\xAFS`\xE3\x80\xC5\x91`@\x91\x84_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R\x82_ a>\x1E\x82a4\xE1V[`\xFF\x19\x81T\x16`\xFF\x83\x16\x17\x90Ua>4\x82a4\xE1V[\x81\x15a>ZW[\x82Q\x91a>G\x81a4\xE1V[\x82Ra>R\x81a4\xE1V[` \x82\x01R\xA2V[a>c\x85a<\xC3V[a>;V[\x91\x7F\xEE\x80\xEF\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Ra>\x98\x81a4\xE1V[`$Ra>\xA4\x81a4\xE1V[`DR`d_\xFD[`\xFF\x91\x92Pa>\xBA\x84a4\xE1V[a>\xC3\x83a4\xE1V[a>\xCC\x83a4\xE1V[\x83\x16\x16\x14\x82_a=\xBFV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a?\x17WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\x02T\x16\x15a?oWV[\x7F'\xA4\x136\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a!\x93W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a!\x93WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a3\xC9W`\x05\x1B` \x01\x90V[\x90a@\r\x82a?\xEBV[a@\x1A`@Q\x91\x82a3\xF6V[\x82\x81R`\x1F\x19a@*\x82\x94a?\xEBV[\x01\x90` 6\x91\x017V[\x80Q\x82\x10\x15a(<W` \x91`\x05\x1B\x01\x01\x90V[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a@sWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@Q\x90a@\xA8\x82a3\xADV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a!\x93W\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a!\x93W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W` \x01\x91\x816\x03\x83\x13a!\x93WV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15aAfWV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80aBBW[\x15aA\xC2WPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15aB\tWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15aB\x1AW`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80aA\xA2WP\x81;\x15\x15aA\xA2V[\x80_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`\x02`@_ TaB\x8F`\xFF\x82\x16a4\xE1V[\x16aB\x97WPV[\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`\x01`$R`D_\xFD[\x80_R\x7F\x95\xDE\x9D$?^\x98\x16\xB0C\xBB\xF3\x87\xB5\x9Fjoj\xE8r\x9B\xED\xCD\xCB\xD7\t\xD1\x1C:3\xE3\0` R`\x01`@_ TaC\x01`\xFF\x82\x16a4\xE1V[\x16aC\tWPV[\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R_`$R`D_\xFD[\x91\x90\x81\x10\x15a(<W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a!\x93W\x01\x90V[\x90`@\x81\x01aC\x87\x81\x83a?\x97V[\x92\x90P_[\x83\x81\x10aC\x9AWPPPPPV[aC\xBBaC\xB1\x82aC\xAB\x86\x86a?\x97V[\x90aC8V[` \x81\x01\x90a@\xE6V[\x81\x01``\x82\x82\x03\x12a!\x93W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a!\x93W` \x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93W\x82aD\x08\x91\x83\x01a4kV[\x91`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93WaD'\x92\x01a4kV[\x90`@Q\x91\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01R`@`$\x84\x01R_\x83\x80aDl`D\x82\x01\x86a4\xBCV[\x03\x81\x83\x88Z\xF1\x92\x83\x15a!\x88W_\x93aE\x0FW[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03aD\xD4WP`\x01\x93\x92\x91\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91aD\xCB`@Q\x92\x83\x92\x83aL\xC3V[\x03\x90\xA2\x01aC\x8CV[\x90PaE\x0B`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01aL\xC3V[\x03\x90\xFD[\x90\x92P=\x80_\x83>aE!\x81\x83a3\xF6V[\x81\x01\x90` \x81\x83\x03\x12a!\x93W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W\x01\x81`\x1F\x82\x01\x12\x15a!\x93W\x80Q\x90aEX\x82a4\x19V[\x92aEf`@Q\x94\x85a3\xF6V[\x82\x84R` \x83\x83\x01\x01\x11a!\x93W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_aD\x80V[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a4\x86\x94\x92\x81R\x81` \x82\x01R\x01\x91aE\x8BV[\x90aE\xCD\x81\x80a?\x97V[_[\x81\x81\x10aGwWPPPaE\xE6` \x82\x01\x82a?\x97V[_[\x81\x81\x10aG\x08WPPPaE\xFF`@\x82\x01\x82a?\x97V[_[\x81\x81\x10aF\x99WPPP\x80``aF\x19\x92\x01\x90a?\x97V[\x90\x91_[\x82\x81\x10aF*WPPPPV[aF5\x81\x84\x86aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aFPW[\x01aF\x1DV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8CaF\x80aC\xB1\x84\x88\x8AaC8V[\x90aF\x91`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aFJV[aF\xA4\x81\x83\x85aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aF\xBFW[\x01aF\x01V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYaF\xEFaC\xB1\x84\x87\x89aC8V[\x90aG\0`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aF\xB9V[aG\x13\x81\x83\x85aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aG.W[\x01aE\xE8V[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3aG^aC\xB1\x84\x87\x89aC8V[\x90aGo`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aG(V[aG\x82\x81\x83\x85aC8V[5\x90`\x02\x82\x10\x15a!\x93W`\x01\x80\x92\x14aG\x9DW[\x01aE\xCFV[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6aG\xCDaC\xB1\x84\x87\x89aC8V[\x90aG\xDE`@Q\x92\x83\x92\x87\x84aE\xABV[\x03\x90\xA2aG\x97V[\x80\x15\x80\x15aH8W[\x80\x15aH0W[\x80\x15aH W[aH\x1AWd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15aG\xFDV[P\x81\x15aG\xF6V[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15aG\xEFV[\x92\x93\x92\x90\x91_\x90\x80\x83\x03aJ\xBEWPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08aHqWPP\x90P_\x90_\x90V[d\x01\0\0\x03\xD0\x19\x80\x80\x80\x85\x80\t\x81\x80\x80\x80\x80`\x01\x80\t\x98\x81\x80\x80\x80\x80\x80\x8B\x87\t`\x04\t\x9D\x80\t_\t\x92\x80\t`\x03\t\x08\x81\x80\x80\x8B\x80\x08aH\xB0\x90\x82a:2V[\x81\x84\x80\t\x08\x99aH\xC0\x8B\x83a:2V[\x90\x08\x90\t\x92\x80\t`\x08\taH\xD4\x90\x83a:2V[\x90\x08\x93`\x01\x90\t`\x02\t\x91\x90[\x82\x15\x15\x83\x81aJ\x80W[P\x80aJxW[\x15aJ\x1AW\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15aI\xC7W\x80\x84\x04\x96\x80\x98aI\x9AWd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a(\x0FWd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15aImW\x90aIc\x91a:2V[\x95\x92\x93\x96\x95aI\x0BV[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80aI\xEDWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01aH\xF2V[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_aH\xEBV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[aJ\xD2\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96aL\xE8V[\x91aH\xE1V[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a!\x93W\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a!\x93W\x81`\x05\x1B6\x03\x83\x13a!\x93WV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a!\x93W\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x826\x03\x01\x90[\x85\x84\x10aK\xAAWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`\x1F\x19\x82\x82\x03\x01\x86R\x875\x83\x81\x12\x15a!\x93W\x84\x01\x805`\x02\x81\x10\x15a!\x93W\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a!\x93W\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\x93W\x806\x03\x82\x13a!\x93WaL=` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91aE\x8BV[\x99\x01\x97\x96\x01\x94\x01\x92\x91\x90aK\x99V[a4\x86\x91aL\xB5aL\xAAaL\x8FaLtaLf\x86\x80aJ\xD8V[`\x80\x87R`\x80\x87\x01\x91aK]V[aL\x81` \x87\x01\x87aJ\xD8V[\x90\x86\x83\x03` \x88\x01RaK]V[aL\x9C`@\x86\x01\x86aJ\xD8V[\x90\x85\x83\x03`@\x87\x01RaK]V[\x92``\x81\x01\x90aJ\xD8V[\x91``\x81\x85\x03\x91\x01RaK]V[\x90\x91aL\xDAa4\x86\x93`@\x84R`@\x84\x01\x90a4\xBCV[\x91` \x81\x84\x03\x91\x01Ra4\xBCV[\x94\x92\x90\x93\x91\x85\x15\x80aN\xEDW[aN\xE2W\x80\x15\x80aN\xDAW[aN\xD2W`@Q`\x80\x91aM\x15\x83\x83a3\xF6V[\x826\x837\x84\x15aJ\x91W\x84`\x01\x80\t\x80\x83R\x93\x85\x85`\x01\t\x98` \x84\x01\x99\x8AR`@\x84\x01\x92\x86\x84R\x87\x84Q`\x01\t\x93``\x86\x01\x94\x85R`@Q\x9A\x87\x8C\x01\x95\x8C\x87\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x11\x17a3\xC9W\x8A\x80\x97\x95\x81\x96\x94\x82\x95`@RQ\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90aN\xC6W[\x15aNhW\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85aM\xC0\x8F\x97\x88a3\xF6V[6\x877Q\x8CQaM\xD0\x90\x83a:2V[\x90\x08\x84RQ\x85QaM\xE1\x90\x83a:2V[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88QaN\x1E\x90\x83a:2V[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\taN5\x90\x83a:2V[\x90\x08\x9CQ\x93Q\x90Q\x90\taNI\x8C\x83a:2V[\x90\x08\x90\t\x92Q\x90Q\x90\taN]\x90\x83a:2V[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15aM\xA4V[PPP`\x01\x90V[P\x81\x15aM\x01V[\x94P\x92P`\x01\x91\x90PV[P\x84\x15aL\xF5V[\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ T\x15_\x14aO\xA5WaOR\x81\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a=FV[\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T\x90_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ U`\x01\x90V[P_\x90V[\x80_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15_\x14aO\xA5WaP\x07\x81\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a=FV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x90_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ U`\x01\x90V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nTaQ\xA1WaQ\x11\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a=FV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nU`\x01\x90V[_\x90V[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81RaQ\xC8``\x82a3\xF6V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a!\x88W_Q\x90V[\x81Q\x91\x90`A\x83\x03aR\x18WaR\x11\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aR\xE9V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[aR+\x81a4\xE1V[\x80aR4WPPV[aR=\x81a4\xE1V[`\x01\x81\x03aRmW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[aRv\x81a4\xE1V[`\x02\x81\x03aR\xAAWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x90aR\xB6\x81a4\xE1V[\x14aR\xBEWPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aSmW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a!\x88W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aScW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V\xFE\xA1dsolcC\0\x08%\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AddressEmptyCode(address)` and selector `0x9996b315`.
```solidity
error AddressEmptyCode(address target);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressEmptyCode {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddressEmptyCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressEmptyCode) -> Self {
                (value.target,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressEmptyCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { target: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressEmptyCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressEmptyCode(address)";
            const SELECTOR: [u8; 4] = [153u8, 150u8, 179u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DeltaMismatch(address,address)` and selector `0xe6d44b4c`.
```solidity
error DeltaMismatch(address expected, address actual);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeltaMismatch {
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DeltaMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: DeltaMismatch) -> Self {
                (value.expected, value.actual)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DeltaMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    actual: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DeltaMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DeltaMismatch(address,address)";
            const SELECTOR: [u8; 4] = [230u8, 212u8, 75u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expected,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.actual,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`.
```solidity
error ERC1967InvalidImplementation(address implementation);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967InvalidImplementation {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967InvalidImplementation>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967InvalidImplementation) -> Self {
                (value.implementation,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC1967InvalidImplementation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { implementation: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967InvalidImplementation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967InvalidImplementation(address)";
            const SELECTOR: [u8; 4] = [76u8, 156u8, 140u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.implementation,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
```solidity
error ERC1967NonPayable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967NonPayable> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967NonPayable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967NonPayable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967NonPayable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967NonPayable()";
            const SELECTOR: [u8; 4] = [179u8, 152u8, 151u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyTransactionNotAllowed()` and selector `0xcf1b093c`.
```solidity
error EmptyTransactionNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyTransactionNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyTransactionNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyTransactionNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyTransactionNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyTransactionNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyTransactionNotAllowed()";
            const SELECTOR: [u8; 4] = [207u8, 27u8, 9u8, 60u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
```solidity
error EnforcedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
```solidity
error ExpectedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedCall()` and selector `0xd6bda275`.
```solidity
error FailedCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedCall;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedCall> for UnderlyingRustTuple<'_> {
            fn from(value: FailedCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedCall()";
            const SELECTOR: [u8; 4] = [214u8, 189u8, 162u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ForwarderCallOutputMismatch(bytes,bytes)` and selector `0xc504fada`.
```solidity
error ForwarderCallOutputMismatch(bytes expected, bytes actual);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ForwarderCallOutputMismatch {
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ForwarderCallOutputMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: ForwarderCallOutputMismatch) -> Self {
                (value.expected, value.actual)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ForwarderCallOutputMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    actual: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ForwarderCallOutputMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ForwarderCallOutputMismatch(bytes,bytes)";
            const SELECTOR: [u8; 4] = [197u8, 4u8, 250u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.expected,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.actual,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
```solidity
error InvalidInitialization();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidInitialization> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInitialization) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInitialization {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInitialization {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInitialization()";
            const SELECTOR: [u8; 4] = [249u8, 46u8, 232u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidLogicRefPolicyTransition(bytes32,uint8,uint8)` and selector `0xee80efae`.
```solidity
error InvalidLogicRefPolicyTransition(bytes32 logicRef, ILogicRefPolicyRegistry.LogicRefPolicy previousPolicy, ILogicRefPolicyRegistry.LogicRefPolicy newPolicy);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLogicRefPolicyTransition {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousPolicy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub newPolicy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            ILogicRefPolicyRegistry::LogicRefPolicy,
            ILogicRefPolicyRegistry::LogicRefPolicy,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
            <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLogicRefPolicyTransition>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLogicRefPolicyTransition) -> Self {
                (value.logicRef, value.previousPolicy, value.newPolicy)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidLogicRefPolicyTransition {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    logicRef: tuple.0,
                    previousPolicy: tuple.1,
                    newPolicy: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLogicRefPolicyTransition {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLogicRefPolicyTransition(bytes32,uint8,uint8)";
            const SELECTOR: [u8; 4] = [238u8, 128u8, 239u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                    <ILogicRefPolicyRegistry::LogicRefPolicy as alloy_sol_types::SolType>::tokenize(
                        &self.previousPolicy,
                    ),
                    <ILogicRefPolicyRegistry::LogicRefPolicy as alloy_sol_types::SolType>::tokenize(
                        &self.newPolicy,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `LogicRefPoliciesNotInitialized()` and selector `0x27a41336`.
```solidity
error LogicRefPoliciesNotInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LogicRefPoliciesNotInitialized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LogicRefPoliciesNotInitialized>
        for UnderlyingRustTuple<'_> {
            fn from(value: LogicRefPoliciesNotInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LogicRefPoliciesNotInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LogicRefPoliciesNotInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LogicRefPoliciesNotInitialized()";
            const SELECTOR: [u8; 4] = [39u8, 164u8, 19u8, 54u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NonExistingRoot(bytes32)` and selector `0xf9849ea3`.
```solidity
error NonExistingRoot(bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonExistingRoot {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonExistingRoot> for UnderlyingRustTuple<'_> {
            fn from(value: NonExistingRoot) -> Self {
                (value.root,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonExistingRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { root: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonExistingRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonExistingRoot(bytes32)";
            const SELECTOR: [u8; 4] = [249u8, 132u8, 158u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
```solidity
error NotInitializing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotInitializing> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitializing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializing()";
            const SELECTOR: [u8; 4] = [215u8, 230u8, 188u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PointNotOnCurve((uint256,uint256))` and selector `0xb8a0e8a1`.
```solidity
error PointNotOnCurve(IProtocolAdapter.Delta point);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PointNotOnCurve {
        #[allow(missing_docs)]
        pub point: <IProtocolAdapter::Delta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (IProtocolAdapter::Delta,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <IProtocolAdapter::Delta as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PointNotOnCurve> for UnderlyingRustTuple<'_> {
            fn from(value: PointNotOnCurve) -> Self {
                (value.point,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PointNotOnCurve {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { point: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PointNotOnCurve {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PointNotOnCurve((uint256,uint256))";
            const SELECTOR: [u8; 4] = [184u8, 160u8, 232u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IProtocolAdapter::Delta as alloy_sol_types::SolType>::tokenize(
                        &self.point,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PreExistingNullifier(bytes32)` and selector `0x39a940c5`.
```solidity
error PreExistingNullifier(bytes32 nullifier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PreExistingNullifier {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PreExistingNullifier> for UnderlyingRustTuple<'_> {
            fn from(value: PreExistingNullifier) -> Self {
                (value.nullifier,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PreExistingNullifier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { nullifier: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PreExistingNullifier {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PreExistingNullifier(bytes32)";
            const SELECTOR: [u8; 4] = [57u8, 169u8, 64u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifier),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PreExistingRoot(bytes32)` and selector `0xdb788c2b`.
```solidity
error PreExistingRoot(bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PreExistingRoot {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PreExistingRoot> for UnderlyingRustTuple<'_> {
            fn from(value: PreExistingRoot) -> Self {
                (value.root,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PreExistingRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { root: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PreExistingRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PreExistingRoot(bytes32)";
            const SELECTOR: [u8; 4] = [219u8, 120u8, 140u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`.
```solidity
error ReentrancyGuardReentrantCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReentrancyGuardReentrantCall;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReentrancyGuardReentrantCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: ReentrancyGuardReentrantCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ReentrancyGuardReentrantCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReentrancyGuardReentrantCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReentrancyGuardReentrantCall()";
            const SELECTOR: [u8; 4] = [62u8, 229u8, 174u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ResourceWithDeniedLogicRef(bytes32,bool)` and selector `0x597abd20`.
```solidity
error ResourceWithDeniedLogicRef(bytes32 logicRef, bool consumed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ResourceWithDeniedLogicRef {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub consumed: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, bool);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ResourceWithDeniedLogicRef>
        for UnderlyingRustTuple<'_> {
            fn from(value: ResourceWithDeniedLogicRef) -> Self {
                (value.logicRef, value.consumed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ResourceWithDeniedLogicRef {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    logicRef: tuple.0,
                    consumed: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ResourceWithDeniedLogicRef {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ResourceWithDeniedLogicRef(bytes32,bool)";
            const SELECTOR: [u8; 4] = [89u8, 122u8, 189u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.consumed,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `RiscZeroVerifierPaused()` and selector `0x25ae6eaa`.
```solidity
error RiscZeroVerifierPaused();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RiscZeroVerifierPaused;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RiscZeroVerifierPaused> for UnderlyingRustTuple<'_> {
            fn from(value: RiscZeroVerifierPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RiscZeroVerifierPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RiscZeroVerifierPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RiscZeroVerifierPaused()";
            const SELECTOR: [u8; 4] = [37u8, 174u8, 110u8, 170u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `RiscZeroVerifierSelectorMismatch(bytes4,bytes4)` and selector `0x78a2221c`.
```solidity
error RiscZeroVerifierSelectorMismatch(bytes4 expected, bytes4 actual);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RiscZeroVerifierSelectorMismatch {
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<4>,
            alloy::sol_types::sol_data::FixedBytes<4>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<4>,
            alloy::sol_types::private::FixedBytes<4>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RiscZeroVerifierSelectorMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: RiscZeroVerifierSelectorMismatch) -> Self {
                (value.expected, value.actual)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RiscZeroVerifierSelectorMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    actual: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RiscZeroVerifierSelectorMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RiscZeroVerifierSelectorMismatch(bytes4,bytes4)";
            const SELECTOR: [u8; 4] = [120u8, 162u8, 34u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.expected),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.actual),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `Simulated(uint256)` and selector `0x6f149831`.
```solidity
error Simulated(uint256 gasUsed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Simulated {
        #[allow(missing_docs)]
        pub gasUsed: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Simulated> for UnderlyingRustTuple<'_> {
            fn from(value: Simulated) -> Self {
                (value.gasUsed,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Simulated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { gasUsed: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Simulated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Simulated(uint256)";
            const SELECTOR: [u8; 4] = [111u8, 20u8, 152u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUsed),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
```solidity
error UUPSUnauthorizedCallContext();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnauthorizedCallContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnauthorizedCallContext) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnauthorizedCallContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnauthorizedCallContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnauthorizedCallContext()";
            const SELECTOR: [u8; 4] = [224u8, 124u8, 141u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`.
```solidity
error UUPSUnsupportedProxiableUUID(bytes32 slot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnsupportedProxiableUUID {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
                (value.slot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnsupportedProxiableUUID {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { slot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnsupportedProxiableUUID {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnsupportedProxiableUUID(bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 29u8, 73u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnacceptedKindTableCommitment(bytes32)` and selector `0xd6d8891b`.
```solidity
error UnacceptedKindTableCommitment(bytes32 kindTableCommitment);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnacceptedKindTableCommitment {
        #[allow(missing_docs)]
        pub kindTableCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnacceptedKindTableCommitment>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnacceptedKindTableCommitment) -> Self {
                (value.kindTableCommitment,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnacceptedKindTableCommitment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    kindTableCommitment: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnacceptedKindTableCommitment {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnacceptedKindTableCommitment(bytes32)";
            const SELECTOR: [u8; 4] = [214u8, 216u8, 137u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.kindTableCommitment),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroKindTableCommitmentNotAllowed()` and selector `0x774d5de1`.
```solidity
error ZeroKindTableCommitmentNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroKindTableCommitmentNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroKindTableCommitmentNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: ZeroKindTableCommitmentNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ZeroKindTableCommitmentNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroKindTableCommitmentNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroKindTableCommitmentNotAllowed()";
            const SELECTOR: [u8; 4] = [119u8, 77u8, 93u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroLogicRefNotAllowed()` and selector `0x7af62341`.
```solidity
error ZeroLogicRefNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroLogicRefNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroLogicRefNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroLogicRefNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroLogicRefNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroLogicRefNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroLogicRefNotAllowed()";
            const SELECTOR: [u8; 4] = [122u8, 246u8, 35u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroRiscZeroVerifierRouterNotAllowed()` and selector `0xa6d82a02`.
```solidity
error ZeroRiscZeroVerifierRouterNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroRiscZeroVerifierRouterNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroRiscZeroVerifierRouterNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: ZeroRiscZeroVerifierRouterNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ZeroRiscZeroVerifierRouterNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroRiscZeroVerifierRouterNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroRiscZeroVerifierRouterNotAllowed()";
            const SELECTOR: [u8; 4] = [166u8, 216u8, 42u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroRiscZeroVerifierSelectorNotAllowed()` and selector `0xb1b25aaf`.
```solidity
error ZeroRiscZeroVerifierSelectorNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroRiscZeroVerifierSelectorNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroRiscZeroVerifierSelectorNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: ZeroRiscZeroVerifierSelectorNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ZeroRiscZeroVerifierSelectorNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroRiscZeroVerifierSelectorNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroRiscZeroVerifierSelectorNotAllowed()";
            const SELECTOR: [u8; 4] = [177u8, 178u8, 90u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ActionExecuted(bytes32,bytes32[],bytes32[],bytes32[],bytes32[])` and selector `0x4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32`.
```solidity
event ActionExecuted(bytes32 actionTreeRoot, bytes32[] nullifiers, bytes32[] consumedLogicRefs, bytes32[] commitments, bytes32[] createdLogicRefs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActionExecuted {
        #[allow(missing_docs)]
        pub actionTreeRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub nullifiers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub consumedLogicRefs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub commitments: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub createdLogicRefs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ActionExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActionExecuted(bytes32,bytes32[],bytes32[],bytes32[],bytes32[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                76u8, 231u8, 193u8, 16u8, 32u8, 96u8, 190u8, 79u8, 40u8, 178u8, 41u8,
                25u8, 145u8, 64u8, 253u8, 213u8, 17u8, 45u8, 66u8, 20u8, 254u8, 149u8,
                65u8, 188u8, 155u8, 35u8, 84u8, 30u8, 69u8, 146u8, 204u8, 50u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    actionTreeRoot: data.0,
                    nullifiers: data.1,
                    consumedLogicRefs: data.2,
                    commitments: data.3,
                    createdLogicRefs: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionTreeRoot),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifiers),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.consumedLogicRefs),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitments),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdLogicRefs),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ActionExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActionExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActionExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ApplicationPayload(bytes32,uint256,bytes)` and selector `0xa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c`.
```solidity
event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ApplicationPayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ApplicationPayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ApplicationPayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 148u8, 218u8, 196u8, 183u8, 24u8, 72u8, 67u8, 88u8, 63u8, 151u8,
                46u8, 6u8, 120u8, 62u8, 44u8, 59u8, 180u8, 127u8, 79u8, 1u8, 55u8, 184u8,
                223u8, 82u8, 168u8, 96u8, 223u8, 7u8, 33u8, 159u8, 140u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ApplicationPayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ApplicationPayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ApplicationPayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CommitmentTreeRootAdded(bytes32)` and selector `0x0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d2`.
```solidity
event CommitmentTreeRootAdded(bytes32 root);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CommitmentTreeRootAdded {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for CommitmentTreeRootAdded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CommitmentTreeRootAdded(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 45u8, 197u8, 72u8, 237u8, 149u8, 10u8, 204u8, 180u8, 13u8, 93u8,
                120u8, 84u8, 31u8, 57u8, 84u8, 197u8, 225u8, 130u8, 168u8, 236u8, 241u8,
                158u8, 88u8, 26u8, 79u8, 34u8, 99u8, 246u8, 31u8, 89u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { root: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CommitmentTreeRootAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CommitmentTreeRootAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CommitmentTreeRootAdded,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DiscoveryPayload(bytes32,uint256,bytes)` and selector `0x48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f3`.
```solidity
event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DiscoveryPayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DiscoveryPayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "DiscoveryPayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                72u8, 36u8, 56u8, 115u8, 180u8, 117u8, 45u8, 220u8, 180u8, 94u8, 13u8,
                123u8, 17u8, 196u8, 194u8, 102u8, 88u8, 62u8, 94u8, 9u8, 154u8, 11u8,
                121u8, 143u8, 221u8, 156u8, 26u8, 247u8, 212u8, 147u8, 36u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DiscoveryPayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DiscoveryPayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DiscoveryPayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExternalPayload(bytes32,uint256,bytes)` and selector `0x9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd59`.
```solidity
event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExternalPayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExternalPayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ExternalPayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 97u8, 178u8, 144u8, 246u8, 49u8, 9u8, 127u8, 86u8, 39u8, 60u8,
                244u8, 218u8, 244u8, 13u8, 241u8, 255u8, 156u8, 204u8, 51u8, 241u8, 1u8,
                212u8, 100u8, 131u8, 125u8, 161u8, 245u8, 174u8, 24u8, 189u8, 89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExternalPayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExternalPayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExternalPayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ForwarderCallExecuted(address,bytes,bytes)` and selector `0xcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf1`.
```solidity
event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ForwarderCallExecuted {
        #[allow(missing_docs)]
        pub untrustedForwarder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub input: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub output: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ForwarderCallExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ForwarderCallExecuted(address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                205u8, 219u8, 50u8, 122u8, 219u8, 49u8, 254u8, 84u8, 55u8, 223u8, 42u8,
                140u8, 104u8, 48u8, 27u8, 177u8, 58u8, 107u8, 170u8, 228u8, 50u8, 168u8,
                4u8, 131u8, 140u8, 170u8, 246u8, 130u8, 80u8, 106u8, 173u8, 241u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    untrustedForwarder: topics.1,
                    input: data.0,
                    output: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.input,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.output,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.untrustedForwarder.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.untrustedForwarder,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ForwarderCallExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ForwarderCallExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ForwarderCallExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint64)` and selector `0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2`.
```solidity
event Initialized(uint64 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KindTableCommitmentUpdated(bytes32)` and selector `0x902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee6`.
```solidity
event KindTableCommitmentUpdated(bytes32 indexed kindTableCommitment);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KindTableCommitmentUpdated {
        #[allow(missing_docs)]
        pub kindTableCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for KindTableCommitmentUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "KindTableCommitmentUpdated(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                144u8, 45u8, 104u8, 8u8, 12u8, 159u8, 75u8, 247u8, 237u8, 28u8, 146u8,
                103u8, 88u8, 35u8, 106u8, 252u8, 43u8, 64u8, 68u8, 72u8, 127u8, 111u8,
                134u8, 132u8, 53u8, 45u8, 56u8, 8u8, 114u8, 68u8, 174u8, 230u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    kindTableCommitment: topics.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.kindTableCommitment.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.kindTableCommitment,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KindTableCommitmentUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KindTableCommitmentUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &KindTableCommitmentUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LogicRefPolicyChanged(bytes32,uint8,uint8)` and selector `0x1470948b32386703e3756d8e14d51fad3baf8f25d8248c248df7af5360e380c5`.
```solidity
event LogicRefPolicyChanged(bytes32 indexed logicRef, ILogicRefPolicyRegistry.LogicRefPolicy previousPolicy, ILogicRefPolicyRegistry.LogicRefPolicy newPolicy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LogicRefPolicyChanged {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousPolicy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub newPolicy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LogicRefPolicyChanged {
            type DataTuple<'a> = (
                ILogicRefPolicyRegistry::LogicRefPolicy,
                ILogicRefPolicyRegistry::LogicRefPolicy,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "LogicRefPolicyChanged(bytes32,uint8,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                20u8, 112u8, 148u8, 139u8, 50u8, 56u8, 103u8, 3u8, 227u8, 117u8, 109u8,
                142u8, 20u8, 213u8, 31u8, 173u8, 59u8, 175u8, 143u8, 37u8, 216u8, 36u8,
                140u8, 36u8, 141u8, 247u8, 175u8, 83u8, 96u8, 227u8, 128u8, 197u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    logicRef: topics.1,
                    previousPolicy: data.0,
                    newPolicy: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <ILogicRefPolicyRegistry::LogicRefPolicy as alloy_sol_types::SolType>::tokenize(
                        &self.previousPolicy,
                    ),
                    <ILogicRefPolicyRegistry::LogicRefPolicy as alloy_sol_types::SolType>::tokenize(
                        &self.newPolicy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.logicRef.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.logicRef);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LogicRefPolicyChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LogicRefPolicyChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LogicRefPolicyChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ResourcePayload(bytes32,uint256,bytes)` and selector `0x3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae6`.
```solidity
event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ResourcePayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ResourcePayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ResourcePayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                58u8, 19u8, 77u8, 1u8, 192u8, 120u8, 3u8, 0u8, 60u8, 99u8, 48u8, 23u8,
                23u8, 221u8, 196u8, 97u8, 46u8, 108u8, 71u8, 174u8, 64u8, 142u8, 238u8,
                163u8, 34u8, 44u8, 222u8, 213u8, 50u8, 208u8, 42u8, 230u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ResourcePayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ResourcePayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ResourcePayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransactionExecuted(bytes32)` and selector `0x862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce`.
```solidity
event TransactionExecuted(bytes32 indexed transactionId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionExecuted {
        #[allow(missing_docs)]
        pub transactionId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TransactionExecuted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionExecuted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                134u8, 45u8, 131u8, 198u8, 197u8, 175u8, 39u8, 94u8, 105u8, 123u8, 253u8,
                78u8, 39u8, 200u8, 50u8, 60u8, 25u8, 107u8, 68u8, 189u8, 208u8, 17u8,
                221u8, 154u8, 170u8, 182u8, 219u8, 14u8, 201u8, 148u8, 61u8, 206u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { transactionId: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.transactionId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.transactionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub riscZeroVerifierRouter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub riscZeroVerifierSelector: alloy::sol_types::private::FixedBytes<4>,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.riscZeroVerifierRouter, value.riscZeroVerifierSelector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        riscZeroVerifierRouter: tuple.0,
                        riscZeroVerifierSelector: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.riscZeroVerifierRouter,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.riscZeroVerifierSelector,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `EMPTY_KIND_TABLE_COMMITMENT()` and selector `0xe1c50958`.
```solidity
function EMPTY_KIND_TABLE_COMMITMENT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMPTY_KIND_TABLE_COMMITMENTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`EMPTY_KIND_TABLE_COMMITMENT()`](EMPTY_KIND_TABLE_COMMITMENTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMPTY_KIND_TABLE_COMMITMENTReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EMPTY_KIND_TABLE_COMMITMENTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMPTY_KIND_TABLE_COMMITMENTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMPTY_KIND_TABLE_COMMITMENTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EMPTY_KIND_TABLE_COMMITMENTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMPTY_KIND_TABLE_COMMITMENTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMPTY_KIND_TABLE_COMMITMENTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EMPTY_KIND_TABLE_COMMITMENTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EMPTY_KIND_TABLE_COMMITMENT()";
            const SELECTOR: [u8; 4] = [225u8, 197u8, 9u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: EMPTY_KIND_TABLE_COMMITMENTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: EMPTY_KIND_TABLE_COMMITMENTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `RISC_ZERO_VERIFIER_ROUTER()` and selector `0x6bf0a04b`.
```solidity
function RISC_ZERO_VERIFIER_ROUTER() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RISC_ZERO_VERIFIER_ROUTERCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`RISC_ZERO_VERIFIER_ROUTER()`](RISC_ZERO_VERIFIER_ROUTERCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RISC_ZERO_VERIFIER_ROUTERReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RISC_ZERO_VERIFIER_ROUTERCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: RISC_ZERO_VERIFIER_ROUTERCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for RISC_ZERO_VERIFIER_ROUTERCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RISC_ZERO_VERIFIER_ROUTERReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: RISC_ZERO_VERIFIER_ROUTERReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for RISC_ZERO_VERIFIER_ROUTERReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for RISC_ZERO_VERIFIER_ROUTERCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RISC_ZERO_VERIFIER_ROUTER()";
            const SELECTOR: [u8; 4] = [107u8, 240u8, 160u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: RISC_ZERO_VERIFIER_ROUTERReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: RISC_ZERO_VERIFIER_ROUTERReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `RISC_ZERO_VERIFIER_SELECTOR()` and selector `0xe35a5d2f`.
```solidity
function RISC_ZERO_VERIFIER_SELECTOR() external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RISC_ZERO_VERIFIER_SELECTORCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`RISC_ZERO_VERIFIER_SELECTOR()`](RISC_ZERO_VERIFIER_SELECTORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RISC_ZERO_VERIFIER_SELECTORReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RISC_ZERO_VERIFIER_SELECTORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: RISC_ZERO_VERIFIER_SELECTORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for RISC_ZERO_VERIFIER_SELECTORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RISC_ZERO_VERIFIER_SELECTORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: RISC_ZERO_VERIFIER_SELECTORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for RISC_ZERO_VERIFIER_SELECTORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for RISC_ZERO_VERIFIER_SELECTORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<4>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RISC_ZERO_VERIFIER_SELECTOR()";
            const SELECTOR: [u8; 4] = [227u8, 90u8, 93u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: RISC_ZERO_VERIFIER_SELECTORReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: RISC_ZERO_VERIFIER_SELECTORReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
```solidity
function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`UPGRADE_INTERFACE_VERSION()`](UPGRADE_INTERFACE_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPGRADE_INTERFACE_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPGRADE_INTERFACE_VERSION()";
            const SELECTOR: [u8; 4] = [173u8, 60u8, 177u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `VERSION()` and selector `0xffa1ad74`.
```solidity
function VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`VERSION()`](VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<VERSIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<VERSIONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VERSION()";
            const SELECTOR: [u8; 4] = [255u8, 161u8, 173u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentCount()` and selector `0xc44956d1`.
```solidity
function commitmentCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentCount()`](commitmentCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: commitmentCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for commitmentCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentCount()";
            const SELECTOR: [u8; 4] = [196u8, 73u8, 86u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentTreeCapacity()` and selector `0xfe18ab91`.
```solidity
function commitmentTreeCapacity() external view returns (uint256 capacity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeCapacityCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentTreeCapacity()`](commitmentTreeCapacityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeCapacityReturn {
        #[allow(missing_docs)]
        pub capacity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeCapacityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeCapacityCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeCapacityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeCapacityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeCapacityReturn) -> Self {
                    (value.capacity,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeCapacityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { capacity: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentTreeCapacityCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentTreeCapacity()";
            const SELECTOR: [u8; 4] = [254u8, 24u8, 171u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentTreeCapacityReturn = r.into();
                        r.capacity
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentTreeCapacityReturn = r.into();
                        r.capacity
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentTreeDepth()` and selector `0xa06056f7`.
```solidity
function commitmentTreeDepth() external view returns (uint8 depth);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeDepthCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentTreeDepth()`](commitmentTreeDepthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeDepthReturn {
        #[allow(missing_docs)]
        pub depth: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeDepthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeDepthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeDepthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeDepthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeDepthReturn) -> Self {
                    (value.depth,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeDepthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { depth: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentTreeDepthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentTreeDepth()";
            const SELECTOR: [u8; 4] = [160u8, 96u8, 86u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentTreeDepthReturn = r.into();
                        r.depth
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentTreeDepthReturn = r.into();
                        r.depth
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentTreeRootAtIndex(uint256)` and selector `0x31ee6242`.
```solidity
function commitmentTreeRootAtIndex(uint256 index) external view returns (bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeRootAtIndexCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentTreeRootAtIndex(uint256)`](commitmentTreeRootAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeRootAtIndexReturn {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeRootAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeRootAtIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeRootAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeRootAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeRootAtIndexReturn) -> Self {
                    (value.root,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeRootAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { root: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentTreeRootAtIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentTreeRootAtIndex(uint256)";
            const SELECTOR: [u8; 4] = [49u8, 238u8, 98u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentTreeRootAtIndexReturn = r.into();
                        r.root
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentTreeRootAtIndexReturn = r.into();
                        r.root
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentTreeRootCount()` and selector `0x59ba9258`.
```solidity
function commitmentTreeRootCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeRootCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentTreeRootCount()`](commitmentTreeRootCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeRootCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeRootCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeRootCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeRootCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeRootCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeRootCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeRootCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentTreeRootCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentTreeRootCount()";
            const SELECTOR: [u8; 4] = [89u8, 186u8, 146u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentTreeRootCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentTreeRootCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentTreeSides()` and selector `0xed83cdc7`.
```solidity
function commitmentTreeSides() external view returns (bytes32[] memory sides);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeSidesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentTreeSides()`](commitmentTreeSidesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeSidesReturn {
        #[allow(missing_docs)]
        pub sides: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeSidesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeSidesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeSidesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeSidesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeSidesReturn) -> Self {
                    (value.sides,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeSidesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { sides: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentTreeSidesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentTreeSides()";
            const SELECTOR: [u8; 4] = [237u8, 131u8, 205u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentTreeSidesReturn = r.into();
                        r.sides
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentTreeSidesReturn = r.into();
                        r.sides
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentTreeZeros()` and selector `0x51945b06`.
```solidity
function commitmentTreeZeros() external view returns (bytes32[] memory zeros);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeZerosCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentTreeZeros()`](commitmentTreeZerosCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentTreeZerosReturn {
        #[allow(missing_docs)]
        pub zeros: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeZerosCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeZerosCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeZerosCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentTreeZerosReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentTreeZerosReturn) -> Self {
                    (value.zeros,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentTreeZerosReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { zeros: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentTreeZerosCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentTreeZeros()";
            const SELECTOR: [u8; 4] = [81u8, 148u8, 91u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentTreeZerosReturn = r.into();
                        r.zeros
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentTreeZerosReturn = r.into();
                        r.zeros
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `execute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32))` and selector `0x9239ac0c`.
```solidity
function execute(IProtocolAdapter.Transaction memory transaction) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`execute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32))`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IProtocolAdapter::Transaction,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCall) -> Self {
                    (value.transaction,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { transaction: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl executeReturn {
            fn _tokenize(
                &self,
            ) -> <executeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (IProtocolAdapter::Transaction,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32))";
            const SELECTOR: [u8; 4] = [146u8, 57u8, 172u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IProtocolAdapter::Transaction as alloy_sol_types::SolType>::tokenize(
                        &self.transaction,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                executeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getImplementation()` and selector `0xaaf10f42`.
```solidity
function getImplementation() external view returns (address current);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getImplementationCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getImplementation()`](getImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getImplementationReturn {
        #[allow(missing_docs)]
        pub current: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getImplementationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getImplementationReturn) -> Self {
                    (value.current,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { current: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getImplementation()";
            const SELECTOR: [u8; 4] = [170u8, 241u8, 15u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getImplementationReturn = r.into();
                        r.current
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getImplementationReturn = r.into();
                        r.current
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKindTableCommitment()` and selector `0xffc33f72`.
```solidity
function getKindTableCommitment() external view returns (bytes32 kindTableCommitment);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKindTableCommitmentCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKindTableCommitment()`](getKindTableCommitmentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKindTableCommitmentReturn {
        #[allow(missing_docs)]
        pub kindTableCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKindTableCommitmentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKindTableCommitmentCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKindTableCommitmentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKindTableCommitmentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKindTableCommitmentReturn) -> Self {
                    (value.kindTableCommitment,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKindTableCommitmentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kindTableCommitment: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKindTableCommitmentCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKindTableCommitment()";
            const SELECTOR: [u8; 4] = [255u8, 195u8, 63u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKindTableCommitmentReturn = r.into();
                        r.kindTableCommitment
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: getKindTableCommitmentReturn = r.into();
                        r.kindTableCommitment
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
```solidity
function initialize(address initialOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { initialOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeReturn {
            fn _tokenize(
                &self,
            ) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initialOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initializeLogicRefPolicies()` and selector `0x328c2433`.
```solidity
function initializeLogicRefPolicies() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeLogicRefPoliciesCall;
    ///Container type for the return parameters of the [`initializeLogicRefPolicies()`](initializeLogicRefPoliciesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeLogicRefPoliciesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeLogicRefPoliciesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeLogicRefPoliciesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeLogicRefPoliciesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeLogicRefPoliciesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeLogicRefPoliciesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeLogicRefPoliciesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeLogicRefPoliciesReturn {
            fn _tokenize(
                &self,
            ) -> <initializeLogicRefPoliciesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeLogicRefPoliciesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeLogicRefPoliciesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeLogicRefPolicies()";
            const SELECTOR: [u8; 4] = [50u8, 140u8, 36u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeLogicRefPoliciesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isCommitmentTreeRootContained(bytes32)` and selector `0xc879dbe4`.
```solidity
function isCommitmentTreeRootContained(bytes32 root) external view returns (bool isContained);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCommitmentTreeRootContainedCall {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCommitmentTreeRootContained(bytes32)`](isCommitmentTreeRootContainedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCommitmentTreeRootContainedReturn {
        #[allow(missing_docs)]
        pub isContained: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isCommitmentTreeRootContainedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCommitmentTreeRootContainedCall) -> Self {
                    (value.root,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCommitmentTreeRootContainedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { root: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isCommitmentTreeRootContainedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCommitmentTreeRootContainedReturn) -> Self {
                    (value.isContained,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCommitmentTreeRootContainedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isContained: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCommitmentTreeRootContainedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCommitmentTreeRootContained(bytes32)";
            const SELECTOR: [u8; 4] = [200u8, 121u8, 219u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isCommitmentTreeRootContainedReturn = r.into();
                        r.isContained
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isCommitmentTreeRootContainedReturn = r.into();
                        r.isContained
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isNullifierContained(bytes32)` and selector `0xc1b0bed7`.
```solidity
function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNullifierContainedCall {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isNullifierContained(bytes32)`](isNullifierContainedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNullifierContainedReturn {
        #[allow(missing_docs)]
        pub isContained: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isNullifierContainedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isNullifierContainedCall) -> Self {
                    (value.nullifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isNullifierContainedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nullifier: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isNullifierContainedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isNullifierContainedReturn) -> Self {
                    (value.isContained,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isNullifierContainedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isContained: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isNullifierContainedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isNullifierContained(bytes32)";
            const SELECTOR: [u8; 4] = [193u8, 176u8, 190u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifier),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isNullifierContainedReturn = r.into();
                        r.isContained
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isNullifierContainedReturn = r.into();
                        r.isContained
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestCommitmentTreeRoot()` and selector `0xbdeb442d`.
```solidity
function latestCommitmentTreeRoot() external view returns (bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestCommitmentTreeRootCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`latestCommitmentTreeRoot()`](latestCommitmentTreeRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestCommitmentTreeRootReturn {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestCommitmentTreeRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestCommitmentTreeRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestCommitmentTreeRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestCommitmentTreeRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestCommitmentTreeRootReturn) -> Self {
                    (value.root,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestCommitmentTreeRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { root: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestCommitmentTreeRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestCommitmentTreeRoot()";
            const SELECTOR: [u8; 4] = [189u8, 235u8, 68u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: latestCommitmentTreeRootReturn = r.into();
                        r.root
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: latestCommitmentTreeRootReturn = r.into();
                        r.root
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `logicRefPolicy(bytes32)` and selector `0xe5edfd22`.
```solidity
function logicRefPolicy(bytes32 logicRef) external view returns (ILogicRefPolicyRegistry.LogicRefPolicy policy);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct logicRefPolicyCall {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`logicRefPolicy(bytes32)`](logicRefPolicyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct logicRefPolicyReturn {
        #[allow(missing_docs)]
        pub policy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<logicRefPolicyCall> for UnderlyingRustTuple<'_> {
                fn from(value: logicRefPolicyCall) -> Self {
                    (value.logicRef,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for logicRefPolicyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { logicRef: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (ILogicRefPolicyRegistry::LogicRefPolicy,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<logicRefPolicyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: logicRefPolicyReturn) -> Self {
                    (value.policy,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for logicRefPolicyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { policy: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for logicRefPolicyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ILogicRefPolicyRegistry::LogicRefPolicy,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "logicRefPolicy(bytes32)";
            const SELECTOR: [u8; 4] = [229u8, 237u8, 253u8, 34u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <ILogicRefPolicyRegistry::LogicRefPolicy as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: logicRefPolicyReturn = r.into();
                        r.policy
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: logicRefPolicyReturn = r.into();
                        r.policy
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nullifierAtIndex(uint256)` and selector `0x9ad91d4c`.
```solidity
function nullifierAtIndex(uint256 index) external view returns (bytes32 nullifier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nullifierAtIndexCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nullifierAtIndex(uint256)`](nullifierAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nullifierAtIndexReturn {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nullifierAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nullifierAtIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nullifierAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nullifierAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nullifierAtIndexReturn) -> Self {
                    (value.nullifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nullifierAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nullifier: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nullifierAtIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nullifierAtIndex(uint256)";
            const SELECTOR: [u8; 4] = [154u8, 217u8, 29u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nullifierAtIndexReturn = r.into();
                        r.nullifier
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: nullifierAtIndexReturn = r.into();
                        r.nullifier
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nullifierCount()` and selector `0x40f34d42`.
```solidity
function nullifierCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nullifierCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nullifierCount()`](nullifierCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nullifierCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nullifierCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: nullifierCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nullifierCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nullifierCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nullifierCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nullifierCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nullifierCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nullifierCount()";
            const SELECTOR: [u8; 4] = [64u8, 243u8, 77u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nullifierCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: nullifierCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall;
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseReturn {
            fn _tokenize(
                &self,
            ) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                pauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool isPaused);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        #[allow(missing_docs)]
        pub isPaused: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value.isPaused,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isPaused: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r.isPaused
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r.isPaused
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `restrictedLogicRefAtIndex(uint256)` and selector `0xc37d757a`.
```solidity
function restrictedLogicRefAtIndex(uint256 index) external view returns (bytes32 logicRef);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct restrictedLogicRefAtIndexCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`restrictedLogicRefAtIndex(uint256)`](restrictedLogicRefAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct restrictedLogicRefAtIndexReturn {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<restrictedLogicRefAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: restrictedLogicRefAtIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for restrictedLogicRefAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<restrictedLogicRefAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: restrictedLogicRefAtIndexReturn) -> Self {
                    (value.logicRef,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for restrictedLogicRefAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { logicRef: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for restrictedLogicRefAtIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "restrictedLogicRefAtIndex(uint256)";
            const SELECTOR: [u8; 4] = [195u8, 125u8, 117u8, 122u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: restrictedLogicRefAtIndexReturn = r.into();
                        r.logicRef
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: restrictedLogicRefAtIndexReturn = r.into();
                        r.logicRef
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `restrictedLogicRefCount()` and selector `0x8ee0a261`.
```solidity
function restrictedLogicRefCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct restrictedLogicRefCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`restrictedLogicRefCount()`](restrictedLogicRefCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct restrictedLogicRefCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<restrictedLogicRefCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: restrictedLogicRefCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for restrictedLogicRefCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<restrictedLogicRefCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: restrictedLogicRefCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for restrictedLogicRefCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for restrictedLogicRefCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "restrictedLogicRefCount()";
            const SELECTOR: [u8; 4] = [142u8, 224u8, 162u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: restrictedLogicRefCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: restrictedLogicRefCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `riscZeroVerifierPaused()` and selector `0x67353122`.
```solidity
function riscZeroVerifierPaused() external view returns (bool isPaused);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct riscZeroVerifierPausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`riscZeroVerifierPaused()`](riscZeroVerifierPausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct riscZeroVerifierPausedReturn {
        #[allow(missing_docs)]
        pub isPaused: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<riscZeroVerifierPausedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: riscZeroVerifierPausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for riscZeroVerifierPausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<riscZeroVerifierPausedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: riscZeroVerifierPausedReturn) -> Self {
                    (value.isPaused,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for riscZeroVerifierPausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isPaused: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for riscZeroVerifierPausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "riscZeroVerifierPaused()";
            const SELECTOR: [u8; 4] = [103u8, 53u8, 49u8, 34u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: riscZeroVerifierPausedReturn = r.into();
                        r.isPaused
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: riscZeroVerifierPausedReturn = r.into();
                        r.isPaused
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setKindTableCommitment(bytes32)` and selector `0xc0253023`.
```solidity
function setKindTableCommitment(bytes32 newKindTableCommitment) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setKindTableCommitmentCall {
        #[allow(missing_docs)]
        pub newKindTableCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`setKindTableCommitment(bytes32)`](setKindTableCommitmentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setKindTableCommitmentReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setKindTableCommitmentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setKindTableCommitmentCall) -> Self {
                    (value.newKindTableCommitment,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setKindTableCommitmentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newKindTableCommitment: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setKindTableCommitmentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setKindTableCommitmentReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setKindTableCommitmentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setKindTableCommitmentReturn {
            fn _tokenize(
                &self,
            ) -> <setKindTableCommitmentCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setKindTableCommitmentCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setKindTableCommitmentReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setKindTableCommitment(bytes32)";
            const SELECTOR: [u8; 4] = [192u8, 37u8, 48u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newKindTableCommitment,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setKindTableCommitmentReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setLogicRefPolicies((bytes32,uint8)[])` and selector `0xc75f304d`.
```solidity
function setLogicRefPolicies(ILogicRefPolicyRegistry.PolicyUpdate[] memory updates) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setLogicRefPoliciesCall {
        #[allow(missing_docs)]
        pub updates: alloy::sol_types::private::Vec<
            <ILogicRefPolicyRegistry::PolicyUpdate as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`setLogicRefPolicies((bytes32,uint8)[])`](setLogicRefPoliciesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setLogicRefPoliciesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ILogicRefPolicyRegistry::PolicyUpdate>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ILogicRefPolicyRegistry::PolicyUpdate as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setLogicRefPoliciesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setLogicRefPoliciesCall) -> Self {
                    (value.updates,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setLogicRefPoliciesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { updates: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setLogicRefPoliciesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setLogicRefPoliciesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setLogicRefPoliciesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setLogicRefPoliciesReturn {
            fn _tokenize(
                &self,
            ) -> <setLogicRefPoliciesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setLogicRefPoliciesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<ILogicRefPolicyRegistry::PolicyUpdate>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setLogicRefPoliciesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setLogicRefPolicies((bytes32,uint8)[])";
            const SELECTOR: [u8; 4] = [199u8, 95u8, 48u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        ILogicRefPolicyRegistry::PolicyUpdate,
                    > as alloy_sol_types::SolType>::tokenize(&self.updates),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setLogicRefPoliciesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `simulateExecute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32),bool)` and selector `0xa600458b`.
```solidity
function simulateExecute(IProtocolAdapter.Transaction memory transaction, bool skipRiscZeroProofVerification) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateExecuteCall {
        #[allow(missing_docs)]
        pub transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub skipRiscZeroProofVerification: bool,
    }
    ///Container type for the return parameters of the [`simulateExecute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32),bool)`](simulateExecuteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateExecuteReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                IProtocolAdapter::Transaction,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulateExecuteCall> for UnderlyingRustTuple<'_> {
                fn from(value: simulateExecuteCall) -> Self {
                    (value.transaction, value.skipRiscZeroProofVerification)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for simulateExecuteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        transaction: tuple.0,
                        skipRiscZeroProofVerification: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulateExecuteReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulateExecuteReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulateExecuteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl simulateExecuteReturn {
            fn _tokenize(
                &self,
            ) -> <simulateExecuteCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for simulateExecuteCall {
            type Parameters<'a> = (
                IProtocolAdapter::Transaction,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = simulateExecuteReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "simulateExecute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32),bool)";
            const SELECTOR: [u8; 4] = [166u8, 0u8, 69u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IProtocolAdapter::Transaction as alloy_sol_types::SolType>::tokenize(
                        &self.transaction,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.skipRiscZeroProofVerification,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                simulateExecuteReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall;
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseReturn {
            fn _tokenize(
                &self,
            ) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                unpauseReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`.
```solidity
function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallCall {
        #[allow(missing_docs)]
        pub newImplementation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`upgradeToAndCall(address,bytes)`](upgradeToAndCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallCall) -> Self {
                    (value.newImplementation, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newImplementation: tuple.0,
                        data: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl upgradeToAndCallReturn {
            fn _tokenize(
                &self,
            ) -> <upgradeToAndCallCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeToAndCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeToAndCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeToAndCall(address,bytes)";
            const SELECTOR: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newImplementation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                upgradeToAndCallReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    ///Container for all the [`ProtocolAdapter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolAdapterCalls {
        #[allow(missing_docs)]
        EMPTY_KIND_TABLE_COMMITMENT(EMPTY_KIND_TABLE_COMMITMENTCall),
        #[allow(missing_docs)]
        RISC_ZERO_VERIFIER_ROUTER(RISC_ZERO_VERIFIER_ROUTERCall),
        #[allow(missing_docs)]
        RISC_ZERO_VERIFIER_SELECTOR(RISC_ZERO_VERIFIER_SELECTORCall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        VERSION(VERSIONCall),
        #[allow(missing_docs)]
        commitmentCount(commitmentCountCall),
        #[allow(missing_docs)]
        commitmentTreeCapacity(commitmentTreeCapacityCall),
        #[allow(missing_docs)]
        commitmentTreeDepth(commitmentTreeDepthCall),
        #[allow(missing_docs)]
        commitmentTreeRootAtIndex(commitmentTreeRootAtIndexCall),
        #[allow(missing_docs)]
        commitmentTreeRootCount(commitmentTreeRootCountCall),
        #[allow(missing_docs)]
        commitmentTreeSides(commitmentTreeSidesCall),
        #[allow(missing_docs)]
        commitmentTreeZeros(commitmentTreeZerosCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        getImplementation(getImplementationCall),
        #[allow(missing_docs)]
        getKindTableCommitment(getKindTableCommitmentCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        initializeLogicRefPolicies(initializeLogicRefPoliciesCall),
        #[allow(missing_docs)]
        isCommitmentTreeRootContained(isCommitmentTreeRootContainedCall),
        #[allow(missing_docs)]
        isNullifierContained(isNullifierContainedCall),
        #[allow(missing_docs)]
        latestCommitmentTreeRoot(latestCommitmentTreeRootCall),
        #[allow(missing_docs)]
        logicRefPolicy(logicRefPolicyCall),
        #[allow(missing_docs)]
        nullifierAtIndex(nullifierAtIndexCall),
        #[allow(missing_docs)]
        nullifierCount(nullifierCountCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        restrictedLogicRefAtIndex(restrictedLogicRefAtIndexCall),
        #[allow(missing_docs)]
        restrictedLogicRefCount(restrictedLogicRefCountCall),
        #[allow(missing_docs)]
        riscZeroVerifierPaused(riscZeroVerifierPausedCall),
        #[allow(missing_docs)]
        setKindTableCommitment(setKindTableCommitmentCall),
        #[allow(missing_docs)]
        setLogicRefPolicies(setLogicRefPoliciesCall),
        #[allow(missing_docs)]
        simulateExecute(simulateExecuteCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl ProtocolAdapterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [49u8, 238u8, 98u8, 66u8],
            [50u8, 140u8, 36u8, 51u8],
            [63u8, 75u8, 168u8, 58u8],
            [64u8, 243u8, 77u8, 66u8],
            [79u8, 30u8, 242u8, 134u8],
            [81u8, 148u8, 91u8, 6u8],
            [82u8, 209u8, 144u8, 45u8],
            [89u8, 186u8, 146u8, 88u8],
            [92u8, 151u8, 90u8, 187u8],
            [103u8, 53u8, 49u8, 34u8],
            [107u8, 240u8, 160u8, 75u8],
            [113u8, 80u8, 24u8, 166u8],
            [132u8, 86u8, 203u8, 89u8],
            [141u8, 165u8, 203u8, 91u8],
            [142u8, 224u8, 162u8, 97u8],
            [146u8, 57u8, 172u8, 12u8],
            [154u8, 217u8, 29u8, 76u8],
            [160u8, 96u8, 86u8, 247u8],
            [166u8, 0u8, 69u8, 139u8],
            [170u8, 241u8, 15u8, 66u8],
            [173u8, 60u8, 177u8, 204u8],
            [189u8, 235u8, 68u8, 45u8],
            [192u8, 37u8, 48u8, 35u8],
            [193u8, 176u8, 190u8, 215u8],
            [195u8, 125u8, 117u8, 122u8],
            [196u8, 73u8, 86u8, 209u8],
            [196u8, 214u8, 109u8, 232u8],
            [199u8, 95u8, 48u8, 77u8],
            [200u8, 121u8, 219u8, 228u8],
            [225u8, 197u8, 9u8, 88u8],
            [227u8, 90u8, 93u8, 47u8],
            [229u8, 237u8, 253u8, 34u8],
            [237u8, 131u8, 205u8, 199u8],
            [242u8, 253u8, 227u8, 139u8],
            [254u8, 24u8, 171u8, 145u8],
            [255u8, 161u8, 173u8, 116u8],
            [255u8, 195u8, 63u8, 114u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(commitmentTreeRootAtIndex),
            ::core::stringify!(initializeLogicRefPolicies),
            ::core::stringify!(unpause),
            ::core::stringify!(nullifierCount),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(commitmentTreeZeros),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(commitmentTreeRootCount),
            ::core::stringify!(paused),
            ::core::stringify!(riscZeroVerifierPaused),
            ::core::stringify!(RISC_ZERO_VERIFIER_ROUTER),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(pause),
            ::core::stringify!(owner),
            ::core::stringify!(restrictedLogicRefCount),
            ::core::stringify!(execute),
            ::core::stringify!(nullifierAtIndex),
            ::core::stringify!(commitmentTreeDepth),
            ::core::stringify!(simulateExecute),
            ::core::stringify!(getImplementation),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(latestCommitmentTreeRoot),
            ::core::stringify!(setKindTableCommitment),
            ::core::stringify!(isNullifierContained),
            ::core::stringify!(restrictedLogicRefAtIndex),
            ::core::stringify!(commitmentCount),
            ::core::stringify!(initialize),
            ::core::stringify!(setLogicRefPolicies),
            ::core::stringify!(isCommitmentTreeRootContained),
            ::core::stringify!(EMPTY_KIND_TABLE_COMMITMENT),
            ::core::stringify!(RISC_ZERO_VERIFIER_SELECTOR),
            ::core::stringify!(logicRefPolicy),
            ::core::stringify!(commitmentTreeSides),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(commitmentTreeCapacity),
            ::core::stringify!(VERSION),
            ::core::stringify!(getKindTableCommitment),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeLogicRefPoliciesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <unpauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nullifierCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeZerosCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <restrictedLogicRefCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <executeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nullifierAtIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <simulateExecuteCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getImplementationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setKindTableCommitmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isNullifierContainedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <restrictedLogicRefAtIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setLogicRefPoliciesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <EMPTY_KIND_TABLE_COMMITMENTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::SIGNATURE,
            <logicRefPolicyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeSidesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::SIGNATURE,
            <VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKindTableCommitmentCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolAdapterCalls {
        const NAME: &'static str = "ProtocolAdapterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 37usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EMPTY_KIND_TABLE_COMMITMENT(_) => {
                    <EMPTY_KIND_TABLE_COMMITMENTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::RISC_ZERO_VERIFIER_ROUTER(_) => {
                    <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::RISC_ZERO_VERIFIER_SELECTOR(_) => {
                    <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::VERSION(_) => <VERSIONCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::commitmentCount(_) => {
                    <commitmentCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeCapacity(_) => {
                    <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeDepth(_) => {
                    <commitmentTreeDepthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeRootAtIndex(_) => {
                    <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeRootCount(_) => {
                    <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeSides(_) => {
                    <commitmentTreeSidesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeZeros(_) => {
                    <commitmentTreeZerosCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getImplementation(_) => {
                    <getImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKindTableCommitment(_) => {
                    <getKindTableCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeLogicRefPolicies(_) => {
                    <initializeLogicRefPoliciesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCommitmentTreeRootContained(_) => {
                    <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isNullifierContained(_) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestCommitmentTreeRoot(_) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::logicRefPolicy(_) => {
                    <logicRefPolicyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nullifierAtIndex(_) => {
                    <nullifierAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nullifierCount(_) => {
                    <nullifierCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::restrictedLogicRefAtIndex(_) => {
                    <restrictedLogicRefAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::restrictedLogicRefCount(_) => {
                    <restrictedLogicRefCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::riscZeroVerifierPaused(_) => {
                    <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setKindTableCommitment(_) => {
                    <setKindTableCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setLogicRefPolicies(_) => {
                    <setLogicRefPoliciesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::simulateExecute(_) => {
                    <simulateExecuteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_with_config(
            selector: [u8; 4],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<ProtocolAdapterCalls>] = &[
                {
                    fn commitmentTreeRootAtIndex(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeRootAtIndex)
                    }
                    commitmentTreeRootAtIndex
                },
                {
                    fn initializeLogicRefPolicies(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <initializeLogicRefPoliciesCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::initializeLogicRefPolicies)
                    }
                    initializeLogicRefPolicies
                },
                {
                    fn unpause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::unpause)
                    }
                    unpause
                },
                {
                    fn nullifierCount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <nullifierCountCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::nullifierCount)
                    }
                    nullifierCount
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn commitmentTreeZeros(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeZerosCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeZeros)
                    }
                    commitmentTreeZeros
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn commitmentTreeRootCount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeRootCount)
                    }
                    commitmentTreeRootCount
                },
                {
                    fn paused(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn riscZeroVerifierPaused(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::riscZeroVerifierPaused)
                    }
                    riscZeroVerifierPaused
                },
                {
                    fn RISC_ZERO_VERIFIER_ROUTER(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::RISC_ZERO_VERIFIER_ROUTER)
                    }
                    RISC_ZERO_VERIFIER_ROUTER
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn pause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::pause)
                    }
                    pause
                },
                {
                    fn owner(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn restrictedLogicRefCount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <restrictedLogicRefCountCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::restrictedLogicRefCount)
                    }
                    restrictedLogicRefCount
                },
                {
                    fn execute(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::execute)
                    }
                    execute
                },
                {
                    fn nullifierAtIndex(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <nullifierAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::nullifierAtIndex)
                    }
                    nullifierAtIndex
                },
                {
                    fn commitmentTreeDepth(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeDepth)
                    }
                    commitmentTreeDepth
                },
                {
                    fn simulateExecute(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <simulateExecuteCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::simulateExecute)
                    }
                    simulateExecute
                },
                {
                    fn getImplementation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <getImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::getImplementation)
                    }
                    getImplementation
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn latestCommitmentTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::latestCommitmentTreeRoot)
                    }
                    latestCommitmentTreeRoot
                },
                {
                    fn setKindTableCommitment(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::setKindTableCommitment)
                    }
                    setKindTableCommitment
                },
                {
                    fn isNullifierContained(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::isNullifierContained)
                    }
                    isNullifierContained
                },
                {
                    fn restrictedLogicRefAtIndex(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <restrictedLogicRefAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::restrictedLogicRefAtIndex)
                    }
                    restrictedLogicRefAtIndex
                },
                {
                    fn commitmentCount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentCountCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentCount)
                    }
                    commitmentCount
                },
                {
                    fn initialize(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setLogicRefPolicies(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <setLogicRefPoliciesCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::setLogicRefPolicies)
                    }
                    setLogicRefPolicies
                },
                {
                    fn isCommitmentTreeRootContained(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::isCommitmentTreeRootContained)
                    }
                    isCommitmentTreeRootContained
                },
                {
                    fn EMPTY_KIND_TABLE_COMMITMENT(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <EMPTY_KIND_TABLE_COMMITMENTCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::EMPTY_KIND_TABLE_COMMITMENT)
                    }
                    EMPTY_KIND_TABLE_COMMITMENT
                },
                {
                    fn RISC_ZERO_VERIFIER_SELECTOR(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::RISC_ZERO_VERIFIER_SELECTOR)
                    }
                    RISC_ZERO_VERIFIER_SELECTOR
                },
                {
                    fn logicRefPolicy(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <logicRefPolicyCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::logicRefPolicy)
                    }
                    logicRefPolicy
                },
                {
                    fn commitmentTreeSides(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeSidesCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeSides)
                    }
                    commitmentTreeSides
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn commitmentTreeCapacity(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeCapacity)
                    }
                    commitmentTreeCapacity
                },
                {
                    fn VERSION(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::VERSION)
                    }
                    VERSION
                },
                {
                    fn getKindTableCommitment(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::getKindTableCommitment)
                    }
                    getKindTableCommitment
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, config)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
            )
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::EMPTY_KIND_TABLE_COMMITMENT(inner) => {
                    <EMPTY_KIND_TABLE_COMMITMENTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RISC_ZERO_VERIFIER_ROUTER(inner) => {
                    <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RISC_ZERO_VERIFIER_SELECTOR(inner) => {
                    <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::commitmentCount(inner) => {
                    <commitmentCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentTreeCapacity(inner) => {
                    <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentTreeDepth(inner) => {
                    <commitmentTreeDepthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentTreeRootAtIndex(inner) => {
                    <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentTreeRootCount(inner) => {
                    <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentTreeSides(inner) => {
                    <commitmentTreeSidesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentTreeZeros(inner) => {
                    <commitmentTreeZerosCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getImplementation(inner) => {
                    <getImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKindTableCommitment(inner) => {
                    <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initializeLogicRefPolicies(inner) => {
                    <initializeLogicRefPoliciesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isCommitmentTreeRootContained(inner) => {
                    <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isNullifierContained(inner) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestCommitmentTreeRoot(inner) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::logicRefPolicy(inner) => {
                    <logicRefPolicyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nullifierAtIndex(inner) => {
                    <nullifierAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nullifierCount(inner) => {
                    <nullifierCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::restrictedLogicRefAtIndex(inner) => {
                    <restrictedLogicRefAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::restrictedLogicRefCount(inner) => {
                    <restrictedLogicRefCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::riscZeroVerifierPaused(inner) => {
                    <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setKindTableCommitment(inner) => {
                    <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setLogicRefPolicies(inner) => {
                    <setLogicRefPoliciesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::simulateExecute(inner) => {
                    <simulateExecuteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EMPTY_KIND_TABLE_COMMITMENT(inner) => {
                    <EMPTY_KIND_TABLE_COMMITMENTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RISC_ZERO_VERIFIER_ROUTER(inner) => {
                    <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RISC_ZERO_VERIFIER_SELECTOR(inner) => {
                    <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::commitmentCount(inner) => {
                    <commitmentCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentTreeCapacity(inner) => {
                    <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentTreeDepth(inner) => {
                    <commitmentTreeDepthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentTreeRootAtIndex(inner) => {
                    <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentTreeRootCount(inner) => {
                    <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentTreeSides(inner) => {
                    <commitmentTreeSidesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentTreeZeros(inner) => {
                    <commitmentTreeZerosCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getImplementation(inner) => {
                    <getImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKindTableCommitment(inner) => {
                    <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeLogicRefPolicies(inner) => {
                    <initializeLogicRefPoliciesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isCommitmentTreeRootContained(inner) => {
                    <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isNullifierContained(inner) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestCommitmentTreeRoot(inner) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::logicRefPolicy(inner) => {
                    <logicRefPolicyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nullifierAtIndex(inner) => {
                    <nullifierAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nullifierCount(inner) => {
                    <nullifierCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::restrictedLogicRefAtIndex(inner) => {
                    <restrictedLogicRefAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::restrictedLogicRefCount(inner) => {
                    <restrictedLogicRefCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::riscZeroVerifierPaused(inner) => {
                    <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setKindTableCommitment(inner) => {
                    <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setLogicRefPolicies(inner) => {
                    <setLogicRefPoliciesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::simulateExecute(inner) => {
                    <simulateExecuteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ProtocolAdapter`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolAdapterErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        DeltaMismatch(DeltaMismatch),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EmptyTransactionNotAllowed(EmptyTransactionNotAllowed),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        ForwarderCallOutputMismatch(ForwarderCallOutputMismatch),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidLogicRefPolicyTransition(InvalidLogicRefPolicyTransition),
        #[allow(missing_docs)]
        LogicRefPoliciesNotInitialized(LogicRefPoliciesNotInitialized),
        #[allow(missing_docs)]
        NonExistingRoot(NonExistingRoot),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        PointNotOnCurve(PointNotOnCurve),
        #[allow(missing_docs)]
        PreExistingNullifier(PreExistingNullifier),
        #[allow(missing_docs)]
        PreExistingRoot(PreExistingRoot),
        #[allow(missing_docs)]
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        #[allow(missing_docs)]
        ResourceWithDeniedLogicRef(ResourceWithDeniedLogicRef),
        #[allow(missing_docs)]
        RiscZeroVerifierPaused(RiscZeroVerifierPaused),
        #[allow(missing_docs)]
        RiscZeroVerifierSelectorMismatch(RiscZeroVerifierSelectorMismatch),
        #[allow(missing_docs)]
        Simulated(Simulated),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnacceptedKindTableCommitment(UnacceptedKindTableCommitment),
        #[allow(missing_docs)]
        ZeroKindTableCommitmentNotAllowed(ZeroKindTableCommitmentNotAllowed),
        #[allow(missing_docs)]
        ZeroLogicRefNotAllowed(ZeroLogicRefNotAllowed),
        #[allow(missing_docs)]
        ZeroRiscZeroVerifierRouterNotAllowed(ZeroRiscZeroVerifierRouterNotAllowed),
        #[allow(missing_docs)]
        ZeroRiscZeroVerifierSelectorNotAllowed(ZeroRiscZeroVerifierSelectorNotAllowed),
    }
    impl ProtocolAdapterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [37u8, 174u8, 110u8, 170u8],
            [39u8, 164u8, 19u8, 54u8],
            [57u8, 169u8, 64u8, 197u8],
            [62u8, 229u8, 174u8, 181u8],
            [76u8, 156u8, 140u8, 227u8],
            [89u8, 122u8, 189u8, 32u8],
            [111u8, 20u8, 152u8, 49u8],
            [119u8, 77u8, 93u8, 225u8],
            [120u8, 162u8, 34u8, 28u8],
            [122u8, 246u8, 35u8, 65u8],
            [141u8, 252u8, 32u8, 43u8],
            [153u8, 150u8, 179u8, 21u8],
            [166u8, 216u8, 42u8, 2u8],
            [170u8, 29u8, 73u8, 164u8],
            [177u8, 178u8, 90u8, 175u8],
            [179u8, 152u8, 151u8, 159u8],
            [184u8, 160u8, 232u8, 161u8],
            [197u8, 4u8, 250u8, 218u8],
            [207u8, 27u8, 9u8, 60u8],
            [214u8, 189u8, 162u8, 117u8],
            [214u8, 216u8, 137u8, 27u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [219u8, 120u8, 140u8, 43u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 212u8, 75u8, 76u8],
            [238u8, 128u8, 239u8, 174u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [249u8, 132u8, 158u8, 163u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(RiscZeroVerifierPaused),
            ::core::stringify!(LogicRefPoliciesNotInitialized),
            ::core::stringify!(PreExistingNullifier),
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(ResourceWithDeniedLogicRef),
            ::core::stringify!(Simulated),
            ::core::stringify!(ZeroKindTableCommitmentNotAllowed),
            ::core::stringify!(RiscZeroVerifierSelectorMismatch),
            ::core::stringify!(ZeroLogicRefNotAllowed),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(ZeroRiscZeroVerifierRouterNotAllowed),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ZeroRiscZeroVerifierSelectorNotAllowed),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(PointNotOnCurve),
            ::core::stringify!(ForwarderCallOutputMismatch),
            ::core::stringify!(EmptyTransactionNotAllowed),
            ::core::stringify!(FailedCall),
            ::core::stringify!(UnacceptedKindTableCommitment),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(PreExistingRoot),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(DeltaMismatch),
            ::core::stringify!(InvalidLogicRefPolicyTransition),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(NonExistingRoot),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierPaused as alloy_sol_types::SolError>::SIGNATURE,
            <LogicRefPoliciesNotInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingNullifier as alloy_sol_types::SolError>::SIGNATURE,
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::SIGNATURE,
            <Simulated as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <PointNotOnCurve as alloy_sol_types::SolError>::SIGNATURE,
            <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <UnacceptedKindTableCommitment as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingRoot as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <DeltaMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidLogicRefPolicyTransition as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
            <NonExistingRoot as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolAdapterErrors {
        const NAME: &'static str = "ProtocolAdapterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DeltaMismatch(_) => {
                    <DeltaMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyTransactionNotAllowed(_) => {
                    <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => {
                    <EnforcedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpectedPause(_) => {
                    <ExpectedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ForwarderCallOutputMismatch(_) => {
                    <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLogicRefPolicyTransition(_) => {
                    <InvalidLogicRefPolicyTransition as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LogicRefPoliciesNotInitialized(_) => {
                    <LogicRefPoliciesNotInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonExistingRoot(_) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PointNotOnCurve(_) => {
                    <PointNotOnCurve as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PreExistingNullifier(_) => {
                    <PreExistingNullifier as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PreExistingRoot(_) => {
                    <PreExistingRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReentrancyGuardReentrantCall(_) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ResourceWithDeniedLogicRef(_) => {
                    <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RiscZeroVerifierPaused(_) => {
                    <RiscZeroVerifierPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RiscZeroVerifierSelectorMismatch(_) => {
                    <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Simulated(_) => <Simulated as alloy_sol_types::SolError>::SELECTOR,
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnacceptedKindTableCommitment(_) => {
                    <UnacceptedKindTableCommitment as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroKindTableCommitmentNotAllowed(_) => {
                    <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroLogicRefNotAllowed(_) => {
                    <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroRiscZeroVerifierRouterNotAllowed(_) => {
                    <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroRiscZeroVerifierSelectorNotAllowed(_) => {
                    <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_with_config(
            selector: [u8; 4],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<ProtocolAdapterErrors>] = &[
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn RiscZeroVerifierPaused(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <RiscZeroVerifierPaused as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::RiscZeroVerifierPaused)
                    }
                    RiscZeroVerifierPaused
                },
                {
                    fn LogicRefPoliciesNotInitialized(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <LogicRefPoliciesNotInitialized as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::LogicRefPoliciesNotInitialized)
                    }
                    LogicRefPoliciesNotInitialized
                },
                {
                    fn PreExistingNullifier(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PreExistingNullifier as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::PreExistingNullifier)
                    }
                    PreExistingNullifier
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn ResourceWithDeniedLogicRef(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ResourceWithDeniedLogicRef)
                    }
                    ResourceWithDeniedLogicRef
                },
                {
                    fn Simulated(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <Simulated as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::Simulated)
                    }
                    Simulated
                },
                {
                    fn ZeroKindTableCommitmentNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                ProtocolAdapterErrors::ZeroKindTableCommitmentNotAllowed,
                            )
                    }
                    ZeroKindTableCommitmentNotAllowed
                },
                {
                    fn RiscZeroVerifierSelectorMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::RiscZeroVerifierSelectorMismatch)
                    }
                    RiscZeroVerifierSelectorMismatch
                },
                {
                    fn ZeroLogicRefNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ZeroLogicRefNotAllowed)
                    }
                    ZeroLogicRefNotAllowed
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn ZeroRiscZeroVerifierRouterNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                ProtocolAdapterErrors::ZeroRiscZeroVerifierRouterNotAllowed,
                            )
                    }
                    ZeroRiscZeroVerifierRouterNotAllowed
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ZeroRiscZeroVerifierSelectorNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                ProtocolAdapterErrors::ZeroRiscZeroVerifierSelectorNotAllowed,
                            )
                    }
                    ZeroRiscZeroVerifierSelectorNotAllowed
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn PointNotOnCurve(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PointNotOnCurve as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::PointNotOnCurve)
                    }
                    PointNotOnCurve
                },
                {
                    fn ForwarderCallOutputMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ForwarderCallOutputMismatch)
                    }
                    ForwarderCallOutputMismatch
                },
                {
                    fn EmptyTransactionNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::EmptyTransactionNotAllowed)
                    }
                    EmptyTransactionNotAllowed
                },
                {
                    fn FailedCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn UnacceptedKindTableCommitment(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UnacceptedKindTableCommitment as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::UnacceptedKindTableCommitment)
                    }
                    UnacceptedKindTableCommitment
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn PreExistingRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PreExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::PreExistingRoot)
                    }
                    PreExistingRoot
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn DeltaMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <DeltaMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::DeltaMismatch)
                    }
                    DeltaMismatch
                },
                {
                    fn InvalidLogicRefPolicyTransition(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <InvalidLogicRefPolicyTransition as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::InvalidLogicRefPolicyTransition)
                    }
                    InvalidLogicRefPolicyTransition
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn NonExistingRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <NonExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::NonExistingRoot)
                    }
                    NonExistingRoot
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, config)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
            )
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DeltaMismatch(inner) => {
                    <DeltaMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyTransactionNotAllowed(inner) => {
                    <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ForwarderCallOutputMismatch(inner) => {
                    <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLogicRefPolicyTransition(inner) => {
                    <InvalidLogicRefPolicyTransition as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LogicRefPoliciesNotInitialized(inner) => {
                    <LogicRefPoliciesNotInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonExistingRoot(inner) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PointNotOnCurve(inner) => {
                    <PointNotOnCurve as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PreExistingNullifier(inner) => {
                    <PreExistingNullifier as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PreExistingRoot(inner) => {
                    <PreExistingRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ResourceWithDeniedLogicRef(inner) => {
                    <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RiscZeroVerifierPaused(inner) => {
                    <RiscZeroVerifierPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RiscZeroVerifierSelectorMismatch(inner) => {
                    <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Simulated(inner) => {
                    <Simulated as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnacceptedKindTableCommitment(inner) => {
                    <UnacceptedKindTableCommitment as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroKindTableCommitmentNotAllowed(inner) => {
                    <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroLogicRefNotAllowed(inner) => {
                    <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroRiscZeroVerifierRouterNotAllowed(inner) => {
                    <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroRiscZeroVerifierSelectorNotAllowed(inner) => {
                    <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DeltaMismatch(inner) => {
                    <DeltaMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyTransactionNotAllowed(inner) => {
                    <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ForwarderCallOutputMismatch(inner) => {
                    <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLogicRefPolicyTransition(inner) => {
                    <InvalidLogicRefPolicyTransition as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LogicRefPoliciesNotInitialized(inner) => {
                    <LogicRefPoliciesNotInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonExistingRoot(inner) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PointNotOnCurve(inner) => {
                    <PointNotOnCurve as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PreExistingNullifier(inner) => {
                    <PreExistingNullifier as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PreExistingRoot(inner) => {
                    <PreExistingRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ResourceWithDeniedLogicRef(inner) => {
                    <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RiscZeroVerifierPaused(inner) => {
                    <RiscZeroVerifierPaused as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RiscZeroVerifierSelectorMismatch(inner) => {
                    <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Simulated(inner) => {
                    <Simulated as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnacceptedKindTableCommitment(inner) => {
                    <UnacceptedKindTableCommitment as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroKindTableCommitmentNotAllowed(inner) => {
                    <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroLogicRefNotAllowed(inner) => {
                    <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroRiscZeroVerifierRouterNotAllowed(inner) => {
                    <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroRiscZeroVerifierSelectorNotAllowed(inner) => {
                    <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ProtocolAdapterErrors {
        /**Creates a [`AddressEmptyCode`] error.

```solidity
error AddressEmptyCode(address)
```*/
        #[inline]
        pub fn address_empty_code(target: alloy::sol_types::private::Address) -> Self {
            Self::AddressEmptyCode(AddressEmptyCode { target: target })
        }
        /**Creates a [`DeltaMismatch`] error.

```solidity
error DeltaMismatch(address,address)
```*/
        #[inline]
        pub fn delta_mismatch(
            expected: alloy::sol_types::private::Address,
            actual: alloy::sol_types::private::Address,
        ) -> Self {
            Self::DeltaMismatch(DeltaMismatch {
                expected: expected,
                actual: actual,
            })
        }
        /**Creates a [`ECDSAInvalidSignature`] error.

```solidity
error ECDSAInvalidSignature()
```*/
        #[inline]
        pub fn ecdsa_invalid_signature() -> Self {
            Self::ECDSAInvalidSignature(ECDSAInvalidSignature)
        }
        /**Creates a [`ECDSAInvalidSignatureLength`] error.

```solidity
error ECDSAInvalidSignatureLength(uint256)
```*/
        #[inline]
        pub fn ecdsa_invalid_signature_length(
            length: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength {
                length: length,
            })
        }
        /**Creates a [`ECDSAInvalidSignatureS`] error.

```solidity
error ECDSAInvalidSignatureS(bytes32)
```*/
        #[inline]
        pub fn ecdsa_invalid_signature_s(
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::ECDSAInvalidSignatureS(ECDSAInvalidSignatureS { s: s })
        }
        /**Creates a [`ERC1967InvalidImplementation`] error.

```solidity
error ERC1967InvalidImplementation(address)
```*/
        #[inline]
        pub fn erc_1967_invalid_implementation(
            implementation: alloy::sol_types::private::Address,
        ) -> Self {
            Self::ERC1967InvalidImplementation(ERC1967InvalidImplementation {
                implementation: implementation,
            })
        }
        /**Creates a [`ERC1967NonPayable`] error.

```solidity
error ERC1967NonPayable()
```*/
        #[inline]
        pub fn erc_1967_non_payable() -> Self {
            Self::ERC1967NonPayable(ERC1967NonPayable)
        }
        /**Creates a [`EmptyTransactionNotAllowed`] error.

```solidity
error EmptyTransactionNotAllowed()
```*/
        #[inline]
        pub fn empty_transaction_not_allowed() -> Self {
            Self::EmptyTransactionNotAllowed(EmptyTransactionNotAllowed)
        }
        /**Creates a [`EnforcedPause`] error.

```solidity
error EnforcedPause()
```*/
        #[inline]
        pub fn enforced_pause() -> Self {
            Self::EnforcedPause(EnforcedPause)
        }
        /**Creates a [`ExpectedPause`] error.

```solidity
error ExpectedPause()
```*/
        #[inline]
        pub fn expected_pause() -> Self {
            Self::ExpectedPause(ExpectedPause)
        }
        /**Creates a [`FailedCall`] error.

```solidity
error FailedCall()
```*/
        #[inline]
        pub fn failed_call() -> Self {
            Self::FailedCall(FailedCall)
        }
        /**Creates a [`ForwarderCallOutputMismatch`] error.

```solidity
error ForwarderCallOutputMismatch(bytes,bytes)
```*/
        #[inline]
        pub fn forwarder_call_output_mismatch(
            expected: alloy::sol_types::private::Bytes,
            actual: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ForwarderCallOutputMismatch(ForwarderCallOutputMismatch {
                expected: expected,
                actual: actual,
            })
        }
        /**Creates a [`InvalidInitialization`] error.

```solidity
error InvalidInitialization()
```*/
        #[inline]
        pub fn invalid_initialization() -> Self {
            Self::InvalidInitialization(InvalidInitialization)
        }
        /**Creates a [`InvalidLogicRefPolicyTransition`] error.

```solidity
error InvalidLogicRefPolicyTransition(bytes32,uint8,uint8)
```*/
        #[inline]
        pub fn invalid_logic_ref_policy_transition(
            logic_ref: alloy::sol_types::private::FixedBytes<32>,
            previous_policy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
            new_policy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
        ) -> Self {
            Self::InvalidLogicRefPolicyTransition(InvalidLogicRefPolicyTransition {
                logicRef: logic_ref,
                previousPolicy: previous_policy,
                newPolicy: new_policy,
            })
        }
        /**Creates a [`LogicRefPoliciesNotInitialized`] error.

```solidity
error LogicRefPoliciesNotInitialized()
```*/
        #[inline]
        pub fn logic_ref_policies_not_initialized() -> Self {
            Self::LogicRefPoliciesNotInitialized(LogicRefPoliciesNotInitialized)
        }
        /**Creates a [`NonExistingRoot`] error.

```solidity
error NonExistingRoot(bytes32)
```*/
        #[inline]
        pub fn non_existing_root(
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::NonExistingRoot(NonExistingRoot { root: root })
        }
        /**Creates a [`NotInitializing`] error.

```solidity
error NotInitializing()
```*/
        #[inline]
        pub fn not_initializing() -> Self {
            Self::NotInitializing(NotInitializing)
        }
        /**Creates a [`OwnableInvalidOwner`] error.

```solidity
error OwnableInvalidOwner(address)
```*/
        #[inline]
        pub fn ownable_invalid_owner(owner: alloy::sol_types::private::Address) -> Self {
            Self::OwnableInvalidOwner(OwnableInvalidOwner {
                owner: owner,
            })
        }
        /**Creates a [`OwnableUnauthorizedAccount`] error.

```solidity
error OwnableUnauthorizedAccount(address)
```*/
        #[inline]
        pub fn ownable_unauthorized_account(
            account: alloy::sol_types::private::Address,
        ) -> Self {
            Self::OwnableUnauthorizedAccount(OwnableUnauthorizedAccount {
                account: account,
            })
        }
        /**Creates a [`PointNotOnCurve`] error.

```solidity
error PointNotOnCurve((uint256,uint256))
```*/
        #[inline]
        pub fn point_not_on_curve(
            point: <IProtocolAdapter::Delta as alloy::sol_types::SolType>::RustType,
        ) -> Self {
            Self::PointNotOnCurve(PointNotOnCurve { point: point })
        }
        /**Creates a [`PreExistingNullifier`] error.

```solidity
error PreExistingNullifier(bytes32)
```*/
        #[inline]
        pub fn pre_existing_nullifier(
            nullifier: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::PreExistingNullifier(PreExistingNullifier {
                nullifier: nullifier,
            })
        }
        /**Creates a [`PreExistingRoot`] error.

```solidity
error PreExistingRoot(bytes32)
```*/
        #[inline]
        pub fn pre_existing_root(
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::PreExistingRoot(PreExistingRoot { root: root })
        }
        /**Creates a [`ReentrancyGuardReentrantCall`] error.

```solidity
error ReentrancyGuardReentrantCall()
```*/
        #[inline]
        pub fn reentrancy_guard_reentrant_call() -> Self {
            Self::ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall)
        }
        /**Creates a [`ResourceWithDeniedLogicRef`] error.

```solidity
error ResourceWithDeniedLogicRef(bytes32,bool)
```*/
        #[inline]
        pub fn resource_with_denied_logic_ref(
            logic_ref: alloy::sol_types::private::FixedBytes<32>,
            consumed: bool,
        ) -> Self {
            Self::ResourceWithDeniedLogicRef(ResourceWithDeniedLogicRef {
                logicRef: logic_ref,
                consumed: consumed,
            })
        }
        /**Creates a [`RiscZeroVerifierPaused`] error.

```solidity
error RiscZeroVerifierPaused()
```*/
        #[inline]
        pub fn risc_zero_verifier_paused() -> Self {
            Self::RiscZeroVerifierPaused(RiscZeroVerifierPaused)
        }
        /**Creates a [`RiscZeroVerifierSelectorMismatch`] error.

```solidity
error RiscZeroVerifierSelectorMismatch(bytes4,bytes4)
```*/
        #[inline]
        pub fn risc_zero_verifier_selector_mismatch(
            expected: alloy::sol_types::private::FixedBytes<4>,
            actual: alloy::sol_types::private::FixedBytes<4>,
        ) -> Self {
            Self::RiscZeroVerifierSelectorMismatch(RiscZeroVerifierSelectorMismatch {
                expected: expected,
                actual: actual,
            })
        }
        /**Creates a [`Simulated`] error.

```solidity
error Simulated(uint256)
```*/
        #[inline]
        pub fn simulated(
            gas_used: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::Simulated(Simulated { gasUsed: gas_used })
        }
        /**Creates a [`UUPSUnauthorizedCallContext`] error.

```solidity
error UUPSUnauthorizedCallContext()
```*/
        #[inline]
        pub fn uups_unauthorized_call_context() -> Self {
            Self::UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext)
        }
        /**Creates a [`UUPSUnsupportedProxiableUUID`] error.

```solidity
error UUPSUnsupportedProxiableUUID(bytes32)
```*/
        #[inline]
        pub fn uups_unsupported_proxiable_uuid(
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID {
                slot: slot,
            })
        }
        /**Creates a [`UnacceptedKindTableCommitment`] error.

```solidity
error UnacceptedKindTableCommitment(bytes32)
```*/
        #[inline]
        pub fn unaccepted_kind_table_commitment(
            kind_table_commitment: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::UnacceptedKindTableCommitment(UnacceptedKindTableCommitment {
                kindTableCommitment: kind_table_commitment,
            })
        }
        /**Creates a [`ZeroKindTableCommitmentNotAllowed`] error.

```solidity
error ZeroKindTableCommitmentNotAllowed()
```*/
        #[inline]
        pub fn zero_kind_table_commitment_not_allowed() -> Self {
            Self::ZeroKindTableCommitmentNotAllowed(ZeroKindTableCommitmentNotAllowed)
        }
        /**Creates a [`ZeroLogicRefNotAllowed`] error.

```solidity
error ZeroLogicRefNotAllowed()
```*/
        #[inline]
        pub fn zero_logic_ref_not_allowed() -> Self {
            Self::ZeroLogicRefNotAllowed(ZeroLogicRefNotAllowed)
        }
        /**Creates a [`ZeroRiscZeroVerifierRouterNotAllowed`] error.

```solidity
error ZeroRiscZeroVerifierRouterNotAllowed()
```*/
        #[inline]
        pub fn zero_risc_zero_verifier_router_not_allowed() -> Self {
            Self::ZeroRiscZeroVerifierRouterNotAllowed(
                ZeroRiscZeroVerifierRouterNotAllowed,
            )
        }
        /**Creates a [`ZeroRiscZeroVerifierSelectorNotAllowed`] error.

```solidity
error ZeroRiscZeroVerifierSelectorNotAllowed()
```*/
        #[inline]
        pub fn zero_risc_zero_verifier_selector_not_allowed() -> Self {
            Self::ZeroRiscZeroVerifierSelectorNotAllowed(
                ZeroRiscZeroVerifierSelectorNotAllowed,
            )
        }
    }
    ///Container for all the [`ProtocolAdapter`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolAdapterEvents {
        #[allow(missing_docs)]
        ActionExecuted(ActionExecuted),
        #[allow(missing_docs)]
        ApplicationPayload(ApplicationPayload),
        #[allow(missing_docs)]
        CommitmentTreeRootAdded(CommitmentTreeRootAdded),
        #[allow(missing_docs)]
        DiscoveryPayload(DiscoveryPayload),
        #[allow(missing_docs)]
        ExternalPayload(ExternalPayload),
        #[allow(missing_docs)]
        ForwarderCallExecuted(ForwarderCallExecuted),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KindTableCommitmentUpdated(KindTableCommitmentUpdated),
        #[allow(missing_docs)]
        LogicRefPolicyChanged(LogicRefPolicyChanged),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        ResourcePayload(ResourcePayload),
        #[allow(missing_docs)]
        TransactionExecuted(TransactionExecuted),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl ProtocolAdapterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 45u8, 197u8, 72u8, 237u8, 149u8, 10u8, 204u8, 180u8, 13u8, 93u8,
                120u8, 84u8, 31u8, 57u8, 84u8, 197u8, 225u8, 130u8, 168u8, 236u8, 241u8,
                158u8, 88u8, 26u8, 79u8, 34u8, 99u8, 246u8, 31u8, 89u8, 210u8,
            ],
            [
                20u8, 112u8, 148u8, 139u8, 50u8, 56u8, 103u8, 3u8, 227u8, 117u8, 109u8,
                142u8, 20u8, 213u8, 31u8, 173u8, 59u8, 175u8, 143u8, 37u8, 216u8, 36u8,
                140u8, 36u8, 141u8, 247u8, 175u8, 83u8, 96u8, 227u8, 128u8, 197u8,
            ],
            [
                58u8, 19u8, 77u8, 1u8, 192u8, 120u8, 3u8, 0u8, 60u8, 99u8, 48u8, 23u8,
                23u8, 221u8, 196u8, 97u8, 46u8, 108u8, 71u8, 174u8, 64u8, 142u8, 238u8,
                163u8, 34u8, 44u8, 222u8, 213u8, 50u8, 208u8, 42u8, 230u8,
            ],
            [
                72u8, 36u8, 56u8, 115u8, 180u8, 117u8, 45u8, 220u8, 180u8, 94u8, 13u8,
                123u8, 17u8, 196u8, 194u8, 102u8, 88u8, 62u8, 94u8, 9u8, 154u8, 11u8,
                121u8, 143u8, 221u8, 156u8, 26u8, 247u8, 212u8, 147u8, 36u8, 243u8,
            ],
            [
                76u8, 231u8, 193u8, 16u8, 32u8, 96u8, 190u8, 79u8, 40u8, 178u8, 41u8,
                25u8, 145u8, 64u8, 253u8, 213u8, 17u8, 45u8, 66u8, 20u8, 254u8, 149u8,
                65u8, 188u8, 155u8, 35u8, 84u8, 30u8, 69u8, 146u8, 204u8, 50u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                134u8, 45u8, 131u8, 198u8, 197u8, 175u8, 39u8, 94u8, 105u8, 123u8, 253u8,
                78u8, 39u8, 200u8, 50u8, 60u8, 25u8, 107u8, 68u8, 189u8, 208u8, 17u8,
                221u8, 154u8, 170u8, 182u8, 219u8, 14u8, 201u8, 148u8, 61u8, 206u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                144u8, 45u8, 104u8, 8u8, 12u8, 159u8, 75u8, 247u8, 237u8, 28u8, 146u8,
                103u8, 88u8, 35u8, 106u8, 252u8, 43u8, 64u8, 68u8, 72u8, 127u8, 111u8,
                134u8, 132u8, 53u8, 45u8, 56u8, 8u8, 114u8, 68u8, 174u8, 230u8,
            ],
            [
                156u8, 97u8, 178u8, 144u8, 246u8, 49u8, 9u8, 127u8, 86u8, 39u8, 60u8,
                244u8, 218u8, 244u8, 13u8, 241u8, 255u8, 156u8, 204u8, 51u8, 241u8, 1u8,
                212u8, 100u8, 131u8, 125u8, 161u8, 245u8, 174u8, 24u8, 189u8, 89u8,
            ],
            [
                164u8, 148u8, 218u8, 196u8, 183u8, 24u8, 72u8, 67u8, 88u8, 63u8, 151u8,
                46u8, 6u8, 120u8, 62u8, 44u8, 59u8, 180u8, 127u8, 79u8, 1u8, 55u8, 184u8,
                223u8, 82u8, 168u8, 96u8, 223u8, 7u8, 33u8, 159u8, 140u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                205u8, 219u8, 50u8, 122u8, 219u8, 49u8, 254u8, 84u8, 55u8, 223u8, 42u8,
                140u8, 104u8, 48u8, 27u8, 177u8, 58u8, 107u8, 170u8, 228u8, 50u8, 168u8,
                4u8, 131u8, 140u8, 170u8, 246u8, 130u8, 80u8, 106u8, 173u8, 241u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(CommitmentTreeRootAdded),
            ::core::stringify!(LogicRefPolicyChanged),
            ::core::stringify!(ResourcePayload),
            ::core::stringify!(DiscoveryPayload),
            ::core::stringify!(ActionExecuted),
            ::core::stringify!(Unpaused),
            ::core::stringify!(Paused),
            ::core::stringify!(TransactionExecuted),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(KindTableCommitmentUpdated),
            ::core::stringify!(ExternalPayload),
            ::core::stringify!(ApplicationPayload),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
            ::core::stringify!(ForwarderCallExecuted),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <LogicRefPolicyChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <KindTableCommitmentUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <ApplicationPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <ForwarderCallExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ProtocolAdapterEvents {
        const NAME: &'static str = "ProtocolAdapterEvents";
        const COUNT: usize = 15usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            <Self as alloy_sol_types::SolEventInterface>::decode_raw_log_with_config(
                topics,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        fn decode_raw_log_with_config(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActionExecuted as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ActionExecuted)
                }
                Some(
                    <ApplicationPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ApplicationPayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ApplicationPayload)
                }
                Some(
                    <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::CommitmentTreeRootAdded)
                }
                Some(<DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DiscoveryPayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::DiscoveryPayload)
                }
                Some(<ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExternalPayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ExternalPayload)
                }
                Some(
                    <ForwarderCallExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ForwarderCallExecuted as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ForwarderCallExecuted)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <KindTableCommitmentUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KindTableCommitmentUpdated as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::KindTableCommitmentUpdated)
                }
                Some(
                    <LogicRefPolicyChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LogicRefPolicyChanged as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::LogicRefPolicyChanged)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::Paused)
                }
                Some(<ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ResourcePayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ResourcePayload)
                }
                Some(
                    <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionExecuted as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::TransactionExecuted)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::Unpaused)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::Upgraded)
                }
                _ => {
                    if topics
                        .len()
                        .checked_mul(alloy_sol_types::Word::len_bytes())
                        .and_then(|len| len.checked_add(data.len()))
                        .is_none_or(|len| len > config.get_memory_limit())
                    {
                        return alloy_sol_types::private::Err(
                            alloy_sol_types::Error::MemoryLimitExceeded(
                                config.get_memory_limit(),
                            ),
                        );
                    }
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ProtocolAdapterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ApplicationPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CommitmentTreeRootAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DiscoveryPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExternalPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ForwarderCallExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KindTableCommitmentUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LogicRefPolicyChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ResourcePayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ApplicationPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CommitmentTreeRootAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DiscoveryPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExternalPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ForwarderCallExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KindTableCommitmentUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LogicRefPolicyChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ResourcePayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    #[automatically_derived]
    impl ProtocolAdapterEvents {
        /**Creates a [`ActionExecuted`] event.

```solidity
event ActionExecuted(bytes32,bytes32[],bytes32[],bytes32[],bytes32[])
```*/
        #[inline]
        pub fn action_executed(
            action_tree_root: alloy::sol_types::private::FixedBytes<32>,
            nullifiers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            consumed_logic_refs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            commitments: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            created_logic_refs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> Self {
            Self::ActionExecuted(ActionExecuted {
                actionTreeRoot: action_tree_root,
                nullifiers: nullifiers,
                consumedLogicRefs: consumed_logic_refs,
                commitments: commitments,
                createdLogicRefs: created_logic_refs,
            })
        }
        /**Creates a [`ApplicationPayload`] event.

```solidity
event ApplicationPayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn application_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ApplicationPayload(ApplicationPayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`CommitmentTreeRootAdded`] event.

```solidity
event CommitmentTreeRootAdded(bytes32)
```*/
        #[inline]
        pub fn commitment_tree_root_added(
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::CommitmentTreeRootAdded(CommitmentTreeRootAdded {
                root: root,
            })
        }
        /**Creates a [`DiscoveryPayload`] event.

```solidity
event DiscoveryPayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn discovery_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::DiscoveryPayload(DiscoveryPayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`ExternalPayload`] event.

```solidity
event ExternalPayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn external_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ExternalPayload(ExternalPayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`ForwarderCallExecuted`] event.

```solidity
event ForwarderCallExecuted(address,bytes,bytes)
```*/
        #[inline]
        pub fn forwarder_call_executed(
            untrusted_forwarder: alloy::sol_types::private::Address,
            input: alloy::sol_types::private::Bytes,
            output: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ForwarderCallExecuted(ForwarderCallExecuted {
                untrustedForwarder: untrusted_forwarder,
                input: input,
                output: output,
            })
        }
        /**Creates a [`Initialized`] event.

```solidity
event Initialized(uint64)
```*/
        #[inline]
        pub fn initialized(version: u64) -> Self {
            Self::Initialized(Initialized { version: version })
        }
        /**Creates a [`KindTableCommitmentUpdated`] event.

```solidity
event KindTableCommitmentUpdated(bytes32)
```*/
        #[inline]
        pub fn kind_table_commitment_updated(
            kind_table_commitment: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::KindTableCommitmentUpdated(KindTableCommitmentUpdated {
                kindTableCommitment: kind_table_commitment,
            })
        }
        /**Creates a [`LogicRefPolicyChanged`] event.

```solidity
event LogicRefPolicyChanged(bytes32,uint8,uint8)
```*/
        #[inline]
        pub fn logic_ref_policy_changed(
            logic_ref: alloy::sol_types::private::FixedBytes<32>,
            previous_policy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
            new_policy: <ILogicRefPolicyRegistry::LogicRefPolicy as alloy::sol_types::SolType>::RustType,
        ) -> Self {
            Self::LogicRefPolicyChanged(LogicRefPolicyChanged {
                logicRef: logic_ref,
                previousPolicy: previous_policy,
                newPolicy: new_policy,
            })
        }
        /**Creates a [`OwnershipTransferred`] event.

```solidity
event OwnershipTransferred(address,address)
```*/
        #[inline]
        pub fn ownership_transferred(
            previous_owner: alloy::sol_types::private::Address,
            new_owner: alloy::sol_types::private::Address,
        ) -> Self {
            Self::OwnershipTransferred(OwnershipTransferred {
                previousOwner: previous_owner,
                newOwner: new_owner,
            })
        }
        /**Creates a [`Paused`] event.

```solidity
event Paused(address)
```*/
        #[inline]
        pub fn paused(account: alloy::sol_types::private::Address) -> Self {
            Self::Paused(Paused { account: account })
        }
        /**Creates a [`ResourcePayload`] event.

```solidity
event ResourcePayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn resource_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ResourcePayload(ResourcePayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`TransactionExecuted`] event.

```solidity
event TransactionExecuted(bytes32)
```*/
        #[inline]
        pub fn transaction_executed(
            transaction_id: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::TransactionExecuted(TransactionExecuted {
                transactionId: transaction_id,
            })
        }
        /**Creates a [`Unpaused`] event.

```solidity
event Unpaused(address)
```*/
        #[inline]
        pub fn unpaused(account: alloy::sol_types::private::Address) -> Self {
            Self::Unpaused(Unpaused { account: account })
        }
        /**Creates a [`Upgraded`] event.

```solidity
event Upgraded(address)
```*/
        #[inline]
        pub fn upgraded(implementation: alloy::sol_types::private::Address) -> Self {
            Self::Upgraded(Upgraded {
                implementation: implementation,
            })
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ProtocolAdapter`](self) contract instance.

See the [wrapper's documentation](`ProtocolAdapterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ProtocolAdapterInstance<P, N> {
        ProtocolAdapterInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        riscZeroVerifierRouter: alloy::sol_types::private::Address,
        riscZeroVerifierSelector: alloy::sol_types::private::FixedBytes<4>,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ProtocolAdapterInstance<P, N>>,
    > {
        ProtocolAdapterInstance::<
            P,
            N,
        >::deploy(__provider, riscZeroVerifierRouter, riscZeroVerifierSelector)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        riscZeroVerifierRouter: alloy::sol_types::private::Address,
        riscZeroVerifierSelector: alloy::sol_types::private::FixedBytes<4>,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        ProtocolAdapterInstance::<
            P,
            N,
        >::deploy_builder(__provider, riscZeroVerifierRouter, riscZeroVerifierSelector)
    }
    /**A [`ProtocolAdapter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ProtocolAdapter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ProtocolAdapterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ProtocolAdapterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ProtocolAdapterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ProtocolAdapterInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ProtocolAdapter`](self) contract instance.

See the [wrapper's documentation](`ProtocolAdapterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            riscZeroVerifierRouter: alloy::sol_types::private::Address,
            riscZeroVerifierSelector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::Result<ProtocolAdapterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                riscZeroVerifierRouter,
                riscZeroVerifierSelector,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            riscZeroVerifierRouter: alloy::sol_types::private::Address,
            riscZeroVerifierSelector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            riscZeroVerifierRouter,
                            riscZeroVerifierSelector,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ProtocolAdapterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ProtocolAdapterInstance<P, N> {
            ProtocolAdapterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ProtocolAdapterInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`EMPTY_KIND_TABLE_COMMITMENT`] function.
        pub fn EMPTY_KIND_TABLE_COMMITMENT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, EMPTY_KIND_TABLE_COMMITMENTCall, N> {
            self.call_builder(&EMPTY_KIND_TABLE_COMMITMENTCall)
        }
        ///Creates a new call builder for the [`RISC_ZERO_VERIFIER_ROUTER`] function.
        pub fn RISC_ZERO_VERIFIER_ROUTER(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, RISC_ZERO_VERIFIER_ROUTERCall, N> {
            self.call_builder(&RISC_ZERO_VERIFIER_ROUTERCall)
        }
        ///Creates a new call builder for the [`RISC_ZERO_VERIFIER_SELECTOR`] function.
        pub fn RISC_ZERO_VERIFIER_SELECTOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, RISC_ZERO_VERIFIER_SELECTORCall, N> {
            self.call_builder(&RISC_ZERO_VERIFIER_SELECTORCall)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`VERSION`] function.
        pub fn VERSION(&self) -> alloy_contract::SolCallBuilder<&P, VERSIONCall, N> {
            self.call_builder(&VERSIONCall)
        }
        ///Creates a new call builder for the [`commitmentCount`] function.
        pub fn commitmentCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentCountCall, N> {
            self.call_builder(&commitmentCountCall)
        }
        ///Creates a new call builder for the [`commitmentTreeCapacity`] function.
        pub fn commitmentTreeCapacity(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentTreeCapacityCall, N> {
            self.call_builder(&commitmentTreeCapacityCall)
        }
        ///Creates a new call builder for the [`commitmentTreeDepth`] function.
        pub fn commitmentTreeDepth(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentTreeDepthCall, N> {
            self.call_builder(&commitmentTreeDepthCall)
        }
        ///Creates a new call builder for the [`commitmentTreeRootAtIndex`] function.
        pub fn commitmentTreeRootAtIndex(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentTreeRootAtIndexCall, N> {
            self.call_builder(
                &commitmentTreeRootAtIndexCall {
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`commitmentTreeRootCount`] function.
        pub fn commitmentTreeRootCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentTreeRootCountCall, N> {
            self.call_builder(&commitmentTreeRootCountCall)
        }
        ///Creates a new call builder for the [`commitmentTreeSides`] function.
        pub fn commitmentTreeSides(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentTreeSidesCall, N> {
            self.call_builder(&commitmentTreeSidesCall)
        }
        ///Creates a new call builder for the [`commitmentTreeZeros`] function.
        pub fn commitmentTreeZeros(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentTreeZerosCall, N> {
            self.call_builder(&commitmentTreeZerosCall)
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, executeCall, N> {
            self.call_builder(&executeCall { transaction })
        }
        ///Creates a new call builder for the [`getImplementation`] function.
        pub fn getImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getImplementationCall, N> {
            self.call_builder(&getImplementationCall)
        }
        ///Creates a new call builder for the [`getKindTableCommitment`] function.
        pub fn getKindTableCommitment(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKindTableCommitmentCall, N> {
            self.call_builder(&getKindTableCommitmentCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { initialOwner })
        }
        ///Creates a new call builder for the [`initializeLogicRefPolicies`] function.
        pub fn initializeLogicRefPolicies(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, initializeLogicRefPoliciesCall, N> {
            self.call_builder(&initializeLogicRefPoliciesCall)
        }
        ///Creates a new call builder for the [`isCommitmentTreeRootContained`] function.
        pub fn isCommitmentTreeRootContained(
            &self,
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isCommitmentTreeRootContainedCall, N> {
            self.call_builder(
                &isCommitmentTreeRootContainedCall {
                    root,
                },
            )
        }
        ///Creates a new call builder for the [`isNullifierContained`] function.
        pub fn isNullifierContained(
            &self,
            nullifier: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isNullifierContainedCall, N> {
            self.call_builder(
                &isNullifierContainedCall {
                    nullifier,
                },
            )
        }
        ///Creates a new call builder for the [`latestCommitmentTreeRoot`] function.
        pub fn latestCommitmentTreeRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, latestCommitmentTreeRootCall, N> {
            self.call_builder(&latestCommitmentTreeRootCall)
        }
        ///Creates a new call builder for the [`logicRefPolicy`] function.
        pub fn logicRefPolicy(
            &self,
            logicRef: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, logicRefPolicyCall, N> {
            self.call_builder(&logicRefPolicyCall { logicRef })
        }
        ///Creates a new call builder for the [`nullifierAtIndex`] function.
        pub fn nullifierAtIndex(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, nullifierAtIndexCall, N> {
            self.call_builder(&nullifierAtIndexCall { index })
        }
        ///Creates a new call builder for the [`nullifierCount`] function.
        pub fn nullifierCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nullifierCountCall, N> {
            self.call_builder(&nullifierCountCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`restrictedLogicRefAtIndex`] function.
        pub fn restrictedLogicRefAtIndex(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, restrictedLogicRefAtIndexCall, N> {
            self.call_builder(
                &restrictedLogicRefAtIndexCall {
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`restrictedLogicRefCount`] function.
        pub fn restrictedLogicRefCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, restrictedLogicRefCountCall, N> {
            self.call_builder(&restrictedLogicRefCountCall)
        }
        ///Creates a new call builder for the [`riscZeroVerifierPaused`] function.
        pub fn riscZeroVerifierPaused(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, riscZeroVerifierPausedCall, N> {
            self.call_builder(&riscZeroVerifierPausedCall)
        }
        ///Creates a new call builder for the [`setKindTableCommitment`] function.
        pub fn setKindTableCommitment(
            &self,
            newKindTableCommitment: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, setKindTableCommitmentCall, N> {
            self.call_builder(
                &setKindTableCommitmentCall {
                    newKindTableCommitment,
                },
            )
        }
        ///Creates a new call builder for the [`setLogicRefPolicies`] function.
        pub fn setLogicRefPolicies(
            &self,
            updates: alloy::sol_types::private::Vec<
                <ILogicRefPolicyRegistry::PolicyUpdate as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, setLogicRefPoliciesCall, N> {
            self.call_builder(&setLogicRefPoliciesCall { updates })
        }
        ///Creates a new call builder for the [`simulateExecute`] function.
        pub fn simulateExecute(
            &self,
            transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
            skipRiscZeroProofVerification: bool,
        ) -> alloy_contract::SolCallBuilder<&P, simulateExecuteCall, N> {
            self.call_builder(
                &simulateExecuteCall {
                    transaction,
                    skipRiscZeroProofVerification,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall)
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeToAndCallCall, N> {
            self.call_builder(
                &upgradeToAndCallCall {
                    newImplementation,
                    data,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ProtocolAdapterInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ActionExecuted`] event.
        pub fn ActionExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ActionExecuted, N> {
            self.event_filter::<ActionExecuted>()
        }
        ///Creates a new event filter for the [`ApplicationPayload`] event.
        pub fn ApplicationPayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, ApplicationPayload, N> {
            self.event_filter::<ApplicationPayload>()
        }
        ///Creates a new event filter for the [`CommitmentTreeRootAdded`] event.
        pub fn CommitmentTreeRootAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, CommitmentTreeRootAdded, N> {
            self.event_filter::<CommitmentTreeRootAdded>()
        }
        ///Creates a new event filter for the [`DiscoveryPayload`] event.
        pub fn DiscoveryPayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, DiscoveryPayload, N> {
            self.event_filter::<DiscoveryPayload>()
        }
        ///Creates a new event filter for the [`ExternalPayload`] event.
        pub fn ExternalPayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExternalPayload, N> {
            self.event_filter::<ExternalPayload>()
        }
        ///Creates a new event filter for the [`ForwarderCallExecuted`] event.
        pub fn ForwarderCallExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ForwarderCallExecuted, N> {
            self.event_filter::<ForwarderCallExecuted>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`KindTableCommitmentUpdated`] event.
        pub fn KindTableCommitmentUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, KindTableCommitmentUpdated, N> {
            self.event_filter::<KindTableCommitmentUpdated>()
        }
        ///Creates a new event filter for the [`LogicRefPolicyChanged`] event.
        pub fn LogicRefPolicyChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, LogicRefPolicyChanged, N> {
            self.event_filter::<LogicRefPolicyChanged>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`ResourcePayload`] event.
        pub fn ResourcePayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, ResourcePayload, N> {
            self.event_filter::<ResourcePayload>()
        }
        ///Creates a new event filter for the [`TransactionExecuted`] event.
        pub fn TransactionExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionExecuted, N> {
            self.event_filter::<TransactionExecuted>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
