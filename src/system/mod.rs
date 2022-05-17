pub mod discrete_field;
pub mod physical_objects;

#[derive(Clone)]
/// System Enumeration
pub enum System {
    DiscreteField(discrete_field::DiscreteField),
    PhysicalObjects(physical_objects::PhysicalObjects),
}
