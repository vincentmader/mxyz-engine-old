use super::PhysicalObject;
use crate::attribute::*;

#[derive(Clone)]
pub struct Planet {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}
impl PhysicalObject for Planet {}
impl Planet {
    pub fn new(mass: f64, position: [f64; 3], velocity: [f64; 3]) -> Self {
        Planet {
            position,
            velocity,
            mass,
        }
    }
}
impl Mass for Planet {
    fn get_mass(&self) -> f64 {
        self.mass
    }
    fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }
}
impl Position for Planet {
    fn get_position(&self) -> &[f64; 3] {
        &self.position
    }
    fn set_position(&mut self, position: &[f64; 3]) {
        self.position = *position;
    }
}
impl Velocity for Planet {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {
        self.velocity = *velocity;
    }
}
impl Charge for Planet {}
