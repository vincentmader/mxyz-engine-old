#![allow(dead_code)]
use super::integrator::Integrator;

#[derive(Debug)]
pub enum Interaction {
    Force(Force),

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

#[derive(Debug)]
pub struct Force {
    variant: ForceVariant,
    integrator: ForceIntegrator,
}

#[derive(Debug)]
pub enum ForceVariant {
    NewtonianGravity,
    Coulomb,
    LennardJones,
}

#[derive(Debug)]
pub enum ForceIntegrator {
    EulerExplicit,
    EulerImplicit,
    RungeKutta1,
    RungeKutta2,
    RungeKutta4,
    LeapFrog,
    Verlet,
    BulirschStoer,
    // Symplectic,
}
