use crate::config::{EngineConfig, SystemConfig};
use crate::entity;
use crate::integrator::{Integrator, IntegratorVariant};
use crate::interaction::force::{Force, ForceVariant};
use crate::interaction::interaction_matrix::InteractionMatrix;
use crate::interaction::{Interaction, InteractionVariant};
use crate::system::{self, SystemVariant};

const NR_OF_STEPS: usize = 2221;

pub enum SimulationId {
    ThreeBodyFigureEight,
}

/// Initialize State & Config
pub fn initialize(sim_id: &Option<SimulationId>, config: &mut EngineConfig) -> Vec<SystemVariant> {
    let mut systems = vec![];
    match sim_id {
        None => {}
        Some(id) => match id {
            SimulationId::ThreeBodyFigureEight => three_body_figure_eight(&mut systems, config),
        },
    }
    systems
}

pub fn three_body_figure_eight(systems: &mut Vec<SystemVariant>, config: &mut EngineConfig) {
    // I. SYSTEMS
    // ========================================================================
    config.step_id.1 = NR_OF_STEPS;

    // Objects
    // ------------------------------------------------------------------------
    let mut sys = system::physical_objects::PhysicalObjects::new();
    let speed = 0.;
    for entity_id in 0..2 {
        let m = 1.;
        let x = [2. * (entity_id as f64 - 0.5), 0., 0.];
        let v = [0., speed * (2. * entity_id as f64 - 1.), 0.];
        let entity = entity::object::planet::Planet::new(m, x, v);
        sys.entities.push(Box::new(entity));
    }
    systems.push(SystemVariant::PhysicalObjects(sys));
    let sys_conf = SystemConfig::new();
    config.systems.push(sys_conf);

    // Objects
    // ------------------------------------------------------------------------
    let mut sys = system::physical_objects::PhysicalObjects::new();
    let speed = 0.;
    for entity_id in 0..2 {
        let m = 1.;
        let x = [2. * (entity_id as f64 - 0.5), 0., 0.];
        let v = [0., speed * (2. * entity_id as f64 - 1.), 0.];
        let entity = entity::object::planet::Planet::new(m, x, v);
        sys.entities.push(Box::new(entity));
    }
    systems.push(SystemVariant::PhysicalObjects(sys));
    let sys_conf = SystemConfig::new();
    config.systems.push(sys_conf);

    // II. INTERACTIONS ()
    // ========================================================================
    let interactions = &mut config.interactions;

    // 1. Interaction Variant
    let force = Force::new(ForceVariant::NewtonianGravity);
    let interaction_variant = InteractionVariant::Force(force);
    // 2. Matrix Init
    let mut matrix = InteractionMatrix::new();
    matrix.init(&systems);
    // 3. Matrix Integrator Init
    let integrator_variant = IntegratorVariant::EulerExplicit;
    matrix.rows[0].entries[0].integrator = Some(integrator_variant);
    let integrator_variant = IntegratorVariant::RungeKutta4;
    matrix.rows[1].entries[1].integrator = Some(integrator_variant);
    // 4. Push Interaction
    let interaction = Interaction {
        variant: interaction_variant,
        // active: true,
        matrix,
    };
    interactions.push(interaction);
}
