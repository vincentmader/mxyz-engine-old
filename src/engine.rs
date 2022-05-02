use super::state::State;

pub struct Engine {
    states: Vec<State>,
}
impl Engine {
    pub fn new() -> Engine {
        let states = vec![];
        Engine { states }
    }
    pub fn init(&mut self) {}
    pub fn run(&mut self) {}
    pub fn step(&mut self) {}
}
