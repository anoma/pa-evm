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
    struct Transaction { Action[] actions; bytes deltaProof; bytes aggregationProof; }
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
            pub const NAME: &'static str = stringify!(@ name);
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
struct Transaction { Action[] actions; bytes deltaProof; bytes aggregationProof; }
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <Action as alloy::sol_types::SolType>::RustType,
            >,
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
        impl ::core::convert::From<Transaction> for UnderlyingRustTuple<'_> {
            fn from(value: Transaction) -> Self {
                (value.actions, value.deltaProof, value.aggregationProof)
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
                    "Transaction(Action[] actions,bytes deltaProof,bytes aggregationProof)",
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
    error NonExistingRoot(bytes32 root);
    error NotInitializing();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error PointNotOnCurve(IProtocolAdapter.Delta point);
    error PreExistingNullifier(bytes32 nullifier);
    error PreExistingRoot(bytes32 root);
    error ReentrancyGuardReentrantCall();
    error RiscZeroVerifierSelectorMismatch(bytes4 expected, bytes4 actual);
    error RiscZeroVerifierStopped();
    error Simulated(uint256 gasUsed);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error ZeroKindTableCommitmentNotAllowed();
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
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address account);
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
    event TransactionExecuted(bytes32 indexed transactionId);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);

    constructor(address riscZeroVerifierRouter, bytes4 riscZeroVerifierSelector);

    function RISC_ZERO_VERIFIER_ROUTER() external view returns (address);
    function RISC_ZERO_VERIFIER_SELECTOR() external view returns (bytes4);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function VERSION() external view returns (string memory);
    function commitmentCount() external view returns (uint256 count);
    function commitmentTreeCapacity() external view returns (uint256 capacity);
    function commitmentTreeDepth() external view returns (uint8 depth);
    function commitmentTreeRootAtIndex(uint256 index) external view returns (bytes32 root);
    function commitmentTreeRootCount() external view returns (uint256 count);
    function emergencyStop() external;
    function execute(IProtocolAdapter.Transaction memory transaction) external;
    function getKindTableCommitment() external view returns (bytes32 kindTableCommitment);
    function implementation() external view returns (address current);
    function initialize(address initialOwner) external;
    function isCommitmentTreeRootContained(bytes32 root) external view returns (bool isContained);
    function isEmergencyStopped() external view returns (bool isStopped);
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
    function latestCommitmentTreeRoot() external view returns (bytes32 root);
    function nullifierAtIndex(uint256 index) external view returns (bytes32 nullifier);
    function nullifierCount() external view returns (uint256 count);
    function owner() external view returns (address);
    function paused() external view returns (bool);
    function proxiableUUID() external view returns (bytes32);
    function renounceOwnership() external;
    function setKindTableCommitment(bytes32 newKindTableCommitment) external;
    function simulateExecute(IProtocolAdapter.Transaction memory transaction, bool skipRiscZeroProofVerification) external;
    function transferOwnership(address newOwner) external;
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
    "name": "emergencyStop",
    "inputs": [],
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
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "implementation",
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
    "name": "isEmergencyStopped",
    "inputs": [],
    "outputs": [
      {
        "name": "isStopped",
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
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
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
    "name": "RiscZeroVerifierStopped",
    "inputs": []
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
    "name": "ZeroKindTableCommitmentNotAllowed",
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
    ///0x60e03461010e57601f61473f38819003918201601f19168301916001600160401b0383118484101761011257808492604094855283398101031261010e5780516001600160a01b0381169182820361010e5760200151916001600160e01b031983169081840361010e5730608052610075610126565b61007d610126565b156100ff57156100f05760a05260c052610095610126565b60405161456290816101bd8239608051818181612859015261291d015260a051818181610f1101528181611bb7015281816126820152612ea4015260c0518181816102d701528181610db701528181611b770152612e620152f35b63b1b25aaf60e01b5f5260045ffd5b63536c150160e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f51602061471f5f395f51905f525460ff8160401c166101ad576002600160401b03196001600160401b0382160161015b5750565b6001600160401b0319166001600160401b039081175f51602061471f5f395f51905f52556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a1565b63f92ee8a960e01b5f5260045ffdfe6101606040526004361015610012575f80fd5b5f610120525f3560e01c806331ee624214612bdd57806340f34d4214612ba15780634f1ef286146128d157806352d1902d1461283257806359ba9258146127f65780635c60da1b146127a45780635c975abb1461276357806363a599a4146126a65780636bf0a04b14612656578063715018a61461259a57806373ab9916146118fb57806387093eba14610b0e5780638da5cb5b14610ab95780639ad91d4c14610a02578063a06056f7146109c0578063ad3cb1cc1461095d578063bdeb442d146108d0578063c02530231461082c578063c1b0bed7146107d6578063c44956d114610797578063c4d66de814610344578063c879dbe4146102fb578063e35a5d2f1461029c578063f2fde38b1461026c578063fddd483714610245578063fe18ab9114610200578063ffa1ad74146101995763ffc33f7214610153575f80fd5b3461019257610120516003193601126101925760207f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a70054604051908152f35b6101205180fd5b34610192576101205160031936011261019257604080516101fc916101be9082612c67565b600d81527f322e302e302d616c7068612e36000000000000000000000000000000000000006020820152604051918291602083526020830190612cfa565b0390f35b346101925761012051600319360112610192576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b346101925761012051600319360112610192576020610262612e19565b6040519015158152f35b3461019257602060031936011261019257610295610288612bfb565b610290612fe8565b612d2c565b6101205180f35b3461019257610120516003193601126101925760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101925760206003193601126101925760206102626004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b346101925760206003193601126101925761035d612bfb565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c16159167ffffffffffffffff82168015908161078f575b6001149081610785575b15908161077c575b5061074e57818360017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000006104169516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00556106f9575b5061040e613227565b610290613227565b61041e613227565b610426613227565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156106c6578060016104a992017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9026132d4565b81549060031b905f197fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06831b921b1916179055610120517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005561050a613f50565b507f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a161055d613227565b7fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a700557fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26105f3612e19565b61069857610602576101205180f35b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1610295565b7f0b1d38a3000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526041600452602461012051fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005583610405565b7ff92ee8a9000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b905015846103ae565b303b1591506103a6565b84915061039c565b3461019257610120516003193601126101925760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b3461019257602060031936011261019257610120515060043561012051527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc01602052602060406101205120541515604051908152f35b3461019257602060031936011261019257600435610848612fe8565b80156108a257807f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a700557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26101205180f35b7f774d5de1000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b346101925761012051600319360112610192577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903545f19810190811161092a5761091b60209161327e565b90549060031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526011600452602461012051fd5b34610192576101205160031936011261019257604080516101fc916109829082612c67565b600581527f352e302e300000000000000000000000000000000000000000000000000000006020820152604051918291602083526020830190612cfa565b34610192576101205160031936011261019257602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b34610192576020600319360112610192577f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc005460043590811015610a86576020907f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc006101205152816101205120016101205150546101205160031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526032600452602461012051fd5b34610192576101205160031936011261019257602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346101925760406003193601126101925767ffffffffffffffff60043511610192576060600319600435360301126101925760243560a05260a051151560a05103610192575a60e0527f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c6118cd5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d610ba9613054565b610bb76004803501806130a7565b8091501561189f57610bc881613113565b608052610bd3613158565b50604051610be081612c1e565b610120518152610120516020820152906101205161010052610120515b81811061126f5782610c476080515160051b6020608051012091610c3e610c38610c316024600435016004356004016131d6565b3691612ca6565b846143b4565b909391936143ee565b6020815191015190610120515260205273ffffffffffffffffffffffffffffffffffffffff80604061012051201691169080820361123b5782610c8e6004803501806130a7565b7f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a7009291925490806040519260808401907f7b657df4c7ee3ef8592894761aefc80f196e5b97dd27d43a98628b2ce2ef91f0602086015260408501526060808501525260a082019060a08160051b84010194809261012051915b83831061101b5786602087610d25818c03601f198101835282612c67565b8160405191805191829101835e810190610120518252806101205192039060025afa15610fd9576101205151610d656044600435016004356004016131d6565b80600493929311610192577fffffffff000000000000000000000000000000000000000000000000000000008335167fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016808203610fe757505060a05115610efa575b50505061010051610e82575b7f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d610e515a60e051612d1f565b7f6f149831000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b610e8e61010051613ea0565b15610ec5577f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d26020604051610100518152a1610df5565b7fdb788c2b00000000000000000000000000000000000000000000000000000000610120515261010051600452602461012051fd5b73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b1561019257610f756040519485937fab750e75000000000000000000000000000000000000000000000000000000008552606060048601526064850191613741565b927fa46d8bf487ebfdbe1d611a766b6a3fcb2884d2f226b3ce629f2bf25c411bce91602484015260448301528180610120519403915afa8015610fd957610fbe575b8080610de9565b61012051610fcb91612c67565b610120516101925781610fb7565b6040513d61012051823e3d90fd5b7f78a2221c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b90919293967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608682030183528735907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61833603018212156101925760a0810191839061108981830180613c05565b809560a086525260c0840160c08660051b8601019582610120515b8281106111b45750505050506110c1602083830101838301613c05565b94908482036020860152858252602082019060208760051b84010196819361012051927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b83811061114e5750505050505091016040818101359084015260608082013590840152608090810135920191909152509660209081019493600101920190610d07565b91939599909294969750601f198482030187528935868112156101925760206111a06001936060611192888596018035845285810135868501526040810190613c58565b918160408201520190613d74565b9b0197019101918a9796959391949261110b565b91939760019193959650602061122982608061121b6111fb8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f889903018d528a613c58565b803584528581013586850152604081013560408501526060810190613c58565b918160608201520190613d74565b990195019101918895949391926110a4565b7fe6d44b4c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b61127d6004803501806130a7565b9060c052811015610a86578060051b60c0510135927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6160c051360301841215610192576101205160c05185906112d5908201806130a7565b90506112e081613113565b6112e982613113565b91610120515b81811061176b57505060c05161130b90840160208101906130a7565b92905061131783613113565b9161132184613113565b9361012051905b8082106114b7575050926113a07f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc3295936113926113ae94611384608098604051998a9960c051010135895260a060208a015260a08901906131a3565b9087820360408901526131a3565b9085820360608701526131a3565b9083820360808501526131a3565b0390a1806114ad575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08560c051013603011261019257604051906113f582612c1e565b60408560c05101013582526020820160608660c05101013581526040519261141c84612c1e565b5f84525f6020850152611432815183519061399c565b15611475578260019594926114549260206080965193015190519151926139fe565b602083015281529460c05101013561146e82608051613144565b5201610bfd565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b61010052846113b7565b91929394956114d491975060209060c05101018b60c051016130a7565b8791971015610a86578060051b870135967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1813603018812156101925760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f19811461092a57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90055908983013591610120515b8281106116aa5750509060019392917f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548560ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b1461164f575b5098602081830101359161162b61162260408484010161161a61161482878701613170565b8761352e565b848401613170565b83830135613778565b01356116378389613144565b526116428289613144565b52018a9594939291611328565b61169f6116986116a492611662856132e9565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90261012051526020610120512001548094614371565b9280614371565b613382565b8e6115ef565b909260019081851661172c57611721907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901610120515280846020610120512001557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90261012051528360206101205120015490614371565b935b811c9101611591565b611765907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901610120515283602061012051200154614371565b93611723565b909192936117819060c051018a60c051016130a7565b821015610a8657611798908260051b810190613170565b60408101356117d1815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b1561186e57506117e18135613deb565b1561183c579081602060019301359061181a6118136060830161180d6118078286613170565b8661352e565b83613170565b8235613778565b356118258387613144565b526118308287613144565b520190889392916112ef565b7f39a940c500000000000000000000000000000000000000000000000000000000610120515235600452602461012051fd5b7ff9849ea3000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b7fcf1b093c000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b34611d4c576020600319360112611d4c5767ffffffffffffffff60043511611d4c57606060031960043536030112611d4c577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c6125725760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d61197f613054565b61198d6004803501806130a7565b8091501561254a5761199e81613113565b906119a7613158565b506040516119b481612c1e565b5f81525f6020820152915f915f5b818110611f4d5750508060206119fa925160051b91012092610c3e6119f4610c316024600435016004356004016131d6565b856143b4565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2016911690808203611f1f5750611a3d90506004803501806130a7565b907f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a70054916040519181602084019460808501907f7b657df4c7ee3ef8592894761aefc80f196e5b97dd27d43a98628b2ce2ef91f0875260408601526060808601525260a0830160a08360051b85010192825f907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603015b838310611d7e578a8a60205f8c8c611af7818e03601f198101835282612c67565b604051918291518091835e8101838152039060025afa15611d41575f51611b286044600435016004356004016131d6565b80600411611d4c577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016808203611d5057505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15611d4c575f92611c2292604051958694859384937fab750e75000000000000000000000000000000000000000000000000000000008552606060048601526064850191613741565b907fa46d8bf487ebfdbe1d611a766b6a3fcb2884d2f226b3ce629f2bf25c411bce916024840152604483015203915afa8015611d4157611d2c575b5080611cbc575b507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d6101205180f35b611cc581613ea0565b15611cfb5760207f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d291604051908152a181611c64565b7fdb788c2b000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b5f611d3691612c67565b5f6101205282611c5d565b6040513d5f823e3d90fd5b5f80fd5b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9091929394957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60888203018652863582811215611d4c5783019060a0810191611dc78180613c05565b809460a085525260c0830160c08560051b85010194825f5b828110611ece575050505050611df86020820182613c05565b93908382036020850152848252602082019060208660051b8401019581935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b838110611e7e57505050506040808501359086015250505060608082013590830152608090810135910152956020908101950193926001019190611ad6565b90919293949598601f19848203018752893586811215611d4c576020611ebf6001936060611192888596018035845285810135868501526040810190613c58565b9b019701959493929101611e3f565b9091929396602080611f1283608061121b6111fb8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f60019a03018d528a613c58565b9901950193929101611ddf565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b611f5b6004803501806130a7565b821015612407578160051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6181360301821215611d4c57015f95611fa382806130a7565b979050611faf88613113565b97611fb981613113565b905f5b818110612434575050611fd260208501856130a7565b9050611fdd81613113565b61014052611fea81613113565b905f905b808210612120575050987f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32916120676120569b6113a060808901359d8e94612048604051978897885260a0602089015260a08801906131a3565b9086820360408801526131a3565b8481036060860152610140516131a3565b0390a180612118575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08336030112611d4c57604051916120aa83612c1e565b604081013583526060602084019101358152604051926120c984612c1e565b5f84525f60208501526120df815183519061399c565b1561147557916120ff9160019594936020835193015190519151926139fe565b60208301528152956121118286613144565b52016119c2565b955087612070565b90935061213060208701876130a7565b8591951015612407578060051b850135947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301861215611d4c5760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f1981146123da57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900559087830135915f5b8281106122f65750509060019392917f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548560ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b1461228f575b5096602081830101359161226d61162260408484010161161a61161482878701613170565b013561227c8361014051613144565b526122878286613144565b520190611fee565b61169f6116986122f0926122a2856132e9565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4310154938490614371565b8f612248565b9092600190818516612383577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d43183015461237891614371565b935b811c91016121ea565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f8301546123d49190614371565b9361237a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b61243e86806130a7565b82101561240757612455908260051b810190613170565b604081013561248e815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b1561251f5750803561249f81613deb565b156124f4579082916124e26001948f6124dd6124d760208701359660608101906124d26124cc8383613170565b8a61352e565b613170565b85613778565b613144565b526124ed8286613144565b5201611fbc565b7f39a940c5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34611d4c575f600319360112611d4c576125b2612fe8565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34611d4c575f600319360112611d4c57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34611d4c575f600319360112611d4c576126be612fe8565b6126c6613054565b6126ce613054565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff007fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b34611d4c575f600319360112611d4c57602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b34611d4c575f600319360112611d4c57602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b34611d4c575f600319360112611d4c5760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354604051908152f35b34611d4c575f600319360112611d4c5773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001630036128a95760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b6040600319360112611d4c576128e5612bfb565b60243567ffffffffffffffff8111611d4c57612905903690600401612cdc565b9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803014908115612b5f575b506128a957612955612fe8565b73ffffffffffffffffffffffffffffffffffffffff8116916040517f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f9181612b2b575b506129d557837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc859203612b005750823b15612ad557807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2805115612aa457612aa291613427565b005b505034612aad57005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d602011612b57575b81612b4760209383612c67565b81010312611d4c575190856129a4565b3d9150612b3a565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416141583612948565b34611d4c575f600319360112611d4c5760207f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054604051908152f35b34611d4c576020600319360112611d4c57602061091b60043561327e565b6004359073ffffffffffffffffffffffffffffffffffffffff82168203611d4c57565b6040810190811067ffffffffffffffff821117612c3a57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff821117612c3a57604052565b67ffffffffffffffff8111612c3a57601f01601f191660200190565b929192612cb282612c8a565b91612cc06040519384612c67565b829481845281830111611d4c578281602093845f960137010152565b9080601f83011215611d4c57816020612cf793359101612ca6565b90565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b919082039182116123da57565b73ffffffffffffffffffffffffffffffffffffffff168015612ded5773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611d41575f91612f98575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa908115611d41575f91612f5d575b508015612f345790565b5060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541690565b90506020813d602011612f90575b81612f7860209383612c67565b81010312611d4c57518015158103611d4c575f612f2a565b3d9150612f6b565b90506020813d602011612fe0575b81612fb360209383612c67565b81010312611d4c575173ffffffffffffffffffffffffffffffffffffffff81168103611d4c576020612ed4565b3d9150612fa6565b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416330361302857565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541661307f57565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215611d4c570180359067ffffffffffffffff8211611d4c57602001918160051b36038313611d4c57565b67ffffffffffffffff8111612c3a5760051b60200190565b9061311d826130fb565b61312a6040519182612c67565b828152601f1961313a82946130fb565b0190602036910137565b80518210156124075760209160051b010190565b6040519061316582612c1e565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215611d4c570190565b90602080835192838152019201905f5b8181106131c05750505090565b82518452602093840193909201916001016131b3565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215611d4c570180359067ffffffffffffffff8211611d4c57602001918136038313611d4c57565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c161561325657565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354811015612407577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9035f5260205f2001905f90565b8054821015612407575f5260205f2001905f90565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015468010000000000000000811015612c3a5780600161336c92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9016132d4565b5f19829392549160031b92831b921b1916179055565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025468010000000000000000811015612c3a5780600161336c92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9026132d4565b805468010000000000000000811015612c3a5761336c916001820181556132d4565b905f8091602081519101845af480806134db575b1561345b5750506040513d81523d5f602083013e60203d82010160405290565b156134a25773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d156134b3576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d15158061343b5750813b151561343b565b91908110156124075760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215611d4c570190565b906040810161353d81836130a7565b9290505f5b838110613550575050505050565b6135716135678261356186866130a7565b906134ee565b60208101906131d6565b8101606082820312611d4c5781359173ffffffffffffffffffffffffffffffffffffffff8316809303611d4c57602081013567ffffffffffffffff8111611d4c57826135be918301612cdc565b91604082013567ffffffffffffffff8111611d4c576135dd9201612cdc565b90604051917f33a89203000000000000000000000000000000000000000000000000000000008352886004840152604060248401525f83806136226044820186612cfa565b038183885af1928315611d41575f936136c5575b5082516020840120815160208301200361368a575060019392917fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf1916136816040519283928361409b565b0390a201613542565b90506136c16040519283927fc504fada0000000000000000000000000000000000000000000000000000000084526004840161409b565b0390fd5b9092503d805f833e6136d78183612c67565b810190602081830312611d4c5780519067ffffffffffffffff8211611d4c570181601f82011215611d4c5780519061370e82612c8a565b9261371c6040519485612c67565b82845260208383010111611d4c57815f9260208093018386015e83010152915f613636565b601f8260209493601f1993818652868601375f8582860101520116010190565b604090612cf7949281528160208201520191613741565b9061378381806130a7565b5f5b81811061392d5750505061379c60208201826130a7565b5f5b8181106138be575050506137b560408201826130a7565b5f5b81811061384f575050508060606137cf9201906130a7565b90915f5b8281106137e05750505050565b6137eb8184866134ee565b35906002821015611d4c576001809214613806575b016137d3565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c61383661356784888a6134ee565b906138476040519283928784613761565b0390a2613800565b61385a8183856134ee565b35906002821015611d4c576001809214613875575b016137b7565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596138a56135678487896134ee565b906138b66040519283928784613761565b0390a261386f565b6138c98183856134ee565b35906002821015611d4c5760018092146138e4575b0161379e565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f36139146135678487896134ee565b906139256040519283928784613761565b0390a26138de565b6139388183856134ee565b35906002821015611d4c576001809214613953575b01613785565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66139836135678487896134ee565b906139946040519283928784613761565b0390a261394d565b801580156139ee575b80156139e6575b80156139d6575b6139d0576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d0198210156139b3565b5081156139ac565b506401000003d0198110156139a5565b92939290915f90808303613beb5750506401000003d0195f948308613a2757505090505f905f90565b5f613a39926401000003d019926142fa565b915b8215158381613bda575b5080613bd2575b15613b74579084939293906001936401000003d0199187965b8015613b2157808404968098613af4576401000003d0199088096401000003d019036401000003d01981116123da576401000003d019908a960897938197828102928184041490151715613ac75790613abd91612d1f565b9592939695613a65565b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b5094509450509380613b475750906401000003d019809281808780098092099509900990565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b506001613a4c565b6401000003d019915014155f613a45565b613bff936401000003d019939692966140c0565b91613a3b565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe182360301811215611d4c57016020813591019167ffffffffffffffff8211611d4c578160051b36038313611d4c57565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8182360301811215611d4c570190565b906020838281520160208260051b85010193835f915b848310613cb05750505050505090565b909192939495601f1982820301855286357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc184360301811215611d4c57830180356002811015611d4c57825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe182360301811215611d4c57016020813591019067ffffffffffffffff8111611d4c578036038213611d4c57613d666020928392604086818660019901520191613741565b980196950193019190613ca0565b612cf791613ddd613dd2613db7613d9c613d8e8680613c05565b608087526080870191613c8a565b613da96020870187613c05565b908683036020880152613c8a565b613dc46040860186613c05565b908583036040870152613c8a565b926060810190613c05565b916060818503910152613c8a565b805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2054155f14613e9b57613e48817f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00613405565b7f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054905f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2055600190565b505f90565b805f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054155f14613e9b57613efd817f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613405565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354905f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2055600190565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e54614097576140077fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613405565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903547fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e55600190565b5f90565b90916140b2612cf793604084526040840190612cfa565b916020818403910152612cfa565b94929093918515806142f2575b6142e7578015806142df575b6142d7576040516080916140ed8383612c67565b8236833784156142aa5784600180098083529385856001099860208401998a52604084019286845287845160010993606086019485526040519a878c01958c871067ffffffffffffffff881117612c3a578a80979581969482956040525190098d525190099460208b019586525190099860408901998a5251900960608701908152865188511480159061429e575b1561424057849283808093816040519c856141988f9788612c67565b368737518c516141a89083612d1f565b900884525185516141b99083612d1f565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a5251800988516141f69083612d1f565b9008818087518551900960020961420d9083612d1f565b90089c519351905190096142218c83612d1f565b900890099251905190096142359083612d1f565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b5081518151141561417c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b505050600190565b5081156140d9565b945092506001919050565b5084156140cd565b939192909281156142aa5760018280614367818080809781806143579e818f81818192099a8b96818f8180828193099a8809600409988009900992800960030908918161434a8183800882612d1f565b81858009089e8f83612d1f565b9008900993800960080983612d1f565b9008940960020990565b5f9060209260405190848201928352604082015260408152614394606082612c67565b604051918291518091835e8101838152039060025afa15611d41575f5190565b81519190604183036143e4576143dd9250602082015190606060408401519301515f1a906144c6565b9192909190565b50505f9160029190565b60048110156144995780614400575050565b60018103614430577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002810361446457507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b60031461446e5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161454a579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15611d41575f5173ffffffffffffffffffffffffffffffffffffffff81161561454057905f905f90565b505f906001905f90565b5050505f916003919056fea164736f6c6343000824000af0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE04a\x01\x0EW`\x1FaG?8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x12W\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x01\x0EW\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x82\x82\x03a\x01\x0EW` \x01Q\x91`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x90\x81\x84\x03a\x01\x0EW0`\x80Ra\0ua\x01&V[a\0}a\x01&V[\x15a\0\xFFW\x15a\0\xF0W`\xA0R`\xC0Ra\0\x95a\x01&V[`@QaEb\x90\x81a\x01\xBD\x829`\x80Q\x81\x81\x81a(Y\x01Ra)\x1D\x01R`\xA0Q\x81\x81\x81a\x0F\x11\x01R\x81\x81a\x1B\xB7\x01R\x81\x81a&\x82\x01Ra.\xA4\x01R`\xC0Q\x81\x81\x81a\x02\xD7\x01R\x81\x81a\r\xB7\x01R\x81\x81a\x1Bw\x01Ra.b\x01R\xF3[c\xB1\xB2Z\xAF`\xE0\x1B_R`\x04_\xFD[cSl\x15\x01`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_Q` aG\x1F_9_Q\x90_RT`\xFF\x81`@\x1C\x16a\x01\xADW`\x02`\x01`@\x1B\x03\x19`\x01`\x01`@\x1B\x03\x82\x16\x01a\x01[WPV[`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17_Q` aG\x1F_9_Q\x90_RU`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1V[c\xF9.\xE8\xA9`\xE0\x1B_R`\x04_\xFD\xFEa\x01``@R`\x046\x10\x15a\0\x12W_\x80\xFD[_a\x01 R_5`\xE0\x1C\x80c1\xEEbB\x14a+\xDDW\x80c@\xF3MB\x14a+\xA1W\x80cO\x1E\xF2\x86\x14a(\xD1W\x80cR\xD1\x90-\x14a(2W\x80cY\xBA\x92X\x14a'\xF6W\x80c\\`\xDA\x1B\x14a'\xA4W\x80c\\\x97Z\xBB\x14a'cW\x80cc\xA5\x99\xA4\x14a&\xA6W\x80ck\xF0\xA0K\x14a&VW\x80cqP\x18\xA6\x14a%\x9AW\x80cs\xAB\x99\x16\x14a\x18\xFBW\x80c\x87\t>\xBA\x14a\x0B\x0EW\x80c\x8D\xA5\xCB[\x14a\n\xB9W\x80c\x9A\xD9\x1DL\x14a\n\x02W\x80c\xA0`V\xF7\x14a\t\xC0W\x80c\xAD<\xB1\xCC\x14a\t]W\x80c\xBD\xEBD-\x14a\x08\xD0W\x80c\xC0%0#\x14a\x08,W\x80c\xC1\xB0\xBE\xD7\x14a\x07\xD6W\x80c\xC4IV\xD1\x14a\x07\x97W\x80c\xC4\xD6m\xE8\x14a\x03DW\x80c\xC8y\xDB\xE4\x14a\x02\xFBW\x80c\xE3Z]/\x14a\x02\x9CW\x80c\xF2\xFD\xE3\x8B\x14a\x02lW\x80c\xFD\xDDH7\x14a\x02EW\x80c\xFE\x18\xAB\x91\x14a\x02\0W\x80c\xFF\xA1\xADt\x14a\x01\x99Wc\xFF\xC3?r\x14a\x01SW_\x80\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` \x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0T`@Q\x90\x81R\xF3[a\x01 Q\x80\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W`@\x80Qa\x01\xFC\x91a\x01\xBE\x90\x82a,gV[`\r\x81R\x7F2.0.0-alpha.6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a,\xFAV[\x03\x90\xF3[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` a\x02ba.\x19V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92Wa\x02\x95a\x02\x88a+\xFBV[a\x02\x90a/\xE8V[a-,V[a\x01 Q\x80\xF3[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92W` a\x02b`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92Wa\x03]a+\xFBV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16\x15\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x15\x90\x81a\x07\x8FW[`\x01\x14\x90\x81a\x07\x85W[\x15\x90\x81a\x07|W[Pa\x07NW\x81\x83`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x04\x16\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x06\xF9W[Pa\x04\x0Ea2'V[a\x02\x90a2'V[a\x04\x1Ea2'V[a\x04&a2'V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x06\xC6W\x80`\x01a\x04\xA9\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a2\xD4V[\x81T\x90`\x03\x1B\x90_\x19\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x83\x1B\x92\x1B\x19\x16\x17\x90Ua\x01 Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ua\x05\na?PV[P\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x05]a2'V[\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0U\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x05\xF3a.\x19V[a\x06\x98Wa\x06\x02Wa\x01 Q\x80\xF3[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1a\x02\x95V[\x7F\x0B\x1D8\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`A`\x04R`$a\x01 Q\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x83a\x04\x05V[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x90P\x15\x84a\x03\xAEV[0;\x15\x91Pa\x03\xA6V[\x84\x91Pa\x03\x9CV[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92Wa\x01 QP`\x045a\x01 QR\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R` `@a\x01 Q T\x15\x15`@Q\x90\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92W`\x045a\x08Ha/\xE8V[\x80\x15a\x08\xA2W\x80\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x01 Q\x80\xF3[\x7FwM]\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T_\x19\x81\x01\x90\x81\x11a\t*Wa\t\x1B` \x91a2~V[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x11`\x04R`$a\x01 Q\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W`@\x80Qa\x01\xFC\x91a\t\x82\x90\x82a,gV[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a,\xFAV[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92W\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`\x045\x90\x81\x10\x15a\n\x86W` \x90\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a\x01 QR\x81a\x01 Q \x01a\x01 QPTa\x01 Q`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`2`\x04R`$a\x01 Q\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x01\x92W`@`\x03\x196\x01\x12a\x01\x92Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x01\x92W```\x03\x19`\x0456\x03\x01\x12a\x01\x92W`$5`\xA0R`\xA0Q\x15\x15`\xA0Q\x03a\x01\x92WZ`\xE0R\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x18\xCDW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x0B\xA9a0TV[a\x0B\xB7`\x04\x805\x01\x80a0\xA7V[\x80\x91P\x15a\x18\x9FWa\x0B\xC8\x81a1\x13V[`\x80Ra\x0B\xD3a1XV[P`@Qa\x0B\xE0\x81a,\x1EV[a\x01 Q\x81Ra\x01 Q` \x82\x01R\x90a\x01 Qa\x01\0Ra\x01 Q[\x81\x81\x10a\x12oW\x82a\x0CG`\x80QQ`\x05\x1B` `\x80Q\x01 \x91a\x0C>a\x0C8a\x0C1`$`\x045\x01`\x045`\x04\x01a1\xD6V[6\x91a,\xA6V[\x84aC\xB4V[\x90\x93\x91\x93aC\xEEV[` \x81Q\x91\x01Q\x90a\x01 QR` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@a\x01 Q \x16\x91\x16\x90\x80\x82\x03a\x12;W\x82a\x0C\x8E`\x04\x805\x01\x80a0\xA7V[\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0\x92\x91\x92T\x90\x80`@Q\x92`\x80\x84\x01\x90\x7F{e}\xF4\xC7\xEE>\xF8Y(\x94v\x1A\xEF\xC8\x0F\x19n[\x97\xDD'\xD4:\x98b\x8B,\xE2\xEF\x91\xF0` \x86\x01R`@\x85\x01R``\x80\x85\x01RR`\xA0\x82\x01\x90`\xA0\x81`\x05\x1B\x84\x01\x01\x94\x80\x92a\x01 Q\x91[\x83\x83\x10a\x10\x1BW\x86` \x87a\r%\x81\x8C\x03`\x1F\x19\x81\x01\x83R\x82a,gV[\x81`@Q\x91\x80Q\x91\x82\x91\x01\x83^\x81\x01\x90a\x01 Q\x82R\x80a\x01 Q\x92\x03\x90`\x02Z\xFA\x15a\x0F\xD9Wa\x01 QQa\re`D`\x045\x01`\x045`\x04\x01a1\xD6V[\x80`\x04\x93\x92\x93\x11a\x01\x92W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x835\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x0F\xE7WPP`\xA0Q\x15a\x0E\xFAW[PPPa\x01\0Qa\x0E\x82W[\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x0EQZ`\xE0Qa-\x1FV[\x7Fo\x14\x981\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[a\x0E\x8Ea\x01\0Qa>\xA0V[\x15a\x0E\xC5W\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Qa\x01\0Q\x81R\xA1a\r\xF5V[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QRa\x01\0Q`\x04R`$a\x01 Q\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x01\x92Wa\x0Fu`@Q\x94\x85\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a7AV[\x92\x7F\xA4m\x8B\xF4\x87\xEB\xFD\xBE\x1Da\x1Avkj?\xCB(\x84\xD2\xF2&\xB3\xCEb\x9F+\xF2\\A\x1B\xCE\x91`$\x84\x01R`D\x83\x01R\x81\x80a\x01 Q\x94\x03\x91Z\xFA\x80\x15a\x0F\xD9Wa\x0F\xBEW[\x80\x80a\r\xE9V[a\x01 Qa\x0F\xCB\x91a,gV[a\x01 Qa\x01\x92W\x81a\x0F\xB7V[`@Q=a\x01 Q\x82>=\x90\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[\x90\x91\x92\x93\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x86\x82\x03\x01\x83R\x875\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x82\x12\x15a\x01\x92W`\xA0\x81\x01\x91\x83\x90a\x10\x89\x81\x83\x01\x80a<\x05V[\x80\x95`\xA0\x86RR`\xC0\x84\x01`\xC0\x86`\x05\x1B\x86\x01\x01\x95\x82a\x01 Q[\x82\x81\x10a\x11\xB4WPPPPPa\x10\xC1` \x83\x83\x01\x01\x83\x83\x01a<\x05V[\x94\x90\x84\x82\x03` \x86\x01R\x85\x82R` \x82\x01\x90` \x87`\x05\x1B\x84\x01\x01\x96\x81\x93a\x01 Q\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x11NWPPPPPP\x91\x01`@\x81\x81\x015\x90\x84\x01R``\x80\x82\x015\x90\x84\x01R`\x80\x90\x81\x015\x92\x01\x91\x90\x91RP\x96` \x90\x81\x01\x94\x93`\x01\x01\x92\x01\x90a\r\x07V[\x91\x93\x95\x99\x90\x92\x94\x96\x97P`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x01\x92W` a\x11\xA0`\x01\x93``a\x11\x92\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90a<XV[\x91\x81`@\x82\x01R\x01\x90a=tV[\x9B\x01\x97\x01\x91\x01\x91\x8A\x97\x96\x95\x93\x91\x94\x92a\x11\x0BV[\x91\x93\x97`\x01\x91\x93\x95\x96P` a\x12)\x82`\x80a\x12\x1Ba\x11\xFB\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F\x88\x99\x03\x01\x8DR\x8Aa<XV[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90a<XV[\x91\x81``\x82\x01R\x01\x90a=tV[\x99\x01\x95\x01\x91\x01\x91\x88\x95\x94\x93\x91\x92a\x10\xA4V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[a\x12}`\x04\x805\x01\x80a0\xA7V[\x90`\xC0R\x81\x10\x15a\n\x86W\x80`\x05\x1B`\xC0Q\x015\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa`\xC0Q6\x03\x01\x84\x12\x15a\x01\x92Wa\x01 Q`\xC0Q\x85\x90a\x12\xD5\x90\x82\x01\x80a0\xA7V[\x90Pa\x12\xE0\x81a1\x13V[a\x12\xE9\x82a1\x13V[\x91a\x01 Q[\x81\x81\x10a\x17kWPP`\xC0Qa\x13\x0B\x90\x84\x01` \x81\x01\x90a0\xA7V[\x92\x90Pa\x13\x17\x83a1\x13V[\x91a\x13!\x84a1\x13V[\x93a\x01 Q\x90[\x80\x82\x10a\x14\xB7WPP\x92a\x13\xA0\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a\x13\x92a\x13\xAE\x94a\x13\x84`\x80\x98`@Q\x99\x8A\x99`\xC0Q\x01\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a1\xA3V[\x90\x87\x82\x03`@\x89\x01Ra1\xA3V[\x90\x85\x82\x03``\x87\x01Ra1\xA3V[\x90\x83\x82\x03`\x80\x85\x01Ra1\xA3V[\x03\x90\xA1\x80a\x14\xADW[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x85`\xC0Q\x016\x03\x01\x12a\x01\x92W`@Q\x90a\x13\xF5\x82a,\x1EV[`@\x85`\xC0Q\x01\x015\x82R` \x82\x01``\x86`\xC0Q\x01\x015\x81R`@Q\x92a\x14\x1C\x84a,\x1EV[_\x84R_` \x85\x01Ra\x142\x81Q\x83Q\x90a9\x9CV[\x15a\x14uW\x82`\x01\x95\x94\x92a\x14T\x92` `\x80\x96Q\x93\x01Q\x90Q\x91Q\x92a9\xFEV[` \x83\x01R\x81R\x94`\xC0Q\x01\x015a\x14n\x82`\x80Qa1DV[R\x01a\x0B\xFDV[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[a\x01\0R\x84a\x13\xB7V[\x91\x92\x93\x94\x95a\x14\xD4\x91\x97P` \x90`\xC0Q\x01\x01\x8B`\xC0Q\x01a0\xA7V[\x87\x91\x97\x10\x15a\n\x86W\x80`\x05\x1B\x87\x015\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x88\x12\x15a\x01\x92W`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a\t*W`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x89\x83\x015\x91a\x01 Q[\x82\x81\x10a\x16\xAAWPP\x90`\x01\x93\x92\x91\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x85`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\x16OW[P\x98` \x81\x83\x01\x015\x91a\x16+a\x16\"`@\x84\x84\x01\x01a\x16\x1Aa\x16\x14\x82\x87\x87\x01a1pV[\x87a5.V[\x84\x84\x01a1pV[\x83\x83\x015a7xV[\x015a\x167\x83\x89a1DV[Ra\x16B\x82\x89a1DV[R\x01\x8A\x95\x94\x93\x92\x91a\x13(V[a\x16\x9Fa\x16\x98a\x16\xA4\x92a\x16b\x85a2\xE9V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR` a\x01 Q \x01T\x80\x94aCqV[\x92\x80aCqV[a3\x82V[\x8Ea\x15\xEFV[\x90\x92`\x01\x90\x81\x85\x16a\x17,Wa\x17!\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x80\x84` a\x01 Q \x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR\x83` a\x01 Q \x01T\x90aCqV[\x93[\x81\x1C\x91\x01a\x15\x91V[a\x17e\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x83` a\x01 Q \x01TaCqV[\x93a\x17#V[\x90\x91\x92\x93a\x17\x81\x90`\xC0Q\x01\x8A`\xC0Q\x01a0\xA7V[\x82\x10\x15a\n\x86Wa\x17\x98\x90\x82`\x05\x1B\x81\x01\x90a1pV[`@\x81\x015a\x17\xD1\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a\x18nWPa\x17\xE1\x815a=\xEBV[\x15a\x18<W\x90\x81` `\x01\x93\x015\x90a\x18\x1Aa\x18\x13``\x83\x01a\x18\ra\x18\x07\x82\x86a1pV[\x86a5.V[\x83a1pV[\x825a7xV[5a\x18%\x83\x87a1DV[Ra\x180\x82\x87a1DV[R\x01\x90\x88\x93\x92\x91a\x12\xEFV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR5`\x04R`$a\x01 Q\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x1DLW` `\x03\x196\x01\x12a\x1DLWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x1DLW```\x03\x19`\x0456\x03\x01\x12a\x1DLW\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a%rW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x19\x7Fa0TV[a\x19\x8D`\x04\x805\x01\x80a0\xA7V[\x80\x91P\x15a%JWa\x19\x9E\x81a1\x13V[\x90a\x19\xA7a1XV[P`@Qa\x19\xB4\x81a,\x1EV[_\x81R_` \x82\x01R\x91_\x91_[\x81\x81\x10a\x1FMWPP\x80` a\x19\xFA\x92Q`\x05\x1B\x91\x01 \x92a\x0C>a\x19\xF4a\x0C1`$`\x045\x01`\x045`\x04\x01a1\xD6V[\x85aC\xB4V[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a\x1F\x1FWPa\x1A=\x90P`\x04\x805\x01\x80a0\xA7V[\x90\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0T\x91`@Q\x91\x81` \x84\x01\x94`\x80\x85\x01\x90\x7F{e}\xF4\xC7\xEE>\xF8Y(\x94v\x1A\xEF\xC8\x0F\x19n[\x97\xDD'\xD4:\x98b\x8B,\xE2\xEF\x91\xF0\x87R`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01`\xA0\x83`\x05\x1B\x85\x01\x01\x92\x82_\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01[\x83\x83\x10a\x1D~W\x8A\x8A` _\x8C\x8Ca\x1A\xF7\x81\x8E\x03`\x1F\x19\x81\x01\x83R\x82a,gV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x1DAW_Qa\x1B(`D`\x045\x01`\x045`\x04\x01a1\xD6V[\x80`\x04\x11a\x1DLW\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x1DPWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x1DLW_\x92a\x1C\"\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a7AV[\x90\x7F\xA4m\x8B\xF4\x87\xEB\xFD\xBE\x1Da\x1Avkj?\xCB(\x84\xD2\xF2&\xB3\xCEb\x9F+\xF2\\A\x1B\xCE\x91`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a\x1DAWa\x1D,W[P\x80a\x1C\xBCW[P\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x01 Q\x80\xF3[a\x1C\xC5\x81a>\xA0V[\x15a\x1C\xFBW` \x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x91`@Q\x90\x81R\xA1\x81a\x1CdV[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[_a\x1D6\x91a,gV[_a\x01 R\x82a\x1C]V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x88\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x1DLW\x83\x01\x90`\xA0\x81\x01\x91a\x1D\xC7\x81\x80a<\x05V[\x80\x94`\xA0\x85RR`\xC0\x83\x01`\xC0\x85`\x05\x1B\x85\x01\x01\x94\x82_[\x82\x81\x10a\x1E\xCEWPPPPPa\x1D\xF8` \x82\x01\x82a<\x05V[\x93\x90\x83\x82\x03` \x85\x01R\x84\x82R` \x82\x01\x90` \x86`\x05\x1B\x84\x01\x01\x95\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x1E~WPPPP`@\x80\x85\x015\x90\x86\x01RPPP``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01R\x95` \x90\x81\x01\x95\x01\x93\x92`\x01\x01\x91\x90a\x1A\xD6V[\x90\x91\x92\x93\x94\x95\x98`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x1DLW` a\x1E\xBF`\x01\x93``a\x11\x92\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90a<XV[\x9B\x01\x97\x01\x95\x94\x93\x92\x91\x01a\x1E?V[\x90\x91\x92\x93\x96` \x80a\x1F\x12\x83`\x80a\x12\x1Ba\x11\xFB\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F`\x01\x9A\x03\x01\x8DR\x8Aa<XV[\x99\x01\x95\x01\x93\x92\x91\x01a\x1D\xDFV[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[a\x1F[`\x04\x805\x01\x80a0\xA7V[\x82\x10\x15a$\x07W\x81`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a\x1DLW\x01_\x95a\x1F\xA3\x82\x80a0\xA7V[\x97\x90Pa\x1F\xAF\x88a1\x13V[\x97a\x1F\xB9\x81a1\x13V[\x90_[\x81\x81\x10a$4WPPa\x1F\xD2` \x85\x01\x85a0\xA7V[\x90Pa\x1F\xDD\x81a1\x13V[a\x01@Ra\x1F\xEA\x81a1\x13V[\x90_\x90[\x80\x82\x10a! WPP\x98\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x91a ga V\x9Ba\x13\xA0`\x80\x89\x015\x9D\x8E\x94a H`@Q\x97\x88\x97\x88R`\xA0` \x89\x01R`\xA0\x88\x01\x90a1\xA3V[\x90\x86\x82\x03`@\x88\x01Ra1\xA3V[\x84\x81\x03``\x86\x01Ra\x01@Qa1\xA3V[\x03\x90\xA1\x80a!\x18W[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x836\x03\x01\x12a\x1DLW`@Q\x91a \xAA\x83a,\x1EV[`@\x81\x015\x83R``` \x84\x01\x91\x015\x81R`@Q\x92a \xC9\x84a,\x1EV[_\x84R_` \x85\x01Ra \xDF\x81Q\x83Q\x90a9\x9CV[\x15a\x14uW\x91a \xFF\x91`\x01\x95\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92a9\xFEV[` \x83\x01R\x81R\x95a!\x11\x82\x86a1DV[R\x01a\x19\xC2V[\x95P\x87a pV[\x90\x93Pa!0` \x87\x01\x87a0\xA7V[\x85\x91\x95\x10\x15a$\x07W\x80`\x05\x1B\x85\x015\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x86\x12\x15a\x1DLW`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a#\xDAW`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x87\x83\x015\x91_[\x82\x81\x10a\"\xF6WPP\x90`\x01\x93\x92\x91\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x85`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\"\x8FW[P\x96` \x81\x83\x01\x015\x91a\"ma\x16\"`@\x84\x84\x01\x01a\x16\x1Aa\x16\x14\x82\x87\x87\x01a1pV[\x015a\"|\x83a\x01@Qa1DV[Ra\"\x87\x82\x86a1DV[R\x01\x90a\x1F\xEEV[a\x16\x9Fa\x16\x98a\"\xF0\x92a\"\xA2\x85a2\xE9V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x01T\x93\x84\x90aCqV[\x8Fa\"HV[\x90\x92`\x01\x90\x81\x85\x16a#\x83W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta#x\x91aCqV[\x93[\x81\x1C\x91\x01a!\xEAV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta#\xD4\x91\x90aCqV[\x93a#zV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a$>\x86\x80a0\xA7V[\x82\x10\x15a$\x07Wa$U\x90\x82`\x05\x1B\x81\x01\x90a1pV[`@\x81\x015a$\x8E\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a%\x1FWP\x805a$\x9F\x81a=\xEBV[\x15a$\xF4W\x90\x82\x91a$\xE2`\x01\x94\x8Fa$\xDDa$\xD7` \x87\x015\x96``\x81\x01\x90a$\xD2a$\xCC\x83\x83a1pV[\x8Aa5.V[a1pV[\x85a7xV[a1DV[Ra$\xED\x82\x86a1DV[R\x01a\x1F\xBCV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x1DLW_`\x03\x196\x01\x12a\x1DLWa%\xB2a/\xE8V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLWa&\xBEa/\xE8V[a&\xC6a0TV[a&\xCEa0TV[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T`@Q\x90\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a(\xA9W` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@`\x03\x196\x01\x12a\x1DLWa(\xE5a+\xFBV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLWa)\x05\x906\x90`\x04\x01a,\xDCV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a+_W[Pa(\xA9Wa)Ua/\xE8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a++W[Pa)\xD5W\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a+\0WP\x82;\x15a*\xD5W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a*\xA4Wa*\xA2\x91a4'V[\0[PP4a*\xADW\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a+WW[\x81a+G` \x93\x83a,gV[\x81\x01\x03\x12a\x1DLWQ\x90\x85a)\xA4V[=\x91Pa+:V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a)HV[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` \x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`@Q\x90\x81R\xF3[4a\x1DLW` `\x03\x196\x01\x12a\x1DLW` a\t\x1B`\x045a2~V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1DLWV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a,:W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a,:W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a,:W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a,\xB2\x82a,\x8AV[\x91a,\xC0`@Q\x93\x84a,gV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x1DLW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x1DLW\x81` a,\xF7\x935\x91\x01a,\xA6V[\x90V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x03\x91\x82\x11a#\xDAWV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a-\xEDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x1DAW_\x91a/\x98W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a\x1DAW_\x91a/]W[P\x80\x15a/4W\x90V[P`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x90V[\x90P` \x81=` \x11a/\x90W[\x81a/x` \x93\x83a,gV[\x81\x01\x03\x12a\x1DLWQ\x80\x15\x15\x81\x03a\x1DLW_a/*V[=\x91Pa/kV[\x90P` \x81=` \x11a/\xE0W[\x81a/\xB3` \x93\x83a,gV[\x81\x01\x03\x12a\x1DLWQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x1DLW` a.\xD4V[=\x91Pa/\xA6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a0(WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a0\x7FWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x1DLWV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a,:W`\x05\x1B` \x01\x90V[\x90a1\x1D\x82a0\xFBV[a1*`@Q\x91\x82a,gV[\x82\x81R`\x1F\x19a1:\x82\x94a0\xFBV[\x01\x90` 6\x91\x017V[\x80Q\x82\x10\x15a$\x07W` \x91`\x05\x1B\x01\x01\x90V[`@Q\x90a1e\x82a,\x1EV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a1\xC0WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\xB3V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW` \x01\x91\x816\x03\x83\x13a\x1DLWV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a2VWV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x81\x10\x15a$\x07W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03_R` _ \x01\x90_\x90V[\x80T\x82\x10\x15a$\x07W_R` _ \x01\x90_\x90V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a,:W\x80`\x01a3l\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a2\xD4V[_\x19\x82\x93\x92T\x91`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90UV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a,:W\x80`\x01a3l\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a2\xD4V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a,:Wa3l\x91`\x01\x82\x01\x81Ua2\xD4V[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80a4\xDBW[\x15a4[WPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15a4\xA2Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15a4\xB3W`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80a4;WP\x81;\x15\x15a4;V[\x91\x90\x81\x10\x15a$\x07W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x90V[\x90`@\x81\x01a5=\x81\x83a0\xA7V[\x92\x90P_[\x83\x81\x10a5PWPPPPPV[a5qa5g\x82a5a\x86\x86a0\xA7V[\x90a4\xEEV[` \x81\x01\x90a1\xD6V[\x81\x01``\x82\x82\x03\x12a\x1DLW\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x1DLW` \x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLW\x82a5\xBE\x91\x83\x01a,\xDCV[\x91`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLWa5\xDD\x92\x01a,\xDCV[\x90`@Q\x91\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01R`@`$\x84\x01R_\x83\x80a6\"`D\x82\x01\x86a,\xFAV[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x1DAW_\x93a6\xC5W[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03a6\x8AWP`\x01\x93\x92\x91\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91a6\x81`@Q\x92\x83\x92\x83a@\x9BV[\x03\x90\xA2\x01a5BV[\x90Pa6\xC1`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a@\x9BV[\x03\x90\xFD[\x90\x92P=\x80_\x83>a6\xD7\x81\x83a,gV[\x81\x01\x90` \x81\x83\x03\x12a\x1DLW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW\x01\x81`\x1F\x82\x01\x12\x15a\x1DLW\x80Q\x90a7\x0E\x82a,\x8AV[\x92a7\x1C`@Q\x94\x85a,gV[\x82\x84R` \x83\x83\x01\x01\x11a\x1DLW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_a66V[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a,\xF7\x94\x92\x81R\x81` \x82\x01R\x01\x91a7AV[\x90a7\x83\x81\x80a0\xA7V[_[\x81\x81\x10a9-WPPPa7\x9C` \x82\x01\x82a0\xA7V[_[\x81\x81\x10a8\xBEWPPPa7\xB5`@\x82\x01\x82a0\xA7V[_[\x81\x81\x10a8OWPPP\x80``a7\xCF\x92\x01\x90a0\xA7V[\x90\x91_[\x82\x81\x10a7\xE0WPPPPV[a7\xEB\x81\x84\x86a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a8\x06W[\x01a7\xD3V[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca86a5g\x84\x88\x8Aa4\xEEV[\x90a8G`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a8\0V[a8Z\x81\x83\x85a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a8uW[\x01a7\xB7V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa8\xA5a5g\x84\x87\x89a4\xEEV[\x90a8\xB6`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a8oV[a8\xC9\x81\x83\x85a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a8\xE4W[\x01a7\x9EV[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3a9\x14a5g\x84\x87\x89a4\xEEV[\x90a9%`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a8\xDEV[a98\x81\x83\x85a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a9SW[\x01a7\x85V[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6a9\x83a5g\x84\x87\x89a4\xEEV[\x90a9\x94`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a9MV[\x80\x15\x80\x15a9\xEEW[\x80\x15a9\xE6W[\x80\x15a9\xD6W[a9\xD0Wd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15a9\xB3V[P\x81\x15a9\xACV[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15a9\xA5V[\x92\x93\x92\x90\x91_\x90\x80\x83\x03a;\xEBWPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08a:'WPP\x90P_\x90_\x90V[_a:9\x92d\x01\0\0\x03\xD0\x19\x92aB\xFAV[\x91[\x82\x15\x15\x83\x81a;\xDAW[P\x80a;\xD2W[\x15a;tW\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15a;!W\x80\x84\x04\x96\x80\x98a:\xF4Wd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a#\xDAWd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a:\xC7W\x90a:\xBD\x91a-\x1FV[\x95\x92\x93\x96\x95a:eV[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80a;GWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01a:LV[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_a:EV[a;\xFF\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96a@\xC0V[\x91a:;V[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x1DLW\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW\x81`\x05\x1B6\x03\x83\x13a\x1DLWV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a\x1DLW\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91[\x84\x83\x10a<\xB0WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95`\x1F\x19\x82\x82\x03\x01\x85R\x865\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x846\x03\x01\x81\x12\x15a\x1DLW\x83\x01\x805`\x02\x81\x10\x15a\x1DLW\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x1DLW\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLW\x806\x03\x82\x13a\x1DLWa=f` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91a7AV[\x98\x01\x96\x95\x01\x93\x01\x91\x90a<\xA0V[a,\xF7\x91a=\xDDa=\xD2a=\xB7a=\x9Ca=\x8E\x86\x80a<\x05V[`\x80\x87R`\x80\x87\x01\x91a<\x8AV[a=\xA9` \x87\x01\x87a<\x05V[\x90\x86\x83\x03` \x88\x01Ra<\x8AV[a=\xC4`@\x86\x01\x86a<\x05V[\x90\x85\x83\x03`@\x87\x01Ra<\x8AV[\x92``\x81\x01\x90a<\x05V[\x91``\x81\x85\x03\x91\x01Ra<\x8AV[\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ T\x15_\x14a>\x9BWa>H\x81\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a4\x05V[\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T\x90_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ U`\x01\x90V[P_\x90V[\x80_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15_\x14a>\x9BWa>\xFD\x81\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a4\x05V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x90_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ U`\x01\x90V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nTa@\x97Wa@\x07\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a4\x05V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nU`\x01\x90V[_\x90V[\x90\x91a@\xB2a,\xF7\x93`@\x84R`@\x84\x01\x90a,\xFAV[\x91` \x81\x84\x03\x91\x01Ra,\xFAV[\x94\x92\x90\x93\x91\x85\x15\x80aB\xF2W[aB\xE7W\x80\x15\x80aB\xDFW[aB\xD7W`@Q`\x80\x91a@\xED\x83\x83a,gV[\x826\x837\x84\x15aB\xAAW\x84`\x01\x80\t\x80\x83R\x93\x85\x85`\x01\t\x98` \x84\x01\x99\x8AR`@\x84\x01\x92\x86\x84R\x87\x84Q`\x01\t\x93``\x86\x01\x94\x85R`@Q\x9A\x87\x8C\x01\x95\x8C\x87\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x11\x17a,:W\x8A\x80\x97\x95\x81\x96\x94\x82\x95`@RQ\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90aB\x9EW[\x15aB@W\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85aA\x98\x8F\x97\x88a,gV[6\x877Q\x8CQaA\xA8\x90\x83a-\x1FV[\x90\x08\x84RQ\x85QaA\xB9\x90\x83a-\x1FV[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88QaA\xF6\x90\x83a-\x1FV[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\taB\r\x90\x83a-\x1FV[\x90\x08\x9CQ\x93Q\x90Q\x90\taB!\x8C\x83a-\x1FV[\x90\x08\x90\t\x92Q\x90Q\x90\taB5\x90\x83a-\x1FV[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15aA|V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[PPP`\x01\x90V[P\x81\x15a@\xD9V[\x94P\x92P`\x01\x91\x90PV[P\x84\x15a@\xCDV[\x93\x91\x92\x90\x92\x81\x15aB\xAAW`\x01\x82\x80aCg\x81\x80\x80\x80\x97\x81\x80aCW\x9E\x81\x8F\x81\x81\x81\x92\t\x9A\x8B\x96\x81\x8F\x81\x80\x82\x81\x93\t\x9A\x88\t`\x04\t\x98\x80\t\x90\t\x92\x80\t`\x03\t\x08\x91\x81aCJ\x81\x83\x80\x08\x82a-\x1FV[\x81\x85\x80\t\x08\x9E\x8F\x83a-\x1FV[\x90\x08\x90\t\x93\x80\t`\x08\t\x83a-\x1FV[\x90\x08\x94\t`\x02\t\x90V[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81RaC\x94``\x82a,gV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x1DAW_Q\x90V[\x81Q\x91\x90`A\x83\x03aC\xE4WaC\xDD\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aD\xC6V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aD\x99W\x80aD\0WPPV[`\x01\x81\x03aD0W\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03aDdWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14aDnWPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aEJW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x1DAW_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aE@W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V\xFE\xA1dsolcC\0\x08$\0\n\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6101606040526004361015610012575f80fd5b5f610120525f3560e01c806331ee624214612bdd57806340f34d4214612ba15780634f1ef286146128d157806352d1902d1461283257806359ba9258146127f65780635c60da1b146127a45780635c975abb1461276357806363a599a4146126a65780636bf0a04b14612656578063715018a61461259a57806373ab9916146118fb57806387093eba14610b0e5780638da5cb5b14610ab95780639ad91d4c14610a02578063a06056f7146109c0578063ad3cb1cc1461095d578063bdeb442d146108d0578063c02530231461082c578063c1b0bed7146107d6578063c44956d114610797578063c4d66de814610344578063c879dbe4146102fb578063e35a5d2f1461029c578063f2fde38b1461026c578063fddd483714610245578063fe18ab9114610200578063ffa1ad74146101995763ffc33f7214610153575f80fd5b3461019257610120516003193601126101925760207f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a70054604051908152f35b6101205180fd5b34610192576101205160031936011261019257604080516101fc916101be9082612c67565b600d81527f322e302e302d616c7068612e36000000000000000000000000000000000000006020820152604051918291602083526020830190612cfa565b0390f35b346101925761012051600319360112610192576020600160ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b604051908152f35b346101925761012051600319360112610192576020610262612e19565b6040519015158152f35b3461019257602060031936011261019257610295610288612bfb565b610290612fe8565b612d2c565b6101205180f35b3461019257610120516003193601126101925760206040517fffffffff000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101925760206003193601126101925760206102626004355f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b346101925760206003193601126101925761035d612bfb565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460ff8160401c16159167ffffffffffffffff82168015908161078f575b6001149081610785575b15908161077c575b5061074e57818360017fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000006104169516177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00556106f9575b5061040e613227565b610290613227565b61041e613227565b610426613227565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90254680100000000000000008110156106c6578060016104a992017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9026132d4565b81549060031b905f197fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b06831b921b1916179055610120517f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9005561050a613f50565b507f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d260206040517fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b068152a161055d613227565b7fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a700557fe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26105f3612e19565b61069857610602576101205180f35b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054167ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160018152a1610295565b7f0b1d38a3000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526041600452602461012051fd5b7fffffffffffffffffffffffffffffffffffffffffffffff0000000000000000001668010000000000000001177ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005583610405565b7ff92ee8a9000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b905015846103ae565b303b1591506103a6565b84915061039c565b3461019257610120516003193601126101925760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90054604051908152f35b3461019257602060031936011261019257610120515060043561012051527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc01602052602060406101205120541515604051908152f35b3461019257602060031936011261019257600435610848612fe8565b80156108a257807f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a700557f902d68080c9f4bf7ed1c926758236afc2b4044487f6f8684352d38087244aee66101205161012051a26101205180f35b7f774d5de1000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b346101925761012051600319360112610192577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903545f19810190811161092a5761091b60209161327e565b90549060031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526011600452602461012051fd5b34610192576101205160031936011261019257604080516101fc916109829082612c67565b600581527f352e302e300000000000000000000000000000000000000000000000000000006020820152604051918291602083526020830190612cfa565b34610192576101205160031936011261019257602060ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015416604051908152f35b34610192576020600319360112610192577f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc005460043590811015610a86576020907f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc006101205152816101205120016101205150546101205160031b1c604051908152f35b7f4e487b710000000000000000000000000000000000000000000000000000000061012051526032600452602461012051fd5b34610192576101205160031936011261019257602073ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416604051908152f35b346101925760406003193601126101925767ffffffffffffffff60043511610192576060600319600435360301126101925760243560a05260a051151560a05103610192575a60e0527f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c6118cd5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d610ba9613054565b610bb76004803501806130a7565b8091501561189f57610bc881613113565b608052610bd3613158565b50604051610be081612c1e565b610120518152610120516020820152906101205161010052610120515b81811061126f5782610c476080515160051b6020608051012091610c3e610c38610c316024600435016004356004016131d6565b3691612ca6565b846143b4565b909391936143ee565b6020815191015190610120515260205273ffffffffffffffffffffffffffffffffffffffff80604061012051201691169080820361123b5782610c8e6004803501806130a7565b7f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a7009291925490806040519260808401907f7b657df4c7ee3ef8592894761aefc80f196e5b97dd27d43a98628b2ce2ef91f0602086015260408501526060808501525260a082019060a08160051b84010194809261012051915b83831061101b5786602087610d25818c03601f198101835282612c67565b8160405191805191829101835e810190610120518252806101205192039060025afa15610fd9576101205151610d656044600435016004356004016131d6565b80600493929311610192577fffffffff000000000000000000000000000000000000000000000000000000008335167fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016808203610fe757505060a05115610efa575b50505061010051610e82575b7f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d610e515a60e051612d1f565b7f6f149831000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b610e8e61010051613ea0565b15610ec5577f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d26020604051610100518152a1610df5565b7fdb788c2b00000000000000000000000000000000000000000000000000000000610120515261010051600452602461012051fd5b73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b1561019257610f756040519485937fab750e75000000000000000000000000000000000000000000000000000000008552606060048601526064850191613741565b927fa46d8bf487ebfdbe1d611a766b6a3fcb2884d2f226b3ce629f2bf25c411bce91602484015260448301528180610120519403915afa8015610fd957610fbe575b8080610de9565b61012051610fcb91612c67565b610120516101925781610fb7565b6040513d61012051823e3d90fd5b7f78a2221c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b90919293967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff608682030183528735907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61833603018212156101925760a0810191839061108981830180613c05565b809560a086525260c0840160c08660051b8601019582610120515b8281106111b45750505050506110c1602083830101838301613c05565b94908482036020860152858252602082019060208760051b84010196819361012051927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b83811061114e5750505050505091016040818101359084015260608082013590840152608090810135920191909152509660209081019493600101920190610d07565b91939599909294969750601f198482030187528935868112156101925760206111a06001936060611192888596018035845285810135868501526040810190613c58565b918160408201520190613d74565b9b0197019101918a9796959391949261110b565b91939760019193959650602061122982608061121b6111fb8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f889903018d528a613c58565b803584528581013586850152604081013560408501526060810190613c58565b918160608201520190613d74565b990195019101918895949391926110a4565b7fe6d44b4c000000000000000000000000000000000000000000000000000000006101205152600452602452604461012051fd5b61127d6004803501806130a7565b9060c052811015610a86578060051b60c0510135927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6160c051360301841215610192576101205160c05185906112d5908201806130a7565b90506112e081613113565b6112e982613113565b91610120515b81811061176b57505060c05161130b90840160208101906130a7565b92905061131783613113565b9161132184613113565b9361012051905b8082106114b7575050926113a07f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc3295936113926113ae94611384608098604051998a9960c051010135895260a060208a015260a08901906131a3565b9087820360408901526131a3565b9085820360608701526131a3565b9083820360808501526131a3565b0390a1806114ad575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08560c051013603011261019257604051906113f582612c1e565b60408560c05101013582526020820160608660c05101013581526040519261141c84612c1e565b5f84525f6020850152611432815183519061399c565b15611475578260019594926114549260206080965193015190519151926139fe565b602083015281529460c05101013561146e82608051613144565b5201610bfd565b604491604051917fb8a0e8a1000000000000000000000000000000000000000000000000000000008352516004830152516024820152fd5b61010052846113b7565b91929394956114d491975060209060c05101018b60c051016130a7565b8791971015610a86578060051b870135967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1813603018812156101925760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f19811461092a57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90055908983013591610120515b8281106116aa5750509060019392917f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548560ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b1461164f575b5098602081830101359161162b61162260408484010161161a61161482878701613170565b8761352e565b848401613170565b83830135613778565b01356116378389613144565b526116428289613144565b52018a9594939291611328565b61169f6116986116a492611662856132e9565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90261012051526020610120512001548094614371565b9280614371565b613382565b8e6115ef565b909260019081851661172c57611721907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901610120515280846020610120512001557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90261012051528360206101205120015490614371565b935b811c9101611591565b611765907f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901610120515283602061012051200154614371565b93611723565b909192936117819060c051018a60c051016130a7565b821015610a8657611798908260051b810190613170565b60408101356117d1815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b1561186e57506117e18135613deb565b1561183c579081602060019301359061181a6118136060830161180d6118078286613170565b8661352e565b83613170565b8235613778565b356118258387613144565b526118308287613144565b520190889392916112ef565b7f39a940c500000000000000000000000000000000000000000000000000000000610120515235600452602461012051fd5b7ff9849ea3000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b7fcf1b093c000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000006101205152600461012051fd5b34611d4c576020600319360112611d4c5767ffffffffffffffff60043511611d4c57606060031960043536030112611d4c577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c6125725760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d61197f613054565b61198d6004803501806130a7565b8091501561254a5761199e81613113565b906119a7613158565b506040516119b481612c1e565b5f81525f6020820152915f915f5b818110611f4d5750508060206119fa925160051b91012092610c3e6119f4610c316024600435016004356004016131d6565b856143b4565b60208151910151905f5260205273ffffffffffffffffffffffffffffffffffffffff8060405f2016911690808203611f1f5750611a3d90506004803501806130a7565b907f3d00115d316bc70efe890550f490ccb6fcbb5768711f93a773ced4553de0a70054916040519181602084019460808501907f7b657df4c7ee3ef8592894761aefc80f196e5b97dd27d43a98628b2ce2ef91f0875260408601526060808601525260a0830160a08360051b85010192825f907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61813603015b838310611d7e578a8a60205f8c8c611af7818e03601f198101835282612c67565b604051918291518091835e8101838152039060025afa15611d41575f51611b286044600435016004356004016131d6565b80600411611d4c577fffffffff000000000000000000000000000000000000000000000000000000008235167fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016808203611d5057505073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803b15611d4c575f92611c2292604051958694859384937fab750e75000000000000000000000000000000000000000000000000000000008552606060048601526064850191613741565b907fa46d8bf487ebfdbe1d611a766b6a3fcb2884d2f226b3ce629f2bf25c411bce916024840152604483015203915afa8015611d4157611d2c575b5080611cbc575b507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce6101205161012051a2610120517f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d6101205180f35b611cc581613ea0565b15611cfb5760207f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d291604051908152a181611c64565b7fdb788c2b000000000000000000000000000000000000000000000000000000006101205152600452602461012051fd5b5f611d3691612c67565b5f6101205282611c5d565b6040513d5f823e3d90fd5b5f80fd5b7f78a2221c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9091929394957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60888203018652863582811215611d4c5783019060a0810191611dc78180613c05565b809460a085525260c0830160c08560051b85010194825f5b828110611ece575050505050611df86020820182613c05565b93908382036020850152848252602082019060208660051b8401019581935f927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301935b838110611e7e57505050506040808501359086015250505060608082013590830152608090810135910152956020908101950193926001019190611ad6565b90919293949598601f19848203018752893586811215611d4c576020611ebf6001936060611192888596018035845285810135868501526040810190613c58565b9b019701959493929101611e3f565b9091929396602080611f1283608061121b6111fb8e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408f60019a03018d528a613c58565b9901950193929101611ddf565b7fe6d44b4c000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b611f5b6004803501806130a7565b821015612407578160051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6181360301821215611d4c57015f95611fa382806130a7565b979050611faf88613113565b97611fb981613113565b905f5b818110612434575050611fd260208501856130a7565b9050611fdd81613113565b61014052611fea81613113565b905f905b808210612120575050987f4ce7c1102060be4f28b229199140fdd5112d4214fe9541bc9b23541e4592cc32916120676120569b6113a060808901359d8e94612048604051978897885260a0602089015260a08801906131a3565b9086820360408801526131a3565b8481036060860152610140516131a3565b0390a180612118575b5060407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08336030112611d4c57604051916120aa83612c1e565b604081013583526060602084019101358152604051926120c984612c1e565b5f84525f60208501526120df815183519061399c565b1561147557916120ff9160019594936020835193015190519151926139fe565b60208301528152956121118286613144565b52016119c2565b955087612070565b90935061213060208701876130a7565b8591951015612407578060051b850135947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301861215611d4c5760ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154167f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900545f1981146123da57600181017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900559087830135915f5b8281106122f65750509060019392917f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900548560ff7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90154161b1461228f575b5096602081830101359161226d61162260408484010161161a61161482878701613170565b013561227c8361014051613144565b526122878286613144565b520190611fee565b61169f6116986122f0926122a2856132e9565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d4310154938490614371565b8f612248565b9092600190818516612383577fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f83018190557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025f527fc16ee73eec0ef4d2332a24ca3268ed9e51b774c2c5af5f5c59c25dbb3b74d43183015461237891614371565b935b811c91016121ea565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015f527fc36adbe7d1fe63428efa33f52fcbd6115d89171225057b1e1f8e7e98617a479f8301546123d49190614371565b9361237a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b61243e86806130a7565b82101561240757612455908260051b810190613170565b604081013561248e815f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054151590565b1561251f5750803561249f81613deb565b156124f4579082916124e26001948f6124dd6124d760208701359660608101906124d26124cc8383613170565b8a61352e565b613170565b85613778565b613144565b526124ed8286613144565b5201611fbc565b7f39a940c5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b34611d4c575f600319360112611d4c576125b2612fe8565b5f73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300547fffffffffffffffffffffffff000000000000000000000000000000000000000081167f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34611d4c575f600319360112611d4c57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34611d4c575f600319360112611d4c576126be612fe8565b6126c6613054565b6126ce613054565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff007fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005416177fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b34611d4c575f600319360112611d4c57602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b34611d4c575f600319360112611d4c57602073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416604051908152f35b34611d4c575f600319360112611d4c5760207f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354604051908152f35b34611d4c575f600319360112611d4c5773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001630036128a95760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b7fe07c8dba000000000000000000000000000000000000000000000000000000005f5260045ffd5b6040600319360112611d4c576128e5612bfb565b60243567ffffffffffffffff8111611d4c57612905903690600401612cdc565b9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016803014908115612b5f575b506128a957612955612fe8565b73ffffffffffffffffffffffffffffffffffffffff8116916040517f52d1902d000000000000000000000000000000000000000000000000000000008152602081600481875afa5f9181612b2b575b506129d557837f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc859203612b005750823b15612ad557807fffffffffffffffffffffffff00000000000000000000000000000000000000007f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416177f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a2805115612aa457612aa291613427565b005b505034612aad57005b7fb398979f000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f4c9c8ce3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7faa1d49a4000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9091506020813d602011612b57575b81612b4760209383612c67565b81010312611d4c575190856129a4565b3d9150612b3a565b905073ffffffffffffffffffffffffffffffffffffffff7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416141583612948565b34611d4c575f600319360112611d4c5760207f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054604051908152f35b34611d4c576020600319360112611d4c57602061091b60043561327e565b6004359073ffffffffffffffffffffffffffffffffffffffff82168203611d4c57565b6040810190811067ffffffffffffffff821117612c3a57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b90601f601f19910116810190811067ffffffffffffffff821117612c3a57604052565b67ffffffffffffffff8111612c3a57601f01601f191660200190565b929192612cb282612c8a565b91612cc06040519384612c67565b829481845281830111611d4c578281602093845f960137010152565b9080601f83011215611d4c57816020612cf793359101612ca6565b90565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b919082039182116123da57565b73ffffffffffffffffffffffffffffffffffffffff168015612ded5773ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930054827fffffffffffffffffffffffff00000000000000000000000000000000000000008216177f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b7f1e4fbdf7000000000000000000000000000000000000000000000000000000005f525f60045260245ffd5b6040517f3cadf4490000000000000000000000000000000000000000000000000000000081527fffffffff000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000016600482015260208160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611d41575f91612f98575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f5c975abb000000000000000000000000000000000000000000000000000000008352165afa908115611d41575f91612f5d575b508015612f345790565b5060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541690565b90506020813d602011612f90575b81612f7860209383612c67565b81010312611d4c57518015158103611d4c575f612f2a565b3d9150612f6b565b90506020813d602011612fe0575b81612fb360209383612c67565b81010312611d4c575173ffffffffffffffffffffffffffffffffffffffff81168103611d4c576020612ed4565b3d9150612fa6565b73ffffffffffffffffffffffffffffffffffffffff7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c1993005416330361302857565b7f118cdaa7000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541661307f57565b7fd93c0665000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215611d4c570180359067ffffffffffffffff8211611d4c57602001918160051b36038313611d4c57565b67ffffffffffffffff8111612c3a5760051b60200190565b9061311d826130fb565b61312a6040519182612c67565b828152601f1961313a82946130fb565b0190602036910137565b80518210156124075760209160051b010190565b6040519061316582612c1e565b5f6020838281520152565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215611d4c570190565b90602080835192838152019201905f5b8181106131c05750505090565b82518452602093840193909201916001016131b3565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215611d4c570180359067ffffffffffffffff8211611d4c57602001918136038313611d4c57565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c161561325657565b7fd7e6bcf8000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354811015612407577f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9035f5260205f2001905f90565b8054821015612407575f5260205f2001905f90565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9015468010000000000000000811015612c3a5780600161336c92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc901557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9016132d4565b5f19829392549160031b92831b921b1916179055565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9025468010000000000000000811015612c3a5780600161336c92017f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc902557f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9026132d4565b805468010000000000000000811015612c3a5761336c916001820181556132d4565b905f8091602081519101845af480806134db575b1561345b5750506040513d81523d5f602083013e60203d82010160405290565b156134a25773ffffffffffffffffffffffffffffffffffffffff907f9996b315000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b3d156134b3576040513d5f823e3d90fd5b7fd6bda275000000000000000000000000000000000000000000000000000000005f5260045ffd5b503d15158061343b5750813b151561343b565b91908110156124075760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215611d4c570190565b906040810161353d81836130a7565b9290505f5b838110613550575050505050565b6135716135678261356186866130a7565b906134ee565b60208101906131d6565b8101606082820312611d4c5781359173ffffffffffffffffffffffffffffffffffffffff8316809303611d4c57602081013567ffffffffffffffff8111611d4c57826135be918301612cdc565b91604082013567ffffffffffffffff8111611d4c576135dd9201612cdc565b90604051917f33a89203000000000000000000000000000000000000000000000000000000008352886004840152604060248401525f83806136226044820186612cfa565b038183885af1928315611d41575f936136c5575b5082516020840120815160208301200361368a575060019392917fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf1916136816040519283928361409b565b0390a201613542565b90506136c16040519283927fc504fada0000000000000000000000000000000000000000000000000000000084526004840161409b565b0390fd5b9092503d805f833e6136d78183612c67565b810190602081830312611d4c5780519067ffffffffffffffff8211611d4c570181601f82011215611d4c5780519061370e82612c8a565b9261371c6040519485612c67565b82845260208383010111611d4c57815f9260208093018386015e83010152915f613636565b601f8260209493601f1993818652868601375f8582860101520116010190565b604090612cf7949281528160208201520191613741565b9061378381806130a7565b5f5b81811061392d5750505061379c60208201826130a7565b5f5b8181106138be575050506137b560408201826130a7565b5f5b81811061384f575050508060606137cf9201906130a7565b90915f5b8281106137e05750505050565b6137eb8184866134ee565b35906002821015611d4c576001809214613806575b016137d3565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c61383661356784888a6134ee565b906138476040519283928784613761565b0390a2613800565b61385a8183856134ee565b35906002821015611d4c576001809214613875575b016137b7565b857f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596138a56135678487896134ee565b906138b66040519283928784613761565b0390a261386f565b6138c98183856134ee565b35906002821015611d4c5760018092146138e4575b0161379e565b857f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f36139146135678487896134ee565b906139256040519283928784613761565b0390a26138de565b6139388183856134ee565b35906002821015611d4c576001809214613953575b01613785565b857f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66139836135678487896134ee565b906139946040519283928784613761565b0390a261394d565b801580156139ee575b80156139e6575b80156139d6575b6139d0576401000003d01960078180938181800909089180091490565b50505f90565b506401000003d0198210156139b3565b5081156139ac565b506401000003d0198110156139a5565b92939290915f90808303613beb5750506401000003d0195f948308613a2757505090505f905f90565b5f613a39926401000003d019926142fa565b915b8215158381613bda575b5080613bd2575b15613b74579084939293906001936401000003d0199187965b8015613b2157808404968098613af4576401000003d0199088096401000003d019036401000003d01981116123da576401000003d019908a960897938197828102928184041490151715613ac75790613abd91612d1f565b9592939695613a65565b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b60248a7f4e487b710000000000000000000000000000000000000000000000000000000081526012600452fd5b5094509450509380613b475750906401000003d019809281808780098092099509900990565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526012600452fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f496e76616c6964206e756d6265720000000000000000000000000000000000006044820152fd5b506001613a4c565b6401000003d019915014155f613a45565b613bff936401000003d019939692966140c0565b91613a3b565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe182360301811215611d4c57016020813591019167ffffffffffffffff8211611d4c578160051b36038313611d4c57565b90357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8182360301811215611d4c570190565b906020838281520160208260051b85010193835f915b848310613cb05750505050505090565b909192939495601f1982820301855286357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc184360301811215611d4c57830180356002811015611d4c57825260208101357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe182360301811215611d4c57016020813591019067ffffffffffffffff8111611d4c578036038213611d4c57613d666020928392604086818660019901520191613741565b980196950193019190613ca0565b612cf791613ddd613dd2613db7613d9c613d8e8680613c05565b608087526080870191613c8a565b613da96020870187613c05565b908683036020880152613c8a565b613dc46040860186613c05565b908583036040870152613c8a565b926060810190613c05565b916060818503910152613c8a565b805f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2054155f14613e9b57613e48817f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc00613405565b7f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0054905f527f5d2cc44bcf41d6324882f1c0d709f1b9ae20caeebcbf2358127a3d0a8c22cc0160205260405f2055600190565b505f90565b805f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2054155f14613e9b57613efd817f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613405565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90354905f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc90460205260405f2055600190565b7fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e54614097576140077fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b067f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903613405565b7f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc903547fcc1d2f838445db7aec431df9ee8a871f40e7aa5e064fc056633ef8c60fab7b065f527f762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc9046020527f5deb946ebbf617e36331f9b8f582f11645232839ec35aa4ea6e3124c4262e96e55600190565b5f90565b90916140b2612cf793604084526040840190612cfa565b916020818403910152612cfa565b94929093918515806142f2575b6142e7578015806142df575b6142d7576040516080916140ed8383612c67565b8236833784156142aa5784600180098083529385856001099860208401998a52604084019286845287845160010993606086019485526040519a878c01958c871067ffffffffffffffff881117612c3a578a80979581969482956040525190098d525190099460208b019586525190099860408901998a5251900960608701908152865188511480159061429e575b1561424057849283808093816040519c856141988f9788612c67565b368737518c516141a89083612d1f565b900884525185516141b99083612d1f565b90089860208301998a5281808b8180808089518a5190099360408a019485528185518b5190096060909a01998a5251800988516141f69083612d1f565b9008818087518551900960020961420d9083612d1f565b90089c519351905190096142218c83612d1f565b900890099251905190096142359083612d1f565b900894510991929190565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f557365206a6163446f75626c652066756e6374696f6e20696e737465616400006044820152fd5b5081518151141561417c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b505050600190565b5081156140d9565b945092506001919050565b5084156140cd565b939192909281156142aa5760018280614367818080809781806143579e818f81818192099a8b96818f8180828193099a8809600409988009900992800960030908918161434a8183800882612d1f565b81858009089e8f83612d1f565b9008900993800960080983612d1f565b9008940960020990565b5f9060209260405190848201928352604082015260408152614394606082612c67565b604051918291518091835e8101838152039060025afa15611d41575f5190565b81519190604183036143e4576143dd9250602082015190606060408401519301515f1a906144c6565b9192909190565b50505f9160029190565b60048110156144995780614400575050565b60018103614430577ff645eedf000000000000000000000000000000000000000000000000000000005f5260045ffd5b6002810361446457507ffce698f7000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b60031461446e5750565b7fd78bce0c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841161454a579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15611d41575f5173ffffffffffffffffffffffffffffffffffffffff81161561454057905f905f90565b505f906001905f90565b5050505f916003919056fea164736f6c6343000824000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R`\x046\x10\x15a\0\x12W_\x80\xFD[_a\x01 R_5`\xE0\x1C\x80c1\xEEbB\x14a+\xDDW\x80c@\xF3MB\x14a+\xA1W\x80cO\x1E\xF2\x86\x14a(\xD1W\x80cR\xD1\x90-\x14a(2W\x80cY\xBA\x92X\x14a'\xF6W\x80c\\`\xDA\x1B\x14a'\xA4W\x80c\\\x97Z\xBB\x14a'cW\x80cc\xA5\x99\xA4\x14a&\xA6W\x80ck\xF0\xA0K\x14a&VW\x80cqP\x18\xA6\x14a%\x9AW\x80cs\xAB\x99\x16\x14a\x18\xFBW\x80c\x87\t>\xBA\x14a\x0B\x0EW\x80c\x8D\xA5\xCB[\x14a\n\xB9W\x80c\x9A\xD9\x1DL\x14a\n\x02W\x80c\xA0`V\xF7\x14a\t\xC0W\x80c\xAD<\xB1\xCC\x14a\t]W\x80c\xBD\xEBD-\x14a\x08\xD0W\x80c\xC0%0#\x14a\x08,W\x80c\xC1\xB0\xBE\xD7\x14a\x07\xD6W\x80c\xC4IV\xD1\x14a\x07\x97W\x80c\xC4\xD6m\xE8\x14a\x03DW\x80c\xC8y\xDB\xE4\x14a\x02\xFBW\x80c\xE3Z]/\x14a\x02\x9CW\x80c\xF2\xFD\xE3\x8B\x14a\x02lW\x80c\xFD\xDDH7\x14a\x02EW\x80c\xFE\x18\xAB\x91\x14a\x02\0W\x80c\xFF\xA1\xADt\x14a\x01\x99Wc\xFF\xC3?r\x14a\x01SW_\x80\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` \x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0T`@Q\x90\x81R\xF3[a\x01 Q\x80\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W`@\x80Qa\x01\xFC\x91a\x01\xBE\x90\x82a,gV[`\r\x81R\x7F2.0.0-alpha.6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a,\xFAV[\x03\x90\xF3[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` `\x01`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B`@Q\x90\x81R\xF3[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` a\x02ba.\x19V[`@Q\x90\x15\x15\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92Wa\x02\x95a\x02\x88a+\xFBV[a\x02\x90a/\xE8V[a-,V[a\x01 Q\x80\xF3[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` `@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92W` a\x02b`\x045_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92Wa\x03]a+\xFBV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\xFF\x81`@\x1C\x16\x15\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x15\x90\x81a\x07\x8FW[`\x01\x14\x90\x81a\x07\x85W[\x15\x90\x81a\x07|W[Pa\x07NW\x81\x83`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x04\x16\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x06\xF9W[Pa\x04\x0Ea2'V[a\x02\x90a2'V[a\x04\x1Ea2'V[a\x04&a2'V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x06\xC6W\x80`\x01a\x04\xA9\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a2\xD4V[\x81T\x90`\x03\x1B\x90_\x19\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x83\x1B\x92\x1B\x19\x16\x17\x90Ua\x01 Q\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0Ua\x05\na?PV[P\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Q\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x81R\xA1a\x05]a2'V[\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0U\x7F\xE3\xB0\xC4B\x98\xFC\x1C\x14\x9A\xFB\xF4\xC8\x99o\xB9$'\xAEA\xE4d\x9B\x93L\xA4\x95\x99\x1BxR\xB8U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x05\xF3a.\x19V[a\x06\x98Wa\x06\x02Wa\x01 Q\x80\xF3[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1a\x02\x95V[\x7F\x0B\x1D8\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`A`\x04R`$a\x01 Q\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x83a\x04\x05V[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x90P\x15\x84a\x03\xAEV[0;\x15\x91Pa\x03\xA6V[\x84\x91Pa\x03\x9CV[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T`@Q\x90\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92Wa\x01 QP`\x045a\x01 QR\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R` `@a\x01 Q T\x15\x15`@Q\x90\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92W`\x045a\x08Ha/\xE8V[\x80\x15a\x08\xA2W\x80\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0U\x7F\x90-h\x08\x0C\x9FK\xF7\xED\x1C\x92gX#j\xFC+@DH\x7Fo\x86\x845-8\x08rD\xAE\xE6a\x01 Qa\x01 Q\xA2a\x01 Q\x80\xF3[\x7FwM]\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T_\x19\x81\x01\x90\x81\x11a\t*Wa\t\x1B` \x91a2~V[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x11`\x04R`$a\x01 Q\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W`@\x80Qa\x01\xFC\x91a\t\x82\x90\x82a,gV[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a,\xFAV[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` `\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16`@Q\x90\x81R\xF3[4a\x01\x92W` `\x03\x196\x01\x12a\x01\x92W\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`\x045\x90\x81\x10\x15a\n\x86W` \x90\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a\x01 QR\x81a\x01 Q \x01a\x01 QPTa\x01 Q`\x03\x1B\x1C`@Q\x90\x81R\xF3[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`2`\x04R`$a\x01 Q\xFD[4a\x01\x92Wa\x01 Q`\x03\x196\x01\x12a\x01\x92W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x01\x92W`@`\x03\x196\x01\x12a\x01\x92Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x01\x92W```\x03\x19`\x0456\x03\x01\x12a\x01\x92W`$5`\xA0R`\xA0Q\x15\x15`\xA0Q\x03a\x01\x92WZ`\xE0R\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x18\xCDW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x0B\xA9a0TV[a\x0B\xB7`\x04\x805\x01\x80a0\xA7V[\x80\x91P\x15a\x18\x9FWa\x0B\xC8\x81a1\x13V[`\x80Ra\x0B\xD3a1XV[P`@Qa\x0B\xE0\x81a,\x1EV[a\x01 Q\x81Ra\x01 Q` \x82\x01R\x90a\x01 Qa\x01\0Ra\x01 Q[\x81\x81\x10a\x12oW\x82a\x0CG`\x80QQ`\x05\x1B` `\x80Q\x01 \x91a\x0C>a\x0C8a\x0C1`$`\x045\x01`\x045`\x04\x01a1\xD6V[6\x91a,\xA6V[\x84aC\xB4V[\x90\x93\x91\x93aC\xEEV[` \x81Q\x91\x01Q\x90a\x01 QR` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@a\x01 Q \x16\x91\x16\x90\x80\x82\x03a\x12;W\x82a\x0C\x8E`\x04\x805\x01\x80a0\xA7V[\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0\x92\x91\x92T\x90\x80`@Q\x92`\x80\x84\x01\x90\x7F{e}\xF4\xC7\xEE>\xF8Y(\x94v\x1A\xEF\xC8\x0F\x19n[\x97\xDD'\xD4:\x98b\x8B,\xE2\xEF\x91\xF0` \x86\x01R`@\x85\x01R``\x80\x85\x01RR`\xA0\x82\x01\x90`\xA0\x81`\x05\x1B\x84\x01\x01\x94\x80\x92a\x01 Q\x91[\x83\x83\x10a\x10\x1BW\x86` \x87a\r%\x81\x8C\x03`\x1F\x19\x81\x01\x83R\x82a,gV[\x81`@Q\x91\x80Q\x91\x82\x91\x01\x83^\x81\x01\x90a\x01 Q\x82R\x80a\x01 Q\x92\x03\x90`\x02Z\xFA\x15a\x0F\xD9Wa\x01 QQa\re`D`\x045\x01`\x045`\x04\x01a1\xD6V[\x80`\x04\x93\x92\x93\x11a\x01\x92W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x835\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x0F\xE7WPP`\xA0Q\x15a\x0E\xFAW[PPPa\x01\0Qa\x0E\x82W[\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x0EQZ`\xE0Qa-\x1FV[\x7Fo\x14\x981\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[a\x0E\x8Ea\x01\0Qa>\xA0V[\x15a\x0E\xC5W\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` `@Qa\x01\0Q\x81R\xA1a\r\xF5V[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QRa\x01\0Q`\x04R`$a\x01 Q\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x01\x92Wa\x0Fu`@Q\x94\x85\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a7AV[\x92\x7F\xA4m\x8B\xF4\x87\xEB\xFD\xBE\x1Da\x1Avkj?\xCB(\x84\xD2\xF2&\xB3\xCEb\x9F+\xF2\\A\x1B\xCE\x91`$\x84\x01R`D\x83\x01R\x81\x80a\x01 Q\x94\x03\x91Z\xFA\x80\x15a\x0F\xD9Wa\x0F\xBEW[\x80\x80a\r\xE9V[a\x01 Qa\x0F\xCB\x91a,gV[a\x01 Qa\x01\x92W\x81a\x0F\xB7V[`@Q=a\x01 Q\x82>=\x90\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[\x90\x91\x92\x93\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x86\x82\x03\x01\x83R\x875\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x836\x03\x01\x82\x12\x15a\x01\x92W`\xA0\x81\x01\x91\x83\x90a\x10\x89\x81\x83\x01\x80a<\x05V[\x80\x95`\xA0\x86RR`\xC0\x84\x01`\xC0\x86`\x05\x1B\x86\x01\x01\x95\x82a\x01 Q[\x82\x81\x10a\x11\xB4WPPPPPa\x10\xC1` \x83\x83\x01\x01\x83\x83\x01a<\x05V[\x94\x90\x84\x82\x03` \x86\x01R\x85\x82R` \x82\x01\x90` \x87`\x05\x1B\x84\x01\x01\x96\x81\x93a\x01 Q\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x11NWPPPPPP\x91\x01`@\x81\x81\x015\x90\x84\x01R``\x80\x82\x015\x90\x84\x01R`\x80\x90\x81\x015\x92\x01\x91\x90\x91RP\x96` \x90\x81\x01\x94\x93`\x01\x01\x92\x01\x90a\r\x07V[\x91\x93\x95\x99\x90\x92\x94\x96\x97P`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x01\x92W` a\x11\xA0`\x01\x93``a\x11\x92\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90a<XV[\x91\x81`@\x82\x01R\x01\x90a=tV[\x9B\x01\x97\x01\x91\x01\x91\x8A\x97\x96\x95\x93\x91\x94\x92a\x11\x0BV[\x91\x93\x97`\x01\x91\x93\x95\x96P` a\x12)\x82`\x80a\x12\x1Ba\x11\xFB\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F\x88\x99\x03\x01\x8DR\x8Aa<XV[\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x015`@\x85\x01R``\x81\x01\x90a<XV[\x91\x81``\x82\x01R\x01\x90a=tV[\x99\x01\x95\x01\x91\x01\x91\x88\x95\x94\x93\x91\x92a\x10\xA4V[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$R`Da\x01 Q\xFD[a\x12}`\x04\x805\x01\x80a0\xA7V[\x90`\xC0R\x81\x10\x15a\n\x86W\x80`\x05\x1B`\xC0Q\x015\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa`\xC0Q6\x03\x01\x84\x12\x15a\x01\x92Wa\x01 Q`\xC0Q\x85\x90a\x12\xD5\x90\x82\x01\x80a0\xA7V[\x90Pa\x12\xE0\x81a1\x13V[a\x12\xE9\x82a1\x13V[\x91a\x01 Q[\x81\x81\x10a\x17kWPP`\xC0Qa\x13\x0B\x90\x84\x01` \x81\x01\x90a0\xA7V[\x92\x90Pa\x13\x17\x83a1\x13V[\x91a\x13!\x84a1\x13V[\x93a\x01 Q\x90[\x80\x82\x10a\x14\xB7WPP\x92a\x13\xA0\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x95\x93a\x13\x92a\x13\xAE\x94a\x13\x84`\x80\x98`@Q\x99\x8A\x99`\xC0Q\x01\x015\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a1\xA3V[\x90\x87\x82\x03`@\x89\x01Ra1\xA3V[\x90\x85\x82\x03``\x87\x01Ra1\xA3V[\x90\x83\x82\x03`\x80\x85\x01Ra1\xA3V[\x03\x90\xA1\x80a\x14\xADW[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x85`\xC0Q\x016\x03\x01\x12a\x01\x92W`@Q\x90a\x13\xF5\x82a,\x1EV[`@\x85`\xC0Q\x01\x015\x82R` \x82\x01``\x86`\xC0Q\x01\x015\x81R`@Q\x92a\x14\x1C\x84a,\x1EV[_\x84R_` \x85\x01Ra\x142\x81Q\x83Q\x90a9\x9CV[\x15a\x14uW\x82`\x01\x95\x94\x92a\x14T\x92` `\x80\x96Q\x93\x01Q\x90Q\x91Q\x92a9\xFEV[` \x83\x01R\x81R\x94`\xC0Q\x01\x015a\x14n\x82`\x80Qa1DV[R\x01a\x0B\xFDV[`D\x91`@Q\x91\x7F\xB8\xA0\xE8\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83RQ`\x04\x83\x01RQ`$\x82\x01R\xFD[a\x01\0R\x84a\x13\xB7V[\x91\x92\x93\x94\x95a\x14\xD4\x91\x97P` \x90`\xC0Q\x01\x01\x8B`\xC0Q\x01a0\xA7V[\x87\x91\x97\x10\x15a\n\x86W\x80`\x05\x1B\x87\x015\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x88\x12\x15a\x01\x92W`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a\t*W`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x89\x83\x015\x91a\x01 Q[\x82\x81\x10a\x16\xAAWPP\x90`\x01\x93\x92\x91\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x85`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\x16OW[P\x98` \x81\x83\x01\x015\x91a\x16+a\x16\"`@\x84\x84\x01\x01a\x16\x1Aa\x16\x14\x82\x87\x87\x01a1pV[\x87a5.V[\x84\x84\x01a1pV[\x83\x83\x015a7xV[\x015a\x167\x83\x89a1DV[Ra\x16B\x82\x89a1DV[R\x01\x8A\x95\x94\x93\x92\x91a\x13(V[a\x16\x9Fa\x16\x98a\x16\xA4\x92a\x16b\x85a2\xE9V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR` a\x01 Q \x01T\x80\x94aCqV[\x92\x80aCqV[a3\x82V[\x8Ea\x15\xEFV[\x90\x92`\x01\x90\x81\x85\x16a\x17,Wa\x17!\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x80\x84` a\x01 Q \x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a\x01 QR\x83` a\x01 Q \x01T\x90aCqV[\x93[\x81\x1C\x91\x01a\x15\x91V[a\x17e\x90\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a\x01 QR\x83` a\x01 Q \x01TaCqV[\x93a\x17#V[\x90\x91\x92\x93a\x17\x81\x90`\xC0Q\x01\x8A`\xC0Q\x01a0\xA7V[\x82\x10\x15a\n\x86Wa\x17\x98\x90\x82`\x05\x1B\x81\x01\x90a1pV[`@\x81\x015a\x17\xD1\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a\x18nWPa\x17\xE1\x815a=\xEBV[\x15a\x18<W\x90\x81` `\x01\x93\x015\x90a\x18\x1Aa\x18\x13``\x83\x01a\x18\ra\x18\x07\x82\x86a1pV[\x86a5.V[\x83a1pV[\x825a7xV[5a\x18%\x83\x87a1DV[Ra\x180\x82\x87a1DV[R\x01\x90\x88\x93\x92\x91a\x12\xEFV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR5`\x04R`$a\x01 Q\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04a\x01 Q\xFD[4a\x1DLW` `\x03\x196\x01\x12a\x1DLWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x11a\x1DLW```\x03\x19`\x0456\x03\x01\x12a\x1DLW\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a%rW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x19\x7Fa0TV[a\x19\x8D`\x04\x805\x01\x80a0\xA7V[\x80\x91P\x15a%JWa\x19\x9E\x81a1\x13V[\x90a\x19\xA7a1XV[P`@Qa\x19\xB4\x81a,\x1EV[_\x81R_` \x82\x01R\x91_\x91_[\x81\x81\x10a\x1FMWPP\x80` a\x19\xFA\x92Q`\x05\x1B\x91\x01 \x92a\x0C>a\x19\xF4a\x0C1`$`\x045\x01`\x045`\x04\x01a1\xD6V[\x85aC\xB4V[` \x81Q\x91\x01Q\x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`@_ \x16\x91\x16\x90\x80\x82\x03a\x1F\x1FWPa\x1A=\x90P`\x04\x805\x01\x80a0\xA7V[\x90\x7F=\0\x11]1k\xC7\x0E\xFE\x89\x05P\xF4\x90\xCC\xB6\xFC\xBBWhq\x1F\x93\xA7s\xCE\xD4U=\xE0\xA7\0T\x91`@Q\x91\x81` \x84\x01\x94`\x80\x85\x01\x90\x7F{e}\xF4\xC7\xEE>\xF8Y(\x94v\x1A\xEF\xC8\x0F\x19n[\x97\xDD'\xD4:\x98b\x8B,\xE2\xEF\x91\xF0\x87R`@\x86\x01R``\x80\x86\x01RR`\xA0\x83\x01`\xA0\x83`\x05\x1B\x85\x01\x01\x92\x82_\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01[\x83\x83\x10a\x1D~W\x8A\x8A` _\x8C\x8Ca\x1A\xF7\x81\x8E\x03`\x1F\x19\x81\x01\x83R\x82a,gV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x1DAW_Qa\x1B(`D`\x045\x01`\x045`\x04\x01a1\xD6V[\x80`\x04\x11a\x1DLW\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x825\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80\x82\x03a\x1DPWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x80;\x15a\x1DLW_\x92a\x1C\"\x92`@Q\x95\x86\x94\x85\x93\x84\x93\x7F\xABu\x0Eu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R```\x04\x86\x01R`d\x85\x01\x91a7AV[\x90\x7F\xA4m\x8B\xF4\x87\xEB\xFD\xBE\x1Da\x1Avkj?\xCB(\x84\xD2\xF2&\xB3\xCEb\x9F+\xF2\\A\x1B\xCE\x91`$\x84\x01R`D\x83\x01R\x03\x91Z\xFA\x80\x15a\x1DAWa\x1D,W[P\x80a\x1C\xBCW[P\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCEa\x01 Qa\x01 Q\xA2a\x01 Q\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x01 Q\x80\xF3[a\x1C\xC5\x81a>\xA0V[\x15a\x1C\xFBW` \x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x91`@Q\x90\x81R\xA1\x81a\x1CdV[\x7F\xDBx\x8C+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01 QR`\x04R`$a\x01 Q\xFD[_a\x1D6\x91a,gV[_a\x01 R\x82a\x1C]V[`@Q=_\x82>=\x90\xFD[_\x80\xFD[\x7Fx\xA2\"\x1C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x91\x92\x93\x94\x95\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x88\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x1DLW\x83\x01\x90`\xA0\x81\x01\x91a\x1D\xC7\x81\x80a<\x05V[\x80\x94`\xA0\x85RR`\xC0\x83\x01`\xC0\x85`\x05\x1B\x85\x01\x01\x94\x82_[\x82\x81\x10a\x1E\xCEWPPPPPa\x1D\xF8` \x82\x01\x82a<\x05V[\x93\x90\x83\x82\x03` \x85\x01R\x84\x82R` \x82\x01\x90` \x86`\x05\x1B\x84\x01\x01\x95\x81\x93_\x92\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x93[\x83\x81\x10a\x1E~WPPPP`@\x80\x85\x015\x90\x86\x01RPPP``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01R\x95` \x90\x81\x01\x95\x01\x93\x92`\x01\x01\x91\x90a\x1A\xD6V[\x90\x91\x92\x93\x94\x95\x98`\x1F\x19\x84\x82\x03\x01\x87R\x895\x86\x81\x12\x15a\x1DLW` a\x1E\xBF`\x01\x93``a\x11\x92\x88\x85\x96\x01\x805\x84R\x85\x81\x015\x86\x85\x01R`@\x81\x01\x90a<XV[\x9B\x01\x97\x01\x95\x94\x93\x92\x91\x01a\x1E?V[\x90\x91\x92\x93\x96` \x80a\x1F\x12\x83`\x80a\x12\x1Ba\x11\xFB\x8E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x8F`\x01\x9A\x03\x01\x8DR\x8Aa<XV[\x99\x01\x95\x01\x93\x92\x91\x01a\x1D\xDFV[\x7F\xE6\xD4KL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[a\x1F[`\x04\x805\x01\x80a0\xA7V[\x82\x10\x15a$\x07W\x81`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x816\x03\x01\x82\x12\x15a\x1DLW\x01_\x95a\x1F\xA3\x82\x80a0\xA7V[\x97\x90Pa\x1F\xAF\x88a1\x13V[\x97a\x1F\xB9\x81a1\x13V[\x90_[\x81\x81\x10a$4WPPa\x1F\xD2` \x85\x01\x85a0\xA7V[\x90Pa\x1F\xDD\x81a1\x13V[a\x01@Ra\x1F\xEA\x81a1\x13V[\x90_\x90[\x80\x82\x10a! WPP\x98\x7FL\xE7\xC1\x10 `\xBEO(\xB2)\x19\x91@\xFD\xD5\x11-B\x14\xFE\x95A\xBC\x9B#T\x1EE\x92\xCC2\x91a ga V\x9Ba\x13\xA0`\x80\x89\x015\x9D\x8E\x94a H`@Q\x97\x88\x97\x88R`\xA0` \x89\x01R`\xA0\x88\x01\x90a1\xA3V[\x90\x86\x82\x03`@\x88\x01Ra1\xA3V[\x84\x81\x03``\x86\x01Ra\x01@Qa1\xA3V[\x03\x90\xA1\x80a!\x18W[P`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x836\x03\x01\x12a\x1DLW`@Q\x91a \xAA\x83a,\x1EV[`@\x81\x015\x83R``` \x84\x01\x91\x015\x81R`@Q\x92a \xC9\x84a,\x1EV[_\x84R_` \x85\x01Ra \xDF\x81Q\x83Q\x90a9\x9CV[\x15a\x14uW\x91a \xFF\x91`\x01\x95\x94\x93` \x83Q\x93\x01Q\x90Q\x91Q\x92a9\xFEV[` \x83\x01R\x81R\x95a!\x11\x82\x86a1DV[R\x01a\x19\xC2V[\x95P\x87a pV[\x90\x93Pa!0` \x87\x01\x87a0\xA7V[\x85\x91\x95\x10\x15a$\x07W\x80`\x05\x1B\x85\x015\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x86\x12\x15a\x1DLW`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T_\x19\x81\x14a#\xDAW`\x01\x81\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0U\x90\x87\x83\x015\x91_[\x82\x81\x10a\"\xF6WPP\x90`\x01\x93\x92\x91\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\0T\x85`\xFF\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01T\x16\x1B\x14a\"\x8FW[P\x96` \x81\x83\x01\x015\x91a\"ma\x16\"`@\x84\x84\x01\x01a\x16\x1Aa\x16\x14\x82\x87\x87\x01a1pV[\x015a\"|\x83a\x01@Qa1DV[Ra\"\x87\x82\x86a1DV[R\x01\x90a\x1F\xEEV[a\x16\x9Fa\x16\x98a\"\xF0\x92a\"\xA2\x85a2\xE9V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x01T\x93\x84\x90aCqV[\x8Fa\"HV[\x90\x92`\x01\x90\x81\x85\x16a#\x83W\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01\x81\x90U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02_R\x7F\xC1n\xE7>\xEC\x0E\xF4\xD23*$\xCA2h\xED\x9EQ\xB7t\xC2\xC5\xAF_\\Y\xC2]\xBB;t\xD41\x83\x01Ta#x\x91aCqV[\x93[\x81\x1C\x91\x01a!\xEAV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01_R\x7F\xC3j\xDB\xE7\xD1\xFEcB\x8E\xFA3\xF5/\xCB\xD6\x11]\x89\x17\x12%\x05{\x1E\x1F\x8E~\x98azG\x9F\x83\x01Ta#\xD4\x91\x90aCqV[\x93a#zV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a$>\x86\x80a0\xA7V[\x82\x10\x15a$\x07Wa$U\x90\x82`\x05\x1B\x81\x01\x90a1pV[`@\x81\x015a$\x8E\x81_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15\x15\x90V[\x15a%\x1FWP\x805a$\x9F\x81a=\xEBV[\x15a$\xF4W\x90\x82\x91a$\xE2`\x01\x94\x8Fa$\xDDa$\xD7` \x87\x015\x96``\x81\x01\x90a$\xD2a$\xCC\x83\x83a1pV[\x8Aa5.V[a1pV[\x85a7xV[a1DV[Ra$\xED\x82\x86a1DV[R\x01a\x1F\xBCV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x1DLW_`\x03\x196\x01\x12a\x1DLWa%\xB2a/\xE8V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLWa&\xBEa/\xE8V[a&\xC6a0TV[a&\xCEa0TV[`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x17\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16`@Q\x90\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` \x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T`@Q\x90\x81R\xF3[4a\x1DLW_`\x03\x196\x01\x12a\x1DLWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a(\xA9W` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@`\x03\x196\x01\x12a\x1DLWa(\xE5a+\xFBV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLWa)\x05\x906\x90`\x04\x01a,\xDCV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a+_W[Pa(\xA9Wa)Ua/\xE8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a++W[Pa)\xD5W\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a+\0WP\x82;\x15a*\xD5W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x80Q\x15a*\xA4Wa*\xA2\x91a4'V[\0[PP4a*\xADW\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a+WW[\x81a+G` \x93\x83a,gV[\x81\x01\x03\x12a\x1DLWQ\x90\x85a)\xA4V[=\x91Pa+:V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a)HV[4a\x1DLW_`\x03\x196\x01\x12a\x1DLW` \x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T`@Q\x90\x81R\xF3[4a\x1DLW` `\x03\x196\x01\x12a\x1DLW` a\t\x1B`\x045a2~V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1DLWV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a,:W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a,:W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a,:W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a,\xB2\x82a,\x8AV[\x91a,\xC0`@Q\x93\x84a,gV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x1DLW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x1DLW\x81` a,\xF7\x935\x91\x01a,\xA6V[\x90V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x03\x91\x82\x11a#\xDAWV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a-\xEDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[`@Q\x7F<\xAD\xF4I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x04\x82\x01R` \x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x1DAW_\x91a/\x98W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\\\x97Z\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x90\x81\x15a\x1DAW_\x91a/]W[P\x80\x15a/4W\x90V[P`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16\x90V[\x90P` \x81=` \x11a/\x90W[\x81a/x` \x93\x83a,gV[\x81\x01\x03\x12a\x1DLWQ\x80\x15\x15\x81\x03a\x1DLW_a/*V[=\x91Pa/kV[\x90P` \x81=` \x11a/\xE0W[\x81a/\xB3` \x93\x83a,gV[\x81\x01\x03\x12a\x1DLWQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x1DLW` a.\xD4V[=\x91Pa/\xA6V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a0(WV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16a0\x7FWV[\x7F\xD9<\x06e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x1DLWV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a,:W`\x05\x1B` \x01\x90V[\x90a1\x1D\x82a0\xFBV[a1*`@Q\x91\x82a,gV[\x82\x81R`\x1F\x19a1:\x82\x94a0\xFBV[\x01\x90` 6\x91\x017V[\x80Q\x82\x10\x15a$\x07W` \x91`\x05\x1B\x01\x01\x90V[`@Q\x90a1e\x82a,\x1EV[_` \x83\x82\x81R\x01RV[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a1\xC0WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\xB3V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW` \x01\x91\x816\x03\x83\x13a\x1DLWV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a2VWV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x81\x10\x15a$\x07W\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03_R` _ \x01\x90_\x90V[\x80T\x82\x10\x15a$\x07W_R` _ \x01\x90_\x90V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a,:W\x80`\x01a3l\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x01a2\xD4V[_\x19\x82\x93\x92T\x91`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90UV[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a,:W\x80`\x01a3l\x92\x01\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02U\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x02a2\xD4V[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a,:Wa3l\x91`\x01\x82\x01\x81Ua2\xD4V[\x90_\x80\x91` \x81Q\x91\x01\x84Z\xF4\x80\x80a4\xDBW[\x15a4[WPP`@Q=\x81R=_` \x83\x01>` =\x82\x01\x01`@R\x90V[\x15a4\xA2Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[=\x15a4\xB3W`@Q=_\x82>=\x90\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[P=\x15\x15\x80a4;WP\x81;\x15\x15a4;V[\x91\x90\x81\x10\x15a$\x07W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x1DLW\x01\x90V[\x90`@\x81\x01a5=\x81\x83a0\xA7V[\x92\x90P_[\x83\x81\x10a5PWPPPPPV[a5qa5g\x82a5a\x86\x86a0\xA7V[\x90a4\xEEV[` \x81\x01\x90a1\xD6V[\x81\x01``\x82\x82\x03\x12a\x1DLW\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x1DLW` \x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLW\x82a5\xBE\x91\x83\x01a,\xDCV[\x91`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLWa5\xDD\x92\x01a,\xDCV[\x90`@Q\x91\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x88`\x04\x84\x01R`@`$\x84\x01R_\x83\x80a6\"`D\x82\x01\x86a,\xFAV[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x1DAW_\x93a6\xC5W[P\x82Q` \x84\x01 \x81Q` \x83\x01 \x03a6\x8AWP`\x01\x93\x92\x91\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91a6\x81`@Q\x92\x83\x92\x83a@\x9BV[\x03\x90\xA2\x01a5BV[\x90Pa6\xC1`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a@\x9BV[\x03\x90\xFD[\x90\x92P=\x80_\x83>a6\xD7\x81\x83a,gV[\x81\x01\x90` \x81\x83\x03\x12a\x1DLW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW\x01\x81`\x1F\x82\x01\x12\x15a\x1DLW\x80Q\x90a7\x0E\x82a,\x8AV[\x92a7\x1C`@Q\x94\x85a,gV[\x82\x84R` \x83\x83\x01\x01\x11a\x1DLW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_a66V[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[`@\x90a,\xF7\x94\x92\x81R\x81` \x82\x01R\x01\x91a7AV[\x90a7\x83\x81\x80a0\xA7V[_[\x81\x81\x10a9-WPPPa7\x9C` \x82\x01\x82a0\xA7V[_[\x81\x81\x10a8\xBEWPPPa7\xB5`@\x82\x01\x82a0\xA7V[_[\x81\x81\x10a8OWPPP\x80``a7\xCF\x92\x01\x90a0\xA7V[\x90\x91_[\x82\x81\x10a7\xE0WPPPPV[a7\xEB\x81\x84\x86a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a8\x06W[\x01a7\xD3V[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca86a5g\x84\x88\x8Aa4\xEEV[\x90a8G`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a8\0V[a8Z\x81\x83\x85a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a8uW[\x01a7\xB7V[\x85\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa8\xA5a5g\x84\x87\x89a4\xEEV[\x90a8\xB6`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a8oV[a8\xC9\x81\x83\x85a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a8\xE4W[\x01a7\x9EV[\x85\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3a9\x14a5g\x84\x87\x89a4\xEEV[\x90a9%`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a8\xDEV[a98\x81\x83\x85a4\xEEV[5\x90`\x02\x82\x10\x15a\x1DLW`\x01\x80\x92\x14a9SW[\x01a7\x85V[\x85\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6a9\x83a5g\x84\x87\x89a4\xEEV[\x90a9\x94`@Q\x92\x83\x92\x87\x84a7aV[\x03\x90\xA2a9MV[\x80\x15\x80\x15a9\xEEW[\x80\x15a9\xE6W[\x80\x15a9\xD6W[a9\xD0Wd\x01\0\0\x03\xD0\x19`\x07\x81\x80\x93\x81\x81\x80\t\t\x08\x91\x80\t\x14\x90V[PP_\x90V[Pd\x01\0\0\x03\xD0\x19\x82\x10\x15a9\xB3V[P\x81\x15a9\xACV[Pd\x01\0\0\x03\xD0\x19\x81\x10\x15a9\xA5V[\x92\x93\x92\x90\x91_\x90\x80\x83\x03a;\xEBWPPd\x01\0\0\x03\xD0\x19_\x94\x83\x08a:'WPP\x90P_\x90_\x90V[_a:9\x92d\x01\0\0\x03\xD0\x19\x92aB\xFAV[\x91[\x82\x15\x15\x83\x81a;\xDAW[P\x80a;\xD2W[\x15a;tW\x90\x84\x93\x92\x93\x90`\x01\x93d\x01\0\0\x03\xD0\x19\x91\x87\x96[\x80\x15a;!W\x80\x84\x04\x96\x80\x98a:\xF4Wd\x01\0\0\x03\xD0\x19\x90\x88\td\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x81\x11a#\xDAWd\x01\0\0\x03\xD0\x19\x90\x8A\x96\x08\x97\x93\x81\x97\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a:\xC7W\x90a:\xBD\x91a-\x1FV[\x95\x92\x93\x96\x95a:eV[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[`$\x8A\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x12`\x04R\xFD[P\x94P\x94PP\x93\x80a;GWP\x90d\x01\0\0\x03\xD0\x19\x80\x92\x81\x80\x87\x80\t\x80\x92\t\x95\t\x90\t\x90V[\x80\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x92R`\x12`\x04R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FInvalid number\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[P`\x01a:LV[d\x01\0\0\x03\xD0\x19\x91P\x14\x15_a:EV[a;\xFF\x93d\x01\0\0\x03\xD0\x19\x93\x96\x92\x96a@\xC0V[\x91a:;V[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x1DLW\x01` \x815\x91\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1DLW\x81`\x05\x1B6\x03\x83\x13a\x1DLWV[\x905\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x826\x03\x01\x81\x12\x15a\x1DLW\x01\x90V[\x90` \x83\x82\x81R\x01` \x82`\x05\x1B\x85\x01\x01\x93\x83_\x91[\x84\x83\x10a<\xB0WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95`\x1F\x19\x82\x82\x03\x01\x85R\x865\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x846\x03\x01\x81\x12\x15a\x1DLW\x83\x01\x805`\x02\x81\x10\x15a\x1DLW\x82R` \x81\x015\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x826\x03\x01\x81\x12\x15a\x1DLW\x01` \x815\x91\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1DLW\x806\x03\x82\x13a\x1DLWa=f` \x92\x83\x92`@\x86\x81\x86`\x01\x99\x01R\x01\x91a7AV[\x98\x01\x96\x95\x01\x93\x01\x91\x90a<\xA0V[a,\xF7\x91a=\xDDa=\xD2a=\xB7a=\x9Ca=\x8E\x86\x80a<\x05V[`\x80\x87R`\x80\x87\x01\x91a<\x8AV[a=\xA9` \x87\x01\x87a<\x05V[\x90\x86\x83\x03` \x88\x01Ra<\x8AV[a=\xC4`@\x86\x01\x86a<\x05V[\x90\x85\x83\x03`@\x87\x01Ra<\x8AV[\x92``\x81\x01\x90a<\x05V[\x91``\x81\x85\x03\x91\x01Ra<\x8AV[\x80_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ T\x15_\x14a>\x9BWa>H\x81\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0a4\x05V[\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\0T\x90_R\x7F],\xC4K\xCFA\xD62H\x82\xF1\xC0\xD7\t\xF1\xB9\xAE \xCA\xEE\xBC\xBF#X\x12z=\n\x8C\"\xCC\x01` R`@_ U`\x01\x90V[P_\x90V[\x80_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ T\x15_\x14a>\x9BWa>\xFD\x81\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a4\x05V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x90_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R`@_ U`\x01\x90V[\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nTa@\x97Wa@\x07\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03a4\x05V[\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x03T\x7F\xCC\x1D/\x83\x84E\xDBz\xECC\x1D\xF9\xEE\x8A\x87\x1F@\xE7\xAA^\x06O\xC0Vc>\xF8\xC6\x0F\xAB{\x06_R\x7Fv*F\xA1\x1CF\x0B\x9B\xCB+\xB9\x86Q\xDA\x03\xB1\x92\xA0.*3\xAB&\xDA\x9B\xF9\xEDS\x82k\xC9\x04` R\x7F]\xEB\x94n\xBB\xF6\x17\xE3c1\xF9\xB8\xF5\x82\xF1\x16E#(9\xEC5\xAAN\xA6\xE3\x12LBb\xE9nU`\x01\x90V[_\x90V[\x90\x91a@\xB2a,\xF7\x93`@\x84R`@\x84\x01\x90a,\xFAV[\x91` \x81\x84\x03\x91\x01Ra,\xFAV[\x94\x92\x90\x93\x91\x85\x15\x80aB\xF2W[aB\xE7W\x80\x15\x80aB\xDFW[aB\xD7W`@Q`\x80\x91a@\xED\x83\x83a,gV[\x826\x837\x84\x15aB\xAAW\x84`\x01\x80\t\x80\x83R\x93\x85\x85`\x01\t\x98` \x84\x01\x99\x8AR`@\x84\x01\x92\x86\x84R\x87\x84Q`\x01\t\x93``\x86\x01\x94\x85R`@Q\x9A\x87\x8C\x01\x95\x8C\x87\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x11\x17a,:W\x8A\x80\x97\x95\x81\x96\x94\x82\x95`@RQ\x90\t\x8DRQ\x90\t\x94` \x8B\x01\x95\x86RQ\x90\t\x98`@\x89\x01\x99\x8ARQ\x90\t``\x87\x01\x90\x81R\x86Q\x88Q\x14\x80\x15\x90aB\x9EW[\x15aB@W\x84\x92\x83\x80\x80\x93\x81`@Q\x9C\x85aA\x98\x8F\x97\x88a,gV[6\x877Q\x8CQaA\xA8\x90\x83a-\x1FV[\x90\x08\x84RQ\x85QaA\xB9\x90\x83a-\x1FV[\x90\x08\x98` \x83\x01\x99\x8AR\x81\x80\x8B\x81\x80\x80\x80\x89Q\x8AQ\x90\t\x93`@\x8A\x01\x94\x85R\x81\x85Q\x8BQ\x90\t``\x90\x9A\x01\x99\x8ARQ\x80\t\x88QaA\xF6\x90\x83a-\x1FV[\x90\x08\x81\x80\x87Q\x85Q\x90\t`\x02\taB\r\x90\x83a-\x1FV[\x90\x08\x9CQ\x93Q\x90Q\x90\taB!\x8C\x83a-\x1FV[\x90\x08\x90\t\x92Q\x90Q\x90\taB5\x90\x83a-\x1FV[\x90\x08\x94Q\t\x91\x92\x91\x90V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FUse jacDouble function instead\0\0`D\x82\x01R\xFD[P\x81Q\x81Q\x14\x15aA|V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[PPP`\x01\x90V[P\x81\x15a@\xD9V[\x94P\x92P`\x01\x91\x90PV[P\x84\x15a@\xCDV[\x93\x91\x92\x90\x92\x81\x15aB\xAAW`\x01\x82\x80aCg\x81\x80\x80\x80\x97\x81\x80aCW\x9E\x81\x8F\x81\x81\x81\x92\t\x9A\x8B\x96\x81\x8F\x81\x80\x82\x81\x93\t\x9A\x88\t`\x04\t\x98\x80\t\x90\t\x92\x80\t`\x03\t\x08\x91\x81aCJ\x81\x83\x80\x08\x82a-\x1FV[\x81\x85\x80\t\x08\x9E\x8F\x83a-\x1FV[\x90\x08\x90\t\x93\x80\t`\x08\t\x83a-\x1FV[\x90\x08\x94\t`\x02\t\x90V[_\x90` \x92`@Q\x90\x84\x82\x01\x92\x83R`@\x82\x01R`@\x81RaC\x94``\x82a,gV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x1DAW_Q\x90V[\x81Q\x91\x90`A\x83\x03aC\xE4WaC\xDD\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aD\xC6V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aD\x99W\x80aD\0WPPV[`\x01\x81\x03aD0W\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03aDdWP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14aDnWPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aEJW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x1DAW_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15aE@W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V\xFE\xA1dsolcC\0\x08$\0\n",
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `RiscZeroVerifierStopped()` and selector `0x0b1d38a3`.
```solidity
error RiscZeroVerifierStopped();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RiscZeroVerifierStopped;
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
        impl ::core::convert::From<RiscZeroVerifierStopped> for UnderlyingRustTuple<'_> {
            fn from(value: RiscZeroVerifierStopped) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RiscZeroVerifierStopped {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RiscZeroVerifierStopped {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RiscZeroVerifierStopped()";
            const SELECTOR: [u8; 4] = [11u8, 29u8, 56u8, 163u8];
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: RISC_ZERO_VERIFIER_ROUTERReturn = r.into();
                        r._0
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: RISC_ZERO_VERIFIER_SELECTORReturn = r.into();
                        r._0
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: VERSIONReturn = r.into();
                        r._0
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: commitmentCountReturn = r.into();
                        r.count
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: commitmentTreeCapacityReturn = r.into();
                        r.capacity
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: commitmentTreeDepthReturn = r.into();
                        r.depth
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: commitmentTreeRootAtIndexReturn = r.into();
                        r.root
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: commitmentTreeRootCountReturn = r.into();
                        r.count
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `emergencyStop()` and selector `0x63a599a4`.
```solidity
function emergencyStop() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emergencyStopCall;
    ///Container type for the return parameters of the [`emergencyStop()`](emergencyStopCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emergencyStopReturn {}
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
            impl ::core::convert::From<emergencyStopCall> for UnderlyingRustTuple<'_> {
                fn from(value: emergencyStopCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emergencyStopCall {
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
            impl ::core::convert::From<emergencyStopReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emergencyStopReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emergencyStopReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl emergencyStopReturn {
            fn _tokenize(
                &self,
            ) -> <emergencyStopCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emergencyStopCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emergencyStopReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emergencyStop()";
            const SELECTOR: [u8; 4] = [99u8, 165u8, 153u8, 164u8];
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
                emergencyStopReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `execute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes))` and selector `0x73ab9916`.
```solidity
function execute(IProtocolAdapter.Transaction memory transaction) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`execute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes))`](executeCall) function.
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
            const SIGNATURE: &'static str = "execute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes))";
            const SELECTOR: [u8; 4] = [115u8, 171u8, 153u8, 22u8];
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getKindTableCommitmentReturn = r.into();
                        r.kindTableCommitment
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `implementation()` and selector `0x5c60da1b`.
```solidity
function implementation() external view returns (address current);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct implementationCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`implementation()`](implementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct implementationReturn {
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
            impl ::core::convert::From<implementationCall> for UnderlyingRustTuple<'_> {
                fn from(value: implementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for implementationCall {
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
            impl ::core::convert::From<implementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: implementationReturn) -> Self {
                    (value.current,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for implementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { current: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for implementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "implementation()";
            const SELECTOR: [u8; 4] = [92u8, 96u8, 218u8, 27u8];
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
                        let r: implementationReturn = r.into();
                        r.current
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: implementationReturn = r.into();
                        r.current
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isCommitmentTreeRootContainedReturn = r.into();
                        r.isContained
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isEmergencyStopped()` and selector `0xfddd4837`.
```solidity
function isEmergencyStopped() external view returns (bool isStopped);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isEmergencyStoppedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isEmergencyStopped()`](isEmergencyStoppedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isEmergencyStoppedReturn {
        #[allow(missing_docs)]
        pub isStopped: bool,
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
            impl ::core::convert::From<isEmergencyStoppedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isEmergencyStoppedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isEmergencyStoppedCall {
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
            impl ::core::convert::From<isEmergencyStoppedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isEmergencyStoppedReturn) -> Self {
                    (value.isStopped,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isEmergencyStoppedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isStopped: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isEmergencyStoppedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isEmergencyStopped()";
            const SELECTOR: [u8; 4] = [253u8, 221u8, 72u8, 55u8];
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
                        let r: isEmergencyStoppedReturn = r.into();
                        r.isStopped
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isEmergencyStoppedReturn = r.into();
                        r.isStopped
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isNullifierContainedReturn = r.into();
                        r.isContained
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: latestCommitmentTreeRootReturn = r.into();
                        r.root
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: nullifierAtIndexReturn = r.into();
                        r.nullifier
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: nullifierCountReturn = r.into();
                        r.count
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
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
        pub _0: bool,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `simulateExecute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes),bool)` and selector `0x87093eba`.
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
    ///Container type for the return parameters of the [`simulateExecute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes),bool)`](simulateExecuteCall) function.
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
            const SIGNATURE: &'static str = "simulateExecute((((bytes32,bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[]))[],(uint256,uint256),bytes32)[],bytes,bytes),bool)";
            const SELECTOR: [u8; 4] = [135u8, 9u8, 62u8, 186u8];
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
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
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`ProtocolAdapter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolAdapterCalls {
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
        emergencyStop(emergencyStopCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        getKindTableCommitment(getKindTableCommitmentCall),
        #[allow(missing_docs)]
        implementation(implementationCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isCommitmentTreeRootContained(isCommitmentTreeRootContainedCall),
        #[allow(missing_docs)]
        isEmergencyStopped(isEmergencyStoppedCall),
        #[allow(missing_docs)]
        isNullifierContained(isNullifierContainedCall),
        #[allow(missing_docs)]
        latestCommitmentTreeRoot(latestCommitmentTreeRootCall),
        #[allow(missing_docs)]
        nullifierAtIndex(nullifierAtIndexCall),
        #[allow(missing_docs)]
        nullifierCount(nullifierCountCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setKindTableCommitment(setKindTableCommitmentCall),
        #[allow(missing_docs)]
        simulateExecute(simulateExecuteCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
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
            [64u8, 243u8, 77u8, 66u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [89u8, 186u8, 146u8, 88u8],
            [92u8, 96u8, 218u8, 27u8],
            [92u8, 151u8, 90u8, 187u8],
            [99u8, 165u8, 153u8, 164u8],
            [107u8, 240u8, 160u8, 75u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 171u8, 153u8, 22u8],
            [135u8, 9u8, 62u8, 186u8],
            [141u8, 165u8, 203u8, 91u8],
            [154u8, 217u8, 29u8, 76u8],
            [160u8, 96u8, 86u8, 247u8],
            [173u8, 60u8, 177u8, 204u8],
            [189u8, 235u8, 68u8, 45u8],
            [192u8, 37u8, 48u8, 35u8],
            [193u8, 176u8, 190u8, 215u8],
            [196u8, 73u8, 86u8, 209u8],
            [196u8, 214u8, 109u8, 232u8],
            [200u8, 121u8, 219u8, 228u8],
            [227u8, 90u8, 93u8, 47u8],
            [242u8, 253u8, 227u8, 139u8],
            [253u8, 221u8, 72u8, 55u8],
            [254u8, 24u8, 171u8, 145u8],
            [255u8, 161u8, 173u8, 116u8],
            [255u8, 195u8, 63u8, 114u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(commitmentTreeRootAtIndex),
            ::core::stringify!(nullifierCount),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(commitmentTreeRootCount),
            ::core::stringify!(implementation),
            ::core::stringify!(paused),
            ::core::stringify!(emergencyStop),
            ::core::stringify!(RISC_ZERO_VERIFIER_ROUTER),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(execute),
            ::core::stringify!(simulateExecute),
            ::core::stringify!(owner),
            ::core::stringify!(nullifierAtIndex),
            ::core::stringify!(commitmentTreeDepth),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(latestCommitmentTreeRoot),
            ::core::stringify!(setKindTableCommitment),
            ::core::stringify!(isNullifierContained),
            ::core::stringify!(commitmentCount),
            ::core::stringify!(initialize),
            ::core::stringify!(isCommitmentTreeRootContained),
            ::core::stringify!(RISC_ZERO_VERIFIER_SELECTOR),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(isEmergencyStopped),
            ::core::stringify!(commitmentTreeCapacity),
            ::core::stringify!(VERSION),
            ::core::stringify!(getKindTableCommitment),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nullifierCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <implementationCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <emergencyStopCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <executeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <simulateExecuteCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nullifierAtIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentTreeDepthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setKindTableCommitmentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isNullifierContainedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isEmergencyStoppedCall as alloy_sol_types::SolCall>::SIGNATURE,
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
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                Self::emergencyStop(_) => {
                    <emergencyStopCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getKindTableCommitment(_) => {
                    <getKindTableCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::implementation(_) => {
                    <implementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCommitmentTreeRootContained(_) => {
                    <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isEmergencyStopped(_) => {
                    <isEmergencyStoppedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isNullifierContained(_) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestCommitmentTreeRoot(_) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nullifierAtIndex(_) => {
                    <nullifierAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nullifierCount(_) => {
                    <nullifierCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setKindTableCommitment(_) => {
                    <setKindTableCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::simulateExecute(_) => {
                    <simulateExecuteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ProtocolAdapterCalls>] = &[
                {
                    fn commitmentTreeRootAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeRootAtIndex)
                    }
                    commitmentTreeRootAtIndex
                },
                {
                    fn nullifierCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <nullifierCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::nullifierCount)
                    }
                    nullifierCount
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn commitmentTreeRootCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeRootCount)
                    }
                    commitmentTreeRootCount
                },
                {
                    fn implementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <implementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::implementation)
                    }
                    implementation
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ProtocolAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn emergencyStop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <emergencyStopCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::emergencyStop)
                    }
                    emergencyStop
                },
                {
                    fn RISC_ZERO_VERIFIER_ROUTER(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::RISC_ZERO_VERIFIER_ROUTER)
                    }
                    RISC_ZERO_VERIFIER_ROUTER
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn execute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ProtocolAdapterCalls::execute)
                    }
                    execute
                },
                {
                    fn simulateExecute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <simulateExecuteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::simulateExecute)
                    }
                    simulateExecute
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ProtocolAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn nullifierAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <nullifierAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::nullifierAtIndex)
                    }
                    nullifierAtIndex
                },
                {
                    fn commitmentTreeDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeDepthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeDepth)
                    }
                    commitmentTreeDepth
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn latestCommitmentTreeRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::latestCommitmentTreeRoot)
                    }
                    latestCommitmentTreeRoot
                },
                {
                    fn setKindTableCommitment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::setKindTableCommitment)
                    }
                    setKindTableCommitment
                },
                {
                    fn isNullifierContained(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::isNullifierContained)
                    }
                    isNullifierContained
                },
                {
                    fn commitmentCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentCount)
                    }
                    commitmentCount
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn isCommitmentTreeRootContained(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::isCommitmentTreeRootContained)
                    }
                    isCommitmentTreeRootContained
                },
                {
                    fn RISC_ZERO_VERIFIER_SELECTOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::RISC_ZERO_VERIFIER_SELECTOR)
                    }
                    RISC_ZERO_VERIFIER_SELECTOR
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn isEmergencyStopped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isEmergencyStoppedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::isEmergencyStopped)
                    }
                    isEmergencyStopped
                },
                {
                    fn commitmentTreeCapacity(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeCapacity)
                    }
                    commitmentTreeCapacity
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ProtocolAdapterCalls::VERSION)
                    }
                    VERSION
                },
                {
                    fn getKindTableCommitment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ProtocolAdapterCalls>] = &[
                {
                    fn commitmentTreeRootAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeRootAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeRootAtIndex)
                    }
                    commitmentTreeRootAtIndex
                },
                {
                    fn nullifierCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <nullifierCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::nullifierCount)
                    }
                    nullifierCount
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn commitmentTreeRootCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeRootCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeRootCount)
                    }
                    commitmentTreeRootCount
                },
                {
                    fn implementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <implementationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::implementation)
                    }
                    implementation
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn emergencyStop(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <emergencyStopCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::emergencyStop)
                    }
                    emergencyStop
                },
                {
                    fn RISC_ZERO_VERIFIER_ROUTER(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_ROUTERCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::RISC_ZERO_VERIFIER_ROUTER)
                    }
                    RISC_ZERO_VERIFIER_ROUTER
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn execute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::execute)
                    }
                    execute
                },
                {
                    fn simulateExecute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <simulateExecuteCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::simulateExecute)
                    }
                    simulateExecute
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn nullifierAtIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <nullifierAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::nullifierAtIndex)
                    }
                    nullifierAtIndex
                },
                {
                    fn commitmentTreeDepth(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeDepthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeDepth)
                    }
                    commitmentTreeDepth
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn latestCommitmentTreeRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::latestCommitmentTreeRoot)
                    }
                    latestCommitmentTreeRoot
                },
                {
                    fn setKindTableCommitment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::setKindTableCommitment)
                    }
                    setKindTableCommitment
                },
                {
                    fn isNullifierContained(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::isNullifierContained)
                    }
                    isNullifierContained
                },
                {
                    fn commitmentCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentCount)
                    }
                    commitmentCount
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn isCommitmentTreeRootContained(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::isCommitmentTreeRootContained)
                    }
                    isCommitmentTreeRootContained
                },
                {
                    fn RISC_ZERO_VERIFIER_SELECTOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <RISC_ZERO_VERIFIER_SELECTORCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::RISC_ZERO_VERIFIER_SELECTOR)
                    }
                    RISC_ZERO_VERIFIER_SELECTOR
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn isEmergencyStopped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <isEmergencyStoppedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::isEmergencyStopped)
                    }
                    isEmergencyStopped
                },
                {
                    fn commitmentTreeCapacity(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <commitmentTreeCapacityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::commitmentTreeCapacity)
                    }
                    commitmentTreeCapacity
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterCalls::VERSION)
                    }
                    VERSION
                },
                {
                    fn getKindTableCommitment(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterCalls> {
                        <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
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
                Self::emergencyStop(inner) => {
                    <emergencyStopCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getKindTableCommitment(inner) => {
                    <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::implementation(inner) => {
                    <implementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isCommitmentTreeRootContained(inner) => {
                    <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isEmergencyStopped(inner) => {
                    <isEmergencyStoppedCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setKindTableCommitment(inner) => {
                    <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::emergencyStop(inner) => {
                    <emergencyStopCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getKindTableCommitment(inner) => {
                    <getKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::implementation(inner) => {
                    <implementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isCommitmentTreeRootContained(inner) => {
                    <isCommitmentTreeRootContainedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isEmergencyStopped(inner) => {
                    <isEmergencyStoppedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setKindTableCommitment(inner) => {
                    <setKindTableCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        RiscZeroVerifierSelectorMismatch(RiscZeroVerifierSelectorMismatch),
        #[allow(missing_docs)]
        RiscZeroVerifierStopped(RiscZeroVerifierStopped),
        #[allow(missing_docs)]
        Simulated(Simulated),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        ZeroKindTableCommitmentNotAllowed(ZeroKindTableCommitmentNotAllowed),
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
            [11u8, 29u8, 56u8, 163u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [57u8, 169u8, 64u8, 197u8],
            [62u8, 229u8, 174u8, 181u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 20u8, 152u8, 49u8],
            [119u8, 77u8, 93u8, 225u8],
            [120u8, 162u8, 34u8, 28u8],
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
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [219u8, 120u8, 140u8, 43u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 212u8, 75u8, 76u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [249u8, 132u8, 158u8, 163u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(RiscZeroVerifierStopped),
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(PreExistingNullifier),
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(Simulated),
            ::core::stringify!(ZeroKindTableCommitmentNotAllowed),
            ::core::stringify!(RiscZeroVerifierSelectorMismatch),
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
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(PreExistingRoot),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(DeltaMismatch),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(NonExistingRoot),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RiscZeroVerifierStopped as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingNullifier as alloy_sol_types::SolError>::SIGNATURE,
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <Simulated as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::SIGNATURE,
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
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingRoot as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <DeltaMismatch as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 29usize;
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
                Self::RiscZeroVerifierSelectorMismatch(_) => {
                    <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RiscZeroVerifierStopped(_) => {
                    <RiscZeroVerifierStopped as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Simulated(_) => <Simulated as alloy_sol_types::SolError>::SELECTOR,
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroKindTableCommitmentNotAllowed(_) => {
                    <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::SELECTOR
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
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ProtocolAdapterErrors>] = &[
                {
                    fn RiscZeroVerifierStopped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <RiscZeroVerifierStopped as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::RiscZeroVerifierStopped)
                    }
                    RiscZeroVerifierStopped
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn PreExistingNullifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PreExistingNullifier as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::PreExistingNullifier)
                    }
                    PreExistingNullifier
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn Simulated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <Simulated as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolAdapterErrors::Simulated)
                    }
                    Simulated
                },
                {
                    fn ZeroKindTableCommitmentNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::RiscZeroVerifierSelectorMismatch)
                    }
                    RiscZeroVerifierSelectorMismatch
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn ZeroRiscZeroVerifierRouterNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ZeroRiscZeroVerifierSelectorNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn PointNotOnCurve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PointNotOnCurve as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::PointNotOnCurve)
                    }
                    PointNotOnCurve
                },
                {
                    fn ForwarderCallOutputMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ForwarderCallOutputMismatch)
                    }
                    ForwarderCallOutputMismatch
                },
                {
                    fn EmptyTransactionNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::EmptyTransactionNotAllowed)
                    }
                    EmptyTransactionNotAllowed
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolAdapterErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn PreExistingRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PreExistingRoot as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::PreExistingRoot)
                    }
                    PreExistingRoot
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn DeltaMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <DeltaMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::DeltaMismatch)
                    }
                    DeltaMismatch
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn NonExistingRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <NonExistingRoot as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolAdapterErrors::NonExistingRoot)
                    }
                    NonExistingRoot
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<ProtocolAdapterErrors>] = &[
                {
                    fn RiscZeroVerifierStopped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <RiscZeroVerifierStopped as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::RiscZeroVerifierStopped)
                    }
                    RiscZeroVerifierStopped
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn PreExistingNullifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PreExistingNullifier as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::PreExistingNullifier)
                    }
                    PreExistingNullifier
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn Simulated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <Simulated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::Simulated)
                    }
                    Simulated
                },
                {
                    fn ZeroKindTableCommitmentNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::RiscZeroVerifierSelectorMismatch)
                    }
                    RiscZeroVerifierSelectorMismatch
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn ZeroRiscZeroVerifierRouterNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierRouterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ZeroRiscZeroVerifierSelectorNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ZeroRiscZeroVerifierSelectorNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn PointNotOnCurve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PointNotOnCurve as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::PointNotOnCurve)
                    }
                    PointNotOnCurve
                },
                {
                    fn ForwarderCallOutputMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ForwarderCallOutputMismatch)
                    }
                    ForwarderCallOutputMismatch
                },
                {
                    fn EmptyTransactionNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::EmptyTransactionNotAllowed)
                    }
                    EmptyTransactionNotAllowed
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn PreExistingRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <PreExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::PreExistingRoot)
                    }
                    PreExistingRoot
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn DeltaMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <DeltaMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::DeltaMismatch)
                    }
                    DeltaMismatch
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn NonExistingRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <NonExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolAdapterErrors::NonExistingRoot)
                    }
                    NonExistingRoot
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolAdapterErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
            DECODE_VALIDATE_SHIMS[idx](data)
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
                Self::RiscZeroVerifierSelectorMismatch(inner) => {
                    <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RiscZeroVerifierStopped(inner) => {
                    <RiscZeroVerifierStopped as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ZeroKindTableCommitmentNotAllowed(inner) => {
                    <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::RiscZeroVerifierSelectorMismatch(inner) => {
                    <RiscZeroVerifierSelectorMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RiscZeroVerifierStopped(inner) => {
                    <RiscZeroVerifierStopped as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ZeroKindTableCommitmentNotAllowed(inner) => {
                    <ZeroKindTableCommitmentNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
        const COUNT: usize = 14usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActionExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActionExecuted)
                }
                Some(
                    <ApplicationPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ApplicationPayload as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ApplicationPayload)
                }
                Some(
                    <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CommitmentTreeRootAdded)
                }
                Some(<DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DiscoveryPayload as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DiscoveryPayload)
                }
                Some(<ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExternalPayload as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ExternalPayload)
                }
                Some(
                    <ForwarderCallExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ForwarderCallExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ForwarderCallExecuted)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <KindTableCommitmentUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KindTableCommitmentUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KindTableCommitmentUpdated)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(<ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ResourcePayload as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ResourcePayload)
                }
                Some(
                    <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TransactionExecuted)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
                }
                _ => {
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
        ///Creates a new call builder for the [`emergencyStop`] function.
        pub fn emergencyStop(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, emergencyStopCall, N> {
            self.call_builder(&emergencyStopCall)
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            transaction: <IProtocolAdapter::Transaction as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, executeCall, N> {
            self.call_builder(&executeCall { transaction })
        }
        ///Creates a new call builder for the [`getKindTableCommitment`] function.
        pub fn getKindTableCommitment(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKindTableCommitmentCall, N> {
            self.call_builder(&getKindTableCommitmentCall)
        }
        ///Creates a new call builder for the [`implementation`] function.
        pub fn implementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, implementationCall, N> {
            self.call_builder(&implementationCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { initialOwner })
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
        ///Creates a new call builder for the [`isEmergencyStopped`] function.
        pub fn isEmergencyStopped(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, isEmergencyStoppedCall, N> {
            self.call_builder(&isEmergencyStoppedCall)
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
