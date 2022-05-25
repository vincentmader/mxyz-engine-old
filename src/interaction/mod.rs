pub mod collision;
pub mod composed;
pub mod diffusion;
pub mod force;
pub mod game_of_life;
pub mod interaction_matrix;
pub mod ising;
mod testing;
use interaction_matrix::InteractionMatrix;

/// Interaction
pub struct Interaction {
    pub variant: InteractionVariant,
    pub matrix: InteractionMatrix,
    pub active: bool,
}
impl Interaction {
    pub fn new(variant: InteractionVariant) -> Self {
        Interaction {
            variant,
            matrix: InteractionMatrix::new(),
            active: true,
        }
    }
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
