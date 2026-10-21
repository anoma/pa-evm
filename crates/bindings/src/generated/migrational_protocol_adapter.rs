///Module containing a contract's types and functions.
/**

```solidity
library ILogicRefDenylist {
    struct DeniedLogicRef { bytes32 logicRef; bool consumed; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ILogicRefDenylist {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct DeniedLogicRef { bytes32 logicRef; bool consumed; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeniedLogicRef {
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
        impl ::core::convert::From<DeniedLogicRef> for UnderlyingRustTuple<'_> {
            fn from(value: DeniedLogicRef) -> Self {
                (value.logicRef, value.consumed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DeniedLogicRef {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    logicRef: tuple.0,
                    consumed: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DeniedLogicRef {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DeniedLogicRef {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
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
        impl alloy_sol_types::SolType for DeniedLogicRef {
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
        impl alloy_sol_types::SolStruct for DeniedLogicRef {
            const NAME: &'static str = "DeniedLogicRef";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DeniedLogicRef(bytes32 logicRef,bool consumed)",
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.consumed,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeniedLogicRef {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.logicRef,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.consumed,
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
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.consumed,
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
    /**Creates a new wrapper around an on-chain [`ILogicRefDenylist`](self) contract instance.

See the [wrapper's documentation](`ILogicRefDenylistInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ILogicRefDenylistInstance<P, N> {
        ILogicRefDenylistInstance::<P, N>::new(address, __provider)
    }
    /**A [`ILogicRefDenylist`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ILogicRefDenylist`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ILogicRefDenylistInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ILogicRefDenylistInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ILogicRefDenylistInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ILogicRefDenylistInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ILogicRefDenylist`](self) contract instance.

See the [wrapper's documentation](`ILogicRefDenylistInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> ILogicRefDenylistInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ILogicRefDenylistInstance<P, N> {
            ILogicRefDenylistInstance {
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
    > ILogicRefDenylistInstance<P, N> {
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
    > ILogicRefDenylistInstance<P, N> {
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
    struct Transaction { Action[] actions; bytes deltaProof; bytes aggregationProof; bytes32 kindTreeRoot; }
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
struct Transaction { Action[] actions; bytes deltaProof; bytes aggregationProof; bytes32 kindTreeRoot; }
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
        pub kindTreeRoot: alloy::sol_types::private::FixedBytes<32>,
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
                    value.kindTreeRoot,
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
                    kindTreeRoot: tuple.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kindTreeRoot),
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
                    "Transaction(Action[] actions,bytes deltaProof,bytes aggregationProof,bytes32 kindTreeRoot)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.kindTreeRoot)
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
                        &rust.kindTreeRoot,
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
                    &rust.kindTreeRoot,
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
library ILogicRefDenylist {
    struct DeniedLogicRef {
        bytes32 logicRef;
        bool consumed;
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
        bytes32 kindTreeRoot;
    }
}

interface MigrationalProtocolAdapter {
    error AddressEmptyCode(address target);
    error AfterTagNotInTransaction(bytes32 tag);
    error BeforeTagNotInTransaction(bytes32 tag);
    error CommitmentCountMismatch(uint256 expected, uint256 actual);
    error CommitmentTreeNotEmpty();
    error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);
    error CyclicConstraints();
    error DeltaMismatch(address expected, address actual);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyCommitmentTreeNotAllowed();
    error EmptyTransactionNotAllowed();
    error EnforcedPause();
    error ExpectedPause();
    error FailedCall();
    error ForwarderCallOutputMismatch(bytes expected, bytes actual);
    error InvalidInitialization();
    error LeafCountExceedsCapacity(uint256 leafCount, uint256 capacity);
    error LegacyProtocolAdapterNotStopped(address legacyProtocolAdapter);
    error LogicRefAlreadyDenied(bytes32 logicRef, bool consumed);
    error MissingHistoricalRoot(bytes32 root);
    error MissingNullifier(bytes32 nullifier);
    error NonExistingRoot(bytes32 root);
    error NotInitializing();
    error NullifierBatchLeavesGap(uint256 start);
    error NullifierBatchOutOfRange(uint256 end, uint256 nullifierCount);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error PointNotOnCurve(IProtocolAdapter.Delta point);
    error PreExistingNullifier(bytes32 nullifier);
    error PreExistingRoot(bytes32 root);
    error ReentrancyGuardReentrantCall();
    error ResourceWithDeniedLogicRef(bytes32 logicRef, bool consumed);
    error RiscZeroVerifierPaused();
    error RiscZeroVerifierSelectorMismatch(bytes4 expected, bytes4 actual);
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnacceptedKindTreeRoot(bytes32 kindTreeRoot);
    error ZeroKindTreeRootNotAllowed();
    error ZeroLegacyProtocolAdapterNotAllowed();
    error ZeroLogicRefNotAllowed();
    error ZeroRiscZeroVerifierRouterNotAllowed();
    error ZeroRiscZeroVerifierSelectorNotAllowed();

    event ActionExecuted(bytes32 actionTreeRoot, bytes32[] nullifiers, bytes32[] consumedLogicRefs, bytes32[] commitments, bytes32[] createdLogicRefs);
    event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event CommitmentTreeMigrated(bytes32 indexed root, uint256 leafCount);
    event CommitmentTreeRootAdded(bytes32 root);
    event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);
    event Initialized(uint64 version);
    event KindTreeRootUpdated(bytes32 indexed kindTreeRoot);
    event LogicRefDenied(bytes32 indexed logicRef, bool consumed);
    event MigrationFinalized();
    event NullifierBatchMigrated(uint256 start, uint256 count);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address account);
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
    event TransactionExecuted(bytes32 indexed transactionId);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);

    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector, address legacyProtocolAdapter);

    function EMPTY_KIND_TREE_ROOT() external view returns (bytes32);
    function RISC_ZERO_VERIFIER_ROUTER() external view returns (address);
    function RISC_ZERO_VERIFIER_SELECTOR() external view returns (bytes4);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function VERSION() external view returns (string memory);
    function commitmentCount() external view returns (uint256 count);
    function commitmentTreeCapacity() external view returns (uint256 capacity);
    function commitmentTreeDepth() external view returns (uint8 depth);
    function commitmentTreeSides() external view returns (bytes32[] memory sides);
    function commitmentTreeZeros() external view returns (bytes32[] memory zeros);
    function denyLogicRefs(ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) external;
    function execute(IProtocolAdapter.Transaction memory transaction) external;
    function getImplementation() external view returns (address current);
    function getKindTreeRoot() external view returns (bytes32 kindTreeRoot);
    function getLegacyProtocolAdapter() external view returns (address legacyProtocolAdapter);
    function initialize(address initialOwner) external;
    function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
    function isLogicRefDenied(bytes32 logicRef, bool consumed) external view returns (bool isDenied);
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
    function latestCommitmentTreeRoot() external view returns (bytes32 root);
    function migrateCommitmentTree(bytes32[] memory sides) external;
    function migrateNullifierSet(uint256 start, uint256 count) external;
    function owner() external view returns (address);
    function pause() external;
    function paused() external view returns (bool isPaused);
    function proxiableUUID() external view returns (bytes32);
    function renounceOwnership() external;
    function riscZeroVerifierPaused() external view returns (bool isPaused);
    function setKindTreeRoot(bytes32 newKindTreeRoot) external;
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
      },
      {
        "name": "legacyProtocolAdapter",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "EMPTY_KIND_TREE_ROOT",
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
    "name": "denyLogicRefs",
    "inputs": [
      {
        "name": "logicRefs",
        "type": "tuple[]",
        "internalType": "struct ILogicRefDenylist.DeniedLogicRef[]",
        "components": [
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
            "name": "kindTreeRoot",
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
    "name": "getKindTreeRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "kindTreeRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLegacyProtocolAdapter",
    "inputs": [],
    "outputs": [
      {
        "name": "legacyProtocolAdapter",
        "type": "address",
        "internalType": "address"
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
    "name": "isCommitmentTreeRootHistorical",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "isHistorical",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isLogicRefDenied",
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
    ],
    "outputs": [
      {
        "name": "isDenied",
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
    "name": "migrateCommitmentTree",
    "inputs": [
      {
        "name": "sides",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "migrateNullifierSet",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "setKindTreeRoot",
    "inputs": [
      {
        "name": "newKindTreeRoot",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "CommitmentTreeMigrated",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "leafCount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "KindTreeRootUpdated",
    "inputs": [
      {
        "name": "kindTreeRoot",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LogicRefDenied",
    "inputs": [
      {
        "name": "logicRef",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "consumed",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MigrationFinalized",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NullifierBatchMigrated",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "count",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "AfterTagNotInTransaction",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "BeforeTagNotInTransaction",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "CommitmentCountMismatch",
    "inputs": [
      {
        "name": "expected",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "actual",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CommitmentTreeNotEmpty",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CommitmentTreeRootMismatch",
    "inputs": [
      {
        "name": "expected",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "actual",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "CyclicConstraints",
    "inputs": []
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
    "name": "EmptyCommitmentTreeNotAllowed",
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
    "name": "LeafCountExceedsCapacity",
    "inputs": [
      {
        "name": "leafCount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "capacity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "LegacyProtocolAdapterNotStopped",
    "inputs": [
      {
        "name": "legacyProtocolAdapter",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "LogicRefAlreadyDenied",
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
    "name": "MissingHistoricalRoot",
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
    "name": "MissingNullifier",
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
    "name": "NullifierBatchLeavesGap",
    "inputs": [
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "NullifierBatchOutOfRange",
    "inputs": [
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nullifierCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
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
    "name": "UnacceptedKindTreeRoot",
    "inputs": [
      {
        "name": "kindTreeRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroKindTreeRootNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroLegacyProtocolAdapterNotAllowed",
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
pub mod MigrationalProtocolAdapter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101003461016457601f61577e38819003918201601f19168301916001600160401b0383118484101761016857808492606094604052833981010312610164576100488161017c565b60208201519063ffffffff60e01b82169283830361016457604061006c910161017c565b9230608052610079610190565b610081610190565b610089610190565b610091610190565b6001600160a01b0382161561015557156101465760a05260c0526100b3610190565b6001600160a01b038116156101375760e05260405161553790816102278239608051818181612613015261276b015260a0518181816113c60152818161253301526135d8015260c051818181610a2c015281816113860152613596015260e05181818161030901528181612a4301528181612eec0152818161312d01526139c60152f35b630286b4a360e21b5f5260045ffd5b63b1b25aaf60e01b5f5260045ffd5b63536c150160e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361016457565b5f51602061575e5f395f51905f525460ff8160401c16610217576002600160401b03196001600160401b038216016101c55750565b6001600160401b0319166001600160401b039081175f51602061575e5f395f51905f52556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a1565b63f92ee8a960e01b5f5260045ffdfe60a06040526004361015610011575f80fd5b5f5f3560e01c806303a7fde5146130dd578063171eac18146130b25780631a8473bc14612f10578063364f375b14612ec05780633f4ba83a14612a145780634f1ef2861461271f57806351945b061461268b57806352d1902d146125ec5780635c975abb146125ab57806364c12af21461257157806367353122146125575780636bf0a04b14612507578063715018a61461244b5780638456cb59146123b45780638b33e200146123215780638da5cb5b146122cf5780639239ac0c146110fa578063a06056f7146110ba578063aaf10f4214611067578063ad3cb1cc14611006578063bdeb442d14610fe3578063c1b0bed714610f95578063c44956d114610f58578063c4d66de814610a50578063e35a5d2f146109f3578063ed83cdc71461094b578063f2fde38b1461091e578063f8adf4c3146108c9578063fa83dd8f14610261578063fb0ff68014610224578063fe18ab91146101e15763ffa1ad741461017a575f80fd5b346101de57806003193601126101de57506101da60405161019c604082613420565b600b81527f322e302e302d72632e313100000000000000000000000000000000000000000060208201526040519182916020835260208301906134e6565b0390f35b80fd5b50346101de57806003193601126101de576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b50346101de57806003193601126101de5760207f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb0054604051908152f35b50346101de5760206003193601126101de5760043567ffffffffffffffff81116108c557366023820112156108c55780600401359067ffffffffffffffff82116108c1573660248360051b830101116108c1576102bc6138ef565b6102c461395b565b6102cc6139af565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900546108995773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690604051927fc44956d1000000000000000000000000000000000000000000000000000000008452602084600481865afa93841561088e57859461085a575b5083156108325760ff81116108025760ff8116916001831b808610156107d25750847f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900556801000000000000000082116107be577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154827f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90155808310610757575b507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90186526024017fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f865b83811061074357505050506001810180821161072f5761047290613746565b80511561071b5760208101917fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068352855b8181106106c8575050519067ffffffffffffffff82116106b4576801000000000000000082116106b4577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254827f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025580831061064d575b507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90285527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431855b838110610639576040517fbdeb442d0000000000000000000000000000000000000000000000000000000081528790876020826004818b5afa91821561062e5783926105f5575b5060207f9cfc151cd0c8c49916e15e9af15c5d553ff85cc13d96fef94340655fd6794e9f916105e26105d8613d60565b85819682146138b8565b6105eb84613ca0565b604051908152a280f35b9091506020813d602011610626575b8161061160209383613420565b8101031261062257519060206105a8565b5f80fd5b3d9150610604565b6040513d85823e3d90fd5b600190602084519401938184015501610561565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90286528290037fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301865b8281106106a657505061051a565b808860019284015501610698565b602485634e487b7160e01b81526041600452fd5b6106e76106d582856137b7565b516106e083866137b7565b519061463e565b60018201808311610707579061070060019392866137b7565b52016104a3565b602489634e487b7160e01b81526011600452fd5b602485634e487b7160e01b81526032600452fd5b602485634e487b7160e01b81526011600452fd5b600190602084359401938184015501610453565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90187528290037fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f8301875b8281106107b0575050610409565b8089600192840155016107a2565b602486634e487b7160e01b81526041600452fd5b86604491877f6be696a3000000000000000000000000000000000000000000000000000000008352600452602452fd5b7f6dfcc6500000000000000000000000000000000000000000000000000000000085526008600452602452604484fd5b6004857f4cd41ad8000000000000000000000000000000000000000000000000000000008152fd5b9093506020813d602011610886575b8161087660209383613420565b810103126106225751925f610368565b3d9150610869565b6040513d87823e3d90fd5b6004837f66fda36f000000000000000000000000000000000000000000000000000000008152fd5b8280fd5b5080fd5b50346101de5760206003193601126101de5760206109146004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b6040519015158152f35b50346101de5760206003193601126101de5761094861093b6133a9565b6109436138ef565b6137cb565b80f35b50346101de57806003193601126101de576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9018054808352908352909160208301917fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f915b8181106109dd576101da856109c981870382613420565b6040519182916020835260208301906134b3565b82548452602090930192600192830192016109b2565b50346101de57806003193601126101de5760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101de5760206003193601126101de57610a6a6133a9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549060ff8260401c16159167ffffffffffffffff811680159081610f50575b6001149081610f46575b159081610f3d575b50610f155790818360017fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000610b2d9516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610ec0575b50610b1d614681565b610b25614681565b610943614681565b610b35614681565b610b3d614681565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025468010000000000000000811015610eac5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015610e98577fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9028452602084200155817f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b0682527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260408220600160ff198254161790557f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a1610cb4614681565b610cbc614681565b610cc4614681565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e28380a2610d5461354d565b610e7057610d60613b4c565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1610ddc5780f35b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a180f35b6004827f25ae6eaa000000000000000000000000000000000000000000000000000000008152fd5b602483634e487b7160e01b81526032600452fd5b602483634e487b7160e01b81526041600452fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00555f610b14565b6004847ff92ee8a9000000000000000000000000000000000000000000000000000000008152fd5b9050155f610abc565b303b159150610ab4565b849150610aaa565b50346101de57806003193601126101de5760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b50346101de5760206003193601126101de5760ff604060209260043581527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0084522054166040519015158152f35b50346101de57806003193601126101de576020610ffe613d60565b604051908152f35b50346101de57806003193601126101de57506101da604051611029604082613420565b600581527f352e302e3000000000000000000000000000000000000000000000000000000060208201526040519182916020835260208301906134e6565b50346101de57806003193601126101de57602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b50346101de57806003193601126101de57602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b50346106225760206003193601126106225767ffffffffffffffff6004351161062257608060031960043536030112610622577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c6122a75760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d61117f613b4c565b61118d6004803501806136da565b8091501561227f577f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb005460646004350135148015612250575b1561221e576111d481613746565b906111dd613b9f565b506040516111ea816133cc565b5f81525f6020820152915f915f915f915f905b808210611a59575050806020611245925160051b9101209461123c61123661122f602460043501600435600401613c4f565b369161345f565b87614f09565b90939193614f43565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2016911690808203611a2b575061128890506004803501806136da565b906040519180608084017f0ebc6c35680d6c45d27f0686aaa3a4d0331220c5bd2e52ce1195a5bce1033e0860208601526064600435013560408601526060808601525260a083019060a08160051b8501019280925f915b83831061181157505050505060208261130382945f9403601f198101835282613420565b604051918183925191829101835e8101838152039060025afa156117d8575f51611337604460043501600435600401613c4f565b80600411610622577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168082036117e357505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15610622575f9261143192604051958694859384937fab750e75000000000000000000000000000000000000000000000000000000008552606060048601526064850191613f7c565b907f896fd82f0d437a4c50a3d9986bcc1c03bb4f9e9bf83b0243dbb5907dd98b6cb66024840152604483015203915afa80156117d8576117c3575b50816114d5575b5050806114c6575b507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce8280a2807f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d80f35b6114cf90613ca0565b5f61147b565b6114e36004803501806136da565b9091156117175785906114f58461372e565b926115036040519485613420565b848452601f196115128661372e565b01885b8181106116e857505087885b8381106115c457505090611536929184614aa5565b855b83811061154b57505050505b5f80611473565b61155f61155882846137b7565b51846137b7565b51606081018051519160200190895b8381106115815750505050600101611538565b806115be61159260019385516137b7565b5185519073ffffffffffffffffffffffffffffffffffffffff8151166040602083015192015192614d96565b0161156e565b6115d86115d2828686613777565b806136da565b8b5b818110611691575050506115fc6115f2828686613777565b60208101906136da565b8b5b81811061161057505050600101611521565b8861161c828486613c0f565b61163661162c6040830183613bb7565b60408101906136da565b9182611649575b505050506001016115fe565b9161167f916001959c9361166b8b611665611685989d91613c01565b9c6137b7565b518a6040820152602082013591359061505f565b9061350b565b979050885f808061163d565b8861169d828486613bea565b6116ad61162c6060830183613bb7565b91826116c0575b505050506001016115da565b9161167f916001959c9361166b8b6116656116dc989d91613c01565b979050885f80806116b4565b6020906040516116f7816133e8565b8b81528b838201528b604082015260608082015282828901015201611515565b909150845b81811061172b57505050611544565b6117396115d2828486613777565b875b81811061179a575050506117536115f2828486613777565b875b8181106117675750505060010161171c565b80611794602061177a6001948688613c0f565b61178a61162c6040830183613bb7565b9290910135614a08565b01611755565b806117bd60206117ad6001948688613bea565b61178a61162c6060830183613bb7565b0161173b565b6117d09195505f90613420565b5f935f61146c565b6040513d5f823e3d90fd5b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b90919293947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608782030183528535907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61833603018212156106225760a0810191839061187f81830180614453565b809560a086525260c0840160c08660051b86010195825f5b8281106119a45750505050506118b4602083830101838301614453565b94908482036020860152858252602082019060208760051b8401019681935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b83811061193e57505050505050910160408181013590840152606080820135908401526080908101359201919091525094602090810194936001019201906112df565b91939599909294969750601f1984820301875289358681121561062257602061199060019360606119828885960180358452858101358685015260408101906144a6565b9181604082015201906145c7565b9b0197019101918a979695939194926118fb565b919397600191939596506020611a19826080611a0b6119eb8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f889903018d528a6144a6565b8035845285810135868501526040810135604085015260608101906144a6565b9181606082015201906145c7565b99019501910191889594939192611897565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9095611a7387611a6d6004803501806136da565b90613777565b6080525f5f5f91611a886080516080516136da565b9050611a9381613746565b90611a9d81613746565b905f5b8181106120be575050611aba6020608051016080516136da565b919050611ac682613746565b90611ad083613746565b925f905b808210611c7c575050611b2d92611b497f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc329593611b3b611b5794604051978897608080510135895260a060208a015260a08901906134b3565b9087820360408901526134b3565b9085820360608701526134b3565b9083820360808501526134b3565b0390a180611c74575b5080611c54575b505060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0608051360301126106225760405190611ba4826133cc565b6040608051013582526020820160606080510135815260405192611bc7846133cc565b5f84525f6020850152611bdd81518351906141d7565b15611c1c5791611bfc9160019493602083519301519051915192614239565b6020830152815296608080510135611c1482866137b7565b5201906111fd565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b611c5e919661350b565b94611c6b575b5f80611b67565b60019350611c64565b97505f611b60565b909550611c9a86611c946020608051016080516136da565b90613c0f565b95611ca95f6020890135613b0b565b61208a5760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054611cfd81613c01565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90055908835915f5b828110611fa65750507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b14611e0b575b509660408101611d9161162c8284613bb7565b80611dd4575b505081611db3611dac60209360019695613bb7565b8235613fb3565b8035611dbf84896137b7565b520135611dcc82886137b7565b520190611ad4565b909a611ddf90613c01565b9a8c611d97576020929c50611dac611dfe6001969593611db393613ee2565b9d93505081939450611d97565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015468010000000000000000811015611f925760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90155811015611f7e577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f018290557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431015490611eee90611ee790839061463e565b918061463e565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025468010000000000000000811015611f925760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015611f7e577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2001555f611d7e565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52604160045260245ffd5b9092600190818516612033577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301546120289161463e565b935b811c9101611d25565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154612084919061463e565b9361202a565b6020877f597abd20000000000000000000000000000000000000000000000000000000005f5201356004525f60245260445ffd5b6120d6816120d06080516080516136da565b90613bea565b6120e560016020830135613b0b565b6121e9576040810135612123815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b156121be5750803561213481613a76565b6060820161214561162c8285613bb7565b80612185575b5050916020916121676121616001969584613bb7565b82613fb3565b61217184896137b7565b52013561217e82866137b7565b5201611aa0565b909961219090613c01565b998b61214b5760019594929b506121616121b06020959361216793613ee2565b9c939596505081935061214b565b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6020907f597abd20000000000000000000000000000000000000000000000000000000005f520135600452600160245260445ffd5b7fc93ee163000000000000000000000000000000000000000000000000000000005f526064600435013560045260245ffd5b507fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b0660646004350135146111c6565b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610622575f60031936011261062257602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346106225760206003193601126106225760043561233d6138ef565b801561238c57807f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e25f80a2005b7f0bf7718c000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610622575f600319360112610622576123cc6138ef565b6123d4613b4c565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b34610622575f600319360112610622576124636138ef565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610622575f60031936011261062257602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610622575f60031936011261062257602061091461354d565b34610622575f6003193601126106225760206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152f35b34610622575f60031936011261062257602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b34610622575f6003193601126106225773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001630036126635760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610622575f600319360112610622576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f20905f5b818110612709576101da856109c981870382613420565b82548452602090930192600192830192016126f2565b6040600319360112610622576127336133a9565b60243567ffffffffffffffff811161062257612753903690600401613495565b9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168030149081156129d2575b50612663576127a36138ef565b604051917f0418479ca6ac8dbf83626a5e3cdc5915233f1a7570c935115db0acf533b649bf5f80a173ffffffffffffffffffffffffffffffffffffffff8216927f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f918161299e575b5061284857837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8592036129735750823b1561294857807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a280511561291757612915916146d8565b005b50503461292057005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d6020116129ca575b816129ba60209383613420565b8101031261062257519085612817565b3d91506129ad565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416141583612796565b34610622575f60031936011261062257612a2c6138ef565b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517fc44956d1000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91612e8e575b507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005490808203612e6057826040517fbdeb442d000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91612e2e575b50612b1e612b15613d60565b828082146138b8565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9036020527f0939a46aa27b548df60f716e5dceb508ef327534304dc662fe5cf37ce9eb37d65460ff1615612de257612bc4815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b15612db757506040517f40f34d42000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91612d85575b5080612c8a575b612c1661395b565b60ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054167fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b5f198101908111612d71576020906024604051809481937f9ad91d4c00000000000000000000000000000000000000000000000000000000835260048301525afa9081156117d8575f91612d3f575b50805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f20541615612d145780612c0e565b7fdbc1aa1e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b90506020813d602011612d69575b81612d5a60209383613420565b81010312610622575181612cd9565b3d9150612d4d565b634e487b7160e01b5f52601160045260245ffd5b90506020813d602011612daf575b81612da060209383613420565b81010312610622575182612c07565b3d9150612d93565b7f79930a36000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f79930a36000000000000000000000000000000000000000000000000000000005f527fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b0660045260245ffd5b90506020813d602011612e58575b81612e4960209383613420565b81010312610622575182612b09565b3d9150612e3c565b7f33474d08000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b90506020813d602011612eb8575b81612ea960209383613420565b81010312610622575182612aa0565b3d9150612e9c565b34610622575f60031936011261062257602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346106225760206003193601126106225760043567ffffffffffffffff8111610622573660238201121561062257806004013567ffffffffffffffff8111610622576024820191602436918360061b0101116106225790612f6f6138ef565b5f915b808310612f7b57005b612f86838284613525565b35926020612f95828486613525565b0135938415159485810361062257811561308a57612fb38183613b0b565b61305a57602060019495967fa70f4433615e083adf1b97e0c32d9a721c39c26d6dd6c28fe96176ab909f0eca92845f527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900835260405f20905f1461302b578660ff198254161790555b604051908152a2019190612f72565b6101007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff82541617905561301c565b85827f127d335e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7f7af62341000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461062257604060031936011261062257602435801515810361062257610914602091600435613b0b565b34610622576040600319360112610622576004356024356130fc6138ef565b61310461395b565b61310c6139af565b613116818361350b565b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517f40f34d42000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91613377575b50808311613348575083158015613293575b1561326757835b8281106131dc577f63b187673d450ed7145495477aa47714450eb17d84b291d809daf29da80967586040868682519182526020820152a1005b604051907f9ad91d4c000000000000000000000000000000000000000000000000000000008252806004830152602082602481865afa9182156117d8575f92613232575b5061322c600192613a76565b016131a3565b91506020823d821161325f575b8161324c60209383613420565b810103126106225790519061322c613220565b3d915061323f565b837fc65c7170000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b505f198401848111612d7157604051907f9ad91d4c0000000000000000000000000000000000000000000000000000000082526004820152602081602481855afa9081156117d8575f91613316575b505f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f20541661319c565b90506020813d602011613340575b8161333160209383613420565b810103126106225751856132e2565b3d9150613324565b827f716b33b3000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b90506020813d6020116133a1575b8161339260209383613420565b8101031261062257518561318a565b3d9150613385565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361062257565b6040810190811067ffffffffffffffff821117611f9257604052565b6080810190811067ffffffffffffffff821117611f9257604052565b6060810190811067ffffffffffffffff821117611f9257604052565b90601f601f19910116810190811067ffffffffffffffff821117611f9257604052565b67ffffffffffffffff8111611f9257601f01601f191660200190565b92919261346b82613443565b916134796040519384613420565b829481845281830111610622578281602093845f960137010152565b9080601f83011215610622578160206134b09335910161345f565b90565b90602080835192838152019201905f5b8181106134d05750505090565b82518452602093840193909201916001016134c3565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b91908201809211612d7157565b91908203918211612d7157565b9190811015611f7e5760061b0190565b90816020910312610622575180151581036106225790565b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa9081156117d8575f9161368a575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa9081156117d8575f91613661575090565b6134b0915060203d602011613683575b61367b8183613420565b810190613535565b503d613671565b90506020813d6020116136d2575b816136a560209383613420565b81010312610622575173ffffffffffffffffffffffffffffffffffffffff81168103610622576020613608565b3d9150613698565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610622570180359067ffffffffffffffff821161062257602001918160051b3603831361062257565b67ffffffffffffffff8111611f925760051b60200190565b906137508261372e565b61375d6040519182613420565b828152601f1961376d829461372e565b0190602036910137565b9190811015611f7e5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6181360301821215610622570190565b8051821015611f7e5760209160051b010190565b73ffffffffffffffffffffffffffffffffffffffff16801561388c5773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b156138c1575050565b7fd5df6dd6000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416330361392f57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054161561398757565b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517f5c975abb000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91613a57575b5015613a2c5750565b7ff4ef172d000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b613a70915060203d6020116136835761367b8183613420565b5f613a23565b805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f205416613ae0575f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260405f20600160ff19825416179055565b7f39a940c5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5f9081527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629006020526040902060ff9115613b4457541690565b5460081c1690565b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416613b7757565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b60405190613bac826133cc565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215610622570190565b90821015611f7e576134b09160051b810190613bb7565b5f198114612d715760010190565b9190811015611f7e5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301821215610622570190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610622570180359067ffffffffffffffff82116106225760200191813603831361062257565b805f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f205416613d35576020817f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d2925f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903825260405f20600160ff19825416179055604051908152a1565b7fdb788c2b000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b60ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900547f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2054925f5b818110613ddf57505050565b909193600190818616155f14613e4b577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431830154613e3f9161463e565b945b811c929101613dd3565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154613e9c919061463e565b94613e41565b9190811015611f7e5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215610622570190565b5f92918390815b818110613ef65750505050565b613f0e613f04828487613ea2565b6020810190613c4f565b84613f785780608011610622576020613f2c6060840135838561479f565b1592831593613f4f575b505050613f4557600101613ee9565b5060019450505050565b91955091505f948060a01161062257613f6e925060808201359161479f565b15155f8080613f36565b8480fd5b601f8260209493601f1993818652868601375f8582860101520116010190565b6040906134b0949281528160208201520191613f7c565b90613fbe81806136da565b5f5b81811061416857505050613fd760208201826136da565b5f5b8181106140f957505050613ff060408201826136da565b5f5b81811061408a5750505080606061400a9201906136da565b90915f5b82811061401b5750505050565b614026818486613ea2565b35906002821015610622576001809214614041575b0161400e565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c614071613f0484888a613ea2565b906140826040519283928784613f9c565b0390a261403b565b614095818385613ea2565b359060028210156106225760018092146140b0575b01613ff2565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596140e0613f04848789613ea2565b906140f16040519283928784613f9c565b0390a26140aa565b614104818385613ea2565b3590600282101561062257600180921461411f575b01613fd9565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f361414f613f04848789613ea2565b906141606040519283928784613f9c565b0390a2614119565b614173818385613ea2565b3590600282101561062257600180921461418e575b01613fc0565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66141be613f04848789613ea2565b906141cf6040519283928784613f9c565b0390a2614188565b80158015614229575b8015614221575b8015614211575b61420b576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d0198210156141ee565b5081156141e7565b506401000003d0198110156141e0565b92939290915f908083036144395750506401000003d0195f94830861426257505090505f905f90565b6401000003d0198280096401000003d0195f8180808086880960040995800960030908906401000003d01991828480088303838111612d7157839081838009089384840392848411612d715784809180096008098403848111612d715784600193818096819508900908940960020991905b8215158381614428575b5080614420575b156143c2579084939293906001936401000003d0199187965b801561438857808404968098614374576401000003d0199088096401000003d019036401000003d0198111612d71576401000003d019908a960897938197828102928184041490151715614360579061435691613518565b95929396956142fe565b60248a634e487b7160e01b81526011600452fd5b60248a634e487b7160e01b81526012600452fd5b50945094505093806143ae5750906401000003d019809281808780098092099509900990565b80634e487b7160e01b602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b5060016142e5565b6401000003d019915014155f6142de565b61444d936401000003d019939692966147d8565b916142d4565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561062257016020813591019167ffffffffffffffff8211610622578160051b3603831361062257565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8182360301811215610622570190565b906020838281520160208260051b85010193835f917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc182360301905b858410614525575050505050505090565b90919293949596601f198282030186528735838112156106225784018035600281101561062257825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561062257016020813591019067ffffffffffffffff8111610622578036038213610622576145b86020928392604086818660019901520191613f7c565b99019796019401929190614514565b6134b09161463061462561460a6145ef6145e18680614453565b6080875260808701916144d8565b6145fc6020870187614453565b9086830360208801526144d8565b6146176040860186614453565b9085830360408701526144d8565b926060810190614453565b9160608185039101526144d8565b5f9060209260405190848201928352604082015260408152614661606082613420565b604051918291518091835e8101838152039060025afa156117d8575f5190565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c16156146b057565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b905f8091602081519101845af4808061478c575b1561470c5750506040513d81523d5f602083013e60203d82010160405290565b156147535773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d15614764576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d1515806146ec5750813b15156146ec565b602083019291838311612d7157838311610622578311610622578101359103602081106147ca575090565b5f199060200360031b1b1690565b9492918515806149df575b6149d3578015806149cb575b6149c1576040516080916148038383613420565b8236833786156149ad5786948580928180600180098087529781896001099c602088019d8e5282604089019d8e8c8152516001099160608a019283526040519e8f61484d906133e8565b5190098d525190099460208b019586525190099860408901998a525190096060870190815286518851148015906149a1575b1561494357849283808093816040519c8561489b8f9788613420565b368737518c516148ab9083613518565b900884525185516148bc9083613518565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a5251800988516148f99083613518565b900881808751855190096002096149109083613518565b90089c519351905190096149248c83613518565b900890099251905190096149389083613518565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b5081518151141561487f565b634e487b7160e01b5f52601260045260245ffd5b5092506001919050565b5082156147ef565b94509092506001919050565b5081156147e3565b359073ffffffffffffffffffffffffffffffffffffffff8216820361062257565b915f5b818110614a185750505050565b614a26613f04828486613ea2565b8192910160608382031261062257614a3d836149e7565b602084013567ffffffffffffffff81116106225782614a5d918601613495565b604085013567ffffffffffffffff811161062257614a97614a9f9473ffffffffffffffffffffffffffffffffffffffff9260019801613495565b921688614d96565b01614a0b565b909392606092614ab482613746565b93614abe83613746565b9060405195614acc87613404565b86526020860191825260408601905f8252614aef614ae986613746565b95613746565b60405195614afc87613404565b8652602086015260408501985f8a528651915f5b838110614bdc575050505090614b2f918551965191519051918761520e565b9551918215614bd55790816020614b4b9493519101519161520e565b614b5483613746565b945f918051935f5b858110614b6c5750505050505050565b5f5b878110614b7e5750600101614b5c565b806040614b97614b90600194896137b7565b51866137b7565b510151614ba484876137b7565b5114614bb1575b01614b6e565b614bbb81876137b7565b51614bcf614bc889613c01565b988d6137b7565b52614bab565b5050505050565b81614be7828b6137b7565b5101518051905f5b828110614c0157505050600101614b10565b8b8d8b89878f8a614c12888a6137b7565b5101518051905f5b828110614cdc575050506080614c30888a6137b7565b510151948551965f955b888710614c5257505050505050505050600101614bef565b614c5c87896137b7565b5190614c6882846153b4565b968714614c9057505091614c8193916001969593615482565b018f8f8f8f928a928e94614c3a565b9250945050614ca392508391508c6153f1565b15614cb15750600190614c81565b7f45bd4960000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b95614cee878399949a939796976137b7565b5190614cfa82846153b4565b958614614d2557505091614d1393916001969593615482565b018f918f938f8f93918d938b93614c1a565b9250945050614d3892508391508c6153f1565b15614d465750600190614d13565b7f211b12b7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091614d886134b0936040845260408401906134e6565b9160208184039101526134e6565b919073ffffffffffffffffffffffffffffffffffffffff1692604051927f33a892030000000000000000000000000000000000000000000000000000000084526004840152604060248401525f8380614df260448201866134e6565b038183885af19283156117d8575f93614e8d575b50825160208401208151602083012003614e525750907fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf191614e4d60405192839283614d71565b0390a2565b9050614e896040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401614d71565b0390fd5b9092503d805f833e614e9f8183613420565b8101906020818303126106225780519067ffffffffffffffff8211610622570181601f8201121561062257805190614ed682613443565b92614ee46040519485613420565b8284526020838301011161062257815f9260208093018386015e83010152915f614e06565b8151919060418303614f3957614f329250602082015190606060408401519301515f1a90615325565b9192909190565b50505f9160029190565b6004811015614fee5780614f55575050565b60018103614f85577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b60028103614fb957507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b600314614fc35750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b9080601f830112156106225781356150198161372e565b926150276040519485613420565b81845260208085019260051b82010192831161062257602001905b82821061504f5750505090565b8135815260209182019101615042565b90949392915f95825260208201526150768361372e565b906150846040519283613420565b838252601f196150938561372e565b015f5b8181106151bb5750506060019081525f925b8084106150b55750505050565b909192946150c48684516137b7565b51906150d4613f04888588613ea2565b8192910160a083820312610622576150eb836149e7565b93602084013567ffffffffffffffff8111610622578261510c918601613495565b94604085013567ffffffffffffffff8111610622578361512d918701613495565b91606086013567ffffffffffffffff8111610622578461514e918801615002565b9160808701359067ffffffffffffffff82116106225760019861518e61167f9773ffffffffffffffffffffffffffffffffffffffff946151b19b01615002565b95866080860152856060860152604085015260208401521690525190519061350b565b95019291906150a8565b604095939495519060a082019180831067ffffffffffffffff841117611f92576020926040525f815260608382015260606040820152606080820152606060808201528282880101520194939294615096565b939261521985613746565b925f5b8281106152f3575061522d86613746565b955f5b81811061523f57505050505050565b615248866154f5565b8281146152cb575f1961525b82896137b7565b5280615267838b6137b7565b525f5b85811061527b575050600101615230565b8161528682896137b7565b5114615295575b60010161526a565b61529f81866137b7565b51906152ab828a6137b7565b51918215612d71576152c35f1960019401918b6137b7565b52905061528d565b7fa8129095000000000000000000000000000000000000000000000000000000005f5260045ffd5b80615300600192846137b7565b5161531e615317615311838a6137b7565b51613c01565b91886137b7565b520161521c565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116153a9579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156117d8575f5173ffffffffffffffffffffffffffffffffffffffff81161561539f57905f905f90565b505f906001905f90565b5050505f9160039190565b91908251905f935b8285106153c857505050565b90919293826153d782846137b7565b5151146153ea57600101939291906153bc565b9250505090565b9092915f935f5b8181106154055750505050565b6154136115d2828487613777565b5f5b8181106154685750505061542d6115f2828487613777565b5f5b818110615441575050506001016153f8565b8561544d828486613c0f565b351461545b5760010161542f565b5060019750505050505050565b85615474828486613bea565b351461545b57600101615415565b91909392939160406154a2858261549989866137b7565b510151936137b7565b5101518082036154e3575b5050506020816154d49251946154c960408301968751906137b7565b5201518351906137b7565b526154df8151613c01565b9052565b9094509250905060206154d45f6154ad565b9081515f925b818410615506575050565b90919261551381836137b7565b5115615524576001019291906154fb565b9150509056fea164736f6c6343000825000af0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x004a\x01dW`\x1FaW~8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01hW\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\x01dWa\0H\x81a\x01|V[` \x82\x01Q\x90c\xFF\xFF\xFF\xFF`\xE0\x1B\x82\x16\x92\x83\x83\x03a\x01dW`@a\0l\x91\x01a\x01|V[\x920`\x80Ra\0ya\x01\x90V[a\0\x81a\x01\x90V[a\0\x89a\x01\x90V[a\0\x91a\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x01UW\x15a\x01FW`\xA0R`\xC0Ra\0\xB3a\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x017W`\xE0R`@QaU7\x90\x81a\x02'\x829`\x80Q\x81\x81\x81a&\x13\x01Ra'k\x01R`\xA0Q\x81\x81\x81a\x13\xC6\x01R\x81\x81a%3\x01Ra5\xD8\x01R`\xC0Q\x81\x81\x81a\n,\x01R\x81\x81a\x13\x86\x01Ra5\x96\x01R`\xE0Q\x81\x81\x81a\x03\t\x01R\x81\x81a*C\x01R\x81\x81a.\xEC\x01R\x81\x81a1-\x01Ra9\xC6\x01R\xF3[c\x02\x86\xB4\xA3`\xE2\x1B_R`\x04_\xFD[c\xB1\xB2Z\xAF`\xE0\x1B_R`\x04_\xFD[cSl\x15\x01`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01dWV[_Q` aW^_9_Q\x90_RT`\xFF\x81`@\x1C\x16a\x02\x17W`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01a\x01\xC5WPV[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17_Q` aW^_9_Q\x90_RU`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD\xFE`\xA0`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x03\xA7\xFD\xE5\x14a0\xDDW\x80c\x17\x1E\xAC\x18\x14a0\xB2W\x80c\x1A\x84s\xBC\x14a/\x10W\x80c6O7[\x14a.\xC0W\x80c?K\xA8:\x14a*\x14W\x80cO\x1E\xF2\x86\x14a'\x1FW\x80cQ\x94[\x06\x14a&\x8BW\x80cR\xD1\x90-\x14a%\xECW\x80c\\\x97Z\xBB\x14a%\xABW\x80cd\xC1*\xF2\x14a%qW\x80cg51\"\x14a%WW\x80ck\xF0\xA0K\x14a%\x07W\x80cqP\x18\xA6\x14a$KW\x80c\x84V\xCBY\x14a#\xB4W\x80c\x8B3\xE2\0\x14a#!W\x80c\x8D\xA5\xCB[\x14a\"\xCFW\x80c\x929\xAC\x0C\x14a\x10\xFAW\x80c\xA0`V\xF7\x14a\x10\xBAW\x80c\xAA\xF1\x0FB\x14a\x10gW\x80c\xAD<\xB1\xCC\x14a\x10\x06W\x80c\xBD\xEBD-\x14a\x0F\xE3W\x80c\xC1\xB0\xBE\xD7\x14a\x0F\x95W\x80c\xC4IV\xD1\x14a\x0FXW\x80c\xC4\xD6m\xE8\x14a\nPW\x80c\xE3Z]/\x14a\t\xF3W\x80c\xED\x83\xCD\xC7\x14a\tKW\x80c\xF2\xFD\xE3\x8B\x14a\t\x1EW\x80c\xF8\xAD\xF4\xC3\x14a\x08\xC9W\x80c\xFA\x83\xDD\x8F\x14a\x02aW\x80c\xFB\x0F\xF6\x80\x14a\x02$W\x80c\xFE\x18\xAB\x91\x14a\x01\xE1Wc\xFF\xA1\xADt\x14a\x01zW_\x80\xFD[4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEWPa\x01\xDA`@Qa\x01\x9C`@\x82a4 V[`\x0B\x81R\x7F2.0.0-rc.11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xE6V[\x03\x90\xF3[\x80\xFD[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` \x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T`@Q\x90\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\xC5W6`#\x82\x01\x12\x15a\x08\xC5W\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08\xC1W6`$\x83`\x05\x1B\x83\x01\x01\x11a\x08\xC1Wa\x02\xBCa8\xEFV[a\x02\xC4a9[V[a\x02\xCCa9\xAFV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ta\x08\x99Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90`@Q\x92\x7F\xC4IV\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R` \x84`\x04\x81\x86Z\xFA\x93\x84\x15a\x08\x8EW\x85\x94a\x08ZW[P\x83\x15a\x082W`\xFF\x81\x11a\x08\x02W`\xFF\x81\x16\x91`\x01\x83\x1B\x80\x86\x10\x15a\x07\xD2WP\x84\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Uh\x01\0\0\0\0\0\0\0\0\x82\x11a\x07\xBEW\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x82\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x80\x83\x10a\x07WW[P\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01\x86R`$\x01\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x86[\x83\x81\x10a\x07CWPPPP`\x01\x81\x01\x80\x82\x11a\x07/Wa\x04r\x90a7FV[\x80Q\x15a\x07\x1BW` \x81\x01\x91\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x83R\x85[\x81\x81\x10a\x06\xC8WPPQ\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\xB4Wh\x01\0\0\0\0\0\0\0\0\x82\x11a\x06\xB4W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x82\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x80\x83\x10a\x06MW[P\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02\x85R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x85[\x83\x81\x10a\x069W`@Q\x7F\xBD\xEBD-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x87\x90\x87` \x82`\x04\x81\x8BZ\xFA\x91\x82\x15a\x06.W\x83\x92a\x05\xF5W[P` \x7F\x9C\xFC\x15\x1C\xD0\xC8\xC4\x99\x16\xE1^\x9A\xF1\\]U?\xF8\\\xC1=\x96\xFE\xF9C@e_\xD6yN\x9F\x91a\x05\xE2a\x05\xD8a=`V[\x85\x81\x96\x82\x14a8\xB8V[a\x05\xEB\x84a<\xA0V[`@Q\x90\x81R\xA2\x80\xF3[\x90\x91P` \x81=` \x11a\x06&W[\x81a\x06\x11` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x90` a\x05\xA8V[_\x80\xFD[=\x91Pa\x06\x04V[`@Q=\x85\x82>=\x90\xFD[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x05aV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02\x86R\x82\x90\x03\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01\x86[\x82\x81\x10a\x06\xA6WPPa\x05\x1AV[\x80\x88`\x01\x92\x84\x01U\x01a\x06\x98V[`$\x85cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[a\x06\xE7a\x06\xD5\x82\x85a7\xB7V[Qa\x06\xE0\x83\x86a7\xB7V[Q\x90aF>V[`\x01\x82\x01\x80\x83\x11a\x07\x07W\x90a\x07\0`\x01\x93\x92\x86a7\xB7V[R\x01a\x04\xA3V[`$\x89cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x85cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[`$\x85cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`\x01\x90` \x845\x94\x01\x93\x81\x84\x01U\x01a\x04SV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01\x87R\x82\x90\x03\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x87[\x82\x81\x10a\x07\xB0WPPa\x04\tV[\x80\x89`\x01\x92\x84\x01U\x01a\x07\xA2V[`$\x86cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x86`D\x91\x87\x7Fk\xE6\x96\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04R`$R\xFD[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x08`\x04R`$R`D\x84\xFD[`\x04\x85\x7FL\xD4\x1A\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90\x93P` \x81=` \x11a\x08\x86W[\x81a\x08v` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x92_a\x03hV[=\x91Pa\x08iV[`@Q=\x87\x82>=\x90\xFD[`\x04\x83\x7Ff\xFD\xA3o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x82\x80\xFD[P\x80\xFD[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEW` a\t\x14`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEWa\tHa\t;a3\xA9V[a\tCa8\xEFV[a7\xCBV[\x80\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01\x80T\x80\x83R\x90\x83R\x90\x91` \x83\x01\x91\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x91[\x81\x81\x10a\t\xDDWa\x01\xDA\x85a\t\xC9\x81\x87\x03\x82a4 V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xB3V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\t\xB2V[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEWa\nja3\xA9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x90`\xFF\x82`@\x1C\x16\x15\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x0FPW[`\x01\x14\x90\x81a\x0FFW[\x15\x90\x81a\x0F=W[Pa\x0F\x15W\x90\x81\x83`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x0B-\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x0E\xC0W[Pa\x0B\x1DaF\x81V[a\x0B%aF\x81V[a\tCaF\x81V[a\x0B5aF\x81V[a\x0B=aF\x81V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0E\xACW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x0E\x98W\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02\x84R` \x84 \x01U\x81\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x82R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`@\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x0C\xB4aF\x81V[a\x0C\xBCaF\x81V[a\x0C\xC4aF\x81V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2\x83\x80\xA2a\rTa5MV[a\x0EpWa\r`a;LV[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1a\r\xDCW\x80\xF3[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\x80\xF3[`\x04\x82\x7F%\xAEn\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`$\x83cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[`$\x83cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U_a\x0B\x14V[`\x04\x84\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x15_a\n\xBCV[0;\x15\x91Pa\n\xB4V[\x84\x91Pa\n\xAAV[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEW`\xFF`@` \x92`\x045\x81R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` a\x0F\xFEa=`V[`@Q\x90\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEWPa\x01\xDA`@Qa\x10)`@\x82a4 V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xE6V[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[P4a\x06\"W` `\x03\x196\x01\x12a\x06\"Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x06\"W`\x80`\x03\x19`\x0456\x03\x01\x12a\x06\"W\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\"\xA7W`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x11\x7Fa;LV[a\x11\x8D`\x04\x805\x01\x80a6\xDAV[\x80\x91P\x15a\"\x7FW\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T`d`\x045\x015\x14\x80\x15a\"PW[\x15a\"\x1EWa\x11\xD4\x81a7FV[\x90a\x11\xDDa;\x9FV[P`@Qa\x11\xEA\x81a3\xCCV[_\x81R_` \x82\x01R\x91_\x91_\x91_\x91_\x90[\x80\x82\x10a\x1AYWPP\x80` a\x12E\x92Q`\x05\x1B\x91\x01 \x94a\x12<a\x126a\x12/`$`\x045\x01`\x045`\x04\x01a<OV[6\x91a4_V[\x87aO\tV[\x90\x93\x91\x93aOCV[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a\x1A+WPa\x12\x88\x90P`\x04\x805\x01\x80a6\xDAV[\x90`@Q\x91\x80`\x80\x84\x01\x7F\x0E\xBCl5h\rlE\xD2\x7F\x06\x86\xAA\xA3\xA4\xD03\x12 \xC5\xBD.R\xCE\x11\x95\xA5\xBC\xE1\x03>\x08` \x86\x01R`d`\x045\x015`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01\x90`\xA0\x81`\x05\x1B\x85\x01\x01\x92\x80\x92_\x91[\x83\x83\x10a\x18\x11WPPPPP` \x82a\x13\x03\x82\x94_\x94\x03`\x1F\x19\x81\x01\x83R\x82a4 V[`@Q\x91\x81\x83\x92Q\x91\x82\x91\x01\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x17\xD8W_Qa\x137`D`\x045\x01`\x045`\x04\x01a<OV[\x80`\x04\x11a\x06\"W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x17\xE3WPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x06\"W_\x92a\x141\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a?|V[\x90\x7F\x89o\xD8/\rCzLP\xA3\xD9\x98k\xCC\x1C\x03\xBBO\x9E\x9B\xF8;\x02C\xDB\xB5\x90}\xD9\x8Bl\xB6`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a\x17\xD8Wa\x17\xC3W[P\x81a\x14\xD5W[PP\x80a\x14\xC6W[P\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCE\x82\x80\xA2\x80\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]\x80\xF3[a\x14\xCF\x90a<\xA0V[_a\x14{V[a\x14\xE3`\x04\x805\x01\x80a6\xDAV[\x90\x91\x15a\x17\x17W\x85\x90a\x14\xF5\x84a7.V[\x92a\x15\x03`@Q\x94\x85a4 V[\x84\x84R`\x1F\x19a\x15\x12\x86a7.V[\x01\x88[\x81\x81\x10a\x16\xE8WPP\x87\x88[\x83\x81\x10a\x15\xC4WPP\x90a\x156\x92\x91\x84aJ\xA5V[\x85[\x83\x81\x10a\x15KWPPPP[_\x80a\x14sV[a\x15_a\x15X\x82\x84a7\xB7V[Q\x84a7\xB7V[Q``\x81\x01\x80QQ\x91` \x01\x90\x89[\x83\x81\x10a\x15\x81WPPPP`\x01\x01a\x158V[\x80a\x15\xBEa\x15\x92`\x01\x93\x85Qa7\xB7V[Q\x85Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16`@` \x83\x01Q\x92\x01Q\x92aM\x96V[\x01a\x15nV[a\x15\xD8a\x15\xD2\x82\x86\x86a7wV[\x80a6\xDAV[\x8B[\x81\x81\x10a\x16\x91WPPPa\x15\xFCa\x15\xF2\x82\x86\x86a7wV[` \x81\x01\x90a6\xDAV[\x8B[\x81\x81\x10a\x16\x10WPPP`\x01\x01a\x15!V[\x88a\x16\x1C\x82\x84\x86a<\x0FV[a\x166a\x16,`@\x83\x01\x83a;\xB7V[`@\x81\x01\x90a6\xDAV[\x91\x82a\x16IW[PPPP`\x01\x01a\x15\xFEV[\x91a\x16\x7F\x91`\x01\x95\x9C\x93a\x16k\x8Ba\x16ea\x16\x85\x98\x9D\x91a<\x01V[\x9Ca7\xB7V[Q\x8A`@\x82\x01R` \x82\x015\x915\x90aP_V[\x90a5\x0BV[\x97\x90P\x88_\x80\x80a\x16=V[\x88a\x16\x9D\x82\x84\x86a;\xEAV[a\x16\xADa\x16,``\x83\x01\x83a;\xB7V[\x91\x82a\x16\xC0W[PPPP`\x01\x01a\x15\xDAV[\x91a\x16\x7F\x91`\x01\x95\x9C\x93a\x16k\x8Ba\x16ea\x16\xDC\x98\x9D\x91a<\x01V[\x97\x90P\x88_\x80\x80a\x16\xB4V[` \x90`@Qa\x16\xF7\x81a3\xE8V[\x8B\x81R\x8B\x83\x82\x01R\x8B`@\x82\x01R``\x80\x82\x01R\x82\x82\x89\x01\x01R\x01a\x15\x15V[\x90\x91P\x84[\x81\x81\x10a\x17+WPPPa\x15DV[a\x179a\x15\xD2\x82\x84\x86a7wV[\x87[\x81\x81\x10a\x17\x9AWPPPa\x17Sa\x15\xF2\x82\x84\x86a7wV[\x87[\x81\x81\x10a\x17gWPPP`\x01\x01a\x17\x1CV[\x80a\x17\x94` a\x17z`\x01\x94\x86\x88a<\x0FV[a\x17\x8Aa\x16,`@\x83\x01\x83a;\xB7V[\x92\x90\x91\x015aJ\x08V[\x01a\x17UV[\x80a\x17\xBD` a\x17\xAD`\x01\x94\x86\x88a;\xEAV[a\x17\x8Aa\x16,``\x83\x01\x83a;\xB7V[\x01a\x17;V[a\x17\xD0\x91\x95P_\x90a4 V[_\x93_a\x14lV[`@Q=_\x82>=\x90\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x87\x82\x03\x01\x83R\x855\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x82\x12\x15a\x06\"W`\xA0\x81\x01\x91\x83\x90a\x18\x7F\x81\x83\x01\x80aDSV[\x80\x95`\xA0\x86RR`\xC0\x84\x01`\xC0\x86`\x05\x1B\x86\x01\x01\x95\x82_[\x82\x81\x10a\x19\xA4WPPPPPa\x18\xB4` \x83\x83\x01\x01\x83\x83\x01aDSV[\x94\x90\x84\x82\x03` \x86\x01R\x85\x82R` \x82\x01\x90` \x87`\x05\x1B\x84\x01\x01\x96\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x19>WPPPPPP\x91\x01`@\x81\x81\x015\x90\x84\x01R``\x80\x82\x015\x90\x84\x01R`\x80\x90\x81\x015\x92\x01\x91\x90\x91RP\x94` \x90\x81\x01\x94\x93`\x01\x01\x92\x01\x90a\x12\xDFV[\x91\x93\x95\x99\x90\x92\x94\x96\x97P`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x06\"W` a\x19\x90`\x01\x93``a\x19\x82\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90aD\xA6V[\x91\x81`@\x82\x01R\x01\x90aE\xC7V[\x9B\x01\x97\x01\x91\x01\x91\x8A\x97\x96\x95\x93\x91\x94\x92a\x18\xFBV[\x91\x93\x97`\x01\x91\x93\x95\x96P` a\x1A\x19\x82`\x80a\x1A\x0Ba\x19\xEB\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F\x88\x99\x03\x01\x8DR\x8AaD\xA6V[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90aD\xA6V[\x91\x81``\x82\x01R\x01\x90aE\xC7V[\x99\x01\x95\x01\x91\x01\x91\x88\x95\x94\x93\x91\x92a\x18\x97V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x95a\x1As\x87a\x1Am`\x04\x805\x01\x80a6\xDAV[\x90a7wV[`\x80R___\x91a\x1A\x88`\x80Q`\x80Qa6\xDAV[\x90Pa\x1A\x93\x81a7FV[\x90a\x1A\x9D\x81a7FV[\x90_[\x81\x81\x10a \xBEWPPa\x1A\xBA` `\x80Q\x01`\x80Qa6\xDAV[\x91\x90Pa\x1A\xC6\x82a7FV[\x90a\x1A\xD0\x83a7FV[\x92_\x90[\x80\x82\x10a\x1C|WPPa\x1B-\x92a\x1BI\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a\x1B;a\x1BW\x94`@Q\x97\x88\x97`\x80\x80Q\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a4\xB3V[\x90\x87\x82\x03`@\x89\x01Ra4\xB3V[\x90\x85\x82\x03``\x87\x01Ra4\xB3V[\x90\x83\x82\x03`\x80\x85\x01Ra4\xB3V[\x03\x90\xA1\x80a\x1CtW[P\x80a\x1CTW[PP`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x80Q6\x03\x01\x12a\x06\"W`@Q\x90a\x1B\xA4\x82a3\xCCV[`@`\x80Q\x015\x82R` \x82\x01```\x80Q\x015\x81R`@Q\x92a\x1B\xC7\x84a3\xCCV[_\x84R_` \x85\x01Ra\x1B\xDD\x81Q\x83Q\x90aA\xD7V[\x15a\x1C\x1CW\x91a\x1B\xFC\x91`\x01\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92aB9V[` \x83\x01R\x81R\x96`\x80\x80Q\x015a\x1C\x14\x82\x86a7\xB7V[R\x01\x90a\x11\xFDV[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[a\x1C^\x91\x96a5\x0BV[\x94a\x1CkW[_\x80a\x1BgV[`\x01\x93Pa\x1CdV[\x97P_a\x1B`V[\x90\x95Pa\x1C\x9A\x86a\x1C\x94` `\x80Q\x01`\x80Qa6\xDAV[\x90a<\x0FV[\x95a\x1C\xA9_` \x89\x015a;\x0BV[a \x8AW`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ta\x1C\xFD\x81a<\x01V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x885\x91_[\x82\x81\x10a\x1F\xA6WPP\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\x1E\x0BW[P\x96`@\x81\x01a\x1D\x91a\x16,\x82\x84a;\xB7V[\x80a\x1D\xD4W[PP\x81a\x1D\xB3a\x1D\xAC` \x93`\x01\x96\x95a;\xB7V[\x825a?\xB3V[\x805a\x1D\xBF\x84\x89a7\xB7V[R\x015a\x1D\xCC\x82\x88a7\xB7V[R\x01\x90a\x1A\xD4V[\x90\x9Aa\x1D\xDF\x90a<\x01V[\x9A\x8Ca\x1D\x97W` \x92\x9CPa\x1D\xACa\x1D\xFE`\x01\x96\x95\x93a\x1D\xB3\x93a>\xE2V[\x9D\x93PP\x81\x93\x94Pa\x1D\x97V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x1F\x92W`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x81\x10\x15a\x1F~W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x01\x82\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x01T\x90a\x1E\xEE\x90a\x1E\xE7\x90\x83\x90aF>V[\x91\x80aF>V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x1F\x92W`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x1F~W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x01U_a\x1D~V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90\x92`\x01\x90\x81\x85\x16a 3W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta (\x91aF>V[\x93[\x81\x1C\x91\x01a\x1D%V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta \x84\x91\x90aF>V[\x93a *V[` \x87\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R_`$R`D_\xFD[a \xD6\x81a \xD0`\x80Q`\x80Qa6\xDAV[\x90a;\xEAV[a \xE5`\x01` \x83\x015a;\x0BV[a!\xE9W`@\x81\x015a!#\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[\x15a!\xBEWP\x805a!4\x81a:vV[``\x82\x01a!Ea\x16,\x82\x85a;\xB7V[\x80a!\x85W[PP\x91` \x91a!ga!a`\x01\x96\x95\x84a;\xB7V[\x82a?\xB3V[a!q\x84\x89a7\xB7V[R\x015a!~\x82\x86a7\xB7V[R\x01a\x1A\xA0V[\x90\x99a!\x90\x90a<\x01V[\x99\x8Ba!KW`\x01\x95\x94\x92\x9BPa!aa!\xB0` \x95\x93a!g\x93a>\xE2V[\x9C\x93\x95\x96PP\x81\x93Pa!KV[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[` \x90\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R`\x01`$R`D_\xFD[\x7F\xC9>\xE1c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`d`\x045\x015`\x04R`$_\xFD[P\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06`d`\x045\x015\x14a\x11\xC6V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x06\"W` `\x03\x196\x01\x12a\x06\"W`\x045a#=a8\xEFV[\x80\x15a#\x8CW\x80\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2_\x80\xA2\0[\x7F\x0B\xF7q\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Wa#\xCCa8\xEFV[a#\xD4a;LV[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Wa$ca8\xEFV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` a\t\x14a5MV[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xF3[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a&cW` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x90_[\x81\x81\x10a'\tWa\x01\xDA\x85a\t\xC9\x81\x87\x03\x82a4 V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a&\xF2V[`@`\x03\x196\x01\x12a\x06\"Wa'3a3\xA9V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"Wa'S\x906\x90`\x04\x01a4\x95V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a)\xD2W[Pa&cWa'\xA3a8\xEFV[`@Q\x91\x7F\x04\x18G\x9C\xA6\xAC\x8D\xBF\x83bj^<\xDCY\x15#?\x1Aup\xC95\x11]\xB0\xAC\xF53\xB6I\xBF_\x80\xA1s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x92\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a)\x9EW[Pa(HW\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a)sWP\x82;\x15a)HW\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a)\x17Wa)\x15\x91aF\xD8V[\0[PP4a) W\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a)\xCAW[\x81a)\xBA` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x90\x85a(\x17V[=\x91Pa)\xADV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a'\x96V[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Wa*,a8\xEFV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F\xC4IV\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a.\x8EW[P\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x90\x80\x82\x03a.`W\x82`@Q\x7F\xBD\xEBD-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a..W[Pa+\x1Ea+\x15a=`V[\x82\x80\x82\x14a8\xB8V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R\x7F\t9\xA4j\xA2{T\x8D\xF6\x0Fqn]\xCE\xB5\x08\xEF2u40M\xC6b\xFE\\\xF3|\xE9\xEB7\xD6T`\xFF\x16\x15a-\xE2Wa+\xC4\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[\x15a-\xB7WP`@Q\x7F@\xF3MB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a-\x85W[P\x80a,\x8AW[a,\x16a9[V[`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[_\x19\x81\x01\x90\x81\x11a-qW` \x90`$`@Q\x80\x94\x81\x93\x7F\x9A\xD9\x1DL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x17\xD8W_\x91a-?W[P\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16\x15a-\x14W\x80a,\x0EV[\x7F\xDB\xC1\xAA\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90P` \x81=` \x11a-iW[\x81a-Z` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x81a,\xD9V[=\x91Pa-MV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90P` \x81=` \x11a-\xAFW[\x81a-\xA0` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x82a,\x07V[=\x91Pa-\x93V[\x7Fy\x93\n6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7Fy\x93\n6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06`\x04R`$_\xFD[\x90P` \x81=` \x11a.XW[\x81a.I` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x82a+\tV[=\x91Pa.<V[\x7F3GM\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90P` \x81=` \x11a.\xB8W[\x81a.\xA9` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x82a*\xA0V[=\x91Pa.\x9CV[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x06\"W` `\x03\x196\x01\x12a\x06\"W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W6`#\x82\x01\x12\x15a\x06\"W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W`$\x82\x01\x91`$6\x91\x83`\x06\x1B\x01\x01\x11a\x06\"W\x90a/oa8\xEFV[_\x91[\x80\x83\x10a/{W\0[a/\x86\x83\x82\x84a5%V[5\x92` a/\x95\x82\x84\x86a5%V[\x015\x93\x84\x15\x15\x94\x85\x81\x03a\x06\"W\x81\x15a0\x8AWa/\xB3\x81\x83a;\x0BV[a0ZW` `\x01\x94\x95\x96\x7F\xA7\x0FD3a^\x08:\xDF\x1B\x97\xE0\xC3-\x9Ar\x1C9\xC2mm\xD6\xC2\x8F\xE9av\xAB\x90\x9F\x0E\xCA\x92\x84_R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0\x83R`@_ \x90_\x14a0+W\x86`\xFF\x19\x82T\x16\x17\x90U[`@Q\x90\x81R\xA2\x01\x91\x90a/rV[a\x01\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x82T\x16\x17\x90Ua0\x1CV[\x85\x82\x7F\x12}3^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7Fz\xF6#A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W`@`\x03\x196\x01\x12a\x06\"W`$5\x80\x15\x15\x81\x03a\x06\"Wa\t\x14` \x91`\x045a;\x0BV[4a\x06\"W`@`\x03\x196\x01\x12a\x06\"W`\x045`$5a0\xFCa8\xEFV[a1\x04a9[V[a1\x0Ca9\xAFV[a1\x16\x81\x83a5\x0BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F@\xF3MB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a3wW[P\x80\x83\x11a3HWP\x83\x15\x80\x15a2\x93W[\x15a2gW\x83[\x82\x81\x10a1\xDCW\x7Fc\xB1\x87g=E\x0E\xD7\x14T\x95Gz\xA4w\x14E\x0E\xB1}\x84\xB2\x91\xD8\t\xDA\xF2\x9D\xA8\tgX`@\x86\x86\x82Q\x91\x82R` \x82\x01R\xA1\0[`@Q\x90\x7F\x9A\xD9\x1DL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x80`\x04\x83\x01R` \x82`$\x81\x86Z\xFA\x91\x82\x15a\x17\xD8W_\x92a22W[Pa2,`\x01\x92a:vV[\x01a1\xA3V[\x91P` \x82=\x82\x11a2_W[\x81a2L` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"W\x90Q\x90a2,a2 V[=\x91Pa2?V[\x83\x7F\xC6\\qp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[P_\x19\x84\x01\x84\x81\x11a-qW`@Q\x90\x7F\x9A\xD9\x1DL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a3\x16W[P_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16a1\x9CV[\x90P` \x81=` \x11a3@W[\x81a31` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x85a2\xE2V[=\x91Pa3$V[\x82\x7Fqk3\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90P` \x81=` \x11a3\xA1W[\x81a3\x92` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x85a1\x8AV[=\x91Pa3\x85V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x06\"WV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1F\x92W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a4k\x82a4CV[\x91a4y`@Q\x93\x84a4 V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x06\"W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x06\"W\x81` a4\xB0\x935\x91\x01a4_V[\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a4\xD0WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4\xC3V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a-qWV[\x91\x90\x82\x03\x91\x82\x11a-qWV[\x91\x90\x81\x10\x15a\x1F~W`\x06\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x06\"WQ\x80\x15\x15\x81\x03a\x06\"W\x90V[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x17\xD8W_\x91a6\x8AW[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a\x17\xD8W_\x91a6aWP\x90V[a4\xB0\x91P` =` \x11a6\x83W[a6{\x81\x83a4 V[\x81\x01\x90a55V[P=a6qV[\x90P` \x81=` \x11a6\xD2W[\x81a6\xA5` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x06\"W` a6\x08V[=\x91Pa6\x98V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x06\"WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1F\x92W`\x05\x1B` \x01\x90V[\x90a7P\x82a7.V[a7]`@Q\x91\x82a4 V[\x82\x81R`\x1F\x19a7m\x82\x94a7.V[\x01\x90` 6\x91\x017V[\x91\x90\x81\x10\x15a\x1F~W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[\x80Q\x82\x10\x15a\x1F~W` \x91`\x05\x1B\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a8\x8CWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x15a8\xC1WPPV[\x7F\xD5\xDFm\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a9/WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x15a9\x87WV[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a:WW[P\x15a:,WPV[\x7F\xF4\xEF\x17-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a:p\x91P` =` \x11a6\x83Wa6{\x81\x83a4 V[_a:#V[\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16a:\xE0W_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[_\x90\x81R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0` R`@\x90 `\xFF\x91\x15a;DWT\x16\x90V[T`\x08\x1C\x16\x90V[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a;wWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@Q\x90a;\xAC\x82a3\xCCV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[\x90\x82\x10\x15a\x1F~Wa4\xB0\x91`\x05\x1B\x81\x01\x90a;\xB7V[_\x19\x81\x14a-qW`\x01\x01\x90V[\x91\x90\x81\x10\x15a\x1F~W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W` \x01\x91\x816\x03\x83\x13a\x06\"WV[\x80_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16a=5W` \x81\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x92_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03\x82R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA1V[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ T\x92_[\x81\x81\x10a=\xDFWPPPV[\x90\x91\x93`\x01\x90\x81\x86\x16\x15_\x14a>KW\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta>?\x91aF>V[\x94[\x81\x1C\x92\x91\x01a=\xD3V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta>\x9C\x91\x90aF>V[\x94a>AV[\x91\x90\x81\x10\x15a\x1F~W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[_\x92\x91\x83\x90\x81[\x81\x81\x10a>\xF6WPPPPV[a?\x0Ea?\x04\x82\x84\x87a>\xA2V[` \x81\x01\x90a<OV[\x84a?xW\x80`\x80\x11a\x06\"W` a?,``\x84\x015\x83\x85aG\x9FV[\x15\x92\x83\x15\x93a?OW[PPPa?EW`\x01\x01a>\xE9V[P`\x01\x94PPPPV[\x91\x95P\x91P_\x94\x80`\xA0\x11a\x06\"Wa?n\x92P`\x80\x82\x015\x91aG\x9FV[\x15\x15_\x80\x80a?6V[\x84\x80\xFD[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a4\xB0\x94\x92\x81R\x81` \x82\x01R\x01\x91a?|V[\x90a?\xBE\x81\x80a6\xDAV[_[\x81\x81\x10aAhWPPPa?\xD7` \x82\x01\x82a6\xDAV[_[\x81\x81\x10a@\xF9WPPPa?\xF0`@\x82\x01\x82a6\xDAV[_[\x81\x81\x10a@\x8AWPPP\x80``a@\n\x92\x01\x90a6\xDAV[\x90\x91_[\x82\x81\x10a@\x1BWPPPPV[a@&\x81\x84\x86a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14a@AW[\x01a@\x0EV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca@qa?\x04\x84\x88\x8Aa>\xA2V[\x90a@\x82`@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2a@;V[a@\x95\x81\x83\x85a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14a@\xB0W[\x01a?\xF2V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa@\xE0a?\x04\x84\x87\x89a>\xA2V[\x90a@\xF1`@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2a@\xAAV[aA\x04\x81\x83\x85a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14aA\x1FW[\x01a?\xD9V[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3aAOa?\x04\x84\x87\x89a>\xA2V[\x90aA``@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2aA\x19V[aAs\x81\x83\x85a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14aA\x8EW[\x01a?\xC0V[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6aA\xBEa?\x04\x84\x87\x89a>\xA2V[\x90aA\xCF`@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2aA\x88V[\x80\x15\x80\x15aB)W[\x80\x15aB!W[\x80\x15aB\x11W[aB\x0BWd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15aA\xEEV[P\x81\x15aA\xE7V[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15aA\xE0V[\x92\x93\x92\x90\x91_\x90\x80\x83\x03aD9WPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08aBbWPP\x90P_\x90_\x90V[d\x01\0\0\x03\xD0\x19\x82\x80\td\x01\0\0\x03\xD0\x19_\x81\x80\x80\x80\x86\x88\t`\x04\t\x95\x80\t`\x03\t\x08\x90d\x01\0\0\x03\xD0\x19\x91\x82\x84\x80\x08\x83\x03\x83\x81\x11a-qW\x83\x90\x81\x83\x80\t\x08\x93\x84\x84\x03\x92\x84\x84\x11a-qW\x84\x80\x91\x80\t`\x08\t\x84\x03\x84\x81\x11a-qW\x84`\x01\x93\x81\x80\x96\x81\x95\x08\x90\t\x08\x94\t`\x02\t\x91\x90[\x82\x15\x15\x83\x81aD(W[P\x80aD W[\x15aC\xC2W\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15aC\x88W\x80\x84\x04\x96\x80\x98aCtWd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a-qWd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15aC`W\x90aCV\x91a5\x18V[\x95\x92\x93\x96\x95aB\xFEV[`$\x8AcNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x8AcNH{q`\xE0\x1B\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80aC\xAEWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80cNH{q`\xE0\x1B`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01aB\xE5V[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_aB\xDEV[aDM\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96aG\xD8V[\x91aB\xD4V[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x06\"W\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W\x81`\x05\x1B6\x03\x83\x13a\x06\"WV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a\x06\"W\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x826\x03\x01\x90[\x85\x84\x10aE%WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`\x1F\x19\x82\x82\x03\x01\x86R\x875\x83\x81\x12\x15a\x06\"W\x84\x01\x805`\x02\x81\x10\x15a\x06\"W\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x06\"W\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x806\x03\x82\x13a\x06\"WaE\xB8` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91a?|V[\x99\x01\x97\x96\x01\x94\x01\x92\x91\x90aE\x14V[a4\xB0\x91aF0aF%aF\naE\xEFaE\xE1\x86\x80aDSV[`\x80\x87R`\x80\x87\x01\x91aD\xD8V[aE\xFC` \x87\x01\x87aDSV[\x90\x86\x83\x03` \x88\x01RaD\xD8V[aF\x17`@\x86\x01\x86aDSV[\x90\x85\x83\x03`@\x87\x01RaD\xD8V[\x92``\x81\x01\x90aDSV[\x91``\x81\x85\x03\x91\x01RaD\xD8V[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81RaFa``\x82a4 V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x17\xD8W_Q\x90V[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15aF\xB0WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80aG\x8CW[\x15aG\x0CWPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15aGSWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15aGdW`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80aF\xECWP\x81;\x15\x15aF\xECV[` \x83\x01\x92\x91\x83\x83\x11a-qW\x83\x83\x11a\x06\"W\x83\x11a\x06\"W\x81\x015\x91\x03` \x81\x10aG\xCAWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x94\x92\x91\x85\x15\x80aI\xDFW[aI\xD3W\x80\x15\x80aI\xCBW[aI\xC1W`@Q`\x80\x91aH\x03\x83\x83a4 V[\x826\x837\x86\x15aI\xADW\x86\x94\x85\x80\x92\x81\x80`\x01\x80\t\x80\x87R\x97\x81\x89`\x01\t\x9C` \x88\x01\x9D\x8ER\x82`@\x89\x01\x9D\x8E\x8C\x81RQ`\x01\t\x91``\x8A\x01\x92\x83R`@Q\x9E\x8FaHM\x90a3\xE8V[Q\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90aI\xA1W[\x15aICW\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85aH\x9B\x8F\x97\x88a4 V[6\x877Q\x8CQaH\xAB\x90\x83a5\x18V[\x90\x08\x84RQ\x85QaH\xBC\x90\x83a5\x18V[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88QaH\xF9\x90\x83a5\x18V[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\taI\x10\x90\x83a5\x18V[\x90\x08\x9CQ\x93Q\x90Q\x90\taI$\x8C\x83a5\x18V[\x90\x08\x90\t\x92Q\x90Q\x90\taI8\x90\x83a5\x18V[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15aH\x7FV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x92P`\x01\x91\x90PV[P\x82\x15aG\xEFV[\x94P\x90\x92P`\x01\x91\x90PV[P\x81\x15aG\xE3V[5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x06\"WV[\x91_[\x81\x81\x10aJ\x18WPPPPV[aJ&a?\x04\x82\x84\x86a>\xA2V[\x81\x92\x91\x01``\x83\x82\x03\x12a\x06\"WaJ=\x83aI\xE7V[` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x82aJ]\x91\x86\x01a4\x95V[`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"WaJ\x97aJ\x9F\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\x01\x98\x01a4\x95V[\x92\x16\x88aM\x96V[\x01aJ\x0BV[\x90\x93\x92``\x92aJ\xB4\x82a7FV[\x93aJ\xBE\x83a7FV[\x90`@Q\x95aJ\xCC\x87a4\x04V[\x86R` \x86\x01\x91\x82R`@\x86\x01\x90_\x82RaJ\xEFaJ\xE9\x86a7FV[\x95a7FV[`@Q\x95aJ\xFC\x87a4\x04V[\x86R` \x86\x01R`@\x85\x01\x98_\x8AR\x86Q\x91_[\x83\x81\x10aK\xDCWPPPP\x90aK/\x91\x85Q\x96Q\x91Q\x90Q\x91\x87aR\x0EV[\x95Q\x91\x82\x15aK\xD5W\x90\x81` aKK\x94\x93Q\x91\x01Q\x91aR\x0EV[aKT\x83a7FV[\x94_\x91\x80Q\x93_[\x85\x81\x10aKlWPPPPPPPV[_[\x87\x81\x10aK~WP`\x01\x01aK\\V[\x80`@aK\x97aK\x90`\x01\x94\x89a7\xB7V[Q\x86a7\xB7V[Q\x01QaK\xA4\x84\x87a7\xB7V[Q\x14aK\xB1W[\x01aKnV[aK\xBB\x81\x87a7\xB7V[QaK\xCFaK\xC8\x89a<\x01V[\x98\x8Da7\xB7V[RaK\xABV[PPPPPV[\x81aK\xE7\x82\x8Ba7\xB7V[Q\x01Q\x80Q\x90_[\x82\x81\x10aL\x01WPPP`\x01\x01aK\x10V[\x8B\x8D\x8B\x89\x87\x8F\x8AaL\x12\x88\x8Aa7\xB7V[Q\x01Q\x80Q\x90_[\x82\x81\x10aL\xDCWPPP`\x80aL0\x88\x8Aa7\xB7V[Q\x01Q\x94\x85Q\x96_\x95[\x88\x87\x10aLRWPPPPPPPPP`\x01\x01aK\xEFV[aL\\\x87\x89a7\xB7V[Q\x90aLh\x82\x84aS\xB4V[\x96\x87\x14aL\x90WPP\x91aL\x81\x93\x91`\x01\x96\x95\x93aT\x82V[\x01\x8F\x8F\x8F\x8F\x92\x8A\x92\x8E\x94aL:V[\x92P\x94PPaL\xA3\x92P\x83\x91P\x8CaS\xF1V[\x15aL\xB1WP`\x01\x90aL\x81V[\x7FE\xBDI`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x95aL\xEE\x87\x83\x99\x94\x9A\x93\x97\x96\x97a7\xB7V[Q\x90aL\xFA\x82\x84aS\xB4V[\x95\x86\x14aM%WPP\x91aM\x13\x93\x91`\x01\x96\x95\x93aT\x82V[\x01\x8F\x91\x8F\x93\x8F\x8F\x93\x91\x8D\x93\x8B\x93aL\x1AV[\x92P\x94PPaM8\x92P\x83\x91P\x8CaS\xF1V[\x15aMFWP`\x01\x90aM\x13V[\x7F!\x1B\x12\xB7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91aM\x88a4\xB0\x93`@\x84R`@\x84\x01\x90a4\xE6V[\x91` \x81\x84\x03\x91\x01Ra4\xE6V[\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92`@Q\x92\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`@`$\x84\x01R_\x83\x80aM\xF2`D\x82\x01\x86a4\xE6V[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x17\xD8W_\x93aN\x8DW[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03aNRWP\x90\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91aNM`@Q\x92\x83\x92\x83aMqV[\x03\x90\xA2V[\x90PaN\x89`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01aMqV[\x03\x90\xFD[\x90\x92P=\x80_\x83>aN\x9F\x81\x83a4 V[\x81\x01\x90` \x81\x83\x03\x12a\x06\"W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W\x01\x81`\x1F\x82\x01\x12\x15a\x06\"W\x80Q\x90aN\xD6\x82a4CV[\x92aN\xE4`@Q\x94\x85a4 V[\x82\x84R` \x83\x83\x01\x01\x11a\x06\"W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_aN\x06V[\x81Q\x91\x90`A\x83\x03aO9WaO2\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aS%V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aO\xEEW\x80aOUWPPV[`\x01\x81\x03aO\x85W\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03aO\xB9WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14aO\xC3WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x06\"W\x815aP\x19\x81a7.V[\x92aP'`@Q\x94\x85a4 V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x06\"W` \x01\x90[\x82\x82\x10aPOWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01aPBV[\x90\x94\x93\x92\x91_\x95\x82R` \x82\x01RaPv\x83a7.V[\x90aP\x84`@Q\x92\x83a4 V[\x83\x82R`\x1F\x19aP\x93\x85a7.V[\x01_[\x81\x81\x10aQ\xBBWPP``\x01\x90\x81R_\x92[\x80\x84\x10aP\xB5WPPPPV[\x90\x91\x92\x94aP\xC4\x86\x84Qa7\xB7V[Q\x90aP\xD4a?\x04\x88\x85\x88a>\xA2V[\x81\x92\x91\x01`\xA0\x83\x82\x03\x12a\x06\"WaP\xEB\x83aI\xE7V[\x93` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x82aQ\x0C\x91\x86\x01a4\x95V[\x94`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x83aQ-\x91\x87\x01a4\x95V[\x91``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x84aQN\x91\x88\x01aP\x02V[\x91`\x80\x87\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W`\x01\x98aQ\x8Ea\x16\x7F\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94aQ\xB1\x9B\x01aP\x02V[\x95\x86`\x80\x86\x01R\x85``\x86\x01R`@\x85\x01R` \x84\x01R\x16\x90RQ\x90Q\x90a5\x0BV[\x95\x01\x92\x91\x90aP\xA8V[`@\x95\x93\x94\x95Q\x90`\xA0\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x1F\x92W` \x92`@R_\x81R``\x83\x82\x01R```@\x82\x01R``\x80\x82\x01R```\x80\x82\x01R\x82\x82\x88\x01\x01R\x01\x94\x93\x92\x94aP\x96V[\x93\x92aR\x19\x85a7FV[\x92_[\x82\x81\x10aR\xF3WPaR-\x86a7FV[\x95_[\x81\x81\x10aR?WPPPPPPV[aRH\x86aT\xF5V[\x82\x81\x14aR\xCBW_\x19aR[\x82\x89a7\xB7V[R\x80aRg\x83\x8Ba7\xB7V[R_[\x85\x81\x10aR{WPP`\x01\x01aR0V[\x81aR\x86\x82\x89a7\xB7V[Q\x14aR\x95W[`\x01\x01aRjV[aR\x9F\x81\x86a7\xB7V[Q\x90aR\xAB\x82\x8Aa7\xB7V[Q\x91\x82\x15a-qWaR\xC3_\x19`\x01\x94\x01\x91\x8Ba7\xB7V[R\x90PaR\x8DV[\x7F\xA8\x12\x90\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x80aS\0`\x01\x92\x84a7\xB7V[QaS\x1EaS\x17aS\x11\x83\x8Aa7\xB7V[Qa<\x01V[\x91\x88a7\xB7V[R\x01aR\x1CV[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aS\xA9W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x17\xD8W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aS\x9FW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x91\x90\x82Q\x90_\x93[\x82\x85\x10aS\xC8WPPPV[\x90\x91\x92\x93\x82aS\xD7\x82\x84a7\xB7V[QQ\x14aS\xEAW`\x01\x01\x93\x92\x91\x90aS\xBCV[\x92PPP\x90V[\x90\x92\x91_\x93_[\x81\x81\x10aT\x05WPPPPV[aT\x13a\x15\xD2\x82\x84\x87a7wV[_[\x81\x81\x10aThWPPPaT-a\x15\xF2\x82\x84\x87a7wV[_[\x81\x81\x10aTAWPPP`\x01\x01aS\xF8V[\x85aTM\x82\x84\x86a<\x0FV[5\x14aT[W`\x01\x01aT/V[P`\x01\x97PPPPPPPV[\x85aTt\x82\x84\x86a;\xEAV[5\x14aT[W`\x01\x01aT\x15V[\x91\x90\x93\x92\x93\x91`@aT\xA2\x85\x82aT\x99\x89\x86a7\xB7V[Q\x01Q\x93a7\xB7V[Q\x01Q\x80\x82\x03aT\xE3W[PPP` \x81aT\xD4\x92Q\x94aT\xC9`@\x83\x01\x96\x87Q\x90a7\xB7V[R\x01Q\x83Q\x90a7\xB7V[RaT\xDF\x81Qa<\x01V[\x90RV[\x90\x94P\x92P\x90P` aT\xD4_aT\xADV[\x90\x81Q_\x92[\x81\x84\x10aU\x06WPPV[\x90\x91\x92aU\x13\x81\x83a7\xB7V[Q\x15aU$W`\x01\x01\x92\x91\x90aT\xFBV[\x91PP\x90V\xFE\xA1dsolcC\0\x08%\0\n\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60a06040526004361015610011575f80fd5b5f5f3560e01c806303a7fde5146130dd578063171eac18146130b25780631a8473bc14612f10578063364f375b14612ec05780633f4ba83a14612a145780634f1ef2861461271f57806351945b061461268b57806352d1902d146125ec5780635c975abb146125ab57806364c12af21461257157806367353122146125575780636bf0a04b14612507578063715018a61461244b5780638456cb59146123b45780638b33e200146123215780638da5cb5b146122cf5780639239ac0c146110fa578063a06056f7146110ba578063aaf10f4214611067578063ad3cb1cc14611006578063bdeb442d14610fe3578063c1b0bed714610f95578063c44956d114610f58578063c4d66de814610a50578063e35a5d2f146109f3578063ed83cdc71461094b578063f2fde38b1461091e578063f8adf4c3146108c9578063fa83dd8f14610261578063fb0ff68014610224578063fe18ab91146101e15763ffa1ad741461017a575f80fd5b346101de57806003193601126101de57506101da60405161019c604082613420565b600b81527f322e302e302d72632e313100000000000000000000000000000000000000000060208201526040519182916020835260208301906134e6565b0390f35b80fd5b50346101de57806003193601126101de576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b50346101de57806003193601126101de5760207f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb0054604051908152f35b50346101de5760206003193601126101de5760043567ffffffffffffffff81116108c557366023820112156108c55780600401359067ffffffffffffffff82116108c1573660248360051b830101116108c1576102bc6138ef565b6102c461395b565b6102cc6139af565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900546108995773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690604051927fc44956d1000000000000000000000000000000000000000000000000000000008452602084600481865afa93841561088e57859461085a575b5083156108325760ff81116108025760ff8116916001831b808610156107d25750847f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900556801000000000000000082116107be577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154827f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90155808310610757575b507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90186526024017fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f865b83811061074357505050506001810180821161072f5761047290613746565b80511561071b5760208101917fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068352855b8181106106c8575050519067ffffffffffffffff82116106b4576801000000000000000082116106b4577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254827f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025580831061064d575b507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90285527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431855b838110610639576040517fbdeb442d0000000000000000000000000000000000000000000000000000000081528790876020826004818b5afa91821561062e5783926105f5575b5060207f9cfc151cd0c8c49916e15e9af15c5d553ff85cc13d96fef94340655fd6794e9f916105e26105d8613d60565b85819682146138b8565b6105eb84613ca0565b604051908152a280f35b9091506020813d602011610626575b8161061160209383613420565b8101031261062257519060206105a8565b5f80fd5b3d9150610604565b6040513d85823e3d90fd5b600190602084519401938184015501610561565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90286528290037fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301865b8281106106a657505061051a565b808860019284015501610698565b602485634e487b7160e01b81526041600452fd5b6106e76106d582856137b7565b516106e083866137b7565b519061463e565b60018201808311610707579061070060019392866137b7565b52016104a3565b602489634e487b7160e01b81526011600452fd5b602485634e487b7160e01b81526032600452fd5b602485634e487b7160e01b81526011600452fd5b600190602084359401938184015501610453565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90187528290037fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f8301875b8281106107b0575050610409565b8089600192840155016107a2565b602486634e487b7160e01b81526041600452fd5b86604491877f6be696a3000000000000000000000000000000000000000000000000000000008352600452602452fd5b7f6dfcc6500000000000000000000000000000000000000000000000000000000085526008600452602452604484fd5b6004857f4cd41ad8000000000000000000000000000000000000000000000000000000008152fd5b9093506020813d602011610886575b8161087660209383613420565b810103126106225751925f610368565b3d9150610869565b6040513d87823e3d90fd5b6004837f66fda36f000000000000000000000000000000000000000000000000000000008152fd5b8280fd5b5080fd5b50346101de5760206003193601126101de5760206109146004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b6040519015158152f35b50346101de5760206003193601126101de5761094861093b6133a9565b6109436138ef565b6137cb565b80f35b50346101de57806003193601126101de576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9018054808352908352909160208301917fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f915b8181106109dd576101da856109c981870382613420565b6040519182916020835260208301906134b3565b82548452602090930192600192830192016109b2565b50346101de57806003193601126101de5760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101de5760206003193601126101de57610a6a6133a9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00549060ff8260401c16159167ffffffffffffffff811680159081610f50575b6001149081610f46575b159081610f3d575b50610f155790818360017fffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000610b2d9516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0055610ec0575b50610b1d614681565b610b25614681565b610943614681565b610b35614681565b610b3d614681565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025468010000000000000000811015610eac5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015610e98577fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9028452602084200155817f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b0682527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260408220600160ff198254161790557f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a1610cb4614681565b610cbc614681565b610cc4614681565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e28380a2610d5461354d565b610e7057610d60613b4c565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1610ddc5780f35b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a180f35b6004827f25ae6eaa000000000000000000000000000000000000000000000000000000008152fd5b602483634e487b7160e01b81526032600452fd5b602483634e487b7160e01b81526041600452fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00555f610b14565b6004847ff92ee8a9000000000000000000000000000000000000000000000000000000008152fd5b9050155f610abc565b303b159150610ab4565b849150610aaa565b50346101de57806003193601126101de5760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b50346101de5760206003193601126101de5760ff604060209260043581527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0084522054166040519015158152f35b50346101de57806003193601126101de576020610ffe613d60565b604051908152f35b50346101de57806003193601126101de57506101da604051611029604082613420565b600581527f352e302e3000000000000000000000000000000000000000000000000000000060208201526040519182916020835260208301906134e6565b50346101de57806003193601126101de57602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b50346101de57806003193601126101de57602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b50346106225760206003193601126106225767ffffffffffffffff6004351161062257608060031960043536030112610622577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c6122a75760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d61117f613b4c565b61118d6004803501806136da565b8091501561227f577f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb005460646004350135148015612250575b1561221e576111d481613746565b906111dd613b9f565b506040516111ea816133cc565b5f81525f6020820152915f915f915f915f905b808210611a59575050806020611245925160051b9101209461123c61123661122f602460043501600435600401613c4f565b369161345f565b87614f09565b90939193614f43565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2016911690808203611a2b575061128890506004803501806136da565b906040519180608084017f0ebc6c35680d6c45d27f0686aaa3a4d0331220c5bd2e52ce1195a5bce1033e0860208601526064600435013560408601526060808601525260a083019060a08160051b8501019280925f915b83831061181157505050505060208261130382945f9403601f198101835282613420565b604051918183925191829101835e8101838152039060025afa156117d8575f51611337604460043501600435600401613c4f565b80600411610622577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168082036117e357505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15610622575f9261143192604051958694859384937fab750e75000000000000000000000000000000000000000000000000000000008552606060048601526064850191613f7c565b907f896fd82f0d437a4c50a3d9986bcc1c03bb4f9e9bf83b0243dbb5907dd98b6cb66024840152604483015203915afa80156117d8576117c3575b50816114d5575b5050806114c6575b507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce8280a2807f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d80f35b6114cf90613ca0565b5f61147b565b6114e36004803501806136da565b9091156117175785906114f58461372e565b926115036040519485613420565b848452601f196115128661372e565b01885b8181106116e857505087885b8381106115c457505090611536929184614aa5565b855b83811061154b57505050505b5f80611473565b61155f61155882846137b7565b51846137b7565b51606081018051519160200190895b8381106115815750505050600101611538565b806115be61159260019385516137b7565b5185519073ffffffffffffffffffffffffffffffffffffffff8151166040602083015192015192614d96565b0161156e565b6115d86115d2828686613777565b806136da565b8b5b818110611691575050506115fc6115f2828686613777565b60208101906136da565b8b5b81811061161057505050600101611521565b8861161c828486613c0f565b61163661162c6040830183613bb7565b60408101906136da565b9182611649575b505050506001016115fe565b9161167f916001959c9361166b8b611665611685989d91613c01565b9c6137b7565b518a6040820152602082013591359061505f565b9061350b565b979050885f808061163d565b8861169d828486613bea565b6116ad61162c6060830183613bb7565b91826116c0575b505050506001016115da565b9161167f916001959c9361166b8b6116656116dc989d91613c01565b979050885f80806116b4565b6020906040516116f7816133e8565b8b81528b838201528b604082015260608082015282828901015201611515565b909150845b81811061172b57505050611544565b6117396115d2828486613777565b875b81811061179a575050506117536115f2828486613777565b875b8181106117675750505060010161171c565b80611794602061177a6001948688613c0f565b61178a61162c6040830183613bb7565b9290910135614a08565b01611755565b806117bd60206117ad6001948688613bea565b61178a61162c6060830183613bb7565b0161173b565b6117d09195505f90613420565b5f935f61146c565b6040513d5f823e3d90fd5b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b90919293947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608782030183528535907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61833603018212156106225760a0810191839061187f81830180614453565b809560a086525260c0840160c08660051b86010195825f5b8281106119a45750505050506118b4602083830101838301614453565b94908482036020860152858252602082019060208760051b8401019681935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b83811061193e57505050505050910160408181013590840152606080820135908401526080908101359201919091525094602090810194936001019201906112df565b91939599909294969750601f1984820301875289358681121561062257602061199060019360606119828885960180358452858101358685015260408101906144a6565b9181604082015201906145c7565b9b0197019101918a979695939194926118fb565b919397600191939596506020611a19826080611a0b6119eb8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f889903018d528a6144a6565b8035845285810135868501526040810135604085015260608101906144a6565b9181606082015201906145c7565b99019501910191889594939192611897565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9095611a7387611a6d6004803501806136da565b90613777565b6080525f5f5f91611a886080516080516136da565b9050611a9381613746565b90611a9d81613746565b905f5b8181106120be575050611aba6020608051016080516136da565b919050611ac682613746565b90611ad083613746565b925f905b808210611c7c575050611b2d92611b497f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc329593611b3b611b5794604051978897608080510135895260a060208a015260a08901906134b3565b9087820360408901526134b3565b9085820360608701526134b3565b9083820360808501526134b3565b0390a180611c74575b5080611c54575b505060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0608051360301126106225760405190611ba4826133cc565b6040608051013582526020820160606080510135815260405192611bc7846133cc565b5f84525f6020850152611bdd81518351906141d7565b15611c1c5791611bfc9160019493602083519301519051915192614239565b6020830152815296608080510135611c1482866137b7565b5201906111fd565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b611c5e919661350b565b94611c6b575b5f80611b67565b60019350611c64565b97505f611b60565b909550611c9a86611c946020608051016080516136da565b90613c0f565b95611ca95f6020890135613b0b565b61208a5760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054611cfd81613c01565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90055908835915f5b828110611fa65750507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b14611e0b575b509660408101611d9161162c8284613bb7565b80611dd4575b505081611db3611dac60209360019695613bb7565b8235613fb3565b8035611dbf84896137b7565b520135611dcc82886137b7565b520190611ad4565b909a611ddf90613c01565b9a8c611d97576020929c50611dac611dfe6001969593611db393613ee2565b9d93505081939450611d97565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015468010000000000000000811015611f925760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90155811015611f7e577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f018290557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431015490611eee90611ee790839061463e565b918061463e565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025468010000000000000000811015611f925760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015611f7e577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2001555f611d7e565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52604160045260245ffd5b9092600190818516612033577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301546120289161463e565b935b811c9101611d25565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154612084919061463e565b9361202a565b6020877f597abd20000000000000000000000000000000000000000000000000000000005f5201356004525f60245260445ffd5b6120d6816120d06080516080516136da565b90613bea565b6120e560016020830135613b0b565b6121e9576040810135612123815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b156121be5750803561213481613a76565b6060820161214561162c8285613bb7565b80612185575b5050916020916121676121616001969584613bb7565b82613fb3565b61217184896137b7565b52013561217e82866137b7565b5201611aa0565b909961219090613c01565b998b61214b5760019594929b506121616121b06020959361216793613ee2565b9c939596505081935061214b565b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6020907f597abd20000000000000000000000000000000000000000000000000000000005f520135600452600160245260445ffd5b7fc93ee163000000000000000000000000000000000000000000000000000000005f526064600435013560045260245ffd5b507fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b0660646004350135146111c6565b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610622575f60031936011261062257602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346106225760206003193601126106225760043561233d6138ef565b801561238c57807f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e25f80a2005b7f0bf7718c000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610622575f600319360112610622576123cc6138ef565b6123d4613b4c565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b34610622575f600319360112610622576124636138ef565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610622575f60031936011261062257602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610622575f60031936011261062257602061091461354d565b34610622575f6003193601126106225760206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152f35b34610622575f60031936011261062257602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b34610622575f6003193601126106225773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001630036126635760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b34610622575f600319360112610622576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f20905f5b818110612709576101da856109c981870382613420565b82548452602090930192600192830192016126f2565b6040600319360112610622576127336133a9565b60243567ffffffffffffffff811161062257612753903690600401613495565b9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168030149081156129d2575b50612663576127a36138ef565b604051917f0418479ca6ac8dbf83626a5e3cdc5915233f1a7570c935115db0acf533b649bf5f80a173ffffffffffffffffffffffffffffffffffffffff8216927f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f918161299e575b5061284857837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8592036129735750823b1561294857807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a280511561291757612915916146d8565b005b50503461292057005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d6020116129ca575b816129ba60209383613420565b8101031261062257519085612817565b3d91506129ad565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416141583612796565b34610622575f60031936011261062257612a2c6138ef565b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517fc44956d1000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91612e8e575b507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005490808203612e6057826040517fbdeb442d000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91612e2e575b50612b1e612b15613d60565b828082146138b8565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9036020527f0939a46aa27b548df60f716e5dceb508ef327534304dc662fe5cf37ce9eb37d65460ff1615612de257612bc4815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b15612db757506040517f40f34d42000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91612d85575b5080612c8a575b612c1661395b565b60ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054167fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b5f198101908111612d71576020906024604051809481937f9ad91d4c00000000000000000000000000000000000000000000000000000000835260048301525afa9081156117d8575f91612d3f575b50805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f20541615612d145780612c0e565b7fdbc1aa1e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b90506020813d602011612d69575b81612d5a60209383613420565b81010312610622575181612cd9565b3d9150612d4d565b634e487b7160e01b5f52601160045260245ffd5b90506020813d602011612daf575b81612da060209383613420565b81010312610622575182612c07565b3d9150612d93565b7f79930a36000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f79930a36000000000000000000000000000000000000000000000000000000005f527fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b0660045260245ffd5b90506020813d602011612e58575b81612e4960209383613420565b81010312610622575182612b09565b3d9150612e3c565b7f33474d08000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b90506020813d602011612eb8575b81612ea960209383613420565b81010312610622575182612aa0565b3d9150612e9c565b34610622575f60031936011261062257602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346106225760206003193601126106225760043567ffffffffffffffff8111610622573660238201121561062257806004013567ffffffffffffffff8111610622576024820191602436918360061b0101116106225790612f6f6138ef565b5f915b808310612f7b57005b612f86838284613525565b35926020612f95828486613525565b0135938415159485810361062257811561308a57612fb38183613b0b565b61305a57602060019495967fa70f4433615e083adf1b97e0c32d9a721c39c26d6dd6c28fe96176ab909f0eca92845f527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900835260405f20905f1461302b578660ff198254161790555b604051908152a2019190612f72565b6101007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff82541617905561301c565b85827f127d335e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7f7af62341000000000000000000000000000000000000000000000000000000005f5260045ffd5b3461062257604060031936011261062257602435801515810361062257610914602091600435613b0b565b34610622576040600319360112610622576004356024356130fc6138ef565b61310461395b565b61310c6139af565b613116818361350b565b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517f40f34d42000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91613377575b50808311613348575083158015613293575b1561326757835b8281106131dc577f63b187673d450ed7145495477aa47714450eb17d84b291d809daf29da80967586040868682519182526020820152a1005b604051907f9ad91d4c000000000000000000000000000000000000000000000000000000008252806004830152602082602481865afa9182156117d8575f92613232575b5061322c600192613a76565b016131a3565b91506020823d821161325f575b8161324c60209383613420565b810103126106225790519061322c613220565b3d915061323f565b837fc65c7170000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b505f198401848111612d7157604051907f9ad91d4c0000000000000000000000000000000000000000000000000000000082526004820152602081602481855afa9081156117d8575f91613316575b505f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f20541661319c565b90506020813d602011613340575b8161333160209383613420565b810103126106225751856132e2565b3d9150613324565b827f716b33b3000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b90506020813d6020116133a1575b8161339260209383613420565b8101031261062257518561318a565b3d9150613385565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361062257565b6040810190811067ffffffffffffffff821117611f9257604052565b6080810190811067ffffffffffffffff821117611f9257604052565b6060810190811067ffffffffffffffff821117611f9257604052565b90601f601f19910116810190811067ffffffffffffffff821117611f9257604052565b67ffffffffffffffff8111611f9257601f01601f191660200190565b92919261346b82613443565b916134796040519384613420565b829481845281830111610622578281602093845f960137010152565b9080601f83011215610622578160206134b09335910161345f565b90565b90602080835192838152019201905f5b8181106134d05750505090565b82518452602093840193909201916001016134c3565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b91908201809211612d7157565b91908203918211612d7157565b9190811015611f7e5760061b0190565b90816020910312610622575180151581036106225790565b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa9081156117d8575f9161368a575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa9081156117d8575f91613661575090565b6134b0915060203d602011613683575b61367b8183613420565b810190613535565b503d613671565b90506020813d6020116136d2575b816136a560209383613420565b81010312610622575173ffffffffffffffffffffffffffffffffffffffff81168103610622576020613608565b3d9150613698565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610622570180359067ffffffffffffffff821161062257602001918160051b3603831361062257565b67ffffffffffffffff8111611f925760051b60200190565b906137508261372e565b61375d6040519182613420565b828152601f1961376d829461372e565b0190602036910137565b9190811015611f7e5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6181360301821215610622570190565b8051821015611f7e5760209160051b010190565b73ffffffffffffffffffffffffffffffffffffffff16801561388c5773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b156138c1575050565b7fd5df6dd6000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416330361392f57565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054161561398757565b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517f5c975abb000000000000000000000000000000000000000000000000000000008152602081600481855afa9081156117d8575f91613a57575b5015613a2c5750565b7ff4ef172d000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b613a70915060203d6020116136835761367b8183613420565b5f613a23565b805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f205416613ae0575f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260405f20600160ff19825416179055565b7f39a940c5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5f9081527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629006020526040902060ff9115613b4457541690565b5460081c1690565b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416613b7757565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b60405190613bac826133cc565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215610622570190565b90821015611f7e576134b09160051b810190613bb7565b5f198114612d715760010190565b9190811015611f7e5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301821215610622570190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610622570180359067ffffffffffffffff82116106225760200191813603831361062257565b805f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f205416613d35576020817f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d2925f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903825260405f20600160ff19825416179055604051908152a1565b7fdb788c2b000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b60ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900547f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2054925f5b818110613ddf57505050565b909193600190818616155f14613e4b577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431830154613e3f9161463e565b945b811c929101613dd3565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154613e9c919061463e565b94613e41565b9190811015611f7e5760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215610622570190565b5f92918390815b818110613ef65750505050565b613f0e613f04828487613ea2565b6020810190613c4f565b84613f785780608011610622576020613f2c6060840135838561479f565b1592831593613f4f575b505050613f4557600101613ee9565b5060019450505050565b91955091505f948060a01161062257613f6e925060808201359161479f565b15155f8080613f36565b8480fd5b601f8260209493601f1993818652868601375f8582860101520116010190565b6040906134b0949281528160208201520191613f7c565b90613fbe81806136da565b5f5b81811061416857505050613fd760208201826136da565b5f5b8181106140f957505050613ff060408201826136da565b5f5b81811061408a5750505080606061400a9201906136da565b90915f5b82811061401b5750505050565b614026818486613ea2565b35906002821015610622576001809214614041575b0161400e565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c614071613f0484888a613ea2565b906140826040519283928784613f9c565b0390a261403b565b614095818385613ea2565b359060028210156106225760018092146140b0575b01613ff2565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596140e0613f04848789613ea2565b906140f16040519283928784613f9c565b0390a26140aa565b614104818385613ea2565b3590600282101561062257600180921461411f575b01613fd9565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f361414f613f04848789613ea2565b906141606040519283928784613f9c565b0390a2614119565b614173818385613ea2565b3590600282101561062257600180921461418e575b01613fc0565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66141be613f04848789613ea2565b906141cf6040519283928784613f9c565b0390a2614188565b80158015614229575b8015614221575b8015614211575b61420b576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d0198210156141ee565b5081156141e7565b506401000003d0198110156141e0565b92939290915f908083036144395750506401000003d0195f94830861426257505090505f905f90565b6401000003d0198280096401000003d0195f8180808086880960040995800960030908906401000003d01991828480088303838111612d7157839081838009089384840392848411612d715784809180096008098403848111612d715784600193818096819508900908940960020991905b8215158381614428575b5080614420575b156143c2579084939293906001936401000003d0199187965b801561438857808404968098614374576401000003d0199088096401000003d019036401000003d0198111612d71576401000003d019908a960897938197828102928184041490151715614360579061435691613518565b95929396956142fe565b60248a634e487b7160e01b81526011600452fd5b60248a634e487b7160e01b81526012600452fd5b50945094505093806143ae5750906401000003d019809281808780098092099509900990565b80634e487b7160e01b602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b5060016142e5565b6401000003d019915014155f6142de565b61444d936401000003d019939692966147d8565b916142d4565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561062257016020813591019167ffffffffffffffff8211610622578160051b3603831361062257565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8182360301811215610622570190565b906020838281520160208260051b85010193835f917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc182360301905b858410614525575050505050505090565b90919293949596601f198282030186528735838112156106225784018035600281101561062257825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18236030181121561062257016020813591019067ffffffffffffffff8111610622578036038213610622576145b86020928392604086818660019901520191613f7c565b99019796019401929190614514565b6134b09161463061462561460a6145ef6145e18680614453565b6080875260808701916144d8565b6145fc6020870187614453565b9086830360208801526144d8565b6146176040860186614453565b9085830360408701526144d8565b926060810190614453565b9160608185039101526144d8565b5f9060209260405190848201928352604082015260408152614661606082613420565b604051918291518091835e8101838152039060025afa156117d8575f5190565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c16156146b057565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b905f8091602081519101845af4808061478c575b1561470c5750506040513d81523d5f602083013e60203d82010160405290565b156147535773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d15614764576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d1515806146ec5750813b15156146ec565b602083019291838311612d7157838311610622578311610622578101359103602081106147ca575090565b5f199060200360031b1b1690565b9492918515806149df575b6149d3578015806149cb575b6149c1576040516080916148038383613420565b8236833786156149ad5786948580928180600180098087529781896001099c602088019d8e5282604089019d8e8c8152516001099160608a019283526040519e8f61484d906133e8565b5190098d525190099460208b019586525190099860408901998a525190096060870190815286518851148015906149a1575b1561494357849283808093816040519c8561489b8f9788613420565b368737518c516148ab9083613518565b900884525185516148bc9083613518565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a5251800988516148f99083613518565b900881808751855190096002096149109083613518565b90089c519351905190096149248c83613518565b900890099251905190096149389083613518565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b5081518151141561487f565b634e487b7160e01b5f52601260045260245ffd5b5092506001919050565b5082156147ef565b94509092506001919050565b5081156147e3565b359073ffffffffffffffffffffffffffffffffffffffff8216820361062257565b915f5b818110614a185750505050565b614a26613f04828486613ea2565b8192910160608382031261062257614a3d836149e7565b602084013567ffffffffffffffff81116106225782614a5d918601613495565b604085013567ffffffffffffffff811161062257614a97614a9f9473ffffffffffffffffffffffffffffffffffffffff9260019801613495565b921688614d96565b01614a0b565b909392606092614ab482613746565b93614abe83613746565b9060405195614acc87613404565b86526020860191825260408601905f8252614aef614ae986613746565b95613746565b60405195614afc87613404565b8652602086015260408501985f8a528651915f5b838110614bdc575050505090614b2f918551965191519051918761520e565b9551918215614bd55790816020614b4b9493519101519161520e565b614b5483613746565b945f918051935f5b858110614b6c5750505050505050565b5f5b878110614b7e5750600101614b5c565b806040614b97614b90600194896137b7565b51866137b7565b510151614ba484876137b7565b5114614bb1575b01614b6e565b614bbb81876137b7565b51614bcf614bc889613c01565b988d6137b7565b52614bab565b5050505050565b81614be7828b6137b7565b5101518051905f5b828110614c0157505050600101614b10565b8b8d8b89878f8a614c12888a6137b7565b5101518051905f5b828110614cdc575050506080614c30888a6137b7565b510151948551965f955b888710614c5257505050505050505050600101614bef565b614c5c87896137b7565b5190614c6882846153b4565b968714614c9057505091614c8193916001969593615482565b018f8f8f8f928a928e94614c3a565b9250945050614ca392508391508c6153f1565b15614cb15750600190614c81565b7f45bd4960000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b95614cee878399949a939796976137b7565b5190614cfa82846153b4565b958614614d2557505091614d1393916001969593615482565b018f918f938f8f93918d938b93614c1a565b9250945050614d3892508391508c6153f1565b15614d465750600190614d13565b7f211b12b7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091614d886134b0936040845260408401906134e6565b9160208184039101526134e6565b919073ffffffffffffffffffffffffffffffffffffffff1692604051927f33a892030000000000000000000000000000000000000000000000000000000084526004840152604060248401525f8380614df260448201866134e6565b038183885af19283156117d8575f93614e8d575b50825160208401208151602083012003614e525750907fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf191614e4d60405192839283614d71565b0390a2565b9050614e896040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401614d71565b0390fd5b9092503d805f833e614e9f8183613420565b8101906020818303126106225780519067ffffffffffffffff8211610622570181601f8201121561062257805190614ed682613443565b92614ee46040519485613420565b8284526020838301011161062257815f9260208093018386015e83010152915f614e06565b8151919060418303614f3957614f329250602082015190606060408401519301515f1a90615325565b9192909190565b50505f9160029190565b6004811015614fee5780614f55575050565b60018103614f85577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b60028103614fb957507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b600314614fc35750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b9080601f830112156106225781356150198161372e565b926150276040519485613420565b81845260208085019260051b82010192831161062257602001905b82821061504f5750505090565b8135815260209182019101615042565b90949392915f95825260208201526150768361372e565b906150846040519283613420565b838252601f196150938561372e565b015f5b8181106151bb5750506060019081525f925b8084106150b55750505050565b909192946150c48684516137b7565b51906150d4613f04888588613ea2565b8192910160a083820312610622576150eb836149e7565b93602084013567ffffffffffffffff8111610622578261510c918601613495565b94604085013567ffffffffffffffff8111610622578361512d918701613495565b91606086013567ffffffffffffffff8111610622578461514e918801615002565b9160808701359067ffffffffffffffff82116106225760019861518e61167f9773ffffffffffffffffffffffffffffffffffffffff946151b19b01615002565b95866080860152856060860152604085015260208401521690525190519061350b565b95019291906150a8565b604095939495519060a082019180831067ffffffffffffffff841117611f92576020926040525f815260608382015260606040820152606080820152606060808201528282880101520194939294615096565b939261521985613746565b925f5b8281106152f3575061522d86613746565b955f5b81811061523f57505050505050565b615248866154f5565b8281146152cb575f1961525b82896137b7565b5280615267838b6137b7565b525f5b85811061527b575050600101615230565b8161528682896137b7565b5114615295575b60010161526a565b61529f81866137b7565b51906152ab828a6137b7565b51918215612d71576152c35f1960019401918b6137b7565b52905061528d565b7fa8129095000000000000000000000000000000000000000000000000000000005f5260045ffd5b80615300600192846137b7565b5161531e615317615311838a6137b7565b51613c01565b91886137b7565b520161521c565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116153a9579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156117d8575f5173ffffffffffffffffffffffffffffffffffffffff81161561539f57905f905f90565b505f906001905f90565b5050505f9160039190565b91908251905f935b8285106153c857505050565b90919293826153d782846137b7565b5151146153ea57600101939291906153bc565b9250505090565b9092915f935f5b8181106154055750505050565b6154136115d2828487613777565b5f5b8181106154685750505061542d6115f2828487613777565b5f5b818110615441575050506001016153f8565b8561544d828486613c0f565b351461545b5760010161542f565b5060019750505050505050565b85615474828486613bea565b351461545b57600101615415565b91909392939160406154a2858261549989866137b7565b510151936137b7565b5101518082036154e3575b5050506020816154d49251946154c960408301968751906137b7565b5201518351906137b7565b526154df8151613c01565b9052565b9094509250905060206154d45f6154ad565b9081515f925b818410615506575050565b90919261551381836137b7565b5115615524576001019291906154fb565b9150509056fea164736f6c6343000825000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x03\xA7\xFD\xE5\x14a0\xDDW\x80c\x17\x1E\xAC\x18\x14a0\xB2W\x80c\x1A\x84s\xBC\x14a/\x10W\x80c6O7[\x14a.\xC0W\x80c?K\xA8:\x14a*\x14W\x80cO\x1E\xF2\x86\x14a'\x1FW\x80cQ\x94[\x06\x14a&\x8BW\x80cR\xD1\x90-\x14a%\xECW\x80c\\\x97Z\xBB\x14a%\xABW\x80cd\xC1*\xF2\x14a%qW\x80cg51\"\x14a%WW\x80ck\xF0\xA0K\x14a%\x07W\x80cqP\x18\xA6\x14a$KW\x80c\x84V\xCBY\x14a#\xB4W\x80c\x8B3\xE2\0\x14a#!W\x80c\x8D\xA5\xCB[\x14a\"\xCFW\x80c\x929\xAC\x0C\x14a\x10\xFAW\x80c\xA0`V\xF7\x14a\x10\xBAW\x80c\xAA\xF1\x0FB\x14a\x10gW\x80c\xAD<\xB1\xCC\x14a\x10\x06W\x80c\xBD\xEBD-\x14a\x0F\xE3W\x80c\xC1\xB0\xBE\xD7\x14a\x0F\x95W\x80c\xC4IV\xD1\x14a\x0FXW\x80c\xC4\xD6m\xE8\x14a\nPW\x80c\xE3Z]/\x14a\t\xF3W\x80c\xED\x83\xCD\xC7\x14a\tKW\x80c\xF2\xFD\xE3\x8B\x14a\t\x1EW\x80c\xF8\xAD\xF4\xC3\x14a\x08\xC9W\x80c\xFA\x83\xDD\x8F\x14a\x02aW\x80c\xFB\x0F\xF6\x80\x14a\x02$W\x80c\xFE\x18\xAB\x91\x14a\x01\xE1Wc\xFF\xA1\xADt\x14a\x01zW_\x80\xFD[4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEWPa\x01\xDA`@Qa\x01\x9C`@\x82a4 V[`\x0B\x81R\x7F2.0.0-rc.11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xE6V[\x03\x90\xF3[\x80\xFD[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` \x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T`@Q\x90\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\xC5W6`#\x82\x01\x12\x15a\x08\xC5W\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x08\xC1W6`$\x83`\x05\x1B\x83\x01\x01\x11a\x08\xC1Wa\x02\xBCa8\xEFV[a\x02\xC4a9[V[a\x02\xCCa9\xAFV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ta\x08\x99Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90`@Q\x92\x7F\xC4IV\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R` \x84`\x04\x81\x86Z\xFA\x93\x84\x15a\x08\x8EW\x85\x94a\x08ZW[P\x83\x15a\x082W`\xFF\x81\x11a\x08\x02W`\xFF\x81\x16\x91`\x01\x83\x1B\x80\x86\x10\x15a\x07\xD2WP\x84\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Uh\x01\0\0\0\0\0\0\0\0\x82\x11a\x07\xBEW\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x82\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x80\x83\x10a\x07WW[P\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01\x86R`$\x01\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x86[\x83\x81\x10a\x07CWPPPP`\x01\x81\x01\x80\x82\x11a\x07/Wa\x04r\x90a7FV[\x80Q\x15a\x07\x1BW` \x81\x01\x91\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x83R\x85[\x81\x81\x10a\x06\xC8WPPQ\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\xB4Wh\x01\0\0\0\0\0\0\0\0\x82\x11a\x06\xB4W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x82\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x80\x83\x10a\x06MW[P\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02\x85R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x85[\x83\x81\x10a\x069W`@Q\x7F\xBD\xEBD-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x87\x90\x87` \x82`\x04\x81\x8BZ\xFA\x91\x82\x15a\x06.W\x83\x92a\x05\xF5W[P` \x7F\x9C\xFC\x15\x1C\xD0\xC8\xC4\x99\x16\xE1^\x9A\xF1\\]U?\xF8\\\xC1=\x96\xFE\xF9C@e_\xD6yN\x9F\x91a\x05\xE2a\x05\xD8a=`V[\x85\x81\x96\x82\x14a8\xB8V[a\x05\xEB\x84a<\xA0V[`@Q\x90\x81R\xA2\x80\xF3[\x90\x91P` \x81=` \x11a\x06&W[\x81a\x06\x11` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x90` a\x05\xA8V[_\x80\xFD[=\x91Pa\x06\x04V[`@Q=\x85\x82>=\x90\xFD[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x05aV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02\x86R\x82\x90\x03\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01\x86[\x82\x81\x10a\x06\xA6WPPa\x05\x1AV[\x80\x88`\x01\x92\x84\x01U\x01a\x06\x98V[`$\x85cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[a\x06\xE7a\x06\xD5\x82\x85a7\xB7V[Qa\x06\xE0\x83\x86a7\xB7V[Q\x90aF>V[`\x01\x82\x01\x80\x83\x11a\x07\x07W\x90a\x07\0`\x01\x93\x92\x86a7\xB7V[R\x01a\x04\xA3V[`$\x89cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x85cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[`$\x85cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`\x01\x90` \x845\x94\x01\x93\x81\x84\x01U\x01a\x04SV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01\x87R\x82\x90\x03\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x87[\x82\x81\x10a\x07\xB0WPPa\x04\tV[\x80\x89`\x01\x92\x84\x01U\x01a\x07\xA2V[`$\x86cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x86`D\x91\x87\x7Fk\xE6\x96\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04R`$R\xFD[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`\x08`\x04R`$R`D\x84\xFD[`\x04\x85\x7FL\xD4\x1A\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90\x93P` \x81=` \x11a\x08\x86W[\x81a\x08v` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x92_a\x03hV[=\x91Pa\x08iV[`@Q=\x87\x82>=\x90\xFD[`\x04\x83\x7Ff\xFD\xA3o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x82\x80\xFD[P\x80\xFD[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEW` a\t\x14`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEWa\tHa\t;a3\xA9V[a\tCa8\xEFV[a7\xCBV[\x80\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01\x80T\x80\x83R\x90\x83R\x90\x91` \x83\x01\x91\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x91[\x81\x81\x10a\t\xDDWa\x01\xDA\x85a\t\xC9\x81\x87\x03\x82a4 V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xB3V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\t\xB2V[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEWa\nja3\xA9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x90`\xFF\x82`@\x1C\x16\x15\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x0FPW[`\x01\x14\x90\x81a\x0FFW[\x15\x90\x81a\x0F=W[Pa\x0F\x15W\x90\x81\x83`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x0B-\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x0E\xC0W[Pa\x0B\x1DaF\x81V[a\x0B%aF\x81V[a\tCaF\x81V[a\x0B5aF\x81V[a\x0B=aF\x81V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0E\xACW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x0E\x98W\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02\x84R` \x84 \x01U\x81\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x82R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`@\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x0C\xB4aF\x81V[a\x0C\xBCaF\x81V[a\x0C\xC4aF\x81V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2\x83\x80\xA2a\rTa5MV[a\x0EpWa\r`a;LV[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1a\r\xDCW\x80\xF3[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\x80\xF3[`\x04\x82\x7F%\xAEn\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[`$\x83cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[`$\x83cNH{q`\xE0\x1B\x81R`A`\x04R\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U_a\x0B\x14V[`\x04\x84\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x90P\x15_a\n\xBCV[0;\x15\x91Pa\n\xB4V[\x84\x91Pa\n\xAAV[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[P4a\x01\xDEW` `\x03\x196\x01\x12a\x01\xDEW`\xFF`@` \x92`\x045\x81R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` a\x0F\xFEa=`V[`@Q\x90\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEWPa\x01\xDA`@Qa\x10)`@\x82a4 V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a4\xE6V[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[P4a\x01\xDEW\x80`\x03\x196\x01\x12a\x01\xDEW` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[P4a\x06\"W` `\x03\x196\x01\x12a\x06\"Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x06\"W`\x80`\x03\x19`\x0456\x03\x01\x12a\x06\"W\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\"\xA7W`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x11\x7Fa;LV[a\x11\x8D`\x04\x805\x01\x80a6\xDAV[\x80\x91P\x15a\"\x7FW\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T`d`\x045\x015\x14\x80\x15a\"PW[\x15a\"\x1EWa\x11\xD4\x81a7FV[\x90a\x11\xDDa;\x9FV[P`@Qa\x11\xEA\x81a3\xCCV[_\x81R_` \x82\x01R\x91_\x91_\x91_\x91_\x90[\x80\x82\x10a\x1AYWPP\x80` a\x12E\x92Q`\x05\x1B\x91\x01 \x94a\x12<a\x126a\x12/`$`\x045\x01`\x045`\x04\x01a<OV[6\x91a4_V[\x87aO\tV[\x90\x93\x91\x93aOCV[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a\x1A+WPa\x12\x88\x90P`\x04\x805\x01\x80a6\xDAV[\x90`@Q\x91\x80`\x80\x84\x01\x7F\x0E\xBCl5h\rlE\xD2\x7F\x06\x86\xAA\xA3\xA4\xD03\x12 \xC5\xBD.R\xCE\x11\x95\xA5\xBC\xE1\x03>\x08` \x86\x01R`d`\x045\x015`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01\x90`\xA0\x81`\x05\x1B\x85\x01\x01\x92\x80\x92_\x91[\x83\x83\x10a\x18\x11WPPPPP` \x82a\x13\x03\x82\x94_\x94\x03`\x1F\x19\x81\x01\x83R\x82a4 V[`@Q\x91\x81\x83\x92Q\x91\x82\x91\x01\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x17\xD8W_Qa\x137`D`\x045\x01`\x045`\x04\x01a<OV[\x80`\x04\x11a\x06\"W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x17\xE3WPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x06\"W_\x92a\x141\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a?|V[\x90\x7F\x89o\xD8/\rCzLP\xA3\xD9\x98k\xCC\x1C\x03\xBBO\x9E\x9B\xF8;\x02C\xDB\xB5\x90}\xD9\x8Bl\xB6`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a\x17\xD8Wa\x17\xC3W[P\x81a\x14\xD5W[PP\x80a\x14\xC6W[P\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCE\x82\x80\xA2\x80\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]\x80\xF3[a\x14\xCF\x90a<\xA0V[_a\x14{V[a\x14\xE3`\x04\x805\x01\x80a6\xDAV[\x90\x91\x15a\x17\x17W\x85\x90a\x14\xF5\x84a7.V[\x92a\x15\x03`@Q\x94\x85a4 V[\x84\x84R`\x1F\x19a\x15\x12\x86a7.V[\x01\x88[\x81\x81\x10a\x16\xE8WPP\x87\x88[\x83\x81\x10a\x15\xC4WPP\x90a\x156\x92\x91\x84aJ\xA5V[\x85[\x83\x81\x10a\x15KWPPPP[_\x80a\x14sV[a\x15_a\x15X\x82\x84a7\xB7V[Q\x84a7\xB7V[Q``\x81\x01\x80QQ\x91` \x01\x90\x89[\x83\x81\x10a\x15\x81WPPPP`\x01\x01a\x158V[\x80a\x15\xBEa\x15\x92`\x01\x93\x85Qa7\xB7V[Q\x85Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16`@` \x83\x01Q\x92\x01Q\x92aM\x96V[\x01a\x15nV[a\x15\xD8a\x15\xD2\x82\x86\x86a7wV[\x80a6\xDAV[\x8B[\x81\x81\x10a\x16\x91WPPPa\x15\xFCa\x15\xF2\x82\x86\x86a7wV[` \x81\x01\x90a6\xDAV[\x8B[\x81\x81\x10a\x16\x10WPPP`\x01\x01a\x15!V[\x88a\x16\x1C\x82\x84\x86a<\x0FV[a\x166a\x16,`@\x83\x01\x83a;\xB7V[`@\x81\x01\x90a6\xDAV[\x91\x82a\x16IW[PPPP`\x01\x01a\x15\xFEV[\x91a\x16\x7F\x91`\x01\x95\x9C\x93a\x16k\x8Ba\x16ea\x16\x85\x98\x9D\x91a<\x01V[\x9Ca7\xB7V[Q\x8A`@\x82\x01R` \x82\x015\x915\x90aP_V[\x90a5\x0BV[\x97\x90P\x88_\x80\x80a\x16=V[\x88a\x16\x9D\x82\x84\x86a;\xEAV[a\x16\xADa\x16,``\x83\x01\x83a;\xB7V[\x91\x82a\x16\xC0W[PPPP`\x01\x01a\x15\xDAV[\x91a\x16\x7F\x91`\x01\x95\x9C\x93a\x16k\x8Ba\x16ea\x16\xDC\x98\x9D\x91a<\x01V[\x97\x90P\x88_\x80\x80a\x16\xB4V[` \x90`@Qa\x16\xF7\x81a3\xE8V[\x8B\x81R\x8B\x83\x82\x01R\x8B`@\x82\x01R``\x80\x82\x01R\x82\x82\x89\x01\x01R\x01a\x15\x15V[\x90\x91P\x84[\x81\x81\x10a\x17+WPPPa\x15DV[a\x179a\x15\xD2\x82\x84\x86a7wV[\x87[\x81\x81\x10a\x17\x9AWPPPa\x17Sa\x15\xF2\x82\x84\x86a7wV[\x87[\x81\x81\x10a\x17gWPPP`\x01\x01a\x17\x1CV[\x80a\x17\x94` a\x17z`\x01\x94\x86\x88a<\x0FV[a\x17\x8Aa\x16,`@\x83\x01\x83a;\xB7V[\x92\x90\x91\x015aJ\x08V[\x01a\x17UV[\x80a\x17\xBD` a\x17\xAD`\x01\x94\x86\x88a;\xEAV[a\x17\x8Aa\x16,``\x83\x01\x83a;\xB7V[\x01a\x17;V[a\x17\xD0\x91\x95P_\x90a4 V[_\x93_a\x14lV[`@Q=_\x82>=\x90\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x87\x82\x03\x01\x83R\x855\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x82\x12\x15a\x06\"W`\xA0\x81\x01\x91\x83\x90a\x18\x7F\x81\x83\x01\x80aDSV[\x80\x95`\xA0\x86RR`\xC0\x84\x01`\xC0\x86`\x05\x1B\x86\x01\x01\x95\x82_[\x82\x81\x10a\x19\xA4WPPPPPa\x18\xB4` \x83\x83\x01\x01\x83\x83\x01aDSV[\x94\x90\x84\x82\x03` \x86\x01R\x85\x82R` \x82\x01\x90` \x87`\x05\x1B\x84\x01\x01\x96\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x19>WPPPPPP\x91\x01`@\x81\x81\x015\x90\x84\x01R``\x80\x82\x015\x90\x84\x01R`\x80\x90\x81\x015\x92\x01\x91\x90\x91RP\x94` \x90\x81\x01\x94\x93`\x01\x01\x92\x01\x90a\x12\xDFV[\x91\x93\x95\x99\x90\x92\x94\x96\x97P`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x06\"W` a\x19\x90`\x01\x93``a\x19\x82\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90aD\xA6V[\x91\x81`@\x82\x01R\x01\x90aE\xC7V[\x9B\x01\x97\x01\x91\x01\x91\x8A\x97\x96\x95\x93\x91\x94\x92a\x18\xFBV[\x91\x93\x97`\x01\x91\x93\x95\x96P` a\x1A\x19\x82`\x80a\x1A\x0Ba\x19\xEB\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F\x88\x99\x03\x01\x8DR\x8AaD\xA6V[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90aD\xA6V[\x91\x81``\x82\x01R\x01\x90aE\xC7V[\x99\x01\x95\x01\x91\x01\x91\x88\x95\x94\x93\x91\x92a\x18\x97V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x95a\x1As\x87a\x1Am`\x04\x805\x01\x80a6\xDAV[\x90a7wV[`\x80R___\x91a\x1A\x88`\x80Q`\x80Qa6\xDAV[\x90Pa\x1A\x93\x81a7FV[\x90a\x1A\x9D\x81a7FV[\x90_[\x81\x81\x10a \xBEWPPa\x1A\xBA` `\x80Q\x01`\x80Qa6\xDAV[\x91\x90Pa\x1A\xC6\x82a7FV[\x90a\x1A\xD0\x83a7FV[\x92_\x90[\x80\x82\x10a\x1C|WPPa\x1B-\x92a\x1BI\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a\x1B;a\x1BW\x94`@Q\x97\x88\x97`\x80\x80Q\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a4\xB3V[\x90\x87\x82\x03`@\x89\x01Ra4\xB3V[\x90\x85\x82\x03``\x87\x01Ra4\xB3V[\x90\x83\x82\x03`\x80\x85\x01Ra4\xB3V[\x03\x90\xA1\x80a\x1CtW[P\x80a\x1CTW[PP`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\x80Q6\x03\x01\x12a\x06\"W`@Q\x90a\x1B\xA4\x82a3\xCCV[`@`\x80Q\x015\x82R` \x82\x01```\x80Q\x015\x81R`@Q\x92a\x1B\xC7\x84a3\xCCV[_\x84R_` \x85\x01Ra\x1B\xDD\x81Q\x83Q\x90aA\xD7V[\x15a\x1C\x1CW\x91a\x1B\xFC\x91`\x01\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92aB9V[` \x83\x01R\x81R\x96`\x80\x80Q\x015a\x1C\x14\x82\x86a7\xB7V[R\x01\x90a\x11\xFDV[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[a\x1C^\x91\x96a5\x0BV[\x94a\x1CkW[_\x80a\x1BgV[`\x01\x93Pa\x1CdV[\x97P_a\x1B`V[\x90\x95Pa\x1C\x9A\x86a\x1C\x94` `\x80Q\x01`\x80Qa6\xDAV[\x90a<\x0FV[\x95a\x1C\xA9_` \x89\x015a;\x0BV[a \x8AW`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ta\x1C\xFD\x81a<\x01V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x885\x91_[\x82\x81\x10a\x1F\xA6WPP\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\x1E\x0BW[P\x96`@\x81\x01a\x1D\x91a\x16,\x82\x84a;\xB7V[\x80a\x1D\xD4W[PP\x81a\x1D\xB3a\x1D\xAC` \x93`\x01\x96\x95a;\xB7V[\x825a?\xB3V[\x805a\x1D\xBF\x84\x89a7\xB7V[R\x015a\x1D\xCC\x82\x88a7\xB7V[R\x01\x90a\x1A\xD4V[\x90\x9Aa\x1D\xDF\x90a<\x01V[\x9A\x8Ca\x1D\x97W` \x92\x9CPa\x1D\xACa\x1D\xFE`\x01\x96\x95\x93a\x1D\xB3\x93a>\xE2V[\x9D\x93PP\x81\x93\x94Pa\x1D\x97V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x1F\x92W`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x81\x10\x15a\x1F~W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x01\x82\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x01T\x90a\x1E\xEE\x90a\x1E\xE7\x90\x83\x90aF>V[\x91\x80aF>V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x1F\x92W`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x1F~W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x01U_a\x1D~V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90\x92`\x01\x90\x81\x85\x16a 3W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta (\x91aF>V[\x93[\x81\x1C\x91\x01a\x1D%V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta \x84\x91\x90aF>V[\x93a *V[` \x87\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R_`$R`D_\xFD[a \xD6\x81a \xD0`\x80Q`\x80Qa6\xDAV[\x90a;\xEAV[a \xE5`\x01` \x83\x015a;\x0BV[a!\xE9W`@\x81\x015a!#\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[\x15a!\xBEWP\x805a!4\x81a:vV[``\x82\x01a!Ea\x16,\x82\x85a;\xB7V[\x80a!\x85W[PP\x91` \x91a!ga!a`\x01\x96\x95\x84a;\xB7V[\x82a?\xB3V[a!q\x84\x89a7\xB7V[R\x015a!~\x82\x86a7\xB7V[R\x01a\x1A\xA0V[\x90\x99a!\x90\x90a<\x01V[\x99\x8Ba!KW`\x01\x95\x94\x92\x9BPa!aa!\xB0` \x95\x93a!g\x93a>\xE2V[\x9C\x93\x95\x96PP\x81\x93Pa!KV[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[` \x90\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R`\x01`$R`D_\xFD[\x7F\xC9>\xE1c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`d`\x045\x015`\x04R`$_\xFD[P\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06`d`\x045\x015\x14a\x11\xC6V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x06\"W` `\x03\x196\x01\x12a\x06\"W`\x045a#=a8\xEFV[\x80\x15a#\x8CW\x80\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2_\x80\xA2\0[\x7F\x0B\xF7q\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Wa#\xCCa8\xEFV[a#\xD4a;LV[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Wa$ca8\xEFV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` a\t\x14a5MV[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xF3[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a&cW` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x90_[\x81\x81\x10a'\tWa\x01\xDA\x85a\t\xC9\x81\x87\x03\x82a4 V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a&\xF2V[`@`\x03\x196\x01\x12a\x06\"Wa'3a3\xA9V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"Wa'S\x906\x90`\x04\x01a4\x95V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a)\xD2W[Pa&cWa'\xA3a8\xEFV[`@Q\x91\x7F\x04\x18G\x9C\xA6\xAC\x8D\xBF\x83bj^<\xDCY\x15#?\x1Aup\xC95\x11]\xB0\xAC\xF53\xB6I\xBF_\x80\xA1s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x92\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a)\x9EW[Pa(HW\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a)sWP\x82;\x15a)HW\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a)\x17Wa)\x15\x91aF\xD8V[\0[PP4a) W\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a)\xCAW[\x81a)\xBA` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x90\x85a(\x17V[=\x91Pa)\xADV[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a'\x96V[4a\x06\"W_`\x03\x196\x01\x12a\x06\"Wa*,a8\xEFV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F\xC4IV\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a.\x8EW[P\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x90\x80\x82\x03a.`W\x82`@Q\x7F\xBD\xEBD-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a..W[Pa+\x1Ea+\x15a=`V[\x82\x80\x82\x14a8\xB8V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R\x7F\t9\xA4j\xA2{T\x8D\xF6\x0Fqn]\xCE\xB5\x08\xEF2u40M\xC6b\xFE\\\xF3|\xE9\xEB7\xD6T`\xFF\x16\x15a-\xE2Wa+\xC4\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[\x15a-\xB7WP`@Q\x7F@\xF3MB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a-\x85W[P\x80a,\x8AW[a,\x16a9[V[`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[_\x19\x81\x01\x90\x81\x11a-qW` \x90`$`@Q\x80\x94\x81\x93\x7F\x9A\xD9\x1DL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x17\xD8W_\x91a-?W[P\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16\x15a-\x14W\x80a,\x0EV[\x7F\xDB\xC1\xAA\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90P` \x81=` \x11a-iW[\x81a-Z` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x81a,\xD9V[=\x91Pa-MV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90P` \x81=` \x11a-\xAFW[\x81a-\xA0` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x82a,\x07V[=\x91Pa-\x93V[\x7Fy\x93\n6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7Fy\x93\n6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06`\x04R`$_\xFD[\x90P` \x81=` \x11a.XW[\x81a.I` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x82a+\tV[=\x91Pa.<V[\x7F3GM\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90P` \x81=` \x11a.\xB8W[\x81a.\xA9` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x82a*\xA0V[=\x91Pa.\x9CV[4a\x06\"W_`\x03\x196\x01\x12a\x06\"W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x06\"W` `\x03\x196\x01\x12a\x06\"W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W6`#\x82\x01\x12\x15a\x06\"W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W`$\x82\x01\x91`$6\x91\x83`\x06\x1B\x01\x01\x11a\x06\"W\x90a/oa8\xEFV[_\x91[\x80\x83\x10a/{W\0[a/\x86\x83\x82\x84a5%V[5\x92` a/\x95\x82\x84\x86a5%V[\x015\x93\x84\x15\x15\x94\x85\x81\x03a\x06\"W\x81\x15a0\x8AWa/\xB3\x81\x83a;\x0BV[a0ZW` `\x01\x94\x95\x96\x7F\xA7\x0FD3a^\x08:\xDF\x1B\x97\xE0\xC3-\x9Ar\x1C9\xC2mm\xD6\xC2\x8F\xE9av\xAB\x90\x9F\x0E\xCA\x92\x84_R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0\x83R`@_ \x90_\x14a0+W\x86`\xFF\x19\x82T\x16\x17\x90U[`@Q\x90\x81R\xA2\x01\x91\x90a/rV[a\x01\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x82T\x16\x17\x90Ua0\x1CV[\x85\x82\x7F\x12}3^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7Fz\xF6#A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x06\"W`@`\x03\x196\x01\x12a\x06\"W`$5\x80\x15\x15\x81\x03a\x06\"Wa\t\x14` \x91`\x045a;\x0BV[4a\x06\"W`@`\x03\x196\x01\x12a\x06\"W`\x045`$5a0\xFCa8\xEFV[a1\x04a9[V[a1\x0Ca9\xAFV[a1\x16\x81\x83a5\x0BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F@\xF3MB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a3wW[P\x80\x83\x11a3HWP\x83\x15\x80\x15a2\x93W[\x15a2gW\x83[\x82\x81\x10a1\xDCW\x7Fc\xB1\x87g=E\x0E\xD7\x14T\x95Gz\xA4w\x14E\x0E\xB1}\x84\xB2\x91\xD8\t\xDA\xF2\x9D\xA8\tgX`@\x86\x86\x82Q\x91\x82R` \x82\x01R\xA1\0[`@Q\x90\x7F\x9A\xD9\x1DL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x80`\x04\x83\x01R` \x82`$\x81\x86Z\xFA\x91\x82\x15a\x17\xD8W_\x92a22W[Pa2,`\x01\x92a:vV[\x01a1\xA3V[\x91P` \x82=\x82\x11a2_W[\x81a2L` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"W\x90Q\x90a2,a2 V[=\x91Pa2?V[\x83\x7F\xC6\\qp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[P_\x19\x84\x01\x84\x81\x11a-qW`@Q\x90\x7F\x9A\xD9\x1DL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a3\x16W[P_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16a1\x9CV[\x90P` \x81=` \x11a3@W[\x81a31` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x85a2\xE2V[=\x91Pa3$V[\x82\x7Fqk3\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90P` \x81=` \x11a3\xA1W[\x81a3\x92` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQ\x85a1\x8AV[=\x91Pa3\x85V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x06\"WV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1F\x92W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1F\x92W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a4k\x82a4CV[\x91a4y`@Q\x93\x84a4 V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x06\"W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x06\"W\x81` a4\xB0\x935\x91\x01a4_V[\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a4\xD0WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4\xC3V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a-qWV[\x91\x90\x82\x03\x91\x82\x11a-qWV[\x91\x90\x81\x10\x15a\x1F~W`\x06\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x06\"WQ\x80\x15\x15\x81\x03a\x06\"W\x90V[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x17\xD8W_\x91a6\x8AW[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a\x17\xD8W_\x91a6aWP\x90V[a4\xB0\x91P` =` \x11a6\x83W[a6{\x81\x83a4 V[\x81\x01\x90a55V[P=a6qV[\x90P` \x81=` \x11a6\xD2W[\x81a6\xA5` \x93\x83a4 V[\x81\x01\x03\x12a\x06\"WQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x06\"W` a6\x08V[=\x91Pa6\x98V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x06\"WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1F\x92W`\x05\x1B` \x01\x90V[\x90a7P\x82a7.V[a7]`@Q\x91\x82a4 V[\x82\x81R`\x1F\x19a7m\x82\x94a7.V[\x01\x90` 6\x91\x017V[\x91\x90\x81\x10\x15a\x1F~W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[\x80Q\x82\x10\x15a\x1F~W` \x91`\x05\x1B\x01\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a8\x8CWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x15a8\xC1WPPV[\x7F\xD5\xDFm\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a9/WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x15a9\x87WV[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x17\xD8W_\x91a:WW[P\x15a:,WPV[\x7F\xF4\xEF\x17-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a:p\x91P` =` \x11a6\x83Wa6{\x81\x83a4 V[_a:#V[\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16a:\xE0W_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[_\x90\x81R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0` R`@\x90 `\xFF\x91\x15a;DWT\x16\x90V[T`\x08\x1C\x16\x90V[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a;wWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@Q\x90a;\xAC\x82a3\xCCV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[\x90\x82\x10\x15a\x1F~Wa4\xB0\x91`\x05\x1B\x81\x01\x90a;\xB7V[_\x19\x81\x14a-qW`\x01\x01\x90V[\x91\x90\x81\x10\x15a\x1F~W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W` \x01\x91\x816\x03\x83\x13a\x06\"WV[\x80_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16a=5W` \x81\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x92_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03\x82R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA1V[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ T\x92_[\x81\x81\x10a=\xDFWPPPV[\x90\x91\x93`\x01\x90\x81\x86\x16\x15_\x14a>KW\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta>?\x91aF>V[\x94[\x81\x1C\x92\x91\x01a=\xD3V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta>\x9C\x91\x90aF>V[\x94a>AV[\x91\x90\x81\x10\x15a\x1F~W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x06\"W\x01\x90V[_\x92\x91\x83\x90\x81[\x81\x81\x10a>\xF6WPPPPV[a?\x0Ea?\x04\x82\x84\x87a>\xA2V[` \x81\x01\x90a<OV[\x84a?xW\x80`\x80\x11a\x06\"W` a?,``\x84\x015\x83\x85aG\x9FV[\x15\x92\x83\x15\x93a?OW[PPPa?EW`\x01\x01a>\xE9V[P`\x01\x94PPPPV[\x91\x95P\x91P_\x94\x80`\xA0\x11a\x06\"Wa?n\x92P`\x80\x82\x015\x91aG\x9FV[\x15\x15_\x80\x80a?6V[\x84\x80\xFD[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a4\xB0\x94\x92\x81R\x81` \x82\x01R\x01\x91a?|V[\x90a?\xBE\x81\x80a6\xDAV[_[\x81\x81\x10aAhWPPPa?\xD7` \x82\x01\x82a6\xDAV[_[\x81\x81\x10a@\xF9WPPPa?\xF0`@\x82\x01\x82a6\xDAV[_[\x81\x81\x10a@\x8AWPPP\x80``a@\n\x92\x01\x90a6\xDAV[\x90\x91_[\x82\x81\x10a@\x1BWPPPPV[a@&\x81\x84\x86a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14a@AW[\x01a@\x0EV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca@qa?\x04\x84\x88\x8Aa>\xA2V[\x90a@\x82`@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2a@;V[a@\x95\x81\x83\x85a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14a@\xB0W[\x01a?\xF2V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa@\xE0a?\x04\x84\x87\x89a>\xA2V[\x90a@\xF1`@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2a@\xAAV[aA\x04\x81\x83\x85a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14aA\x1FW[\x01a?\xD9V[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3aAOa?\x04\x84\x87\x89a>\xA2V[\x90aA``@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2aA\x19V[aAs\x81\x83\x85a>\xA2V[5\x90`\x02\x82\x10\x15a\x06\"W`\x01\x80\x92\x14aA\x8EW[\x01a?\xC0V[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6aA\xBEa?\x04\x84\x87\x89a>\xA2V[\x90aA\xCF`@Q\x92\x83\x92\x87\x84a?\x9CV[\x03\x90\xA2aA\x88V[\x80\x15\x80\x15aB)W[\x80\x15aB!W[\x80\x15aB\x11W[aB\x0BWd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15aA\xEEV[P\x81\x15aA\xE7V[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15aA\xE0V[\x92\x93\x92\x90\x91_\x90\x80\x83\x03aD9WPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08aBbWPP\x90P_\x90_\x90V[d\x01\0\0\x03\xD0\x19\x82\x80\td\x01\0\0\x03\xD0\x19_\x81\x80\x80\x80\x86\x88\t`\x04\t\x95\x80\t`\x03\t\x08\x90d\x01\0\0\x03\xD0\x19\x91\x82\x84\x80\x08\x83\x03\x83\x81\x11a-qW\x83\x90\x81\x83\x80\t\x08\x93\x84\x84\x03\x92\x84\x84\x11a-qW\x84\x80\x91\x80\t`\x08\t\x84\x03\x84\x81\x11a-qW\x84`\x01\x93\x81\x80\x96\x81\x95\x08\x90\t\x08\x94\t`\x02\t\x91\x90[\x82\x15\x15\x83\x81aD(W[P\x80aD W[\x15aC\xC2W\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15aC\x88W\x80\x84\x04\x96\x80\x98aCtWd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a-qWd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15aC`W\x90aCV\x91a5\x18V[\x95\x92\x93\x96\x95aB\xFEV[`$\x8AcNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$\x8AcNH{q`\xE0\x1B\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80aC\xAEWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80cNH{q`\xE0\x1B`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01aB\xE5V[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_aB\xDEV[aDM\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96aG\xD8V[\x91aB\xD4V[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x06\"W\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W\x81`\x05\x1B6\x03\x83\x13a\x06\"WV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a\x06\"W\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x826\x03\x01\x90[\x85\x84\x10aE%WPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`\x1F\x19\x82\x82\x03\x01\x86R\x875\x83\x81\x12\x15a\x06\"W\x84\x01\x805`\x02\x81\x10\x15a\x06\"W\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x06\"W\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x806\x03\x82\x13a\x06\"WaE\xB8` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91a?|V[\x99\x01\x97\x96\x01\x94\x01\x92\x91\x90aE\x14V[a4\xB0\x91aF0aF%aF\naE\xEFaE\xE1\x86\x80aDSV[`\x80\x87R`\x80\x87\x01\x91aD\xD8V[aE\xFC` \x87\x01\x87aDSV[\x90\x86\x83\x03` \x88\x01RaD\xD8V[aF\x17`@\x86\x01\x86aDSV[\x90\x85\x83\x03`@\x87\x01RaD\xD8V[\x92``\x81\x01\x90aDSV[\x91``\x81\x85\x03\x91\x01RaD\xD8V[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81RaFa``\x82a4 V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x17\xD8W_Q\x90V[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15aF\xB0WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80aG\x8CW[\x15aG\x0CWPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15aGSWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15aGdW`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80aF\xECWP\x81;\x15\x15aF\xECV[` \x83\x01\x92\x91\x83\x83\x11a-qW\x83\x83\x11a\x06\"W\x83\x11a\x06\"W\x81\x015\x91\x03` \x81\x10aG\xCAWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x94\x92\x91\x85\x15\x80aI\xDFW[aI\xD3W\x80\x15\x80aI\xCBW[aI\xC1W`@Q`\x80\x91aH\x03\x83\x83a4 V[\x826\x837\x86\x15aI\xADW\x86\x94\x85\x80\x92\x81\x80`\x01\x80\t\x80\x87R\x97\x81\x89`\x01\t\x9C` \x88\x01\x9D\x8ER\x82`@\x89\x01\x9D\x8E\x8C\x81RQ`\x01\t\x91``\x8A\x01\x92\x83R`@Q\x9E\x8FaHM\x90a3\xE8V[Q\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90aI\xA1W[\x15aICW\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85aH\x9B\x8F\x97\x88a4 V[6\x877Q\x8CQaH\xAB\x90\x83a5\x18V[\x90\x08\x84RQ\x85QaH\xBC\x90\x83a5\x18V[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88QaH\xF9\x90\x83a5\x18V[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\taI\x10\x90\x83a5\x18V[\x90\x08\x9CQ\x93Q\x90Q\x90\taI$\x8C\x83a5\x18V[\x90\x08\x90\t\x92Q\x90Q\x90\taI8\x90\x83a5\x18V[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15aH\x7FV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x92P`\x01\x91\x90PV[P\x82\x15aG\xEFV[\x94P\x90\x92P`\x01\x91\x90PV[P\x81\x15aG\xE3V[5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x06\"WV[\x91_[\x81\x81\x10aJ\x18WPPPPV[aJ&a?\x04\x82\x84\x86a>\xA2V[\x81\x92\x91\x01``\x83\x82\x03\x12a\x06\"WaJ=\x83aI\xE7V[` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x82aJ]\x91\x86\x01a4\x95V[`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"WaJ\x97aJ\x9F\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\x01\x98\x01a4\x95V[\x92\x16\x88aM\x96V[\x01aJ\x0BV[\x90\x93\x92``\x92aJ\xB4\x82a7FV[\x93aJ\xBE\x83a7FV[\x90`@Q\x95aJ\xCC\x87a4\x04V[\x86R` \x86\x01\x91\x82R`@\x86\x01\x90_\x82RaJ\xEFaJ\xE9\x86a7FV[\x95a7FV[`@Q\x95aJ\xFC\x87a4\x04V[\x86R` \x86\x01R`@\x85\x01\x98_\x8AR\x86Q\x91_[\x83\x81\x10aK\xDCWPPPP\x90aK/\x91\x85Q\x96Q\x91Q\x90Q\x91\x87aR\x0EV[\x95Q\x91\x82\x15aK\xD5W\x90\x81` aKK\x94\x93Q\x91\x01Q\x91aR\x0EV[aKT\x83a7FV[\x94_\x91\x80Q\x93_[\x85\x81\x10aKlWPPPPPPPV[_[\x87\x81\x10aK~WP`\x01\x01aK\\V[\x80`@aK\x97aK\x90`\x01\x94\x89a7\xB7V[Q\x86a7\xB7V[Q\x01QaK\xA4\x84\x87a7\xB7V[Q\x14aK\xB1W[\x01aKnV[aK\xBB\x81\x87a7\xB7V[QaK\xCFaK\xC8\x89a<\x01V[\x98\x8Da7\xB7V[RaK\xABV[PPPPPV[\x81aK\xE7\x82\x8Ba7\xB7V[Q\x01Q\x80Q\x90_[\x82\x81\x10aL\x01WPPP`\x01\x01aK\x10V[\x8B\x8D\x8B\x89\x87\x8F\x8AaL\x12\x88\x8Aa7\xB7V[Q\x01Q\x80Q\x90_[\x82\x81\x10aL\xDCWPPP`\x80aL0\x88\x8Aa7\xB7V[Q\x01Q\x94\x85Q\x96_\x95[\x88\x87\x10aLRWPPPPPPPPP`\x01\x01aK\xEFV[aL\\\x87\x89a7\xB7V[Q\x90aLh\x82\x84aS\xB4V[\x96\x87\x14aL\x90WPP\x91aL\x81\x93\x91`\x01\x96\x95\x93aT\x82V[\x01\x8F\x8F\x8F\x8F\x92\x8A\x92\x8E\x94aL:V[\x92P\x94PPaL\xA3\x92P\x83\x91P\x8CaS\xF1V[\x15aL\xB1WP`\x01\x90aL\x81V[\x7FE\xBDI`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x95aL\xEE\x87\x83\x99\x94\x9A\x93\x97\x96\x97a7\xB7V[Q\x90aL\xFA\x82\x84aS\xB4V[\x95\x86\x14aM%WPP\x91aM\x13\x93\x91`\x01\x96\x95\x93aT\x82V[\x01\x8F\x91\x8F\x93\x8F\x8F\x93\x91\x8D\x93\x8B\x93aL\x1AV[\x92P\x94PPaM8\x92P\x83\x91P\x8CaS\xF1V[\x15aMFWP`\x01\x90aM\x13V[\x7F!\x1B\x12\xB7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91aM\x88a4\xB0\x93`@\x84R`@\x84\x01\x90a4\xE6V[\x91` \x81\x84\x03\x91\x01Ra4\xE6V[\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92`@Q\x92\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`@`$\x84\x01R_\x83\x80aM\xF2`D\x82\x01\x86a4\xE6V[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x17\xD8W_\x93aN\x8DW[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03aNRWP\x90\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91aNM`@Q\x92\x83\x92\x83aMqV[\x03\x90\xA2V[\x90PaN\x89`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01aMqV[\x03\x90\xFD[\x90\x92P=\x80_\x83>aN\x9F\x81\x83a4 V[\x81\x01\x90` \x81\x83\x03\x12a\x06\"W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W\x01\x81`\x1F\x82\x01\x12\x15a\x06\"W\x80Q\x90aN\xD6\x82a4CV[\x92aN\xE4`@Q\x94\x85a4 V[\x82\x84R` \x83\x83\x01\x01\x11a\x06\"W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_aN\x06V[\x81Q\x91\x90`A\x83\x03aO9WaO2\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aS%V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aO\xEEW\x80aOUWPPV[`\x01\x81\x03aO\x85W\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03aO\xB9WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14aO\xC3WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x06\"W\x815aP\x19\x81a7.V[\x92aP'`@Q\x94\x85a4 V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x06\"W` \x01\x90[\x82\x82\x10aPOWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01aPBV[\x90\x94\x93\x92\x91_\x95\x82R` \x82\x01RaPv\x83a7.V[\x90aP\x84`@Q\x92\x83a4 V[\x83\x82R`\x1F\x19aP\x93\x85a7.V[\x01_[\x81\x81\x10aQ\xBBWPP``\x01\x90\x81R_\x92[\x80\x84\x10aP\xB5WPPPPV[\x90\x91\x92\x94aP\xC4\x86\x84Qa7\xB7V[Q\x90aP\xD4a?\x04\x88\x85\x88a>\xA2V[\x81\x92\x91\x01`\xA0\x83\x82\x03\x12a\x06\"WaP\xEB\x83aI\xE7V[\x93` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x82aQ\x0C\x91\x86\x01a4\x95V[\x94`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x83aQ-\x91\x87\x01a4\x95V[\x91``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06\"W\x84aQN\x91\x88\x01aP\x02V[\x91`\x80\x87\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x06\"W`\x01\x98aQ\x8Ea\x16\x7F\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94aQ\xB1\x9B\x01aP\x02V[\x95\x86`\x80\x86\x01R\x85``\x86\x01R`@\x85\x01R` \x84\x01R\x16\x90RQ\x90Q\x90a5\x0BV[\x95\x01\x92\x91\x90aP\xA8V[`@\x95\x93\x94\x95Q\x90`\xA0\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x1F\x92W` \x92`@R_\x81R``\x83\x82\x01R```@\x82\x01R``\x80\x82\x01R```\x80\x82\x01R\x82\x82\x88\x01\x01R\x01\x94\x93\x92\x94aP\x96V[\x93\x92aR\x19\x85a7FV[\x92_[\x82\x81\x10aR\xF3WPaR-\x86a7FV[\x95_[\x81\x81\x10aR?WPPPPPPV[aRH\x86aT\xF5V[\x82\x81\x14aR\xCBW_\x19aR[\x82\x89a7\xB7V[R\x80aRg\x83\x8Ba7\xB7V[R_[\x85\x81\x10aR{WPP`\x01\x01aR0V[\x81aR\x86\x82\x89a7\xB7V[Q\x14aR\x95W[`\x01\x01aRjV[aR\x9F\x81\x86a7\xB7V[Q\x90aR\xAB\x82\x8Aa7\xB7V[Q\x91\x82\x15a-qWaR\xC3_\x19`\x01\x94\x01\x91\x8Ba7\xB7V[R\x90PaR\x8DV[\x7F\xA8\x12\x90\x95\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x80aS\0`\x01\x92\x84a7\xB7V[QaS\x1EaS\x17aS\x11\x83\x8Aa7\xB7V[Qa<\x01V[\x91\x88a7\xB7V[R\x01aR\x1CV[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aS\xA9W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x17\xD8W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aS\x9FW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x91\x90\x82Q\x90_\x93[\x82\x85\x10aS\xC8WPPPV[\x90\x91\x92\x93\x82aS\xD7\x82\x84a7\xB7V[QQ\x14aS\xEAW`\x01\x01\x93\x92\x91\x90aS\xBCV[\x92PPP\x90V[\x90\x92\x91_\x93_[\x81\x81\x10aT\x05WPPPPV[aT\x13a\x15\xD2\x82\x84\x87a7wV[_[\x81\x81\x10aThWPPPaT-a\x15\xF2\x82\x84\x87a7wV[_[\x81\x81\x10aTAWPPP`\x01\x01aS\xF8V[\x85aTM\x82\x84\x86a<\x0FV[5\x14aT[W`\x01\x01aT/V[P`\x01\x97PPPPPPPV[\x85aTt\x82\x84\x86a;\xEAV[5\x14aT[W`\x01\x01aT\x15V[\x91\x90\x93\x92\x93\x91`@aT\xA2\x85\x82aT\x99\x89\x86a7\xB7V[Q\x01Q\x93a7\xB7V[Q\x01Q\x80\x82\x03aT\xE3W[PPP` \x81aT\xD4\x92Q\x94aT\xC9`@\x83\x01\x96\x87Q\x90a7\xB7V[R\x01Q\x83Q\x90a7\xB7V[RaT\xDF\x81Qa<\x01V[\x90RV[\x90\x94P\x92P\x90P` aT\xD4_aT\xADV[\x90\x81Q_\x92[\x81\x84\x10aU\x06WPPV[\x90\x91\x92aU\x13\x81\x83a7\xB7V[Q\x15aU$W`\x01\x01\x92\x91\x90aT\xFBV[\x91PP\x90V\xFE\xA1dsolcC\0\x08%\0\n",
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
    /**Custom error with signature `AfterTagNotInTransaction(bytes32)` and selector `0x211b12b7`.
```solidity
error AfterTagNotInTransaction(bytes32 tag);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AfterTagNotInTransaction {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<AfterTagNotInTransaction>
        for UnderlyingRustTuple<'_> {
            fn from(value: AfterTagNotInTransaction) -> Self {
                (value.tag,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AfterTagNotInTransaction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tag: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AfterTagNotInTransaction {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AfterTagNotInTransaction(bytes32)";
            const SELECTOR: [u8; 4] = [33u8, 27u8, 18u8, 183u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tag),
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
    /**Custom error with signature `BeforeTagNotInTransaction(bytes32)` and selector `0x45bd4960`.
```solidity
error BeforeTagNotInTransaction(bytes32 tag);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeforeTagNotInTransaction {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<BeforeTagNotInTransaction>
        for UnderlyingRustTuple<'_> {
            fn from(value: BeforeTagNotInTransaction) -> Self {
                (value.tag,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BeforeTagNotInTransaction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tag: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BeforeTagNotInTransaction {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BeforeTagNotInTransaction(bytes32)";
            const SELECTOR: [u8; 4] = [69u8, 189u8, 73u8, 96u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tag),
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
    /**Custom error with signature `CommitmentCountMismatch(uint256,uint256)` and selector `0x33474d08`.
```solidity
error CommitmentCountMismatch(uint256 expected, uint256 actual);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CommitmentCountMismatch {
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CommitmentCountMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: CommitmentCountMismatch) -> Self {
                (value.expected, value.actual)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CommitmentCountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    actual: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CommitmentCountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CommitmentCountMismatch(uint256,uint256)";
            const SELECTOR: [u8; 4] = [51u8, 71u8, 77u8, 8u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.expected),
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
    /**Custom error with signature `CommitmentTreeNotEmpty()` and selector `0x66fda36f`.
```solidity
error CommitmentTreeNotEmpty();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CommitmentTreeNotEmpty;
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
        impl ::core::convert::From<CommitmentTreeNotEmpty> for UnderlyingRustTuple<'_> {
            fn from(value: CommitmentTreeNotEmpty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CommitmentTreeNotEmpty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CommitmentTreeNotEmpty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CommitmentTreeNotEmpty()";
            const SELECTOR: [u8; 4] = [102u8, 253u8, 163u8, 111u8];
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
    /**Custom error with signature `CommitmentTreeRootMismatch(bytes32,bytes32)` and selector `0xd5df6dd6`.
```solidity
error CommitmentTreeRootMismatch(bytes32 expected, bytes32 actual);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CommitmentTreeRootMismatch {
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::FixedBytes<32>,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<CommitmentTreeRootMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: CommitmentTreeRootMismatch) -> Self {
                (value.expected, value.actual)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CommitmentTreeRootMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    actual: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CommitmentTreeRootMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CommitmentTreeRootMismatch(bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [213u8, 223u8, 109u8, 214u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.expected),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
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
    /**Custom error with signature `CyclicConstraints()` and selector `0xa8129095`.
```solidity
error CyclicConstraints();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CyclicConstraints;
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
        impl ::core::convert::From<CyclicConstraints> for UnderlyingRustTuple<'_> {
            fn from(value: CyclicConstraints) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CyclicConstraints {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CyclicConstraints {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CyclicConstraints()";
            const SELECTOR: [u8; 4] = [168u8, 18u8, 144u8, 149u8];
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
    /**Custom error with signature `EmptyCommitmentTreeNotAllowed()` and selector `0x4cd41ad8`.
```solidity
error EmptyCommitmentTreeNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCommitmentTreeNotAllowed;
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
        impl ::core::convert::From<EmptyCommitmentTreeNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCommitmentTreeNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyCommitmentTreeNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCommitmentTreeNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCommitmentTreeNotAllowed()";
            const SELECTOR: [u8; 4] = [76u8, 212u8, 26u8, 216u8];
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
    /**Custom error with signature `LeafCountExceedsCapacity(uint256,uint256)` and selector `0x6be696a3`.
```solidity
error LeafCountExceedsCapacity(uint256 leafCount, uint256 capacity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeafCountExceedsCapacity {
        #[allow(missing_docs)]
        pub leafCount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<LeafCountExceedsCapacity>
        for UnderlyingRustTuple<'_> {
            fn from(value: LeafCountExceedsCapacity) -> Self {
                (value.leafCount, value.capacity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LeafCountExceedsCapacity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    leafCount: tuple.0,
                    capacity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeafCountExceedsCapacity {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeafCountExceedsCapacity(uint256,uint256)";
            const SELECTOR: [u8; 4] = [107u8, 230u8, 150u8, 163u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.leafCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.capacity),
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
    /**Custom error with signature `LegacyProtocolAdapterNotStopped(address)` and selector `0xf4ef172d`.
```solidity
error LegacyProtocolAdapterNotStopped(address legacyProtocolAdapter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LegacyProtocolAdapterNotStopped {
        #[allow(missing_docs)]
        pub legacyProtocolAdapter: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<LegacyProtocolAdapterNotStopped>
        for UnderlyingRustTuple<'_> {
            fn from(value: LegacyProtocolAdapterNotStopped) -> Self {
                (value.legacyProtocolAdapter,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LegacyProtocolAdapterNotStopped {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    legacyProtocolAdapter: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LegacyProtocolAdapterNotStopped {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LegacyProtocolAdapterNotStopped(address)";
            const SELECTOR: [u8; 4] = [244u8, 239u8, 23u8, 45u8];
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
                        &self.legacyProtocolAdapter,
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
    /**Custom error with signature `LogicRefAlreadyDenied(bytes32,bool)` and selector `0x127d335e`.
```solidity
error LogicRefAlreadyDenied(bytes32 logicRef, bool consumed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LogicRefAlreadyDenied {
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
        impl ::core::convert::From<LogicRefAlreadyDenied> for UnderlyingRustTuple<'_> {
            fn from(value: LogicRefAlreadyDenied) -> Self {
                (value.logicRef, value.consumed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LogicRefAlreadyDenied {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    logicRef: tuple.0,
                    consumed: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LogicRefAlreadyDenied {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LogicRefAlreadyDenied(bytes32,bool)";
            const SELECTOR: [u8; 4] = [18u8, 125u8, 51u8, 94u8];
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
    /**Custom error with signature `MissingHistoricalRoot(bytes32)` and selector `0x79930a36`.
```solidity
error MissingHistoricalRoot(bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MissingHistoricalRoot {
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
        impl ::core::convert::From<MissingHistoricalRoot> for UnderlyingRustTuple<'_> {
            fn from(value: MissingHistoricalRoot) -> Self {
                (value.root,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MissingHistoricalRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { root: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MissingHistoricalRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MissingHistoricalRoot(bytes32)";
            const SELECTOR: [u8; 4] = [121u8, 147u8, 10u8, 54u8];
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
    /**Custom error with signature `MissingNullifier(bytes32)` and selector `0xdbc1aa1e`.
```solidity
error MissingNullifier(bytes32 nullifier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MissingNullifier {
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
        impl ::core::convert::From<MissingNullifier> for UnderlyingRustTuple<'_> {
            fn from(value: MissingNullifier) -> Self {
                (value.nullifier,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MissingNullifier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { nullifier: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MissingNullifier {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MissingNullifier(bytes32)";
            const SELECTOR: [u8; 4] = [219u8, 193u8, 170u8, 30u8];
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
    /**Custom error with signature `NullifierBatchLeavesGap(uint256)` and selector `0xc65c7170`.
```solidity
error NullifierBatchLeavesGap(uint256 start);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NullifierBatchLeavesGap {
        #[allow(missing_docs)]
        pub start: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<NullifierBatchLeavesGap> for UnderlyingRustTuple<'_> {
            fn from(value: NullifierBatchLeavesGap) -> Self {
                (value.start,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NullifierBatchLeavesGap {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { start: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NullifierBatchLeavesGap {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NullifierBatchLeavesGap(uint256)";
            const SELECTOR: [u8; 4] = [198u8, 92u8, 113u8, 112u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
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
    /**Custom error with signature `NullifierBatchOutOfRange(uint256,uint256)` and selector `0x716b33b3`.
```solidity
error NullifierBatchOutOfRange(uint256 end, uint256 nullifierCount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NullifierBatchOutOfRange {
        #[allow(missing_docs)]
        pub end: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nullifierCount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<NullifierBatchOutOfRange>
        for UnderlyingRustTuple<'_> {
            fn from(value: NullifierBatchOutOfRange) -> Self {
                (value.end, value.nullifierCount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NullifierBatchOutOfRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    end: tuple.0,
                    nullifierCount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NullifierBatchOutOfRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NullifierBatchOutOfRange(uint256,uint256)";
            const SELECTOR: [u8; 4] = [113u8, 107u8, 51u8, 179u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.end),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifierCount),
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
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
```solidity
error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        #[allow(missing_docs)]
        pub bits: u8,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
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
        impl ::core::convert::From<SafeCastOverflowedUintDowncast>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bits: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bits),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
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
    /**Custom error with signature `UnacceptedKindTreeRoot(bytes32)` and selector `0xc93ee163`.
```solidity
error UnacceptedKindTreeRoot(bytes32 kindTreeRoot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnacceptedKindTreeRoot {
        #[allow(missing_docs)]
        pub kindTreeRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<UnacceptedKindTreeRoot> for UnderlyingRustTuple<'_> {
            fn from(value: UnacceptedKindTreeRoot) -> Self {
                (value.kindTreeRoot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnacceptedKindTreeRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { kindTreeRoot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnacceptedKindTreeRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnacceptedKindTreeRoot(bytes32)";
            const SELECTOR: [u8; 4] = [201u8, 62u8, 225u8, 99u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kindTreeRoot),
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
    /**Custom error with signature `ZeroKindTreeRootNotAllowed()` and selector `0x0bf7718c`.
```solidity
error ZeroKindTreeRootNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroKindTreeRootNotAllowed;
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
        impl ::core::convert::From<ZeroKindTreeRootNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: ZeroKindTreeRootNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ZeroKindTreeRootNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroKindTreeRootNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroKindTreeRootNotAllowed()";
            const SELECTOR: [u8; 4] = [11u8, 247u8, 113u8, 140u8];
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
    /**Custom error with signature `ZeroLegacyProtocolAdapterNotAllowed()` and selector `0x0a1ad28c`.
```solidity
error ZeroLegacyProtocolAdapterNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroLegacyProtocolAdapterNotAllowed;
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
        impl ::core::convert::From<ZeroLegacyProtocolAdapterNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: ZeroLegacyProtocolAdapterNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ZeroLegacyProtocolAdapterNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroLegacyProtocolAdapterNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroLegacyProtocolAdapterNotAllowed()";
            const SELECTOR: [u8; 4] = [10u8, 26u8, 210u8, 140u8];
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
    /**Event with signature `CommitmentTreeMigrated(bytes32,uint256)` and selector `0x9cfc151cd0c8c49916e15e9af15c5d553ff85cc13d96fef94340655fd6794e9f`.
```solidity
event CommitmentTreeMigrated(bytes32 indexed root, uint256 leafCount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CommitmentTreeMigrated {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub leafCount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CommitmentTreeMigrated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "CommitmentTreeMigrated(bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 252u8, 21u8, 28u8, 208u8, 200u8, 196u8, 153u8, 22u8, 225u8, 94u8,
                154u8, 241u8, 92u8, 93u8, 85u8, 63u8, 248u8, 92u8, 193u8, 61u8, 150u8,
                254u8, 249u8, 67u8, 64u8, 101u8, 95u8, 214u8, 121u8, 78u8, 159u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    root: topics.1,
                    leafCount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.leafCount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.root.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.root);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CommitmentTreeMigrated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CommitmentTreeMigrated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CommitmentTreeMigrated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `KindTreeRootUpdated(bytes32)` and selector `0x10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e2`.
```solidity
event KindTreeRootUpdated(bytes32 indexed kindTreeRoot);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KindTreeRootUpdated {
        #[allow(missing_docs)]
        pub kindTreeRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for KindTreeRootUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "KindTreeRootUpdated(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                16u8, 99u8, 93u8, 76u8, 95u8, 1u8, 141u8, 159u8, 78u8, 123u8, 152u8,
                227u8, 97u8, 41u8, 25u8, 81u8, 98u8, 216u8, 53u8, 67u8, 238u8, 26u8,
                158u8, 47u8, 209u8, 220u8, 205u8, 27u8, 37u8, 35u8, 242u8, 226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { kindTreeRoot: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.kindTreeRoot.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.kindTreeRoot);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KindTreeRootUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KindTreeRootUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KindTreeRootUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LogicRefDenied(bytes32,bool)` and selector `0xa70f4433615e083adf1b97e0c32d9a721c39c26d6dd6c28fe96176ab909f0eca`.
```solidity
event LogicRefDenied(bytes32 indexed logicRef, bool consumed);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LogicRefDenied {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LogicRefDenied {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "LogicRefDenied(bytes32,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8, 15u8, 68u8, 51u8, 97u8, 94u8, 8u8, 58u8, 223u8, 27u8, 151u8,
                224u8, 195u8, 45u8, 154u8, 114u8, 28u8, 57u8, 194u8, 109u8, 109u8, 214u8,
                194u8, 143u8, 233u8, 97u8, 118u8, 171u8, 144u8, 159u8, 14u8, 202u8,
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
                    consumed: data.0,
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.consumed,
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
        impl alloy_sol_types::private::IntoLogData for LogicRefDenied {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LogicRefDenied> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LogicRefDenied) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `MigrationFinalized()` and selector `0x0418479ca6ac8dbf83626a5e3cdc5915233f1a7570c935115db0acf533b649bf`.
```solidity
event MigrationFinalized();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MigrationFinalized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MigrationFinalized {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MigrationFinalized()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                4u8, 24u8, 71u8, 156u8, 166u8, 172u8, 141u8, 191u8, 131u8, 98u8, 106u8,
                94u8, 60u8, 220u8, 89u8, 21u8, 35u8, 63u8, 26u8, 117u8, 112u8, 201u8,
                53u8, 17u8, 93u8, 176u8, 172u8, 245u8, 51u8, 182u8, 73u8, 191u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for MigrationFinalized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MigrationFinalized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MigrationFinalized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NullifierBatchMigrated(uint256,uint256)` and selector `0x63b187673d450ed7145495477aa47714450eb17d84b291d809daf29da8096758`.
```solidity
event NullifierBatchMigrated(uint256 start, uint256 count);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NullifierBatchMigrated {
        #[allow(missing_docs)]
        pub start: alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NullifierBatchMigrated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NullifierBatchMigrated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                99u8, 177u8, 135u8, 103u8, 61u8, 69u8, 14u8, 215u8, 20u8, 84u8, 149u8,
                71u8, 122u8, 164u8, 119u8, 20u8, 69u8, 14u8, 177u8, 125u8, 132u8, 178u8,
                145u8, 216u8, 9u8, 218u8, 242u8, 157u8, 168u8, 9u8, 103u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    start: data.0,
                    count: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.count),
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
        impl alloy_sol_types::private::IntoLogData for NullifierBatchMigrated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NullifierBatchMigrated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NullifierBatchMigrated) -> alloy_sol_types::private::LogData {
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
constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector, address legacyProtocolAdapter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub riscZeroVerifierRouter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub riscZeroVerifierSelector: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub legacyProtocolAdapter: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value.riscZeroVerifierRouter,
                        value.riscZeroVerifierSelector,
                        value.legacyProtocolAdapter,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        riscZeroVerifierRouter: tuple.0,
                        riscZeroVerifierSelector: tuple.1,
                        legacyProtocolAdapter: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.legacyProtocolAdapter,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `EMPTY_KIND_TREE_ROOT()` and selector `0x64c12af2`.
```solidity
function EMPTY_KIND_TREE_ROOT() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMPTY_KIND_TREE_ROOTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`EMPTY_KIND_TREE_ROOT()`](EMPTY_KIND_TREE_ROOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMPTY_KIND_TREE_ROOTReturn {
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
            impl ::core::convert::From<EMPTY_KIND_TREE_ROOTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMPTY_KIND_TREE_ROOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMPTY_KIND_TREE_ROOTCall {
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
            impl ::core::convert::From<EMPTY_KIND_TREE_ROOTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMPTY_KIND_TREE_ROOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMPTY_KIND_TREE_ROOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EMPTY_KIND_TREE_ROOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EMPTY_KIND_TREE_ROOT()";
            const SELECTOR: [u8; 4] = [100u8, 193u8, 42u8, 242u8];
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
                        let r: EMPTY_KIND_TREE_ROOTReturn = r.into();
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
                        let r: EMPTY_KIND_TREE_ROOTReturn = r.into();
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
    /**Function with signature `denyLogicRefs((bytes32,bool)[])` and selector `0x1a8473bc`.
```solidity
function denyLogicRefs(ILogicRefDenylist.DeniedLogicRef[] memory logicRefs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct denyLogicRefsCall {
        #[allow(missing_docs)]
        pub logicRefs: alloy::sol_types::private::Vec<
            <ILogicRefDenylist::DeniedLogicRef as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`denyLogicRefs((bytes32,bool)[])`](denyLogicRefsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct denyLogicRefsReturn {}
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
                alloy::sol_types::sol_data::Array<ILogicRefDenylist::DeniedLogicRef>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ILogicRefDenylist::DeniedLogicRef as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<denyLogicRefsCall> for UnderlyingRustTuple<'_> {
                fn from(value: denyLogicRefsCall) -> Self {
                    (value.logicRefs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for denyLogicRefsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { logicRefs: tuple.0 }
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
            impl ::core::convert::From<denyLogicRefsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: denyLogicRefsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for denyLogicRefsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl denyLogicRefsReturn {
            fn _tokenize(
                &self,
            ) -> <denyLogicRefsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for denyLogicRefsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<ILogicRefDenylist::DeniedLogicRef>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = denyLogicRefsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "denyLogicRefs((bytes32,bool)[])";
            const SELECTOR: [u8; 4] = [26u8, 132u8, 115u8, 188u8];
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
                        ILogicRefDenylist::DeniedLogicRef,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRefs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                denyLogicRefsReturn::_tokenize(ret)
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
    /**Function with signature `getKindTreeRoot()` and selector `0xfb0ff680`.
```solidity
function getKindTreeRoot() external view returns (bytes32 kindTreeRoot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKindTreeRootCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKindTreeRoot()`](getKindTreeRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKindTreeRootReturn {
        #[allow(missing_docs)]
        pub kindTreeRoot: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getKindTreeRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKindTreeRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKindTreeRootCall {
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
            impl ::core::convert::From<getKindTreeRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKindTreeRootReturn) -> Self {
                    (value.kindTreeRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKindTreeRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { kindTreeRoot: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKindTreeRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKindTreeRoot()";
            const SELECTOR: [u8; 4] = [251u8, 15u8, 246u8, 128u8];
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
                        let r: getKindTreeRootReturn = r.into();
                        r.kindTreeRoot
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
                        let r: getKindTreeRootReturn = r.into();
                        r.kindTreeRoot
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
    /**Function with signature `getLegacyProtocolAdapter()` and selector `0x364f375b`.
```solidity
function getLegacyProtocolAdapter() external view returns (address legacyProtocolAdapter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLegacyProtocolAdapterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getLegacyProtocolAdapter()`](getLegacyProtocolAdapterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLegacyProtocolAdapterReturn {
        #[allow(missing_docs)]
        pub legacyProtocolAdapter: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getLegacyProtocolAdapterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLegacyProtocolAdapterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLegacyProtocolAdapterCall {
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
            impl ::core::convert::From<getLegacyProtocolAdapterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLegacyProtocolAdapterReturn) -> Self {
                    (value.legacyProtocolAdapter,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLegacyProtocolAdapterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        legacyProtocolAdapter: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLegacyProtocolAdapterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLegacyProtocolAdapter()";
            const SELECTOR: [u8; 4] = [54u8, 79u8, 55u8, 91u8];
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
                        let r: getLegacyProtocolAdapterReturn = r.into();
                        r.legacyProtocolAdapter
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
                        let r: getLegacyProtocolAdapterReturn = r.into();
                        r.legacyProtocolAdapter
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
    /**Function with signature `isCommitmentTreeRootHistorical(bytes32)` and selector `0xf8adf4c3`.
```solidity
function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCommitmentTreeRootHistoricalCall {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCommitmentTreeRootHistorical(bytes32)`](isCommitmentTreeRootHistoricalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCommitmentTreeRootHistoricalReturn {
        #[allow(missing_docs)]
        pub isHistorical: bool,
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
            impl ::core::convert::From<isCommitmentTreeRootHistoricalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCommitmentTreeRootHistoricalCall) -> Self {
                    (value.root,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCommitmentTreeRootHistoricalCall {
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
            impl ::core::convert::From<isCommitmentTreeRootHistoricalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCommitmentTreeRootHistoricalReturn) -> Self {
                    (value.isHistorical,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCommitmentTreeRootHistoricalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isHistorical: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCommitmentTreeRootHistoricalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCommitmentTreeRootHistorical(bytes32)";
            const SELECTOR: [u8; 4] = [248u8, 173u8, 244u8, 195u8];
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
                        let r: isCommitmentTreeRootHistoricalReturn = r.into();
                        r.isHistorical
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
                        let r: isCommitmentTreeRootHistoricalReturn = r.into();
                        r.isHistorical
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
    /**Function with signature `isLogicRefDenied(bytes32,bool)` and selector `0x171eac18`.
```solidity
function isLogicRefDenied(bytes32 logicRef, bool consumed) external view returns (bool isDenied);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLogicRefDeniedCall {
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub consumed: bool,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isLogicRefDenied(bytes32,bool)`](isLogicRefDeniedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLogicRefDeniedReturn {
        #[allow(missing_docs)]
        pub isDenied: bool,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<isLogicRefDeniedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isLogicRefDeniedCall) -> Self {
                    (value.logicRef, value.consumed)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isLogicRefDeniedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        logicRef: tuple.0,
                        consumed: tuple.1,
                    }
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
            impl ::core::convert::From<isLogicRefDeniedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isLogicRefDeniedReturn) -> Self {
                    (value.isDenied,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isLogicRefDeniedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isDenied: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isLogicRefDeniedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isLogicRefDenied(bytes32,bool)";
            const SELECTOR: [u8; 4] = [23u8, 30u8, 172u8, 24u8];
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
                        let r: isLogicRefDeniedReturn = r.into();
                        r.isDenied
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
                        let r: isLogicRefDeniedReturn = r.into();
                        r.isDenied
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
    /**Function with signature `migrateCommitmentTree(bytes32[])` and selector `0xfa83dd8f`.
```solidity
function migrateCommitmentTree(bytes32[] memory sides) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateCommitmentTreeCall {
        #[allow(missing_docs)]
        pub sides: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`migrateCommitmentTree(bytes32[])`](migrateCommitmentTreeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateCommitmentTreeReturn {}
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
            impl ::core::convert::From<migrateCommitmentTreeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateCommitmentTreeCall) -> Self {
                    (value.sides,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateCommitmentTreeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { sides: tuple.0 }
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
            impl ::core::convert::From<migrateCommitmentTreeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateCommitmentTreeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateCommitmentTreeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl migrateCommitmentTreeReturn {
            fn _tokenize(
                &self,
            ) -> <migrateCommitmentTreeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateCommitmentTreeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateCommitmentTreeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateCommitmentTree(bytes32[])";
            const SELECTOR: [u8; 4] = [250u8, 131u8, 221u8, 143u8];
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.sides),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                migrateCommitmentTreeReturn::_tokenize(ret)
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
    /**Function with signature `migrateNullifierSet(uint256,uint256)` and selector `0x03a7fde5`.
```solidity
function migrateNullifierSet(uint256 start, uint256 count) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateNullifierSetCall {
        #[allow(missing_docs)]
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`migrateNullifierSet(uint256,uint256)`](migrateNullifierSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateNullifierSetReturn {}
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
            impl ::core::convert::From<migrateNullifierSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateNullifierSetCall) -> Self {
                    (value.start, value.count)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateNullifierSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        start: tuple.0,
                        count: tuple.1,
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
            impl ::core::convert::From<migrateNullifierSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateNullifierSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateNullifierSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl migrateNullifierSetReturn {
            fn _tokenize(
                &self,
            ) -> <migrateNullifierSetCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateNullifierSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateNullifierSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateNullifierSet(uint256,uint256)";
            const SELECTOR: [u8; 4] = [3u8, 167u8, 253u8, 229u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.count),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                migrateNullifierSetReturn::_tokenize(ret)
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
    /**Function with signature `setKindTreeRoot(bytes32)` and selector `0x8b33e200`.
```solidity
function setKindTreeRoot(bytes32 newKindTreeRoot) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setKindTreeRootCall {
        #[allow(missing_docs)]
        pub newKindTreeRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`setKindTreeRoot(bytes32)`](setKindTreeRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setKindTreeRootReturn {}
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
            impl ::core::convert::From<setKindTreeRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: setKindTreeRootCall) -> Self {
                    (value.newKindTreeRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setKindTreeRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newKindTreeRoot: tuple.0 }
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
            impl ::core::convert::From<setKindTreeRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setKindTreeRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setKindTreeRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setKindTreeRootReturn {
            fn _tokenize(
                &self,
            ) -> <setKindTreeRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setKindTreeRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setKindTreeRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setKindTreeRoot(bytes32)";
            const SELECTOR: [u8; 4] = [139u8, 51u8, 226u8, 0u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newKindTreeRoot),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setKindTreeRootReturn::_tokenize(ret)
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
    ///Container for all the [`MigrationalProtocolAdapter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum MigrationalProtocolAdapterCalls {
        #[allow(missing_docs)]
        EMPTY_KIND_TREE_ROOT(EMPTY_KIND_TREE_ROOTCall),
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
        commitmentTreeSides(commitmentTreeSidesCall),
        #[allow(missing_docs)]
        commitmentTreeZeros(commitmentTreeZerosCall),
        #[allow(missing_docs)]
        denyLogicRefs(denyLogicRefsCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        getImplementation(getImplementationCall),
        #[allow(missing_docs)]
        getKindTreeRoot(getKindTreeRootCall),
        #[allow(missing_docs)]
        getLegacyProtocolAdapter(getLegacyProtocolAdapterCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isCommitmentTreeRootHistorical(isCommitmentTreeRootHistoricalCall),
        #[allow(missing_docs)]
        isLogicRefDenied(isLogicRefDeniedCall),
        #[allow(missing_docs)]
        isNullifierContained(isNullifierContainedCall),
        #[allow(missing_docs)]
        latestCommitmentTreeRoot(latestCommitmentTreeRootCall),
        #[allow(missing_docs)]
        migrateCommitmentTree(migrateCommitmentTreeCall),
        #[allow(missing_docs)]
        migrateNullifierSet(migrateNullifierSetCall),
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
        riscZeroVerifierPaused(riscZeroVerifierPausedCall),
        #[allow(missing_docs)]
        setKindTreeRoot(setKindTreeRootCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl MigrationalProtocolAdapterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 167u8, 253u8, 229u8],
            [23u8, 30u8, 172u8, 24u8],
            [26u8, 132u8, 115u8, 188u8],
            [54u8, 79u8, 55u8, 91u8],
            [63u8, 75u8, 168u8, 58u8],
            [79u8, 30u8, 242u8, 134u8],
            [81u8, 148u8, 91u8, 6u8],
            [82u8, 209u8, 144u8, 45u8],
            [92u8, 151u8, 90u8, 187u8],
            [100u8, 193u8, 42u8, 242u8],
            [103u8, 53u8, 49u8, 34u8],
            [107u8, 240u8, 160u8, 75u8],
            [113u8, 80u8, 24u8, 166u8],
            [132u8, 86u8, 203u8, 89u8],
            [139u8, 51u8, 226u8, 0u8],
            [141u8, 165u8, 203u8, 91u8],
            [146u8, 57u8, 172u8, 12u8],
            [160u8, 96u8, 86u8, 247u8],
            [170u8, 241u8, 15u8, 66u8],
            [173u8, 60u8, 177u8, 204u8],
            [189u8, 235u8, 68u8, 45u8],
            [193u8, 176u8, 190u8, 215u8],
            [196u8, 73u8, 86u8, 209u8],
            [196u8, 214u8, 109u8, 232u8],
            [227u8, 90u8, 93u8, 47u8],
            [237u8, 131u8, 205u8, 199u8],
            [242u8, 253u8, 227u8, 139u8],
            [248u8, 173u8, 244u8, 195u8],
            [250u8, 131u8, 221u8, 143u8],
            [251u8, 15u8, 246u8, 128u8],
            [254u8, 24u8, 171u8, 145u8],
            [255u8, 161u8, 173u8, 116u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(migrateNullifierSet),
            ::core::stringify!(isLogicRefDenied),
            ::core::stringify!(denyLogicRefs),
            ::core::stringify!(getLegacyProtocolAdapter),
            ::core::stringify!(unpause),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(commitmentTreeZeros),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(paused),
            ::core::stringify!(EMPTY_KIND_TREE_ROOT),
            ::core::stringify!(riscZeroVerifierPaused),
            ::core::stringify!(RISC_ZERO_VERIFIER_ROUTER),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(pause),
            ::core::stringify!(setKindTreeRoot),
            ::core::stringify!(owner),
            ::core::stringify!(execute),
            ::core::stringify!(commitmentTreeDepth),
            ::core::stringify!(getImplementation),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(latestCommitmentTreeRoot),
            ::core::stringify!(isNullifierContained),
            ::core::stringify!(commitmentCount),
            ::core::stringify!(initialize),
            ::core::stringify!(RISC_ZERO_VERIFIER_SELECTOR),
            ::core::stringify!(commitmentTreeSides),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(isCommitmentTreeRootHistorical),
            ::core::stringify!(migrateCommitmentTree),
            ::core::stringify!(getKindTreeRoot),
            ::core::stringify!(commitmentTreeCapacity),
            ::core::stringify!(VERSION),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <migrateNullifierSetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isLogicRefDeniedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <denyLogicRefsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getLegacyProtocolAdapterCall as alloy_sol_types::SolCall>::SIGNATURE,
            <unpauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeZerosCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <EMPTY_KIND_TREE_ROOTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setKindTreeRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <executeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getImplementationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isNullifierContainedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeSidesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <migrateCommitmentTreeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKindTreeRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::SIGNATURE,
            <VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for MigrationalProtocolAdapterCalls {
        const NAME: &'static str = "MigrationalProtocolAdapterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EMPTY_KIND_TREE_ROOT(_) => {
                    <EMPTY_KIND_TREE_ROOTCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::commitmentTreeSides(_) => {
                    <commitmentTreeSidesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentTreeZeros(_) => {
                    <commitmentTreeZerosCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::denyLogicRefs(_) => {
                    <denyLogicRefsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getImplementation(_) => {
                    <getImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKindTreeRoot(_) => {
                    <getKindTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLegacyProtocolAdapter(_) => {
                    <getLegacyProtocolAdapterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCommitmentTreeRootHistorical(_) => {
                    <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isLogicRefDenied(_) => {
                    <isLogicRefDeniedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isNullifierContained(_) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestCommitmentTreeRoot(_) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrateCommitmentTree(_) => {
                    <migrateCommitmentTreeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrateNullifierSet(_) => {
                    <migrateNullifierSetCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::riscZeroVerifierPaused(_) => {
                    <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setKindTreeRoot(_) => {
                    <setKindTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls>] = &[
                {
                    fn migrateNullifierSet(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <migrateNullifierSetCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::migrateNullifierSet)
                    }
                    migrateNullifierSet
                },
                {
                    fn isLogicRefDenied(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <isLogicRefDeniedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::isLogicRefDenied)
                    }
                    isLogicRefDenied
                },
                {
                    fn denyLogicRefs(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <denyLogicRefsCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::denyLogicRefs)
                    }
                    denyLogicRefs
                },
                {
                    fn getLegacyProtocolAdapter(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <getLegacyProtocolAdapterCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterCalls::getLegacyProtocolAdapter,
                            )
                    }
                    getLegacyProtocolAdapter
                },
                {
                    fn unpause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::unpause)
                    }
                    unpause
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn commitmentTreeZeros(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <commitmentTreeZerosCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::commitmentTreeZeros)
                    }
                    commitmentTreeZeros
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn paused(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn EMPTY_KIND_TREE_ROOT(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <EMPTY_KIND_TREE_ROOTCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::EMPTY_KIND_TREE_ROOT)
                    }
                    EMPTY_KIND_TREE_ROOT
                },
                {
                    fn riscZeroVerifierPaused(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::riscZeroVerifierPaused)
                    }
                    riscZeroVerifierPaused
                },
                {
                    fn RISC_ZERO_VERIFIER_ROUTER(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterCalls::RISC_ZERO_VERIFIER_ROUTER,
                            )
                    }
                    RISC_ZERO_VERIFIER_ROUTER
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn pause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::pause)
                    }
                    pause
                },
                {
                    fn setKindTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <setKindTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::setKindTreeRoot)
                    }
                    setKindTreeRoot
                },
                {
                    fn owner(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn execute(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::execute)
                    }
                    execute
                },
                {
                    fn commitmentTreeDepth(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <commitmentTreeDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::commitmentTreeDepth)
                    }
                    commitmentTreeDepth
                },
                {
                    fn getImplementation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <getImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::getImplementation)
                    }
                    getImplementation
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterCalls::UPGRADE_INTERFACE_VERSION,
                            )
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn latestCommitmentTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterCalls::latestCommitmentTreeRoot,
                            )
                    }
                    latestCommitmentTreeRoot
                },
                {
                    fn isNullifierContained(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::isNullifierContained)
                    }
                    isNullifierContained
                },
                {
                    fn commitmentCount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <commitmentCountCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::commitmentCount)
                    }
                    commitmentCount
                },
                {
                    fn initialize(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn RISC_ZERO_VERIFIER_SELECTOR(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterCalls::RISC_ZERO_VERIFIER_SELECTOR,
                            )
                    }
                    RISC_ZERO_VERIFIER_SELECTOR
                },
                {
                    fn commitmentTreeSides(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <commitmentTreeSidesCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::commitmentTreeSides)
                    }
                    commitmentTreeSides
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn isCommitmentTreeRootHistorical(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterCalls::isCommitmentTreeRootHistorical,
                            )
                    }
                    isCommitmentTreeRootHistorical
                },
                {
                    fn migrateCommitmentTree(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <migrateCommitmentTreeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::migrateCommitmentTree)
                    }
                    migrateCommitmentTree
                },
                {
                    fn getKindTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <getKindTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::getKindTreeRoot)
                    }
                    getKindTreeRoot
                },
                {
                    fn commitmentTreeCapacity(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::commitmentTreeCapacity)
                    }
                    commitmentTreeCapacity
                },
                {
                    fn VERSION(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterCalls::VERSION)
                    }
                    VERSION
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
                Self::EMPTY_KIND_TREE_ROOT(inner) => {
                    <EMPTY_KIND_TREE_ROOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::denyLogicRefs(inner) => {
                    <denyLogicRefsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getKindTreeRoot(inner) => {
                    <getKindTreeRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLegacyProtocolAdapter(inner) => {
                    <getLegacyProtocolAdapterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isCommitmentTreeRootHistorical(inner) => {
                    <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isLogicRefDenied(inner) => {
                    <isLogicRefDeniedCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::migrateCommitmentTree(inner) => {
                    <migrateCommitmentTreeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrateNullifierSet(inner) => {
                    <migrateNullifierSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::riscZeroVerifierPaused(inner) => {
                    <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setKindTreeRoot(inner) => {
                    <setKindTreeRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::EMPTY_KIND_TREE_ROOT(inner) => {
                    <EMPTY_KIND_TREE_ROOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::denyLogicRefs(inner) => {
                    <denyLogicRefsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getKindTreeRoot(inner) => {
                    <getKindTreeRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLegacyProtocolAdapter(inner) => {
                    <getLegacyProtocolAdapterCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isCommitmentTreeRootHistorical(inner) => {
                    <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isLogicRefDenied(inner) => {
                    <isLogicRefDeniedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::migrateCommitmentTree(inner) => {
                    <migrateCommitmentTreeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrateNullifierSet(inner) => {
                    <migrateNullifierSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::riscZeroVerifierPaused(inner) => {
                    <riscZeroVerifierPausedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setKindTreeRoot(inner) => {
                    <setKindTreeRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`MigrationalProtocolAdapter`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum MigrationalProtocolAdapterErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        AfterTagNotInTransaction(AfterTagNotInTransaction),
        #[allow(missing_docs)]
        BeforeTagNotInTransaction(BeforeTagNotInTransaction),
        #[allow(missing_docs)]
        CommitmentCountMismatch(CommitmentCountMismatch),
        #[allow(missing_docs)]
        CommitmentTreeNotEmpty(CommitmentTreeNotEmpty),
        #[allow(missing_docs)]
        CommitmentTreeRootMismatch(CommitmentTreeRootMismatch),
        #[allow(missing_docs)]
        CyclicConstraints(CyclicConstraints),
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
        EmptyCommitmentTreeNotAllowed(EmptyCommitmentTreeNotAllowed),
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
        LeafCountExceedsCapacity(LeafCountExceedsCapacity),
        #[allow(missing_docs)]
        LegacyProtocolAdapterNotStopped(LegacyProtocolAdapterNotStopped),
        #[allow(missing_docs)]
        LogicRefAlreadyDenied(LogicRefAlreadyDenied),
        #[allow(missing_docs)]
        MissingHistoricalRoot(MissingHistoricalRoot),
        #[allow(missing_docs)]
        MissingNullifier(MissingNullifier),
        #[allow(missing_docs)]
        NonExistingRoot(NonExistingRoot),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NullifierBatchLeavesGap(NullifierBatchLeavesGap),
        #[allow(missing_docs)]
        NullifierBatchOutOfRange(NullifierBatchOutOfRange),
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
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnacceptedKindTreeRoot(UnacceptedKindTreeRoot),
        #[allow(missing_docs)]
        ZeroKindTreeRootNotAllowed(ZeroKindTreeRootNotAllowed),
        #[allow(missing_docs)]
        ZeroLegacyProtocolAdapterNotAllowed(ZeroLegacyProtocolAdapterNotAllowed),
        #[allow(missing_docs)]
        ZeroLogicRefNotAllowed(ZeroLogicRefNotAllowed),
        #[allow(missing_docs)]
        ZeroRiscZeroVerifierRouterNotAllowed(ZeroRiscZeroVerifierRouterNotAllowed),
        #[allow(missing_docs)]
        ZeroRiscZeroVerifierSelectorNotAllowed(ZeroRiscZeroVerifierSelectorNotAllowed),
    }
    impl MigrationalProtocolAdapterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 26u8, 210u8, 140u8],
            [11u8, 247u8, 113u8, 140u8],
            [17u8, 140u8, 218u8, 167u8],
            [18u8, 125u8, 51u8, 94u8],
            [30u8, 79u8, 189u8, 247u8],
            [33u8, 27u8, 18u8, 183u8],
            [37u8, 174u8, 110u8, 170u8],
            [51u8, 71u8, 77u8, 8u8],
            [57u8, 169u8, 64u8, 197u8],
            [62u8, 229u8, 174u8, 181u8],
            [69u8, 189u8, 73u8, 96u8],
            [76u8, 156u8, 140u8, 227u8],
            [76u8, 212u8, 26u8, 216u8],
            [89u8, 122u8, 189u8, 32u8],
            [102u8, 253u8, 163u8, 111u8],
            [107u8, 230u8, 150u8, 163u8],
            [109u8, 252u8, 198u8, 80u8],
            [113u8, 107u8, 51u8, 179u8],
            [120u8, 162u8, 34u8, 28u8],
            [121u8, 147u8, 10u8, 54u8],
            [122u8, 246u8, 35u8, 65u8],
            [141u8, 252u8, 32u8, 43u8],
            [153u8, 150u8, 179u8, 21u8],
            [166u8, 216u8, 42u8, 2u8],
            [168u8, 18u8, 144u8, 149u8],
            [170u8, 29u8, 73u8, 164u8],
            [177u8, 178u8, 90u8, 175u8],
            [179u8, 152u8, 151u8, 159u8],
            [184u8, 160u8, 232u8, 161u8],
            [197u8, 4u8, 250u8, 218u8],
            [198u8, 92u8, 113u8, 112u8],
            [201u8, 62u8, 225u8, 99u8],
            [207u8, 27u8, 9u8, 60u8],
            [213u8, 223u8, 109u8, 214u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [219u8, 120u8, 140u8, 43u8],
            [219u8, 193u8, 170u8, 30u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 212u8, 75u8, 76u8],
            [244u8, 239u8, 23u8, 45u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [249u8, 132u8, 158u8, 163u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ZeroLegacyProtocolAdapterNotAllowed),
            ::core::stringify!(ZeroKindTreeRootNotAllowed),
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(LogicRefAlreadyDenied),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(AfterTagNotInTransaction),
            ::core::stringify!(RiscZeroVerifierPaused),
            ::core::stringify!(CommitmentCountMismatch),
            ::core::stringify!(PreExistingNullifier),
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(BeforeTagNotInTransaction),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(EmptyCommitmentTreeNotAllowed),
            ::core::stringify!(ResourceWithDeniedLogicRef),
            ::core::stringify!(CommitmentTreeNotEmpty),
            ::core::stringify!(LeafCountExceedsCapacity),
            ::core::stringify!(SafeCastOverflowedUintDowncast),
            ::core::stringify!(NullifierBatchOutOfRange),
            ::core::stringify!(RiscZeroVerifierSelectorMismatch),
            ::core::stringify!(MissingHistoricalRoot),
            ::core::stringify!(ZeroLogicRefNotAllowed),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(ZeroRiscZeroVerifierRouterNotAllowed),
            ::core::stringify!(CyclicConstraints),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ZeroRiscZeroVerifierSelectorNotAllowed),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(PointNotOnCurve),
            ::core::stringify!(ForwarderCallOutputMismatch),
            ::core::stringify!(NullifierBatchLeavesGap),
            ::core::stringify!(UnacceptedKindTreeRoot),
            ::core::stringify!(EmptyTransactionNotAllowed),
            ::core::stringify!(CommitmentTreeRootMismatch),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(PreExistingRoot),
            ::core::stringify!(MissingNullifier),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(DeltaMismatch),
            ::core::stringify!(LegacyProtocolAdapterNotStopped),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(NonExistingRoot),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ZeroLegacyProtocolAdapterNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <LogicRefAlreadyDenied as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <AfterTagNotInTransaction as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierPaused as alloy_sol_types::SolError>::SIGNATURE,
            <CommitmentCountMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingNullifier as alloy_sol_types::SolError>::SIGNATURE,
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <BeforeTagNotInTransaction as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyCommitmentTreeNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::SIGNATURE,
            <CommitmentTreeNotEmpty as alloy_sol_types::SolError>::SIGNATURE,
            <LeafCountExceedsCapacity as alloy_sol_types::SolError>::SIGNATURE,
            <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SIGNATURE,
            <NullifierBatchOutOfRange as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <MissingHistoricalRoot as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <CyclicConstraints as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <PointNotOnCurve as alloy_sol_types::SolError>::SIGNATURE,
            <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <NullifierBatchLeavesGap as alloy_sol_types::SolError>::SIGNATURE,
            <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <CommitmentTreeRootMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingRoot as alloy_sol_types::SolError>::SIGNATURE,
            <MissingNullifier as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <DeltaMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <LegacyProtocolAdapterNotStopped as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for MigrationalProtocolAdapterErrors {
        const NAME: &'static str = "MigrationalProtocolAdapterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 47usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AfterTagNotInTransaction(_) => {
                    <AfterTagNotInTransaction as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BeforeTagNotInTransaction(_) => {
                    <BeforeTagNotInTransaction as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CommitmentCountMismatch(_) => {
                    <CommitmentCountMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CommitmentTreeNotEmpty(_) => {
                    <CommitmentTreeNotEmpty as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CommitmentTreeRootMismatch(_) => {
                    <CommitmentTreeRootMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CyclicConstraints(_) => {
                    <CyclicConstraints as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyCommitmentTreeNotAllowed(_) => {
                    <EmptyCommitmentTreeNotAllowed as alloy_sol_types::SolError>::SELECTOR
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
                Self::LeafCountExceedsCapacity(_) => {
                    <LeafCountExceedsCapacity as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LegacyProtocolAdapterNotStopped(_) => {
                    <LegacyProtocolAdapterNotStopped as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LogicRefAlreadyDenied(_) => {
                    <LogicRefAlreadyDenied as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MissingHistoricalRoot(_) => {
                    <MissingHistoricalRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MissingNullifier(_) => {
                    <MissingNullifier as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonExistingRoot(_) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NullifierBatchLeavesGap(_) => {
                    <NullifierBatchLeavesGap as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NullifierBatchOutOfRange(_) => {
                    <NullifierBatchOutOfRange as alloy_sol_types::SolError>::SELECTOR
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
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnacceptedKindTreeRoot(_) => {
                    <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroKindTreeRootNotAllowed(_) => {
                    <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroLegacyProtocolAdapterNotAllowed(_) => {
                    <ZeroLegacyProtocolAdapterNotAllowed as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors>] = &[
                {
                    fn ZeroLegacyProtocolAdapterNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ZeroLegacyProtocolAdapterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ZeroLegacyProtocolAdapterNotAllowed,
                            )
                    }
                    ZeroLegacyProtocolAdapterNotAllowed
                },
                {
                    fn ZeroKindTreeRootNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ZeroKindTreeRootNotAllowed,
                            )
                    }
                    ZeroKindTreeRootNotAllowed
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::OwnableUnauthorizedAccount,
                            )
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn LogicRefAlreadyDenied(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <LogicRefAlreadyDenied as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::LogicRefAlreadyDenied)
                    }
                    LogicRefAlreadyDenied
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn AfterTagNotInTransaction(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <AfterTagNotInTransaction as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::AfterTagNotInTransaction,
                            )
                    }
                    AfterTagNotInTransaction
                },
                {
                    fn RiscZeroVerifierPaused(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <RiscZeroVerifierPaused as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::RiscZeroVerifierPaused,
                            )
                    }
                    RiscZeroVerifierPaused
                },
                {
                    fn CommitmentCountMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <CommitmentCountMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::CommitmentCountMismatch,
                            )
                    }
                    CommitmentCountMismatch
                },
                {
                    fn PreExistingNullifier(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <PreExistingNullifier as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::PreExistingNullifier)
                    }
                    PreExistingNullifier
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ReentrancyGuardReentrantCall,
                            )
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn BeforeTagNotInTransaction(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <BeforeTagNotInTransaction as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::BeforeTagNotInTransaction,
                            )
                    }
                    BeforeTagNotInTransaction
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ERC1967InvalidImplementation,
                            )
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn EmptyCommitmentTreeNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <EmptyCommitmentTreeNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::EmptyCommitmentTreeNotAllowed,
                            )
                    }
                    EmptyCommitmentTreeNotAllowed
                },
                {
                    fn ResourceWithDeniedLogicRef(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ResourceWithDeniedLogicRef,
                            )
                    }
                    ResourceWithDeniedLogicRef
                },
                {
                    fn CommitmentTreeNotEmpty(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <CommitmentTreeNotEmpty as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::CommitmentTreeNotEmpty,
                            )
                    }
                    CommitmentTreeNotEmpty
                },
                {
                    fn LeafCountExceedsCapacity(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <LeafCountExceedsCapacity as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::LeafCountExceedsCapacity,
                            )
                    }
                    LeafCountExceedsCapacity
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::SafeCastOverflowedUintDowncast,
                            )
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn NullifierBatchOutOfRange(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <NullifierBatchOutOfRange as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::NullifierBatchOutOfRange,
                            )
                    }
                    NullifierBatchOutOfRange
                },
                {
                    fn RiscZeroVerifierSelectorMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::RiscZeroVerifierSelectorMismatch,
                            )
                    }
                    RiscZeroVerifierSelectorMismatch
                },
                {
                    fn MissingHistoricalRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <MissingHistoricalRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::MissingHistoricalRoot)
                    }
                    MissingHistoricalRoot
                },
                {
                    fn ZeroLogicRefNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ZeroLogicRefNotAllowed,
                            )
                    }
                    ZeroLogicRefNotAllowed
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn ZeroRiscZeroVerifierRouterNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ZeroRiscZeroVerifierRouterNotAllowed,
                            )
                    }
                    ZeroRiscZeroVerifierRouterNotAllowed
                },
                {
                    fn CyclicConstraints(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <CyclicConstraints as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::CyclicConstraints)
                    }
                    CyclicConstraints
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::UUPSUnsupportedProxiableUUID,
                            )
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ZeroRiscZeroVerifierSelectorNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ZeroRiscZeroVerifierSelectorNotAllowed,
                            )
                    }
                    ZeroRiscZeroVerifierSelectorNotAllowed
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn PointNotOnCurve(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <PointNotOnCurve as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::PointNotOnCurve)
                    }
                    PointNotOnCurve
                },
                {
                    fn ForwarderCallOutputMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ForwarderCallOutputMismatch,
                            )
                    }
                    ForwarderCallOutputMismatch
                },
                {
                    fn NullifierBatchLeavesGap(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <NullifierBatchLeavesGap as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::NullifierBatchLeavesGap,
                            )
                    }
                    NullifierBatchLeavesGap
                },
                {
                    fn UnacceptedKindTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::UnacceptedKindTreeRoot,
                            )
                    }
                    UnacceptedKindTreeRoot
                },
                {
                    fn EmptyTransactionNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::EmptyTransactionNotAllowed,
                            )
                    }
                    EmptyTransactionNotAllowed
                },
                {
                    fn CommitmentTreeRootMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <CommitmentTreeRootMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::CommitmentTreeRootMismatch,
                            )
                    }
                    CommitmentTreeRootMismatch
                },
                {
                    fn FailedCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ECDSAInvalidSignatureS,
                            )
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn PreExistingRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <PreExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::PreExistingRoot)
                    }
                    PreExistingRoot
                },
                {
                    fn MissingNullifier(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <MissingNullifier as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::MissingNullifier)
                    }
                    MissingNullifier
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::UUPSUnauthorizedCallContext,
                            )
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn DeltaMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <DeltaMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::DeltaMismatch)
                    }
                    DeltaMismatch
                },
                {
                    fn LegacyProtocolAdapterNotStopped(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <LegacyProtocolAdapterNotStopped as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::LegacyProtocolAdapterNotStopped,
                            )
                    }
                    LegacyProtocolAdapterNotStopped
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn NonExistingRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <NonExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(MigrationalProtocolAdapterErrors::NonExistingRoot)
                    }
                    NonExistingRoot
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<MigrationalProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                MigrationalProtocolAdapterErrors::ECDSAInvalidSignatureLength,
                            )
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
                Self::AfterTagNotInTransaction(inner) => {
                    <AfterTagNotInTransaction as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BeforeTagNotInTransaction(inner) => {
                    <BeforeTagNotInTransaction as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CommitmentCountMismatch(inner) => {
                    <CommitmentCountMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CommitmentTreeNotEmpty(inner) => {
                    <CommitmentTreeNotEmpty as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CommitmentTreeRootMismatch(inner) => {
                    <CommitmentTreeRootMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CyclicConstraints(inner) => {
                    <CyclicConstraints as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyCommitmentTreeNotAllowed(inner) => {
                    <EmptyCommitmentTreeNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::LeafCountExceedsCapacity(inner) => {
                    <LeafCountExceedsCapacity as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LegacyProtocolAdapterNotStopped(inner) => {
                    <LegacyProtocolAdapterNotStopped as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LogicRefAlreadyDenied(inner) => {
                    <LogicRefAlreadyDenied as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MissingHistoricalRoot(inner) => {
                    <MissingHistoricalRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MissingNullifier(inner) => {
                    <MissingNullifier as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NullifierBatchLeavesGap(inner) => {
                    <NullifierBatchLeavesGap as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NullifierBatchOutOfRange(inner) => {
                    <NullifierBatchOutOfRange as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::UnacceptedKindTreeRoot(inner) => {
                    <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroKindTreeRootNotAllowed(inner) => {
                    <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroLegacyProtocolAdapterNotAllowed(inner) => {
                    <ZeroLegacyProtocolAdapterNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::AfterTagNotInTransaction(inner) => {
                    <AfterTagNotInTransaction as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BeforeTagNotInTransaction(inner) => {
                    <BeforeTagNotInTransaction as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CommitmentCountMismatch(inner) => {
                    <CommitmentCountMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CommitmentTreeNotEmpty(inner) => {
                    <CommitmentTreeNotEmpty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CommitmentTreeRootMismatch(inner) => {
                    <CommitmentTreeRootMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CyclicConstraints(inner) => {
                    <CyclicConstraints as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyCommitmentTreeNotAllowed(inner) => {
                    <EmptyCommitmentTreeNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::LeafCountExceedsCapacity(inner) => {
                    <LeafCountExceedsCapacity as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LegacyProtocolAdapterNotStopped(inner) => {
                    <LegacyProtocolAdapterNotStopped as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LogicRefAlreadyDenied(inner) => {
                    <LogicRefAlreadyDenied as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MissingHistoricalRoot(inner) => {
                    <MissingHistoricalRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MissingNullifier(inner) => {
                    <MissingNullifier as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NullifierBatchLeavesGap(inner) => {
                    <NullifierBatchLeavesGap as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NullifierBatchOutOfRange(inner) => {
                    <NullifierBatchOutOfRange as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::UnacceptedKindTreeRoot(inner) => {
                    <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroKindTreeRootNotAllowed(inner) => {
                    <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroLegacyProtocolAdapterNotAllowed(inner) => {
                    <ZeroLegacyProtocolAdapterNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
    impl MigrationalProtocolAdapterErrors {
        /**Creates a [`AddressEmptyCode`] error.

```solidity
error AddressEmptyCode(address)
```*/
        #[inline]
        pub fn address_empty_code(target: alloy::sol_types::private::Address) -> Self {
            Self::AddressEmptyCode(AddressEmptyCode { target: target })
        }
        /**Creates a [`AfterTagNotInTransaction`] error.

```solidity
error AfterTagNotInTransaction(bytes32)
```*/
        #[inline]
        pub fn after_tag_not_in_transaction(
            tag: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::AfterTagNotInTransaction(AfterTagNotInTransaction {
                tag: tag,
            })
        }
        /**Creates a [`BeforeTagNotInTransaction`] error.

```solidity
error BeforeTagNotInTransaction(bytes32)
```*/
        #[inline]
        pub fn before_tag_not_in_transaction(
            tag: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::BeforeTagNotInTransaction(BeforeTagNotInTransaction {
                tag: tag,
            })
        }
        /**Creates a [`CommitmentCountMismatch`] error.

```solidity
error CommitmentCountMismatch(uint256,uint256)
```*/
        #[inline]
        pub fn commitment_count_mismatch(
            expected: alloy::sol_types::private::primitives::aliases::U256,
            actual: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::CommitmentCountMismatch(CommitmentCountMismatch {
                expected: expected,
                actual: actual,
            })
        }
        /**Creates a [`CommitmentTreeNotEmpty`] error.

```solidity
error CommitmentTreeNotEmpty()
```*/
        #[inline]
        pub fn commitment_tree_not_empty() -> Self {
            Self::CommitmentTreeNotEmpty(CommitmentTreeNotEmpty)
        }
        /**Creates a [`CommitmentTreeRootMismatch`] error.

```solidity
error CommitmentTreeRootMismatch(bytes32,bytes32)
```*/
        #[inline]
        pub fn commitment_tree_root_mismatch(
            expected: alloy::sol_types::private::FixedBytes<32>,
            actual: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::CommitmentTreeRootMismatch(CommitmentTreeRootMismatch {
                expected: expected,
                actual: actual,
            })
        }
        /**Creates a [`CyclicConstraints`] error.

```solidity
error CyclicConstraints()
```*/
        #[inline]
        pub fn cyclic_constraints() -> Self {
            Self::CyclicConstraints(CyclicConstraints)
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
        /**Creates a [`EmptyCommitmentTreeNotAllowed`] error.

```solidity
error EmptyCommitmentTreeNotAllowed()
```*/
        #[inline]
        pub fn empty_commitment_tree_not_allowed() -> Self {
            Self::EmptyCommitmentTreeNotAllowed(EmptyCommitmentTreeNotAllowed)
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
        /**Creates a [`LeafCountExceedsCapacity`] error.

```solidity
error LeafCountExceedsCapacity(uint256,uint256)
```*/
        #[inline]
        pub fn leaf_count_exceeds_capacity(
            leaf_count: alloy::sol_types::private::primitives::aliases::U256,
            capacity: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::LeafCountExceedsCapacity(LeafCountExceedsCapacity {
                leafCount: leaf_count,
                capacity: capacity,
            })
        }
        /**Creates a [`LegacyProtocolAdapterNotStopped`] error.

```solidity
error LegacyProtocolAdapterNotStopped(address)
```*/
        #[inline]
        pub fn legacy_protocol_adapter_not_stopped(
            legacy_protocol_adapter: alloy::sol_types::private::Address,
        ) -> Self {
            Self::LegacyProtocolAdapterNotStopped(LegacyProtocolAdapterNotStopped {
                legacyProtocolAdapter: legacy_protocol_adapter,
            })
        }
        /**Creates a [`LogicRefAlreadyDenied`] error.

```solidity
error LogicRefAlreadyDenied(bytes32,bool)
```*/
        #[inline]
        pub fn logic_ref_already_denied(
            logic_ref: alloy::sol_types::private::FixedBytes<32>,
            consumed: bool,
        ) -> Self {
            Self::LogicRefAlreadyDenied(LogicRefAlreadyDenied {
                logicRef: logic_ref,
                consumed: consumed,
            })
        }
        /**Creates a [`MissingHistoricalRoot`] error.

```solidity
error MissingHistoricalRoot(bytes32)
```*/
        #[inline]
        pub fn missing_historical_root(
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::MissingHistoricalRoot(MissingHistoricalRoot {
                root: root,
            })
        }
        /**Creates a [`MissingNullifier`] error.

```solidity
error MissingNullifier(bytes32)
```*/
        #[inline]
        pub fn missing_nullifier(
            nullifier: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::MissingNullifier(MissingNullifier {
                nullifier: nullifier,
            })
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
        /**Creates a [`NullifierBatchLeavesGap`] error.

```solidity
error NullifierBatchLeavesGap(uint256)
```*/
        #[inline]
        pub fn nullifier_batch_leaves_gap(
            start: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::NullifierBatchLeavesGap(NullifierBatchLeavesGap {
                start: start,
            })
        }
        /**Creates a [`NullifierBatchOutOfRange`] error.

```solidity
error NullifierBatchOutOfRange(uint256,uint256)
```*/
        #[inline]
        pub fn nullifier_batch_out_of_range(
            end: alloy::sol_types::private::primitives::aliases::U256,
            nullifier_count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::NullifierBatchOutOfRange(NullifierBatchOutOfRange {
                end: end,
                nullifierCount: nullifier_count,
            })
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
        /**Creates a [`SafeCastOverflowedUintDowncast`] error.

```solidity
error SafeCastOverflowedUintDowncast(uint8,uint256)
```*/
        #[inline]
        pub fn safe_cast_overflowed_uint_downcast(
            bits: u8,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast {
                bits: bits,
                value: value,
            })
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
        /**Creates a [`UnacceptedKindTreeRoot`] error.

```solidity
error UnacceptedKindTreeRoot(bytes32)
```*/
        #[inline]
        pub fn unaccepted_kind_tree_root(
            kind_tree_root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::UnacceptedKindTreeRoot(UnacceptedKindTreeRoot {
                kindTreeRoot: kind_tree_root,
            })
        }
        /**Creates a [`ZeroKindTreeRootNotAllowed`] error.

```solidity
error ZeroKindTreeRootNotAllowed()
```*/
        #[inline]
        pub fn zero_kind_tree_root_not_allowed() -> Self {
            Self::ZeroKindTreeRootNotAllowed(ZeroKindTreeRootNotAllowed)
        }
        /**Creates a [`ZeroLegacyProtocolAdapterNotAllowed`] error.

```solidity
error ZeroLegacyProtocolAdapterNotAllowed()
```*/
        #[inline]
        pub fn zero_legacy_protocol_adapter_not_allowed() -> Self {
            Self::ZeroLegacyProtocolAdapterNotAllowed(
                ZeroLegacyProtocolAdapterNotAllowed,
            )
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
    ///Container for all the [`MigrationalProtocolAdapter`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum MigrationalProtocolAdapterEvents {
        #[allow(missing_docs)]
        ActionExecuted(ActionExecuted),
        #[allow(missing_docs)]
        ApplicationPayload(ApplicationPayload),
        #[allow(missing_docs)]
        CommitmentTreeMigrated(CommitmentTreeMigrated),
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
        KindTreeRootUpdated(KindTreeRootUpdated),
        #[allow(missing_docs)]
        LogicRefDenied(LogicRefDenied),
        #[allow(missing_docs)]
        MigrationFinalized(MigrationFinalized),
        #[allow(missing_docs)]
        NullifierBatchMigrated(NullifierBatchMigrated),
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
    impl MigrationalProtocolAdapterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                4u8, 24u8, 71u8, 156u8, 166u8, 172u8, 141u8, 191u8, 131u8, 98u8, 106u8,
                94u8, 60u8, 220u8, 89u8, 21u8, 35u8, 63u8, 26u8, 117u8, 112u8, 201u8,
                53u8, 17u8, 93u8, 176u8, 172u8, 245u8, 51u8, 182u8, 73u8, 191u8,
            ],
            [
                10u8, 45u8, 197u8, 72u8, 237u8, 149u8, 10u8, 204u8, 180u8, 13u8, 93u8,
                120u8, 84u8, 31u8, 57u8, 84u8, 197u8, 225u8, 130u8, 168u8, 236u8, 241u8,
                158u8, 88u8, 26u8, 79u8, 34u8, 99u8, 246u8, 31u8, 89u8, 210u8,
            ],
            [
                16u8, 99u8, 93u8, 76u8, 95u8, 1u8, 141u8, 159u8, 78u8, 123u8, 152u8,
                227u8, 97u8, 41u8, 25u8, 81u8, 98u8, 216u8, 53u8, 67u8, 238u8, 26u8,
                158u8, 47u8, 209u8, 220u8, 205u8, 27u8, 37u8, 35u8, 242u8, 226u8,
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
                99u8, 177u8, 135u8, 103u8, 61u8, 69u8, 14u8, 215u8, 20u8, 84u8, 149u8,
                71u8, 122u8, 164u8, 119u8, 20u8, 69u8, 14u8, 177u8, 125u8, 132u8, 178u8,
                145u8, 216u8, 9u8, 218u8, 242u8, 157u8, 168u8, 9u8, 103u8, 88u8,
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
                156u8, 97u8, 178u8, 144u8, 246u8, 49u8, 9u8, 127u8, 86u8, 39u8, 60u8,
                244u8, 218u8, 244u8, 13u8, 241u8, 255u8, 156u8, 204u8, 51u8, 241u8, 1u8,
                212u8, 100u8, 131u8, 125u8, 161u8, 245u8, 174u8, 24u8, 189u8, 89u8,
            ],
            [
                156u8, 252u8, 21u8, 28u8, 208u8, 200u8, 196u8, 153u8, 22u8, 225u8, 94u8,
                154u8, 241u8, 92u8, 93u8, 85u8, 63u8, 248u8, 92u8, 193u8, 61u8, 150u8,
                254u8, 249u8, 67u8, 64u8, 101u8, 95u8, 214u8, 121u8, 78u8, 159u8,
            ],
            [
                164u8, 148u8, 218u8, 196u8, 183u8, 24u8, 72u8, 67u8, 88u8, 63u8, 151u8,
                46u8, 6u8, 120u8, 62u8, 44u8, 59u8, 180u8, 127u8, 79u8, 1u8, 55u8, 184u8,
                223u8, 82u8, 168u8, 96u8, 223u8, 7u8, 33u8, 159u8, 140u8,
            ],
            [
                167u8, 15u8, 68u8, 51u8, 97u8, 94u8, 8u8, 58u8, 223u8, 27u8, 151u8,
                224u8, 195u8, 45u8, 154u8, 114u8, 28u8, 57u8, 194u8, 109u8, 109u8, 214u8,
                194u8, 143u8, 233u8, 97u8, 118u8, 171u8, 144u8, 159u8, 14u8, 202u8,
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
            ::core::stringify!(MigrationFinalized),
            ::core::stringify!(CommitmentTreeRootAdded),
            ::core::stringify!(KindTreeRootUpdated),
            ::core::stringify!(ResourcePayload),
            ::core::stringify!(DiscoveryPayload),
            ::core::stringify!(ActionExecuted),
            ::core::stringify!(Unpaused),
            ::core::stringify!(Paused),
            ::core::stringify!(NullifierBatchMigrated),
            ::core::stringify!(TransactionExecuted),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(ExternalPayload),
            ::core::stringify!(CommitmentTreeMigrated),
            ::core::stringify!(ApplicationPayload),
            ::core::stringify!(LogicRefDenied),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
            ::core::stringify!(ForwarderCallExecuted),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <MigrationFinalized as alloy_sol_types::SolEvent>::SIGNATURE,
            <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <KindTreeRootUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <NullifierBatchMigrated as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <CommitmentTreeMigrated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ApplicationPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <LogicRefDenied as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for MigrationalProtocolAdapterEvents {
        const NAME: &'static str = "MigrationalProtocolAdapterEvents";
        const COUNT: usize = 18usize;
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
                    <CommitmentTreeMigrated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CommitmentTreeMigrated as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::CommitmentTreeMigrated)
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
                    <KindTreeRootUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KindTreeRootUpdated as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::KindTreeRootUpdated)
                }
                Some(<LogicRefDenied as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LogicRefDenied as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::LogicRefDenied)
                }
                Some(
                    <MigrationFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MigrationFinalized as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::MigrationFinalized)
                }
                Some(
                    <NullifierBatchMigrated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NullifierBatchMigrated as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::NullifierBatchMigrated)
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
    impl alloy_sol_types::private::IntoLogData for MigrationalProtocolAdapterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ApplicationPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CommitmentTreeMigrated(inner) => {
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
                Self::KindTreeRootUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LogicRefDenied(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MigrationFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NullifierBatchMigrated(inner) => {
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
                Self::CommitmentTreeMigrated(inner) => {
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
                Self::KindTreeRootUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LogicRefDenied(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MigrationFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NullifierBatchMigrated(inner) => {
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
    impl MigrationalProtocolAdapterEvents {
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
        /**Creates a [`CommitmentTreeMigrated`] event.

```solidity
event CommitmentTreeMigrated(bytes32,uint256)
```*/
        #[inline]
        pub fn commitment_tree_migrated(
            root: alloy::sol_types::private::FixedBytes<32>,
            leaf_count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::CommitmentTreeMigrated(CommitmentTreeMigrated {
                root: root,
                leafCount: leaf_count,
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
        /**Creates a [`KindTreeRootUpdated`] event.

```solidity
event KindTreeRootUpdated(bytes32)
```*/
        #[inline]
        pub fn kind_tree_root_updated(
            kind_tree_root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::KindTreeRootUpdated(KindTreeRootUpdated {
                kindTreeRoot: kind_tree_root,
            })
        }
        /**Creates a [`LogicRefDenied`] event.

```solidity
event LogicRefDenied(bytes32,bool)
```*/
        #[inline]
        pub fn logic_ref_denied(
            logic_ref: alloy::sol_types::private::FixedBytes<32>,
            consumed: bool,
        ) -> Self {
            Self::LogicRefDenied(LogicRefDenied {
                logicRef: logic_ref,
                consumed: consumed,
            })
        }
        /**Creates a [`MigrationFinalized`] event.

```solidity
event MigrationFinalized()
```*/
        #[inline]
        pub fn migration_finalized() -> Self {
            Self::MigrationFinalized(MigrationFinalized)
        }
        /**Creates a [`NullifierBatchMigrated`] event.

```solidity
event NullifierBatchMigrated(uint256,uint256)
```*/
        #[inline]
        pub fn nullifier_batch_migrated(
            start: alloy::sol_types::private::primitives::aliases::U256,
            count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::NullifierBatchMigrated(NullifierBatchMigrated {
                start: start,
                count: count,
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
    /**Creates a new wrapper around an on-chain [`MigrationalProtocolAdapter`](self) contract instance.

See the [wrapper's documentation](`MigrationalProtocolAdapterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> MigrationalProtocolAdapterInstance<P, N> {
        MigrationalProtocolAdapterInstance::<P, N>::new(address, __provider)
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
        legacyProtocolAdapter: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MigrationalProtocolAdapterInstance<P, N>>,
    > {
        MigrationalProtocolAdapterInstance::<
            P,
            N,
        >::deploy(
            __provider,
            riscZeroVerifierRouter,
            riscZeroVerifierSelector,
            legacyProtocolAdapter,
        )
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
        legacyProtocolAdapter: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        MigrationalProtocolAdapterInstance::<
            P,
            N,
        >::deploy_builder(
            __provider,
            riscZeroVerifierRouter,
            riscZeroVerifierSelector,
            legacyProtocolAdapter,
        )
    }
    /**A [`MigrationalProtocolAdapter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MigrationalProtocolAdapter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MigrationalProtocolAdapterInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for MigrationalProtocolAdapterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MigrationalProtocolAdapterInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MigrationalProtocolAdapterInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`MigrationalProtocolAdapter`](self) contract instance.

See the [wrapper's documentation](`MigrationalProtocolAdapterInstance`) for more details.*/
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
            legacyProtocolAdapter: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MigrationalProtocolAdapterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                riscZeroVerifierRouter,
                riscZeroVerifierSelector,
                legacyProtocolAdapter,
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
            legacyProtocolAdapter: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            riscZeroVerifierRouter,
                            riscZeroVerifierSelector,
                            legacyProtocolAdapter,
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
    impl<P: ::core::clone::Clone, N> MigrationalProtocolAdapterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MigrationalProtocolAdapterInstance<P, N> {
            MigrationalProtocolAdapterInstance {
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
    > MigrationalProtocolAdapterInstance<P, N> {
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
        ///Creates a new call builder for the [`EMPTY_KIND_TREE_ROOT`] function.
        pub fn EMPTY_KIND_TREE_ROOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, EMPTY_KIND_TREE_ROOTCall, N> {
            self.call_builder(&EMPTY_KIND_TREE_ROOTCall)
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
        ///Creates a new call builder for the [`denyLogicRefs`] function.
        pub fn denyLogicRefs(
            &self,
            logicRefs: alloy::sol_types::private::Vec<
                <ILogicRefDenylist::DeniedLogicRef as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, denyLogicRefsCall, N> {
            self.call_builder(&denyLogicRefsCall { logicRefs })
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
        ///Creates a new call builder for the [`getKindTreeRoot`] function.
        pub fn getKindTreeRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKindTreeRootCall, N> {
            self.call_builder(&getKindTreeRootCall)
        }
        ///Creates a new call builder for the [`getLegacyProtocolAdapter`] function.
        pub fn getLegacyProtocolAdapter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getLegacyProtocolAdapterCall, N> {
            self.call_builder(&getLegacyProtocolAdapterCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { initialOwner })
        }
        ///Creates a new call builder for the [`isCommitmentTreeRootHistorical`] function.
        pub fn isCommitmentTreeRootHistorical(
            &self,
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isCommitmentTreeRootHistoricalCall, N> {
            self.call_builder(
                &isCommitmentTreeRootHistoricalCall {
                    root,
                },
            )
        }
        ///Creates a new call builder for the [`isLogicRefDenied`] function.
        pub fn isLogicRefDenied(
            &self,
            logicRef: alloy::sol_types::private::FixedBytes<32>,
            consumed: bool,
        ) -> alloy_contract::SolCallBuilder<&P, isLogicRefDeniedCall, N> {
            self.call_builder(
                &isLogicRefDeniedCall {
                    logicRef,
                    consumed,
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
        ///Creates a new call builder for the [`migrateCommitmentTree`] function.
        pub fn migrateCommitmentTree(
            &self,
            sides: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, migrateCommitmentTreeCall, N> {
            self.call_builder(&migrateCommitmentTreeCall { sides })
        }
        ///Creates a new call builder for the [`migrateNullifierSet`] function.
        pub fn migrateNullifierSet(
            &self,
            start: alloy::sol_types::private::primitives::aliases::U256,
            count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, migrateNullifierSetCall, N> {
            self.call_builder(
                &migrateNullifierSetCall {
                    start,
                    count,
                },
            )
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
        ///Creates a new call builder for the [`riscZeroVerifierPaused`] function.
        pub fn riscZeroVerifierPaused(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, riscZeroVerifierPausedCall, N> {
            self.call_builder(&riscZeroVerifierPausedCall)
        }
        ///Creates a new call builder for the [`setKindTreeRoot`] function.
        pub fn setKindTreeRoot(
            &self,
            newKindTreeRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, setKindTreeRootCall, N> {
            self.call_builder(
                &setKindTreeRootCall {
                    newKindTreeRoot,
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
    > MigrationalProtocolAdapterInstance<P, N> {
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
        ///Creates a new event filter for the [`CommitmentTreeMigrated`] event.
        pub fn CommitmentTreeMigrated_filter(
            &self,
        ) -> alloy_contract::Event<&P, CommitmentTreeMigrated, N> {
            self.event_filter::<CommitmentTreeMigrated>()
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
        ///Creates a new event filter for the [`KindTreeRootUpdated`] event.
        pub fn KindTreeRootUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, KindTreeRootUpdated, N> {
            self.event_filter::<KindTreeRootUpdated>()
        }
        ///Creates a new event filter for the [`LogicRefDenied`] event.
        pub fn LogicRefDenied_filter(
            &self,
        ) -> alloy_contract::Event<&P, LogicRefDenied, N> {
            self.event_filter::<LogicRefDenied>()
        }
        ///Creates a new event filter for the [`MigrationFinalized`] event.
        pub fn MigrationFinalized_filter(
            &self,
        ) -> alloy_contract::Event<&P, MigrationFinalized, N> {
            self.event_filter::<MigrationFinalized>()
        }
        ///Creates a new event filter for the [`NullifierBatchMigrated`] event.
        pub fn NullifierBatchMigrated_filter(
            &self,
        ) -> alloy_contract::Event<&P, NullifierBatchMigrated, N> {
            self.event_filter::<NullifierBatchMigrated>()
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
