use alloy::node_bindings::AnvilInstance;
use anoma_pa_testkit::environment::Environment as CoreEnvironment;
use anoma_pa_testkit::environment::State;

pub mod config;
mod setup;

pub use super::common::commitment_tree::CommitmentTree;
pub use super::common::protocol_adapter::ProtocolAdapter;
pub use anoma_pa_testkit::transaction::Transaction;
pub use config::E2eConfig;

pub struct Environment {
    pub anvil: AnvilInstance,
    pub state: State,
    pub prover: anoma_pa_testkit::prover::QueueProver,
    pub protocol_adapter: ProtocolAdapter,
}

impl CoreEnvironment for Environment {
    type Transaction = Transaction;
    type ProtocolAdapter = ProtocolAdapter;
    type Prover = anoma_pa_testkit::prover::QueueProver;

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
