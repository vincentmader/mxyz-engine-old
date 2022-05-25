use crate::integrator::{Integrator, IntegratorVariant};
use crate::system::SystemVariant;

// TODO move elsewhere
/// Interaction Matrix
pub struct InteractionMatrix {
    pub rows: Vec<InteractionMatrixRow>, // pub entries: Vec<Vec<Option<bool>>>,
}
impl InteractionMatrix {
    pub fn new() -> Self {
        let rows = vec![];
        InteractionMatrix { rows }
    }
    pub fn init(&mut self, systems: &Vec<SystemVariant>) {
        for _ in 0..systems.len() {
            let mut row = InteractionMatrixRow::new();
            row.init(&systems);
            self.rows.push(row);
        }
    }
    // TODO auto-add/rm rows/cells on system-add/rm
    // TODO run tests for matrix on system-delete
    // TODO run test for all sim_ids (initialization)
}

pub struct InteractionMatrixRow {
    pub entries: Vec<InteractionMatrixEntry>,
}
impl InteractionMatrixRow {
    pub fn new() -> Self {
        let entries = vec![];
        InteractionMatrixRow { entries }
    }
    pub fn init(&mut self, systems: &Vec<SystemVariant>) {
        for _ in 0..systems.len() {
            self.entries.push(InteractionMatrixEntry::new());
        }
    }
}

pub struct InteractionMatrixEntry {
    pub active: Option<bool>,
    pub integrator: Option<Integrator>,
}
impl InteractionMatrixEntry {
    pub fn new() -> Self {
        let active = None;
        let integrator = None;
        InteractionMatrixEntry { active, integrator }
    }
}
