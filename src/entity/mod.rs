pub mod attribute;
use attribute::*;

pub trait Entity {}

#[derive(Debug, Clone)]
pub struct PhysicalBody {
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}
impl Entity for PhysicalBody {}
impl Mass for PhysicalBody {
    fn get_mass(&self) -> &f64 {
        &self.mass
    }
    fn get_mut_mass(&mut self) -> &mut f64 {
        &mut self.mass
    }
}
impl Position for PhysicalBody {
    fn get_position(&self) -> &[f64; 3] {
        &self.position
    }
    fn get_mut_position(&mut self) -> &mut [f64; 3] {
        &mut self.position
    }
}
impl Velocity for PhysicalBody {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn get_mut_velocity(&mut self) -> &mut [f64; 3] {
        &mut self.velocity
    }
}
impl PhysicalBody {
    pub fn new(mass: f64, position: [f64; 3], velocity: [f64; 3]) -> Self {
        PhysicalBody {
            mass,
            position,
            velocity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FluidCell {}

#[derive(Debug, Clone)]
pub struct ForceVector {
    mass: f64,
}
impl Entity for ForceVector {}
impl Mass for ForceVector {
    fn get_mut_mass(&mut self) -> &mut f64 {
        &mut self.mass
    }
}
impl ForceVector {
    pub fn new() -> Self {
        let mass = 0.;
        ForceVector { mass }
    }
}
