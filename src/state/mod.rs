pub mod preset;
pub mod tmp;
use super::config::EngineConfig;
use mxyz_universe::system::System;
use preset::SimulationId;

/// State
#[derive(Clone)]
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
    pub fn step(&mut self, config: &EngineConfig, states: &Vec<State>) {
        println!("\n--------\n {}.\n--------\n", config.step_id.0);
        /// Loads current State
        let state = &states[config.step_id.0];
        /// Creates "neighborhoods"
        let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes
        /// Prepares system-ids   TODO remove maybe?
        for (id, sys) in self.systems.iter_mut().enumerate() {
            sys.id = id; // needed e.g. when removing/adding systems
        }
        /// Loops over all Systems
        for system in self.systems.iter_mut() {
            println!(
                "SYS-{}: {:?} ({} entities)",
                system.id,
                system.variant,
                system.entities.len()
            );

            let system_id = system.id; // TODO some-day, remove (with trees)
            /// Gets all Integrators for this System
            let integrators = tmp::get_integrators(system_id, &config).unwrap();
            /// Loops over all Integrators
            for integrator in integrators {
                /// Gets all Interacting Systems for this Interaction
                let other_ids = tmp::get_other_ids(&integrator, &state);
                /// Applies Interaction
                integrator.step(system, &state, &other_ids);
            }
        }
    }
}
