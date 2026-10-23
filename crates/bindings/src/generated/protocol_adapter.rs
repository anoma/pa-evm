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

interface ProtocolAdapter {
    error AddressEmptyCode(address target);
    error AfterTagNamesNoCallCarryingResource(bytes32 tag);
    error BeforeTagNamesNoCallCarryingResource(bytes32 tag);
    error CallOrderNotAPermutation();
    error CallOrderNotInActionOrder(uint256 actionIndex);
    error CallOrderViolatesConstraint(bytes32 tag);
    error CyclicOrderingConstraints();
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
    error LogicRefAlreadyDenied(bytes32 logicRef, bool consumed);
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
    error TooManyCallCarryingResources(uint256 count);
    error TooManyNodes(uint256 nodeCount);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnacceptedKindTreeRoot(bytes32 kindTreeRoot);
    error ZeroKindTreeRootNotAllowed();
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
    event KindTreeRootUpdated(bytes32 indexed kindTreeRoot);
    event LogicRefDenied(bytes32 indexed logicRef, bool consumed);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address account);
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
    event TransactionExecuted(bytes32 indexed transactionId);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);

    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector);

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
    function executeWithCallOrder(IProtocolAdapter.Transaction memory transaction, uint256[] memory callOrder) external;
    function getImplementation() external view returns (address current);
    function getKindTreeRoot() external view returns (bytes32 kindTreeRoot);
    function initialize(address initialOwner) external;
    function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
    function isLogicRefDenied(bytes32 logicRef, bool consumed) external view returns (bool isDenied);
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
    function latestCommitmentTreeRoot() external view returns (bytes32 root);
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
    "name": "executeWithCallOrder",
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
      },
      {
        "name": "callOrder",
        "type": "uint256[]",
        "internalType": "uint256[]"
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
    "name": "AfterTagNamesNoCallCarryingResource",
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
    "name": "BeforeTagNamesNoCallCarryingResource",
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
    "name": "CallOrderNotAPermutation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CallOrderNotInActionOrder",
    "inputs": [
      {
        "name": "actionIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CallOrderViolatesConstraint",
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
    "name": "CyclicOrderingConstraints",
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
    "name": "TooManyCallCarryingResources",
    "inputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "TooManyNodes",
    "inputs": [
      {
        "name": "nodeCount",
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
    ///0x60e03461011057601f614cab38819003918201601f19168301916001600160401b038311848410176101145780849260409485528339810103126101105780516001600160a01b038116918282036101105760200151916001600160e01b03198316908184036101105730608052610075610128565b61007d610128565b610085610128565b61008d610128565b1561010157156100f25760a05260c0526100a5610128565b604051614acc90816101bf8239608051818181610f1e0152611076015260a051818181610e3e015281816118460152611e54015260c0518181816103a7015281816118040152611e140152f35b63b1b25aaf60e01b5f5260045ffd5b63536c150160e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f516020614c8b5f395f51905f525460ff8160401c166101af576002600160401b03196001600160401b0382160161015d5750565b6001600160401b0319166001600160401b039081175f516020614c8b5f395f51905f52556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a1565b63f92ee8a960e01b5f5260045ffdfe61010060405260033611610011575f80fd5b5f3560e01c8063155f9f0214611582578063171eac18146115575780631a8473bc146113b55780633f4ba83a146112f85780634f1ef2861461102a57806351945b0614610f9657806352d1902d14610ef75780635c975abb14610eb657806364c12af214610e7c5780636735312214610e625780636bf0a04b14610e12578063715018a614610d565780638456cb5914610cbf5780638b33e20014610c2c5780638da5cb5b14610bda5780639239ac0c14610b52578063a06056f714610b13578063aaf10f4214610ac1578063ad3cb1cc14610a62578063bdeb442d1461090d578063c1b0bed7146108bf578063c44956d114610883578063c4d66de8146103cb578063e35a5d2f1461036f578063ed83cdc7146102c7578063f2fde38b1461029c578063f8adf4c314610248578063fb0ff6801461020c578063fe18ab91146101ca5763ffa1ad7414610163575f80fd5b346101c6575f6003193601126101c6576101c26040516101846040826116a8565b600b81527f322e302e302d72632e3132000000000000000000000000000000000000000000602082015260405191829160208352602083019061176e565b0390f35b5f80fd5b346101c6575f6003193601126101c6576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b346101c6575f6003193601126101c65760207f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb0054604051908152f35b346101c65760206003193601126101c65760206102926004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b6040519015158152f35b346101c65760206003193601126101c6576102c56102b861164d565b6102c0613129565b611993565b005b346101c6575f6003193601126101c6576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f5260205f20905f5b818110610359576101c285610345818703826116a8565b60405191829160208352602083019061173b565b825484526020909301926001928301920161032e565b346101c6575f6003193601126101c65760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101c65760206003193601126101c6576103e461164d565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c16159167ffffffffffffffff82168015908161087b575b6001149081610871575b159081610868575b5061084057818360017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000006104a59516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00556107eb575b506104956132d9565b61049d6132d9565b6102c06132d9565b6104ad6132d9565b6104b56132d9565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156107be5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015610791577fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2001555f7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260405f20600160ff198254161790557f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a161062c6132d9565b6106346132d9565b61063c6132d9565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e25f80a26106cc6117bb565b610769576106d657005b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b7f25ae6eaa000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558361048c565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b90501584610435565b303b15915061042d565b849150610423565b346101c6575f6003193601126101c65760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b346101c65760206003193601126101c6576004355f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00602052602060ff60405f2054166040519015158152f35b346101c6575f6003193601126101c65760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2054905f5b8181106109a257602083604051908152f35b9091600190848216610a0b577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301546109fe91613296565b935b811c93929101610990565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154610a5c9190613296565b93610a00565b346101c6575f6003193601126101c6576101c2604051610a836040826116a8565b600581527f352e302e30000000000000000000000000000000000000000000000000000000602082015260405191829160208352602083019061176e565b346101c6575f6003193601126101c657602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b346101c6575f6003193601126101c657602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b346101c65760206003193601126101c65760043567ffffffffffffffff81116101c657608060031982360301126101c657610bb590610b8f611a80565b610b97611af4565b60405190610ba66020836116a8565b5f82525f368137600401611bef565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d005b346101c6575f6003193601126101c657602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346101c65760206003193601126101c657600435610c48613129565b8015610c9757807f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e25f80a2005b7f0bf7718c000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c6575f6003193601126101c657610cd7613129565b610cdf611af4565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b346101c6575f6003193601126101c657610d6e613129565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346101c6575f6003193601126101c657602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101c6575f6003193601126101c65760206102926117bb565b346101c6575f6003193601126101c65760206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152f35b346101c6575f6003193601126101c657602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b346101c6575f6003193601126101c65773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163003610f6e5760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c6575f6003193601126101c6576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f20905f5b818110611014576101c285610345818703826116a8565b8254845260209093019260019283019201610ffd565b60406003193601126101c65761103e61164d565b60243567ffffffffffffffff81116101c65761105e90369060040161171d565b9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168030149081156112b6575b50610f6e576110ae613129565b73ffffffffffffffffffffffffffffffffffffffff8116916040517f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f9181611282575b5061112e57837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8592036112575750823b1561122c57807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a28051156111fb576102c591613b17565b50503461120457005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d6020116112ae575b8161129e602093836116a8565b810103126101c6575190856110fd565b3d9150611291565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54161415836110a1565b346101c6575f6003193601126101c657611310613129565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff81161561138d5760ff19167fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c65760206003193601126101c65760043567ffffffffffffffff81116101c657366023820112156101c657806004013567ffffffffffffffff81116101c6576024820191602436918360061b0101116101c65790611414613129565b5f915b80831061142057005b61142b8382846117ab565b3592602061143a8284866117ab565b013593841515948581036101c657811561152f5761145881836130e8565b6114ff57602060019495967fa70f4433615e083adf1b97e0c32d9a721c39c26d6dd6c28fe96176ab909f0eca92845f527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900835260405f20905f146114d0578660ff198254161790555b604051908152a2019190611417565b6101007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff8254161790556114c1565b85827f127d335e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7f7af62341000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c65760406003193601126101c65760243580151581036101c6576102926020916004356130e8565b346101c65760406003193601126101c65760043567ffffffffffffffff81116101c657608060031982360301126101c6576024359067ffffffffffffffff82116101c657366023830112156101c65781600401359067ffffffffffffffff82116101c65760248260051b8401013681116101c6576115fe611a80565b611606611af4565b61160f83611793565b9261161d60405194856116a8565b83526024602084019401935b81851061163d57610bb58484600401611bef565b8435815260209485019401611629565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101c657565b6040810190811067ffffffffffffffff8211176107be57604052565b6080810190811067ffffffffffffffff8211176107be57604052565b90601f601f19910116810190811067ffffffffffffffff8211176107be57604052565b67ffffffffffffffff81116107be57601f01601f191660200190565b9291926116f3826116cb565b9161170160405193846116a8565b8294818452818301116101c6578281602093845f960137010152565b9080601f830112156101c657816020611738933591016116e7565b90565b90602080835192838152019201905f5b8181106117585750505090565b825184526020938401939092019160010161174b565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b67ffffffffffffffff81116107be5760051b60200190565b91908110156107915760061b0190565b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611907575f91611912575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa908115611907575f916118cf575090565b90506020813d6020116118ff575b816118ea602093836116a8565b810103126101c6575180151581036101c65790565b3d91506118dd565b6040513d5f823e3d90fd5b90506020813d60201161195a575b8161192d602093836116a8565b810103126101c6575173ffffffffffffffffffffffffffffffffffffffff811681036101c6576020611876565b3d9150611920565b9061196c82611793565b61197960405191826116a8565b828152601f196119898294611793565b0190602036910137565b73ffffffffffffffffffffffffffffffffffffffff168015611a545773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c611acc5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d565b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416611b1f57565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101c6570180359067ffffffffffffffff82116101c657602001918160051b360383136101c657565b91908110156107915760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603018212156101c6570190565b80518210156107915760209160051b010190565b9060a052611bfd8180611b47565b90505f60c05280156130c0577f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00546060830135148015613093575b1561306357611c4681611962565b91611c4f613195565b50604051611c5c81611670565b5f81525f6020820152925f905f6080525f935f905b8082106128115750506020815160051b9101209360a05151155f146127a357611cbf90611cb6865b611cb0611ca96020880188613245565b36916116e7565b90614183565b909391936141bd565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2016911690808203612775575050611cfd8280611b47565b60405190602082019281608084017fbc4edaeea98f6501547cba34cf94d8b59631e7f86cf1a47119ba6f17c29a517c8652606088013560408601526060808601525260a0830160a08360051b85010192825f907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603015b838310612598575050505050509181611d9b5f9360209503601f1981018352826116a8565b604051918291518091835e8101838152039060025afa15611907575f51611dc56040840184613245565b806004116101c6577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000001680820361256a57505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b156101c6575f92611ebf92604051958694859384937fab750e7500000000000000000000000000000000000000000000000000000000855260606004860152606485019161340a565b907f35082a39ecba805a1ca4fdac7eb364b20afce2abf14413ebd910233332ee45c96024840152604483015203915afa801561190757612556575b5080612486575b506080511561244f5780611f1491611b47565b60a0515191929091156121d4575090611f309160805191613ed1565b80518060a051510361219657611f4581611962565b9060c0515b81811061215c575060015b8181106120ac575060c0515b818110611fa257505050611f789060a05190614026565b7f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce60c05160c051a2565b6060611fae8286611bdb565b51015180519060c0515b828110611fca57505050600101611f61565b6060611fd68284611bdb565b5101518051908660c0515b878b8b86841061206d57505050505050506080611ffe8284611bdb565b51015180519060c0515b88888b85841061202057505050505050600101611fb8565b83926120606120679361204f6120469461203c6001998b611bdb565b5195868092614831565b92831415614117565b6120598d84611bdb565b5192611bdb565b51116140e4565b01612008565b6120606120a49361205960019761209e6120959661208b8a8d611bdb565b5197888092614831565b928314156140b1565b84611bdb565b018790611fe1565b60406120c46120bd8360a051611bdb565b5186611bdb565b5101515f19820182811161212b576120ea6120e360409260a051611bdb565b5187611bdb565b51015181106120fc5750600101611f55565b7ff3474dc20000000000000000000000000000000000000000000000000000000060c05152600452602460c051fd5b7f4e487b710000000000000000000000000000000000000000000000000000000060c051526011600452602460c051fd5b6121688160a051611bdb565b5190828210806121c2575b15612196576001810180821161212b5761218f60019386611bdb565b5201611f4a565b7f9a12c5100000000000000000000000000000000000000000000000000000000060c05152600460c051fd5b506121cd8285611bdb565b5115612173565b1561238657906121e79160805191613ed1565b8051610100811161235757506040516121ff81611670565b60608152602081016060815282519061221782611962565b835261222282611962565b905260c0515b81811061227a57505061223a90614614565b901561224e5761224991614026565b611f78565b7f813c76a00000000000000000000000000000000000000000000000000000000060c05152600460c051fd5b60606122868286611bdb565b51015180519060c0515b8281106122a257505050600101612228565b6122ac8183611bdb565b51606081015180519060c0515b87898b8d86851061232657505050505050506080015180519060c0515b87878a8c8685106122f05750505050505050600101612290565b918493916123209361231b6123076001988a611bdb565b516123128184614831565b95861415614117565b6148fd565b016122d6565b918493916123519361231b61233d6001988a611bdb565b516123488184614831565b948514156140b1565b016122b9565b7f895022880000000000000000000000000000000000000000000000000000000060c05152600452602460c051fd5b60c0515b81811061239957505050611f78565b6123ad6123a7828486611b9b565b80611b47565b60c0515b818110612426575050506123d36123c9828486611b9b565b6020810190611b47565b60c0515b8181106123e95750505060010161238a565b8061242060206123fc6001948688613205565b61241661240c60408301836131ad565b6040810190611b47565b9290910135613e34565b016123d7565b80612449602061243960019486886131e0565b61241661240c60608301836131ad565b016123b1565b505060a0515115611f78577f9a12c5100000000000000000000000000000000000000000000000000000000060c05152600460c051fd5b8060c051527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff604060c051205416612527576020817f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d29260c051527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9038252604060c05120600160ff19825416179055604051908152a15f611f01565b7fdb788c2b0000000000000000000000000000000000000000000000000000000060c05152600452602460c051fd5b5f612560916116a8565b5f60c0525f611efa565b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9091929394957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608882030186528635828112156101c65783019060a08101916125e1818061392c565b809460a085525260c0830160c08560051b85010194825f5b8281106126f6575050505050612612602082018261392c565b93908382036020850152848252602082019060208660051b8401019581935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b83811061269857505050506040808501359086015250505060608082013590830152608090810135910152956020908101950193926001019190611d76565b90919293949598601f198482030187528935868112156101c65760206126e760019360606126d988859601803584528581013586850152604081019061397f565b918160408201520190613aa0565b9b019701959493929101612659565b909192939660208061276883608061275a61273a8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f60019a03018d528a61397f565b80358452858101358685015260408101356040850152606081019061397f565b918160608201520190613aa0565b99019501939291016125f9565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60405160208101906060810187835260408083015260a051518091526080820190602060a05101905f5b8181106127fb5750505091816127f3611cb693611cbf969503601f1981018352826116a8565b519020611c99565b82518452602093840193909201916001016127cd565b9095612827876128218780611b47565b90611b9b565b60e0525f5f5f9161283c60e05160e051611b47565b905061284781611962565b9061285181611962565b905f5b818110612e7e57505061286e602060e0510160e051611b47565b91905061287a82611962565b9061288483611962565b925f905b808210612a645750506128e2926128fe7f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc3295936128f061290c94604051978897608060e0510135895260a060208a015260a089019061173b565b90878203604089015261173b565b90858203606087015261173b565b90838203608085015261173b565b0390a180612a5c575b5080612a0a575b505060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060e051360301126101c6576040519061295982611670565b604060e0510135825260208201606060e051013581526040519261297c84611670565b5f84525f60208501526129928151835190613665565b156129d257916129b191600194936020835193015190519151926136c7565b6020830152815296608060e05101356129ca8286611bdb565b520190611c71565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b608051018060805111612a2f57608052612a26575b5f8061291c565b60019550612a1f565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b95505f612915565b909550612a8286612a7c602060e0510160e051611b47565b90613205565b95612a915f60208901356130e8565b612e4a5760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054612ae5816131f7565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90055908835915f5b828110612d665750507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b14612bf3575b509660408101612b7961240c82846131ad565b80612bbc575b505081612b9b612b94602093600196956131ad565b8235613441565b8035612ba78489611bdb565b520135612bb48288611bdb565b520190612888565b909a612bc7906131f7565b9a8c612b7f576020929c50612b94612be66001969593612b9b93613370565b9d93505081939450612b7f565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154680100000000000000008110156107be5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90155811015610791577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f018290557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431015490612cd690612ccf908390613296565b9180613296565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156107be5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015610791577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2001555f612b66565b9092600190818516612df3577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431830154612de891613296565b935b811c9101612b0d565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154612e449190613296565b93612dea565b6020877f597abd20000000000000000000000000000000000000000000000000000000005f5201356004525f60245260445ffd5b612e9681612e9060e05160e051611b47565b906131e0565b612ea5600160208301356130e8565b61302e576040810135612ee3815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b15613003575080355f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f205416612fd75780355f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260405f20600160ff1982541617905560608101612f6561240c82846131ad565b80612fa0575b505081612f80612b94602093600196956131ad565b8035612f8c8489611bdb565b520135612f998286611bdb565b5201612854565b9098612fab906131f7565b988a612f6b576020929a50612b94612fca6001969593612f8093613370565b9b93505081939450612f6b565b7f39a940c5000000000000000000000000000000000000000000000000000000005f523560045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6020907f597abd20000000000000000000000000000000000000000000000000000000005f520135600452600160245260445ffd5b6060827fc93ee163000000000000000000000000000000000000000000000000000000005f52013560045260245ffd5b507fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06606083013514611c38565b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b5f9081527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629006020526040902060ff911561312157541690565b5460081c1690565b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416330361316957565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b604051906131a282611670565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81813603018212156101c6570190565b90821015610791576117389160051b8101906131ad565b5f198114612a2f5760010190565b91908110156107915760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1813603018212156101c6570190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101c6570180359067ffffffffffffffff82116101c6576020019181360383136101c657565b5f90602092604051908482019283526040820152604081526132b96060826116a8565b604051918291518091835e8101838152039060025afa15611907575f5190565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c161561330857565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b91908110156107915760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc1813603018212156101c6570190565b5f92918390815b8181106133845750505050565b61339c613392828487613330565b6020810190613245565b8461340657806080116101c65760206133ba6060840135838561414a565b15928315936133dd575b5050506133d357600101613377565b5060019450505050565b91955091505f948060a0116101c6576133fc925060808201359161414a565b15155f80806133c4565b8480fd5b601f8260209493601f1993818652868601375f8582860101520116010190565b60409061173894928152816020820152019161340a565b9061344c8180611b47565b5f5b8181106135f6575050506134656020820182611b47565b5f5b8181106135875750505061347e6040820182611b47565b5f5b81811061351857505050806060613498920190611b47565b90915f5b8281106134a95750505050565b6134b4818486613330565b359060028210156101c65760018092146134cf575b0161349c565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c6134ff61339284888a613330565b90613510604051928392878461342a565b0390a26134c9565b613523818385613330565b359060028210156101c657600180921461353e575b01613480565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd5961356e613392848789613330565b9061357f604051928392878461342a565b0390a2613538565b613592818385613330565b359060028210156101c65760018092146135ad575b01613467565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f36135dd613392848789613330565b906135ee604051928392878461342a565b0390a26135a7565b613601818385613330565b359060028210156101c657600180921461361c575b0161344e565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae661364c613392848789613330565b9061365d604051928392878461342a565b0390a2613616565b801580156136b7575b80156136af575b801561369f575b613699576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d01982101561367c565b508115613675565b506401000003d01981101561366e565b92939290915f908083036139125750506401000003d0195f9483086136f057505090505f905f90565b6401000003d0198280096401000003d0195f8180808086880960040995800960030908906401000003d01991828480088303838111612a2f57839081838009089384840392848411612a2f5784809180096008098403848111612a2f5784600193818096819508900908940960020991905b8215158381613901575b50806138f9575b1561389b579084939293906001936401000003d0199187965b80156138485780840496809861381b576401000003d0199088096401000003d019036401000003d0198111612a2f576401000003d019908a9608979381978281029281840414901517156137ee57906137e491613bde565b959293969561378c565b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b509450945050938061386e5750906401000003d019809281808780098092099509900990565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b506001613773565b6401000003d019915014155f61376c565b613926936401000003d01993969296613beb565b91613762565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1823603018112156101c657016020813591019167ffffffffffffffff82116101c6578160051b360383136101c657565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81823603018112156101c6570190565b906020838281520160208260051b85010193835f917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc182360301905b8584106139fe575050505050505090565b90919293949596601f198282030186528735838112156101c6578401803560028110156101c657825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1823603018112156101c657016020813591019067ffffffffffffffff81116101c65780360382136101c657613a91602092839260408681866001990152019161340a565b990197960194019291906139ed565b61173891613b09613afe613ae3613ac8613aba868061392c565b6080875260808701916139b1565b613ad5602087018761392c565b9086830360208801526139b1565b613af0604086018661392c565b9085830360408701526139b1565b92606081019061392c565b9160608185039101526139b1565b905f8091602081519101845af48080613bcb575b15613b4b5750506040513d81523d5f602083013e60203d82010160405290565b15613b925773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d15613ba3576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d151580613b2b5750813b1515613b2b565b91908203918211612a2f57565b949291851580613e0b575b613dff57801580613df7575b613ded57604051608091613c1683836116a8565b823683378615613dc05786948580928180600180098087529781896001099c602088019d8e5282604089019d8e8c8152516001099160608a019283526040519e8f613c609061168c565b5190098d525190099460208b019586525190099860408901998a52519009606087019081528651885114801590613db4575b15613d5657849283808093816040519c85613cae8f97886116a8565b368737518c51613cbe9083613bde565b90088452518551613ccf9083613bde565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a525180098851613d0c9083613bde565b90088180875185519009600209613d239083613bde565b90089c51935190519009613d378c83613bde565b90089009925190519009613d4b9083613bde565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b50815181511415613c92565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5092506001919050565b508215613c02565b94509092506001919050565b508115613bf6565b359073ffffffffffffffffffffffffffffffffffffffff821682036101c657565b915f5b818110613e445750505050565b613e52613392828486613330565b819291016060838203126101c657613e6983613e13565b602084013567ffffffffffffffff81116101c65782613e8991860161171d565b604085013567ffffffffffffffff81116101c657613ec3613ecb9473ffffffffffffffffffffffffffffffffffffffff926001980161171d565b9216886142ba565b01613e37565b909291601f19613ef9613ee383611793565b92613ef160405194856116a8565b808452611793565b015f5b818110613ff757505080935f905f5b818110613f19575050505050565b613f276123a7828488611b9b565b5f5b818110613fb157505050613f416123c9828488611b9b565b5f5b818110613f5557505050600101613f0b565b80613f636001928486613205565b613f7361240c60408301836131ad565b9081613f83575b50505001613f43565b613fa992613f9a613f938c6131f7565b9b8d611bdb565b5189602083013592359161448a565b5f8080613f7a565b80613fbf60019284866131e0565b613fcf61240c60608301836131ad565b9081613fdf575b50505001613f29565b613fef92613f9a613f938c6131f7565b5f8080613fd6565b6020906040516140068161168c565b5f81525f838201525f604082015260608082015282828601015201613efc565b8051915f5b8381106140385750505050565b61404c6140458284611bdb565b5184611bdb565b516060810180515191602001905f5b83811061406e575050505060010161402b565b806140ab61407f6001938551611bdb565b5185519073ffffffffffffffffffffffffffffffffffffffff81511660406020830151920151926142ba565b0161405b565b156140b95750565b7fbde60f60000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b156140ec5750565b7f40a0d1c1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b1561411f5750565b7f3b9b6984000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b602083019291838311612a2f578383116101c65783116101c657810135910360208110614175575090565b5f199060200360031b1b1690565b81519190604183036141b3576141ac9250602082015190606060408401519301515f1a9061486e565b9192909190565b50505f9160029190565b600481101561426857806141cf575050565b600181036141ff577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002810361423357507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b60031461423d5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b90916142ac6117389360408452604084019061176e565b91602081840391015261176e565b919073ffffffffffffffffffffffffffffffffffffffff1692604051927f33a892030000000000000000000000000000000000000000000000000000000084526004840152604060248401525f8380614316604482018661176e565b038183885af1928315611907575f936143b1575b508251602084012081516020830120036143765750907fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf19161437160405192839283614295565b0390a2565b90506143ad6040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401614295565b0390fd5b9092503d805f833e6143c381836116a8565b8101906020818303126101c65780519067ffffffffffffffff82116101c6570181601f820112156101c6578051906143fa826116cb565b9261440860405194856116a8565b828452602083830101116101c657815f9260208093018386015e83010152915f61432a565b9080601f830112156101c657813561444481611793565b9261445260405194856116a8565b81845260208085019260051b8201019283116101c657602001905b82821061447a5750505090565b813581526020918201910161446d565b93949291906040850152835260208301526144a481611793565b916144b260405193846116a8565b818352601f196144c183611793565b015f5b8181106145c95750506060019182525f5b8181106144e25750505050565b6144ed818451611bdb565b51906144fd613392828588613330565b8101929060a0818503126101c65761451481613e13565b602082013567ffffffffffffffff81116101c6578561453491840161171d565b604083013567ffffffffffffffff81116101c6578661455491850161171d565b606084013567ffffffffffffffff81116101c6578761457491860161442d565b60808501359467ffffffffffffffff86116101c65760019873ffffffffffffffffffffffffffffffffffffffff966145ac920161442d565b6080870152606086015260408501526020840152169052016144d5565b6040519060a082019180831067ffffffffffffffff8411176107be576020926040525f81526060838201526060604082015260608082015260606080820152828288010152016144c4565b602081519101519080516101008111614806575f5f5b8281106147e0575061463b82611962565b935f5f5b84811061465157505050505050600191565b83156147d5576147056146e7856001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b946001861b90811916931794806146fe848b611bdb565b5284611bdb565b51808519915b61471c57505060010192909261463f565b6147a9816001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b90826147bd6001841b92831916938b611bdb565b5116156147cc575b508061470b565b819417936147c5565b505f96945050505050565b6147ea8185611bdb565b51156147f9575b60010161462a565b6001811b909117906147f1565b7fa156ec46000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b91908251905f935b82851061484557505050565b90919293826148548284611bdb565b5151146148675760010193929190614839565b9250505090565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116148f2579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15611907575f5173ffffffffffffffffffffffffffffffffffffffff8116156148e857905f905f90565b505f906001905f90565b5050505f9160039190565b929091604061490c8286611bdb565b510151604061491b8487611bdb565b510151146149b55761493261493a918593956149f8565b9490926149f8565b936149458184614a97565b926149508684614a97565b60208601915b83811061499257505050505b83811061496f5750505050565b808261497e6001938651611bdb565b511761498b828651611bdb565b5201614962565b80826149a16001938651611bdb565b51176149ae828651611bdb565b5201614956565b9092506020826149c9856149f59551611bdb565b516001841b176149da868351611bdb565b52019260016149ea838651611bdb565b51911b179251611bdb565b52565b916040614a058385611bdb565b51015190825b8015801580614a75575b15614a2657612a2f575f1901614a0b565b509390919260018101809111612a2f57928151935b8481141580614a5e575b15614a5857614a53906131f7565b614a3b565b93505050565b50816040614a6c8386611bdb565b51015114614a45565b505f198201828111612a2f576040614a8e869289611bdb565b51015114614a15565b9061010003906101008211612a2f576001901b5f198101908111612a2f575f199019911c169056fea164736f6c6343000826000af0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE04a\x01\x10W`\x1FaL\xAB8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x14W\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x01\x10W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x82\x82\x03a\x01\x10W` \x01Q\x91`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x90\x81\x84\x03a\x01\x10W0`\x80Ra\0ua\x01(V[a\0}a\x01(V[a\0\x85a\x01(V[a\0\x8Da\x01(V[\x15a\x01\x01W\x15a\0\xF2W`\xA0R`\xC0Ra\0\xA5a\x01(V[`@QaJ\xCC\x90\x81a\x01\xBF\x829`\x80Q\x81\x81\x81a\x0F\x1E\x01Ra\x10v\x01R`\xA0Q\x81\x81\x81a\x0E>\x01R\x81\x81a\x18F\x01Ra\x1ET\x01R`\xC0Q\x81\x81\x81a\x03\xA7\x01R\x81\x81a\x18\x04\x01Ra\x1E\x14\x01R\xF3[c\xB1\xB2Z\xAF`\xE0\x1B_R`\x04_\xFD[cSl\x15\x01`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_Q` aL\x8B_9_Q\x90_RT`\xFF\x81`@\x1C\x16a\x01\xAFW`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01a\x01]WPV[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17_Q` aL\x8B_9_Q\x90_RU`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD\xFEa\x01\0`@R`\x036\x11a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x15_\x9F\x02\x14a\x15\x82W\x80c\x17\x1E\xAC\x18\x14a\x15WW\x80c\x1A\x84s\xBC\x14a\x13\xB5W\x80c?K\xA8:\x14a\x12\xF8W\x80cO\x1E\xF2\x86\x14a\x10*W\x80cQ\x94[\x06\x14a\x0F\x96W\x80cR\xD1\x90-\x14a\x0E\xF7W\x80c\\\x97Z\xBB\x14a\x0E\xB6W\x80cd\xC1*\xF2\x14a\x0E|W\x80cg51\"\x14a\x0EbW\x80ck\xF0\xA0K\x14a\x0E\x12W\x80cqP\x18\xA6\x14a\rVW\x80c\x84V\xCBY\x14a\x0C\xBFW\x80c\x8B3\xE2\0\x14a\x0C,W\x80c\x8D\xA5\xCB[\x14a\x0B\xDAW\x80c\x929\xAC\x0C\x14a\x0BRW\x80c\xA0`V\xF7\x14a\x0B\x13W\x80c\xAA\xF1\x0FB\x14a\n\xC1W\x80c\xAD<\xB1\xCC\x14a\nbW\x80c\xBD\xEBD-\x14a\t\rW\x80c\xC1\xB0\xBE\xD7\x14a\x08\xBFW\x80c\xC4IV\xD1\x14a\x08\x83W\x80c\xC4\xD6m\xE8\x14a\x03\xCBW\x80c\xE3Z]/\x14a\x03oW\x80c\xED\x83\xCD\xC7\x14a\x02\xC7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9CW\x80c\xF8\xAD\xF4\xC3\x14a\x02HW\x80c\xFB\x0F\xF6\x80\x14a\x02\x0CW\x80c\xFE\x18\xAB\x91\x14a\x01\xCAWc\xFF\xA1\xADt\x14a\x01cW_\x80\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x01\xC2`@Qa\x01\x84`@\x82a\x16\xA8V[`\x0B\x81R\x7F2.0.0-rc.12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17nV[\x03\x90\xF3[_\x80\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` \x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W` a\x02\x92`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6Wa\x02\xC5a\x02\xB8a\x16MV[a\x02\xC0a1)V[a\x19\x93V[\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R` _ \x90_[\x81\x81\x10a\x03YWa\x01\xC2\x85a\x03E\x81\x87\x03\x82a\x16\xA8V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17;V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03.V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6Wa\x03\xE4a\x16MV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16\x15\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x15\x90\x81a\x08{W[`\x01\x14\x90\x81a\x08qW[\x15\x90\x81a\x08hW[Pa\x08@W\x81\x83`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x04\xA5\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x07\xEBW[Pa\x04\x95a2\xD9V[a\x04\x9Da2\xD9V[a\x02\xC0a2\xD9V[a\x04\xADa2\xD9V[a\x04\xB5a2\xD9V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xBEW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x07\x91W\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x01U_\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x06,a2\xD9V[a\x064a2\xD9V[a\x06<a2\xD9V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2_\x80\xA2a\x06\xCCa\x17\xBBV[a\x07iWa\x06\xD6W\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[\x7F%\xAEn\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x83a\x04\x8CV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x84a\x045V[0;\x15\x91Pa\x04-V[\x84\x91Pa\x04#V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ T\x90_[\x81\x81\x10a\t\xA2W` \x83`@Q\x90\x81R\xF3[\x90\x91`\x01\x90\x84\x82\x16a\n\x0BW\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta\t\xFE\x91a2\x96V[\x93[\x81\x1C\x93\x92\x91\x01a\t\x90V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta\n\\\x91\x90a2\x96V[\x93a\n\0V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x01\xC2`@Qa\n\x83`@\x82a\x16\xA8V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17nV[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W`\x80`\x03\x19\x826\x03\x01\x12a\x01\xC6Wa\x0B\xB5\x90a\x0B\x8Fa\x1A\x80V[a\x0B\x97a\x1A\xF4V[`@Q\x90a\x0B\xA6` \x83a\x16\xA8V[_\x82R_6\x817`\x04\x01a\x1B\xEFV[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045a\x0CHa1)V[\x80\x15a\x0C\x97W\x80\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2_\x80\xA2\0[\x7F\x0B\xF7q\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x0C\xD7a1)V[a\x0C\xDFa\x1A\xF4V[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\rna1)V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` a\x02\x92a\x17\xBBV[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0FnW` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x90_[\x81\x81\x10a\x10\x14Wa\x01\xC2\x85a\x03E\x81\x87\x03\x82a\x16\xA8V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x0F\xFDV[`@`\x03\x196\x01\x12a\x01\xC6Wa\x10>a\x16MV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6Wa\x10^\x906\x90`\x04\x01a\x17\x1DV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a\x12\xB6W[Pa\x0FnWa\x10\xAEa1)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a\x12\x82W[Pa\x11.W\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a\x12WWP\x82;\x15a\x12,W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a\x11\xFBWa\x02\xC5\x91a;\x17V[PP4a\x12\x04W\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a\x12\xAEW[\x81a\x12\x9E` \x93\x83a\x16\xA8V[\x81\x01\x03\x12a\x01\xC6WQ\x90\x85a\x10\xFDV[=\x91Pa\x12\x91V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a\x10\xA1V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x13\x10a1)V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x81\x16\x15a\x13\x8DW`\xFF\x19\x16\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W6`#\x82\x01\x12\x15a\x01\xC6W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W`$\x82\x01\x91`$6\x91\x83`\x06\x1B\x01\x01\x11a\x01\xC6W\x90a\x14\x14a1)V[_\x91[\x80\x83\x10a\x14 W\0[a\x14+\x83\x82\x84a\x17\xABV[5\x92` a\x14:\x82\x84\x86a\x17\xABV[\x015\x93\x84\x15\x15\x94\x85\x81\x03a\x01\xC6W\x81\x15a\x15/Wa\x14X\x81\x83a0\xE8V[a\x14\xFFW` `\x01\x94\x95\x96\x7F\xA7\x0FD3a^\x08:\xDF\x1B\x97\xE0\xC3-\x9Ar\x1C9\xC2mm\xD6\xC2\x8F\xE9av\xAB\x90\x9F\x0E\xCA\x92\x84_R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0\x83R`@_ \x90_\x14a\x14\xD0W\x86`\xFF\x19\x82T\x16\x17\x90U[`@Q\x90\x81R\xA2\x01\x91\x90a\x14\x17V[a\x01\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x82T\x16\x17\x90Ua\x14\xC1V[\x85\x82\x7F\x12}3^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7Fz\xF6#A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W`@`\x03\x196\x01\x12a\x01\xC6W`$5\x80\x15\x15\x81\x03a\x01\xC6Wa\x02\x92` \x91`\x045a0\xE8V[4a\x01\xC6W`@`\x03\x196\x01\x12a\x01\xC6W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W`\x80`\x03\x19\x826\x03\x01\x12a\x01\xC6W`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W6`#\x83\x01\x12\x15a\x01\xC6W\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W`$\x82`\x05\x1B\x84\x01\x016\x81\x11a\x01\xC6Wa\x15\xFEa\x1A\x80V[a\x16\x06a\x1A\xF4V[a\x16\x0F\x83a\x17\x93V[\x92a\x16\x1D`@Q\x94\x85a\x16\xA8V[\x83R`$` \x84\x01\x94\x01\x93[\x81\x85\x10a\x16=Wa\x0B\xB5\x84\x84`\x04\x01a\x1B\xEFV[\x845\x81R` \x94\x85\x01\x94\x01a\x16)V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xC6WV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x16\xF3\x82a\x16\xCBV[\x91a\x17\x01`@Q\x93\x84a\x16\xA8V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xC6W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xC6W\x81` a\x178\x935\x91\x01a\x16\xE7V[\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x17XWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17KV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x05\x1B` \x01\x90V[\x91\x90\x81\x10\x15a\x07\x91W`\x06\x1B\x01\x90V[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x19\x07W_\x91a\x19\x12W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a\x19\x07W_\x91a\x18\xCFWP\x90V[\x90P` \x81=` \x11a\x18\xFFW[\x81a\x18\xEA` \x93\x83a\x16\xA8V[\x81\x01\x03\x12a\x01\xC6WQ\x80\x15\x15\x81\x03a\x01\xC6W\x90V[=\x91Pa\x18\xDDV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x19ZW[\x81a\x19-` \x93\x83a\x16\xA8V[\x81\x01\x03\x12a\x01\xC6WQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01\xC6W` a\x18vV[=\x91Pa\x19 V[\x90a\x19l\x82a\x17\x93V[a\x19y`@Q\x91\x82a\x16\xA8V[\x82\x81R`\x1F\x19a\x19\x89\x82\x94a\x17\x93V[\x01\x90` 6\x91\x017V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x1ATWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x1A\xCCW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]V[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a\x1B\x1FWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01\xC6WV[\x91\x90\x81\x10\x15a\x07\x91W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[\x80Q\x82\x10\x15a\x07\x91W` \x91`\x05\x1B\x01\x01\x90V[\x90`\xA0Ra\x1B\xFD\x81\x80a\x1BGV[\x90P_`\xC0R\x80\x15a0\xC0W\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T``\x83\x015\x14\x80\x15a0\x93W[\x15a0cWa\x1CF\x81a\x19bV[\x91a\x1COa1\x95V[P`@Qa\x1C\\\x81a\x16pV[_\x81R_` \x82\x01R\x92_\x90_`\x80R_\x93_\x90[\x80\x82\x10a(\x11WPP` \x81Q`\x05\x1B\x91\x01 \x93`\xA0QQ\x15_\x14a'\xA3Wa\x1C\xBF\x90a\x1C\xB6\x86[a\x1C\xB0a\x1C\xA9` \x88\x01\x88a2EV[6\x91a\x16\xE7V[\x90aA\x83V[\x90\x93\x91\x93aA\xBDV[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a'uWPPa\x1C\xFD\x82\x80a\x1BGV[`@Q\x90` \x82\x01\x92\x81`\x80\x84\x01\x7F\xBCN\xDA\xEE\xA9\x8Fe\x01T|\xBA4\xCF\x94\xD8\xB5\x961\xE7\xF8l\xF1\xA4q\x19\xBAo\x17\xC2\x9AQ|\x86R``\x88\x015`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01`\xA0\x83`\x05\x1B\x85\x01\x01\x92\x82_\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01[\x83\x83\x10a%\x98WPPPPPP\x91\x81a\x1D\x9B_\x93` \x95\x03`\x1F\x19\x81\x01\x83R\x82a\x16\xA8V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x19\x07W_Qa\x1D\xC5`@\x84\x01\x84a2EV[\x80`\x04\x11a\x01\xC6W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a%jWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x01\xC6W_\x92a\x1E\xBF\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a4\nV[\x90\x7F5\x08*9\xEC\xBA\x80Z\x1C\xA4\xFD\xAC~\xB3d\xB2\n\xFC\xE2\xAB\xF1D\x13\xEB\xD9\x10#32\xEEE\xC9`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a\x19\x07Wa%VW[P\x80a$\x86W[P`\x80Q\x15a$OW\x80a\x1F\x14\x91a\x1BGV[`\xA0QQ\x91\x92\x90\x91\x15a!\xD4WP\x90a\x1F0\x91`\x80Q\x91a>\xD1V[\x80Q\x80`\xA0QQ\x03a!\x96Wa\x1FE\x81a\x19bV[\x90`\xC0Q[\x81\x81\x10a!\\WP`\x01[\x81\x81\x10a \xACWP`\xC0Q[\x81\x81\x10a\x1F\xA2WPPPa\x1Fx\x90`\xA0Q\x90a@&V[\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCE`\xC0Q`\xC0Q\xA2V[``a\x1F\xAE\x82\x86a\x1B\xDBV[Q\x01Q\x80Q\x90`\xC0Q[\x82\x81\x10a\x1F\xCAWPPP`\x01\x01a\x1FaV[``a\x1F\xD6\x82\x84a\x1B\xDBV[Q\x01Q\x80Q\x90\x86`\xC0Q[\x87\x8B\x8B\x86\x84\x10a mWPPPPPPP`\x80a\x1F\xFE\x82\x84a\x1B\xDBV[Q\x01Q\x80Q\x90`\xC0Q[\x88\x88\x8B\x85\x84\x10a  WPPPPPP`\x01\x01a\x1F\xB8V[\x83\x92a `a g\x93a Oa F\x94a <`\x01\x99\x8Ba\x1B\xDBV[Q\x95\x86\x80\x92aH1V[\x92\x83\x14\x15aA\x17V[a Y\x8D\x84a\x1B\xDBV[Q\x92a\x1B\xDBV[Q\x11a@\xE4V[\x01a \x08V[a `a \xA4\x93a Y`\x01\x97a \x9Ea \x95\x96a \x8B\x8A\x8Da\x1B\xDBV[Q\x97\x88\x80\x92aH1V[\x92\x83\x14\x15a@\xB1V[\x84a\x1B\xDBV[\x01\x87\x90a\x1F\xE1V[`@a \xC4a \xBD\x83`\xA0Qa\x1B\xDBV[Q\x86a\x1B\xDBV[Q\x01Q_\x19\x82\x01\x82\x81\x11a!+Wa \xEAa \xE3`@\x92`\xA0Qa\x1B\xDBV[Q\x87a\x1B\xDBV[Q\x01Q\x81\x10a \xFCWP`\x01\x01a\x1FUV[\x7F\xF3GM\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04R`$`\xC0Q\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x11`\x04R`$`\xC0Q\xFD[a!h\x81`\xA0Qa\x1B\xDBV[Q\x90\x82\x82\x10\x80a!\xC2W[\x15a!\x96W`\x01\x81\x01\x80\x82\x11a!+Wa!\x8F`\x01\x93\x86a\x1B\xDBV[R\x01a\x1FJV[\x7F\x9A\x12\xC5\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04`\xC0Q\xFD[Pa!\xCD\x82\x85a\x1B\xDBV[Q\x15a!sV[\x15a#\x86W\x90a!\xE7\x91`\x80Q\x91a>\xD1V[\x80Qa\x01\0\x81\x11a#WWP`@Qa!\xFF\x81a\x16pV[``\x81R` \x81\x01``\x81R\x82Q\x90a\"\x17\x82a\x19bV[\x83Ra\"\"\x82a\x19bV[\x90R`\xC0Q[\x81\x81\x10a\"zWPPa\":\x90aF\x14V[\x90\x15a\"NWa\"I\x91a@&V[a\x1FxV[\x7F\x81<v\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04`\xC0Q\xFD[``a\"\x86\x82\x86a\x1B\xDBV[Q\x01Q\x80Q\x90`\xC0Q[\x82\x81\x10a\"\xA2WPPP`\x01\x01a\"(V[a\"\xAC\x81\x83a\x1B\xDBV[Q``\x81\x01Q\x80Q\x90`\xC0Q[\x87\x89\x8B\x8D\x86\x85\x10a#&WPPPPPPP`\x80\x01Q\x80Q\x90`\xC0Q[\x87\x87\x8A\x8C\x86\x85\x10a\"\xF0WPPPPPPP`\x01\x01a\"\x90V[\x91\x84\x93\x91a# \x93a#\x1Ba#\x07`\x01\x98\x8Aa\x1B\xDBV[Qa#\x12\x81\x84aH1V[\x95\x86\x14\x15aA\x17V[aH\xFDV[\x01a\"\xD6V[\x91\x84\x93\x91a#Q\x93a#\x1Ba#=`\x01\x98\x8Aa\x1B\xDBV[Qa#H\x81\x84aH1V[\x94\x85\x14\x15a@\xB1V[\x01a\"\xB9V[\x7F\x89P\"\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04R`$`\xC0Q\xFD[`\xC0Q[\x81\x81\x10a#\x99WPPPa\x1FxV[a#\xADa#\xA7\x82\x84\x86a\x1B\x9BV[\x80a\x1BGV[`\xC0Q[\x81\x81\x10a$&WPPPa#\xD3a#\xC9\x82\x84\x86a\x1B\x9BV[` \x81\x01\x90a\x1BGV[`\xC0Q[\x81\x81\x10a#\xE9WPPP`\x01\x01a#\x8AV[\x80a$ ` a#\xFC`\x01\x94\x86\x88a2\x05V[a$\x16a$\x0C`@\x83\x01\x83a1\xADV[`@\x81\x01\x90a\x1BGV[\x92\x90\x91\x015a>4V[\x01a#\xD7V[\x80a$I` a$9`\x01\x94\x86\x88a1\xE0V[a$\x16a$\x0C``\x83\x01\x83a1\xADV[\x01a#\xB1V[PP`\xA0QQ\x15a\x1FxW\x7F\x9A\x12\xC5\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04`\xC0Q\xFD[\x80`\xC0QR\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@`\xC0Q T\x16a%'W` \x81\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x92`\xC0QR\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03\x82R`@`\xC0Q `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA1_a\x1F\x01V[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04R`$`\xC0Q\xFD[_a%`\x91a\x16\xA8V[_`\xC0R_a\x1E\xFAV[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x88\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x01\xC6W\x83\x01\x90`\xA0\x81\x01\x91a%\xE1\x81\x80a9,V[\x80\x94`\xA0\x85RR`\xC0\x83\x01`\xC0\x85`\x05\x1B\x85\x01\x01\x94\x82_[\x82\x81\x10a&\xF6WPPPPPa&\x12` \x82\x01\x82a9,V[\x93\x90\x83\x82\x03` \x85\x01R\x84\x82R` \x82\x01\x90` \x86`\x05\x1B\x84\x01\x01\x95\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a&\x98WPPPP`@\x80\x85\x015\x90\x86\x01RPPP``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01R\x95` \x90\x81\x01\x95\x01\x93\x92`\x01\x01\x91\x90a\x1DvV[\x90\x91\x92\x93\x94\x95\x98`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x01\xC6W` a&\xE7`\x01\x93``a&\xD9\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90a9\x7FV[\x91\x81`@\x82\x01R\x01\x90a:\xA0V[\x9B\x01\x97\x01\x95\x94\x93\x92\x91\x01a&YV[\x90\x91\x92\x93\x96` \x80a'h\x83`\x80a'Za':\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F`\x01\x9A\x03\x01\x8DR\x8Aa9\x7FV[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90a9\x7FV[\x91\x81``\x82\x01R\x01\x90a:\xA0V[\x99\x01\x95\x01\x93\x92\x91\x01a%\xF9V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`@Q` \x81\x01\x90``\x81\x01\x87\x83R`@\x80\x83\x01R`\xA0QQ\x80\x91R`\x80\x82\x01\x90` `\xA0Q\x01\x90_[\x81\x81\x10a'\xFBWPPP\x91\x81a'\xF3a\x1C\xB6\x93a\x1C\xBF\x96\x95\x03`\x1F\x19\x81\x01\x83R\x82a\x16\xA8V[Q\x90 a\x1C\x99V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xCDV[\x90\x95a('\x87a(!\x87\x80a\x1BGV[\x90a\x1B\x9BV[`\xE0R___\x91a(<`\xE0Q`\xE0Qa\x1BGV[\x90Pa(G\x81a\x19bV[\x90a(Q\x81a\x19bV[\x90_[\x81\x81\x10a.~WPPa(n` `\xE0Q\x01`\xE0Qa\x1BGV[\x91\x90Pa(z\x82a\x19bV[\x90a(\x84\x83a\x19bV[\x92_\x90[\x80\x82\x10a*dWPPa(\xE2\x92a(\xFE\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a(\xF0a)\x0C\x94`@Q\x97\x88\x97`\x80`\xE0Q\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a\x17;V[\x90\x87\x82\x03`@\x89\x01Ra\x17;V[\x90\x85\x82\x03``\x87\x01Ra\x17;V[\x90\x83\x82\x03`\x80\x85\x01Ra\x17;V[\x03\x90\xA1\x80a*\\W[P\x80a*\nW[PP`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\xE0Q6\x03\x01\x12a\x01\xC6W`@Q\x90a)Y\x82a\x16pV[`@`\xE0Q\x015\x82R` \x82\x01```\xE0Q\x015\x81R`@Q\x92a)|\x84a\x16pV[_\x84R_` \x85\x01Ra)\x92\x81Q\x83Q\x90a6eV[\x15a)\xD2W\x91a)\xB1\x91`\x01\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92a6\xC7V[` \x83\x01R\x81R\x96`\x80`\xE0Q\x015a)\xCA\x82\x86a\x1B\xDBV[R\x01\x90a\x1CqV[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[`\x80Q\x01\x80`\x80Q\x11a*/W`\x80Ra*&W[_\x80a)\x1CV[`\x01\x95Pa*\x1FV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x95P_a)\x15V[\x90\x95Pa*\x82\x86a*|` `\xE0Q\x01`\xE0Qa\x1BGV[\x90a2\x05V[\x95a*\x91_` \x89\x015a0\xE8V[a.JW`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ta*\xE5\x81a1\xF7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x885\x91_[\x82\x81\x10a-fWPP\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a+\xF3W[P\x96`@\x81\x01a+ya$\x0C\x82\x84a1\xADV[\x80a+\xBCW[PP\x81a+\x9Ba+\x94` \x93`\x01\x96\x95a1\xADV[\x825a4AV[\x805a+\xA7\x84\x89a\x1B\xDBV[R\x015a+\xB4\x82\x88a\x1B\xDBV[R\x01\x90a(\x88V[\x90\x9Aa+\xC7\x90a1\xF7V[\x9A\x8Ca+\x7FW` \x92\x9CPa+\x94a+\xE6`\x01\x96\x95\x93a+\x9B\x93a3pV[\x9D\x93PP\x81\x93\x94Pa+\x7FV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xBEW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x81\x10\x15a\x07\x91W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x01\x82\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x01T\x90a,\xD6\x90a,\xCF\x90\x83\x90a2\x96V[\x91\x80a2\x96V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xBEW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x07\x91W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x01U_a+fV[\x90\x92`\x01\x90\x81\x85\x16a-\xF3W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta-\xE8\x91a2\x96V[\x93[\x81\x1C\x91\x01a+\rV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta.D\x91\x90a2\x96V[\x93a-\xEAV[` \x87\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R_`$R`D_\xFD[a.\x96\x81a.\x90`\xE0Q`\xE0Qa\x1BGV[\x90a1\xE0V[a.\xA5`\x01` \x83\x015a0\xE8V[a0.W`@\x81\x015a.\xE3\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[\x15a0\x03WP\x805_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16a/\xD7W\x805_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U``\x81\x01a/ea$\x0C\x82\x84a1\xADV[\x80a/\xA0W[PP\x81a/\x80a+\x94` \x93`\x01\x96\x95a1\xADV[\x805a/\x8C\x84\x89a\x1B\xDBV[R\x015a/\x99\x82\x86a\x1B\xDBV[R\x01a(TV[\x90\x98a/\xAB\x90a1\xF7V[\x98\x8Aa/kW` \x92\x9APa+\x94a/\xCA`\x01\x96\x95\x93a/\x80\x93a3pV[\x9B\x93PP\x81\x93\x94Pa/kV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R5`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[` \x90\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R`\x01`$R`D_\xFD[``\x82\x7F\xC9>\xE1c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R`$_\xFD[P\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06``\x83\x015\x14a\x1C8V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[_\x90\x81R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0` R`@\x90 `\xFF\x91\x15a1!WT\x16\x90V[T`\x08\x1C\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a1iWV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`@Q\x90a1\xA2\x82a\x16pV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[\x90\x82\x10\x15a\x07\x91Wa\x178\x91`\x05\x1B\x81\x01\x90a1\xADV[_\x19\x81\x14a*/W`\x01\x01\x90V[\x91\x90\x81\x10\x15a\x07\x91W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W` \x01\x91\x816\x03\x83\x13a\x01\xC6WV[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81Ra2\xB9``\x82a\x16\xA8V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x19\x07W_Q\x90V[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a3\x08WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x07\x91W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[_\x92\x91\x83\x90\x81[\x81\x81\x10a3\x84WPPPPV[a3\x9Ca3\x92\x82\x84\x87a30V[` \x81\x01\x90a2EV[\x84a4\x06W\x80`\x80\x11a\x01\xC6W` a3\xBA``\x84\x015\x83\x85aAJV[\x15\x92\x83\x15\x93a3\xDDW[PPPa3\xD3W`\x01\x01a3wV[P`\x01\x94PPPPV[\x91\x95P\x91P_\x94\x80`\xA0\x11a\x01\xC6Wa3\xFC\x92P`\x80\x82\x015\x91aAJV[\x15\x15_\x80\x80a3\xC4V[\x84\x80\xFD[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a\x178\x94\x92\x81R\x81` \x82\x01R\x01\x91a4\nV[\x90a4L\x81\x80a\x1BGV[_[\x81\x81\x10a5\xF6WPPPa4e` \x82\x01\x82a\x1BGV[_[\x81\x81\x10a5\x87WPPPa4~`@\x82\x01\x82a\x1BGV[_[\x81\x81\x10a5\x18WPPP\x80``a4\x98\x92\x01\x90a\x1BGV[\x90\x91_[\x82\x81\x10a4\xA9WPPPPV[a4\xB4\x81\x84\x86a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a4\xCFW[\x01a4\x9CV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca4\xFFa3\x92\x84\x88\x8Aa30V[\x90a5\x10`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a4\xC9V[a5#\x81\x83\x85a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a5>W[\x01a4\x80V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa5na3\x92\x84\x87\x89a30V[\x90a5\x7F`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a58V[a5\x92\x81\x83\x85a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a5\xADW[\x01a4gV[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3a5\xDDa3\x92\x84\x87\x89a30V[\x90a5\xEE`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a5\xA7V[a6\x01\x81\x83\x85a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a6\x1CW[\x01a4NV[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6a6La3\x92\x84\x87\x89a30V[\x90a6]`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a6\x16V[\x80\x15\x80\x15a6\xB7W[\x80\x15a6\xAFW[\x80\x15a6\x9FW[a6\x99Wd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15a6|V[P\x81\x15a6uV[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15a6nV[\x92\x93\x92\x90\x91_\x90\x80\x83\x03a9\x12WPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08a6\xF0WPP\x90P_\x90_\x90V[d\x01\0\0\x03\xD0\x19\x82\x80\td\x01\0\0\x03\xD0\x19_\x81\x80\x80\x80\x86\x88\t`\x04\t\x95\x80\t`\x03\t\x08\x90d\x01\0\0\x03\xD0\x19\x91\x82\x84\x80\x08\x83\x03\x83\x81\x11a*/W\x83\x90\x81\x83\x80\t\x08\x93\x84\x84\x03\x92\x84\x84\x11a*/W\x84\x80\x91\x80\t`\x08\t\x84\x03\x84\x81\x11a*/W\x84`\x01\x93\x81\x80\x96\x81\x95\x08\x90\t\x08\x94\t`\x02\t\x91\x90[\x82\x15\x15\x83\x81a9\x01W[P\x80a8\xF9W[\x15a8\x9BW\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15a8HW\x80\x84\x04\x96\x80\x98a8\x1BWd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a*/Wd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a7\xEEW\x90a7\xE4\x91a;\xDEV[\x95\x92\x93\x96\x95a7\x8CV[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80a8nWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01a7sV[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_a7lV[a9&\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96a;\xEBV[\x91a7bV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x01\xC6W\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W\x81`\x05\x1B6\x03\x83\x13a\x01\xC6WV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a\x01\xC6W\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x826\x03\x01\x90[\x85\x84\x10a9\xFEWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`\x1F\x19\x82\x82\x03\x01\x86R\x875\x83\x81\x12\x15a\x01\xC6W\x84\x01\x805`\x02\x81\x10\x15a\x01\xC6W\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x01\xC6W\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x806\x03\x82\x13a\x01\xC6Wa:\x91` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91a4\nV[\x99\x01\x97\x96\x01\x94\x01\x92\x91\x90a9\xEDV[a\x178\x91a;\ta:\xFEa:\xE3a:\xC8a:\xBA\x86\x80a9,V[`\x80\x87R`\x80\x87\x01\x91a9\xB1V[a:\xD5` \x87\x01\x87a9,V[\x90\x86\x83\x03` \x88\x01Ra9\xB1V[a:\xF0`@\x86\x01\x86a9,V[\x90\x85\x83\x03`@\x87\x01Ra9\xB1V[\x92``\x81\x01\x90a9,V[\x91``\x81\x85\x03\x91\x01Ra9\xB1V[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80a;\xCBW[\x15a;KWPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15a;\x92Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15a;\xA3W`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80a;+WP\x81;\x15\x15a;+V[\x91\x90\x82\x03\x91\x82\x11a*/WV[\x94\x92\x91\x85\x15\x80a>\x0BW[a=\xFFW\x80\x15\x80a=\xF7W[a=\xEDW`@Q`\x80\x91a<\x16\x83\x83a\x16\xA8V[\x826\x837\x86\x15a=\xC0W\x86\x94\x85\x80\x92\x81\x80`\x01\x80\t\x80\x87R\x97\x81\x89`\x01\t\x9C` \x88\x01\x9D\x8ER\x82`@\x89\x01\x9D\x8E\x8C\x81RQ`\x01\t\x91``\x8A\x01\x92\x83R`@Q\x9E\x8Fa<`\x90a\x16\x8CV[Q\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90a=\xB4W[\x15a=VW\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85a<\xAE\x8F\x97\x88a\x16\xA8V[6\x877Q\x8CQa<\xBE\x90\x83a;\xDEV[\x90\x08\x84RQ\x85Qa<\xCF\x90\x83a;\xDEV[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88Qa=\x0C\x90\x83a;\xDEV[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\ta=#\x90\x83a;\xDEV[\x90\x08\x9CQ\x93Q\x90Q\x90\ta=7\x8C\x83a;\xDEV[\x90\x08\x90\t\x92Q\x90Q\x90\ta=K\x90\x83a;\xDEV[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15a<\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x92P`\x01\x91\x90PV[P\x82\x15a<\x02V[\x94P\x90\x92P`\x01\x91\x90PV[P\x81\x15a;\xF6V[5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xC6WV[\x91_[\x81\x81\x10a>DWPPPPV[a>Ra3\x92\x82\x84\x86a30V[\x81\x92\x91\x01``\x83\x82\x03\x12a\x01\xC6Wa>i\x83a>\x13V[` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x82a>\x89\x91\x86\x01a\x17\x1DV[`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6Wa>\xC3a>\xCB\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\x01\x98\x01a\x17\x1DV[\x92\x16\x88aB\xBAV[\x01a>7V[\x90\x92\x91`\x1F\x19a>\xF9a>\xE3\x83a\x17\x93V[\x92a>\xF1`@Q\x94\x85a\x16\xA8V[\x80\x84Ra\x17\x93V[\x01_[\x81\x81\x10a?\xF7WPP\x80\x93_\x90_[\x81\x81\x10a?\x19WPPPPPV[a?'a#\xA7\x82\x84\x88a\x1B\x9BV[_[\x81\x81\x10a?\xB1WPPPa?Aa#\xC9\x82\x84\x88a\x1B\x9BV[_[\x81\x81\x10a?UWPPP`\x01\x01a?\x0BV[\x80a?c`\x01\x92\x84\x86a2\x05V[a?sa$\x0C`@\x83\x01\x83a1\xADV[\x90\x81a?\x83W[PPP\x01a?CV[a?\xA9\x92a?\x9Aa?\x93\x8Ca1\xF7V[\x9B\x8Da\x1B\xDBV[Q\x89` \x83\x015\x925\x91aD\x8AV[_\x80\x80a?zV[\x80a?\xBF`\x01\x92\x84\x86a1\xE0V[a?\xCFa$\x0C``\x83\x01\x83a1\xADV[\x90\x81a?\xDFW[PPP\x01a?)V[a?\xEF\x92a?\x9Aa?\x93\x8Ca1\xF7V[_\x80\x80a?\xD6V[` \x90`@Qa@\x06\x81a\x16\x8CV[_\x81R_\x83\x82\x01R_`@\x82\x01R``\x80\x82\x01R\x82\x82\x86\x01\x01R\x01a>\xFCV[\x80Q\x91_[\x83\x81\x10a@8WPPPPV[a@La@E\x82\x84a\x1B\xDBV[Q\x84a\x1B\xDBV[Q``\x81\x01\x80QQ\x91` \x01\x90_[\x83\x81\x10a@nWPPPP`\x01\x01a@+V[\x80a@\xABa@\x7F`\x01\x93\x85Qa\x1B\xDBV[Q\x85Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16`@` \x83\x01Q\x92\x01Q\x92aB\xBAV[\x01a@[V[\x15a@\xB9WPV[\x7F\xBD\xE6\x0F`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x15a@\xECWPV[\x7F@\xA0\xD1\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x15aA\x1FWPV[\x7F;\x9Bi\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[` \x83\x01\x92\x91\x83\x83\x11a*/W\x83\x83\x11a\x01\xC6W\x83\x11a\x01\xC6W\x81\x015\x91\x03` \x81\x10aAuWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x81Q\x91\x90`A\x83\x03aA\xB3WaA\xAC\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aHnV[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aBhW\x80aA\xCFWPPV[`\x01\x81\x03aA\xFFW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03aB3WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14aB=WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x90\x91aB\xACa\x178\x93`@\x84R`@\x84\x01\x90a\x17nV[\x91` \x81\x84\x03\x91\x01Ra\x17nV[\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92`@Q\x92\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`@`$\x84\x01R_\x83\x80aC\x16`D\x82\x01\x86a\x17nV[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x19\x07W_\x93aC\xB1W[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03aCvWP\x90\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91aCq`@Q\x92\x83\x92\x83aB\x95V[\x03\x90\xA2V[\x90PaC\xAD`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01aB\x95V[\x03\x90\xFD[\x90\x92P=\x80_\x83>aC\xC3\x81\x83a\x16\xA8V[\x81\x01\x90` \x81\x83\x03\x12a\x01\xC6W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W\x01\x81`\x1F\x82\x01\x12\x15a\x01\xC6W\x80Q\x90aC\xFA\x82a\x16\xCBV[\x92aD\x08`@Q\x94\x85a\x16\xA8V[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xC6W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_aC*V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xC6W\x815aDD\x81a\x17\x93V[\x92aDR`@Q\x94\x85a\x16\xA8V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xC6W` \x01\x90[\x82\x82\x10aDzWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01aDmV[\x93\x94\x92\x91\x90`@\x85\x01R\x83R` \x83\x01RaD\xA4\x81a\x17\x93V[\x91aD\xB2`@Q\x93\x84a\x16\xA8V[\x81\x83R`\x1F\x19aD\xC1\x83a\x17\x93V[\x01_[\x81\x81\x10aE\xC9WPP``\x01\x91\x82R_[\x81\x81\x10aD\xE2WPPPPV[aD\xED\x81\x84Qa\x1B\xDBV[Q\x90aD\xFDa3\x92\x82\x85\x88a30V[\x81\x01\x92\x90`\xA0\x81\x85\x03\x12a\x01\xC6WaE\x14\x81a>\x13V[` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x85aE4\x91\x84\x01a\x17\x1DV[`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x86aET\x91\x85\x01a\x17\x1DV[``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x87aEt\x91\x86\x01aD-V[`\x80\x85\x015\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11a\x01\xC6W`\x01\x98s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96aE\xAC\x92\x01aD-V[`\x80\x87\x01R``\x86\x01R`@\x85\x01R` \x84\x01R\x16\x90R\x01aD\xD5V[`@Q\x90`\xA0\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x07\xBEW` \x92`@R_\x81R``\x83\x82\x01R```@\x82\x01R``\x80\x82\x01R```\x80\x82\x01R\x82\x82\x88\x01\x01R\x01aD\xC4V[` \x81Q\x91\x01Q\x90\x80Qa\x01\0\x81\x11aH\x06W__[\x82\x81\x10aG\xE0WPaF;\x82a\x19bV[\x93__[\x84\x81\x10aFQWPPPPPP`\x01\x91V[\x83\x15aG\xD5WaG\x05aF\xE7\x85`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x94`\x01\x86\x1B\x90\x81\x19\x16\x93\x17\x94\x80aF\xFE\x84\x8Ba\x1B\xDBV[R\x84a\x1B\xDBV[Q\x80\x85\x19\x91[aG\x1CWPP`\x01\x01\x92\x90\x92aF?V[aG\xA9\x81`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x90\x82aG\xBD`\x01\x84\x1B\x92\x83\x19\x16\x93\x8Ba\x1B\xDBV[Q\x16\x15aG\xCCW[P\x80aG\x0BV[\x81\x94\x17\x93aG\xC5V[P_\x96\x94PPPPPV[aG\xEA\x81\x85a\x1B\xDBV[Q\x15aG\xF9W[`\x01\x01aF*V[`\x01\x81\x1B\x90\x91\x17\x90aG\xF1V[\x7F\xA1V\xECF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x91\x90\x82Q\x90_\x93[\x82\x85\x10aHEWPPPV[\x90\x91\x92\x93\x82aHT\x82\x84a\x1B\xDBV[QQ\x14aHgW`\x01\x01\x93\x92\x91\x90aH9V[\x92PPP\x90V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aH\xF2W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x19\x07W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aH\xE8W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x92\x90\x91`@aI\x0C\x82\x86a\x1B\xDBV[Q\x01Q`@aI\x1B\x84\x87a\x1B\xDBV[Q\x01Q\x14aI\xB5WaI2aI:\x91\x85\x93\x95aI\xF8V[\x94\x90\x92aI\xF8V[\x93aIE\x81\x84aJ\x97V[\x92aIP\x86\x84aJ\x97V[` \x86\x01\x91[\x83\x81\x10aI\x92WPPPP[\x83\x81\x10aIoWPPPPV[\x80\x82aI~`\x01\x93\x86Qa\x1B\xDBV[Q\x17aI\x8B\x82\x86Qa\x1B\xDBV[R\x01aIbV[\x80\x82aI\xA1`\x01\x93\x86Qa\x1B\xDBV[Q\x17aI\xAE\x82\x86Qa\x1B\xDBV[R\x01aIVV[\x90\x92P` \x82aI\xC9\x85aI\xF5\x95Qa\x1B\xDBV[Q`\x01\x84\x1B\x17aI\xDA\x86\x83Qa\x1B\xDBV[R\x01\x92`\x01aI\xEA\x83\x86Qa\x1B\xDBV[Q\x91\x1B\x17\x92Qa\x1B\xDBV[RV[\x91`@aJ\x05\x83\x85a\x1B\xDBV[Q\x01Q\x90\x82[\x80\x15\x80\x15\x80aJuW[\x15aJ&Wa*/W_\x19\x01aJ\x0BV[P\x93\x90\x91\x92`\x01\x81\x01\x80\x91\x11a*/W\x92\x81Q\x93[\x84\x81\x14\x15\x80aJ^W[\x15aJXWaJS\x90a1\xF7V[aJ;V[\x93PPPV[P\x81`@aJl\x83\x86a\x1B\xDBV[Q\x01Q\x14aJEV[P_\x19\x82\x01\x82\x81\x11a*/W`@aJ\x8E\x86\x92\x89a\x1B\xDBV[Q\x01Q\x14aJ\x15V[\x90a\x01\0\x03\x90a\x01\0\x82\x11a*/W`\x01\x90\x1B_\x19\x81\x01\x90\x81\x11a*/W_\x19\x90\x19\x91\x1C\x16\x90V\xFE\xA1dsolcC\0\x08&\0\n\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x61010060405260033611610011575f80fd5b5f3560e01c8063155f9f0214611582578063171eac18146115575780631a8473bc146113b55780633f4ba83a146112f85780634f1ef2861461102a57806351945b0614610f9657806352d1902d14610ef75780635c975abb14610eb657806364c12af214610e7c5780636735312214610e625780636bf0a04b14610e12578063715018a614610d565780638456cb5914610cbf5780638b33e20014610c2c5780638da5cb5b14610bda5780639239ac0c14610b52578063a06056f714610b13578063aaf10f4214610ac1578063ad3cb1cc14610a62578063bdeb442d1461090d578063c1b0bed7146108bf578063c44956d114610883578063c4d66de8146103cb578063e35a5d2f1461036f578063ed83cdc7146102c7578063f2fde38b1461029c578063f8adf4c314610248578063fb0ff6801461020c578063fe18ab91146101ca5763ffa1ad7414610163575f80fd5b346101c6575f6003193601126101c6576101c26040516101846040826116a8565b600b81527f322e302e302d72632e3132000000000000000000000000000000000000000000602082015260405191829160208352602083019061176e565b0390f35b5f80fd5b346101c6575f6003193601126101c6576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b346101c6575f6003193601126101c65760207f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb0054604051908152f35b346101c65760206003193601126101c65760206102926004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b6040519015158152f35b346101c65760206003193601126101c6576102c56102b861164d565b6102c0613129565b611993565b005b346101c6575f6003193601126101c6576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f5260205f20905f5b818110610359576101c285610345818703826116a8565b60405191829160208352602083019061173b565b825484526020909301926001928301920161032e565b346101c6575f6003193601126101c65760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101c65760206003193601126101c6576103e461164d565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c16159167ffffffffffffffff82168015908161087b575b6001149081610871575b159081610868575b5061084057818360017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000006104a59516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00556107eb575b506104956132d9565b61049d6132d9565b6102c06132d9565b6104ad6132d9565b6104b56132d9565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156107be5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015610791577fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2001555f7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260405f20600160ff198254161790557f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a161062c6132d9565b6106346132d9565b61063c6132d9565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e25f80a26106cc6117bb565b610769576106d657005b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1005b7f25ae6eaa000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00558361048c565b7ff92ee8a9000000000000000000000000000000000000000000000000000000005f5260045ffd5b90501584610435565b303b15915061042d565b849150610423565b346101c6575f6003193601126101c65760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b346101c65760206003193601126101c6576004355f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00602052602060ff60405f2054166040519015158152f35b346101c6575f6003193601126101c65760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2054905f5b8181106109a257602083604051908152f35b9091600190848216610a0b577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4318301546109fe91613296565b935b811c93929101610990565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154610a5c9190613296565b93610a00565b346101c6575f6003193601126101c6576101c2604051610a836040826116a8565b600581527f352e302e30000000000000000000000000000000000000000000000000000000602082015260405191829160208352602083019061176e565b346101c6575f6003193601126101c657602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b346101c6575f6003193601126101c657602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b346101c65760206003193601126101c65760043567ffffffffffffffff81116101c657608060031982360301126101c657610bb590610b8f611a80565b610b97611af4565b60405190610ba66020836116a8565b5f82525f368137600401611bef565b5f7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d005b346101c6575f6003193601126101c657602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346101c65760206003193601126101c657600435610c48613129565b8015610c9757807f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00557f10635d4c5f018d9f4e7b98e36129195162d83543ee1a9e2fd1dccd1b2523f2e25f80a2005b7f0bf7718c000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c6575f6003193601126101c657610cd7613129565b610cdf611af4565b600160ff197fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b346101c6575f6003193601126101c657610d6e613129565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346101c6575f6003193601126101c657602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101c6575f6003193601126101c65760206102926117bb565b346101c6575f6003193601126101c65760206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152f35b346101c6575f6003193601126101c657602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b346101c6575f6003193601126101c65773ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163003610f6e5760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c6575f6003193601126101c6576040517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025480825260208201907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f20905f5b818110611014576101c285610345818703826116a8565b8254845260209093019260019283019201610ffd565b60406003193601126101c65761103e61164d565b60243567ffffffffffffffff81116101c65761105e90369060040161171d565b9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168030149081156112b6575b50610f6e576110ae613129565b73ffffffffffffffffffffffffffffffffffffffff8116916040517f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f9181611282575b5061112e57837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8592036112575750823b1561122c57807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a28051156111fb576102c591613b17565b50503461120457005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d6020116112ae575b8161129e602093836116a8565b810103126101c6575190856110fd565b3d9150611291565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54161415836110a1565b346101c6575f6003193601126101c657611310613129565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff81161561138d5760ff19167fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b7f8dfc202b000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c65760206003193601126101c65760043567ffffffffffffffff81116101c657366023820112156101c657806004013567ffffffffffffffff81116101c6576024820191602436918360061b0101116101c65790611414613129565b5f915b80831061142057005b61142b8382846117ab565b3592602061143a8284866117ab565b013593841515948581036101c657811561152f5761145881836130e8565b6114ff57602060019495967fa70f4433615e083adf1b97e0c32d9a721c39c26d6dd6c28fe96176ab909f0eca92845f527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900835260405f20905f146114d0578660ff198254161790555b604051908152a2019190611417565b6101007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff8254161790556114c1565b85827f127d335e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7f7af62341000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101c65760406003193601126101c65760243580151581036101c6576102926020916004356130e8565b346101c65760406003193601126101c65760043567ffffffffffffffff81116101c657608060031982360301126101c6576024359067ffffffffffffffff82116101c657366023830112156101c65781600401359067ffffffffffffffff82116101c65760248260051b8401013681116101c6576115fe611a80565b611606611af4565b61160f83611793565b9261161d60405194856116a8565b83526024602084019401935b81851061163d57610bb58484600401611bef565b8435815260209485019401611629565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101c657565b6040810190811067ffffffffffffffff8211176107be57604052565b6080810190811067ffffffffffffffff8211176107be57604052565b90601f601f19910116810190811067ffffffffffffffff8211176107be57604052565b67ffffffffffffffff81116107be57601f01601f191660200190565b9291926116f3826116cb565b9161170160405193846116a8565b8294818452818301116101c6578281602093845f960137010152565b9080601f830112156101c657816020611738933591016116e7565b90565b90602080835192838152019201905f5b8181106117585750505090565b825184526020938401939092019160010161174b565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b67ffffffffffffffff81116107be5760051b60200190565b91908110156107915760061b0190565b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611907575f91611912575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa908115611907575f916118cf575090565b90506020813d6020116118ff575b816118ea602093836116a8565b810103126101c6575180151581036101c65790565b3d91506118dd565b6040513d5f823e3d90fd5b90506020813d60201161195a575b8161192d602093836116a8565b810103126101c6575173ffffffffffffffffffffffffffffffffffffffff811681036101c6576020611876565b3d9150611920565b9061196c82611793565b61197960405191826116a8565b828152601f196119898294611793565b0190602036910137565b73ffffffffffffffffffffffffffffffffffffffff168015611a545773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b7f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c611acc5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d565b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416611b1f57565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101c6570180359067ffffffffffffffff82116101c657602001918160051b360383136101c657565b91908110156107915760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603018212156101c6570190565b80518210156107915760209160051b010190565b9060a052611bfd8180611b47565b90505f60c05280156130c0577f3c4f446b1611bef8becaa7ce2dd3faff29493fef5f93c5c3f81ec61f107dcb00546060830135148015613093575b1561306357611c4681611962565b91611c4f613195565b50604051611c5c81611670565b5f81525f6020820152925f905f6080525f935f905b8082106128115750506020815160051b9101209360a05151155f146127a357611cbf90611cb6865b611cb0611ca96020880188613245565b36916116e7565b90614183565b909391936141bd565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2016911690808203612775575050611cfd8280611b47565b60405190602082019281608084017fbc4edaeea98f6501547cba34cf94d8b59631e7f86cf1a47119ba6f17c29a517c8652606088013560408601526060808601525260a0830160a08360051b85010192825f907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603015b838310612598575050505050509181611d9b5f9360209503601f1981018352826116a8565b604051918291518091835e8101838152039060025afa15611907575f51611dc56040840184613245565b806004116101c6577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000001680820361256a57505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b156101c6575f92611ebf92604051958694859384937fab750e7500000000000000000000000000000000000000000000000000000000855260606004860152606485019161340a565b907f35082a39ecba805a1ca4fdac7eb364b20afce2abf14413ebd910233332ee45c96024840152604483015203915afa801561190757612556575b5080612486575b506080511561244f5780611f1491611b47565b60a0515191929091156121d4575090611f309160805191613ed1565b80518060a051510361219657611f4581611962565b9060c0515b81811061215c575060015b8181106120ac575060c0515b818110611fa257505050611f789060a05190614026565b7f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce60c05160c051a2565b6060611fae8286611bdb565b51015180519060c0515b828110611fca57505050600101611f61565b6060611fd68284611bdb565b5101518051908660c0515b878b8b86841061206d57505050505050506080611ffe8284611bdb565b51015180519060c0515b88888b85841061202057505050505050600101611fb8565b83926120606120679361204f6120469461203c6001998b611bdb565b5195868092614831565b92831415614117565b6120598d84611bdb565b5192611bdb565b51116140e4565b01612008565b6120606120a49361205960019761209e6120959661208b8a8d611bdb565b5197888092614831565b928314156140b1565b84611bdb565b018790611fe1565b60406120c46120bd8360a051611bdb565b5186611bdb565b5101515f19820182811161212b576120ea6120e360409260a051611bdb565b5187611bdb565b51015181106120fc5750600101611f55565b7ff3474dc20000000000000000000000000000000000000000000000000000000060c05152600452602460c051fd5b7f4e487b710000000000000000000000000000000000000000000000000000000060c051526011600452602460c051fd5b6121688160a051611bdb565b5190828210806121c2575b15612196576001810180821161212b5761218f60019386611bdb565b5201611f4a565b7f9a12c5100000000000000000000000000000000000000000000000000000000060c05152600460c051fd5b506121cd8285611bdb565b5115612173565b1561238657906121e79160805191613ed1565b8051610100811161235757506040516121ff81611670565b60608152602081016060815282519061221782611962565b835261222282611962565b905260c0515b81811061227a57505061223a90614614565b901561224e5761224991614026565b611f78565b7f813c76a00000000000000000000000000000000000000000000000000000000060c05152600460c051fd5b60606122868286611bdb565b51015180519060c0515b8281106122a257505050600101612228565b6122ac8183611bdb565b51606081015180519060c0515b87898b8d86851061232657505050505050506080015180519060c0515b87878a8c8685106122f05750505050505050600101612290565b918493916123209361231b6123076001988a611bdb565b516123128184614831565b95861415614117565b6148fd565b016122d6565b918493916123519361231b61233d6001988a611bdb565b516123488184614831565b948514156140b1565b016122b9565b7f895022880000000000000000000000000000000000000000000000000000000060c05152600452602460c051fd5b60c0515b81811061239957505050611f78565b6123ad6123a7828486611b9b565b80611b47565b60c0515b818110612426575050506123d36123c9828486611b9b565b6020810190611b47565b60c0515b8181106123e95750505060010161238a565b8061242060206123fc6001948688613205565b61241661240c60408301836131ad565b6040810190611b47565b9290910135613e34565b016123d7565b80612449602061243960019486886131e0565b61241661240c60608301836131ad565b016123b1565b505060a0515115611f78577f9a12c5100000000000000000000000000000000000000000000000000000000060c05152600460c051fd5b8060c051527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff604060c051205416612527576020817f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d29260c051527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9038252604060c05120600160ff19825416179055604051908152a15f611f01565b7fdb788c2b0000000000000000000000000000000000000000000000000000000060c05152600452602460c051fd5b5f612560916116a8565b5f60c0525f611efa565b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9091929394957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608882030186528635828112156101c65783019060a08101916125e1818061392c565b809460a085525260c0830160c08560051b85010194825f5b8281106126f6575050505050612612602082018261392c565b93908382036020850152848252602082019060208660051b8401019581935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b83811061269857505050506040808501359086015250505060608082013590830152608090810135910152956020908101950193926001019190611d76565b90919293949598601f198482030187528935868112156101c65760206126e760019360606126d988859601803584528581013586850152604081019061397f565b918160408201520190613aa0565b9b019701959493929101612659565b909192939660208061276883608061275a61273a8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f60019a03018d528a61397f565b80358452858101358685015260408101356040850152606081019061397f565b918160608201520190613aa0565b99019501939291016125f9565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60405160208101906060810187835260408083015260a051518091526080820190602060a05101905f5b8181106127fb5750505091816127f3611cb693611cbf969503601f1981018352826116a8565b519020611c99565b82518452602093840193909201916001016127cd565b9095612827876128218780611b47565b90611b9b565b60e0525f5f5f9161283c60e05160e051611b47565b905061284781611962565b9061285181611962565b905f5b818110612e7e57505061286e602060e0510160e051611b47565b91905061287a82611962565b9061288483611962565b925f905b808210612a645750506128e2926128fe7f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc3295936128f061290c94604051978897608060e0510135895260a060208a015260a089019061173b565b90878203604089015261173b565b90858203606087015261173b565b90838203608085015261173b565b0390a180612a5c575b5080612a0a575b505060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc060e051360301126101c6576040519061295982611670565b604060e0510135825260208201606060e051013581526040519261297c84611670565b5f84525f60208501526129928151835190613665565b156129d257916129b191600194936020835193015190519151926136c7565b6020830152815296608060e05101356129ca8286611bdb565b520190611c71565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b608051018060805111612a2f57608052612a26575b5f8061291c565b60019550612a1f565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b95505f612915565b909550612a8286612a7c602060e0510160e051611b47565b90613205565b95612a915f60208901356130e8565b612e4a5760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054612ae5816131f7565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90055908835915f5b828110612d665750507f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b14612bf3575b509660408101612b7961240c82846131ad565b80612bbc575b505081612b9b612b94602093600196956131ad565b8235613441565b8035612ba78489611bdb565b520135612bb48288611bdb565b520190612888565b909a612bc7906131f7565b9a8c612b7f576020929c50612b94612be66001969593612b9b93613370565b9d93505081939450612b7f565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154680100000000000000008110156107be5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90155811015610791577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f018290557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431015490612cd690612ccf908390613296565b9180613296565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156107be5760018101807f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90255811015610791577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f5260205f2001555f612b66565b9092600190818516612df3577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d431830154612de891613296565b935b811c9101612b0d565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f830154612e449190613296565b93612dea565b6020877f597abd20000000000000000000000000000000000000000000000000000000005f5201356004525f60245260445ffd5b612e9681612e9060e05160e051611b47565b906131e0565b612ea5600160208301356130e8565b61302e576040810135612ee3815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90360205260ff60405f20541690565b15613003575080355f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260ff60405f205416612fd75780355f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0060205260405f20600160ff1982541617905560608101612f6561240c82846131ad565b80612fa0575b505081612f80612b94602093600196956131ad565b8035612f8c8489611bdb565b520135612f998286611bdb565b5201612854565b9098612fab906131f7565b988a612f6b576020929a50612b94612fca6001969593612f8093613370565b9b93505081939450612f6b565b7f39a940c5000000000000000000000000000000000000000000000000000000005f523560045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6020907f597abd20000000000000000000000000000000000000000000000000000000005f520135600452600160245260445ffd5b6060827fc93ee163000000000000000000000000000000000000000000000000000000005f52013560045260245ffd5b507fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06606083013514611c38565b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b5f9081527f4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a629006020526040902060ff911561312157541690565b5460081c1690565b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416330361316957565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b604051906131a282611670565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81813603018212156101c6570190565b90821015610791576117389160051b8101906131ad565b5f198114612a2f5760010190565b91908110156107915760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1813603018212156101c6570190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101c6570180359067ffffffffffffffff82116101c6576020019181360383136101c657565b5f90602092604051908482019283526040820152604081526132b96060826116a8565b604051918291518091835e8101838152039060025afa15611907575f5190565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c161561330857565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b91908110156107915760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc1813603018212156101c6570190565b5f92918390815b8181106133845750505050565b61339c613392828487613330565b6020810190613245565b8461340657806080116101c65760206133ba6060840135838561414a565b15928315936133dd575b5050506133d357600101613377565b5060019450505050565b91955091505f948060a0116101c6576133fc925060808201359161414a565b15155f80806133c4565b8480fd5b601f8260209493601f1993818652868601375f8582860101520116010190565b60409061173894928152816020820152019161340a565b9061344c8180611b47565b5f5b8181106135f6575050506134656020820182611b47565b5f5b8181106135875750505061347e6040820182611b47565b5f5b81811061351857505050806060613498920190611b47565b90915f5b8281106134a95750505050565b6134b4818486613330565b359060028210156101c65760018092146134cf575b0161349c565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c6134ff61339284888a613330565b90613510604051928392878461342a565b0390a26134c9565b613523818385613330565b359060028210156101c657600180921461353e575b01613480565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd5961356e613392848789613330565b9061357f604051928392878461342a565b0390a2613538565b613592818385613330565b359060028210156101c65760018092146135ad575b01613467565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f36135dd613392848789613330565b906135ee604051928392878461342a565b0390a26135a7565b613601818385613330565b359060028210156101c657600180921461361c575b0161344e565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae661364c613392848789613330565b9061365d604051928392878461342a565b0390a2613616565b801580156136b7575b80156136af575b801561369f575b613699576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d01982101561367c565b508115613675565b506401000003d01981101561366e565b92939290915f908083036139125750506401000003d0195f9483086136f057505090505f905f90565b6401000003d0198280096401000003d0195f8180808086880960040995800960030908906401000003d01991828480088303838111612a2f57839081838009089384840392848411612a2f5784809180096008098403848111612a2f5784600193818096819508900908940960020991905b8215158381613901575b50806138f9575b1561389b579084939293906001936401000003d0199187965b80156138485780840496809861381b576401000003d0199088096401000003d019036401000003d0198111612a2f576401000003d019908a9608979381978281029281840414901517156137ee57906137e491613bde565b959293969561378c565b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b509450945050938061386e5750906401000003d019809281808780098092099509900990565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b506001613773565b6401000003d019915014155f61376c565b613926936401000003d01993969296613beb565b91613762565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1823603018112156101c657016020813591019167ffffffffffffffff82116101c6578160051b360383136101c657565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81823603018112156101c6570190565b906020838281520160208260051b85010193835f917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc182360301905b8584106139fe575050505050505090565b90919293949596601f198282030186528735838112156101c6578401803560028110156101c657825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1823603018112156101c657016020813591019067ffffffffffffffff81116101c65780360382136101c657613a91602092839260408681866001990152019161340a565b990197960194019291906139ed565b61173891613b09613afe613ae3613ac8613aba868061392c565b6080875260808701916139b1565b613ad5602087018761392c565b9086830360208801526139b1565b613af0604086018661392c565b9085830360408701526139b1565b92606081019061392c565b9160608185039101526139b1565b905f8091602081519101845af48080613bcb575b15613b4b5750506040513d81523d5f602083013e60203d82010160405290565b15613b925773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d15613ba3576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d151580613b2b5750813b1515613b2b565b91908203918211612a2f57565b949291851580613e0b575b613dff57801580613df7575b613ded57604051608091613c1683836116a8565b823683378615613dc05786948580928180600180098087529781896001099c602088019d8e5282604089019d8e8c8152516001099160608a019283526040519e8f613c609061168c565b5190098d525190099460208b019586525190099860408901998a52519009606087019081528651885114801590613db4575b15613d5657849283808093816040519c85613cae8f97886116a8565b368737518c51613cbe9083613bde565b90088452518551613ccf9083613bde565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a525180098851613d0c9083613bde565b90088180875185519009600209613d239083613bde565b90089c51935190519009613d378c83613bde565b90089009925190519009613d4b9083613bde565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b50815181511415613c92565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5092506001919050565b508215613c02565b94509092506001919050565b508115613bf6565b359073ffffffffffffffffffffffffffffffffffffffff821682036101c657565b915f5b818110613e445750505050565b613e52613392828486613330565b819291016060838203126101c657613e6983613e13565b602084013567ffffffffffffffff81116101c65782613e8991860161171d565b604085013567ffffffffffffffff81116101c657613ec3613ecb9473ffffffffffffffffffffffffffffffffffffffff926001980161171d565b9216886142ba565b01613e37565b909291601f19613ef9613ee383611793565b92613ef160405194856116a8565b808452611793565b015f5b818110613ff757505080935f905f5b818110613f19575050505050565b613f276123a7828488611b9b565b5f5b818110613fb157505050613f416123c9828488611b9b565b5f5b818110613f5557505050600101613f0b565b80613f636001928486613205565b613f7361240c60408301836131ad565b9081613f83575b50505001613f43565b613fa992613f9a613f938c6131f7565b9b8d611bdb565b5189602083013592359161448a565b5f8080613f7a565b80613fbf60019284866131e0565b613fcf61240c60608301836131ad565b9081613fdf575b50505001613f29565b613fef92613f9a613f938c6131f7565b5f8080613fd6565b6020906040516140068161168c565b5f81525f838201525f604082015260608082015282828601015201613efc565b8051915f5b8381106140385750505050565b61404c6140458284611bdb565b5184611bdb565b516060810180515191602001905f5b83811061406e575050505060010161402b565b806140ab61407f6001938551611bdb565b5185519073ffffffffffffffffffffffffffffffffffffffff81511660406020830151920151926142ba565b0161405b565b156140b95750565b7fbde60f60000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b156140ec5750565b7f40a0d1c1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b1561411f5750565b7f3b9b6984000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b602083019291838311612a2f578383116101c65783116101c657810135910360208110614175575090565b5f199060200360031b1b1690565b81519190604183036141b3576141ac9250602082015190606060408401519301515f1a9061486e565b9192909190565b50505f9160029190565b600481101561426857806141cf575050565b600181036141ff577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002810361423357507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b60031461423d5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b90916142ac6117389360408452604084019061176e565b91602081840391015261176e565b919073ffffffffffffffffffffffffffffffffffffffff1692604051927f33a892030000000000000000000000000000000000000000000000000000000084526004840152604060248401525f8380614316604482018661176e565b038183885af1928315611907575f936143b1575b508251602084012081516020830120036143765750907fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf19161437160405192839283614295565b0390a2565b90506143ad6040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401614295565b0390fd5b9092503d805f833e6143c381836116a8565b8101906020818303126101c65780519067ffffffffffffffff82116101c6570181601f820112156101c6578051906143fa826116cb565b9261440860405194856116a8565b828452602083830101116101c657815f9260208093018386015e83010152915f61432a565b9080601f830112156101c657813561444481611793565b9261445260405194856116a8565b81845260208085019260051b8201019283116101c657602001905b82821061447a5750505090565b813581526020918201910161446d565b93949291906040850152835260208301526144a481611793565b916144b260405193846116a8565b818352601f196144c183611793565b015f5b8181106145c95750506060019182525f5b8181106144e25750505050565b6144ed818451611bdb565b51906144fd613392828588613330565b8101929060a0818503126101c65761451481613e13565b602082013567ffffffffffffffff81116101c6578561453491840161171d565b604083013567ffffffffffffffff81116101c6578661455491850161171d565b606084013567ffffffffffffffff81116101c6578761457491860161442d565b60808501359467ffffffffffffffff86116101c65760019873ffffffffffffffffffffffffffffffffffffffff966145ac920161442d565b6080870152606086015260408501526020840152169052016144d5565b6040519060a082019180831067ffffffffffffffff8411176107be576020926040525f81526060838201526060604082015260608082015260606080820152828288010152016144c4565b602081519101519080516101008111614806575f5f5b8281106147e0575061463b82611962565b935f5f5b84811061465157505050505050600191565b83156147d5576147056146e7856001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b946001861b90811916931794806146fe848b611bdb565b5284611bdb565b51808519915b61471c57505060010192909261463f565b6147a9816001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b90826147bd6001841b92831916938b611bdb565b5116156147cc575b508061470b565b819417936147c5565b505f96945050505050565b6147ea8185611bdb565b51156147f9575b60010161462a565b6001811b909117906147f1565b7fa156ec46000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b91908251905f935b82851061484557505050565b90919293826148548284611bdb565b5151146148675760010193929190614839565b9250505090565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116148f2579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15611907575f5173ffffffffffffffffffffffffffffffffffffffff8116156148e857905f905f90565b505f906001905f90565b5050505f9160039190565b929091604061490c8286611bdb565b510151604061491b8487611bdb565b510151146149b55761493261493a918593956149f8565b9490926149f8565b936149458184614a97565b926149508684614a97565b60208601915b83811061499257505050505b83811061496f5750505050565b808261497e6001938651611bdb565b511761498b828651611bdb565b5201614962565b80826149a16001938651611bdb565b51176149ae828651611bdb565b5201614956565b9092506020826149c9856149f59551611bdb565b516001841b176149da868351611bdb565b52019260016149ea838651611bdb565b51911b179251611bdb565b52565b916040614a058385611bdb565b51015190825b8015801580614a75575b15614a2657612a2f575f1901614a0b565b509390919260018101809111612a2f57928151935b8481141580614a5e575b15614a5857614a53906131f7565b614a3b565b93505050565b50816040614a6c8386611bdb565b51015114614a45565b505f198201828111612a2f576040614a8e869289611bdb565b51015114614a15565b9061010003906101008211612a2f576001901b5f198101908111612a2f575f199019911c169056fea164736f6c6343000826000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R`\x036\x11a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x15_\x9F\x02\x14a\x15\x82W\x80c\x17\x1E\xAC\x18\x14a\x15WW\x80c\x1A\x84s\xBC\x14a\x13\xB5W\x80c?K\xA8:\x14a\x12\xF8W\x80cO\x1E\xF2\x86\x14a\x10*W\x80cQ\x94[\x06\x14a\x0F\x96W\x80cR\xD1\x90-\x14a\x0E\xF7W\x80c\\\x97Z\xBB\x14a\x0E\xB6W\x80cd\xC1*\xF2\x14a\x0E|W\x80cg51\"\x14a\x0EbW\x80ck\xF0\xA0K\x14a\x0E\x12W\x80cqP\x18\xA6\x14a\rVW\x80c\x84V\xCBY\x14a\x0C\xBFW\x80c\x8B3\xE2\0\x14a\x0C,W\x80c\x8D\xA5\xCB[\x14a\x0B\xDAW\x80c\x929\xAC\x0C\x14a\x0BRW\x80c\xA0`V\xF7\x14a\x0B\x13W\x80c\xAA\xF1\x0FB\x14a\n\xC1W\x80c\xAD<\xB1\xCC\x14a\nbW\x80c\xBD\xEBD-\x14a\t\rW\x80c\xC1\xB0\xBE\xD7\x14a\x08\xBFW\x80c\xC4IV\xD1\x14a\x08\x83W\x80c\xC4\xD6m\xE8\x14a\x03\xCBW\x80c\xE3Z]/\x14a\x03oW\x80c\xED\x83\xCD\xC7\x14a\x02\xC7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9CW\x80c\xF8\xAD\xF4\xC3\x14a\x02HW\x80c\xFB\x0F\xF6\x80\x14a\x02\x0CW\x80c\xFE\x18\xAB\x91\x14a\x01\xCAWc\xFF\xA1\xADt\x14a\x01cW_\x80\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x01\xC2`@Qa\x01\x84`@\x82a\x16\xA8V[`\x0B\x81R\x7F2.0.0-rc.12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17nV[\x03\x90\xF3[_\x80\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` \x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W` a\x02\x92`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6Wa\x02\xC5a\x02\xB8a\x16MV[a\x02\xC0a1)V[a\x19\x93V[\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R` _ \x90_[\x81\x81\x10a\x03YWa\x01\xC2\x85a\x03E\x81\x87\x03\x82a\x16\xA8V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17;V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03.V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6Wa\x03\xE4a\x16MV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16\x15\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x15\x90\x81a\x08{W[`\x01\x14\x90\x81a\x08qW[\x15\x90\x81a\x08hW[Pa\x08@W\x81\x83`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x04\xA5\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x07\xEBW[Pa\x04\x95a2\xD9V[a\x04\x9Da2\xD9V[a\x02\xC0a2\xD9V[a\x04\xADa2\xD9V[a\x04\xB5a2\xD9V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xBEW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x07\x91W\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x01U_\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x06,a2\xD9V[a\x064a2\xD9V[a\x06<a2\xD9V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2_\x80\xA2a\x06\xCCa\x17\xBBV[a\x07iWa\x06\xD6W\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[\x7F%\xAEn\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x83a\x04\x8CV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x84a\x045V[0;\x15\x91Pa\x04-V[\x84\x91Pa\x04#V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ T\x90_[\x81\x81\x10a\t\xA2W` \x83`@Q\x90\x81R\xF3[\x90\x91`\x01\x90\x84\x82\x16a\n\x0BW\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta\t\xFE\x91a2\x96V[\x93[\x81\x1C\x93\x92\x91\x01a\t\x90V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta\n\\\x91\x90a2\x96V[\x93a\n\0V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x01\xC2`@Qa\n\x83`@\x82a\x16\xA8V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17nV[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W`\x80`\x03\x19\x826\x03\x01\x12a\x01\xC6Wa\x0B\xB5\x90a\x0B\x8Fa\x1A\x80V[a\x0B\x97a\x1A\xF4V[`@Q\x90a\x0B\xA6` \x83a\x16\xA8V[_\x82R_6\x817`\x04\x01a\x1B\xEFV[_\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045a\x0CHa1)V[\x80\x15a\x0C\x97W\x80\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0U\x7F\x10c]L_\x01\x8D\x9FN{\x98\xE3a)\x19Qb\xD85C\xEE\x1A\x9E/\xD1\xDC\xCD\x1B%#\xF2\xE2_\x80\xA2\0[\x7F\x0B\xF7q\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x0C\xD7a1)V[a\x0C\xDFa\x1A\xF4V[`\x01`\xFF\x19\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\rna1)V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` a\x02\x92a\x17\xBBV[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0FnW` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6W`@Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02T\x80\x82R` \x82\x01\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x90_[\x81\x81\x10a\x10\x14Wa\x01\xC2\x85a\x03E\x81\x87\x03\x82a\x16\xA8V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x0F\xFDV[`@`\x03\x196\x01\x12a\x01\xC6Wa\x10>a\x16MV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6Wa\x10^\x906\x90`\x04\x01a\x17\x1DV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a\x12\xB6W[Pa\x0FnWa\x10\xAEa1)V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a\x12\x82W[Pa\x11.W\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a\x12WWP\x82;\x15a\x12,W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a\x11\xFBWa\x02\xC5\x91a;\x17V[PP4a\x12\x04W\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a\x12\xAEW[\x81a\x12\x9E` \x93\x83a\x16\xA8V[\x81\x01\x03\x12a\x01\xC6WQ\x90\x85a\x10\xFDV[=\x91Pa\x12\x91V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a\x10\xA1V[4a\x01\xC6W_`\x03\x196\x01\x12a\x01\xC6Wa\x13\x10a1)V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x81\x16\x15a\x13\x8DW`\xFF\x19\x16\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[\x7F\x8D\xFC +\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W` `\x03\x196\x01\x12a\x01\xC6W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W6`#\x82\x01\x12\x15a\x01\xC6W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W`$\x82\x01\x91`$6\x91\x83`\x06\x1B\x01\x01\x11a\x01\xC6W\x90a\x14\x14a1)V[_\x91[\x80\x83\x10a\x14 W\0[a\x14+\x83\x82\x84a\x17\xABV[5\x92` a\x14:\x82\x84\x86a\x17\xABV[\x015\x93\x84\x15\x15\x94\x85\x81\x03a\x01\xC6W\x81\x15a\x15/Wa\x14X\x81\x83a0\xE8V[a\x14\xFFW` `\x01\x94\x95\x96\x7F\xA7\x0FD3a^\x08:\xDF\x1B\x97\xE0\xC3-\x9Ar\x1C9\xC2mm\xD6\xC2\x8F\xE9av\xAB\x90\x9F\x0E\xCA\x92\x84_R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0\x83R`@_ \x90_\x14a\x14\xD0W\x86`\xFF\x19\x82T\x16\x17\x90U[`@Q\x90\x81R\xA2\x01\x91\x90a\x14\x17V[a\x01\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x82T\x16\x17\x90Ua\x14\xC1V[\x85\x82\x7F\x12}3^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7Fz\xF6#A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xC6W`@`\x03\x196\x01\x12a\x01\xC6W`$5\x80\x15\x15\x81\x03a\x01\xC6Wa\x02\x92` \x91`\x045a0\xE8V[4a\x01\xC6W`@`\x03\x196\x01\x12a\x01\xC6W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W`\x80`\x03\x19\x826\x03\x01\x12a\x01\xC6W`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W6`#\x83\x01\x12\x15a\x01\xC6W\x81`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W`$\x82`\x05\x1B\x84\x01\x016\x81\x11a\x01\xC6Wa\x15\xFEa\x1A\x80V[a\x16\x06a\x1A\xF4V[a\x16\x0F\x83a\x17\x93V[\x92a\x16\x1D`@Q\x94\x85a\x16\xA8V[\x83R`$` \x84\x01\x94\x01\x93[\x81\x85\x10a\x16=Wa\x0B\xB5\x84\x84`\x04\x01a\x1B\xEFV[\x845\x81R` \x94\x85\x01\x94\x01a\x16)V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xC6WV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\xBEW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x16\xF3\x82a\x16\xCBV[\x91a\x17\x01`@Q\x93\x84a\x16\xA8V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xC6W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xC6W\x81` a\x178\x935\x91\x01a\x16\xE7V[\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x17XWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17KV[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xBEW`\x05\x1B` \x01\x90V[\x91\x90\x81\x10\x15a\x07\x91W`\x06\x1B\x01\x90V[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x19\x07W_\x91a\x19\x12W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a\x19\x07W_\x91a\x18\xCFWP\x90V[\x90P` \x81=` \x11a\x18\xFFW[\x81a\x18\xEA` \x93\x83a\x16\xA8V[\x81\x01\x03\x12a\x01\xC6WQ\x80\x15\x15\x81\x03a\x01\xC6W\x90V[=\x91Pa\x18\xDDV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x19ZW[\x81a\x19-` \x93\x83a\x16\xA8V[\x81\x01\x03\x12a\x01\xC6WQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01\xC6W` a\x18vV[=\x91Pa\x19 V[\x90a\x19l\x82a\x17\x93V[a\x19y`@Q\x91\x82a\x16\xA8V[\x82\x81R`\x1F\x19a\x19\x89\x82\x94a\x17\x93V[\x01\x90` 6\x91\x017V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a\x1ATWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x1A\xCCW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]V[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a\x1B\x1FWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01\xC6WV[\x91\x90\x81\x10\x15a\x07\x91W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[\x80Q\x82\x10\x15a\x07\x91W` \x91`\x05\x1B\x01\x01\x90V[\x90`\xA0Ra\x1B\xFD\x81\x80a\x1BGV[\x90P_`\xC0R\x80\x15a0\xC0W\x7F<ODk\x16\x11\xBE\xF8\xBE\xCA\xA7\xCE-\xD3\xFA\xFF)I?\xEF_\x93\xC5\xC3\xF8\x1E\xC6\x1F\x10}\xCB\0T``\x83\x015\x14\x80\x15a0\x93W[\x15a0cWa\x1CF\x81a\x19bV[\x91a\x1COa1\x95V[P`@Qa\x1C\\\x81a\x16pV[_\x81R_` \x82\x01R\x92_\x90_`\x80R_\x93_\x90[\x80\x82\x10a(\x11WPP` \x81Q`\x05\x1B\x91\x01 \x93`\xA0QQ\x15_\x14a'\xA3Wa\x1C\xBF\x90a\x1C\xB6\x86[a\x1C\xB0a\x1C\xA9` \x88\x01\x88a2EV[6\x91a\x16\xE7V[\x90aA\x83V[\x90\x93\x91\x93aA\xBDV[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a'uWPPa\x1C\xFD\x82\x80a\x1BGV[`@Q\x90` \x82\x01\x92\x81`\x80\x84\x01\x7F\xBCN\xDA\xEE\xA9\x8Fe\x01T|\xBA4\xCF\x94\xD8\xB5\x961\xE7\xF8l\xF1\xA4q\x19\xBAo\x17\xC2\x9AQ|\x86R``\x88\x015`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01`\xA0\x83`\x05\x1B\x85\x01\x01\x92\x82_\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01[\x83\x83\x10a%\x98WPPPPPP\x91\x81a\x1D\x9B_\x93` \x95\x03`\x1F\x19\x81\x01\x83R\x82a\x16\xA8V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x19\x07W_Qa\x1D\xC5`@\x84\x01\x84a2EV[\x80`\x04\x11a\x01\xC6W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a%jWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x01\xC6W_\x92a\x1E\xBF\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a4\nV[\x90\x7F5\x08*9\xEC\xBA\x80Z\x1C\xA4\xFD\xAC~\xB3d\xB2\n\xFC\xE2\xAB\xF1D\x13\xEB\xD9\x10#32\xEEE\xC9`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a\x19\x07Wa%VW[P\x80a$\x86W[P`\x80Q\x15a$OW\x80a\x1F\x14\x91a\x1BGV[`\xA0QQ\x91\x92\x90\x91\x15a!\xD4WP\x90a\x1F0\x91`\x80Q\x91a>\xD1V[\x80Q\x80`\xA0QQ\x03a!\x96Wa\x1FE\x81a\x19bV[\x90`\xC0Q[\x81\x81\x10a!\\WP`\x01[\x81\x81\x10a \xACWP`\xC0Q[\x81\x81\x10a\x1F\xA2WPPPa\x1Fx\x90`\xA0Q\x90a@&V[\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCE`\xC0Q`\xC0Q\xA2V[``a\x1F\xAE\x82\x86a\x1B\xDBV[Q\x01Q\x80Q\x90`\xC0Q[\x82\x81\x10a\x1F\xCAWPPP`\x01\x01a\x1FaV[``a\x1F\xD6\x82\x84a\x1B\xDBV[Q\x01Q\x80Q\x90\x86`\xC0Q[\x87\x8B\x8B\x86\x84\x10a mWPPPPPPP`\x80a\x1F\xFE\x82\x84a\x1B\xDBV[Q\x01Q\x80Q\x90`\xC0Q[\x88\x88\x8B\x85\x84\x10a  WPPPPPP`\x01\x01a\x1F\xB8V[\x83\x92a `a g\x93a Oa F\x94a <`\x01\x99\x8Ba\x1B\xDBV[Q\x95\x86\x80\x92aH1V[\x92\x83\x14\x15aA\x17V[a Y\x8D\x84a\x1B\xDBV[Q\x92a\x1B\xDBV[Q\x11a@\xE4V[\x01a \x08V[a `a \xA4\x93a Y`\x01\x97a \x9Ea \x95\x96a \x8B\x8A\x8Da\x1B\xDBV[Q\x97\x88\x80\x92aH1V[\x92\x83\x14\x15a@\xB1V[\x84a\x1B\xDBV[\x01\x87\x90a\x1F\xE1V[`@a \xC4a \xBD\x83`\xA0Qa\x1B\xDBV[Q\x86a\x1B\xDBV[Q\x01Q_\x19\x82\x01\x82\x81\x11a!+Wa \xEAa \xE3`@\x92`\xA0Qa\x1B\xDBV[Q\x87a\x1B\xDBV[Q\x01Q\x81\x10a \xFCWP`\x01\x01a\x1FUV[\x7F\xF3GM\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04R`$`\xC0Q\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x11`\x04R`$`\xC0Q\xFD[a!h\x81`\xA0Qa\x1B\xDBV[Q\x90\x82\x82\x10\x80a!\xC2W[\x15a!\x96W`\x01\x81\x01\x80\x82\x11a!+Wa!\x8F`\x01\x93\x86a\x1B\xDBV[R\x01a\x1FJV[\x7F\x9A\x12\xC5\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04`\xC0Q\xFD[Pa!\xCD\x82\x85a\x1B\xDBV[Q\x15a!sV[\x15a#\x86W\x90a!\xE7\x91`\x80Q\x91a>\xD1V[\x80Qa\x01\0\x81\x11a#WWP`@Qa!\xFF\x81a\x16pV[``\x81R` \x81\x01``\x81R\x82Q\x90a\"\x17\x82a\x19bV[\x83Ra\"\"\x82a\x19bV[\x90R`\xC0Q[\x81\x81\x10a\"zWPPa\":\x90aF\x14V[\x90\x15a\"NWa\"I\x91a@&V[a\x1FxV[\x7F\x81<v\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04`\xC0Q\xFD[``a\"\x86\x82\x86a\x1B\xDBV[Q\x01Q\x80Q\x90`\xC0Q[\x82\x81\x10a\"\xA2WPPP`\x01\x01a\"(V[a\"\xAC\x81\x83a\x1B\xDBV[Q``\x81\x01Q\x80Q\x90`\xC0Q[\x87\x89\x8B\x8D\x86\x85\x10a#&WPPPPPPP`\x80\x01Q\x80Q\x90`\xC0Q[\x87\x87\x8A\x8C\x86\x85\x10a\"\xF0WPPPPPPP`\x01\x01a\"\x90V[\x91\x84\x93\x91a# \x93a#\x1Ba#\x07`\x01\x98\x8Aa\x1B\xDBV[Qa#\x12\x81\x84aH1V[\x95\x86\x14\x15aA\x17V[aH\xFDV[\x01a\"\xD6V[\x91\x84\x93\x91a#Q\x93a#\x1Ba#=`\x01\x98\x8Aa\x1B\xDBV[Qa#H\x81\x84aH1V[\x94\x85\x14\x15a@\xB1V[\x01a\"\xB9V[\x7F\x89P\"\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04R`$`\xC0Q\xFD[`\xC0Q[\x81\x81\x10a#\x99WPPPa\x1FxV[a#\xADa#\xA7\x82\x84\x86a\x1B\x9BV[\x80a\x1BGV[`\xC0Q[\x81\x81\x10a$&WPPPa#\xD3a#\xC9\x82\x84\x86a\x1B\x9BV[` \x81\x01\x90a\x1BGV[`\xC0Q[\x81\x81\x10a#\xE9WPPP`\x01\x01a#\x8AV[\x80a$ ` a#\xFC`\x01\x94\x86\x88a2\x05V[a$\x16a$\x0C`@\x83\x01\x83a1\xADV[`@\x81\x01\x90a\x1BGV[\x92\x90\x91\x015a>4V[\x01a#\xD7V[\x80a$I` a$9`\x01\x94\x86\x88a1\xE0V[a$\x16a$\x0C``\x83\x01\x83a1\xADV[\x01a#\xB1V[PP`\xA0QQ\x15a\x1FxW\x7F\x9A\x12\xC5\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04`\xC0Q\xFD[\x80`\xC0QR\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@`\xC0Q T\x16a%'W` \x81\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x92`\xC0QR\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03\x82R`@`\xC0Q `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA1_a\x1F\x01V[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0QR`\x04R`$`\xC0Q\xFD[_a%`\x91a\x16\xA8V[_`\xC0R_a\x1E\xFAV[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x88\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x01\xC6W\x83\x01\x90`\xA0\x81\x01\x91a%\xE1\x81\x80a9,V[\x80\x94`\xA0\x85RR`\xC0\x83\x01`\xC0\x85`\x05\x1B\x85\x01\x01\x94\x82_[\x82\x81\x10a&\xF6WPPPPPa&\x12` \x82\x01\x82a9,V[\x93\x90\x83\x82\x03` \x85\x01R\x84\x82R` \x82\x01\x90` \x86`\x05\x1B\x84\x01\x01\x95\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a&\x98WPPPP`@\x80\x85\x015\x90\x86\x01RPPP``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01R\x95` \x90\x81\x01\x95\x01\x93\x92`\x01\x01\x91\x90a\x1DvV[\x90\x91\x92\x93\x94\x95\x98`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x01\xC6W` a&\xE7`\x01\x93``a&\xD9\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90a9\x7FV[\x91\x81`@\x82\x01R\x01\x90a:\xA0V[\x9B\x01\x97\x01\x95\x94\x93\x92\x91\x01a&YV[\x90\x91\x92\x93\x96` \x80a'h\x83`\x80a'Za':\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F`\x01\x9A\x03\x01\x8DR\x8Aa9\x7FV[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90a9\x7FV[\x91\x81``\x82\x01R\x01\x90a:\xA0V[\x99\x01\x95\x01\x93\x92\x91\x01a%\xF9V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`@Q` \x81\x01\x90``\x81\x01\x87\x83R`@\x80\x83\x01R`\xA0QQ\x80\x91R`\x80\x82\x01\x90` `\xA0Q\x01\x90_[\x81\x81\x10a'\xFBWPPP\x91\x81a'\xF3a\x1C\xB6\x93a\x1C\xBF\x96\x95\x03`\x1F\x19\x81\x01\x83R\x82a\x16\xA8V[Q\x90 a\x1C\x99V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xCDV[\x90\x95a('\x87a(!\x87\x80a\x1BGV[\x90a\x1B\x9BV[`\xE0R___\x91a(<`\xE0Q`\xE0Qa\x1BGV[\x90Pa(G\x81a\x19bV[\x90a(Q\x81a\x19bV[\x90_[\x81\x81\x10a.~WPPa(n` `\xE0Q\x01`\xE0Qa\x1BGV[\x91\x90Pa(z\x82a\x19bV[\x90a(\x84\x83a\x19bV[\x92_\x90[\x80\x82\x10a*dWPPa(\xE2\x92a(\xFE\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a(\xF0a)\x0C\x94`@Q\x97\x88\x97`\x80`\xE0Q\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a\x17;V[\x90\x87\x82\x03`@\x89\x01Ra\x17;V[\x90\x85\x82\x03``\x87\x01Ra\x17;V[\x90\x83\x82\x03`\x80\x85\x01Ra\x17;V[\x03\x90\xA1\x80a*\\W[P\x80a*\nW[PP`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0`\xE0Q6\x03\x01\x12a\x01\xC6W`@Q\x90a)Y\x82a\x16pV[`@`\xE0Q\x015\x82R` \x82\x01```\xE0Q\x015\x81R`@Q\x92a)|\x84a\x16pV[_\x84R_` \x85\x01Ra)\x92\x81Q\x83Q\x90a6eV[\x15a)\xD2W\x91a)\xB1\x91`\x01\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92a6\xC7V[` \x83\x01R\x81R\x96`\x80`\xE0Q\x015a)\xCA\x82\x86a\x1B\xDBV[R\x01\x90a\x1CqV[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[`\x80Q\x01\x80`\x80Q\x11a*/W`\x80Ra*&W[_\x80a)\x1CV[`\x01\x95Pa*\x1FV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x95P_a)\x15V[\x90\x95Pa*\x82\x86a*|` `\xE0Q\x01`\xE0Qa\x1BGV[\x90a2\x05V[\x95a*\x91_` \x89\x015a0\xE8V[a.JW`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ta*\xE5\x81a1\xF7V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x885\x91_[\x82\x81\x10a-fWPP\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a+\xF3W[P\x96`@\x81\x01a+ya$\x0C\x82\x84a1\xADV[\x80a+\xBCW[PP\x81a+\x9Ba+\x94` \x93`\x01\x96\x95a1\xADV[\x825a4AV[\x805a+\xA7\x84\x89a\x1B\xDBV[R\x015a+\xB4\x82\x88a\x1B\xDBV[R\x01\x90a(\x88V[\x90\x9Aa+\xC7\x90a1\xF7V[\x9A\x8Ca+\x7FW` \x92\x9CPa+\x94a+\xE6`\x01\x96\x95\x93a+\x9B\x93a3pV[\x9D\x93PP\x81\x93\x94Pa+\x7FV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xBEW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x81\x10\x15a\x07\x91W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x01\x82\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x01T\x90a,\xD6\x90a,\xCF\x90\x83\x90a2\x96V[\x91\x80a2\x96V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xBEW`\x01\x81\x01\x80\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x81\x10\x15a\x07\x91W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R` _ \x01U_a+fV[\x90\x92`\x01\x90\x81\x85\x16a-\xF3W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta-\xE8\x91a2\x96V[\x93[\x81\x1C\x91\x01a+\rV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta.D\x91\x90a2\x96V[\x93a-\xEAV[` \x87\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R_`$R`D_\xFD[a.\x96\x81a.\x90`\xE0Q`\xE0Qa\x1BGV[\x90a1\xE0V[a.\xA5`\x01` \x83\x015a0\xE8V[a0.W`@\x81\x015a.\xE3\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03` R`\xFF`@_ T\x16\x90V[\x15a0\x03WP\x805_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`\xFF`@_ T\x16a/\xD7W\x805_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U``\x81\x01a/ea$\x0C\x82\x84a1\xADV[\x80a/\xA0W[PP\x81a/\x80a+\x94` \x93`\x01\x96\x95a1\xADV[\x805a/\x8C\x84\x89a\x1B\xDBV[R\x015a/\x99\x82\x86a\x1B\xDBV[R\x01a(TV[\x90\x98a/\xAB\x90a1\xF7V[\x98\x8Aa/kW` \x92\x9APa+\x94a/\xCA`\x01\x96\x95\x93a/\x80\x93a3pV[\x9B\x93PP\x81\x93\x94Pa/kV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R5`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[` \x90\x7FYz\xBD \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R`\x01`$R`D_\xFD[``\x82\x7F\xC9>\xE1c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x015`\x04R`$_\xFD[P\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06``\x83\x015\x14a\x1C8V[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[_\x90\x81R\x7FB6\xE6\xC1\xC0h\xF5\xE3\xB8\x92~\0\x1Eq\x12\xAFF~U\xC6\tM\xF09\xF1\xD5\x8B\x850\xA6)\0` R`@\x90 `\xFF\x91\x15a1!WT\x16\x90V[T`\x08\x1C\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a1iWV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`@Q\x90a1\xA2\x82a\x16pV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[\x90\x82\x10\x15a\x07\x91Wa\x178\x91`\x05\x1B\x81\x01\x90a1\xADV[_\x19\x81\x14a*/W`\x01\x01\x90V[\x91\x90\x81\x10\x15a\x07\x91W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W` \x01\x91\x816\x03\x83\x13a\x01\xC6WV[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81Ra2\xB9``\x82a\x16\xA8V[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x19\x07W_Q\x90V[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a3\x08WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x07\x91W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x01\xC6W\x01\x90V[_\x92\x91\x83\x90\x81[\x81\x81\x10a3\x84WPPPPV[a3\x9Ca3\x92\x82\x84\x87a30V[` \x81\x01\x90a2EV[\x84a4\x06W\x80`\x80\x11a\x01\xC6W` a3\xBA``\x84\x015\x83\x85aAJV[\x15\x92\x83\x15\x93a3\xDDW[PPPa3\xD3W`\x01\x01a3wV[P`\x01\x94PPPPV[\x91\x95P\x91P_\x94\x80`\xA0\x11a\x01\xC6Wa3\xFC\x92P`\x80\x82\x015\x91aAJV[\x15\x15_\x80\x80a3\xC4V[\x84\x80\xFD[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a\x178\x94\x92\x81R\x81` \x82\x01R\x01\x91a4\nV[\x90a4L\x81\x80a\x1BGV[_[\x81\x81\x10a5\xF6WPPPa4e` \x82\x01\x82a\x1BGV[_[\x81\x81\x10a5\x87WPPPa4~`@\x82\x01\x82a\x1BGV[_[\x81\x81\x10a5\x18WPPP\x80``a4\x98\x92\x01\x90a\x1BGV[\x90\x91_[\x82\x81\x10a4\xA9WPPPPV[a4\xB4\x81\x84\x86a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a4\xCFW[\x01a4\x9CV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca4\xFFa3\x92\x84\x88\x8Aa30V[\x90a5\x10`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a4\xC9V[a5#\x81\x83\x85a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a5>W[\x01a4\x80V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa5na3\x92\x84\x87\x89a30V[\x90a5\x7F`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a58V[a5\x92\x81\x83\x85a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a5\xADW[\x01a4gV[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3a5\xDDa3\x92\x84\x87\x89a30V[\x90a5\xEE`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a5\xA7V[a6\x01\x81\x83\x85a30V[5\x90`\x02\x82\x10\x15a\x01\xC6W`\x01\x80\x92\x14a6\x1CW[\x01a4NV[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6a6La3\x92\x84\x87\x89a30V[\x90a6]`@Q\x92\x83\x92\x87\x84a4*V[\x03\x90\xA2a6\x16V[\x80\x15\x80\x15a6\xB7W[\x80\x15a6\xAFW[\x80\x15a6\x9FW[a6\x99Wd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15a6|V[P\x81\x15a6uV[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15a6nV[\x92\x93\x92\x90\x91_\x90\x80\x83\x03a9\x12WPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08a6\xF0WPP\x90P_\x90_\x90V[d\x01\0\0\x03\xD0\x19\x82\x80\td\x01\0\0\x03\xD0\x19_\x81\x80\x80\x80\x86\x88\t`\x04\t\x95\x80\t`\x03\t\x08\x90d\x01\0\0\x03\xD0\x19\x91\x82\x84\x80\x08\x83\x03\x83\x81\x11a*/W\x83\x90\x81\x83\x80\t\x08\x93\x84\x84\x03\x92\x84\x84\x11a*/W\x84\x80\x91\x80\t`\x08\t\x84\x03\x84\x81\x11a*/W\x84`\x01\x93\x81\x80\x96\x81\x95\x08\x90\t\x08\x94\t`\x02\t\x91\x90[\x82\x15\x15\x83\x81a9\x01W[P\x80a8\xF9W[\x15a8\x9BW\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15a8HW\x80\x84\x04\x96\x80\x98a8\x1BWd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a*/Wd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a7\xEEW\x90a7\xE4\x91a;\xDEV[\x95\x92\x93\x96\x95a7\x8CV[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80a8nWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01a7sV[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_a7lV[a9&\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96a;\xEBV[\x91a7bV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x01\xC6W\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W\x81`\x05\x1B6\x03\x83\x13a\x01\xC6WV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a\x01\xC6W\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x826\x03\x01\x90[\x85\x84\x10a9\xFEWPPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x96`\x1F\x19\x82\x82\x03\x01\x86R\x875\x83\x81\x12\x15a\x01\xC6W\x84\x01\x805`\x02\x81\x10\x15a\x01\xC6W\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x01\xC6W\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x806\x03\x82\x13a\x01\xC6Wa:\x91` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91a4\nV[\x99\x01\x97\x96\x01\x94\x01\x92\x91\x90a9\xEDV[a\x178\x91a;\ta:\xFEa:\xE3a:\xC8a:\xBA\x86\x80a9,V[`\x80\x87R`\x80\x87\x01\x91a9\xB1V[a:\xD5` \x87\x01\x87a9,V[\x90\x86\x83\x03` \x88\x01Ra9\xB1V[a:\xF0`@\x86\x01\x86a9,V[\x90\x85\x83\x03`@\x87\x01Ra9\xB1V[\x92``\x81\x01\x90a9,V[\x91``\x81\x85\x03\x91\x01Ra9\xB1V[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80a;\xCBW[\x15a;KWPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15a;\x92Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15a;\xA3W`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80a;+WP\x81;\x15\x15a;+V[\x91\x90\x82\x03\x91\x82\x11a*/WV[\x94\x92\x91\x85\x15\x80a>\x0BW[a=\xFFW\x80\x15\x80a=\xF7W[a=\xEDW`@Q`\x80\x91a<\x16\x83\x83a\x16\xA8V[\x826\x837\x86\x15a=\xC0W\x86\x94\x85\x80\x92\x81\x80`\x01\x80\t\x80\x87R\x97\x81\x89`\x01\t\x9C` \x88\x01\x9D\x8ER\x82`@\x89\x01\x9D\x8E\x8C\x81RQ`\x01\t\x91``\x8A\x01\x92\x83R`@Q\x9E\x8Fa<`\x90a\x16\x8CV[Q\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90a=\xB4W[\x15a=VW\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85a<\xAE\x8F\x97\x88a\x16\xA8V[6\x877Q\x8CQa<\xBE\x90\x83a;\xDEV[\x90\x08\x84RQ\x85Qa<\xCF\x90\x83a;\xDEV[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88Qa=\x0C\x90\x83a;\xDEV[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\ta=#\x90\x83a;\xDEV[\x90\x08\x9CQ\x93Q\x90Q\x90\ta=7\x8C\x83a;\xDEV[\x90\x08\x90\t\x92Q\x90Q\x90\ta=K\x90\x83a;\xDEV[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15a<\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x92P`\x01\x91\x90PV[P\x82\x15a<\x02V[\x94P\x90\x92P`\x01\x91\x90PV[P\x81\x15a;\xF6V[5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xC6WV[\x91_[\x81\x81\x10a>DWPPPPV[a>Ra3\x92\x82\x84\x86a30V[\x81\x92\x91\x01``\x83\x82\x03\x12a\x01\xC6Wa>i\x83a>\x13V[` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x82a>\x89\x91\x86\x01a\x17\x1DV[`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6Wa>\xC3a>\xCB\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\x01\x98\x01a\x17\x1DV[\x92\x16\x88aB\xBAV[\x01a>7V[\x90\x92\x91`\x1F\x19a>\xF9a>\xE3\x83a\x17\x93V[\x92a>\xF1`@Q\x94\x85a\x16\xA8V[\x80\x84Ra\x17\x93V[\x01_[\x81\x81\x10a?\xF7WPP\x80\x93_\x90_[\x81\x81\x10a?\x19WPPPPPV[a?'a#\xA7\x82\x84\x88a\x1B\x9BV[_[\x81\x81\x10a?\xB1WPPPa?Aa#\xC9\x82\x84\x88a\x1B\x9BV[_[\x81\x81\x10a?UWPPP`\x01\x01a?\x0BV[\x80a?c`\x01\x92\x84\x86a2\x05V[a?sa$\x0C`@\x83\x01\x83a1\xADV[\x90\x81a?\x83W[PPP\x01a?CV[a?\xA9\x92a?\x9Aa?\x93\x8Ca1\xF7V[\x9B\x8Da\x1B\xDBV[Q\x89` \x83\x015\x925\x91aD\x8AV[_\x80\x80a?zV[\x80a?\xBF`\x01\x92\x84\x86a1\xE0V[a?\xCFa$\x0C``\x83\x01\x83a1\xADV[\x90\x81a?\xDFW[PPP\x01a?)V[a?\xEF\x92a?\x9Aa?\x93\x8Ca1\xF7V[_\x80\x80a?\xD6V[` \x90`@Qa@\x06\x81a\x16\x8CV[_\x81R_\x83\x82\x01R_`@\x82\x01R``\x80\x82\x01R\x82\x82\x86\x01\x01R\x01a>\xFCV[\x80Q\x91_[\x83\x81\x10a@8WPPPPV[a@La@E\x82\x84a\x1B\xDBV[Q\x84a\x1B\xDBV[Q``\x81\x01\x80QQ\x91` \x01\x90_[\x83\x81\x10a@nWPPPP`\x01\x01a@+V[\x80a@\xABa@\x7F`\x01\x93\x85Qa\x1B\xDBV[Q\x85Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16`@` \x83\x01Q\x92\x01Q\x92aB\xBAV[\x01a@[V[\x15a@\xB9WPV[\x7F\xBD\xE6\x0F`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x15a@\xECWPV[\x7F@\xA0\xD1\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x15aA\x1FWPV[\x7F;\x9Bi\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[` \x83\x01\x92\x91\x83\x83\x11a*/W\x83\x83\x11a\x01\xC6W\x83\x11a\x01\xC6W\x81\x015\x91\x03` \x81\x10aAuWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x81Q\x91\x90`A\x83\x03aA\xB3WaA\xAC\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aHnV[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aBhW\x80aA\xCFWPPV[`\x01\x81\x03aA\xFFW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03aB3WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14aB=WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x90\x91aB\xACa\x178\x93`@\x84R`@\x84\x01\x90a\x17nV[\x91` \x81\x84\x03\x91\x01Ra\x17nV[\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92`@Q\x92\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01R`@`$\x84\x01R_\x83\x80aC\x16`D\x82\x01\x86a\x17nV[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x19\x07W_\x93aC\xB1W[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03aCvWP\x90\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91aCq`@Q\x92\x83\x92\x83aB\x95V[\x03\x90\xA2V[\x90PaC\xAD`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01aB\x95V[\x03\x90\xFD[\x90\x92P=\x80_\x83>aC\xC3\x81\x83a\x16\xA8V[\x81\x01\x90` \x81\x83\x03\x12a\x01\xC6W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xC6W\x01\x81`\x1F\x82\x01\x12\x15a\x01\xC6W\x80Q\x90aC\xFA\x82a\x16\xCBV[\x92aD\x08`@Q\x94\x85a\x16\xA8V[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xC6W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_aC*V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xC6W\x815aDD\x81a\x17\x93V[\x92aDR`@Q\x94\x85a\x16\xA8V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xC6W` \x01\x90[\x82\x82\x10aDzWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01aDmV[\x93\x94\x92\x91\x90`@\x85\x01R\x83R` \x83\x01RaD\xA4\x81a\x17\x93V[\x91aD\xB2`@Q\x93\x84a\x16\xA8V[\x81\x83R`\x1F\x19aD\xC1\x83a\x17\x93V[\x01_[\x81\x81\x10aE\xC9WPP``\x01\x91\x82R_[\x81\x81\x10aD\xE2WPPPPV[aD\xED\x81\x84Qa\x1B\xDBV[Q\x90aD\xFDa3\x92\x82\x85\x88a30V[\x81\x01\x92\x90`\xA0\x81\x85\x03\x12a\x01\xC6WaE\x14\x81a>\x13V[` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x85aE4\x91\x84\x01a\x17\x1DV[`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x86aET\x91\x85\x01a\x17\x1DV[``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xC6W\x87aEt\x91\x86\x01aD-V[`\x80\x85\x015\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11a\x01\xC6W`\x01\x98s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96aE\xAC\x92\x01aD-V[`\x80\x87\x01R``\x86\x01R`@\x85\x01R` \x84\x01R\x16\x90R\x01aD\xD5V[`@Q\x90`\xA0\x82\x01\x91\x80\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x07\xBEW` \x92`@R_\x81R``\x83\x82\x01R```@\x82\x01R``\x80\x82\x01R```\x80\x82\x01R\x82\x82\x88\x01\x01R\x01aD\xC4V[` \x81Q\x91\x01Q\x90\x80Qa\x01\0\x81\x11aH\x06W__[\x82\x81\x10aG\xE0WPaF;\x82a\x19bV[\x93__[\x84\x81\x10aFQWPPPPPP`\x01\x91V[\x83\x15aG\xD5WaG\x05aF\xE7\x85`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x94`\x01\x86\x1B\x90\x81\x19\x16\x93\x17\x94\x80aF\xFE\x84\x8Ba\x1B\xDBV[R\x84a\x1B\xDBV[Q\x80\x85\x19\x91[aG\x1CWPP`\x01\x01\x92\x90\x92aF?V[aG\xA9\x81`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x90\x82aG\xBD`\x01\x84\x1B\x92\x83\x19\x16\x93\x8Ba\x1B\xDBV[Q\x16\x15aG\xCCW[P\x80aG\x0BV[\x81\x94\x17\x93aG\xC5V[P_\x96\x94PPPPPV[aG\xEA\x81\x85a\x1B\xDBV[Q\x15aG\xF9W[`\x01\x01aF*V[`\x01\x81\x1B\x90\x91\x17\x90aG\xF1V[\x7F\xA1V\xECF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x91\x90\x82Q\x90_\x93[\x82\x85\x10aHEWPPPV[\x90\x91\x92\x93\x82aHT\x82\x84a\x1B\xDBV[QQ\x14aHgW`\x01\x01\x93\x92\x91\x90aH9V[\x92PPP\x90V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aH\xF2W\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x19\x07W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aH\xE8W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x92\x90\x91`@aI\x0C\x82\x86a\x1B\xDBV[Q\x01Q`@aI\x1B\x84\x87a\x1B\xDBV[Q\x01Q\x14aI\xB5WaI2aI:\x91\x85\x93\x95aI\xF8V[\x94\x90\x92aI\xF8V[\x93aIE\x81\x84aJ\x97V[\x92aIP\x86\x84aJ\x97V[` \x86\x01\x91[\x83\x81\x10aI\x92WPPPP[\x83\x81\x10aIoWPPPPV[\x80\x82aI~`\x01\x93\x86Qa\x1B\xDBV[Q\x17aI\x8B\x82\x86Qa\x1B\xDBV[R\x01aIbV[\x80\x82aI\xA1`\x01\x93\x86Qa\x1B\xDBV[Q\x17aI\xAE\x82\x86Qa\x1B\xDBV[R\x01aIVV[\x90\x92P` \x82aI\xC9\x85aI\xF5\x95Qa\x1B\xDBV[Q`\x01\x84\x1B\x17aI\xDA\x86\x83Qa\x1B\xDBV[R\x01\x92`\x01aI\xEA\x83\x86Qa\x1B\xDBV[Q\x91\x1B\x17\x92Qa\x1B\xDBV[RV[\x91`@aJ\x05\x83\x85a\x1B\xDBV[Q\x01Q\x90\x82[\x80\x15\x80\x15\x80aJuW[\x15aJ&Wa*/W_\x19\x01aJ\x0BV[P\x93\x90\x91\x92`\x01\x81\x01\x80\x91\x11a*/W\x92\x81Q\x93[\x84\x81\x14\x15\x80aJ^W[\x15aJXWaJS\x90a1\xF7V[aJ;V[\x93PPPV[P\x81`@aJl\x83\x86a\x1B\xDBV[Q\x01Q\x14aJEV[P_\x19\x82\x01\x82\x81\x11a*/W`@aJ\x8E\x86\x92\x89a\x1B\xDBV[Q\x01Q\x14aJ\x15V[\x90a\x01\0\x03\x90a\x01\0\x82\x11a*/W`\x01\x90\x1B_\x19\x81\x01\x90\x81\x11a*/W_\x19\x90\x19\x91\x1C\x16\x90V\xFE\xA1dsolcC\0\x08&\0\n",
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
    /**Custom error with signature `AfterTagNamesNoCallCarryingResource(bytes32)` and selector `0xbde60f60`.
```solidity
error AfterTagNamesNoCallCarryingResource(bytes32 tag);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AfterTagNamesNoCallCarryingResource {
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
        impl ::core::convert::From<AfterTagNamesNoCallCarryingResource>
        for UnderlyingRustTuple<'_> {
            fn from(value: AfterTagNamesNoCallCarryingResource) -> Self {
                (value.tag,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AfterTagNamesNoCallCarryingResource {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tag: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AfterTagNamesNoCallCarryingResource {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AfterTagNamesNoCallCarryingResource(bytes32)";
            const SELECTOR: [u8; 4] = [189u8, 230u8, 15u8, 96u8];
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
    /**Custom error with signature `BeforeTagNamesNoCallCarryingResource(bytes32)` and selector `0x3b9b6984`.
```solidity
error BeforeTagNamesNoCallCarryingResource(bytes32 tag);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeforeTagNamesNoCallCarryingResource {
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
        impl ::core::convert::From<BeforeTagNamesNoCallCarryingResource>
        for UnderlyingRustTuple<'_> {
            fn from(value: BeforeTagNamesNoCallCarryingResource) -> Self {
                (value.tag,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BeforeTagNamesNoCallCarryingResource {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tag: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BeforeTagNamesNoCallCarryingResource {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BeforeTagNamesNoCallCarryingResource(bytes32)";
            const SELECTOR: [u8; 4] = [59u8, 155u8, 105u8, 132u8];
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
    /**Custom error with signature `CallOrderNotAPermutation()` and selector `0x9a12c510`.
```solidity
error CallOrderNotAPermutation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallOrderNotAPermutation;
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
        impl ::core::convert::From<CallOrderNotAPermutation>
        for UnderlyingRustTuple<'_> {
            fn from(value: CallOrderNotAPermutation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CallOrderNotAPermutation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CallOrderNotAPermutation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CallOrderNotAPermutation()";
            const SELECTOR: [u8; 4] = [154u8, 18u8, 197u8, 16u8];
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
    /**Custom error with signature `CallOrderNotInActionOrder(uint256)` and selector `0xf3474dc2`.
```solidity
error CallOrderNotInActionOrder(uint256 actionIndex);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallOrderNotInActionOrder {
        #[allow(missing_docs)]
        pub actionIndex: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CallOrderNotInActionOrder>
        for UnderlyingRustTuple<'_> {
            fn from(value: CallOrderNotInActionOrder) -> Self {
                (value.actionIndex,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CallOrderNotInActionOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { actionIndex: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CallOrderNotInActionOrder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CallOrderNotInActionOrder(uint256)";
            const SELECTOR: [u8; 4] = [243u8, 71u8, 77u8, 194u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.actionIndex),
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
    /**Custom error with signature `CallOrderViolatesConstraint(bytes32)` and selector `0x40a0d1c1`.
```solidity
error CallOrderViolatesConstraint(bytes32 tag);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallOrderViolatesConstraint {
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
        impl ::core::convert::From<CallOrderViolatesConstraint>
        for UnderlyingRustTuple<'_> {
            fn from(value: CallOrderViolatesConstraint) -> Self {
                (value.tag,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CallOrderViolatesConstraint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tag: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CallOrderViolatesConstraint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CallOrderViolatesConstraint(bytes32)";
            const SELECTOR: [u8; 4] = [64u8, 160u8, 209u8, 193u8];
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
    /**Custom error with signature `CyclicOrderingConstraints()` and selector `0x813c76a0`.
```solidity
error CyclicOrderingConstraints();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CyclicOrderingConstraints;
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
        impl ::core::convert::From<CyclicOrderingConstraints>
        for UnderlyingRustTuple<'_> {
            fn from(value: CyclicOrderingConstraints) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CyclicOrderingConstraints {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CyclicOrderingConstraints {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CyclicOrderingConstraints()";
            const SELECTOR: [u8; 4] = [129u8, 60u8, 118u8, 160u8];
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
    /**Custom error with signature `TooManyCallCarryingResources(uint256)` and selector `0x89502288`.
```solidity
error TooManyCallCarryingResources(uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TooManyCallCarryingResources {
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
        impl ::core::convert::From<TooManyCallCarryingResources>
        for UnderlyingRustTuple<'_> {
            fn from(value: TooManyCallCarryingResources) -> Self {
                (value.count,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TooManyCallCarryingResources {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { count: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TooManyCallCarryingResources {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TooManyCallCarryingResources(uint256)";
            const SELECTOR: [u8; 4] = [137u8, 80u8, 34u8, 136u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.count),
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
    /**Custom error with signature `TooManyNodes(uint256)` and selector `0xa156ec46`.
```solidity
error TooManyNodes(uint256 nodeCount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TooManyNodes {
        #[allow(missing_docs)]
        pub nodeCount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<TooManyNodes> for UnderlyingRustTuple<'_> {
            fn from(value: TooManyNodes) -> Self {
                (value.nodeCount,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TooManyNodes {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { nodeCount: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TooManyNodes {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TooManyNodes(uint256)";
            const SELECTOR: [u8; 4] = [161u8, 86u8, 236u8, 70u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.nodeCount),
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
    /**Function with signature `executeWithCallOrder((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32),uint256[])` and selector `0x155f9f02`.
```solidity
function executeWithCallOrder(IProtocolAdapter.Transaction memory transaction, uint256[] memory callOrder) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeWithCallOrderCall {
        #[allow(missing_docs)]
        pub transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub callOrder: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`executeWithCallOrder((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32),uint256[])`](executeWithCallOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeWithCallOrderReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<executeWithCallOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeWithCallOrderCall) -> Self {
                    (value.transaction, value.callOrder)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeWithCallOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        transaction: tuple.0,
                        callOrder: tuple.1,
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
            impl ::core::convert::From<executeWithCallOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeWithCallOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeWithCallOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl executeWithCallOrderReturn {
            fn _tokenize(
                &self,
            ) -> <executeWithCallOrderCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeWithCallOrderCall {
            type Parameters<'a> = (
                IProtocolAdapter::Transaction,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeWithCallOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeWithCallOrder((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes,bytes32),uint256[])";
            const SELECTOR: [u8; 4] = [21u8, 95u8, 159u8, 2u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.callOrder),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                executeWithCallOrderReturn::_tokenize(ret)
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
    ///Container for all the [`ProtocolAdapter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolAdapterCalls {
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
        executeWithCallOrder(executeWithCallOrderCall),
        #[allow(missing_docs)]
        getImplementation(getImplementationCall),
        #[allow(missing_docs)]
        getKindTreeRoot(getKindTreeRootCall),
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
    impl ProtocolAdapterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [21u8, 95u8, 159u8, 2u8],
            [23u8, 30u8, 172u8, 24u8],
            [26u8, 132u8, 115u8, 188u8],
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
            [251u8, 15u8, 246u8, 128u8],
            [254u8, 24u8, 171u8, 145u8],
            [255u8, 161u8, 173u8, 116u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(executeWithCallOrder),
            ::core::stringify!(isLogicRefDenied),
            ::core::stringify!(denyLogicRefs),
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
            ::core::stringify!(getKindTreeRoot),
            ::core::stringify!(commitmentTreeCapacity),
            ::core::stringify!(VERSION),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <executeWithCallOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isLogicRefDeniedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <denyLogicRefsCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for ProtocolAdapterCalls {
        const NAME: &'static str = "ProtocolAdapterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
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
                Self::executeWithCallOrder(_) => {
                    <executeWithCallOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getImplementation(_) => {
                    <getImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKindTreeRoot(_) => {
                    <getKindTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ProtocolAdapterCalls>] = &[
                {
                    fn executeWithCallOrder(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <executeWithCallOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::executeWithCallOrder)
                    }
                    executeWithCallOrder
                },
                {
                    fn isLogicRefDenied(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isLogicRefDeniedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::isLogicRefDenied)
                    }
                    isLogicRefDenied
                },
                {
                    fn denyLogicRefs(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <denyLogicRefsCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::denyLogicRefs)
                    }
                    denyLogicRefs
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
                    fn EMPTY_KIND_TREE_ROOT(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <EMPTY_KIND_TREE_ROOTCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::EMPTY_KIND_TREE_ROOT)
                    }
                    EMPTY_KIND_TREE_ROOT
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
                    fn setKindTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <setKindTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::setKindTreeRoot)
                    }
                    setKindTreeRoot
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
                    fn isCommitmentTreeRootHistorical(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::isCommitmentTreeRootHistorical)
                    }
                    isCommitmentTreeRootHistorical
                },
                {
                    fn getKindTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <getKindTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterCalls::getKindTreeRoot)
                    }
                    getKindTreeRoot
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
                Self::executeWithCallOrder(inner) => {
                    <executeWithCallOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::executeWithCallOrder(inner) => {
                    <executeWithCallOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
    ///Container for all the [`ProtocolAdapter`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolAdapterErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        AfterTagNamesNoCallCarryingResource(AfterTagNamesNoCallCarryingResource),
        #[allow(missing_docs)]
        BeforeTagNamesNoCallCarryingResource(BeforeTagNamesNoCallCarryingResource),
        #[allow(missing_docs)]
        CallOrderNotAPermutation(CallOrderNotAPermutation),
        #[allow(missing_docs)]
        CallOrderNotInActionOrder(CallOrderNotInActionOrder),
        #[allow(missing_docs)]
        CallOrderViolatesConstraint(CallOrderViolatesConstraint),
        #[allow(missing_docs)]
        CyclicOrderingConstraints(CyclicOrderingConstraints),
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
        LogicRefAlreadyDenied(LogicRefAlreadyDenied),
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
        TooManyCallCarryingResources(TooManyCallCarryingResources),
        #[allow(missing_docs)]
        TooManyNodes(TooManyNodes),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnacceptedKindTreeRoot(UnacceptedKindTreeRoot),
        #[allow(missing_docs)]
        ZeroKindTreeRootNotAllowed(ZeroKindTreeRootNotAllowed),
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
            [11u8, 247u8, 113u8, 140u8],
            [17u8, 140u8, 218u8, 167u8],
            [18u8, 125u8, 51u8, 94u8],
            [30u8, 79u8, 189u8, 247u8],
            [37u8, 174u8, 110u8, 170u8],
            [57u8, 169u8, 64u8, 197u8],
            [59u8, 155u8, 105u8, 132u8],
            [62u8, 229u8, 174u8, 181u8],
            [64u8, 160u8, 209u8, 193u8],
            [76u8, 156u8, 140u8, 227u8],
            [89u8, 122u8, 189u8, 32u8],
            [120u8, 162u8, 34u8, 28u8],
            [122u8, 246u8, 35u8, 65u8],
            [129u8, 60u8, 118u8, 160u8],
            [137u8, 80u8, 34u8, 136u8],
            [141u8, 252u8, 32u8, 43u8],
            [153u8, 150u8, 179u8, 21u8],
            [154u8, 18u8, 197u8, 16u8],
            [161u8, 86u8, 236u8, 70u8],
            [166u8, 216u8, 42u8, 2u8],
            [170u8, 29u8, 73u8, 164u8],
            [177u8, 178u8, 90u8, 175u8],
            [179u8, 152u8, 151u8, 159u8],
            [184u8, 160u8, 232u8, 161u8],
            [189u8, 230u8, 15u8, 96u8],
            [197u8, 4u8, 250u8, 218u8],
            [201u8, 62u8, 225u8, 99u8],
            [207u8, 27u8, 9u8, 60u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [219u8, 120u8, 140u8, 43u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 212u8, 75u8, 76u8],
            [243u8, 71u8, 77u8, 194u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [249u8, 132u8, 158u8, 163u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ZeroKindTreeRootNotAllowed),
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(LogicRefAlreadyDenied),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(RiscZeroVerifierPaused),
            ::core::stringify!(PreExistingNullifier),
            ::core::stringify!(BeforeTagNamesNoCallCarryingResource),
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(CallOrderViolatesConstraint),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(ResourceWithDeniedLogicRef),
            ::core::stringify!(RiscZeroVerifierSelectorMismatch),
            ::core::stringify!(ZeroLogicRefNotAllowed),
            ::core::stringify!(CyclicOrderingConstraints),
            ::core::stringify!(TooManyCallCarryingResources),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(CallOrderNotAPermutation),
            ::core::stringify!(TooManyNodes),
            ::core::stringify!(ZeroRiscZeroVerifierRouterNotAllowed),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ZeroRiscZeroVerifierSelectorNotAllowed),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(PointNotOnCurve),
            ::core::stringify!(AfterTagNamesNoCallCarryingResource),
            ::core::stringify!(ForwarderCallOutputMismatch),
            ::core::stringify!(UnacceptedKindTreeRoot),
            ::core::stringify!(EmptyTransactionNotAllowed),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(PreExistingRoot),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(DeltaMismatch),
            ::core::stringify!(CallOrderNotInActionOrder),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(NonExistingRoot),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <LogicRefAlreadyDenied as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierPaused as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingNullifier as alloy_sol_types::SolError>::SIGNATURE,
            <BeforeTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::SIGNATURE,
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <CallOrderViolatesConstraint as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <ResourceWithDeniedLogicRef as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroLogicRefNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <CyclicOrderingConstraints as alloy_sol_types::SolError>::SIGNATURE,
            <TooManyCallCarryingResources as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <CallOrderNotAPermutation as alloy_sol_types::SolError>::SIGNATURE,
            <TooManyNodes as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <PointNotOnCurve as alloy_sol_types::SolError>::SIGNATURE,
            <AfterTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::SIGNATURE,
            <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingRoot as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <DeltaMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <CallOrderNotInActionOrder as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 40usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AfterTagNamesNoCallCarryingResource(_) => {
                    <AfterTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BeforeTagNamesNoCallCarryingResource(_) => {
                    <BeforeTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CallOrderNotAPermutation(_) => {
                    <CallOrderNotAPermutation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CallOrderNotInActionOrder(_) => {
                    <CallOrderNotInActionOrder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CallOrderViolatesConstraint(_) => {
                    <CallOrderViolatesConstraint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CyclicOrderingConstraints(_) => {
                    <CyclicOrderingConstraints as alloy_sol_types::SolError>::SELECTOR
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
                Self::LogicRefAlreadyDenied(_) => {
                    <LogicRefAlreadyDenied as alloy_sol_types::SolError>::SELECTOR
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
                Self::TooManyCallCarryingResources(_) => {
                    <TooManyCallCarryingResources as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TooManyNodes(_) => {
                    <TooManyNodes as alloy_sol_types::SolError>::SELECTOR
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
                    fn ZeroKindTreeRootNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroKindTreeRootNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::ZeroKindTreeRootNotAllowed)
                    }
                    ZeroKindTreeRootNotAllowed
                },
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
                    fn LogicRefAlreadyDenied(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <LogicRefAlreadyDenied as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::LogicRefAlreadyDenied)
                    }
                    LogicRefAlreadyDenied
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
                    fn BeforeTagNamesNoCallCarryingResource(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <BeforeTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                ProtocolAdapterErrors::BeforeTagNamesNoCallCarryingResource,
                            )
                    }
                    BeforeTagNamesNoCallCarryingResource
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
                    fn CallOrderViolatesConstraint(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <CallOrderViolatesConstraint as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::CallOrderViolatesConstraint)
                    }
                    CallOrderViolatesConstraint
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
                    fn CyclicOrderingConstraints(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <CyclicOrderingConstraints as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::CyclicOrderingConstraints)
                    }
                    CyclicOrderingConstraints
                },
                {
                    fn TooManyCallCarryingResources(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <TooManyCallCarryingResources as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::TooManyCallCarryingResources)
                    }
                    TooManyCallCarryingResources
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
                    fn CallOrderNotAPermutation(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <CallOrderNotAPermutation as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::CallOrderNotAPermutation)
                    }
                    CallOrderNotAPermutation
                },
                {
                    fn TooManyNodes(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <TooManyNodes as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::TooManyNodes)
                    }
                    TooManyNodes
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
                    fn AfterTagNamesNoCallCarryingResource(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <AfterTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(
                                ProtocolAdapterErrors::AfterTagNamesNoCallCarryingResource,
                            )
                    }
                    AfterTagNamesNoCallCarryingResource
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
                    fn UnacceptedKindTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UnacceptedKindTreeRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::UnacceptedKindTreeRoot)
                    }
                    UnacceptedKindTreeRoot
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
                    fn CallOrderNotInActionOrder(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <CallOrderNotInActionOrder as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(ProtocolAdapterErrors::CallOrderNotInActionOrder)
                    }
                    CallOrderNotInActionOrder
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
                Self::AfterTagNamesNoCallCarryingResource(inner) => {
                    <AfterTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BeforeTagNamesNoCallCarryingResource(inner) => {
                    <BeforeTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CallOrderNotAPermutation(inner) => {
                    <CallOrderNotAPermutation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CallOrderNotInActionOrder(inner) => {
                    <CallOrderNotInActionOrder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CallOrderViolatesConstraint(inner) => {
                    <CallOrderViolatesConstraint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CyclicOrderingConstraints(inner) => {
                    <CyclicOrderingConstraints as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::LogicRefAlreadyDenied(inner) => {
                    <LogicRefAlreadyDenied as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::TooManyCallCarryingResources(inner) => {
                    <TooManyCallCarryingResources as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TooManyNodes(inner) => {
                    <TooManyNodes as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::AfterTagNamesNoCallCarryingResource(inner) => {
                    <AfterTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BeforeTagNamesNoCallCarryingResource(inner) => {
                    <BeforeTagNamesNoCallCarryingResource as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CallOrderNotAPermutation(inner) => {
                    <CallOrderNotAPermutation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CallOrderNotInActionOrder(inner) => {
                    <CallOrderNotInActionOrder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CallOrderViolatesConstraint(inner) => {
                    <CallOrderViolatesConstraint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CyclicOrderingConstraints(inner) => {
                    <CyclicOrderingConstraints as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::LogicRefAlreadyDenied(inner) => {
                    <LogicRefAlreadyDenied as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::TooManyCallCarryingResources(inner) => {
                    <TooManyCallCarryingResources as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TooManyNodes(inner) => {
                    <TooManyNodes as alloy_sol_types::SolError>::abi_encode_raw(
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
        /**Creates a [`AfterTagNamesNoCallCarryingResource`] error.

```solidity
error AfterTagNamesNoCallCarryingResource(bytes32)
```*/
        #[inline]
        pub fn after_tag_names_no_call_carrying_resource(
            tag: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::AfterTagNamesNoCallCarryingResource(AfterTagNamesNoCallCarryingResource {
                tag: tag,
            })
        }
        /**Creates a [`BeforeTagNamesNoCallCarryingResource`] error.

```solidity
error BeforeTagNamesNoCallCarryingResource(bytes32)
```*/
        #[inline]
        pub fn before_tag_names_no_call_carrying_resource(
            tag: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::BeforeTagNamesNoCallCarryingResource(BeforeTagNamesNoCallCarryingResource {
                tag: tag,
            })
        }
        /**Creates a [`CallOrderNotAPermutation`] error.

```solidity
error CallOrderNotAPermutation()
```*/
        #[inline]
        pub fn call_order_not_a_permutation() -> Self {
            Self::CallOrderNotAPermutation(CallOrderNotAPermutation)
        }
        /**Creates a [`CallOrderNotInActionOrder`] error.

```solidity
error CallOrderNotInActionOrder(uint256)
```*/
        #[inline]
        pub fn call_order_not_in_action_order(
            action_index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::CallOrderNotInActionOrder(CallOrderNotInActionOrder {
                actionIndex: action_index,
            })
        }
        /**Creates a [`CallOrderViolatesConstraint`] error.

```solidity
error CallOrderViolatesConstraint(bytes32)
```*/
        #[inline]
        pub fn call_order_violates_constraint(
            tag: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::CallOrderViolatesConstraint(CallOrderViolatesConstraint {
                tag: tag,
            })
        }
        /**Creates a [`CyclicOrderingConstraints`] error.

```solidity
error CyclicOrderingConstraints()
```*/
        #[inline]
        pub fn cyclic_ordering_constraints() -> Self {
            Self::CyclicOrderingConstraints(CyclicOrderingConstraints)
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
        /**Creates a [`TooManyCallCarryingResources`] error.

```solidity
error TooManyCallCarryingResources(uint256)
```*/
        #[inline]
        pub fn too_many_call_carrying_resources(
            count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::TooManyCallCarryingResources(TooManyCallCarryingResources {
                count: count,
            })
        }
        /**Creates a [`TooManyNodes`] error.

```solidity
error TooManyNodes(uint256)
```*/
        #[inline]
        pub fn too_many_nodes(
            node_count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::TooManyNodes(TooManyNodes {
                nodeCount: node_count,
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
        KindTreeRootUpdated(KindTreeRootUpdated),
        #[allow(missing_docs)]
        LogicRefDenied(LogicRefDenied),
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
            ::core::stringify!(CommitmentTreeRootAdded),
            ::core::stringify!(KindTreeRootUpdated),
            ::core::stringify!(ResourcePayload),
            ::core::stringify!(DiscoveryPayload),
            ::core::stringify!(ActionExecuted),
            ::core::stringify!(Unpaused),
            ::core::stringify!(Paused),
            ::core::stringify!(TransactionExecuted),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(ExternalPayload),
            ::core::stringify!(ApplicationPayload),
            ::core::stringify!(LogicRefDenied),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
            ::core::stringify!(ForwarderCallExecuted),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <KindTreeRootUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE,
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
                Self::KindTreeRootUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LogicRefDenied(inner) => {
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
                Self::KindTreeRootUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LogicRefDenied(inner) => {
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
        ///Creates a new call builder for the [`executeWithCallOrder`] function.
        pub fn executeWithCallOrder(
            &self,
            transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
            callOrder: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, executeWithCallOrderCall, N> {
            self.call_builder(
                &executeWithCallOrderCall {
                    transaction,
                    callOrder,
                },
            )
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
