pub mod discrete_field;
pub mod physical_objects;
use super::config::EngineConfig;
use super::entity::Entity;
use super::state::tmp;
use super::state::State;

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
        /// Loads current State
        let current_state = &states[config.step_id.0];
        let system_id = self.id; // TODO some-day, remove (with trees)

        for (other_id, other) in current_state.systems.iter().enumerate() {
            // println!("    {} - {}", system_id, other_id);
            /// Loads interactions for each pair
            let interactions = tmp::get_interactions(system_id, other_id, &config); // TODO clean up?
            let _self_interaction = system_id == other_id; // TODO some-day, remove (with trees)
            /// Applies interactions between systems (Pass to System)
            for interaction in interactions {
                let matrix = &interaction.matrix;
                let integrator = &matrix.rows[system_id].entries[other_id].integrator;
                match integrator {
                    None => {}
                    Some(integrator) => integrator.step(self, other, interaction),
                }
            }
        }
    }
}

/// System Variant Enumeration
#[derive(Clone)]
pub enum SystemVariant {
    DiscreteField,
    PhysicalObjects,
}

// match system {
//     DiscreteField(system) => match other {
//         DiscreteField(other) => system.step(),
//         PhysicalObjects(other) => system.step(),
//     },
//     PhysicalObjects(system) => match other {
//         DiscreteField(other) => system.step(interactions),
//         PhysicalObjects(other) => system.step(interactions),
//     },
// }
// system.interact_with_field(&other, ints, cfg, self_interaction);
// system.interact_with_objects(&other, ints, cfg, self_interaction);
// system.interact_with_field(&other, ints, cfg, self_interaction);
// system.interact_with_objects(&other, ints, cfg, self_interaction);
