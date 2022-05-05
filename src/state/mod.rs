#![allow(unused_doc_comments)]
pub mod preset;

use super::config::EngineConfig;
use super::entity::attribute;
use super::integrator::Integrator;
use super::interaction::Interaction;
use super::system::System;

/// State
#[derive(Debug, Clone)]
pub struct State {
    // systems: Vec<EntityVector>,
    // id: usize,
    pub systems: Vec<System>,
}
impl State {
    /// Creates new State
    pub fn new() -> Self {
        // let id = 0;
        let systems = vec![];
        State {
            // id,
            systems,
        }
    }

    /// Initializes State & Configuration
    pub fn init(&mut self, config: &mut EngineConfig) {
        self.systems = preset::initialize(config);
    }

    /// Forwards State
    pub fn step(&mut self, config: &EngineConfig, states: &Vec<State>) {
        let current_state = &states[config.step_id];

        let _neighborhoods = prepare_neighborhoods();
        //  sys_id -> fn(entity_id) -> Vec<entity_jd>
        //  TODO implement search for interaction partners
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
                let interactions = match &config.interactions.get(&sys_id) {
                    Some(val) => match val.get(&sys_jd) {
                        Some(val) => val,
                        None => &empty,
                    },
                    None => &empty,
                };

                for interaction in interactions.iter() {
                    match interaction {
                        /// Newtonian Gravity
                        Interaction::NewtonianGravity(integrator) => match sys_1 {
                            System::PhysicalBodies(sys_1) => match sys_2 {
                                System::PhysicalBodies(sys_2) => {
                                    for (ent_id, entity_1) in sys_1.entities.iter_mut().enumerate()
                                    {
                                        for (ent_jd, entity_2) in sys_2.entities.iter().enumerate()
                                        {
                                            let ent_id = (sys_id, ent_id);
                                            let ent_jd = (sys_jd, ent_jd);
                                            match integrator {
                                                Integrator::EulerExplicit => {
                                                    euler_exp_gravity(
                                                        (ent_id, entity_1),
                                                        (ent_jd, entity_2),
                                                    );
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    // e.g. planets influencing each other
                                }
                                System::ForceField(_sys_2) => {
                                    // e.g. object falling to the ground
                                }
                            },
                            System::ForceField(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // e.g. field around star
                                }
                                System::ForceField(_sys_2) => {
                                    // e.g. gravity waves
                                }
                            },
                        },
                        /// Coulomb Interaction
                        Interaction::Coulomb(_integrator) => match sys_1 {
                            System::PhysicalBodies(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // e.g. proton-proton repulsion
                                }
                                System::ForceField(_sys_2) => {
                                    // e.g. charges accelerated by voltage
                                }
                            },
                            System::ForceField(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // e.g. electro-static field generated by charges
                                }
                                System::ForceField(_sys_2) => {
                                    // e.g. light
                                }
                            },
                        },
                        Interaction::Diffusion => match sys_1 {
                            System::ForceField(_sys_1) => match sys_2 {
                                System::ForceField(_sys_2) => {
                                    // e.g. tracer density field
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        Interaction::LennardJones(_integrator) => match sys_1 {
                            System::PhysicalBodies(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // e.g. atom-atom interaction
                                }
                                _ => {}
                            },
                            System::ForceField(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // for visualization?
                                }
                                _ => {}
                            },
                        },
                        Interaction::GameOfLife => match sys_1 {
                            _ => {}
                        },
                        Interaction::Collision => match sys_1 {
                            System::PhysicalBodies(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // e.g. billiard balls
                                }
                                System::ForceField(_sys_2) => {
                                    // e.g. wall collisions (?)
                                }
                            },
                            System::ForceField(_sys_1) => match sys_2 {
                                System::PhysicalBodies(_sys_2) => {
                                    // e.g. ball breaking window
                                }
                                System::ForceField(_sys_2) => {
                                    // ?
                                }
                            },
                        },
                        _ => {}
                    };
                }
            }
        }
    }
}

fn euler_exp_gravity<T>(mass_obj_1: ((usize, usize), &mut T), mass_obj_2: ((usize, usize), &T))
where
    T: attribute::Mass + attribute::Position + attribute::Velocity,
{
    let (ent_id, ent_jd) = (mass_obj_1.0, mass_obj_2.0);
    if ent_id == ent_jd {
        return ();
    }
    let mass_obj_1 = mass_obj_1.1;
    let mass_obj_2 = mass_obj_2.1;
    // println!("{:?} == {:?}", ent_id, ent_jd);
    // println!(
    //     "{:?} == {:?}",
    //     mass_obj_1.get_position(),
    //     mass_obj_2.get_position()
    // );

    const G: f64 = 1.;
    const DT: f64 = 0.01;
    let m1 = mass_obj_1.get_mass();
    let m2 = mass_obj_2.get_mass();
    let pos1 = mass_obj_1.get_position();
    let pos2 = mass_obj_2.get_position();

    let dr: Vec<f64> = (0..3).map(|i| pos2[i] - pos1[i]).collect();
    // println!("{:?} {:?}", pos1, pos2);
    // println!("{:?}", dr);
    let r = (0..3).map(|i| dr[i].powf(2.)).sum::<f64>().powf(0.5);
    let force = G * (m1 * m2) / r.powf(2.);

    let vel1 = mass_obj_1.get_mut_velocity();
    let vel1: Vec<f64> = vel1
        .iter()
        .enumerate()
        .map(|(i, v)| v + force * dr[i] / r * DT)
        .collect();

    let pos1 = mass_obj_1.get_mut_position();
    let a: Vec<f64> = pos1
        .iter()
        .enumerate()
        .map(|(i, x)| x + vel1[i] * DT)
        .collect();
    *pos1 = [a[0], a[1], a[2]];
}

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
