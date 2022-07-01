use super::config::EngineConfig;
use super::state::preset::SimulationId;
use super::state::State;
use mxyz_universe::entity::attribute::Mass;
use mxyz_universe::entity::attribute::Position;
use mxyz_universe::entity::attribute::Velocity;
use mxyz_universe::entity::object::planet::Planet;
use mxyz_universe::system::SystemVariant;
use std::sync::mpsc;

type M = usize; // TODO

/// MXYZ Simulation Engine
pub struct Engine {
    pub config: EngineConfig,
    pub states: Vec<State>,
    tx: mpsc::Sender<M>,
    rx: mpsc::Receiver<M>,
}

impl Engine {
    /// Creates a new engine instance
    pub fn new(rx: mpsc::Receiver<M>, tx: mpsc::Sender<M>) -> Self {
        let config = EngineConfig::new();
        let states = vec![];
        Engine {
            config,
            states,
            rx,
            tx,
        }
    }

    /// Initializes state & config
    pub fn init(&mut self, sim_id: &Option<SimulationId>) {
        println!("MXYZ-Engine: Initializing...");
        let mut initial_state = State::new();
        initial_state.init(sim_id, &mut self.config);
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
        // Forward state to next time-step.
        let next = current_state.next(&self.config, &self.states);
        self.states.push(next);
        self.config.step_id.0 += 1;
        // Export states every few time-steps.
        if self.config.step_id.0 % self.config.nr_of_steps_between_exports == 0 {
            self.export();
        }
    }

    pub fn send(&self) {}

    pub fn receive(&self) {}

    /// Exports States (to File or Database)
    pub fn export(&mut self) {
        use super::config::ExportVariant;
        println!("MXYZ-Engine: Exporting...");
        // Choose export method.
        match self.config.export_variant {
            ExportVariant::ToFile => self.export_to_file(),
            ExportVariant::ToDatabase => self.export_to_database(),
        }
        // Update step-id of last export.
        self.config.last_export_step_id = Some(self.config.step_id.0);
    }
}

// ============================================================================
// TODO move to separate module (?)

impl Engine {
    fn get_unsaved_state_ids(&self) -> Vec<usize> {
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

        println!("bbb: {:?}", a);
        a
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
            println!("{}", state_id);
            let state = self.states.get(state_id).unwrap();
            /// Loops over Systems.
            for system in state.systems.iter() {
                // let _system_variant_id = System::get_variant_id(&system.variant);
                let system_id = system.system_id;
                match &system.variant {
                    SystemVariant::Planets(system) => {
                        /// Loops over Entities.
                        for (planet_id, planet) in system.entities.iter().enumerate() {
                            let db_planet = mxyz_database::models::NewPlanet {
                                state_id: &(state_id as i32),
                                planet_id: &(planet_id as i32),
                                system_id: &(system_id as i32),
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
                let entity_variant_id = match &system.variant {
                    SystemVariant::Planets(_) => 0,
                    SystemVariant::PhysicalObjects(_) => 1,
                    _ => todo!(),
                };
                let db_system = mxyz_database::models::NewSystem {
                    state_id: &(state_id as i32),
                    system_id: &(system_id as i32),
                    entity_variant_id: &(entity_variant_id as i32),
                };
                mxyz_database::create_system(&conn, db_system);
            }
            let db_state = mxyz_database::models::NewState {
                state_id: &(state_id as i32),
            };
            mxyz_database::create_state(&conn, db_state);
        }
    }

    pub fn get_state_ids() -> Vec<usize> {
        use diesel::prelude::*;
        use mxyz_database::models::State;
        // use mxyz_database::schema::systems;
        use mxyz_database::schema::states::dsl::*;
        /// Establishes Connection.
        let connection = mxyz_database::establish_connection();
        let results = states
            .load::<State>(&connection)
            .expect("Error loading states");
        results.iter().map(|i| i.state_id as usize).collect()
    }

    pub fn get_system_ids(state_id_query: usize) -> Vec<usize> {
        use diesel::prelude::*;
        use mxyz_database::models::System;
        use mxyz_database::schema::systems::dsl::*;
        /// Establishes Connection.
        let connection = mxyz_database::establish_connection();
        let results = systems
            .filter(state_id.eq(state_id_query as i32))
            .load::<System>(&connection)
            .expect("Error loading systems");
        results.iter().map(|i| i.system_id as usize).collect()
    }

    pub fn get_entities(state_id_query: usize, system_id_query: usize) -> Vec<Planet> {
        use diesel::prelude::*;
        use mxyz_database::models::Planet;
        use mxyz_database::schema::planets::dsl::*;
        /// Establishes Connection.
        let connection = mxyz_database::establish_connection();

        let results = planets
            .filter(state_id.eq(state_id_query as i32))
            .filter(system_id.eq(system_id_query as i32))
            .load::<Planet>(&connection)
            .expect("Error loading planets");

        println!("planets:\n{:?}", results);
        // planets
        vec![]
    }

    pub fn get_updated_states(last_update: usize) -> Vec<State> {
        let current_state_id = std::cmp::max(1, Self::get_state_ids().len()) - 1;
        for state_id in last_update..current_state_id {
            let system_ids = Self::get_system_ids(state_id);
            for system_id in system_ids.iter() {
                let _entities = Self::get_entities(state_id, *system_id);
            }
        }

        let states = vec![];
        states
    }
}
