pub mod preset;
pub mod tmp;
use super::config::EngineConfig;
use super::system::SystemVariant::{self, DiscreteField, PhysicalObjects};
use preset::SimulationId;
use tmp::{get_interactions, prepare_neighborhoods};

/// State
#[derive(Clone)]
pub struct State {
    pub id: usize,
    pub systems: Vec<SystemVariant>,
}
impl State {
    /// Creates new instance of State Structure
    pub fn new() -> Self {
        let (id, systems) = (0, vec![]);
        State { id, systems }
    }
    /// Initializes State & configuration
    pub fn init(&mut self, sim_id: &Option<SimulationId>, config: &mut EngineConfig) {
        self.systems = preset::initialize(&sim_id, config);
    }
    /// Forwards State
    pub fn step(&mut self, config: &EngineConfig, states: &Vec<State>) {
        println!("\n{}. ----------------------------------", config.step_id.0);
        /// Loads current State & creates neighborhoods
        let current_state = &states[config.step_id.0];
        let _neighborhoods = prepare_neighborhoods(); // TODO get relevant neighbors/nodes
        /// Loops over all pairs of systems & loads interactions for each pair
        for (system_id, system) in self.systems.iter_mut().enumerate() {
            for (other_id, other) in current_state.systems.iter().enumerate() {
                println!("    {} - {}", system_id, other_id);
                let interactions = get_interactions(system_id, other_id, &config); // TODO clean up?
                let self_interaction = system_id == other_id;
                /// Applies interactions between systems (Pass to System)
                let (ints, cfg) = (&interactions, &config);
                match system {
                    DiscreteField(system) => match other {
                        DiscreteField(other) => {
                            system.interact_with_field(&other, ints, cfg, self_interaction);
                        }
                        PhysicalObjects(other) => {
                            system.interact_with_objects(&other, ints, cfg, self_interaction);
                        }
                    },
                    PhysicalObjects(system) => match other {
                        DiscreteField(other) => {
                            system.interact_with_field(&other, ints, cfg, self_interaction);
                        }
                        PhysicalObjects(other) => {
                            system.interact_with_objects(&other, ints, cfg, self_interaction);
                        }
                    },
                }
            }
        }
    }
}
