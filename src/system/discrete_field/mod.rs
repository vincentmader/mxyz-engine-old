use crate::entity::DiscreteFieldCell;

#[derive(Clone)]
/// System: Force Field
pub struct DiscreteField {
    pub entities: Vec<Box<dyn DiscreteFieldCell>>,
}
impl DiscreteField {
    pub fn new() -> Self {
        let entities = vec![];
        DiscreteField { entities }
    }
}
