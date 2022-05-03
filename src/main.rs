use mxyz_engine::Engine;

const NR_OF_STEPS: usize = 1670;

/// Main Engine Initializer
fn main() {
    let mut engine = Engine::new();
    engine.init();
    engine.run(Some(NR_OF_STEPS));
    println!("{:#?}", engine);
}
