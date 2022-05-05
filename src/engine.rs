#![allow(dead_code)]

use super::config::EngineConfig;
use super::state::State;

#[derive(Debug)]
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
    pub fn init(&mut self) {
        let mut initial_state = State::new();
        initial_state.init(&mut self.config);
        self.states.push(initial_state);
    }
    /// Runs Engine
    pub fn run(&mut self) {
        for _ in 0..self.config.max_step_id {
            self.step();
            self.config.step_id += 1;
        }
    }
    /// Forwards engine by one time-step
    pub fn step(&mut self) {
        let current = &self.states[self.config.step_id];
        let mut next = current.clone();
        next.step(&self.config, &self.states);
        self.states.push(next);
    }
    /// Exports States (to File or Database)
    pub fn export(&self) {
        todo!()
    }
}
