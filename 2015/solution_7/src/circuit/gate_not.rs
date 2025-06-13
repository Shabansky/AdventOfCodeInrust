use crate::{SignalState, WireId};
enum RunError {
    InputNoSignal,
}

#[allow(dead_code)]
pub struct GateNot {
    pub input: WireId,
    pub output: WireId,
}

impl GateNot {
    pub fn new(input: WireId, output: WireId) -> Self {
        Self { input, output }
    }

    fn run(&mut self) {}
}
