pub mod attribute;
pub mod field;
pub mod object;
use attribute::*;

/// Entity Trait: Discrete Field Cell
pub trait Entity: EntityClone + Mass + Position + Velocity + Charge + Density + Force {}
pub trait EntityClone {
    fn clone_box(&self) -> Box<dyn Entity>;
}
impl<T> EntityClone for T
where
    T: 'static + Entity + Clone,
{
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Box<dyn Entity> {
        self.clone_box()
    }
}
