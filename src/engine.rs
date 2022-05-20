use super::config::EngineConfig;
use super::state::preset::SimulationId;
use super::state::State;

/// Simulation Engine
pub struct Engine {
    pub config: EngineConfig,
    pub states: Vec<State>,
}
impl Engine {
    /// Creates a new Engine instance
    pub fn new() -> Engine {
        let config = EngineConfig::new();
        let states = vec![];
        Engine { config, states }
    }
    /// Initializes State & Configuration
    pub fn init(&mut self, sim_id: &Option<SimulationId>) {
        let mut initial_state = State::new();
        initial_state.init(sim_id, &mut self.config);
        self.states.push(initial_state);
    }
    /// Runs Engine
    pub fn run(&mut self) {
        let _: Vec<()> = (0..self.config.step_id.1).map(|_| self.step()).collect();
    }
    /// Forwards Engine by 1 Time-Step
    pub fn step(&mut self) {
        let mut next = self.states[self.config.step_id.0].clone();
        next.step(&self.config, &self.states);
        self.states.push(next);
        self.config.step_id.0 += 1;
    }
    /// Exports States (to File or Database)
    pub fn export(&self) {
        todo!()
    }
}
