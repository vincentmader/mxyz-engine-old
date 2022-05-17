use mxyz_engine::state::preset::SimulationId;
use mxyz_engine::Engine;

const NR_OF_STEPS: usize = 20000;

/// Main Engine Initializer
fn main() {
    // Create & Initialize Engine
    let mut engine = Engine::new();
    engine.init(&Some(SimulationId::Foo));
    engine.config.max_step_id = NR_OF_STEPS;
    // Run Engine & Record Execution Time
    let start_time = std::time::Instant::now();
    engine.run();
    // Print Information
    println!("");
    // println!("{:#?}", engine.states.get(engine.config.step_id).unwrap());
    println!("time it took: \t\t{} ms", start_time.elapsed().as_millis());
}
