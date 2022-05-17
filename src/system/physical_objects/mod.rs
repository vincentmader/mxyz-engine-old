use crate::entity::object::PhysicalObject;
use crate::interaction::Interaction;
use crate::system::discrete_field::DiscreteField;

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
    pub fn interact_with_discrete_field(
        &mut self,
        field: &DiscreteField,
        interaction: &Interaction,
    ) {
        match interaction {
            // e.g. wall collisions
            Interaction::Collision => {}
            // e.g. object falling to the ground
            Interaction::NewtonianGravity(integrator) => {}
            // e.g. charges accelerated by voltage
            Interaction::Coulomb(integrator) => {}
            _ => {}
        }
    }
    pub fn interact_with_physical_objects(
        &mut self,
        field: &PhysicalObjects,
        interaction: &Interaction,
    ) {
        match interaction {
            // e.g. Billard balls
            Interaction::Collision => {}
            // e.g. interaction of atoms
            Interaction::LennardJones(integrator) => {}
            // e.g. binary star
            Interaction::NewtonianGravity(integrator) => {}
            // e.g. proton-proton repulsion
            Interaction::Coulomb(integrator) => {}
            _ => {}
        }
    }
}

// for (ent_id, entity_1) in sys_1.entities.iter_mut().enumerate()
//                                                         {
//                                                             let ent_id = (sys_id, ent_id);
//                                                             for (ent_jd, entity_2) in sys_2.entities.iter().enumerate()
//                                                             {
//                                                                 let ent_jd = (sys_jd, ent_jd);
//                                                                 match integrator {
//                                                                     Integrator::EulerExplicit => {
//                                                                         // euler_exp_gravity(
//                                                                         //     (ent_id, entity_1),
//                                                                         //     (ent_jd, entity_2),
//                                                                         // );
//                                                                     }
//                                                                     _ => {}
//                                                                 }
//                                                             }
//                                                         }
//                                                         // e.g. planets influencing each other
