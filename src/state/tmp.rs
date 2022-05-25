use crate::config::EngineConfig;
use crate::integrator::Integrator;
use crate::interaction::Interaction;

pub fn get_interactions(sys_id: usize, sys_jd: usize, config: &EngineConfig) -> Vec<&Interaction> {
    config
        .interactions
        .iter()
        .filter(|interaction| {
            match interaction
                .matrix
                .rows
                .get(sys_id)
                .expect(&format!(
                    "System-ID \"{}\" not found in interaction matrix",
                    sys_id
                ))
                .entries
                .get(sys_jd)
                .expect(&format!(
                    "System-JD \"{}\" not found in interaction matrix",
                    sys_jd
                ))
                .active
            {
                Some(active) => active, // filter out all entries set to `false`
                None => false,          // also disregard entries set to `None`
            }
        })
        .collect()
}

pub fn prepare_neighborhoods() -> Vec<fn() -> Vec<usize>> {
    // TODO create neighborhoods
    // - search for interaction partners
    //  sys_id -> fn(entity_id) -> Vec<entity_jd>
    //    - GameOfLife: create function, later return cells around entity_id
    //    - PhysicalBodies: construct tree -> later return Vec<entity_jd> for entity_id
    //    - only for system that are "felt" by others

    // - from tree
    // - from neighboring sectors
    // - random
    // - all
    vec![]
}

enum _Tree {
    // x,y,z   ->   vec[node]
    Sectors, // all nodes from same + neighboring sectors
    QuadOct, // quad/oct tree, return all nodes in opening angle
    Total,   // all nodes are returned
}
