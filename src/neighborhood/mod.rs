use mxyz_universe::system::System;

/// Neighboorhood Variant (not used at all a.t.m.)
pub enum _NeighbordhoodVariant {
    World,
    OctTree,
    Sectors,
    Random,
    Moore,
    VonNeumann,
}

/// Neighbordhood Trait
pub trait Neighboorhood {
    fn for_entity(&self, entity: (usize, usize), system: System) -> Vec<usize>;
}

mod world {
    use super::Neighboorhood;
    use mxyz_universe::system::System;
    /// World Neighboorhood (all entities)
    pub struct World {}
    impl Neighboorhood for World {
        fn for_entity(&self, _entity: (usize, usize), system: System) -> Vec<usize> {
            (0..system.entities.len()).collect()
        }
    }
    impl World {
        pub fn new() -> Self {
            World {}
        }
    }
}

mod oct_tree {
    use super::Neighboorhood;
    use mxyz_universe::system::System;
    /// Oct-Tree Neighboorhood
    pub struct OctTree {
        // TODO fields?
    }
    impl Neighboorhood for OctTree {
        fn for_entity(&self, _entity: (usize, usize), _system: System) -> Vec<usize> {
            todo!()
        }
    }
    impl OctTree {
        pub fn new() -> Self {
            OctTree {}
        }
    }
}

mod sector {
    use super::Neighboorhood;
    use mxyz_universe::system::System;
    /// Sector Neighborhood
    pub struct Sectors {
        // TODO fields?
    }
    impl Neighboorhood for Sectors {
        fn for_entity(&self, entity: (usize, usize), system: System) -> Vec<usize> {
            (0..system.entities.len())
                .filter(|id| is_in_same_sector(entity, (system.id, *id)))
                .collect()
        }
    }
    impl Sectors {
        pub fn new() -> Self {
            Sectors {}
        }
    }

    /// looks up whether or not two entities are in the same sector
    fn is_in_same_sector(_entity: (usize, usize), _other: (usize, usize)) -> bool {
        todo!()
    }
}

mod random {
    use super::Neighboorhood;
    use mxyz_universe::system::System;
    use rand::Rng;
    /// Randomly Constructed Neighboorhood
    pub struct Random {
        batch_size: usize,
    }
    impl Neighboorhood for Random {
        fn for_entity(&self, _entity: (usize, usize), _system: System) -> Vec<usize> {
            let mut rng = rand::thread_rng();
            (0..self.batch_size).map(|_| rng.gen()).collect()
        }
    }
    impl Random {
        pub fn new(batch_size: usize) -> Self {
            Random { batch_size }
        }
    }
}

mod moore {
    use super::Neighboorhood;
    use mxyz_universe::system::System;
    /// Moore Neighboorhood for Cellular Automata
    pub struct Moore {}
    impl Neighboorhood for Moore {
        fn for_entity(&self, _entity: (usize, usize), _system: System) -> Vec<usize> {
            // NOTE don't loop & filter!
            // TODO instead:
            // - get position from entity-id
            // - construct neighborhood from position
            // - return ids corresponding to position
            todo!()
        }
    }
    impl Moore {
        pub fn new() -> Self {
            Moore {}
        }
    }
}

mod von_neumann {
    use super::Neighboorhood;
    use mxyz_universe::system::System;
    /// Von Neumann Neighboorhood for Cellular Automata
    pub struct VonNeumann {}
    impl Neighboorhood for VonNeumann {
        fn for_entity(&self, _entity: (usize, usize), _system: System) -> Vec<usize> {
            // TODO analogous to Moore
            todo!()
        }
    }
    impl VonNeumann {
        pub fn new() -> Self {
            VonNeumann {}
        }
    }
}
