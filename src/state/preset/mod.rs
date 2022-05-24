use crate::config::{EngineConfig, SystemConfig};
use crate::entity;
use crate::integrator::{Integrator, IntegratorVariant};
use crate::interaction::{
    force, interaction_matrix::InteractionMatrix, Interaction, InteractionVariant,
};
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
    // SYSTEMS
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

    // INTERACTIONS
    // ========================================================================
    let integrators = &mut config.integrators;

    let mut matrix = InteractionMatrix::new();
    matrix.init(&systems);
    matrix.entries[0][0] = Some(true);
    let mut interactions = vec![];

    let variant = force::ForceVariant::NewtonianGravity;
    let force = force::Force { variant };
    let variant = InteractionVariant::Force(force);
    let interaction = Interaction { variant };
    interactions.push(interaction);

    let variant = IntegratorVariant::EulerExplicit;
    let integrator = Integrator {
        matrix,
        variant,
        interactions,
    };
    integrators.push(integrator);
}
