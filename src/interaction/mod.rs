pub mod collision;
pub mod composed;
pub mod diffusion;
pub mod force;
pub mod game_of_life;
pub mod ising;
mod testing;
use crate::integrator::Integrator;
use crate::system::SystemVariant;

/// Interaction
pub struct Interaction {
    pub variant: InteractionVariant,
    pub integrator: Integrator,
    pub matrix: InteractionMatrix,
    //  TODO specify neighborhood/tree calculation
}
/// Interaction Variant
pub enum InteractionVariant {
    Force(force::Force),
    Collision(collision::Collision),
    // Diffusion(diffusion::Diffusion),
    // GameOfLife(game_of_life::GameOfLife),
    // Ising(ising::Ising),
    // Composed(Box<dyn InteractionTrait>),
}
/// Interaction Matrix
pub struct InteractionMatrix {
    pub entries: Vec<Vec<Option<bool>>>,
}
impl InteractionMatrix {
    pub fn new() -> Self {
        let entries = vec![];
        InteractionMatrix { entries }
    }
    pub fn init(&mut self, systems: &Vec<SystemVariant>) {
        for _ in 0..systems.len() {
            let mut row = vec![];
            for _ in 0..systems.len() {
                row.push(None);
            }
            self.entries.push(row);
        }
    }
    // TODO auto-add/rm rows/cells on system-add/rm
    // TODO run tests for matrix on system-delete
    // TODO run test for all sim_ids (initialization)
}
