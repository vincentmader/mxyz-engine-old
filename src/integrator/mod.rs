#[derive(Debug)]
/// Entity Integrator
pub enum Integrator {
    // forces on objects
    EulerExplicit,
    EulerImplicit,
    RungeKutta2,
    RungeKutta4,
    LeapFrog,
    Verlet,
    Collision,
    // fields
    CellularAutomaton,
    MonteCarlo,
}
