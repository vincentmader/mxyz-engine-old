pub mod discrete_field;
pub mod physical_objects;
use discrete_field::DiscreteField;
use physical_objects::PhysicalObjects;

#[derive(Clone)]
/// System Variant Enumeration
pub enum SystemVariant {
    DiscreteField(DiscreteField),
    PhysicalObjects(PhysicalObjects),
}
