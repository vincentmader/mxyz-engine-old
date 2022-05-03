use super::entity;
use super::entity::Entity;

#[derive(Debug, Clone)]
pub enum System {
    // PhysicalBodies(Entities<entity::PhysicalBody>),
    // ForceField(Entities<entity::ForceVector>),
    PhysicalBodies(PhysicalBodies),
    ForceField(ForceField),
}

// #[derive(Debug, Clone)]
// pub struct Entities<T: Entity> {
//     pub entities: Vec<T>,
// }
// impl<T: Entity> Entities<T> {
//     pub fn new() -> Entities<T> {
//         let entities = vec![];
//         Entities { entities }
//     }
// }

#[derive(Debug, Clone)]
pub struct PhysicalBodies {
    pub entities: Vec<entity::PhysicalBody>,
}
impl PhysicalBodies {
    pub fn new() -> Self {
        let mut entities = vec![];
        let mass = 1.;
        let position = [0., 0., 0.];
        let velocity = [0., 0., 0.];
        for i in 0..2 {
            let entity = entity::PhysicalBody::new(mass, [i as f64, 0., 0.], velocity);
            entities.push(entity);
        }
        PhysicalBodies { entities }
    }
}

#[derive(Debug, Clone)]
pub struct ForceField {
    pub entities: Vec<entity::ForceVector>,
}
impl ForceField {
    pub fn new() -> Self {
        let mut entities = vec![];
        for _ in 0..2 {
            let entity = entity::ForceVector::new();
            entities.push(entity);
        }
        ForceField { entities }
    }
}

// #[derive(Debug, Clone)]
// pub struct Force {}

// #[derive(Debug, Clone)]
// pub struct PhysicalBody {}

// pub enum EntityVector {
//     PhysicalBodies(Vec<Box<dyn Entity>>),
// }

// pub trait System {
//     fn entities(&mut self) -> Vec<Box<dyn Entity>>;
//     fn step(&mut self) {
//         for other_system in 0..2 {
//             // construct tree

//             match self.entities() {
//                 _ => {}
//             }
//             for entity in self.entities() {
//                 for other_entity in 0..2 {
//                     for interaction in 0..2 {
//                         interaction.apply()
//                     }
//                 }
//             }
//         }
//     }
// }

// pub struct PhysicalBodies {
//     bodies: EntityVector,
// }
// impl System for PhysicalBodies {
//     fn entities(&mut self) -> EntityVector {
//         self.bodies
//     }
// }

// pub struct ForceField {
//     cells: EntityVector,
// }
// impl System for ForceField {
//     fn entities(&mut self) -> EntityVector {
//         self.cells
//     }
// }

// pub trait SystemClone {
//     fn clone_box(&self) -> Box<dyn System>;
// }
// impl<T> SystemClone for T
// where
//     T: 'static + System + Clone,
// {
//     fn clone_box(&self) -> Box<dyn System> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn System> {
//     fn clone(&self) -> Box<dyn System> {
//         self.clone_box()
//     }
// }
