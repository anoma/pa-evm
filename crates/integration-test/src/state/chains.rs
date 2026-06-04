use alloy_chains::NamedChain;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::environment::State;
use anoma_pa_testkit::environment::StateBuilder;
use anyhow::Context;

use crate::state::keys::chain_id_key;
use crate::state::keys::chain_name_key;

#[inline]
pub fn insert_chain(builder: &mut StateBuilder, chain: NamedChain) {
    builder.insert(chain_id_key(), chain as u64);
    builder.insert(chain_name_key(), chain);
}

#[inline]
pub fn chain_id<E>(env: &E) -> anyhow::Result<u64>
where
    E: Environment,
{
    chain_id_in_state(env.state())
}

#[inline]
pub fn chain_id_in_state(state: &State) -> anyhow::Result<u64> {
    state
        .get::<u64>(chain_id_key())
        .copied()
        .context("failed to retrieve chain id from env")
}

#[inline]
pub fn chain_name<E>(env: &E) -> anyhow::Result<NamedChain>
where
    E: Environment,
{
    chain_name_in_state(env.state())
}

#[inline]
pub fn chain_name_in_state(state: &State) -> anyhow::Result<NamedChain> {
    state
        .get::<NamedChain>(chain_name_key())
        .copied()
        .context("failed to retrieve chain name from env")
}

#[cfg(test)]
mod tests {
    use super::*;

    use anoma_pa_testkit::mocks::MockEnvironment;

    #[test]
    fn insert_and_resolve_chain_id_for_named_chain() {
        let mut builder = StateBuilder::new();
        insert_chain(&mut builder, NamedChain::Sepolia);

        let state = builder.finalize();

        let mut env = MockEnvironment::new();
        env.expect_state().return_const(state);

        let resolved = chain_id(&env).expect("must resolve chain id");
        assert_eq!(resolved, 11155111);
    }

    #[test]
    fn insert_chain_id_resolves_named_chain() {
        let mut builder = StateBuilder::new();
        insert_chain(&mut builder, NamedChain::Mainnet);

        let state = builder.finalize();

        let mut env = MockEnvironment::new();
        env.expect_state().return_const(state);

        let resolved = chain_name(&env).expect("must resolve chain name");
        assert_eq!(resolved, NamedChain::Mainnet);
    }

    #[test]
    fn missing_chain_id_key_errors() {
        let state = StateBuilder::new().finalize();

        let mut env = MockEnvironment::new();
        env.expect_state().return_const(state);

        let err = chain_id(&env).expect_err("must fail on missing key");
        assert!(
            err.to_string()
                .contains("failed to retrieve chain id from env"),
            "error should mention missing key, got: {err}"
        );
    }
}
