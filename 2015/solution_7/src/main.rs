type Signal = u16;

struct Node {
    signal: Signal,
}

struct Wire {
    id: String,
    signal: Signal,
    input: Node,
    output: Vec<Node>,
}
fn main() {
    println!("Hello, world!");
}
