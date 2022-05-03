use super::config::EngineConfig;
use super::state::State;

#[derive(Debug)]
/// Simulation Engine
pub struct Engine {
    states: Vec<State>,
    config: EngineConfig,
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
        let initial_state = State::new(0);
        self.states.push(initial_state);
    }
    pub fn run(&mut self, nr_of_steps: Option<usize>) {
        let nr_of_steps = match nr_of_steps {
            Some(nr) => nr,
            None => usize::MAX,
        };
        for _ in 0..nr_of_steps {
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
    /// Saves state (file or database)
    fn save(&self) {
        todo!()
    }
}
