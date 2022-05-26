use super::entity::Entity as PhysicalObject;
use super::interaction::Interaction;
use crate::state::State;
use crate::system::System;

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
        match self.variant {
            IntegratorVariant::EulerExplicit => {
                for other_id in other_ids.iter() {
                    let other = &state.systems[*other_id];
                    for interaction in self.interactions.iter() {
                        if !interaction.matrix.entries[*other_id].unwrap() {
                            continue;
                        }
                        println!("\t\t{:#?}: {}-{}", self.variant, system.id, other_id);
                    }
                }
            }
            IntegratorVariant::CellularAutomaton => {
                for other_id in other_ids.iter() {
                    let _other = &state.systems[*other_id];
                    // println!("\t\t{:#?}: {} - {}", self.variant, system.id, other_id);
                }
            }
            _ => todo!(),
        }
    }
}

pub fn euler_explicit(
    entity: &mut Box<dyn PhysicalObject>,
    other: &Box<dyn PhysicalObject>,
    force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
) {
    let f = force_getter(entity, &other);
    let m1 = entity.get_mass();
    let a = [f[0] / m1, f[1] / m1, f[2] / m1];
    const DT: f64 = 0.001; // TODO
    let v1 = entity.get_velocity();
    let v1: Vec<f64> = (0..3).map(|i| v1[i] + a[i] * DT).collect();
    entity.set_velocity(&[v1[0], v1[1], v1[2]]);
    let y1 = entity.get_position();
    let y1: Vec<f64> = (0..3).map(|i| y1[i] + v1[i] * DT).collect();
    entity.set_position(&[y1[0], y1[1], y1[2]]);
}

pub fn runge_kutta_4(
    _entity: &mut Box<dyn PhysicalObject>,
    _other: &Box<dyn PhysicalObject>,
    _force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
) {
}

pub fn verlet(
    _entity: &mut Box<dyn PhysicalObject>,
    _other: &Box<dyn PhysicalObject>,
    _force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
) {
}
