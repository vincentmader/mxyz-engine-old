use super::config::EngineConfig;
use super::config::ExportVariant;
use super::state::preset::SimulationId;
use super::state::State;
use mxyz_universe::system::System;
use mxyz_universe::system::SystemVariant;
use std::fs;

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
        next.step(&self.config, &self.states);
        self.states.push(next);
        self.config.step_id.0 += 1;
        self.export(); // TODO don't do this every step
    }
    /// Exports States (to File or Database)
    pub fn export(&self) {
        println!("Exporting...");
        let step_ids: Vec<usize> = (0..self.config.step_id.0).collect();
        match self.config.export_variant {
            /// Exports to File.
            ExportVariant::ToFile => self.export_to_file(&step_ids),
            /// Exports to Database.
            ExportVariant::ToDatabase => self.export_to_database(&step_ids),
        }
    }
    /// Exports States to File
    fn export_to_file(&self, step_ids: &Vec<usize>) {
        let mut contents = String::new();
        // Get current state.
        for step_id in step_ids.iter() {
            contents += &format!("STEP-ID: {}\n", step_id);
            let state = self.states.get(*step_id).unwrap();
            for system in state.systems.iter() {
                contents += &format!("SYSTEM: {}\n", system.id);
                for entity in system.entities.iter().enumerate() {
                    contents += &format!("ENTITY: {}\n", entity.0);
                    // TODO match outside of entity loop?
                    match system.variant {
                        SystemVariant::DiscreteField => {}
                        SystemVariant::PhysicalObjects => {
                            contents += &format!(
                                "{},{},{},{},{},{},{}\n",
                                entity.1.get_mass(),
                                entity.1.get_position()[0],
                                entity.1.get_position()[1],
                                entity.1.get_position()[2],
                                entity.1.get_velocity()[0],
                                entity.1.get_velocity()[1],
                                entity.1.get_velocity()[2]
                            );
                        }
                    }
                }
            }
        }
        let sim_id = 0;
        let path = format!("mxyz-engine/output/{}/output.txt", sim_id);
        fs::write(path, contents).unwrap();
    }
    /// Exports States to Database
    fn export_to_database(&self, step_ids: &Vec<usize>) {
        // TODO only write for recently updated/new step-ids
        // Get current state.
        let state = self.states.get(self.config.step_id.0).unwrap();
        // Establish Connection.
        let conn = mxyz_database::establish_connection();
        // Loop over Systems.
        for system in state.systems.iter() {
            let system_variant_id = System::get_variant_id(&system.variant);
            // Loop over Entities.
            for entity in system.entities.iter() {
                todo!("save entity to database")
            }
        }
        // TODO save: up to which step-id has state-vec been written to db?
    }
}
