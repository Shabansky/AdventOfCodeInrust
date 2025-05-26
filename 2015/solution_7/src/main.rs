type Signal = u16;

struct Node {
    signal: Signal,
}

type NodePair = (Node, Node);

struct Wire {
    id: String,
    signal: Signal,
    input: Node,
    output: Vec<Node>,
}

struct GateShift {
    input: Node,
    output: Node,
}
struct GateAnd {
    input: NodePair,
    signal: Signal,
    output: Node,
}
struct GateOr {
    input: NodePair,
    signal: Signal,
    output: Node,
}
struct GateNot {
    input: NodePair,
    signal: Signal,
    output: Node,
}
fn main() {
    println!("Hello, world!");
}
