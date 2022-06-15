pub mod discrete_field;
pub mod physical_objects;
use super::config::EngineConfig;
use super::entity::Entity;
use super::state::tmp;
use super::state::State;

/// System Variant Enumeration
#[derive(Debug, Clone)]
pub enum SystemVariant {
    DiscreteField,
    PhysicalObjects,
}

/// System Structure
#[derive(Clone)]
pub struct System {
    pub id: usize,
    pub variant: SystemVariant,
    pub entities: Vec<Box<dyn Entity>>,
}
impl System {
    /// Creates a new System Struct Instance
    pub fn new(variant: SystemVariant) -> Self {
        let id = 0; // TODO safe?
        let entities = vec![];
        System {
            id,
            variant,
            entities,
        }
    }
    /// Forwards System to next Time-Step
    pub fn step(&mut self, config: &EngineConfig, states: &Vec<State>) {
        println!(
            "SYS-{}: {:?} ({} entities)",
            self.id,
            self.variant,
            self.entities.len()
        );
        let system_id = self.id; // TODO some-day, remove (with trees)
        /// Loads current State
        let state = &states[config.step_id.0];
        /// Gets all Integrators for this System
        let integrators = tmp::get_integrators(system_id, &config).unwrap();
        /// Loops over Integrators & Applies
        for integrator in integrators {
            let other_ids = tmp::get_other_ids(&integrator, &state);
            integrator.step(self, &state, &other_ids);
        }
    }
}
