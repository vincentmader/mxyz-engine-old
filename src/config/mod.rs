#[derive(Debug)]
pub struct EngineConfig {
    pub step_id: usize,
}
impl EngineConfig {
    pub fn new() -> Self {
        let step_id = 0;
        EngineConfig { step_id }
    }
}
