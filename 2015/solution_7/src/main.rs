type Signal = u16;

#[derive(PartialEq, Debug)]
enum SignalState {
    Signal(Signal),
    NoSignal,
}

struct Node {
    signal: SignalState,
}

impl Node {
    fn new(signal: SignalState) -> Self {
        Self { signal }
    }
}

type NodePair = (Node, Node);
type WireId = String;

mod circuit;

#[allow(dead_code)]
struct GateShift {
    input: SignalState,
    output: SignalState,
}

impl GateShift {
    fn new() -> Self {
        Self {
            input: SignalState::NoSignal,
            output: SignalState::NoSignal,
        }
    }

    fn run(&mut self) -> Result<(), RunError> {
        match self.input {
            SignalState::NoSignal => {
                return Err(RunError::InputNoSignal);
            }
            _ => Ok(()),
        }
    }
}

#[allow(dead_code)]
struct GateAnd {
    input: NodePair,
    signal: Signal,
    output: Node,
}

#[allow(dead_code)]
struct GateOr {
    input: NodePair,
    signal: Signal,
    output: Node,
}

enum RunError {
    InputNoSignal,
}

fn main() {
    println!("Hello, world!");
}
