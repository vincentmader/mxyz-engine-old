use super::interaction::{Interaction, InteractionVariant};
use crate::state::State;
use crate::system::System;

const dt: f64 = 0.001; // TODO move else-where

#[derive(Debug)]
/// Entity Integrator
pub enum IntegratorVariant {
    EulerExplicit,
    EulerImplicit,
    RungeKutta2,
    RungeKutta4,
    RungeKuttaN,
    VelocityVerlet,
    Verlet,
    LeapFrog,
    BulirschStoer,
    Collision,
    CellularAutomaton,
    MonteCarlo,
}

#[derive(Debug)]
pub struct Integrator {
    pub variant: IntegratorVariant,
    pub interactions: Vec<Interaction>,
    //  TODO specify neighborhood/tree calculation (or in interaction?)
}
impl Integrator {
    pub fn new(variant: IntegratorVariant) -> Self {
        let interactions = vec![];
        Integrator {
            variant,
            interactions,
        }
    }
    pub fn step(&self, system: &mut System, state: &State, other_ids: &Vec<usize>) {
        /// Loops over entities (they all need to be updated!)
        let entity_ids = 0..system.entities.len(); // TODO only update some?
        for entity_id in entity_ids {
            let entity = &mut system.entities[entity_id];
            /// Depending on IntegratorVariant, the Updating can take on different forms
            match self.variant {
                /// For Explicit Euler:
                /// - dy/dt = a(t,y) =  f(t,y)
                /// Steps:
                /// - get f(t,y) as sum of f(t,y) for all interacting entities
                /// - update velocity using f(t,y)
                IntegratorVariant::EulerExplicit => {
                    let mut acceleration = [0., 0., 0.];
                    /// Loops over the other systems
                    for other_id in other_ids.iter() {
                        let other = &state.systems[*other_id];
                        /// Loops over the Integrator's Interactions (skips if it doesn't apply)
                        //  TODO get interactions to-apply outside of entity-loop
                        for interaction in self.interactions.iter() {
                            if interaction.matrix.entries[*other_id].unwrap() == false {
                                continue;
                            }
                            println!("\t\t{:#?}: {}-{}", self.variant, system.id, other_id);
                            /// Loops over the Entities in the interacting System
                            let other_ids = 0..other.entities.len(); // TODO get ids
                            for other_id in other_ids {
                                let other = &other.entities[other_id];
                                println!(
                                    "\t\t\t{}-{}: {:?}",
                                    entity_id, other_id, interaction.variant
                                );
                                /// Updates Velocity
                                match &interaction.variant {
                                    InteractionVariant::Force(f) => {
                                        let mass_1 = entity.get_mass(); // TODO move further up?
                                        let force = [0., 0., 0.]; //  TODO calculate force
                                        f.calculate_from(entity, other);
                                        acceleration = [
                                            acceleration[0] + force[0] / mass_1,
                                            acceleration[1] + force[1] / mass_1,
                                            acceleration[2] + force[2] / mass_1,
                                        ];
                                    }
                                    _ => todo!(),
                                }
                            }
                        }
                    }
                    /// Updates Position Vector
                    let velocity = entity.get_velocity();
                    let velocity = [
                        velocity[0] + acceleration[0] * dt,
                        velocity[1] + acceleration[1] * dt,
                        velocity[2] + acceleration[2] * dt,
                    ];
                    entity.set_velocity(&velocity);
                }
                IntegratorVariant::CellularAutomaton => {}
                _ => todo!(),
            }
        }
    }
}

// pub fn euler_explicit(
//     entity: &mut Box<dyn PhysicalObject>,
//     other: &Box<dyn PhysicalObject>,
//     force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
// ) {
//     let f = force_getter(entity, &other);
//     let m1 = entity.get_mass();
//     let a = [f[0] / m1, f[1] / m1, f[2] / m1];
//     const DT: f64 = 0.001; // TODO
//     let v1 = entity.get_velocity();
//     let v1: Vec<f64> = (0..3).map(|i| v1[i] + a[i] * DT).collect();
//     entity.set_velocity(&[v1[0], v1[1], v1[2]]);
//     let y1 = entity.get_position();
//     let y1: Vec<f64> = (0..3).map(|i| y1[i] + v1[i] * DT).collect();
//     entity.set_position(&[y1[0], y1[1], y1[2]]);
// }

// pub fn runge_kutta_4(
//     _entity: &mut Box<dyn PhysicalObject>,
//     _other: &Box<dyn PhysicalObject>,
//     _force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
// ) {
// }

// pub fn verlet(
//     _entity: &mut Box<dyn PhysicalObject>,
//     _other: &Box<dyn PhysicalObject>,
//     _force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
// ) {
// }
