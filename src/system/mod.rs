pub mod discrete_field;
pub mod physical_objects;
use super::config::EngineConfig;
use super::entity::Entity;
use super::integrator::Integrator;
use super::state::tmp;
use super::state::State;

/// System Variant Enumeration
#[derive(Clone)]
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
    pub fn new(variant: SystemVariant) -> Self {
        let id = 0; // TODO safe?
        let entities = vec![];
        System {
            id,
            variant,
            entities,
        }
    }
    pub fn step(&mut self, config: &EngineConfig, states: &Vec<State>) {
        let system_id = self.id; // TODO some-day, remove (with trees)
        println!("\tSYSTEM {}", self.id);

        /// Loads current State
        let state = &states[config.step_id.0];
        /// Gets all Integrators for this System
        let integrators = tmp::get_integrators(system_id, &config).unwrap();
        /// Loops over Integrators & Applies
        for integrator in integrators {
            let other_ids = get_other_ids(&integrator, &state);
            integrator.step(self, &state, &other_ids);
        }
    }
}

pub fn get_other_ids(integrator: &Integrator, state: &State) -> Vec<usize> {
    (0..state.systems.len())
        .filter(|id| {
            let mut foo = false;
            for interaction in integrator.interactions.iter() {
                if match interaction.matrix.entries[*id] {
                    None => false,
                    Some(active) => active,
                } {
                    foo = true;
                }
            }
            foo
        })
        .collect()
}
