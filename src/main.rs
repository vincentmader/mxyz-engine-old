#![allow(unused_doc_comments)]
use mxyz_engine::state::preset::SimulationId;
use mxyz_engine::Engine;

fn main() {
    /// Creates & Initializes Engine
    let mut engine = Engine::new();
    engine.init(&Some(SimulationId::ThreeBodyFigureEight));

    /// Runs Engine & Records Execution Time
    print_state(&engine);
    let start_time = std::time::Instant::now();
    engine.run();
    let duration = start_time.elapsed().as_millis();
    println!("\nruntime:\t{} ms", duration);
    print_state(&engine);
}

fn print_state(engine: &mxyz_engine::Engine) {
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
}
