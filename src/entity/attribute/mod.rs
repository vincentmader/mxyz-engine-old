/// Mass
pub trait Mass {
    fn get_mass(&self) -> &f64 {
        &0.
    }
    fn get_mut_mass(&mut self) -> &mut f64;
}
/// Position (3D)
pub trait Position {
    fn get_position(&self) -> &[f64; 3] {
        &[0., 0., 0.]
    }
    fn get_mut_position(&mut self) -> &mut [f64; 3];
}
/// Velocity (3D)
pub trait Velocity {
    fn get_velocity(&self) -> &[f64; 3] {
        &[0., 0., 0.]
    }
    fn get_mut_velocity(&mut self) -> &mut [f64; 3];
}
/// Charge
pub trait Charge {
    fn get_charge(&self) -> &f64 {
        &0.
    }
    fn get_mut_charge(&mut self) -> &mut f64;
}
