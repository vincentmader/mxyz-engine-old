pub mod preset;
pub mod tmp;
use super::config::EngineConfig;
use mxyz_universe::system::System;
use preset::SimulationId;

/// State
#[derive(Debug, Clone)]
pub struct State {
    pub id: usize,
    pub systems: Vec<System>,
}
impl State {
    /// Creates new instance of State Structure
    pub fn new() -> Self {
        let id = 0;
        let systems = vec![];
        State { id, systems }
    }

    /// Initializes State & configuration
    pub fn init(&mut self, sim_id: &Option<SimulationId>, config: &mut EngineConfig) {
        self.systems = preset::initialize(&sim_id, config);
    }

    /// Forwards State
    pub fn next(&self, config: &EngineConfig, states: &Vec<State>) -> State {
        let mut next_state = State::new();

        /// Loads current State
        let current_state = &states[config.step_id.0];

        /// Creates "neighborhoods"
        let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes

        /// Loops over systems & forwards each
        for system in &current_state.systems {
            let mut next_system = system.clone();

            /// Gets all Integrators for this System & loops over them
            let integrators = tmp::get_integrators(&system, &config).unwrap();
            for integrator in integrators {
                /// Gets all Interacting Systems for this Interaction
                let other_ids = tmp::get_other_ids(&integrator, &current_state);
                /// Applies Interaction
                integrator.step(&mut next_system, &current_state, &other_ids);
            }
            next_state.systems.push(next_system);
        }
        next_state
    }
}
