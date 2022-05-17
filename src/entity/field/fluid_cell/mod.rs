use super::DiscreteFieldCell;
use crate::attribute::*;

#[derive(Clone)]
pub struct FluidCell {
    velocity: [f64; 3],
    density: f64,
}
impl DiscreteFieldCell for FluidCell {}
impl FluidCell {
    pub fn new(velocity: [f64; 3], density: f64) -> Self {
        FluidCell { velocity, density }
    }
}
impl Velocity for FluidCell {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {
        self.velocity = *velocity;
    }
}
impl Density for FluidCell {
    fn get_density(&self) -> f64 {
        self.density
    }
    fn set_density(&mut self, density: f64) {
        self.density = density;
    }
}
impl Force for FluidCell {}
