#![allow(unused_doc_comments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
pub mod preset;

use super::config::EngineConfig;
use super::integrator::Integrator;
use super::interaction::Interaction;
use super::system::System;
use crate::attribute;
use preset::SimulationId;

/// State Structure
#[derive(Clone)]
pub struct State {
    pub id: usize,
    pub systems: Vec<System>,
}
impl State {
    /// Creates new State
    pub fn new() -> Self {
        let id = 0;
        let systems = vec![];
        State { id, systems }
    }
    /// Initializes State & Configuration
    pub fn init(&mut self, sim_id: &Option<SimulationId>, config: &mut EngineConfig) {
        self.systems = preset::initialize(&sim_id, config);
    }
    /// Forwards State
    pub fn step(&mut self, config: &EngineConfig, states: &Vec<State>) {
        let current_state = &states[config.step_id];

        //  TODO implement search for interaction partners
        let _neighborhoods = prepare_neighborhoods();
        //  sys_id -> fn(entity_id) -> Vec<entity_jd>
        //    - GameOfLife: create function, later return cells around entity_id
        //    - PhysicalBodies: construct tree -> later return Vec<entity_jd> for entity_id
        //    - only for system that are "felt" by others

        for (sys_id, sys_1) in self.systems.iter_mut().enumerate() {
            for (sys_jd, sys_2) in current_state.systems.iter().enumerate() {
                // TODO get list of interacting entities
                // let position = [0., 0., 0.];
                // let neighborhood: Vec<usize> = neighborhoods[sys_jd](position);
                // use cache? -> tree speed-up
                // let tree = Tree {};

                let empty = vec![];
                let interactions = match get_interactions(&config, sys_id) {
                    None => &empty,
                    Some(interactions) => interactions,
                };

                use System::DiscreteField;
                use System::PhysicalObjects;

                for interaction in interactions.into_iter() {
                    println!("{:?}", interaction);
                    match sys_1 {
                        DiscreteField(sys_1) => match sys_2 {
                            DiscreteField(sys_2) => {
                                sys_1.interact_with_discrete_field(&sys_2, &interaction)
                            }
                            PhysicalObjects(sys_2) => {
                                sys_1.interact_with_physical_objects(&sys_2, interaction)
                            }
                            _ => todo!(),
                        },
                        PhysicalObjects(sys_1) => match sys_2 {
                            DiscreteField(sys_2) => {
                                sys_1.interact_with_discrete_field(&sys_2, &interaction)
                            }
                            PhysicalObjects(sys_2) => {
                                sys_1.interact_with_physical_objects(&sys_2, interaction)
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
            }
        }
    }
}

pub fn get_interactions(config: &EngineConfig, sys_id: usize) -> Option<&Vec<Interaction>> {
    let interactions = match config.interactions.get(&sys_id) {
        Some(val) => match val.get(&sys_id) {
            Some(val) => Some(val.clone()),
            None => None,
        },
        None => None,
    };
    interactions
}

// fn euler_exp_gravity<T>(mass_obj_1: ((usize, usize), &mut T), mass_obj_2: ((usize, usize), &T))
// where
//     T: attribute::Mass + attribute::Position + attribute::Velocity,
// {
//     let (ent_id, ent_jd) = (mass_obj_1.0, mass_obj_2.0);
//     if ent_id == ent_jd {
//         return ();
//     }
//     let mass_obj_1 = mass_obj_1.1;
//     let mass_obj_2 = mass_obj_2.1;
//     // println!("{:?} == {:?}", ent_id, ent_jd);
//     // println!(
//     //     "{:?} == {:?}",
//     //     mass_obj_1.get_position(),
//     //     mass_obj_2.get_position()
//     // );

//     const G: f64 = 1.;
//     const DT: f64 = 0.01;
//     let m1 = mass_obj_1.get_mass();
//     let m2 = mass_obj_2.get_mass();
//     let pos1 = mass_obj_1.get_position();
//     let pos2 = mass_obj_2.get_position();

//     let dr: Vec<f64> = (0..3).map(|i| pos2[i] - pos1[i]).collect();
//     // println!("{:?} {:?}", pos1, pos2);
//     // println!("{:?}", dr);
//     let r = (0..3).map(|i| dr[i].powf(2.)).sum::<f64>().powf(0.5);
//     let force = G * (m1 * m2) / r.powf(2.);

//     let vel1 = mass_obj_1.get_mut_velocity();
//     let vel1: Vec<f64> = vel1
//         .iter()
//         .enumerate()
//         .map(|(i, v)| v + force * dr[i] / r * DT)
//         .collect();

//     let pos1 = mass_obj_1.get_mut_position();
//     let a: Vec<f64> = pos1
//         .iter()
//         .enumerate()
//         .map(|(i, x)| x + vel1[i] * DT)
//         .collect();
//     *pos1 = [a[0], a[1], a[2]];
// }

fn prepare_neighborhoods() -> Vec<fn() -> Vec<usize>> {
    // let systems = 0..2;
    let res = vec![];
    // for system in systems {
    //     let f = || vec![];
    //     res.push(f);
    // }
    res
}

// enum Tree {
//     // x,y,z   ->   vec[node]
//     Sectors, // all nodes from same + neighboring sectors
//     QuadOct, // quad/oct tree, return all nodes in opening angle
//     Total,   // all nodes are returned
// }
