use mxyz_config::EngineConfig;
use mxyz_config::SimulationVariant;
use mxyz_universe::state::State;

/// MXYZ Simulation Engine
pub struct Engine {
    pub engine_id: usize,
    pub config: EngineConfig,
    pub states: Vec<State>,
}

impl Engine {
    /// Creates a new engine instance
    pub fn new(engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];
        Engine {
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
        // let current_state = &self.states[self.config.step_id.0];

        // // Forward to next time-step.
        // // let next = current_state.next(&self.config, &self.states);

        // let mut next_state = State::new();
        // next_state.state_id = current_state.state_id + 1;

        // /// Loads current State
        // let current_state = &self.states[self.config.step_id.0];

        // /// Creates "neighborhoods"
        // let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes

        // /// Loops over systems & forwards each
        // for system in &current_state.systems {
        //     let mut next_system = system.clone();

        //     /// Gets all Integrators for this System & loops over them
        //     let integrators = tmp::get_integrators(&system, &self.config).unwrap();
        //     for integrator in integrators {
        //         /// Gets all Interacting Systems for this Interaction
        //         let other_ids = tmp::get_other_ids(&integrator, &current_state);
        //         /// Applies Interaction
        //         integrator.step(&mut next_system, &current_state, &other_ids);
        //     }
        //     next_state.systems.push(next_system);
        // }

        // self.states.push(next_state);
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

// mod tmp {

//     pub fn get_integrators<'a>(
//         system: &System,
//         config: &'a EngineConfig,
//     ) -> Option<&'a Vec<Integrator>> {
//         match config.integrators.get(system.system_id) {
//             None => None,
//             Some(vec) => Some(vec),
//         }
//     }

//     pub fn prepare_neighborhoods() -> Vec<fn() -> Vec<usize>> {
//         // TODO create neighborhoods
//         // - search for interaction partners
//         //  sys_id -> fn(entity_id) -> Vec<entity_jd>
//         //    - GameOfLife: create function, later return cells around entity_id
//         //    - PhysicalBodies: construct tree -> later return Vec<entity_jd> for entity_id
//         //    - only for system that are "felt" by others

//         // - from tree
//         // - from neighboring sectors
//         // - random
//         // - all
//         vec![]
//     }

//     enum _Tree {
//         // x,y,z   ->   vec[node]
//         Sectors, // all nodes from same + neighboring sectors
//         QuadOct, // quad/oct tree, return all nodes in opening angle
//         Total,   // all nodes are returned
//     }

//     pub fn get_other_ids(integrator: &Integrator, state: &State) -> Vec<usize> {
//         (0..state.systems.len())
//             .filter(|id| {
//                 let mut foo = false;
//                 for interaction in integrator.interactions.iter() {
//                     if match interaction.matrix.entries[*id] {
//                         None => false,
//                         Some(active) => active,
//                     } {
//                         foo = true;
//                     }
//                 }
//                 foo
//             })
//             .collect()
//     }
// }
