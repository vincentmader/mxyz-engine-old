#![allow(unused_doc_comments)]
use mxyz_config::ExportVariant;
use mxyz_config::SimulationVariant;
use mxyz_engine::Engine;
use std::sync::mpsc;
mod dev_utils;

const EXPORT_VARIANT: ExportVariant = ExportVariant::ToDatabase;
const CLIENT_ID: usize = 0;
const ENGINE_ID: usize = 0;

fn main() {
    /// Creates & Initializes Engine
    let mut engine = Engine::new(ENGINE_ID);
    engine.init(Some(SimulationVariant::ThreeBodyMoon));
    engine.config.export_variant = EXPORT_VARIANT;

    /// Runs Engine & Records Execution Time
    let start_time = std::time::Instant::now();
    engine.run();
    let duration = start_time.elapsed().as_millis();
    println!("\nruntime:\t{} ms", duration);

    // dev_utils::print_state(&engine);
    // dev_utils::_print_interaction_matrix(&engine);
}
