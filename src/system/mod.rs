pub mod discrete_field;
pub mod physical_objects;
use crate::entity::Entity;
// use discrete_field::DiscreteField;
// use physical_objects::PhysicalObjects;

/// System Structure
#[derive(Clone)]
pub struct System {
    pub variant: SystemVariant,
    pub entities: Vec<Box<dyn Entity>>,
}
impl System {
    pub fn new(variant: SystemVariant) -> Self {
        let entities = vec![];
        System { variant, entities }
    }
    pub fn step() {}
}

/// System Variant Enumeration
#[derive(Clone)]
pub enum SystemVariant {
    // DiscreteField(DiscreteField),
    // PhysicalObjects(PhysicalObjects),
    DiscreteField,
    PhysicalObjects,
}
