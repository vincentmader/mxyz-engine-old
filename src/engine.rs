use super::config::EngineConfig;
use super::state::State;

#[derive(Debug)]
pub struct Engine {
    states: Vec<State>,
    config: EngineConfig,
}
impl Engine {
    pub fn new() -> Engine {
        let config = EngineConfig::new();
        let states = vec![];
        Engine { config, states }
    }
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
    pub fn step(&mut self) {
        let current = &self.states[self.config.step_id];
        let mut next = current.clone();
        next.step(&self.config, &self.states);
        self.states.push(next);
    }
}
