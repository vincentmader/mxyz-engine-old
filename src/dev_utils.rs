pub fn print_state(engine: &mxyz_engine::Engine) {
    let state = &engine.states[engine.config.step_id.0];
    println!("\n  Steps: {}", engine.config.step_id.0);
    for system in state.systems.iter() {
        match system {
            mxyz_engine::system::SystemVariant::PhysicalObjects(f) => {
                for e in f.entities.iter() {
                    println!("    {:?}", e.get_position());
                }
            }
            _ => todo!(),
        }
    }
    println!("");
}

pub fn print_interaction_matrix(engine: &mxyz_engine::Engine) {
    let interactions = &engine.config.interactions;
    for (int_id, interaction) in interactions.iter().enumerate() {
        println!("interaction {}", int_id);

        let matrix = &interaction.matrix;
        for (sys_id, row) in matrix.rows.iter().enumerate() {
            println!("  system {}", sys_id);
            for (other_id, entry) in row.entries.iter().enumerate() {
                println!("    other {}\t{:?}", other_id, entry.integrator);
            }
        }
    }
    println!("");
}
