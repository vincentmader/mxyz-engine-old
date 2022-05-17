use super::entity;

// pub mod ant_hill;
// pub mod discrete_field;
// pub mod physical_object;

// use ant_hill::AntHill;
// use discrete_field::DiscreteField;
// use physical_object::PhysicalObjects;

#[derive(Debug, Clone)]
/// System Trait
pub enum System {
    PhysicalObjects(PhysicalObjects),
    DiscreteField(DiscreteField),
}

#[derive(Debug, Clone)]
/// System: Physical Objects
pub struct PhysicalObjects {
    pub entities: Vec<entity::PhysicalObject>,
}
impl PhysicalObjects {
    pub fn new() -> Self {
        let entities = vec![];
        PhysicalObjects { entities }
    }
}

#[derive(Debug, Clone)]
/// System: Force Field
pub struct DiscreteField {
    pub entities: Vec<entity::FieldCell>,
}
impl DiscreteField {
    pub fn new() -> Self {
        let entities = vec![];
        DiscreteField { entities }
    }
}
