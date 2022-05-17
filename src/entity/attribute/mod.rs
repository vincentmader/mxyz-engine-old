#![allow(dead_code)]
#![allow(unused_variables)]

/// Position (3D)
pub trait Position {
    fn get_position(&self) -> &[f64; 3] {
        &[0., 0., 0.]
    }
    fn set_position(&mut self, position: &[f64; 3]) {}
}
/// Velocity (3D)
pub trait Velocity {
    fn get_velocity(&self) -> &[f64; 3] {
        &[0., 0., 0.]
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {}
}
/// Acceleration (3D)
pub trait Acceleration {
    fn get_acceleration(&self) -> &[f64; 3] {
        &[0., 0., 0.]
    }
    fn set_acceleration(&mut self, acceleration: &[f64; 3]) {}
}
/// Force (3D)
pub trait Force {
    fn get_force(&self) -> &[f64; 3] {
        &[0., 0., 0.]
    }
    fn set_force(&mut self, force: &[f64; 3]) {}
}
/// Mass
pub trait Mass {
    fn get_mass(&self) -> f64 {
        0.
    }
    fn set_mass(&mut self, mass: f64) {}
}
/// Charge
pub trait Charge {
    fn get_charge(&self) -> f64 {
        0.
    }
    fn set_charge(&mut self, charge: f64) {}
}
/// Spin (1D)
pub trait Spin {
    /// Get 1D Spin Value
    ///
    /// - NOTE as of yet first case of a non-zero default value
    fn get_spin(&self) -> bool {
        false
    }
    fn set_spin(&mut self, spin: bool) {}
}
/// Radius
///
/// - e.g. for particles
pub trait Radius {
    fn get_radius(&self) -> f64 {
        0.
    }
    fn set_radius(&mut self, radius: f64) {}
}
/// Density
pub trait Density {
    fn get_density(&self) -> f64 {
        0.
    }
    fn set_density(&mut self, density: f64) {}
}
/// Temperature
pub trait Temperature {
    fn get_temperature(&self) -> f64 {
        0.
    }
    fn set_temperature(&mut self, temperature: f64) {}
}
/// GameOfLifeState
///
/// - TODO: use Spin instead?
pub trait GameOfLifeState {
    fn get_gol_state(&self) -> bool {
        false
    }
    fn set_gol_state(&mut self, gol_state: bool) {}
}
