use super::entity;

#[derive(Debug, Clone)]
/// System Trait
pub enum System {
    PhysicalBodies(PhysicalBodies),
    ForceField(ForceField),
}

#[derive(Debug, Clone)]
/// System: Physical Bodies
pub struct PhysicalBodies {
    pub entities: Vec<entity::PhysicalBody>,
}
impl PhysicalBodies {
    pub fn new() -> Self {
        let entities = vec![];
        PhysicalBodies { entities }
    }
}

#[derive(Debug, Clone)]
/// System: Force Field
pub struct ForceField {
    pub entities: Vec<entity::ForceVector>,
}
impl ForceField {
    pub fn new() -> Self {
        let entities = vec![];
        ForceField { entities }
    }
}
