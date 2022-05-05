use mxyz_engine::Engine;
// use mxyz_utils;

const NR_OF_STEPS: usize = 20000;

/// Main Engine Initializer
fn main() {
    let mut engine = Engine::new();
    engine.init();
    engine.config.max_step_id = NR_OF_STEPS;
    engine.run();

    println!("{:#?}", engine.states.get(engine.config.step_id).unwrap());
    // engine.export();
}
