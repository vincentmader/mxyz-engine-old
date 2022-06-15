pub mod three_body_figure_eight;
use crate::config::EngineConfig;
use crate::system::System;
use three_body_figure_eight::three_body_figure_eight;

pub enum SimulationId {
    ThreeBodyFigureEight,
}

/// Initialize State & Config
pub fn initialize(sim_id: &Option<SimulationId>, config: &mut EngineConfig) -> Vec<System> {
    let mut systems = vec![];
    match sim_id {
        None => {}
        Some(id) => match id {
            SimulationId::ThreeBodyFigureEight => three_body_figure_eight(&mut systems, config),
        },
    }
    systems
}
