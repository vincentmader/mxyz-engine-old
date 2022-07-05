use super::config::EngineConfig;
use super::state::State;
use mxyz_universe::preset::SimulationVariant;

/// MXYZ Simulation Engine
pub struct Engine {
    pub client_id: usize,
    pub engine_id: usize,
    pub config: EngineConfig,
    pub states: Vec<State>,
}

impl Engine {
    /// Creates a new engine instance
    pub fn new(client_id: usize, engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];
        Engine {
            client_id,
            engine_id,
            config,
            states,
        }
    }

    /// Initializes state & config
    pub fn init(&mut self, simulation_variant: Option<SimulationVariant>) {
        println!("MXYZ-Engine: Initializing...");
        let mut initial_state = State::new();
        initial_state.init(simulation_variant, &mut self.config);
        self.states.push(initial_state);
    }

    /// Runs engine
    pub fn run(&mut self) {
        println!("MXYZ-Engine: Running...");
        for _ in 0..self.config.step_id.1 {
            self.step();
        }
    }

    /// Forwards engine by one time-step
    pub fn step(&mut self) {
        // Load current state.
        let current_state = &self.states[self.config.step_id.0];

        // Forward to next time-step.
        let next = current_state.next(&self.config, &self.states);
        self.states.push(next);
        self.config.step_id.0 += 1;
    }

    pub fn send(&self) {}

    pub fn receive(&self) {}

    pub fn get_unsaved_state_ids(&self) -> Vec<usize> {
        let a = self
            .states
            .iter()
            .filter(|state| {
                state.state_id
                    >= match self.config.last_export_step_id {
                        None => 0,
                        Some(e) => e + 1,
                    }
            })
            .map(|state| state.state_id)
            .collect();
        a
    }
}
