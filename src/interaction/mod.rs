use super::integrator::Integrator;

#[derive(Debug)]
pub enum Interaction {
    NewtonianGravity(Integrator),
    Coulomb(Integrator),
    LennardJones(Integrator),

    Collision,
    Cohesion,
    Avoidance,
    Alignment,
    Diffusion,
    GameOfLife,
}
