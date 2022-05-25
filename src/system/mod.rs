pub mod discrete_field;
pub mod physical_objects;
use crate::entity::Entity;
use discrete_field::DiscreteField;
use physical_objects::PhysicalObjects;

#[derive(Clone)]
/// System Variant Enumeration
pub enum SystemVariant {
    DiscreteField(DiscreteField),
    PhysicalObjects(PhysicalObjects),
}

pub struct System {
    entities: Vec<Box<dyn Entity>>,
}
impl System {
    pub fn new() -> Self {
        let entities = vec![];
        System { entities }
    }
}
