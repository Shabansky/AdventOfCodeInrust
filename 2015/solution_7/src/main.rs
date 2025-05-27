type Signal = u16;

struct Node {
    signal: Signal,
}

impl Node {
    fn new(signal: Signal) -> Self {
        Self { signal }
    }
}

type NodePair = (Node, Node);

struct Wire {
    id: String,
    signal: Signal,
    input: Node,
    output: Vec<Node>,
}

impl Wire {
    fn new(id: String) -> Self {
        Self {
            id,
            signal: 0,
            input: Node::new(0),
            output: vec![],
        }
    }
}

#[allow(dead_code)]
struct GateShift {
    input: Node,
    output: Node,
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

#[allow(dead_code)]
struct GateNot {
    input: NodePair,
    signal: Signal,
    output: Node,
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wire_has_default_property_values() {
        let wire = Wire::new("id".to_string());

        assert_eq!("id".to_string(), wire.id);
        assert_eq!(0, wire.signal);
        assert_eq!(0, wire.input.signal);
        assert_eq!(0, wire.output.len());
    }
}
