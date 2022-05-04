pub mod system;
pub use system::SystemConfig;

#[derive(Debug)]
pub struct EngineConfig {
    pub step_id: usize,
    pub systems: Vec<SystemConfig>,
}
impl EngineConfig {
    pub fn new() -> Self {
        let step_id = 0;
        let systems = vec![];
        EngineConfig { step_id, systems }
    }
}
