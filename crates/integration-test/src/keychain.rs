//! EVM-signing extension for the testkit [`Keychain`]: derives the secp256k1
//! signer and its address from the keychain's account key. The alloy signer type
//! lives here, in the EVM layer, not in the chain-agnostic testkit.

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use anoma_pa_testkit::fixtures::identities::Keychain;

/// Adds EVM signing to a test [`Keychain`].
pub trait EvmSigner {
    /// The secp256k1 signer for this identity.
    fn signer(&self) -> PrivateKeySigner;

    /// The EVM address of this identity.
    fn address(&self) -> Address {
        self.signer().address()
    }
}

impl EvmSigner for Keychain {
    fn signer(&self) -> PrivateKeySigner {
        PrivateKeySigner::from_bytes(&self.signing_key.into())
            .expect("keychain holds a valid secp256k1 key")
    }
}
