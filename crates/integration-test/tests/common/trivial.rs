//! The testkit's trivial resource, with a nonce that names these tests.

pub use anoma_pa_testkit::fixtures::trivial::{Overrides, created};

use anoma_pa_testkit::fixtures::action::ActionResource;
use anoma_pa_testkit::fixtures::trivial;

/// A consumed trivial resource whose default nonce other senders do not pick, so a forked chain does not hold its nullifier.
pub fn consumed(seed: u8, overrides: Overrides) -> ActionResource {
    let mut nonce = *b"anoma pa-evm integration test 00";
    nonce[31] = seed;
    trivial::consumed(
        seed,
        Overrides {
            nonce: overrides.nonce.or(Some(nonce)),
            ..overrides
        },
    )
}
