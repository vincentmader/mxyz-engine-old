use crate::config::EngineConfig;
use crate::integrator::Integrator;

pub fn get_integrators(sys_id: usize, sys_jd: usize, config: &EngineConfig) -> Vec<&Integrator> {
    config
        .integrators
        .iter()
        .filter(|i| a(i, sys_id, sys_jd))
        .collect()
}
fn a(i: &Integrator, sys_id: usize, sys_jd: usize) -> bool {
    match i.matrix.entries[sys_id][sys_jd] {
        Some(e) => e,  // filter out all entries set to `false`
        None => false, // also disregard interaction-matrix entries set to `None`
    }
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
