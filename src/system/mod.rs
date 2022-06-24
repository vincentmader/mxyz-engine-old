pub mod discrete_field;
pub mod physical_objects;
use super::config::EngineConfig;
use super::entity::Entity;
use super::state::tmp;
use super::state::State;
// use serde::{Deserialize, Serialize};

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
        /// Loops over Integrators & Applies Interactions
        for integrator in integrators {
            let other_ids = tmp::get_other_ids(&integrator, &state);
            integrator.step(self, &state, &other_ids);
        }
    }
}
impl ToBytes for System {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = vec![];

        let _system_variant_id = match self.variant {
            SystemVariant::PhysicalObjects => 0,
            SystemVariant::DiscreteField => 1,
            _ => todo!(),
        };

        for entity in self.entities.iter() {
            let _foo = entity.to_bytes();
        }
        //...
        bytes
    }
}
impl System {
    pub fn get_variant_id(system_variant: &SystemVariant) -> usize {
        match system_variant {
            SystemVariant::PhysicalObjects => 0,
            SystemVariant::DiscreteField => 1,
            _ => todo!(),
        }
    }
}

// TODO move else-where
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
pub trait FromBytes {
    fn from_bytes(bytes: Vec<u8>) -> Self;
}
