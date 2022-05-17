use crate::entity::PhysicalObject;

#[derive(Clone)]
/// System: Physical Objects
pub struct PhysicalObjects {
    pub entities: Vec<Box<dyn PhysicalObject>>,
}
impl PhysicalObjects {
    pub fn new() -> Self {
        let entities = vec![];
        PhysicalObjects { entities }
    }
}
