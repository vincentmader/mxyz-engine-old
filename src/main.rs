#![allow(unused_doc_comments)]
use mxyz_engine::config::ExportVariant;
use mxyz_engine::Engine;
use mxyz_universe::preset::SimulationVariant;
use std::sync::mpsc;
mod dev_utils;

const EXPORT_VARIANT: ExportVariant = ExportVariant::ToDatabase;
const CLIENT_ID: usize = 0;
const ENGINE_ID: usize = 0;

fn main() {
    let (tx, rx) = mpsc::channel();

    /// Creates & Initializes Engine
    let mut engine = Engine::new(CLIENT_ID, ENGINE_ID, rx, tx);
    engine.init(&Some(SimulationVariant::ThreeBodyMoon));
    engine.config.export_variant = EXPORT_VARIANT;

    /// Runs Engine & Records Execution Time
    let start_time = std::time::Instant::now();
    engine.run();
    let duration = start_time.elapsed().as_millis();
    println!("\nruntime:\t{} ms", duration);

    // dev_utils::print_state(&engine);
    // dev_utils::_print_interaction_matrix(&engine);
}
