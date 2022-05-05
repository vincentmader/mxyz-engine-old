use std::collections::HashMap;

use crate::config::EngineConfig;
use crate::config::SystemConfig;
use crate::entity;
use crate::integrator::Integrator;
use crate::interaction::Interaction;
use crate::system;
use crate::system::System;

/// Initialize State & Config
pub fn initialize(config: &mut EngineConfig) -> Vec<System> {
    let mut systems = vec![];

    // 0. SYSTEM
    let _id = 0; // TODO save in system struct? sync with loop over state.systems?
    let mut field = system::ForceField::new();
    for _entity_idx in 0..2 {
        let entity = entity::ForceVector::new();
        field.entities.push(entity);
    }
    systems.push(System::ForceField(field));
    let sys_conf = SystemConfig::new();
    config.systems.push(sys_conf);

    // 1. SYSTEM
    let id = 1;
    let mut field = system::PhysicalBodies::new();
    for entity_id in 0..2 {
        let m = 1.;
        let pos = [2. * entity_id as f64, 0., 0.];
        let vel = [0., 0., 0.];
        let entity = entity::PhysicalBody::new(m, pos, vel);
        field.entities.push(entity);
    }
    systems.push(System::PhysicalBodies(field));
    let sys_conf = SystemConfig::new();
    config.systems.push(sys_conf);

    // INTERACTIONS
    // TODO
    let interactions = HashMap::from([(
        id,
        vec![Interaction::NewtonianGravity(Integrator::EulerExplicit)],
    )]);
    config.interactions.insert(id, interactions);

    systems
}
