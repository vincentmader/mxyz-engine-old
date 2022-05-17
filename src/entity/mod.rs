pub mod attribute;
use attribute::*;

pub trait Entity {}

#[derive(Debug, Clone)]
pub struct PhysicalObject {
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}
impl Entity for PhysicalObject {}
impl Mass for PhysicalObject {
    fn get_mass(&self) -> &f64 {
        &self.mass
    }
    fn get_mut_mass(&mut self) -> &mut f64 {
        &mut self.mass
    }
}
impl Position for PhysicalObject {
    fn get_position(&self) -> &[f64; 3] {
        &self.position
    }
    fn get_mut_position(&mut self) -> &mut [f64; 3] {
        &mut self.position
    }
}
impl Velocity for PhysicalObject {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn get_mut_velocity(&mut self) -> &mut [f64; 3] {
        &mut self.velocity
    }
}
impl PhysicalObject {
    pub fn new(mass: f64, position: [f64; 3], velocity: [f64; 3]) -> Self {
        PhysicalObject {
            mass,
            position,
            velocity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldCell {
    mass: f64,
}
impl Entity for FieldCell {}
impl Mass for FieldCell {
    fn get_mut_mass(&mut self) -> &mut f64 {
        &mut self.mass
    }
}
impl FieldCell {
    pub fn new() -> Self {
        let mass = 0.;
        FieldCell { mass }
    }
}
