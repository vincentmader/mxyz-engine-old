#![allow(unreachable_patterns)]

pub mod three_body_figure_eight;
pub mod three_body_moon;
use crate::config::EngineConfig;
use mxyz_universe::system::System;

#[derive(Debug)]
pub enum SimulationVariant {
    // Undefined, // TODO
    ThreeBodyFigureEight,
    ThreeBodyMoon,
}
// impl From<&str> for SimulationVariant {
//     fn from(a: &str) -> Self {
//         match a {
//             "3body-moon" => SimulationVariant::ThreeBodyMoon,
//             _ => Self::Undefined, // TODO
//         }
//     }
// }

/// Initialize State & Config
pub fn initialize(
    simulation_variant: &Option<SimulationVariant>,
    config: &mut EngineConfig,
) -> Vec<System> {
    let mut systems = vec![];
    match simulation_variant {
        None => {}
        Some(id) => match id {
            SimulationVariant::ThreeBodyFigureEight => {
                three_body_figure_eight::preset(&mut systems, config)
            }
            SimulationVariant::ThreeBodyMoon => three_body_moon::preset(&mut systems, config),
            _ => todo!(),
        },
    }
    systems
}
