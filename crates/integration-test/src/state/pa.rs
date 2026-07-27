use alloy::primitives::Address;
use anoma_pa_testkit::environment::Environment;
use anoma_pa_testkit::environment::State;
use anoma_pa_testkit::environment::StateBuilder;
use anyhow::Context;

pub const KEY_PA_ADDRESS: &str = "evm.pa.address";

#[inline]
pub fn insert_pa_address(builder: &mut StateBuilder, address: Address) {
    builder.insert(KEY_PA_ADDRESS, address);
}

#[inline]
pub fn pa_address<E>(env: &E) -> anyhow::Result<Address>
where
    E: Environment,
{
    pa_address_in_state(env.state())
}

#[inline]
pub fn pa_address_in_state(state: &State) -> anyhow::Result<Address> {
    state
        .get::<Address>(KEY_PA_ADDRESS)
        .copied()
        .context("failed to retrieve protocol adapter address from env")
}

#[cfg(test)]
mod tests {
    use super::*;

    use anoma_pa_testkit::mocks::MockEnvironment;

    #[test]
    fn pa_address_key_format() {
        assert_eq!(KEY_PA_ADDRESS, "evm.pa.address");
    }

    #[test]
    fn insert_and_resolve_pa_address() {
        let expected = Address::from([0x33; 20]);

        let state = {
            let mut builder = StateBuilder::new();
            insert_pa_address(&mut builder, expected);
            builder.finalize()
        };

        let mut env = MockEnvironment::new();
        env.expect_state().return_const(state);

        let resolved = pa_address(&env).expect("must resolve stored protocol adapter address");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn missing_pa_address_errors() {
        let state = StateBuilder::new().finalize();

        let mut env = MockEnvironment::new();
        env.expect_state().return_const(state);

        let err = pa_address(&env).expect_err("must fail on missing protocol adapter address key");
        assert!(
            err.to_string()
                .contains("failed to retrieve protocol adapter address from env"),
            "error should mention missing pa address, got: {err}"
        );
    }
}
