use crate::entity::field::DiscreteFieldCell;
use crate::entity::object::PhysicalObject;
use crate::integrator::Integrator;
use crate::integrator::IntegratorVariant;
use crate::integrator::{euler_explicit, runge_kutta_4};

pub struct Force {
    pub variant: ForceVariant,
}
impl Force {
    pub fn _apply_to_field_from_field(
        &self,
        _entities: &mut Vec<Box<dyn DiscreteFieldCell>>,
        _others: &Vec<Box<dyn DiscreteFieldCell>>,
    ) {
    }
    pub fn _apply_to_field_from_objects(
        &self,
        _entities: &mut Vec<Box<dyn DiscreteFieldCell>>,
        _others: &Vec<Box<dyn PhysicalObject>>,
    ) {
    }
    pub fn _apply_to_objects_from_field(
        &self,
        _entities: &mut Vec<Box<dyn PhysicalObject>>,
        _others: &Vec<Box<dyn DiscreteFieldCell>>,
    ) {
    }
    pub fn apply_to_objects_from_objects(
        &self,
        entities: &mut Vec<Box<dyn PhysicalObject>>,
        others: &Vec<Box<dyn PhysicalObject>>,
        integrator: &Integrator,
        self_interaction: bool,
    ) {
        println!("\tforce!");

        let force_getter = match self.variant {
            ForceVariant::NewtonianGravity => force_newton,
            ForceVariant::Coulomb => force_coulomb,
            ForceVariant::Hooke => force_hooke,
            ForceVariant::LennardJones => force_lennard_jones,
        };

        let integrator = match integrator.variant {
            IntegratorVariant::EulerExplicit => euler_explicit,
            IntegratorVariant::RungeKutta4 => runge_kutta_4,
        };

        for (entity_id, mut entity) in entities.iter_mut().enumerate() {
            println!("\t{}", entity_id);
            for (other_id, other) in others.iter().enumerate() {
                if self_interaction {
                    if entity_id == other_id {
                        continue;
                    }
                }
                println!("\t\t{}", other_id);
                integrator(&mut entity, &other, force_getter);
            }
        }
    }
}

pub enum ForceVariant {
    Coulomb,
    NewtonianGravity,
    LennardJones,
    Hooke,
    // Cohesion,
    // Avoidance,
    // Alignment,
}

fn force_coulomb(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
    println!("\t\t\tCoulomb!");
    let (q1, q2) = (entity.get_charge(), other.get_charge());
    let (y1, y2) = (entity.get_position(), other.get_position());
    let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
    let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
    if r == 0. {
        return [0., 0., 0.]; // TODO sort out self in tree
    }
    const K: f64 = 1.; // TODO
    let force = K * (q1 * q2) / (r * r);
    let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
    let force = [force[0], force[1], force[2]];
    force
}
fn force_newton(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
    println!("\t\t\tNewton!");
    let (m1, m2) = (entity.get_mass(), other.get_mass());
    let (y1, y2) = (entity.get_position(), other.get_position());
    let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
    let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
    let u: Vec<f64> = u.iter().map(|i| i / r).collect();
    if r == 0. {
        return [0., 0., 0.]; // TODO sort out self in tree
    }
    const G: f64 = 1.; // TODO
    let force = G * (m1 * m2) / (r * r);
    let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
    println!("{:?}", force);
    let force = [force[0], force[1], force[2]];
    force
}
fn force_lennard_jones(
    entity: &Box<dyn PhysicalObject>,
    other: &Box<dyn PhysicalObject>,
) -> [f64; 3] {
    println!("\t\t\tLennard-Jones!");
    let (y1, y2) = (entity.get_position(), other.get_position());
    let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
    let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
    if r == 0. {
        return [0., 0., 0.]; // TODO sort out self in tree
    }
    const E: f64 = 1.;
    const S: f64 = 1.;
    let force = E * ((S / r).powf(13.) - (S / r).powf(7.));
    let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
    let force = [force[0], force[1], force[2]];
    force
}
fn force_hooke(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
    println!("\t\t\tHooke!");
    let (y1, y2) = (entity.get_position(), other.get_position());
    let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
    let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
    if r == 0. {
        return [0., 0., 0.]; // TODO sort out self in tree
    }
    const K: f64 = 1.; // TODO
    let force = K * r;
    let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
    let force = [force[0], force[1], force[2]];
    force
}
