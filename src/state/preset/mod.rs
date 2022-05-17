use std::collections::HashMap;

use crate::config::EngineConfig;
use crate::config::SystemConfig;
use crate::entity;
use crate::integrator::Integrator;
use crate::interaction::Interaction;
use crate::system;
use crate::system::System;

pub enum SimulationId {
    Foo,
}

/// Initialize State & Config
pub fn initialize(sim_id: &Option<SimulationId>, config: &mut EngineConfig) -> Vec<System> {
    let mut systems = vec![];
    match sim_id {
        None => {}
        Some(id) => match id {
            SimulationId::Foo => foo(&mut systems, config),
            _ => todo!(),
        },
    }
    systems
}

pub fn foo(systems: &mut Vec<System>, config: &mut EngineConfig) {
    // 0. SYSTEM
    let _id = 0; // TODO save in system struct? sync with loop over state.systems?
    let mut field = system::discrete_field::DiscreteField::new();
    for _entity_idx in 0..2 {
        let entity = entity::field::fluid_cell::FluidCell::new([0., 0., 0.], 0.);
        field.entities.push(Box::new(entity));
    }
    systems.push(System::DiscreteField(field));
    let sys_conf = SystemConfig::new();
    config.systems.push(sys_conf);

    // 1. SYSTEM
    let id = 1;
    let mut field = system::physical_objects::PhysicalObjects::new();
    for entity_id in 0..2 {
        let m = 1.;
        let pos = [2. * entity_id as f64 - 1., 0., 0.];
        let vel = [0., 0.005 * (2. * entity_id as f64 - 1.), 0.];
        let entity = entity::object::planet::Planet::new(m, pos, vel);
        field.entities.push(Box::new(entity));
    }
    systems.push(System::PhysicalObjects(field));
    let sys_conf = SystemConfig::new();
    config.systems.push(sys_conf);

    // INTERACTIONS
    // TODO
    let interactions = HashMap::from([(
        id,
        vec![Interaction::NewtonianGravity(Integrator::EulerExplicit)],
    )]);
    config.interactions.insert(id, interactions);
}
