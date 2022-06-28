pub mod three_body_figure_eight;
pub mod three_body_moon;
use crate::config::EngineConfig;
use mxyz_universe::system::System;

pub enum SimulationId {
    ThreeBodyFigureEight,
    ThreeBodyMoon,
}

/// Initialize State & Config
pub fn initialize(sim_id: &Option<SimulationId>, config: &mut EngineConfig) -> Vec<System> {
    let mut systems = vec![];
    match sim_id {
        None => {}
        Some(id) => match id {
            SimulationId::ThreeBodyFigureEight => {
                three_body_figure_eight::preset(&mut systems, config)
            }
            SimulationId::ThreeBodyMoon => three_body_moon::preset(&mut systems, config),
            _ => todo!(),
        },
    }
    systems
}
