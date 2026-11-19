use alloy::node_bindings::AnvilInstance;
use anoma_pa_testkit::environment::Environment as CoreEnvironment;
use anoma_pa_testkit::environment::State;

mod setup;

pub use super::common::commitment_tree::CommitmentTree;
pub use super::common::protocol_adapter::ProtocolAdapter;
pub use anoma_pa_testkit::transaction::Transaction;

/// Integration test execution environment.
///
/// Setup contract:
/// - All fields on this environment and its nested structures are public on purpose.
/// - Setup code should mutate/inspect the concrete environment directly.
/// - Test execution code should accept `impl anoma_pa_testkit::environment::Environment`
///   and use typed state helpers instead of concrete fields.
pub struct Environment {
    pub anvil: AnvilInstance,
    pub state: State,
    pub prover: anoma_pa_testkit::prover::LocalProver,
    pub protocol_adapter: ProtocolAdapter,
}

impl CoreEnvironment for Environment {
    type Transaction = Transaction;
    type ProtocolAdapter = ProtocolAdapter;
    type Prover = anoma_pa_testkit::prover::LocalProver;

    fn prover(&self) -> &Self::Prover {
        &self.prover
    }

    fn state(&self) -> &State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    fn protocol_adapter(&self) -> &Self::ProtocolAdapter {
        &self.protocol_adapter
    }

    fn protocol_adapter_mut(&mut self) -> &mut Self::ProtocolAdapter {
        &mut self.protocol_adapter
    }
}
