pub mod system;
use super::integrator::Integrator;
// pub use system::SystemConfig;

pub struct EngineConfig {
    pub step_id: (usize, usize),
    // pub systems: Vec<SystemConfig>,
    // pub interactions: Vec<Interaction>,
    pub integrators: Vec<Vec<Integrator>>,
    pub constants: Constants,
    pub export_variant: ExportVariant,
}
impl EngineConfig {
    pub fn new() -> Self {
        // let systems = vec![];
        // let interactions = vec![];
        let integrators = vec![];
        let step_id = (0, usize::MAX);
        let constants = Constants::new();
        let export_variant = ExportVariant::ToDatabase;
        EngineConfig {
            // systems,
            // interactions,
            integrators,
            step_id,
            constants,
            export_variant,
        }
    }
}

pub struct Constants {
    _g: f64, // Newton's gravitational constant
}
impl Constants {
    pub fn new() -> Self {
        let g = 1.;
        Constants { _g: g }
    }
}

pub enum ExportVariant {
    ToDatabase,
    ToFile,
}

// // TODO move else-where
// pub struct InteractionMatrix {
//     rows: Vec<InteractionMatrixRow>,
// }
// impl InteractionMatrix {
//     pub fn new() -> Self {
//         let rows = vec![];
//         InteractionMatrix { rows }
//     }
//     pub fn add_row(&mut self) {
//         let row = InteractionMatrixRow::new(self.rows.len());
//         self.rows.push(row);
//         for row in self.rows.iter_mut() {
//             row.entries.push(vec![]);
//         }
//     }
//     pub fn remove_row(&mut self, row_id: usize) {
//         self.rows.remove(row_id);
//         for row in self.rows.iter_mut() {
//             row.entries.remove(row_id);
//         }
//     }
//     pub fn get_interactions(&self, system: usize, other: usize) -> &Vec<Interaction> {
//         self.rows.get(system).unwrap().entries.get(other).unwrap()
//     }
//     pub fn activate_interaction(&mut self, system: usize, other: usize, interaction: &Interaction) {
//         // let interactions = &mut self.rows[system].entries[other];
//         // if !interactions.contains(&interaction) {
//         //     interactions.push(interaction);
//         // }
//     }
//     pub fn deactivate_interaction(
//         &mut self,
//         system: usize,
//         other: usize,
//         interaction: &Interaction,
//     ) {
//         // let interactions = &mut self.rows[system].entries[other];
//         // interactions.retain(|i| i != interaction);
//     }
// }

// pub struct InteractionMatrixRow {
//     entries: Vec<Vec<Interaction>>,
// }
// impl InteractionMatrixRow {
//     pub fn new(nr_of_entries: usize) -> Self {
//         let mut entries = vec![];
//         for _ in 0..nr_of_entries {
//             entries.push(vec![]);
//         }
//         InteractionMatrixRow { entries }
//     }
// }
