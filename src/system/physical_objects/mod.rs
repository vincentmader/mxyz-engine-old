use crate::config::EngineConfig;
use crate::entity::object::PhysicalObject;
use crate::integrator::Integrator;
use crate::integrator::IntegratorVariant;
use crate::system::discrete_field::DiscreteField;
// use crate::state::tmp;

#[derive(Clone)]
/// System: Physical Objects
pub struct PhysicalObjects {
    pub entities: Vec<Box<dyn PhysicalObject>>,
}
impl PhysicalObjects {
    pub fn new() -> Self {
        let entities = vec![];
        PhysicalObjects { entities }
    }
    fn step(&mut self, sys_id: usize, config: &EngineConfig) {
        // let integrators = tmp::get_integrators(sys_id, sys_jd: usize, config: &EngineConfig)
    }
    /// Object-Field Interaction
    ///
    /// Examples:
    /// - charges accelerated by voltage
    /// - object falling to the ground
    /// - wall collisions
    pub fn interact_with_field(
        &mut self,
        _other: &DiscreteField,
        integrators: &Vec<&Integrator>,
        _config: &EngineConfig,
        _self_interaction: bool,
    ) {
        for integrator in integrators.iter() {
            integrator.step_objects(self);
            match integrator.variant {
                // IntegratorVariant::EulerExplicit => {}
                // IntegratorVariant::Verlet => {}
                // IntegratorVariant::RungeKutta4 => {} // _ => {}
                _ => todo!(),
            }
        }
        // for interaction in interactions.iter() {
        // match interaction.variant {
        //     InteractionVariant::Force(_) => todo!(),
        //     InteractionVariant::Collision(_) => todo!(),
        // }
        // }
    }
    /// Object-Object Interaction
    ///
    /// Examples:
    /// - mutual gravitational attraction
    /// - Coulomb & Lennard-Jones
    /// - boid forces (avoidance, cohesion, alignment)
    pub fn interact_with_objects(
        &mut self,
        // others: &Vec<Box<dyn PhysicalObject>>,
        other: &PhysicalObjects,
        integrators: &Vec<&Integrator>,
        _config: &EngineConfig,
        self_interaction: bool,
    ) {
        for integrator in integrators.iter() {
            integrator.step_objects(self);
            match integrator.variant {
                IntegratorVariant::EulerExplicit => {}
                // IntegratorVariant::RungeKutta4 => {}
                // IntegratorVariant::Verlet => {}
                _ => todo!(),
            }
        }
        // for interaction in interactions.iter() {
        // let entities = &mut self.entities; // TODO filter
        // let others = &other.entities;

        // let integrator = &interaction.integrator;
        // match &interaction.variant {
        //     // InteractionVariant::Collision(_) => todo!(),
        //     // InteractionVariant::Force(f) => {
        //     //     f.apply_to_objects_from_objects(entities, others, integrator, self_interaction)
        //     // }
        //     _ => todo!(),
        // }
        // }
    }
}
