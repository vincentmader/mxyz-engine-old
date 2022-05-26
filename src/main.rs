#![allow(unused_doc_comments)]
use mxyz_engine::state::preset::SimulationId;
use mxyz_engine::Engine;
mod dev_utils;

fn main() {
    /// Creates & Initializes Engine
    let mut engine = Engine::new();
    engine.init(&Some(SimulationId::ThreeBodyFigureEight));

    /// Runs Engine & Records Execution Time
    dev_utils::print_state(&engine);
    let start_time = std::time::Instant::now();
    engine.run();
    let duration = start_time.elapsed().as_millis();
    println!("\nruntime:\t{} ms", duration);
    dev_utils::print_state(&engine);
    // dev_utils::print_interaction_matrix(&engine);
}
