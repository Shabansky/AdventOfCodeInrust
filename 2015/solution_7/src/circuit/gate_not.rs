use super::wire::Wire;

enum RunError {
    InputNoSignal,
}

#[allow(dead_code)]
pub struct GateNot<'a> {
    pub input: &'a Wire,
    pub output: &'a Wire,
}

impl<'a> GateNot<'a> {
    pub fn new(input: &'a Wire, output: &'a Wire) -> Self {
        Self { input, output }
    }

    fn run(&mut self) {}
}
