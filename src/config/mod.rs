use std::collections::HashMap;

pub mod system;
pub use system::SystemConfig;

use super::interaction::Interaction;

#[derive(Debug)]
pub struct EngineConfig {
    pub systems: Vec<SystemConfig>,
    pub interactions: HashMap<usize, HashMap<usize, Vec<Interaction>>>,
    pub step_id: usize,
    pub max_step_id: usize,
}
impl EngineConfig {
    pub fn new() -> Self {
        let systems = vec![];
        let interactions = HashMap::new();
        let step_id = 0;
        let max_step_id = usize::MAX;

        EngineConfig {
            systems,
            interactions,
            step_id,
            max_step_id,
        }
    }
}
