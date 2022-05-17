#![allow(dead_code)]
use super::config::EngineConfig;
use super::state::preset::SimulationId;
use super::state::State;

/// Simulation Engine
pub struct Engine {
    pub config: EngineConfig,
    pub states: Vec<State>,
}
impl Engine {
    /// Creates a new engine instance
    pub fn new() -> Engine {
        let config = EngineConfig::new();
        let states = vec![];
        Engine { config, states }
    }
    /// Initializes initial state & configuration
    pub fn init(&mut self, sim_id: &Option<SimulationId>) {
        let mut initial_state = State::new();
        initial_state.init(sim_id, &mut self.config);
        self.states.push(initial_state);
    }
    /// Runs Engine
    pub fn run(&mut self) {
        let n = self.config.max_step_id;
        let _: Vec<()> = (0..n).map(|_| self.step()).collect();
    }
    /// Forwards engine by one time-step
    pub fn step(&mut self) {
        let mut next = self.states[self.config.step_id].clone();
        next.step(&self.config, &self.states);
        self.states.push(next);
        self.config.step_id += 1;
    }
    /// Exports States (to File or Database)
    pub fn export(&self) {
        todo!()
    }
}
