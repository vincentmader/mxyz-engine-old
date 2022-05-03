#[derive(Debug)]
pub enum Interaction {
    Collision,
    NewtonianGravity,
    Coulomb,
    LennardJones,
    Cohesion,
    Avoidance,
    Alignment,
    Diffusion,
    GameOfLife,
}
