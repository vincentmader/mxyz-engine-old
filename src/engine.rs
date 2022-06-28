use super::config::EngineConfig;
use super::state::preset::SimulationId;
use super::state::State;
use mxyz_universe::system::System;
use mxyz_universe::system::SystemVariant;

/// Simulation Engine
pub struct Engine {
    pub config: EngineConfig,
    pub states: Vec<State>,
}
impl Engine {
    /// Creates a new instance of Engine Structure
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
        next.id = self.config.step_id.0 + 1; // state-id needed for RAM-clearing (?)

        next.step(&self.config, &self.states);
        self.states.push(next);
        self.config.step_id.0 += 1;
        if self.config.step_id.0 % self.config.nr_of_steps_between_exports == 0 {
            self.export();
        }
    }
    /// Exports States (to File or Database)
    pub fn export(&mut self) {
        use super::config::ExportVariant;
        println!("Exporting...");
        match self.config.export_variant {
            ExportVariant::ToFile => self.export_to_file(),
            ExportVariant::ToDatabase => self.export_to_database(),
        }
        self.config.last_export_step_id = Some(self.config.step_id.0);
    }
}

// TODO move to separate module (?)
impl Engine {
    fn get_unsaved_state_ids(&self) -> Vec<usize> {
        self.states
            .iter()
            .filter(|state| {
                state.id
                    >= match self.config.last_export_step_id {
                        None => 0,
                        Some(e) => e + 1,
                    }
            })
            .map(|state| state.id)
            .collect()
    }
    /// Exports States to File
    fn export_to_file(&self) {
        let sim_id = 0; // TODO
        let out_dir = format!("./mxyz-engine/output/{}", sim_id);
        /// Loops over unsaved States.
        for state_id in self.get_unsaved_state_ids() {
            let state = self.states.get(state_id).unwrap();
            /// Saves to File
            let path = format!("{}/{}.txt", out_dir, state_id);
            std::fs::write(path, format!("{:#?}", state)).unwrap();
        }
    }
    // TODO
    /// Exports States to Database
    fn export_to_database(&self) {
        /// Establishes Connection.
        let conn = mxyz_database::establish_connection();
        /// Loops over unsaved States.
        for state_id in self.get_unsaved_state_ids() {
            let state = self.states.get(state_id).unwrap();
            /// Loops over Systems.
            for system in state.systems.iter() {
                let _system_variant_id = System::get_variant_id(&system.variant);
                match system.variant {
                    SystemVariant::PhysicalObjects => {
                        /// Loops over Entities.
                        for (planet_id, planet) in system.entities.iter().enumerate() {
                            let db_planet = mxyz_database::models::NewPlanet {
                                planet_id: &(planet_id as i32),
                                system_id: &(system.id as i32),
                                mass: &planet.get_mass(),
                                pos_x: &planet.get_position()[0],
                                pos_y: &planet.get_position()[1],
                                pos_z: &planet.get_position()[2],
                                vel_x: &planet.get_velocity()[0],
                                vel_y: &planet.get_velocity()[1],
                                vel_z: &planet.get_velocity()[2],
                            };
                            mxyz_database::create_planet(&conn, db_planet);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
